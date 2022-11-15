use super::{AssemblyError, Token, TokenStream, Vec};
use crate::{errors::SerializationError, ProcedureId, MODULE_PATH_DELIM};
use core::{fmt::Display, ops::Deref};
use serde::{ByteReader, ByteWriter, Deserializable, Serializable};
use vm_core::utils::{collections::BTreeMap, string::String, string::ToString};

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

#[allow(dead_code)]
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
/// A module AST consists of a list of procedure ASTs. These procedures could be local or exported.
#[derive(Debug, Eq, PartialEq)]
pub struct ModuleAst {
    pub local_procs: Vec<ProcedureAst>,
}

impl ModuleAst {
    /// Returns byte representation of the `ModuleAst.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut byte_writer = ByteWriter::new();

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

        let local_procs_len = byte_reader.read_u16()?;

        let local_procs = (0..local_procs_len)
            .map(|_| ProcedureAst::read_from(&mut byte_reader))
            .collect::<Result<_, _>>()?;

        Ok(ModuleAst { local_procs })
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
    pub num_locals: u32,
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
        target.write_u16(self.num_locals as u16);
        self.body.write_into(target);
    }
}

impl Deserializable for ProcedureAst {
    /// Returns a `ProcedureAst` from its byte representation stored in provided `ByteReader` struct.
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        let name = bytes.read_proc_name()?;
        let docs = bytes.read_docs()?;
        let is_export = bytes.read_bool()?;
        let num_locals = bytes.read_u16()?.into();
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
pub fn parse_program(source: &str) -> Result<ProgramAst, AssemblyError> {
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
        .ok_or_else(|| AssemblyError::unexpected_eof(tokens.pos()))?;
    if next_token.parts()[0] != Token::BEGIN {
        return Err(AssemblyError::unexpected_token(next_token, Token::BEGIN));
    }

    let program_start = tokens.pos();
    // consume the 'begin' token
    let header = tokens.read().expect("missing program header");
    header.validate_begin()?;
    tokens.advance();

    // make sure there is something to be read
    let start_pos = tokens.pos();
    if tokens.eof() {
        return Err(AssemblyError::unexpected_eof(start_pos));
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
        return Err(AssemblyError::empty_block(start_op));
    }

    // consume the 'end' token
    match tokens.read() {
        None => Err(AssemblyError::unmatched_begin(
            tokens.read_at(program_start).expect("no begin token"),
        )),
        Some(token) => match token.parts()[0] {
            Token::END => token.validate_end(),
            Token::ELSE => Err(AssemblyError::dangling_else(token)),
            _ => Err(AssemblyError::unmatched_begin(
                tokens.read_at(program_start).expect("no begin token"),
            )),
        },
    }?;
    tokens.advance();

    // make sure there are no instructions after the end
    if let Some(token) = tokens.read() {
        return Err(AssemblyError::dangling_ops_after_program(token));
    }

    let local_procs = sort_procs_into_vec(context.local_procs);

    let program = ProgramAst { body, local_procs };

    Ok(program)
}

/// Parses the provided source into a module ST. A module consists of internal and exported
/// procedures but does not contain a body.
pub fn parse_module(source: &str) -> Result<ModuleAst, AssemblyError> {
    let mut tokens = TokenStream::new(source)?;

    let imports = parse_imports(&mut tokens)?;
    let mut context = ParserContext {
        imports,
        ..Default::default()
    };
    context.parse_procedures(&mut tokens, true)?;

    // make sure program body is absent and no more instructions.
    if tokens.read().is_some() {
        return Err(AssemblyError::unexpected_eof(tokens.pos()));
    }

    let module = ModuleAst {
        local_procs: sort_procs_into_vec(context.local_procs),
    };

    Ok(module)
}

/// Parses all `use` statements into a map of imports which maps a module name (e.g., "u64") to
/// its fully-qualified path (e.g., "std::math::u64").
fn parse_imports(tokens: &mut TokenStream) -> Result<BTreeMap<String, String>, AssemblyError> {
    let mut imports = BTreeMap::<String, String>::new();
    // read tokens from the token stream until all `use` tokens are consumed
    while let Some(token) = tokens.read() {
        match token.parts()[0] {
            Token::USE => {
                let module_path = &token.parse_use()?;
                let (_, short_name) = module_path.rsplit_once(MODULE_PATH_DELIM).unwrap();
                if imports.contains_key(short_name) {
                    return Err(AssemblyError::duplicate_module_import(token, module_path));
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

/// Parses a param from the op token with the specified type.
fn parse_param<I: core::str::FromStr>(op: &Token, param_idx: usize) -> Result<I, AssemblyError> {
    let param_value = op.parts()[param_idx];

    let result = match param_value.parse::<I>() {
        Ok(i) => i,
        Err(_) => return Err(AssemblyError::invalid_param(op, param_idx)),
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
) -> Result<I, AssemblyError> {
    let param_value = op.parts()[param_idx];

    let result = match param_value.parse::<I>() {
        Ok(i) => i,
        Err(_) => return Err(AssemblyError::invalid_param(op, param_idx)),
    };

    // check that the parameter is within the specified bounds
    if result < lower_bound || result > upper_bound {
        return Err(AssemblyError::invalid_param_with_reason(
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

/// Sort a map of procedures into a vec, respecting the order set in the map
fn sort_procs_into_vec(proc_map: LocalProcMap) -> Vec<ProcedureAst> {
    let mut procedures: Vec<_> = proc_map.into_values().collect();
    procedures.sort_by_key(|(idx, _proc)| *idx);

    procedures.into_iter().map(|(_idx, proc)| proc).collect()
}
