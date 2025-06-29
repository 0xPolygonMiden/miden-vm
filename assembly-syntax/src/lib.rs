#![no_std]

#[macro_use]
extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;

pub use miden_core::{Felt, FieldElement, StarkField, Word, prettier, utils::DisplayHex};

pub mod ast;
pub mod diagnostics;
pub mod library;
mod parse;
pub mod parser;
mod sema;
pub mod testing;

#[doc(hidden)]
pub use self::{
    diagnostics::{
        DefaultSourceManager, Report, SourceFile, SourceId, SourceManager, SourceSpan, Span,
        Spanned,
    },
    library::{
        KernelLibrary, Library, LibraryError, LibraryNamespace, LibraryPath, LibraryPathComponent,
        PathError, Version, VersionError,
    },
    parser::{ModuleParser, ParsingError},
};

pub use self::{
    parse::{Parse, ParseOptions},
    sema::SemanticAnalysisError,
};
