use super::{
    errors::SerializationError, BTreeMap, Felt, ParsingError, ProcedureId, StarkField, String,
    ToString, Token, TokenStream, Vec, MODULE_PATH_DELIM,
};
use core::{fmt::Display, ops::Deref};
use serde::{ByteReader, ByteWriter, Deserializable, Serializable};

mod nodes;
pub(crate) use nodes::{Instruction, Node};

mod context;
use context::ParserContext;

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

// ABSTRACT SYNTAX TREE STRUCTS
// ================================================================================================

/// An abstract syntax tree (AST) of a Miden program.
///
/// A program AST consists of a list of internal procedure ASTs and a list of body nodes.
#[derive(Debug, Eq, PartialEq)]
pub struct ProgramAst {
    pub local_procs: Vec<ProcedureAst>,
    pub body: Vec<Node>,
}

impl ProgramAst {
    /// Returns byte representation of the `ProgramAst`.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut byte_writer = ByteWriter::new();

        // local procedures
        byte_writer.write_u16(self.local_procs.len() as u16);

        self.local_procs
            .iter()
            .for_each(|proc| proc.write_into(&mut byte_writer));

        // body
        self.body.write_into(&mut byte_writer);

        byte_writer.into_bytes()
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
#[derive(Debug, Eq, PartialEq)]
pub struct ModuleAst {
    pub docs: Option<String>,
    pub local_procs: Vec<ProcedureAst>,
}

impl ModuleAst {
    /// Returns byte representation of the `ModuleAst.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut byte_writer = ByteWriter::new();

        // docs
        byte_writer
            .write_docs(&self.docs)
            .expect("Docs serialization failure");

        // local procedures
        byte_writer.write_u16(self.local_procs.len() as u16);

        self.local_procs
            .iter()
            .for_each(|proc| proc.write_into(&mut byte_writer));

        byte_writer.into_bytes()
    }

    /// Returns a `ModuleAst` struct by its byte representation.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SerializationError> {
        let mut byte_reader = ByteReader::new(bytes);

        // docs
        let docs = byte_reader.read_docs()?;

        // local procedures
        let local_procs_len = byte_reader.read_u16()?;

        let local_procs = (0..local_procs_len)
            .map(|_| ProcedureAst::read_from(&mut byte_reader))
            .collect::<Result<_, _>>()?;

        Ok(ModuleAst { docs, local_procs })
    }

    /// Return a named reference of the module, binding it to an arbitrary path
    pub fn named_ref<N>(&self, path: N) -> NamedModuleAst<'_>
    where
        N: Into<String>,
    {
        NamedModuleAst {
            path: path.into(),
            module: self,
        }
    }
}

/// A reference to a module AST with its name under the provider context (i.e. stdlib).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedModuleAst<'a> {
    // Note: the path is taken as owned string to not leak unnecessary coupling between the path
    // provider and the module owner.
    path: String,
    module: &'a ModuleAst,
}

impl<'a> Deref for NamedModuleAst<'a> {
    type Target = ModuleAst;

    fn deref(&self) -> &Self::Target {
        self.module
    }
}

impl<'a> NamedModuleAst<'a> {
    /// Create a new named module
    pub fn new<P>(path: P, module: &'a ModuleAst) -> Self
    where
        P: Into<String>,
    {
        Self {
            path: path.into(),
            module,
        }
    }

    /// Full path of the module used to compute the proc id
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Full path of a procedure with the given name.
    pub fn label<N>(&self, name: N) -> String
    where
        N: AsRef<str>,
    {
        format!("{}{MODULE_PATH_DELIM}{}", self.path, name.as_ref())
    }

    /// Computed procedure id using as base the full path of the module
    pub fn procedure_id<N>(&self, name: N) -> ProcedureId
    where
        N: AsRef<str>,
    {
        ProcedureId::new(self.label(name))
    }

    pub fn get_procedure(&self, id: &ProcedureId) -> Option<&ProcedureAst> {
        // TODO this should be cached so we don't have to scan every request
        self.module
            .local_procs
            .iter()
            .find(|proc| &self.procedure_id(&proc.name) == id)
    }
}

/// An abstract syntax tree of a Miden procedure.
///
/// A procedure AST consists of a list of body nodes and additional metadata about the procedure
/// (e.g., procedure name, number of memory locals used by the procedure, and whether a procedure
/// is exported or internal).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProcedureAst {
    pub name: String,
    pub docs: Option<String>,
    pub num_locals: u16,
    pub body: Vec<Node>,
    pub is_export: bool,
}

