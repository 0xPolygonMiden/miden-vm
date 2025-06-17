#![no_std]
#![cfg_attr(all(nightly, not(feature = "std")), feature(error_in_core))]

#[macro_use]
extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;

use vm_core::{
    Felt, ONE, ZERO,
    crypto::hash::RpoDigest,
    prettier,
    utils::{
        ByteReader, ByteWriter, Deserializable, DeserializationError, DisplayHex, Serializable,
    },
};

mod assembler;
pub mod ast;
mod compile;
pub mod diagnostics;
mod library;
mod parser;
mod sema;
#[cfg(any(test, feature = "testing"))]
pub mod testing;
#[cfg(test)]
mod tests;

// Re-exported for downstream crates

/// Merkelized abstract syntax tree (MAST) components defining Miden VM programs.
pub use vm_core::mast;
pub use vm_core::utils;

pub use self::{
    assembler::{Assembler, LinkLibraryKind, LinkerError},
    compile::{Compile, Options as CompileOptions},
    diagnostics::{
        DefaultSourceManager, Report, SourceFile, SourceId, SourceManager, SourceSpan, Span,
        Spanned,
    },
    library::{
        KernelLibrary, Library, LibraryError, LibraryNamespace, LibraryPath, LibraryPathComponent,
        PathError, Version, VersionError,
    },
    parser::ModuleParser,
};

// CONSTANTS
// ================================================================================================

/// The maximum number of elements that can be popped from the advice stack in a single `adv_push`
/// instruction.
const ADVICE_READ_LIMIT: u8 = 16;

/// The maximum number of bits by which a u32 value can be shifted in a bitwise operation.
const MAX_U32_SHIFT_VALUE: u8 = 31;

/// The maximum number of bits by which a u32 value can be rotated in a bitwise operation.
const MAX_U32_ROTATE_VALUE: u8 = 31;

/// The maximum number of bits allowed for the exponent parameter for exponentiation instructions.
const MAX_EXP_BITS: u8 = 64;
