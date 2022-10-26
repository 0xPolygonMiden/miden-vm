#![allow(dead_code)]

use super::{AssemblyError, Token, TokenStream, Vec};
use crate::{errors::SerializationError, MODULE_PATH_DELIM};
use serde::{ByteReader, ByteWriter, Deserializable, Serializable};
use vm_core::utils::{collections::BTreeMap, string::String, string::ToString};

mod nodes;
use nodes::{Instruction, Node};

mod context;
use context::ParserContext;

mod io_ops;
mod serde;
mod stack_ops;
mod u32_ops;

#[cfg(test)]
pub mod tests;

const PROC_DIGEST_SIZE: usize = 24;

// TYPE ALIASES
// ================================================================================================
type ProcMap = BTreeMap<String, ProcedureAst>;

/// Represents a parsed program AST.
/// This AST can then be furthur processed to generate MAST.
/// A program has to have a body and no exported procedures.
#[derive(Debug, Eq, PartialEq)]
pub struct ProgramAst {
    pub procedures: ProcMap,
    pub body: Vec<Node>,
}

impl ProgramAst {
    /// Returns byte representation of the `ProgramAst`.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut byte_writer = ByteWriter::new();

        // procedures
        byte_writer.write_u16(self.procedures.len() as u16);
        for (key, value) in self.procedures.iter() {
            byte_writer
                .write_string(key)
                .expect("String serialization failure");
            value.write_into(&mut byte_writer);
        }

        // body
        self.body.write_into(&mut byte_writer);

        byte_writer.into_bytes()
    }

    /// Returns a `ProgramAst` struct by its byte representation.
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, SerializationError> {
        let mut byte_reader = ByteReader::new(bytes.to_vec());

        let mut procedures = ProcMap::new();

        let num_procedures = byte_reader.read_u16()?;
        for _ in 0..num_procedures {
            let proc_name = byte_reader.read_string()?;
            let proc_ast = ProcedureAst::read_from(&mut byte_reader)?;

            procedures.insert(proc_name, proc_ast);
        }

        let body = Deserializable::read_from(&mut byte_reader)?;

        Ok(ProgramAst { procedures, body })
    }
}

/// Represents a parsed module AST.
/// A module can only have exported and local procedures, but no body.
#[derive(Debug, Eq, PartialEq)]
pub struct ModuleAst {
    pub procedures: ProcMap,
}

impl ModuleAst {
    /// Returns byte representation of the `ModuleAst.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut module_ast_bytes = ByteWriter::new();

        // procedures
        module_ast_bytes.write_u16(self.procedures.len() as u16);
        for (key, value) in self.procedures.iter() {
            module_ast_bytes
                .write_string(key)
                .expect("String serialization failure");
            value.write_into(&mut module_ast_bytes);
        }

        module_ast_bytes.into_bytes()
    }

    /// Returns a `ModuleAst` struct by its byte representation.
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, SerializationError> {
        let mut byte_reader = ByteReader::new(bytes.to_vec());

        let mut procedures = ProcMap::new();

        let procedures_len = byte_reader.read_u16()?;

        for _ in 0..procedures_len {
            let proc_name = byte_reader.read_string()?;
            let proc_ast = ProcedureAst::read_from(&mut byte_reader)?;

            procedures.insert(proc_name, proc_ast);
        }

        Ok(ModuleAst { procedures })
    }
}

/// Procedure holds information about a defnied procedure in a Miden program.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProcedureAst {
    pub name: String,
    pub num_locals: u32,
    pub body: Vec<Node>,
    pub is_export: bool,
    pub index: u32,
}

impl Serializable for ProcedureAst {
    /// Writes byte representation of the `ProcedureAst` into the provided `ByteWriter` struct.
    fn write_into(&self, target: &mut ByteWriter) {
        target.write_u16(self.index as u16);
        target
            .write_string(&self.name)
            .expect("String serialization failure");
        target.write_bool(self.is_export);
        target.write_u16(self.num_locals as u16);
        self.body.write_into(target);
    }
}

impl Deserializable for ProcedureAst {
    /// Returns a `ProcedureAst` from its byte representation stored in provided `ByteReader` struct.
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        let index = bytes.read_u16()?.into();
        let name = bytes.read_string()?;
        let is_export = bytes.read_bool()?;
        let num_locals = bytes.read_u16()?.into();
        let body = Deserializable::read_from(bytes)?;
        Ok(ProcedureAst {
            name,
            num_locals,
            body,
            is_export,
            index,
        })
    }
}

// PARSERS
// ================================================================================================

/// Parses the provided source into a AST Program. This Program holds information
/// that can be directly translated into MAST. A Program cannot have any exported procedures.
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
        context.parse_body(&mut tokens, &mut body)?;
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

    let program = ProgramAst {
        body,
        procedures: context.procedures,
    };

    Ok(program)
}

/// Parses the provided source into a AST Module. This AST Module holds information
/// that can be directly translated into MAST. A Module cannot contain any body.
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
        procedures: context.procedures,
    };

    Ok(module)
}

/// Parses the token streams into AST nodes
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

// UTILITY FUNCTIONS
// ==================================================================================

/// Parses a param from the op token with the specified type.
fn parse_param<I: core::str::FromStr>(op: &Token, param_idx: usize) -> Result<I, AssemblyError> {
    let param_value = op.parts()[param_idx];

    let result = match param_value.parse::<I>() {
        Ok(i) => i,
        Err(_) => return Err(AssemblyError::invalid_param(op, param_idx)),
    };

    Ok(result)
}
