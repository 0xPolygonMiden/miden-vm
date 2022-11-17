use super::super::{span_builder::SpanBuilder, AssemblerError};
use vm_core::{code_blocks::CodeBlock, Operation::*};

// CONSTANTS
// ================================================================================================

/// The maximum number of elements that can be read from the advice tape in a single `push`
/// operation.
const ADVICE_READ_LIMIT: u8 = 16;

// NON-DETERMINISTIC (ADVICE) INPUTS
// ================================================================================================

/// Appends the number of `READ` operations specified by the operation's immediate value to the
/// span block. This removes the specified number of items from the advice tape and pushes them
/// onto the stack. The number of items that can be read from the advice tape is limited to 16.
///
/// # Errors
///
/// Returns an `AssemblyError` if the instruction is invalid, malformed, missing a required
/// parameter, or does not match the expected operation. Returns an `invalid_param` `AssemblyError`
/// if the parameter for `adv_push` is not a decimal value.
pub fn adv_push(span: &mut SpanBuilder, n: u8) -> Result<Option<CodeBlock>, AssemblerError> {
    // parse and validate the parameter as the number of items to read from the advice tape
    // it must be between 1 and ADVICE_READ_LIMIT, inclusive, since adv.push.0 is a no-op
    if !(1..=ADVICE_READ_LIMIT).contains(&n) {
        return Err(AssemblerError::imm_out_of_bounds(
            n,
            1,
            ADVICE_READ_LIMIT as u64,
        ));
    }

    // read n items from the advice tape and push then onto the stack
    span.push_op_many(Read, n as usize);

    Ok(None)
}
