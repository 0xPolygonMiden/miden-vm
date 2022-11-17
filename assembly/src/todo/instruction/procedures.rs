use super::{Assembler, AssemblerError, CallSet, CodeBlock, ModuleContext, ProcedureId};

// PROCEDURE INVOCATIONS
// ================================================================================================

impl Assembler {
    pub(super) fn exec_local(
        &self,
        index: u16,
        context: &ModuleContext,
        callset: &mut CallSet,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        // get the procedure from the context of the module currently being compiled
        let proc = context.get_local_proc(index)?;

        // append the callset of the procedure to the current callset as executing this procedure
        // may result in calling all procedures called by it
        callset.append(proc.callset());

        // return the code block of the procedure
        Ok(Some(proc.code_root().clone()))
    }

    pub(super) fn exec_imported(
        &self,
        proc_id: &ProcedureId,
        callset: &mut CallSet,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        // get the procedure from the assembler
        let proc = self.get_imported_proc(proc_id)?;
        debug_assert!(proc.is_export(), "not imported procedure");

        // append the callset of the procedure to the current callset as executing this procedure
        // may result in calling all procedures called by it
        callset.append(proc.callset());

        // return the code block of the procedure
        Ok(Some(proc.code_root().clone()))
    }

    pub(super) fn call_local(
        &self,
        index: u16,
        context: &ModuleContext,
        callset: &mut CallSet,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        // get the procedure from the context of the module currently being compiled
        let proc = context.get_local_proc(index)?;

        // append the callset of the procedure to the current callset as executing this procedure
        // may result in calling all procedures called by it
        callset.append(proc.callset());

        // add ID of the called procedure to the callset. if the call is to an local procedure
        // which is not exported, the ID format will be "module_path::proc_index".
        callset.insert(*proc.id());

        // return the code block of the procedure
        let digest = proc.code_root().hash();
        Ok(Some(CodeBlock::new_call(digest)))
    }

    pub(super) fn call_imported(
        &self,
        proc_id: &ProcedureId,
        callset: &mut CallSet,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        let proc = self.get_imported_proc(proc_id)?;
        debug_assert!(proc.is_export(), "not imported procedure");

        // append the callset of the procedure to the current callset as executing this procedure
        // may result in calling all procedures called by it
        callset.append(proc.callset());

        // add ID of the called procedure to the callset. this must be a procedure which has been
        // exported from another module. the ID format will be "module_path::proc_name".
        callset.insert(*proc_id);

        // return the code block of the procedure
        let digest = proc.code_root().hash();
        Ok(Some(CodeBlock::new_call(digest)))
    }

    pub(super) fn syscall(
        &self,
        proc_id: &ProcedureId,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        // fetch from proc cache and check if its a kernel procedure
        // note: the assembler is expected to have all kernel procedures properly inserted in the
        // proc cache upon initialization, with their correct procedure ids
        let digest = self
            .proc_cache
            .get(proc_id)
            .map(|p| p.code_root().hash())
            .filter(|digest| self.kernel.contains_proc(*digest))
            .ok_or_else(|| AssemblerError::undefined_syscall(proc_id))?;

        // return the code block of the procedure
        Ok(Some(CodeBlock::new_syscall(digest)))
    }
}
