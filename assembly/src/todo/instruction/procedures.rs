use crate::{
    todo::{Assembler, AssemblerContext},
    AssemblerError, ProcedureId,
};
use vm_core::{code_blocks::CodeBlock, CodeBlockTable};

// PROCEDURE INVOCATIONS
// ================================================================================================

impl Assembler {
    pub(super) fn exec_local(
        &self,
        idx: u16,
        context: &AssemblerContext,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        context
            .local_procs()
            .get(idx as usize)
            .map(|p| Some(p.code_root().clone()))
            .ok_or_else(|| AssemblerError::undefined_proc(idx))
    }

    pub(super) fn exec_imported(
        &self,
        id: &ProcedureId,
        cb_table: &mut CodeBlockTable,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        self.fetch_procedure(id, cb_table)
            .map(|proc| Some(proc.code_root.clone()))
    }

    pub(super) fn call_local(
        &self,
        idx: u16,
        context: &AssemblerContext,
        cb_table: &mut CodeBlockTable,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        let proc_root = context
            .local_procs()
            .get(idx as usize)
            .map(|p| p.code_root().clone())
            .ok_or_else(|| AssemblerError::undefined_proc(idx))?;

        let digest = proc_root.hash();
        if !cb_table.has(digest) {
            cb_table.insert(proc_root);
        }

        Ok(Some(CodeBlock::new_call(digest)))
    }

    pub(super) fn call_imported(
        &self,
        id: &ProcedureId,
        cb_table: &mut CodeBlockTable,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        let proc_root = self
            .fetch_procedure(id, cb_table)
            .map(|proc| proc.code_root.clone())?;

        let digest = proc_root.hash();
        if !cb_table.has(digest) {
            cb_table.insert(proc_root);
        }

        Ok(Some(CodeBlock::new_call(digest)))
    }
}
