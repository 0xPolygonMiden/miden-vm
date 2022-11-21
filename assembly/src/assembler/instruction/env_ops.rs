use super::{
    mem_ops::local_to_absolute_addr, push_felt, AssemblyContext, AssemblyError, CodeBlock, Felt,
    Operation::*, SpanBuilder,
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
pub fn push(imms: &[Felt], span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    imms.iter().copied().for_each(|imm| push_felt(span, imm));
    Ok(None)
}

// ENVIRONMENT INPUTS
// ================================================================================================

/// Appends a sequence of operations to the span needed for executing locaddr.i instruction. This
/// consists of putting i onto the stack and then executing LOCADDR operation.
///
/// # Errors
/// Returns an error if index is greater than the number of procedure locals.
pub fn locaddr(
    span: &mut SpanBuilder,
    index: u16,
    context: &AssemblyContext,
) -> Result<Option<CodeBlock>, AssemblyError> {
    local_to_absolute_addr(span, index, context.num_proc_locals())?;
    Ok(None)
}

/// Appends CALLER operation to the span which puts the hash of the function which initiated the
/// current SYSCALL onto the stack.
///
/// # Errors
/// Returns an error if the instruction is being executed outside of kernel context.
pub fn caller(
    span: &mut SpanBuilder,
    context: &AssemblyContext,
) -> Result<Option<CodeBlock>, AssemblyError> {
    if !context.is_kernel() {
        return Err(AssemblyError::caller_out_of_kernel());
    }

    span.add_op(Caller)
}
