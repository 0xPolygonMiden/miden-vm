#![no_std]

#[macro_use]
extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;

use miden_assembly_syntax as syntax;
use miden_core::{ONE, ZERO};

mod assembler;
#[cfg(any(test, feature = "testing"))]
pub mod testing;
#[cfg(test)]
mod tests;

// Re-exported for downstream crates

pub use miden_assembly_syntax::{
    DefaultSourceManager, KernelLibrary, Library, LibraryNamespace, LibraryPath, ModuleParser,
    Parse, ParseOptions, Report, SourceFile, SourceId, SourceManager, SourceSpan, Span, Spanned,
    ast, diagnostics, library, report,
};
/// Syntax components for the Miden Assembly AST
/// Merkelized abstract syntax tree (MAST) components defining Miden VM programs.
pub use miden_core::{mast, utils};

pub use self::assembler::{Assembler, LinkLibraryKind, LinkerError};

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

// LIBRARY EXTENSIONS
// ================================================================================================

#[cfg(feature = "std")]
pub trait LibraryExt {
    /// Create a [Library] from a standard Miden Assembly project layout.
    ///
    /// The standard layout dictates that a given path is the root of a namespace, and the
    /// directory hierarchy corresponds to the namespace hierarchy. A `.masm` file found in a
    /// given subdirectory of the root, will be parsed with its [LibraryPath] set based on
    /// where it resides in the directory structure.
    ///
    /// This function recursively parses the entire directory structure under `path`, ignoring
    /// any files which do not have the `.masm` extension.
    ///
    /// For example, let's say I call this function like so:
    ///
    /// ```rust
    /// use std::sync::Arc;
    ///
    /// use miden_assembly::{Assembler, Library, LibraryExt, LibraryNamespace};
    /// use miden_core::debuginfo::DefaultSourceManager;
    ///
    /// Library::from_dir(
    ///     "~/masm/std",
    ///     LibraryNamespace::new("std").unwrap(),
    ///     Assembler::new(Arc::new(DefaultSourceManager::default())),
    /// );
    /// ```
    ///
    /// Here's how we would handle various files under this path:
    ///
    /// - ~/masm/std/sys.masm            -> Parsed as "std::sys"
    /// - ~/masm/std/crypto/hash.masm    -> Parsed as "std::crypto::hash"
    /// - ~/masm/std/math/u32.masm       -> Parsed as "std::math::u32"
    /// - ~/masm/std/math/u64.masm       -> Parsed as "std::math::u64"
    /// - ~/masm/std/math/README.md      -> Ignored
    fn from_dir(
        path: impl AsRef<std::path::Path>,
        namespace: LibraryNamespace,
        assembler: Assembler,
    ) -> Result<Library, Report>;
}

#[cfg(feature = "std")]
impl LibraryExt for Library {
    fn from_dir(
        path: impl AsRef<std::path::Path>,
        namespace: LibraryNamespace,
        assembler: Assembler,
    ) -> Result<Self, Report> {
        let path = path.as_ref();

        let src_manager = assembler.source_manager();
        let modules = syntax::parser::read_modules_from_dir(namespace, path, &src_manager)?;
        assembler.assemble_library(modules)
    }
}

// KERNEL LIBRARY EXTENSIONS
// ================================================================================================

#[cfg(feature = "std")]
pub trait KernelLibraryExt {
    /// Create a [KernelLibrary] from a standard Miden Assembly kernel project layout.
    ///
    /// The kernel library will export procedures defined by the module at `sys_module_path`.
    /// If the optional `lib_dir` is provided, all modules under this directory will be
    /// available from the kernel module under the `kernel` namespace. For example, if
    /// `lib_dir` is set to "~/masm/lib", the files will be accessible in the kernel module as
    /// follows:
    ///
    /// - ~/masm/lib/foo.masm        -> Can be imported as "kernel::foo"
    /// - ~/masm/lib/bar/baz.masm    -> Can be imported as "kernel::bar::baz"
    ///
    /// Note: this is a temporary structure which will likely change once
    /// <https://github.com/0xMiden/miden-vm/issues/1436> is implemented.
    fn from_dir(
        sys_module_path: impl AsRef<std::path::Path>,
        lib_dir: Option<impl AsRef<std::path::Path>>,
        assembler: Assembler,
    ) -> Result<KernelLibrary, Report>;
}

#[cfg(feature = "std")]
impl KernelLibraryExt for KernelLibrary {
    fn from_dir(
        sys_module_path: impl AsRef<std::path::Path>,
        lib_dir: Option<impl AsRef<std::path::Path>>,
        mut assembler: Assembler,
    ) -> Result<Self, Report> {
        // if library directory is provided, add modules from this directory to the assembler
        if let Some(lib_dir) = lib_dir {
            let lib_dir = lib_dir.as_ref();
            let namespace = LibraryNamespace::new("kernel").expect("invalid namespace");
            assembler.compile_and_statically_link_from_dir(namespace, lib_dir)?;
        }

        assembler.assemble_kernel(sys_module_path.as_ref())
    }
}
