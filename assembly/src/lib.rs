#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

use vm_core::{
    code_blocks::CodeBlock,
    utils::{
        collections::{BTreeMap, BTreeSet, Vec},
        string::{String, ToString},
        Box,
    },
    CodeBlockTable, Kernel, Program,
};

mod procedures;
pub use procedures::ProcedureId;
use procedures::{CallSet, Procedure};

mod parsers;
pub use parsers::{parse_module, ModuleAst, NamedModuleAst, ProcedureAst};

mod tokens;
use tokens::{Token, TokenStream};

mod errors;
pub use errors::{AssemblerError, AssemblyError};

mod assembler;
pub use assembler::Assembler;

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

const MODULE_PATH_DELIM: &str = "::";

/// The maximum number of constant inputs allowed for the `push` instruction.
const MAX_PUSH_INPUTS: usize = 16;

/// The maximum number of elements that can be read from the advice tape in a single `adv_push`
/// instruction.
const ADVICE_READ_LIMIT: u8 = 16;

/// The maximum number of bits by which a u32 value can be shifted in a bitwise operation.
const MAX_U32_SHIFT_VALUE: u8 = 31;

/// The maximum number of bits by which a u32 value can be rotated in a bitwise operation.
const MAX_U32_ROTATE_VALUE: u8 = 31;

/// The maximum number of bits allowed for the exponent parameter for exponentiation instructions.
const MAX_EXP_BITS: u8 = 64;

// MODULE PROVIDER
// ================================================================================================

/// The module provider is now a simplified version of a module cache. It is expected to evolve to
/// a general solution for the module lookup
pub trait ModuleProvider {
    /// Fetch source contents provided a module path
    fn get_source(&self, path: &str) -> Option<&str>;

    /// Fetch a module AST from its ID
    fn get_module(&self, id: &ProcedureId) -> Option<NamedModuleAst<'_>>;
}

// A default provider that won't resolve modules
impl ModuleProvider for () {
    fn get_source(&self, _path: &str) -> Option<&str> {
        None
    }

    fn get_module(&self, _id: &ProcedureId) -> Option<NamedModuleAst<'_>> {
        None
    }
}
