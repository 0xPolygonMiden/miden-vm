use super::{
    super::{check_u32_param, Instruction},
    AssemblyError, Operation,
};
use vm_core::utils::PushMany;

// CONSTANTS
// ================================================================================================

/// The maximum number of elements that can be read from the advice tape in a single `push`
/// operation.
const ADVICE_READ_LIMIT: u32 = 16;

// NON-DETERMINISTIC (ADVICE) INPUTS
// ================================================================================================

/// Appends the number of `READ` operations specified by the operation's immediate value to the
/// span block. This removes the specified number of items from the advice tape and pushes them
/// onto the stack. The number of items that can be read from the advice tape is limited to 16.
pub fn parse_adv_push(
    instruction: &Instruction,
    span_ops: &mut Vec<Operation>,
    n: u32,
) -> Result<(), AssemblyError> {
    check_u32_param(instruction, n, 1, ADVICE_READ_LIMIT)?;
    // read n items from the advice tape and push then onto the stack
    span_ops.push_many(Operation::Read, n as usize);

    Ok(())
}

/// Removes the next word (4 elements) from the advice tape and overwrites the top 4 elements of
/// the stack with it. Fails if the advice tape has fewer than 4 elements. After validation, this
/// operation uses the `READW` machine operation directly.
pub fn parse_adv_loadw(span_ops: &mut Vec<Operation>) -> Result<(), AssemblyError> {
    // load a word from the advice tape
    span_ops.push(Operation::ReadW);
    Ok(())
}
