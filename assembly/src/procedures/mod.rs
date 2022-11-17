use super::{
    combine_blocks, parse_code_blocks, AssemblyContext, AssemblyError, BTreeSet, CodeBlock,
    CodeBlockTable, String, Token, TokenStream, Vec, MODULE_PATH_DELIM,
};
use core::{fmt, ops};
use crypto::{hashers::Blake3_256, Digest, Hasher};
use vm_core::{Felt, Operation};

// PROCEDURE
// ================================================================================================

#[derive(Clone, Debug)]
/// Contains metadata and code of a procedure.
pub struct Procedure {
    id: ProcedureId,
    label: String,
    is_export: bool,
    num_locals: u32,
    code_root: CodeBlock,
    callset: CallSet,
}

impl Procedure {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [Procedure] instantiated with the specified properties.
    pub fn new(
        id: ProcedureId,
        label: String,
        is_export: bool,
        num_locals: u32,
        code_root: CodeBlock,
        callset: CallSet,
    ) -> Self {
        Procedure {
            id,
            label,
            is_export,
            num_locals,
            code_root,
            callset,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns ID of this procedure.
    pub fn id(&self) -> &ProcedureId {
        &self.id
    }

    /// Returns a label of this procedure.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns `true` if this is an exported procedure.
    pub fn is_export(&self) -> bool {
        self.is_export
    }

    /// Returns the number of memory locals reserved by the procedure.
    #[allow(dead_code)]
    pub fn num_locals(&self) -> u32 {
        self.num_locals
    }

    /// Returns a root of this procedure's MAST.
    pub fn code_root(&self) -> &CodeBlock {
        &self.code_root
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
            id: ProcedureId::default(),
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

impl fmt::Display for ProcedureId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;
        for byte in self.0.iter() {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl ProcedureId {
    /// Truncated length of the id
    pub const SIZE: usize = 24;

    /// Base kernel path
    /// TODO better use `MODULE_PATH_DELIM`. maybe require `const_format` crate?
    pub const KERNEL_PATH: &str = "::sys";

    /// Creates a new procedure id from its path, composed by module path + name identifier.
    ///
    /// No validation is performed regarding the consistency of the path format.
    pub fn new<L>(path: L) -> Self
    where
        L: AsRef<str>,
    {
        let mut digest = [0u8; Self::SIZE];
        let hash = Blake3_256::<Felt>::hash(path.as_ref().as_bytes());
        digest.copy_from_slice(&hash.as_bytes()[..Self::SIZE]);
        Self(digest)
    }

    /// Computes the full path, given a module path and the procedure name
    pub fn path<N, M>(name: N, module_path: M) -> String
    where
        N: AsRef<str>,
        M: AsRef<str>,
    {
        format!(
            "{}{MODULE_PATH_DELIM}{}",
            module_path.as_ref(),
            name.as_ref()
        )
    }

    /// Creates a new procedure ID from a name to be resolved in the kernel.
    pub fn from_kernel_name(name: &str) -> Self {
        let path = format!("{}{MODULE_PATH_DELIM}{name}", Self::KERNEL_PATH);
        Self::new(path)
    }

    /// Creates a new procedure ID from its name and module path.
    ///
    /// No validation is performed regarding the consistency of the module path or procedure name
    /// format.
    pub fn from_name(name: &str, module_path: &str) -> Self {
        let path = Self::path(name, module_path);
        Self::new(path)
    }

    /// Creates a new procedure ID from its local index and module path.
    ///
    /// No validation is performed regarding the consistency of the module path format.
    pub fn from_index(index: u16, module_path: &str) -> Self {
        let path = format!("{module_path}{MODULE_PATH_DELIM}{index}");
        Self::new(path)
    }
}

// CALLSET
// ================================================================================================

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CallSet(BTreeSet<ProcedureId>);

impl CallSet {
    pub fn contains(&self, proc_id: &ProcedureId) -> bool {
        self.0.contains(proc_id)
    }

    pub fn insert(&mut self, proc_id: ProcedureId) {
        self.0.insert(proc_id);
    }

    pub fn append(&mut self, other: &CallSet) {
        for &item in other.0.iter() {
            self.0.insert(item);
        }
    }
}

impl ops::Deref for CallSet {
    type Target = BTreeSet<ProcedureId>;

    fn deref(&self) -> &Self::Target {
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
