use super::{
    combine_blocks, parse_code_blocks, AssemblyContext, AssemblyError, BTreeSet, CodeBlock,
    CodeBlockTable, String, Token, TokenStream, Vec,
};
use core::ops;
use crypto::{hashers::Blake3_256, Digest, Hasher};
use vm_core::{Felt, Operation};

// PROCEDURE
// ================================================================================================

#[derive(Clone, Debug)]
/// Contains metadata and code of a procedure.
pub struct Procedure {
    pub(crate) label: String,
    pub(crate) is_export: bool,
    #[allow(dead_code)]
    pub(crate) num_locals: u32,
    pub(crate) code_root: CodeBlock,
    pub(crate) callset: CallSet,
}

impl Procedure {
    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a root of this procedure's MAST.
    pub fn code_root(&self) -> &CodeBlock {
        &self.code_root
    }

    /// Returns a label of this procedure.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns `true` if this is an exported procedure.
    pub fn is_export(&self) -> bool {
        self.is_export
    }

    /// Returns a reference to a set of all procedures (identified by their IDs) which may be
    /// called during the execution of this procedure.
    pub fn callset(&self) -> &CallSet {
        &self.callset
    }

    // PARSER
    // --------------------------------------------------------------------------------------------

    /// Parses and returns a single procedure from the provided token stream.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The token stream does not contain a procedure header token at the current position.
    /// - Parsing of procedure header token fails (e.g., invalid procedure label).
    /// - The procedure is an exported procedure and `allow_export` is false.
    /// - A procedure with the same label already exists in the provided context.
    /// - Parsing of procedure body fails for any reason.
    /// - The procedure body does not terminate with the `END` token.
    pub fn parse(
        tokens: &mut TokenStream,
        context: &AssemblyContext,
        cb_table: &mut CodeBlockTable,
        allow_export: bool,
    ) -> Result<Self, AssemblyError> {
        let proc_start = tokens.pos();

        // read procedure name and consume the procedure header token
        let header = tokens.read().expect("missing procedure header");
        let (label, num_locals, is_export) = header.parse_proc()?;
        if !allow_export && is_export {
            return Err(AssemblyError::proc_export_not_allowed(header, &label));
        }
        if context.contains_proc(&label) {
            return Err(AssemblyError::duplicate_proc_label(header, &label));
        }
        tokens.advance();

        // parse procedure body, and handle memory allocation/deallocation of locals if any are declared
        let code_root = parse_proc_blocks(tokens, context, cb_table, num_locals)?;

        // consume the 'end' token
        match tokens.read() {
            None => Err(AssemblyError::unmatched_proc(
                tokens.read_at(proc_start).expect("no proc token"),
            )),
            Some(token) => match token.parts()[0] {
                Token::END => token.validate_end(),
                _ => Err(AssemblyError::unmatched_proc(
                    tokens.read_at(proc_start).expect("no proc token"),
                )),
            },
        }?;
        tokens.advance();

        // build and return the procedure
        Ok(Self {
            label,
            is_export,
            num_locals,
            code_root,
            callset: CallSet::default(),
        })
    }
}

// PROCEDURE ID
// ================================================================================================

/// A procedure identifier computed as a digest truncated to [`Self::LEN`] bytes, product of the
/// label of a procedure
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcedureId(pub [u8; Self::SIZE]);

impl From<[u8; ProcedureId::SIZE]> for ProcedureId {
    fn from(value: [u8; ProcedureId::SIZE]) -> Self {
        Self(value)
    }
}

impl ops::Deref for ProcedureId {
    type Target = [u8; Self::SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ProcedureId {
    /// Truncated length of the id
    pub const SIZE: usize = 24;

    /// Creates a new procedure id from its label, composed by module path + name identifier.
    ///
    /// No validation is performed regarding the consistency of the label structure
    pub fn new<L>(label: L) -> Self
    where
        L: AsRef<str>,
    {
        let mut digest = [0u8; Self::SIZE];
        let hash = Blake3_256::<Felt>::hash(label.as_ref().as_bytes());
        digest.copy_from_slice(&hash.as_bytes()[..Self::SIZE]);
        Self(digest)
    }
}

// CALLSET
// ================================================================================================

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CallSet(BTreeSet<ProcedureId>);

impl CallSet {
    pub fn insert(&mut self, proc_id: ProcedureId) {
        self.0.insert(proc_id);
    }

    pub fn append(&mut self, other: &CallSet) {
        for &item in other.0.iter() {
            self.0.insert(item);
        }
    }

    /// TODO: ideally should be converted into iter() method which would return an iterator.
    pub fn inner(&self) -> &BTreeSet<ProcedureId> {
        &self.0
    }
}

// HELPER FUNCTIONS
// ================================================================================================

pub fn parse_proc_blocks(
    tokens: &mut TokenStream,
    context: &AssemblyContext,
    cb_table: &mut CodeBlockTable,
    num_proc_locals: u32,
) -> Result<CodeBlock, AssemblyError> {
    // parse the procedure body
    let body = parse_code_blocks(tokens, context, cb_table, num_proc_locals)?;

    if num_proc_locals == 0 {
        // if no allocation of locals is required, return the procedure body
        return Ok(body);
    }

    let mut blocks = Vec::new();
    let locals_felt = Felt::new(num_proc_locals as u64);

    // allocate procedure locals before the procedure body
    let alloc_ops = vec![Operation::Push(locals_felt), Operation::FmpUpdate];
    blocks.push(CodeBlock::new_span(alloc_ops));

    // add the procedure body code block
    blocks.push(body);

    // deallocate procedure locals after the procedure body
    let dealloc_ops = vec![Operation::Push(-locals_felt), Operation::FmpUpdate];
    blocks.push(CodeBlock::new_span(dealloc_ops));

    // combine the local memory alloc/dealloc blocks with the procedure body code block
    Ok(combine_blocks(blocks))
}
