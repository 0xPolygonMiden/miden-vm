use vm_core::errors::KernelError;

use crate::{ast::QualifiedProcedureName, diagnostics::Diagnostic, LibraryNamespace, LibraryPath};

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum LibraryError {
    #[error("kernel library must contain at least one exported procedure")]
    #[diagnostic()]
    EmptyKernel,
    #[error("duplicate module '{0}'")]
    #[diagnostic()]
    DuplicateModulePath(LibraryPath),
    #[error("invalid export in kernel library: {procedure_path}")]
    InvalidKernelExport { procedure_path: QualifiedProcedureName },
    #[error(transparent)]
    Kernel(#[from] KernelError),
    #[error("library '{name}' contains {count} modules, but the max is {max}")]
    #[diagnostic()]
    TooManyModulesInLibrary {
        name: LibraryNamespace,
        count: usize,
        max: usize,
    },
}
