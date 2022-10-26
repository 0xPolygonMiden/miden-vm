use super::{
    super::{check_u32_param, Instruction},
    push_value, AssemblyError, Felt, Operation, Vec,
};

// ENVIRONMENT INPUTS
// ================================================================================================

/// Appends `locaddr.i` operation to the span block to push the absolute address of the local
/// variable at index `i` onto the stack.
///
/// # Errors
///
/// It will return an error if the assembly instruction is malformed or it has inappropriate
/// parameter value according to the number of local variables of the procedurethe
pub fn parse_locaddr(
    instruction: &Instruction,
    span_ops: &mut Vec<Operation>,
    num_proc_locals: u32,
    imm: u32,
) -> Result<(), AssemblyError> {
    if num_proc_locals == 0 {
        return Err(AssemblyError::invalid_instruction_with_reason(
            &instruction.to_string(),
            "no procedure locals available in current context",
        ));
    }

    let index = check_u32_param(instruction, imm, 0, num_proc_locals - 1)?;
    push_value(span_ops, -Felt::from(num_proc_locals - index - 1));
    span_ops.push(Operation::FmpAdd);

    Ok(())
}

/// Appends `sdepth` operation to the current span block to push the current depth of the stack
/// onto the top of the stack. `sdepth` is handled directly by the `SDEPTH` operation.
pub fn parse_sdepth(span_ops: &mut Vec<Operation>) -> Result<(), AssemblyError> {
    span_ops.push(Operation::SDepth);
    Ok(())
}
