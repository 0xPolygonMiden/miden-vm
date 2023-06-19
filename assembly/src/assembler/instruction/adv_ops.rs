use super::{validate_param, AssemblyError, SpanBuilder};
use crate::{ast::AdviceInjectorNode, ADVICE_READ_LIMIT};
use vm_core::{code_blocks::CodeBlock, Operation};

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
    span.push_op_many(Operation::AdvPop, n as usize);
    Ok(None)
}

// ADVICE INJECTORS
// ================================================================================================

/// Appends advice injector decorator to the span.
pub fn adv_inject(
    span: &mut SpanBuilder,
    injector: &AdviceInjectorNode,
) -> Result<Option<CodeBlock>, AssemblyError> {
    span.push_advice_injector(injector.into());
    Ok(None)
}
