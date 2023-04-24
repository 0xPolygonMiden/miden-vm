#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

use core::{ops::Deref, str::from_utf8};
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

mod path;
pub use path::LibraryPath;

mod procedures;
use procedures::{CallSet, Procedure};
pub use procedures::{ProcedureId, ProcedureName};

mod parsers;
use parsers::PROCEDURE_LABEL_PARSER;
pub use parsers::{parse_module, parse_program, ModuleAst, ProcedureAst, ProgramAst};

pub use vm_core::utils::{
    ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable, SliceReader,
};

mod tokens;
pub use tokens::SourceLocation;
use tokens::{Token, TokenStream};

mod errors;
pub use errors::{AssemblyError, LabelError, LibraryError, ParsingError, PathError};

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
    /// TODO
    /// Enforce that we don't allow \# -of bytes in library namespace to exceed (2^16 - 1).
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u16(self.name.len() as u16);
        target.write_bytes(self.name.as_bytes());
    }
}

impl Deserializable for LibraryNamespace {
    /// TODO
    /// Enforce that we don't allow \# -of bytes in library namespace to exceed (2^16 - 1).
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let nlen = source.read_u16()? as usize;
        let name = source.read_vec(nlen)?;
        let name =
            from_utf8(&name).map_err(|e| DeserializationError::InvalidValue(e.to_string()))?;
        Self::validate(name).map_err(|e| DeserializationError::InvalidValue(e.to_string()))?;
        Ok(Self {
            name: name.to_string(),
        })
    }
}
