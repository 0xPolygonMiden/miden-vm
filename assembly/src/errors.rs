use alloc::{string::String, sync::Arc, vec::Vec};
use vm_core::{errors::KernelError, mast::MastForestError};

use crate::{
    ast::FullyQualifiedProcedureName,
    diagnostics::{Diagnostic, RelatedError, RelatedLabel, Report, SourceFile},
    LibraryNamespace, LibraryPath, RpoDigest, SourceSpan,
};

// ASSEMBLER ERROR
// ================================================================================================

/// An error generated during instantiation of an [super::Assembler].
#[derive(Debug, thiserror::Error, Diagnostic)]
#[non_exhaustive]
pub enum AssemblerError {
    #[error("kernel library contains no modules")]
    EmptyKernelLibrary,
    #[error(transparent)]
    Kernel(#[from] KernelError),
    #[error("kernel library does not contain a kernel module")]
    NoKernelModuleInKernelLibrary,
    #[error("non-kernel modules are present in kernel library")]
    NonKernelModulesInKernelLibrary,
}

// ASSEMBLY ERROR
// ================================================================================================

/// An error which can be generated while compiling a Miden assembly program into a MAST.
#[derive(Debug, thiserror::Error, Diagnostic)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum AssemblyError {
    #[error("there are no modules to analyze")]
    #[diagnostic()]
    Empty,
    #[error("assembly failed")]
    #[diagnostic(help("see diagnostics for details"))]
    Failed {
        #[related]
        labels: Vec<RelatedLabel>,
    },
    #[error("found a cycle in the call graph, involving these procedures: {}", nodes.as_slice().join(", "))]
    #[diagnostic()]
    Cycle { nodes: Vec<String> },
    #[error("two procedures found with same mast root, but conflicting definitions ('{first}' and '{second}')")]
    #[diagnostic()]
    ConflictingDefinitions {
        first: FullyQualifiedProcedureName,
        second: FullyQualifiedProcedureName,
    },
    #[error("duplicate definition found for module '{path}'")]
    #[diagnostic()]
    DuplicateModule { path: LibraryPath },
    #[error("undefined module '{path}'")]
    #[diagnostic()]
    UndefinedModule {
        #[label]
        span: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        path: LibraryPath,
    },
    #[error("module namespace is inconsistent with library ('{actual}' vs '{expected}')")]
    #[diagnostic()]
    InconsistentNamespace {
        expected: LibraryNamespace,
        actual: LibraryNamespace,
    },
    #[error("invalid syscall: '{callee}' is not an exported kernel procedure")]
    #[diagnostic()]
    InvalidSysCallTarget {
        #[label("call occurs here")]
        span: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        callee: FullyQualifiedProcedureName,
    },
    #[error("invalid syscall: kernel procedures must be available during compilation, but '{callee}' is not")]
    #[diagnostic()]
    UnknownSysCallTarget {
        #[label("call occurs here")]
        span: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        callee: RpoDigest,
    },
    #[error("invalid use of 'caller' instruction outside of kernel")]
    #[diagnostic(help(
        "the 'caller' instruction is only allowed in procedures defined in a kernel"
    ))]
    CallerOutsideOfKernel {
        #[label]
        span: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
    },
    #[error(transparent)]
    #[diagnostic(transparent)]
    Other(#[from] RelatedError),
    #[error(transparent)]
    Forest(#[from] MastForestError),
}

impl From<Report> for AssemblyError {
    fn from(report: Report) -> Self {
        Self::Other(RelatedError::new(report))
    }
}

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum CompiledLibraryError {
    #[error("Invalid exports: MAST forest has {roots_len} procedure roots, but exports have {exports_len}")]
    #[diagnostic()]
    InvalidExports {
        exports_len: usize,
        roots_len: usize,
    },
}
