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
        // call instructions cannot be executed inside a kernel
        if context.is_kernel() {
            return Err(AssemblerError::call_in_kernel());
        }

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
        context: &ModuleContext,
        callset: &mut CallSet,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        // call instructions cannot be executed inside a kernel
        if context.is_kernel() {
            return Err(AssemblerError::call_in_kernel());
        }

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
        context: &ModuleContext,
        callset: &mut CallSet,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        // syscall instructions cannot be executed inside a kernel
        if context.is_kernel() {
            return Err(AssemblerError::syscall_in_kernel());
        }

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

        // add ID of the called procedure to the callset. this is needed to make sure the
        // procedure is added to the program's cb_table.
        callset.insert(*proc_id);

        // return the code block of the procedure
        let digest = proc.code_root().hash();
        Ok(Some(CodeBlock::new_syscall(digest)))
    }
}
