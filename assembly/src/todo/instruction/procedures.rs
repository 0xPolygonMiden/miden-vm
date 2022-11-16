use super::{Assembler, AssemblerContext, AssemblerError, CallSet, CodeBlock, ProcedureId};

// PROCEDURE INVOCATIONS
// ================================================================================================

impl Assembler {
    pub(super) fn exec_local(
        &self,
        index: u16,
        context: &AssemblerContext,
        callset: &mut CallSet,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        let proc = context.get_local_proc(index)?;

        callset.append(proc.callset());
        Ok(Some(proc.code_root().clone()))
    }

    pub(super) fn exec_imported(
        &self,
        proc_id: &ProcedureId,
        callset: &mut CallSet,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        let proc = self.get_imported_proc(proc_id)?;
        callset.append(proc.callset());
        Ok(Some(proc.code_root().clone()))
    }

    pub(super) fn call_local(
        &self,
        index: u16,
        context: &AssemblerContext,
        callset: &mut CallSet,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        let proc = context.get_local_proc(index)?;

        callset.append(proc.callset());
        // TODO: append own ID

        let digest = proc.code_root().hash();
        Ok(Some(CodeBlock::new_call(digest)))
    }

    pub(super) fn call_imported(
        &self,
        proc_id: &ProcedureId,
        callset: &mut CallSet,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        let proc = self.get_imported_proc(proc_id)?;

        callset.append(proc.callset());
        callset.insert(*proc_id);

        let digest = proc.code_root().hash();
        Ok(Some(CodeBlock::new_call(digest)))
    }
}
