#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

use core::{cmp::Ordering, ops::Deref};
use vm_core::{
    code_blocks::CodeBlock,
    utils::{
        collections::{BTreeMap, BTreeSet, Vec},
        string::{String, ToString},
    },
    CodeBlockTable, Felt, Kernel, Operation, Program, StarkField, ONE, ZERO,
};

mod procedures;
use procedures::{CallSet, Procedure};
pub use procedures::{ProcedureId, ProcedureName};

mod parsers;
pub use parsers::{parse_module, parse_program, ModuleAst, ProcedureAst, ProgramAst};

mod tokens;
use tokens::{Token, TokenStream};

mod errors;
pub use errors::{AssemblyError, LibraryError, ParsingError};

mod assembler;
pub use assembler::Assembler;

#[cfg(test)]
mod tests;

// RE-EXPORTS
// ================================================================================================

pub use vm_core::utils;

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

/// The maximum length of a procedure's name.
const MAX_PROC_NAME_LEN: u8 = 100;

/// The required length of the hexadecimal representation for an input value when more than one hex
/// input is provided to `push` masm operation without period separators.
const HEX_CHUNK_SIZE: usize = 16;

// TYPE-SAFE PATHS
// ================================================================================================

/// Absolute path of a module or a procedure.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AbsolutePath {
    path: String,
}

impl AbsolutePath {
    // CONSTANTS
    // --------------------------------------------------------------------------------------------

    /// Base kernel path
    // TODO better use `MODULE_PATH_DELIM`. maybe require `const_format` crate?
    pub const KERNEL_PATH: &str = "::sys";

    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Create a new absolute path without checking its integrity.
    pub(crate) fn new_unchecked(path: String) -> Self {
        Self { path }
    }

    pub fn kernel_path() -> Self {
        Self {
            path: Self::KERNEL_PATH.into(),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Label of the path.
    ///
    /// Will be the rightmost token separated by [`MODULE_PATH_DELIM`].
    pub fn label(&self) -> &str {
        self.path
            .rsplit_once(MODULE_PATH_DELIM)
            .expect("a valid absolute path should always have a namespace separator")
            .1
    }

    /// Namespace of the path.
    ///
    /// Will be the leftmost token separated by [`MODULE_PATH_DELIM`].
    pub fn namespace(&self) -> &str {
        self.path
            .split_once(MODULE_PATH_DELIM)
            .expect("a valid absolute path should always have a namespace separator")
            .0
    }
}

impl From<&AbsolutePath> for ProcedureId {
    fn from(path: &AbsolutePath) -> Self {
        ProcedureId::new(path)
    }
}

impl Deref for AbsolutePath {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl AsRef<str> for AbsolutePath {
    fn as_ref(&self) -> &str {
        &self.path
    }
}

/// Library namespace.
///
/// Will be `std` in the absolute procedure name `std::foo::bar::baz`.
///
/// # Type-safety
///
/// It is achieved as any instance of this type can be created only via the checked
/// [`Self::try_from`]. A valid library namespace cannot contain a [`MODULE_PATH_DELIM`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LibraryNamespace {
    name: String,
}

impl TryFrom<String> for LibraryNamespace {
    type Error = LibraryError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        if name.contains(MODULE_PATH_DELIM) {
            return Err(LibraryError::library_name_with_delimiter(&name));
        }
        Ok(Self { name })
    }
}

impl Deref for LibraryNamespace {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

impl AsRef<str> for LibraryNamespace {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

/// Module path relative to a namespace.
///
/// Will be `foo::bar` in the absolute procedure name `std::foo::bar::baz`.
///
/// # Type-safety
///
/// It is achieved as any instance of this type can be created only via the checked
/// [`Self::try_from`]. A valid module path cannot start or end with [`MODULE_PATH_DELIM`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModulePath {
    path: String,
}

impl ModulePath {
    // TYPE-SAFE TRANSFORMATION
    // --------------------------------------------------------------------------------------------

    /// Append the module path to a library namespace.
    pub fn to_absolute(&self, library: &LibraryNamespace) -> AbsolutePath {
        let delimiter = if self.path.is_empty() {
            ""
        } else {
            MODULE_PATH_DELIM
        };
        AbsolutePath::new_unchecked(format!("{}{delimiter}{}", library.as_str(), &self.path))
    }
}

impl TryFrom<String> for ModulePath {
    type Error = LibraryError;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        if path.starts_with(MODULE_PATH_DELIM) {
            return Err(LibraryError::module_path_starts_with_delimiter(&path));
        } else if path.ends_with(MODULE_PATH_DELIM) {
            return Err(LibraryError::module_path_ends_with_delimiter(&path));
        }
        Ok(Self { path })
    }
}

impl Deref for ModulePath {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl AsRef<str> for ModulePath {
    fn as_ref(&self) -> &str {
        &self.path
    }
}

// LIBRARY
// ================================================================================================

/// A library definition that provides AST modules for the compilation process.
///
/// Its `IntoIterator` implementation will be used to provide the modules to the assembler.
pub trait Library {
    type ModuleIterator<'a>: Iterator<Item = &'a Module>
    where
        Self: 'a;

    /// Returns the root namespace of this library.
    fn root_ns(&self) -> &LibraryNamespace;

    /// Returns the version number of this library.
    // TODO should have a SEMVER well-formed struct instead of raw string.
    fn version(&self) -> &str;

    /// Iterate the modules available in the library.
    fn modules(&self) -> Self::ModuleIterator<'_>;
}

// MODULE
// ================================================================================================

/// A module containing its absolute path and parsed AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    /// Absolute path of the module.
    pub path: AbsolutePath,
    /// Parsed AST of the module.
    pub ast: ModuleAst,
}

impl Module {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Create a new module from a path and ast.
    pub const fn new(path: AbsolutePath, ast: ModuleAst) -> Self {
        Self { path, ast }
    }

    /// Create a new kernel module from a AST using the constant [`AbsolutePath::kernel_path`].
    pub fn kernel(ast: ModuleAst) -> Self {
        Self {
            path: AbsolutePath::kernel_path(),
            ast,
        }
    }

    // VALIDATIONS
    // --------------------------------------------------------------------------------------------

    /// Validate if the module belongs to the provided namespace.
    pub fn check_namespace(&self, namespace: &LibraryNamespace) -> Result<(), LibraryError> {
        (self.path.namespace() == namespace.as_str())
            .then_some(())
            .ok_or(LibraryError::EmptyProcedureName)
    }
}

impl PartialOrd for Module {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.path.partial_cmp(&other.path)
    }
}

impl Ord for Module {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}
