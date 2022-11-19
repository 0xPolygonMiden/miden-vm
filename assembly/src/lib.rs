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
