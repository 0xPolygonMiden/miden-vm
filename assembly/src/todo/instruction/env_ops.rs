use vm_core::{code_blocks::CodeBlock, Felt, Operation::*};

use crate::{
    todo::{context::AssemblyContext, span_builder::SpanBuilder},
    AssemblerError,
};

// ENVIRONMENT INPUTS
// ================================================================================================

/// Appends `locaddr.i` instruction to the span block to push the absolute address of the local
/// variable at index `i` onto the stack.
///
/// # Errors
/// Returns an error if the assembly instruction is malformed or has a parameter value greater than
/// the number of procedure locals.
pub fn locaddr(
    index: &Felt,
    span: &mut SpanBuilder,
    num_proc_locals: u32,
) -> Result<Option<CodeBlock>, AssemblerError> {
    let index = index.inner();
    let max = num_proc_locals as u64 - 1;

    // check that the parameter is within the specified bounds
    if index > max {
        return Err(AssemblerError::imm_out_of_bounds(index, 0, max));
    }

    let value = max - index;
    if value == 0 {
        span.push_op(Pad);
    } else if value == 1 {
        span.push_op(Pad);
        span.push_op(Incr);
    } else {
        span.push_op(Push(value.into()));
    }

    span.add_op(FmpAdd)
}

/// Appends `caller` instruction to the current span block to put the hash of the function which
/// initiated the current SYSCALL onto the stack. `caller` instruction translates directly into
/// CALLER VM operation.
///
/// # Errors
/// Returns an error if:
/// - The assembly instruction is malformed.
/// - The instruction is being executed outside of kernel context.
pub fn caller(
    span: &mut SpanBuilder,
    context: &AssemblyContext,
) -> Result<Option<CodeBlock>, AssemblerError> {
    if !context.is_kernel() {
        return Err(AssemblerError::caller_out_of_kernel());
    }

    span.add_op(Caller)
}
