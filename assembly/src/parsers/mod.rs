use super::{
    BTreeMap, ByteReader, ByteWriter, Deserializable, DeserializationError, Felt, LabelError,
    LibraryPath, ParsingError, ProcedureId, ProcedureName, Serializable, SliceReader, StarkField,
    String, ToString, Token, TokenStream, Vec, MAX_LABEL_LEN,
};
use core::{fmt::Display, ops::RangeBounds, str::from_utf8};

mod nodes;
use crate::utils::bound_into_included_u64;
pub use nodes::{Instruction, Node};
mod context;
use context::ParserContext;
mod labels;
use labels::CONSTANT_LABEL_PARSER;
pub use labels::{NAMESPACE_LABEL_PARSER, PROCEDURE_LABEL_PARSER};

mod adv_ops;
mod field_ops;
mod io_ops;
mod serde;
mod stack_ops;
mod u32_ops;

#[cfg(test)]
pub mod tests;

// CONSTANTS
// ================================================================================================

/// Maximum number of procedures in a module.
const MAX_LOCL_PROCS: usize = u16::MAX as usize;

/// Maximum number of bytes for a single documentation comment.
const MAX_DOCS_LEN: usize = u16::MAX as usize;

// TYPE ALIASES
// ================================================================================================
type LocalProcMap = BTreeMap<String, (u16, ProcedureAst)>;
type LocalConstMap = BTreeMap<String, u64>;

// EXECUTABLE PROGRAM AST
// ================================================================================================

/// An abstract syntax tree (AST) of a Miden program.
///
/// A program AST consists of a list of internal procedure ASTs and a list of body nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramAst {
    local_procs: Vec<ProcedureAst>,
    body: Vec<Node>,
}

impl ProgramAst {
    // AST
    // --------------------------------------------------------------------------------------------
    /// Constructs a [ProgramAst].
    ///
    /// A program consist of a body and a set of internal (i.e., not exported) procedures.
    pub fn new(local_procs: Vec<ProcedureAst>, body: Vec<Node>) -> Result<Self, ParsingError> {
        if local_procs.len() > MAX_LOCL_PROCS {
            return Err(ParsingError::too_many_module_procs(local_procs.len(), MAX_LOCL_PROCS));
        }
        Ok(Self { local_procs, body })
    }

    // PARSER
    // --------------------------------------------------------------------------------------------
    /// Parses the provided source into a [ProgramAst].
    ///
    /// A program consist of a body and a set of internal (i.e., not exported) procedures.
    pub fn parse(source: &str) -> Result<ProgramAst, ParsingError> {
        let mut tokens = TokenStream::new(source)?;
        let imports = parse_imports(&mut tokens)?;
        let local_constants = parse_constants(&mut tokens)?;

        let mut context = ParserContext {
            imports,
            local_constants,
            ..Default::default()
        };

        context.parse_procedures(&mut tokens, false)?;

        // make sure program body is present
        let next_token = tokens
            .read()
            .ok_or_else(|| ParsingError::unexpected_eof(*tokens.eof_location()))?;
        if next_token.parts()[0] != Token::BEGIN {
            return Err(ParsingError::unexpected_token(next_token, Token::BEGIN));
        }

        let program_start = tokens.pos();
        // consume the 'begin' token
        let header = tokens.read().expect("missing program header");
        header.validate_begin()?;
        tokens.advance();

        // make sure there is something to be read
        if tokens.eof() {
            return Err(ParsingError::unexpected_eof(*tokens.eof_location()));
        }

        // parse the sequence of nodes and add each node to the list
        let body = context.parse_body(&mut tokens, false)?;

        // consume the 'end' token
        match tokens.read() {
            None => Err(ParsingError::unmatched_begin(
                tokens.read_at(program_start).expect("no begin token"),
            )),
            Some(token) => match token.parts()[0] {
                Token::END => token.validate_end(),
                Token::ELSE => Err(ParsingError::dangling_else(token)),
                _ => Err(ParsingError::unmatched_begin(
                    tokens.read_at(program_start).expect("no begin token"),
                )),
            },
        }?;
        tokens.advance();

        // make sure there are no instructions after the end
        if let Some(token) = tokens.read() {
            return Err(ParsingError::dangling_ops_after_program(token));
        }

        let local_procs = sort_procs_into_vec(context.local_procs);
        Self::new(local_procs, body)
    }

    // SERIALIZATION / DESERIALIZATION
    // --------------------------------------------------------------------------------------------

    /// Returns byte representation of this [ProgramAst].
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut target = Vec::<u8>::default();

        // asserts below are OK because we enforce limits on the number of procedure and the
        // number of body instructions in relevant parsers

