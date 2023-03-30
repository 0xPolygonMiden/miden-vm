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
    span.push_op_many(AdvPop, n as usize);
    Ok(None)
}

// ADVICE INJECTORS
// ================================================================================================

/// Appends adv.mem.a.n advice injector to the span. This operation copies n number of words from
/// memory the starting at address a into the advice provider's key-value map.
///
/// # Errors
/// Returns an error is start_addr + num_words > u32::MAX.
pub fn adv_mem(
    span: &mut SpanBuilder,
    start_addr: u32,
    num_words: u32,
) -> Result<Option<CodeBlock>, AssemblyError> {
    validate_param(num_words, 0..=(u32::MAX - start_addr))?;
    span.add_decorator(Decorator::Advice(AdviceInjector::Memory(start_addr, num_words)))
}
