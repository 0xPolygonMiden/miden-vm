use vm_core::{code_blocks::CodeBlock, Felt, FieldElement, Operation::*};

use crate::{
    todo::{context::AssemblyContext, span_builder::SpanBuilder},
    AssemblerError,
};

// CONSTANT INPUTS
// ================================================================================================

/// Appends `PUSH` operations to the span block to push one or more provided constant values onto
/// the stack, up to a maximum of 16 values.
///
/// Constant values may be specified in one of 2 formats:
/// 1. A series of 1-16 valid field elements in decimal or hexadecimal representation separated by
///    periods, e.g. push.0x1234.0xabcd
/// 2. A hexadecimal string without period separators that represents a series of 1-16 elements
///    where the total number of specified bytes is a multiple of 8, e.g.
///    push.0x0000000000001234000000000000abcd
///
/// In cases when the immediate value is 0, `PUSH` operation is replaced with `PAD`. Also, in cases
/// when immediate value is 1, `PUSH` operation is replaced with `PAD INCR` because in most cases
/// this will be more efficient than doing a `PUSH`.
///
/// # Errors
///
/// It will return an error if no immediate value is provided or if any of parameter formats are
/// invalid. It will also return an error if the op token is malformed or doesn't match the expected
/// instruction.
pub fn push(imms: &[Felt], span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblerError> {
    imms.iter().copied().for_each(|imm| {
        if imm == Felt::ZERO {
            span.push_op(Pad);
        } else if imm == Felt::ONE {
            span.push_op(Pad);
            span.push_op(Incr);
        } else {
            span.push_op(Push(imm));
        }
    });

    Ok(None)
}

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
