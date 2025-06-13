use vm_core::mast::MastForestError;

use crate::{
    assembler::LinkerError,
    diagnostics::{Diagnostic, RelatedError, RelatedLabel, Report},
};

// ASSEMBLY ERROR
// ================================================================================================

/// An error which can be generated while compiling a Miden assembly program into a MAST.
#[derive(Debug, thiserror::Error, Diagnostic)]
#[non_exhaustive]
pub enum AssemblyError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Report(RelatedError),
    // Technically MastForestError is the source error here, but since AssemblyError is converted
    // into a Report and that doesn't implement core::error::Error, treating MastForestError as a
    // source error would effectively swallow it, so we include it in the error message instead.
    #[error("{0}: {1}")]
    Forest(&'static str, MastForestError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    Linker(#[from] LinkerError),
}

impl AssemblyError {
    pub(super) fn forest_error(message: &'static str, source: MastForestError) -> Self {
        Self::Forest(message, source)
    }
}

impl From<Report> for AssemblyError {
    fn from(value: Report) -> Self {
        Self::Report(RelatedError::new(value))
    }
}

impl From<RelatedLabel> for AssemblyError {
    fn from(value: RelatedLabel) -> Self {
        Self::Report(RelatedError::new(Report::new(value)))
    }
}
