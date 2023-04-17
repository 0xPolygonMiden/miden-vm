use super::{validate_param, AdviceInjector, AssemblyError, Decorator, SpanBuilder};
use crate::ADVICE_READ_LIMIT;
use vm_core::{code_blocks::CodeBlock, Operation::*};

// NON-DETERMINISTIC (ADVICE) INPUTS
// ================================================================================================

/// Appends the number of ADVPOP operations specified by the instruction's immediate value to the
/// span. This pops the specified number of elements from the advice stack and pushes them onto the
/// operand stack.
///
/// # Errors
/// Returns an error if the specified number of values to pushed is smaller than 1 or greater
/// than 16.
pub fn adv_push(span: &mut SpanBuilder, n: u8) -> Result<Option<CodeBlock>, AssemblyError> {
    validate_param(n, 1..=ADVICE_READ_LIMIT)?;
    span.add_op_many(AdvPop, n as usize)
}

// ADVICE INJECTORS
// ================================================================================================

/// Appends adv.mem advice injector to the span. This operation copies n number of words from
/// memory the starting at address a into the advice provider's key-value map.
pub fn adv_mem(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    span.add_decorator(Decorator::Advice(AdviceInjector::Memory))
}
