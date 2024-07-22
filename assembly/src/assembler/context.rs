use alloc::{boxed::Box, sync::Arc};

use super::{procedure::CallSet, GlobalProcedureIndex, Procedure};
use crate::{
    ast::{FullyQualifiedProcedureName, Visibility},
    diagnostics::SourceFile,
    AssemblyError, LibraryPath, RpoDigest, SourceSpan, Spanned,
};
use vm_core::mast::{MastForest, MastNodeId};

pub struct ProcedureContext {
    span: SourceSpan,
    source_file: Option<Arc<SourceFile>>,
    gid: GlobalProcedureIndex,
    name: FullyQualifiedProcedureName,
    visibility: Visibility,
    num_locals: u16,
    callset: CallSet,
}

impl ProcedureContext {
    pub fn new(
        gid: GlobalProcedureIndex,
        name: FullyQualifiedProcedureName,
        visibility: Visibility,
    ) -> Self {
        Self {
            span: name.span(),
            source_file: None,
            gid,
            name,
            visibility,
            num_locals: 0,
            callset: Default::default(),
        }
    }

    pub fn with_span(mut self, span: SourceSpan) -> Self {
        self.span = span;
        self
    }

    pub fn with_source_file(mut self, source_file: Option<Arc<SourceFile>>) -> Self {
        self.source_file = source_file;
        self
    }

    pub fn with_num_locals(mut self, num_locals: u16) -> Self {
        self.num_locals = num_locals;
        self
    }

    pub fn insert_callee(&mut self, callee: RpoDigest) {
        self.callset.insert(callee);
    }

    pub fn extend_callset<I>(&mut self, callees: I)
    where
        I: IntoIterator<Item = RpoDigest>,
    {
        self.callset.extend(callees);
    }

    pub fn num_locals(&self) -> u16 {
        self.num_locals
    }

    pub fn id(&self) -> GlobalProcedureIndex {
        self.gid
    }

    pub fn name(&self) -> &FullyQualifiedProcedureName {
        &self.name
    }

    #[allow(unused)]
    pub fn module(&self) -> &LibraryPath {
        &self.name.module
    }

    pub fn source_file(&self) -> Option<Arc<SourceFile>> {
        self.source_file.clone()
    }

    pub fn is_kernel(&self) -> bool {
        self.visibility.is_syscall()
    }

    /// Registers a call to an externally-defined procedure which we have previously compiled.
    ///
    /// The call set of the callee is added to the call set of the procedure we are currently
    /// compiling, to reflect that all of the code reachable from the callee is by extension
    /// reachable by the caller.
    pub fn register_external_call(
        &mut self,
        callee: &Procedure,
        inlined: bool,
        mast_forest: &MastForest,
    ) -> Result<(), AssemblyError> {
        // If we call the callee, it's callset is by extension part of our callset
        self.extend_callset(callee.callset().iter().cloned());

        // If the callee is not being inlined, add it to our callset
        if !inlined {
            self.insert_callee(callee.mast_root(mast_forest));
        }

        Ok(())
    }

    pub fn into_procedure(self, body_node_id: MastNodeId) -> Box<Procedure> {
        let procedure =
            Procedure::new(self.name, self.visibility, self.num_locals as u32, body_node_id)
                .with_span(self.span)
                .with_source_file(self.source_file)
                .with_callset(self.callset);
        Box::new(procedure)
    }
}

impl Spanned for ProcedureContext {
    fn span(&self) -> SourceSpan {
        self.span
    }
}
