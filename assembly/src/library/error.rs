use alloc::string::String;
use vm_core::errors::KernelError;

use crate::{
    ast::FullyQualifiedProcedureName,
    diagnostics::Diagnostic,
    library::{LibraryNamespaceError, VersionError},
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
    #[error("Invalid exports: MAST forest has {roots_len} procedure roots, but exports have {exports_len}")]
    #[diagnostic()]
    InvalidExports {
        exports_len: usize,
        roots_len: usize,
    },
    #[error("Invalid export in kernel library: {procedure_path}")]
    InvalidKernelExport {
        procedure_path: FullyQualifiedProcedureName,
    },
    #[error(transparent)]
    Kernel(#[from] KernelError),
}
