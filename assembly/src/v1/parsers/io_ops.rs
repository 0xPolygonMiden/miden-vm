use super::{parse_element_param, AssemblyError, BaseElement, FieldElement, Operation, Token};

// CONSTANT INPUTS
// ================================================================================================

/// Appends a PUSH operation to the span block.
///
/// In cases when the immediate value is 0, PUSH operation is replaced with PAD. Also, in cases
/// when immediate value is 1, PUSH operation is replaced with PAD INCR because in most cases
/// this will be more efficient than doing a PUSH.
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

/// TODO: implement
pub fn parse_pushw(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

// ENVIRONMENT INPUTS
// ================================================================================================

/// TODO: implement
pub fn parse_env(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

// NON-DETERMINISTIC INPUTS
// ================================================================================================

/// TODO: implement
pub fn parse_read(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_readw(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

// RANDOM ACCESS MEMORY
// ================================================================================================

/// TODO: implement
pub fn parse_mem(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}
