use alloc::{string::String, vec::Vec};
use vm_core::errors::KernelError;

use crate::{
    ast::QualifiedProcedureName,
    diagnostics::Diagnostic,
    library::{LibraryNamespaceError, VersionError},
    prettier::pretty_print_csv,
    DeserializationError, LibraryNamespace, LibraryPath, PathError,
};

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum LibraryError {
    #[error("library '{0}' does not contain any modules")]
    #[diagnostic()]
    Empty(LibraryNamespace),
    #[error("module '{0}' not found")]
    #[diagnostic()]
    ModuleNotFound(String),
    #[error("duplicate module '{0}'")]
    #[diagnostic()]
    DuplicateModulePath(LibraryPath),
    #[error("duplicate namespace '{0}'")]
    #[diagnostic()]
    DuplicateNamespace(LibraryNamespace),
    #[error("inconsistent module namespace: expected '{expected}', but was {actual}")]
    #[diagnostic()]
    InconsistentNamespace {
        expected: LibraryNamespace,
        actual: LibraryNamespace,
    },
    #[error("library '{name}' contains {count} dependencies, but the max is {max}")]
    #[diagnostic()]
    TooManyDependenciesInLibrary {
        name: LibraryNamespace,
        count: usize,
        max: usize,
    },
    #[error("library '{name}' contains {count} modules, but the max is {max}")]
    #[diagnostic()]
    TooManyModulesInLibrary {
        name: LibraryNamespace,
        count: usize,
        max: usize,
    },
    #[error("failed to deserialize library from '{path}': {error}")]
    #[diagnostic()]
    DeserializationFailed {
        path: String,
        error: DeserializationError,
    },
    #[error(transparent)]
    #[diagnostic()]
    Namespace(#[from] LibraryNamespaceError),
    #[error(transparent)]
    #[diagnostic()]
    Path(#[from] PathError),
    #[error(transparent)]
    #[diagnostic()]
    Version(#[from] VersionError),
    #[error(transparent)]
    #[diagnostic()]
    #[cfg(feature = "std")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum CompiledLibraryError {
    #[error("Invalid exports: there must be at least one export")]
    #[diagnostic()]
    EmptyExports,
    #[error("exports are not in the same namespace; all namespaces: {namespaces:?}")]
    InconsistentNamespaces { namespaces: Vec<LibraryNamespace> },
    #[error("invalid export in kernel library: {procedure_path}")]
    InvalidKernelExport {
        procedure_path: QualifiedProcedureName,
    },
    #[error(transparent)]
    Kernel(#[from] KernelError),
    #[error("no MAST roots for the following exports: {}", pretty_print_csv(missing_exports.as_slice()))]
    MissingExports {
        missing_exports: Vec<QualifiedProcedureName>,
    },
}
