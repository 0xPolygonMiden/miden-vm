use super::{parse_element_param, AssemblyError, BaseElement, FieldElement, Operation, Token};

// INPUT OPERATIONS
// ================================================================================================

/// Appends a PUSH operation to the span block.
pub fn parse_push(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    let value = parse_element_param(op, 1)?;
    if value == BaseElement::ZERO {
        span_ops.push(Operation::Pad);
    } else if value == BaseElement::ONE {
        span_ops.push(Operation::Pad);
        span_ops.push(Operation::Incr);
    } else {
        span_ops.push(Operation::Push(value));
    }
    Ok(())
}