        assert!(self.local_procs.len() <= u16::MAX as usize, "too many local procs");
        target.write_u16(self.local_procs.len() as u16);
        self.local_procs.write_into(&mut target);

        assert!(self.body.len() <= u16::MAX as usize, "too many body instructions");
        target.write_u16(self.body.len() as u16);
        self.body.write_into(&mut target);

        target
    }

    /// Returns a [ProgramAst] struct deserialized from the provided bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, DeserializationError> {
        let mut source = SliceReader::new(bytes);

        let num_local_procs = source.read_u16()?;
        let local_procs = Deserializable::read_batch_from(&mut source, num_local_procs as usize)?;

        let body_len = source.read_u16()? as usize;
        let body = Deserializable::read_batch_from(&mut source, body_len)?;

        match Self::new(local_procs, body) {
            Err(err) => Err(DeserializationError::UnknownError(err.message().clone())),
            Ok(res) => Ok(res),
        }
    }

    // DESTRUCTURING
    // --------------------------------------------------------------------------------------------

    /// Returns local procedures and body nodes of this program.
    pub fn into_parts(self) -> (Vec<ProcedureAst>, Vec<Node>) {
        (self.local_procs, self.body)
    }
}

// LIBRARY MODULE AST
// ================================================================================================

/// An abstract syntax tree (AST) of a Miden code module.
///
/// A module AST consists of a list of procedure ASTs and module documentation. Procedures in the
/// list could be local or exported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleAst {
    docs: Option<String>,
    local_procs: Vec<ProcedureAst>,
}

impl ModuleAst {
    // AST
    // --------------------------------------------------------------------------------------------
    /// Constructs a [ModuleAst].
    ///
    /// A module consists of internal and exported procedures but does not contain a body.
    pub fn new(local_procs: Vec<ProcedureAst>, docs: Option<String>) -> Result<Self, ParsingError> {
        if local_procs.len() > MAX_LOCL_PROCS {
            return Err(ParsingError::too_many_module_procs(local_procs.len(), MAX_LOCL_PROCS));
        }
        if let Some(ref docs) = docs {
            if docs.len() > MAX_DOCS_LEN {
                return Err(ParsingError::module_docs_too_long(docs.len(), MAX_DOCS_LEN));
            }
        }
        Ok(Self { docs, local_procs })
    }

    // PARSER
    // --------------------------------------------------------------------------------------------
    /// Parses the provided source into a [ModuleAst].
    ///
    /// A module consists of internal and exported procedures but does not contain a body.
    pub fn parse(source: &str) -> Result<Self, ParsingError> {
        let mut tokens = TokenStream::new(source)?;

        let imports = parse_imports(&mut tokens)?;
        let local_constants = parse_constants(&mut tokens)?;
        let mut context = ParserContext {
            imports,
            local_constants,
            ..Default::default()
        };
        context.parse_procedures(&mut tokens, true)?;

        // make sure program body is absent and there are no more instructions.
        if let Some(token) = tokens.read() {
            if token.parts()[0] == Token::BEGIN {
                return Err(ParsingError::not_a_library_module(token));
            } else {
                return Err(ParsingError::dangling_ops_after_module(token));
            }
        }

        // get a list of local procs and make sure the number of procs is within the limit
        let local_procs = sort_procs_into_vec(context.local_procs);

        // get module docs and make sure the size is within the limit
        let docs = tokens.take_module_comments();

        Self::new(local_procs, docs)
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a list of procedures in this module.
    pub fn procs(&self) -> &[ProcedureAst] {
        &self.local_procs
    }

    /// Returns doc comments for this module.
    pub fn docs(&self) -> Option<&String> {
        self.docs.as_ref()
    }

    // SERIALIZATION / DESERIALIZATION
    // --------------------------------------------------------------------------------------------

    /// Returns byte representation of this [ModuleAst].
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut target = Vec::<u8>::default();
        self.write_into(&mut target);
        target
    }

    /// Returns a [ModuleAst] struct deserialized from the provided bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, DeserializationError> {
        let mut source = SliceReader::new(bytes);
        Self::read_from(&mut source)
    }
}

impl Serializable for ModuleAst {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        // asserts below are OK because we enforce limits on the number of procedure and length of
        // module docs in the module parser

        match &self.docs {
            Some(docs) => {
                assert!(docs.len() <= u16::MAX as usize, "docs too long");
                target.write_u16(docs.len() as u16);
                target.write_bytes(docs.as_bytes());
            }
            None => {
                target.write_u16(0);
            }
        }

        assert!(self.local_procs.len() <= u16::MAX as usize, "too many local procs");
        target.write_u16(self.local_procs.len() as u16);
        self.local_procs.write_into(target);
    }
}

