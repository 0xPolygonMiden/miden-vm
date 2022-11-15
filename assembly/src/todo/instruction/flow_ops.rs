use vm_core::code_blocks::CodeBlock;

use crate::{
    procedures::Procedure,
    todo::{Assembler, AssemblerContext},
    AssemblerError, ProcedureId,
};

// FLOW CONTROL OPERATIONS
// ================================================================================================

pub(super) fn exec_local(
    idx: u16,
    context: &AssemblerContext,
) -> Result<Option<CodeBlock>, AssemblerError> {
    context
        .locals
        .get(idx as usize)
        .map(|p| Some(p.code_root().clone()))
        .ok_or_else(|| AssemblerError::undefined_proc(idx))
}

pub(super) fn exec_imported(
    id: &ProcedureId,
    assembler: &Assembler,
    context: &mut AssemblerContext,
) -> Result<Option<CodeBlock>, AssemblerError> {
    fetch_or_insert(id, assembler, context).map(Some)
}

pub(super) fn call_local(
    idx: u16,
    context: &mut AssemblerContext,
) -> Result<Option<CodeBlock>, AssemblerError> {
    let root = context
        .locals
        .get(idx as usize)
        .map(Procedure::code_root)
        .ok_or_else(|| AssemblerError::undefined_proc(idx))?;

    let digest = root.hash();
    if !context.cb_table.has(digest) {
        context.cb_table.insert(root.clone());
    }

    Ok(Some(CodeBlock::new_call(digest)))
}

pub(super) fn call_imported(
    id: &ProcedureId,
    assembler: &Assembler,
    context: &mut AssemblerContext,
) -> Result<Option<CodeBlock>, AssemblerError> {
    let root = fetch_or_insert(id, assembler, context)?;
    let digest = root.hash();
    if !context.cb_table.has(digest) {
        context.cb_table.insert(root);
    }

    Ok(Some(CodeBlock::new_call(digest)))
}

fn fetch_or_insert(
    id: &ProcedureId,
    assembler: &Assembler,
    context: &mut AssemblerContext,
) -> Result<CodeBlock, AssemblerError> {
    if let Some(p) = context.procedures.get(id) {
        return Ok(p.clone());
    }

    let module = assembler
        .module_provider
        .get_module(id)
        .ok_or_else(|| AssemblerError::undefined_imported_proc(id))?;

    let proc = module
        .get_procedure(id)
        .ok_or_else(|| AssemblerError::undefined_imported_proc(id))?;

    let Procedure { code_root, .. } = assembler.compile_procedure(proc, context)?;

    Ok(context.procedures.entry(*id).or_insert(code_root).clone())
}
