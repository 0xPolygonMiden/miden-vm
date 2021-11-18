use super::{AssemblyError, Token};
use vm_core::v1::{program::Operation, BaseElement, StarkField};

// OP PARSER
// ================================================================================================

/// Transforms an assembly instruction into a sequence of one or more VM instructions.
pub fn parse_op_token(op: &Token, span_ops: &mut Vec<Operation>) -> Result<(), AssemblyError> {
    // based on the instruction, invoke the correct parser for the operation
    match op.parts()[0] {
        "push" => parse_push(span_ops, op),

        "add" => parse_add(span_ops, op),
        "sub" => parse_sub(span_ops, op),
        "mul" => parse_mul(span_ops, op),
        "div" => parse_div(span_ops, op),
        "neg" => parse_neg(span_ops, op),
        "inv" => parse_inv(span_ops, op),

        _ => return Err(AssemblyError::invalid_op(op)),
    }?;

    Ok(())
}

// INPUT OPERATIONS
// ================================================================================================

/// Appends a PUSH operation to the span block.
fn parse_push(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    let value = read_element(op)?;
    span_ops.push(Operation::Push(value));
    Ok(())
}

// FIELD ARITHMETIC
// ================================================================================================

/// Appends ADD operation to the span block.
fn parse_add(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    span_ops.push(Operation::Add);
    Ok(())
}

/// Appends NEG ADD operations to the span block.
fn parse_sub(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    span_ops.push(Operation::Neg);
    span_ops.push(Operation::Add);
    Ok(())
}

/// Appends MUL operation to the span block.
fn parse_mul(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    span_ops.push(Operation::Mul);
    Ok(())
}

/// Appends INV MUL operations to the span block.
fn parse_div(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    span_ops.push(Operation::Inv);
    span_ops.push(Operation::Mul);
    Ok(())
}

/// Appends NEG operation to the span block.
fn parse_neg(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    span_ops.push(Operation::Neg);
    Ok(())
}

/// Appends INV operation to the span block.
fn parse_inv(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    span_ops.push(Operation::Inv);
    Ok(())
}

// HELPER FUNCTIONS
// ================================================================================================

fn read_element(op: &Token) -> Result<BaseElement, AssemblyError> {
    // make sure exactly 1 parameter was supplied
    if op.num_parts() == 1 {
        return Err(AssemblyError::missing_param(op));
    } else if op.num_parts() > 2 {
        return Err(AssemblyError::extra_param(op));
    }

    let result = if op.parts()[1].starts_with("0x") {
        // parse hexadecimal number
        match u64::from_str_radix(&op.parts()[1][2..], 16) {
            Ok(i) => i,
            Err(_) => return Err(AssemblyError::invalid_param(op, 1)),
        }
    } else {
        // parse decimal number
        match op.parts()[1].parse::<u64>() {
            Ok(i) => i,
            Err(_) => return Err(AssemblyError::invalid_param(op, 1)),
        }
    };

    // make sure the value is a valid field element
    if result >= BaseElement::MODULUS {
        return Err(AssemblyError::invalid_param_with_reason(
            op,
            1,
            format!(
                "parameter value must be smaller than {}",
                BaseElement::MODULUS
            )
            .as_str(),
        ));
    }

    Ok(BaseElement::new(result))
}