impl Deserializable for ModuleAst {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let docs_len = source.read_u16()? as usize;
        let docs = if docs_len != 0 {
            let str = source.read_vec(docs_len)?;
            let str =
                from_utf8(&str).map_err(|e| DeserializationError::InvalidValue(e.to_string()))?;
            Some(str.to_string())
        } else {
            None
        };

        let num_local_procs = source.read_u16()? as usize;
        let local_procs = Deserializable::read_batch_from(source, num_local_procs)?;

        match Self::new(local_procs, docs) {
            Err(err) => Err(DeserializationError::UnknownError(err.message().clone())),
            Ok(res) => Ok(res),
        }
    }
}

// PROCEDURE AST
// ================================================================================================

/// An abstract syntax tree of a Miden procedure.
///
/// A procedure AST consists of a list of body nodes and additional metadata about the procedure
/// (e.g., procedure name, number of memory locals used by the procedure, and whether a procedure
/// is exported or internal).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcedureAst {
    pub name: ProcedureName,
    pub docs: Option<String>,
    pub num_locals: u16,
    pub body: Vec<Node>,
    pub is_export: bool,
}

impl ProcedureAst {
    // AST
    // --------------------------------------------------------------------------------------------
    /// Constructs a [ProcedureAst].
    ///
    /// A procedure consists of a name, a number of locals, a body, and a flag to signal whether the procedure is exported.
    pub fn new(
        name: ProcedureName,
        num_locals: u16,
        body: Vec<Node>,
        is_export: bool,
        docs: Option<String>,
    ) -> Self {
        Self {
            name,
            docs,
            num_locals,
            body,
            is_export,
        }
    }
}

impl Serializable for ProcedureAst {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        // asserts below are OK because we enforce limits on the procedure body size and length of
        // procedure docs in the procedure parser

        self.name.write_into(target);
        match &self.docs {
            Some(docs) => {
                assert!(docs.len() <= u16::MAX as usize, "docs too long");
                target.write_u16(docs.len() as u16);
                target.write_bytes(docs.as_bytes());
            }
            None => {
                target.write_u16(0);
            }
        }

        target.write_bool(self.is_export);
        target.write_u16(self.num_locals);
        assert!(self.body.len() <= u16::MAX as usize, "too many body instructions");
        target.write_u16(self.body.len() as u16);
        self.body.write_into(target);
    }
}

impl Deserializable for ProcedureAst {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let name = ProcedureName::read_from(source)?;
        let docs_len = source.read_u16()? as usize;
        let docs = if docs_len != 0 {
            let str = source.read_vec(docs_len)?;
            let str =
                from_utf8(&str).map_err(|e| DeserializationError::InvalidValue(e.to_string()))?;
            Some(str.to_string())
        } else {
            None
        };

        let is_export = source.read_bool()?;
        let num_locals = source.read_u16()?;
        let body_len = source.read_u16()? as usize;
        let body = Deserializable::read_batch_from(source, body_len)?;
        Ok(Self {
            name,
            num_locals,
            body,
            is_export,
            docs,
        })
    }
}

// PARSERS
// ================================================================================================

/// Parses all `use` statements into a map of imports which maps a module name (e.g., "u64") to
/// its fully-qualified path (e.g., "std::math::u64").
fn parse_imports(tokens: &mut TokenStream) -> Result<BTreeMap<String, LibraryPath>, ParsingError> {
    let mut imports = BTreeMap::<String, LibraryPath>::new();
    // read tokens from the token stream until all `use` tokens are consumed
    while let Some(token) = tokens.read() {
        match token.parts()[0] {
            Token::USE => {
                let module_path = token.parse_use()?;
                let module_name = module_path.last();
                if imports.contains_key(module_name) {
                    return Err(ParsingError::duplicate_module_import(token, &module_path));
                }

                imports.insert(module_name.to_string(), module_path);

                // consume the `use` token
                tokens.advance();
            }
            _ => break,
        }
    }

    Ok(imports)
}

/// Parses all `const` statements into a map which maps a const name to a value
fn parse_constants(tokens: &mut TokenStream) -> Result<LocalConstMap, ParsingError> {
    // instantiate new constant map for this module
    let mut constants = LocalConstMap::new();

    // iterate over tokens until we find a const declaration
    while let Some(token) = tokens.read() {
        match token.parts()[0] {
            Token::CONST => {
                let (name, value) = parse_constant(token)?;

                if constants.contains_key(&name) {
                    return Err(ParsingError::duplicate_const_name(token, &name));
                }

                constants.insert(name, value);
                tokens.advance();
            }
            _ => break,
        }
    }

    Ok(constants)
}

