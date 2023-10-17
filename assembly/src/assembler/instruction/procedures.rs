use super::{
    super::Vec, Assembler, AssemblyContext, AssemblyError, CodeBlock, Operation, ProcedureId,
    RpoDigest, SpanBuilder,
};

// PROCEDURE INVOCATIONS
// ================================================================================================

impl Assembler {
    pub(super) fn exec_local(
        &self,
        proc_idx: u16,
        context: &mut AssemblyContext,
    ) -> Result<Option<CodeBlock>, AssemblyError> {
        // register an "inlined" call to the procedure at the specified index in the module
        // currently being complied; this updates the callset of the procedure currently being
        // compiled
        let proc = context.register_local_call(proc_idx, true)?;

        // TODO: if the procedure consists of a single SPAN block, we could just append all
        // operations from that SPAN block to the span builder instead of returning a code block

        // return the code block of the procedure
        Ok(Some(proc.code().clone()))
    }

    pub(super) fn exec_imported(
        &self,
        proc_id: &ProcedureId,
        context: &mut AssemblyContext,
    ) -> Result<Option<CodeBlock>, AssemblyError> {
        // make sure the procedure is in procedure cache
        self.ensure_procedure_is_in_cache(proc_id, context)?;

        // get the procedure from the assembler
        let proc_cache = self.proc_cache.borrow();
        let proc = proc_cache.get_by_id(proc_id).expect("procedure not in cache");

        // register an "inlined" call to the procedure; this updates the callset of the
        // procedure currently being compiled
        context.register_external_call(proc, true)?;

        // TODO: if the procedure consists of a single SPAN block, we could just append all
        // operations from that SPAN block to the span builder instead of returning a code block

        // return the code block of the procedure
        Ok(Some(proc.code().clone()))
    }

    pub(super) fn call_local(
        &self,
        index: u16,
        context: &mut AssemblyContext,
    ) -> Result<Option<CodeBlock>, AssemblyError> {
        // register a "non-inlined" call to the procedure at the specified index in the module
        // currently being complied; this updates the callset of the procedure currently being
        // compiled
        let proc = context.register_local_call(index, false)?;

        // create a new CALL block for the procedure call and return
        Ok(Some(CodeBlock::new_call(proc.mast_root())))
    }

    pub(super) fn call_mast_root(
        &self,
        mast_root: &RpoDigest,
        context: &mut AssemblyContext,
    ) -> Result<Option<CodeBlock>, AssemblyError> {
        // get the procedure from the assembler
        let proc_cache = self.proc_cache.borrow();

        // if the procedure with the specified MAST root exists in procedure cache, register a
        // "non-inlined" call to the procedure (to update the callset of the procedure currently
        // being compiled); otherwise, register a "phantom" call.
        match proc_cache.get_by_hash(mast_root) {
            Some(proc) => context.register_external_call(proc, false)?,
            None => context.register_phantom_call(*mast_root)?,
        }

        // create a new CALL block for the procedure call and return
        Ok(Some(CodeBlock::new_call(*mast_root)))
    }

    pub(super) fn call_imported(
        &self,
        proc_id: &ProcedureId,
        context: &mut AssemblyContext,
    ) -> Result<Option<CodeBlock>, AssemblyError> {
        // make sure the procedure is in procedure cache
        self.ensure_procedure_is_in_cache(proc_id, context)?;

        // get the procedure from the assembler
        let proc_cache = self.proc_cache.borrow();
        let proc = proc_cache.get_by_id(proc_id).expect("procedure not in cache");

        // register a "non-inlined" call to the procedure; this updates the callset of the
        // procedure currently being compiled
        context.register_external_call(proc, false)?;

        // create a new CALL block for the procedure call and return
        Ok(Some(CodeBlock::new_call(proc.mast_root())))
    }

    pub(super) fn syscall(
        &self,
        proc_id: &ProcedureId,
        context: &mut AssemblyContext,
    ) -> Result<Option<CodeBlock>, AssemblyError> {
        // fetch from proc cache and check if its a kernel procedure
        // note: the assembler is expected to have all kernel procedures properly inserted in the
        // proc cache upon initialization, with their correct procedure ids
        let proc_cache = self.proc_cache.borrow();

        let proc = proc_cache
            .get_by_id(proc_id)
            .ok_or_else(|| AssemblyError::kernel_proc_not_found(proc_id))?;

        // since call and syscall instructions cannot be executed inside a kernel, a callset for
        // a kernel procedure must be empty.
        debug_assert!(proc.callset().is_empty(), "non-empty callset for a kernel procedure");

        // register a "non-inlined" call to the procedure; this updates the callset of the
        // procedure currently being compiled
        context.register_external_call(proc, false)?;

        // create a new SYSCALL block for the procedure call and return
        Ok(Some(CodeBlock::new_syscall(proc.mast_root())))
    }

    pub(super) fn dynexec(&self) -> Result<Option<CodeBlock>, AssemblyError> {
        // create a new DYN block for the dynamic code execution and return
        Ok(Some(CodeBlock::new_dyn()))
    }

    pub(super) fn dyncall(&self) -> Result<Option<CodeBlock>, AssemblyError> {
        // create a new CALL block whose target is DYN
        Ok(Some(CodeBlock::new_dyncall()))
    }

    pub(super) fn procref_local(
        &self,
        proc_idx: u16,
        context: &mut AssemblyContext,
        span: &mut SpanBuilder,
    ) -> Result<Option<CodeBlock>, AssemblyError> {
        // get root of the compiled local procedure
        let proc_root = context.get_compiled_procedure(proc_idx)?.mast_root();
        // create an array with `Push` operations containing root elements
        let ops: Vec<Operation> = proc_root.iter().map(|elem| Operation::Push(*elem)).collect();
        span.add_ops(ops)
    }

    pub(super) fn procref_imported(
        &self,
        proc_id: &ProcedureId,
        context: &mut AssemblyContext,
        span: &mut SpanBuilder,
    ) -> Result<Option<CodeBlock>, AssemblyError> {
        // make sure the procedure is in procedure cache
        self.ensure_procedure_is_in_cache(proc_id, context)?;

        // get the procedure from the assembler
        let proc_cache = self.proc_cache.borrow();
        let proc = proc_cache.get_by_id(proc_id).expect("procedure not in cache");

        // get root of the cimported procedure
        let proc_root = proc.mast_root();
        // create an array with `Push` operations containing root elements
        let ops: Vec<Operation> = proc_root.iter().map(|elem| Operation::Push(*elem)).collect();
        span.add_ops(ops)
    }
}
