use alloc::{string::String, sync::Arc, vec::Vec};

use crate::{
    ast::{FullyQualifiedProcedureName, ProcedureName},
    diagnostics::{Diagnostic, RelatedError, RelatedLabel, Report, SourceFile},
    KernelError, LibraryNamespace, LibraryPath, RpoDigest, SourceSpan,
};

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
    #[error("attempted to provide multiple kernel modules to the assembler")]
    #[diagnostic()]
    ConflictingKernels,
    #[error("two procedures found with same mast root, but conflicting definitions ('{first}' and '{second}')")]
    #[diagnostic()]
    ConflictingDefinitions {
        first: FullyQualifiedProcedureName,
        second: FullyQualifiedProcedureName,
    },
    #[error("conflicting entrypoints were provided (in '{first}' and '{second}')")]
    #[diagnostic()]
    MultipleEntry {
        first: LibraryPath,
        second: LibraryPath,
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
    #[error("undefined local procedure '{name}'")]
    #[diagnostic()]
    UndefinedLocalProcedure {
        #[label]
        name: ProcedureName,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
    },
    #[error("expected procedure with MAST root '{digest}' to be found in assembler cache, but it was not")]
    #[diagnostic()]
    UndefinedCallSetProcedure { digest: RpoDigest },
    #[error("module namespace is inconsistent with library ('{actual}' vs '{expected}')")]
    #[diagnostic()]
    InconsistentNamespace {
        expected: LibraryNamespace,
        actual: LibraryNamespace,
    },
    #[error(
        "re-exported procedure '{name}' is self-recursive: resolving the alias is not possible"
    )]
    #[diagnostic()]
    RecursiveAlias {
        #[label("recursion starts here")]
        name: FullyQualifiedProcedureName,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
    },
    #[error("cannot call phantom procedure: phantom calls are disabled")]
    #[diagnostic(help("mast root is {digest}"))]
    PhantomCallsNotAllowed {
        #[label("the procedure referenced here is not available")]
        span: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        digest: RpoDigest,
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
    Parsing(#[from] crate::parser::ParsingError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    SemanticAnalysis(#[from] crate::sema::SemanticAnalysisError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    Syntax(#[from] crate::sema::SyntaxError),
    #[error(transparent)]
    #[diagnostic()]
    Kernel(#[from] KernelError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    Library(#[from] crate::library::LibraryError),
    #[error(transparent)]
    #[cfg(feature = "std")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    #[diagnostic(transparent)]
    Other(#[from] RelatedError),
}
impl From<Report> for AssemblyError {
    fn from(report: Report) -> Self {
        Self::Other(RelatedError::new(report))
    }
}