/// Parses a constant token and returns a (constant_name, constant_value) tuple
fn parse_constant(token: &Token) -> Result<(String, u64), ParsingError> {
    match token.num_parts() {
        0 => unreachable!(),
        1 => Err(ParsingError::missing_param(token)),
        2 => {
            let const_declaration: Vec<&str> = token.parts()[1].split('=').collect();
            match const_declaration.len() {
                0 => unreachable!(),
                1 => Err(ParsingError::missing_param(token)),
                2 => {
                    let name = CONSTANT_LABEL_PARSER
                        .parse_label(const_declaration[0])
                        .map_err(|err| ParsingError::invalid_const_name(token, err))?;
                    let value = parse_const_value(token, const_declaration[1])?;
                    Ok((name.to_string(), value))
                }
                _ => Err(ParsingError::extra_param(token)),
            }
        }
        _ => Err(ParsingError::extra_param(token)),
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Sort a map of procedures into a vec, respecting the order set in the map
fn sort_procs_into_vec(proc_map: LocalProcMap) -> Vec<ProcedureAst> {
    let mut procedures: Vec<_> = proc_map.into_values().collect();
    procedures.sort_by_key(|(idx, _proc)| *idx);

    procedures.into_iter().map(|(_idx, proc)| proc).collect()
}

/// Parses a param from the op token with the specified type and index. If the param is a constant
/// label, it will be looked up in the provided constant map.
fn parse_param_with_constant_lookup<R>(
    op: &Token,
    param_idx: usize,
    constants: &LocalConstMap,
) -> Result<R, ParsingError>
where
    R: TryFrom<u64> + core::str::FromStr,
{
    let param_str = op.parts()[param_idx];
    match CONSTANT_LABEL_PARSER.parse_label(param_str) {
        Ok(_) => {
            let constant = constants
                .get(param_str)
                .cloned()
                .ok_or_else(|| ParsingError::const_not_found(op))?;
            constant
                .try_into()
                .map_err(|_| ParsingError::const_conversion_failed(op, core::any::type_name::<R>()))
        }
        Err(_) => parse_param::<R>(op, param_idx),
    }
}

/// Parses a param from the op token with the specified type.
fn parse_param<I: core::str::FromStr>(op: &Token, param_idx: usize) -> Result<I, ParsingError> {
    let param_value = op.parts()[param_idx];

    let result = match param_value.parse::<I>() {
        Ok(i) => i,
        Err(_) => return Err(ParsingError::invalid_param(op, param_idx)),
    };

    Ok(result)
}

/// Parses a constant value and ensures it falls within bounds specified by the caller
fn parse_const_value(op: &Token, const_value: &str) -> Result<u64, ParsingError> {
    let result = const_value
        .parse::<u64>()
        .map_err(|err| ParsingError::invalid_const_value(op, const_value, &err.to_string()))?;

    let range = 0..Felt::MODULUS;
    range.contains(&result).then_some(result).ok_or_else(|| ParsingError::invalid_const_value(op, const_value, format!(
        "constant value must be greater than or equal to {lower_bound} and less than or equal to {upper_bound}", lower_bound = bound_into_included_u64(range.start_bound(), true),
        upper_bound = bound_into_included_u64(range.end_bound(), false)
    )
    .as_str(),))
}

/// Parses a param from the op token with the specified type and ensures that it falls within the
/// bounds specified by the caller.
fn parse_checked_param<I, R>(op: &Token, param_idx: usize, range: R) -> Result<I, ParsingError>
where
    I: core::str::FromStr + Ord + Clone + Into<u64> + Display,
    R: RangeBounds<I>,
{
    let param_value = op.parts()[param_idx];

    let result = match param_value.parse::<I>() {
        Ok(i) => i,
        Err(_) => return Err(ParsingError::invalid_param(op, param_idx)),
    };

    // check that the parameter is within the specified bounds
    range.contains(&result).then_some(result).ok_or_else(||
        ParsingError::invalid_param_with_reason(
            op,
            param_idx,
            format!(
                "parameter value must be greater than or equal to {lower_bound} and less than or equal to {upper_bound}", lower_bound = bound_into_included_u64(range.start_bound(), true),
                upper_bound = bound_into_included_u64(range.end_bound(), false)
            )
            .as_str(),
        )
    )
}

/// Returns an error if the passed in value is 0.
///
/// This is intended to be used when parsing instructions which need to perform division by
/// immediate value.
fn check_div_by_zero(value: u64, op: &Token, param_idx: usize) -> Result<(), ParsingError> {
    if value == 0 {
        Err(ParsingError::invalid_param_with_reason(op, param_idx, "division by zero"))
    } else {
        Ok(())
    }
}
