use alloc::sync::Arc;

use crate::{
    ast::{FullyQualifiedProcedureName, ProcedureName, Visibility},
    diagnostics::SourceFile,
    LibraryPath, RpoDigest, SourceSpan, Spanned,
};
use vm_core::code_blocks::CodeBlock;

pub type CallSet = alloc::collections::BTreeSet<RpoDigest>;

/// A compiled Miden Assembly procedure, consisting of MAST and basic metadata.
///
/// Procedure metadata includes:
///
/// * Fully-qualified path of the procedure in Miden Assembly (if known)
/// * Number of procedure locals to allocate
/// * The set of MAST roots invoked by this procedure
/// * The original source span and file of the procedure (if available)
/// - Number of procedure locals available to the procedure.
/// - A set of MAST roots of procedures which are invoked from this procedure.
#[derive(Clone, Debug)]
pub struct Procedure {
    span: SourceSpan,
    source_file: Option<Arc<SourceFile>>,
    path: FullyQualifiedProcedureName,
    visibility: Visibility,
    num_locals: u32,
    /// The MAST for this procedure
    code: CodeBlock,
    /// The set of MAST roots called by this procedure
    callset: CallSet,
}

/// Builder
impl Procedure {
    pub(crate) fn new(
        path: FullyQualifiedProcedureName,
        visibility: Visibility,
        num_locals: u32,
        code: CodeBlock,
    ) -> Self {
        Self {
            span: SourceSpan::default(),
            source_file: None,
            path,
            visibility,
            num_locals,
            code,
            callset: Default::default(),
        }
    }

    pub(crate) fn with_span(mut self, span: SourceSpan) -> Self {
        self.span = span;
        self
    }

    pub(crate) fn with_source_file(mut self, source_file: Option<Arc<SourceFile>>) -> Self {
        self.source_file = source_file;
        self
    }

    pub(crate) fn with_callset(mut self, callset: CallSet) -> Self {
        self.callset = callset;
        self
    }
}

/// Metadata
impl Procedure {
    /// Returns a reference to the name of this procedure
    pub fn name(&self) -> &ProcedureName {
        &self.path.name
    }

    /// Returns a reference to the fully-qualified name of this procedure
    pub fn fully_qualified_name(&self) -> &FullyQualifiedProcedureName {
        &self.path
    }

    /// Returns the visibility of this procedure as expressed in the original source code
    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    /// Returns a reference to the fully-qualified module path of this procedure
    pub fn path(&self) -> &LibraryPath {
        &self.path.module
    }

    /// Returns a reference to the Miden Assembly source file from which this
    /// procedure was compiled, if available.
    pub fn source_file(&self) -> Option<Arc<SourceFile>> {
        self.source_file.clone()
    }

    /// Returns the number of memory locals reserved by the procedure.
    pub fn num_locals(&self) -> u32 {
        self.num_locals
    }

    /// Returns the root of this procedure's MAST.
    pub fn mast_root(&self) -> RpoDigest {
        self.code.hash()
    }

    /// Returns a reference to the MAST of this procedure.
    pub fn code(&self) -> &CodeBlock {
        &self.code
    }

    /// Returns a reference to a set of all procedures (identified by their MAST roots) which may
    /// be called during the execution of this procedure.
    pub fn callset(&self) -> &CallSet {
        &self.callset
    }
}

impl Spanned for Procedure {
    fn span(&self) -> SourceSpan {
        self.span
    }
}
