use super::{
    AbsolutePath, BTreeMap, ByteReader, ByteWriter, Deserializable, Felt, LabelError, ParsingError,
    ProcedureId, ProcedureName, Serializable, SerializationError, StarkField, String, ToString,
    Token, TokenStream, Vec, MAX_LABEL_LEN,
};
use core::{fmt::Display, ops::RangeBounds};

mod nodes;
use crate::utils::bound_into_included_u64;
pub(crate) use nodes::{Instruction, Node};
mod context;
use context::ParserContext;
mod labels;
use labels::CONSTANT_LABEL_PARSER;
pub use labels::PROCEDURE_LABEL_PARSER;

mod adv_ops;
mod field_ops;
mod io_ops;
mod serde;
mod stack_ops;
mod u32_ops;

#[cfg(test)]
pub mod tests;

// TYPE ALIASES
// ================================================================================================
type LocalProcMap = BTreeMap<String, (u16, ProcedureAst)>;
type LocalConstMap = BTreeMap<String, u64>;

// ABSTRACT SYNTAX TREE STRUCTS
// ================================================================================================

/// An abstract syntax tree (AST) of a Miden program.
///
/// A program AST consists of a list of internal procedure ASTs and a list of body nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramAst {
    pub local_procs: Vec<ProcedureAst>,
    pub body: Vec<Node>,
}

impl ProgramAst {
    /// Returns byte representation of the `ProgramAst`.
    pub fn to_bytes(&self) -> Result<Vec<u8>, SerializationError> {
        let mut byte_writer = ByteWriter::default();

        // local procedures
        byte_writer.write_u16(self.local_procs.len() as u16);

        self.local_procs.iter().try_for_each(|proc| proc.write_into(&mut byte_writer))?;

        // body
        self.body.write_into(&mut byte_writer)?;

        Ok(byte_writer.into_bytes())
    }

    /// Returns a `ProgramAst` struct by its byte representation.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SerializationError> {
        let mut byte_reader = ByteReader::new(bytes);

        let num_local_procs = byte_reader.read_u16()?;

        let local_procs = (0..num_local_procs)
            .map(|_| ProcedureAst::read_from(&mut byte_reader))
            .collect::<Result<_, _>>()?;

        let body = Deserializable::read_from(&mut byte_reader)?;

        Ok(ProgramAst { local_procs, body })
    }
}

/// An abstract syntax tree (AST) of a Miden code module.
///
/// A module AST consists of a list of procedure ASTs and module documentation. Procedures in the
/// list could be local or exported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleAst {
    pub docs: Option<String>,
    pub local_procs: Vec<ProcedureAst>,
}

impl ModuleAst {
    /// Returns byte representation of the `ModuleAst`.
    pub fn to_bytes(&self) -> Result<Vec<u8>, SerializationError> {
        let mut byte_writer = ByteWriter::default();

        // docs
        self.docs.write_into(&mut byte_writer)?;

        // local procedures
        self.local_procs.write_into(&mut byte_writer)?;

        Ok(byte_writer.into_bytes())
    }

    /// Returns a `ModuleAst` struct by its byte representation.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SerializationError> {
        let mut byte_reader = ByteReader::new(bytes);

        // docs
        let docs = Deserializable::read_from(&mut byte_reader)?;

        // local procedures
        let local_procs = Deserializable::read_from(&mut byte_reader)?;

        Ok(ModuleAst { docs, local_procs })
    }
}

impl Serializable for ModuleAst {
    fn write_into(&self, target: &mut ByteWriter) -> Result<(), SerializationError> {
        self.docs.write_into(target)?;
        self.local_procs.write_into(target)?;
        Ok(())
    }
}

impl Deserializable for ModuleAst {
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        let docs = Deserializable::read_from(bytes)?;
        let local_procs = Deserializable::read_from(bytes)?;
        Ok(Self { docs, local_procs })
    }
}

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

