#![no_std]
#![cfg_attr(feature = "nightly", feature(error_in_core))]

#[macro_use]
extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;

use vm_core::{
    code_blocks::CodeBlock,
    crypto,
    errors::KernelError,
    utils::{
        ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable, SliceReader,
    },
    CodeBlockTable, Felt, Kernel, Operation, Program, StarkField, ONE, ZERO,
};

mod assembler;
pub use assembler::{Assembler, AssemblyContext};

pub mod ast;
use ast::{NAMESPACE_LABEL_PARSER, PROCEDURE_LABEL_PARSER};

pub mod diagnostics;
mod errors;
pub use errors::{AssemblyError, LabelError, LibraryError, ParsingError, PathError};

mod library;
pub use library::{Library, LibraryNamespace, LibraryPath, MaslLibrary, Module, Version};

mod procedures;
use procedures::{CallSet, NamedProcedure, Procedure};
pub use procedures::{ProcedureId, ProcedureName};

mod tokens;
use tokens::{Token, TokenStream};

#[cfg(test)]
mod tests;

// RE-EXPORTS
// ================================================================================================

pub use vm_core::utils;

// CONSTANTS
// ================================================================================================

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

/// The maximum length (in bytes) of a constant, procedure, or library namespace labels.
const MAX_LABEL_LEN: usize = 255;

/// The required length of the hexadecimal representation for an input value when more than one hex
/// input is provided to `push` masm operation without period separators.
const HEX_CHUNK_SIZE: usize = 16;
