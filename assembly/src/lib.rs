#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

use core::ops::Deref;
use vm_core::{
    code_blocks::CodeBlock,
    crypto,
    utils::{
        collections::{BTreeMap, BTreeSet, Vec},
        string::{String, ToString},
    },
    CodeBlockTable, Felt, Kernel, Operation, Program, StarkField, ONE, ZERO,
};

mod library;
pub use library::{Library, MaslLibrary, Module, Version};

mod procedures;
use procedures::{CallSet, Procedure};
pub use procedures::{ProcedureId, ProcedureName};

mod parsers;
use parsers::PROCEDURE_LABEL_PARSER;
pub use parsers::{parse_module, parse_program, ModuleAst, ProcedureAst, ProgramAst};

mod serde;
pub use serde::{ByteReader, ByteWriter, Deserializable, Serializable};

mod tokens;
use tokens::{Token, TokenStream};

mod errors;
pub use errors::{AssemblyError, LabelError, LibraryError, ParsingError, SerializationError};

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

/// The maximum number of elements that can be popped from the advice stack in a single `adv_push`
/// instruction.
const ADVICE_READ_LIMIT: u8 = 16;

/// The maximum number of bits by which a u32 value can be shifted in a bitwise operation.
const MAX_U32_SHIFT_VALUE: u8 = 31;

/// The maximum number of bits by which a u32 value can be rotated in a bitwise operation.
const MAX_U32_ROTATE_VALUE: u8 = 31;

/// The maximum number of bits allowed for the exponent parameter for exponentiation instructions.
const MAX_EXP_BITS: u8 = 64;

/// The maximum length of a constant or procedure's label.
const MAX_LABEL_LEN: usize = 100;

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

    // TYPE-SAFE TRANSFORMATION
    // --------------------------------------------------------------------------------------------

    /// Append the name into the absolute path.
    pub fn concatenate<N>(&self, name: N) -> Self
    where
        N: AsRef<str>,
    {
        Self {
            path: format!("{}{MODULE_PATH_DELIM}{}", self.path, name.as_ref()),
        }
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

impl Serializable for AbsolutePath {
    fn write_into(&self, target: &mut ByteWriter) -> Result<(), SerializationError> {
        target.write_str(&self.path)
    }
}

impl Deserializable for AbsolutePath {
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        // TODO add name validation
        // https://github.com/maticnetwork/miden/issues/583
        let path = bytes.read_str()?;
        if !path.contains(MODULE_PATH_DELIM) {
            return Err(SerializationError::InvalidPathNoDelimiter);
        }
        Ok(Self {
            path: path.to_string(),
        })
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

impl LibraryNamespace {
    // VALIDATORS
    // --------------------------------------------------------------------------------------------

    /// Perform the type validations.
    fn validate(name: &str) -> Result<(), LibraryError> {
        // TODO add name validation
        // https://github.com/maticnetwork/miden/issues/583
        if name.contains(MODULE_PATH_DELIM) {
            return Err(LibraryError::library_name_with_delimiter(name));
        }
        Ok(())
    }
}

impl TryFrom<String> for LibraryNamespace {
    type Error = LibraryError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        Self::validate(&name)?;
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

impl Serializable for LibraryNamespace {
    fn write_into(&self, target: &mut ByteWriter) -> Result<(), SerializationError> {
        target.write_str(&self.name)
    }
}

impl Deserializable for LibraryNamespace {
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        let name = bytes.read_str()?;
        Self::validate(name).map_err(|_| SerializationError::InvalidNamespace)?;
        Ok(Self {
            name: name.to_string(),
        })
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
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Create a new empty module path.
    pub fn empty() -> Self {
        Self {
            path: String::new(),
        }
    }

    // VALIDATORS
    // --------------------------------------------------------------------------------------------

    /// Perform the type validations.
    fn validate(path: &str) -> Result<(), LibraryError> {
        if path.starts_with(MODULE_PATH_DELIM) {
            return Err(LibraryError::module_path_starts_with_delimiter(path));
        } else if path.ends_with(MODULE_PATH_DELIM) {
            return Err(LibraryError::module_path_ends_with_delimiter(path));
        }
        Ok(())
    }

    // TYPE-SAFE TRANSFORMATION
    // --------------------------------------------------------------------------------------------

    /// Append the module path to a library namespace.
    pub fn to_absolute(&self, library: &LibraryNamespace) -> AbsolutePath {
        let delimiter = if self.path.is_empty() { "" } else { MODULE_PATH_DELIM };
        AbsolutePath::new_unchecked(format!("{}{delimiter}{}", library.as_str(), &self.path))
    }

    /// Strip the namespace from an absolute path and return the relative module path.
    pub fn strip_namespace(path: &AbsolutePath) -> Self {
        Self {
            path: path
                .as_str()
                .split_once(MODULE_PATH_DELIM)
                .expect("type-safety violation of absolute path")
                .1
                .to_string(),
        }
    }

    /// Appends the given name into the module path. Will not prefix with the delimiter if the
    /// current module path is empty.
    pub fn concatenate<N>(&self, name: N) -> Self
    where
        N: AsRef<str>,
    {
        if self.path.is_empty() {
            Self {
                path: name.as_ref().to_string(),
            }
        } else {
            Self {
                path: format!("{}{MODULE_PATH_DELIM}{}", self.path, name.as_ref()),
            }
        }
    }
}

impl TryFrom<String> for ModulePath {
    type Error = LibraryError;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        Self::validate(&path)?;
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

impl Serializable for ModulePath {
    fn write_into(&self, target: &mut ByteWriter) -> Result<(), SerializationError> {
        target.write_str(&self.path)
    }
}

impl Deserializable for ModulePath {
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        let path = bytes.read_str()?;
        Self::validate(path).map_err(|_| SerializationError::InvalidModulePath)?;
        Ok(Self {
            path: path.to_string(),
        })
    }
}