impl Serializable for ProcedureAst {
    /// Writes byte representation of the `ProcedureAst` into the provided `ByteWriter` struct.
    fn write_into(&self, target: &mut ByteWriter) -> Result<(), SerializationError> {
        self.name.write_into(target)?;
        self.docs.write_into(target)?;
        target.write_bool(self.is_export);
        target.write_u16(self.num_locals);
        self.body.write_into(target)?;
        Ok(())
    }
}

impl Deserializable for ProcedureAst {
    /// Returns a `ProcedureAst` from its byte representation stored in provided `ByteReader` struct.
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        let name = ProcedureName::read_from(bytes)?;
        let docs = Deserializable::read_from(bytes)?;
        let is_export = bytes.read_bool()?;
        let num_locals = bytes.read_u16()?;
        let body = Deserializable::read_from(bytes)?;
        Ok(ProcedureAst {
            name,
            docs,
            num_locals,
            body,
            is_export,
        })
    }
}

// PARSERS
// ================================================================================================

/// Parses the provided source into a program AST. A program consist of a body and a set of
/// internal (i.e., not exported) procedures.
pub fn parse_program(source: &str) -> Result<ProgramAst, ParsingError> {
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
    let next_token = tokens.read().ok_or_else(|| ParsingError::unexpected_eof(tokens.pos()))?;
    if next_token.parts()[0] != Token::BEGIN {
        return Err(ParsingError::unexpected_token(next_token, Token::BEGIN));
    }

    let program_start = tokens.pos();
    // consume the 'begin' token
    let header = tokens.read().expect("missing program header");
    header.validate_begin()?;
    tokens.advance();

    // make sure there is something to be read
    let start_pos = tokens.pos();
    if tokens.eof() {
        return Err(ParsingError::unexpected_eof(start_pos));
    }

    let mut body = Vec::<Node>::new();

    // parse the sequence of nodes and add each node to the list
    let mut end_of_nodes = false;
    while !end_of_nodes {
        let node_count = body.len();
        context.parse_body(&mut tokens, &mut body, false)?;
        end_of_nodes = body.len() == node_count;
    }

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

    let program = ProgramAst { body, local_procs };

    Ok(program)
}

/// Parses the provided source into a module ST. A module consists of internal and exported
/// procedures but does not contain a body.
pub fn parse_module(source: &str) -> Result<ModuleAst, ParsingError> {
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

    let module = ModuleAst {
        docs: tokens.take_module_comments(),
        local_procs: sort_procs_into_vec(context.local_procs),
    };

    Ok(module)
}

/// Parses all `use` statements into a map of imports which maps a module name (e.g., "u64") to
/// its fully-qualified path (e.g., "std::math::u64").
fn parse_imports(tokens: &mut TokenStream) -> Result<BTreeMap<String, AbsolutePath>, ParsingError> {
    let mut imports = BTreeMap::<String, AbsolutePath>::new();
    // read tokens from the token stream until all `use` tokens are consumed
    while let Some(token) = tokens.read() {
        match token.parts()[0] {
            Token::USE => {
                let module_path = token.parse_use()?;
                let module_name = module_path.label();
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

// HELPER FUNCTIONS
// ================================================================================================

/// Sort a map of procedures into a vec, respecting the order set in the map
fn sort_procs_into_vec(proc_map: LocalProcMap) -> Vec<ProcedureAst> {
    let mut procedures: Vec<_> = proc_map.into_values().collect();
    procedures.sort_by_key(|(idx, _proc)| *idx);

    procedures.into_iter().map(|(_idx, proc)| proc).collect()
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

/// parses a constant token and returns a (constant_name, constant_value) tuple
pub fn parse_constant(token: &Token) -> Result<(String, u64), ParsingError> {
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
                        .parse_label(const_declaration[0].to_string())
                        .map_err(|err| ParsingError::invalid_const_name(token, err))?;
                    let value = parse_const_value(token, const_declaration[1])?;
                    Ok((name, value))
                }
                _ => Err(ParsingError::extra_param(token)),
            }
        }
        _ => Err(ParsingError::extra_param(token)),
    }
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
