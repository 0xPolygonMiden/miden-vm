use vm_core::errors::KernelError;

use crate::{ast::QualifiedProcedureName, diagnostics::Diagnostic};

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum LibraryError {
    #[error("kernel library must contain at least one exported procedure")]
    #[diagnostic()]
    EmptyKernel,
    #[error("invalid export in kernel library: {procedure_path}")]
    InvalidKernelExport { procedure_path: QualifiedProcedureName },
    #[error(transparent)]
    Kernel(#[from] KernelError),
    #[error("invalid export: no procedure root for {procedure_path} procedure")]
    NoProcedureRootForExport { procedure_path: QualifiedProcedureName },
}
