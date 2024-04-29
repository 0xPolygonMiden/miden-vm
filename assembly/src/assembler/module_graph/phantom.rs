use alloc::sync::Arc;

use crate::{diagnostics::SourceFile, RpoDigest, SourceSpan, Spanned};

/// Represents a call to a procedure for which we do not have an implementation.
///
/// Such calls are still valid, as at runtime they may be supplied to the VM, but we are limited
/// in how much we can reason about such procedures, so we represent them and handle them
/// explicitly.
#[derive(Clone)]
pub struct PhantomCall {
    /// The source span associated with the call
    pub span: SourceSpan,
    /// The source file corresponding to `span`, if available
    #[allow(dead_code)]
    pub source_file: Option<Arc<SourceFile>>,
    /// The MAST root of the callee
    pub callee: RpoDigest,
}

impl Spanned for PhantomCall {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

impl Eq for PhantomCall {}

impl PartialEq for PhantomCall {
    fn eq(&self, other: &Self) -> bool {
        self.callee.eq(&other.callee)
    }
}

impl Ord for PhantomCall {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.callee.cmp(&other.callee)
    }
}

impl PartialOrd for PhantomCall {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