impl Serializable for ProcedureAst {
    /// Writes byte representation of the `ProcedureAst` into the provided `ByteWriter` struct.
    fn write_into(&self, target: &mut ByteWriter) {
        target
            .write_proc_name(&self.name)
            .expect("String serialization failure");
        target
            .write_docs(&self.docs)
            .expect("Docs serialization failure");
        target.write_bool(self.is_export);
        target.write_u16(self.num_locals);
        self.body.write_into(target);
    }
}

impl Deserializable for ProcedureAst {
    /// Returns a `ProcedureAst` from its byte representation stored in provided `ByteReader` struct.
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        let name = bytes.read_proc_name()?;
        let docs = bytes.read_docs()?;
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

    let mut context = ParserContext {
        imports,
        ..Default::default()
    };

    context.parse_procedures(&mut tokens, false)?;

    // make sure program body is present
    let next_token = tokens
        .read()
        .ok_or_else(|| ParsingError::unexpected_eof(tokens.pos()))?;
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
    let beginning_node_count = body.len();
    while !end_of_nodes {
        let node_count = body.len();
        context.parse_body(&mut tokens, &mut body, false)?;
        end_of_nodes = body.len() == node_count;
    }

    // make sure at least one block has been read
    if body.len() == beginning_node_count {
        let start_op = tokens.read_at(start_pos).expect("no start token");
        return Err(ParsingError::empty_block(start_op));
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
    let mut context = ParserContext {
        imports,
        ..Default::default()
    };
    context.parse_procedures(&mut tokens, true)?;

    // make sure program body is absent and no more instructions.
    if tokens.read().is_some() {
        return Err(ParsingError::unexpected_eof(tokens.pos()));
    }

    let module = ModuleAst {
        docs: tokens.take_module_comments(),
        local_procs: sort_procs_into_vec(context.local_procs),
    };

    Ok(module)
}

/// Parses all `use` statements into a map of imports which maps a module name (e.g., "u64") to
/// its fully-qualified path (e.g., "std::math::u64").
fn parse_imports(tokens: &mut TokenStream) -> Result<BTreeMap<String, String>, ParsingError> {
    let mut imports = BTreeMap::<String, String>::new();
    // read tokens from the token stream until all `use` tokens are consumed
    while let Some(token) = tokens.read() {
        match token.parts()[0] {
            Token::USE => {
                let module_path = &token.parse_use()?;
                let (_, short_name) = module_path.rsplit_once(MODULE_PATH_DELIM).unwrap();
                if imports.contains_key(short_name) {
                    return Err(ParsingError::duplicate_module_import(token, module_path));
                }

                imports.insert(short_name.to_string(), module_path.to_string());

                // consume the `use` token
                tokens.advance();
            }
            _ => break,
        }
    }

    Ok(imports)
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

/// Parses a param from the op token with the specified type and ensures that it falls within the
/// bounds specified by the caller.
fn parse_checked_param<I: core::str::FromStr + Ord + Display>(
    op: &Token,
    param_idx: usize,
    lower_bound: I,
    upper_bound: I,
) -> Result<I, ParsingError> {
    let param_value = op.parts()[param_idx];

    let result = match param_value.parse::<I>() {
        Ok(i) => i,
        Err(_) => return Err(ParsingError::invalid_param(op, param_idx)),
    };

    // check that the parameter is within the specified bounds
    if result < lower_bound || result > upper_bound {
        return Err(ParsingError::invalid_param_with_reason(
            op,
            param_idx,
            format!(
                "parameter value must be greater than or equal to {} and less than or equal to {}",
                lower_bound, upper_bound
            )
            .as_str(),
        ));
    }

    Ok(result)
}

/// Parses a single parameter into a valid field element.
fn parse_element_param(op: &Token, param_idx: usize) -> Result<Felt, ParsingError> {
    // make sure that the parameter value is available
    if op.num_parts() <= param_idx {
        return Err(ParsingError::missing_param(op));
    }
    let param_value = op.parts()[param_idx];

    if let Some(param_value) = param_value.strip_prefix("0x") {
        // parse hexadecimal number
        parse_hex_param(op, param_idx, param_value)
    } else {
        // parse decimal number
        parse_decimal_param(op, param_idx, param_value)
    }
}

/// Parses a decimal parameter value into valid a field element.
fn parse_decimal_param(
    op: &Token,
    param_idx: usize,
    param_str: &str,
) -> Result<Felt, ParsingError> {
    match param_str.parse::<u64>() {
        Ok(value) => get_valid_felt(op, param_idx, value),
        Err(_) => Err(ParsingError::invalid_param(op, param_idx)),
    }
}

/// Parses a hexadecimal parameter value into a valid field element.
fn parse_hex_param(op: &Token, param_idx: usize, param_str: &str) -> Result<Felt, ParsingError> {
    match u64::from_str_radix(param_str, 16) {
        Ok(value) => get_valid_felt(op, param_idx, value),
        Err(_) => Err(ParsingError::invalid_param(op, param_idx)),
    }
}

/// Checks that the u64 parameter value is a valid field element value and returns it as a field
/// element.
fn get_valid_felt(op: &Token, param_idx: usize, param: u64) -> Result<Felt, ParsingError> {
    if param >= Felt::MODULUS {
        return Err(ParsingError::invalid_param_with_reason(
            op,
            param_idx,
            format!("parameter value must be smaller than {}", Felt::MODULUS).as_str(),
        ));
    }

    Ok(Felt::new(param))
}

/// Returns an error if the passed in value is 0.
///
/// This is intended to be used when parsing instructions which need to perform division by
/// immediate value.
fn check_div_by_zero(value: u64, op: &Token, param_idx: usize) -> Result<(), ParsingError> {
    if value == 0 {
        Err(ParsingError::invalid_param_with_reason(
            op,
            param_idx,
            "division by zero",
        ))
    } else {
        Ok(())
    }
}

/// Validates an op Token against a provided instruction string and/or an expected number of
/// parameter inputs and returns an appropriate ParsingError if the operation Token is invalid.
///
/// * To fully validate an operation, pass all of the following:
/// - the parsed operation Token
/// - a string describing a valid instruction, with variants separated by '|' and parameters
///   excluded.
/// - an integer or range for the number of allowed parameters
/// This will attempt to fully validate the operation, so a full-length instruction must be
/// described. For example, `popw.mem` accepts 0 or 1 inputs and can be validated by:
/// ```validate_operation!(op_token, "popw.mem", 0..1)```
///
/// * To validate only the operation parameters, specify @only_params before passing the same inputs
/// used for full validation (above). This will skip validating each part of the instruction.
/// For example, to validate only the parameters of `popw.mem` use:
/// ```validate_operation!(@only_params op_token, "popw.mem", 0..1)```
///
/// * To validate only the instruction portion of the operation, exclude the specification for the
/// number of parameters. This will only validate up to the number of parts in the provided
/// instruction string. For example, `pop.local` and `pop.mem` are the two valid instruction
/// variants for `pop`, so the first 2 parts of `pop` can be validated by:
/// ```validate_operation!(op_token, "pop.local|mem")```
/// or the first part can be validated by:
/// ```validate_operation!(op_token, "pop")```
#[macro_export]
macro_rules! validate_operation {
    // validate that the number of parameters is within the allowed range
    (@only_params $token:expr, $instr:literal, $min_params:literal..$max_params:expr ) => {
        let num_parts = $token.num_parts();
        let num_instr_parts = $instr.split(".").count();

        // token has too few parts to contain the required parameters
        if num_parts < num_instr_parts + $min_params {
            return Err(ParsingError::missing_param($token));
        }
        // token has more than the maximum number of parts
        if num_parts > num_instr_parts + $max_params {
            return Err(ParsingError::extra_param($token));
        }
    };
    // validate the exact number of parameters
    (@only_params $token:expr, $instr:literal, $num_params:literal) => {
        validate_operation!(@only_params $token, $instr, $num_params..$num_params);
    };

    // validate the instruction string and an optional parameter range
    ($token:expr, $instr:literal $(, $min_params:literal..$max_params:expr)?) => {
        // split the expected instruction into a vector of parts
        let instr_parts: Vec<Vec<&str>> = $instr
            .split(".")
            .map(|part| part.split("|").collect())
            .collect();

        let num_parts = $token.num_parts();
        let num_instr_parts = instr_parts.len();

        // token has too few parts to contain the full instruction
        if num_parts < num_instr_parts {
            return Err(ParsingError::invalid_op($token));
        }

        // compare the parts to make sure they match
        for (part_variants, token_part) in instr_parts.iter().zip($token.parts()) {
            if !part_variants.contains(token_part) {
                return Err(ParsingError::unexpected_token($token, $instr));
            }
        }

        $(
            // validate the parameter range, if provided
            validate_operation!(@only_params $token, $instr, $min_params..$max_params);
        )?
    };
    // validate the instruction string and an exact number of parameters
    ($token:expr, $instr:literal, $num_params:literal) => {
        validate_operation!($token, $instr, $num_params..$num_params);
    };
}
