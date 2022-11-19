use super::{Assembler, AssemblerError, AssemblyContext, CodeBlock, ProcedureId};

// PROCEDURE INVOCATIONS
// ================================================================================================

impl Assembler {
    pub(super) fn exec_local(
        &self,
        proc_idx: u16,
        context: &mut AssemblyContext,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        // register an "inlined" call to the procedure at the specified index in the module
        // currently being complied; this updates the callset of the procedure currently being
        // compiled
        let proc = context.register_local_call(proc_idx, true)?;

        // TODO: if the procedure consists of a single SPAN block, we could just append all
        // operations from that SPAN block to the span builder instead of returning a code block

        // return the code block of the procedure
        Ok(Some(proc.code_root().clone()))
    }

    pub(super) fn exec_imported(
        &self,
        proc_id: &ProcedureId,
        context: &mut AssemblyContext,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        // get the procedure from the assembler
        let proc = self.get_imported_proc(proc_id, context)?;
        debug_assert!(proc.is_export(), "not imported procedure");

        // register and "inlined" call to the procedure; this updates the callset of the
        // procedure currently being compiled
        context.register_external_call(proc, true)?;

        // TODO: if the procedure consists of a single SPAN block, we could just append all
        // operations from that SPAN block to the span builder instead of returning a code block

        // return the code block of the procedure
        Ok(Some(proc.code_root().clone()))
    }

    pub(super) fn call_local(
        &self,
        index: u16,
        context: &mut AssemblyContext,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        // register an "non-inlined" call to the procedure at the specified index in the module
        // currently being complied; this updates the callset of the procedure currently being
        // compiled
        let proc = context.register_local_call(index, false)?;

        // create a new CALL block for the procedure call and return
        let digest = proc.code_root().hash();
        Ok(Some(CodeBlock::new_call(digest)))
    }

    pub(super) fn call_imported(
        &self,
        proc_id: &ProcedureId,
        context: &mut AssemblyContext,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        // get the procedure from the assembler
        let proc = self.get_imported_proc(proc_id, context)?;
        debug_assert!(proc.is_export(), "not imported procedure");

        // register and "non-inlined" call to the procedure; this updates the callset of the
        // procedure currently being compiled
        context.register_external_call(proc, false)?;

        // create a new CALL block for the procedure call and return
        let digest = proc.code_root().hash();
        Ok(Some(CodeBlock::new_call(digest)))
    }

    pub(super) fn syscall(
        &self,
        proc_id: &ProcedureId,
        context: &mut AssemblyContext,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        // fetch from proc cache and check if its a kernel procedure
        // note: the assembler is expected to have all kernel procedures properly inserted in the
        // proc cache upon initialization, with their correct procedure ids
        let proc = self
            .proc_cache
            .get(proc_id)
            .ok_or_else(|| AssemblerError::undefined_syscall(proc_id))?;

        // since call and syscall instructions cannot be executed inside a kernel, a callset for
        // a kernel procedure must be empty.
        debug_assert!(
            proc.callset().is_empty(),
            "non-empty callset for a kernel procedure"
        );

        // register and "non-inlined" call to the procedure; this updates the callset of the
        // procedure currently being compiled
        context.register_external_call(proc, false)?;

        // create a new SYSCALL block for the procedure call and return
        let digest = proc.code_root().hash();
        Ok(Some(CodeBlock::new_syscall(digest)))
    }
}
