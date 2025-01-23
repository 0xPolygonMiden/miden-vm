use alloc::{string::String, sync::Arc, vec::Vec};

use vm_core::mast::MastForestError;

use crate::{
    ast::QualifiedProcedureName,
    diagnostics::{Diagnostic, RelatedError, RelatedLabel, Report, SourceFile},
    LibraryNamespace, LibraryPath, SourceSpan,
};

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
        first: QualifiedProcedureName,
        second: QualifiedProcedureName,
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
        callee: QualifiedProcedureName,
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

    #[error("invalid procedure: body must contain at least one instruction if it has decorators")]
    #[diagnostic()]
    EmptyProcedureBodyWithDecorators {
        span: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
    },
    #[error("invalid local memory index: {index} is out of bounds (valid range: 0..{max})")]
    #[diagnostic(help("procedure has {num_locals} locals available"))]
    InvalidLocalMemoryIndex {
        #[label("invalid index used here")]
        span: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        index: u16,
        num_locals: u16,
        max: u16,
    },
    #[error("invalid local word index: {local_addr}")]
    #[diagnostic()]
    InvalidLocalWordIndex {
        #[label]
        span: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        local_addr: u16,
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
