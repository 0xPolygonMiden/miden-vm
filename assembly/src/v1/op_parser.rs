use crate::AssemblyError;
use vm_core::v1::{program::Operation, BaseElement, StarkField};

// OP PARSER
// ================================================================================================

/// Transforms an assembly instruction into a sequence of one or more VM instructions.
pub fn parse_op_token(
    op: &[&str],
    span_ops: &mut Vec<Operation>,
    step: usize,
) -> Result<(), AssemblyError> {
    // based on the instruction, invoke the correct parser for the operation
    match op[0] {
        "noop" => parse_noop(span_ops, op, step),

        "push" => parse_push(span_ops, op, step),

        "add" => parse_add(span_ops, op, step),
        "sub" => parse_sub(span_ops, op, step),
        "mul" => parse_mul(span_ops, op, step),
        "div" => parse_div(span_ops, op, step),
        "neg" => parse_neg(span_ops, op, step),
        "inv" => parse_inv(span_ops, op, step),

        _ => return Err(AssemblyError::invalid_op(op, step)),
    }?;

    Ok(())
}

// GENERAL OPERATIONS
// ================================================================================================

/// Appends a NOOP operations to the span block.
fn parse_noop(
    span_ops: &mut Vec<Operation>,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    span_ops.push(Operation::Noop);
    Ok(())
}

// INPUT OPERATIONS
// ================================================================================================

/// Appends a PUSH operation to the span block.
fn parse_push(
    span_ops: &mut Vec<Operation>,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    let value = read_element(op, step)?;
    span_ops.push(Operation::Push(value));
    Ok(())
}

// FIELD ARITHMETIC
// ================================================================================================

/// Appends ADD operation to the span block.
fn parse_add(span_ops: &mut Vec<Operation>, op: &[&str], step: usize) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    span_ops.push(Operation::Add);
    Ok(())
}

/// Appends NEG ADD operations to the span block.
fn parse_sub(span_ops: &mut Vec<Operation>, op: &[&str], step: usize) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    span_ops.push(Operation::Neg);
    span_ops.push(Operation::Add);
    Ok(())
}

/// Appends MUL operation to the span block.
fn parse_mul(span_ops: &mut Vec<Operation>, op: &[&str], step: usize) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    span_ops.push(Operation::Mul);
    Ok(())
}

/// Appends INV MUL operations to the span block.
fn parse_div(span_ops: &mut Vec<Operation>, op: &[&str], step: usize) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    span_ops.push(Operation::Inv);
    span_ops.push(Operation::Mul);
    Ok(())
}

/// Appends NEG operation to the span block.
fn parse_neg(span_ops: &mut Vec<Operation>, op: &[&str], step: usize) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    span_ops.push(Operation::Neg);
    Ok(())
}

/// Appends INV operation to the span block.
fn parse_inv(span_ops: &mut Vec<Operation>, op: &[&str], step: usize) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    span_ops.push(Operation::Inv);
    Ok(())
}

// HELPER FUNCTIONS
// ================================================================================================

fn read_element(op: &[&str], step: usize) -> Result<BaseElement, AssemblyError> {
    // make sure exactly 1 parameter was supplied
    if op.len() == 1 {
        return Err(AssemblyError::missing_param(op, step));
    } else if op.len() > 2 {
        return Err(AssemblyError::extra_param(op, step));
    }

    let result = if op[1].starts_with("0x") {
        // parse hexadecimal number
        match u64::from_str_radix(&op[1][2..], 16) {
            Ok(i) => i,
            Err(_) => return Err(AssemblyError::invalid_param(op, step)),
        }
    } else {
        // parse decimal number
        match op[1].parse::<u64>() {
            Ok(i) => i,
            Err(_) => return Err(AssemblyError::invalid_param(op, step)),
        }
    };

    // make sure the value is a valid field element
    if result >= BaseElement::MODULUS {
        return Err(AssemblyError::invalid_param_reason(
            op,
            step,
            format!(
                "parameter value must be smaller than {}",
                BaseElement::MODULUS
            ),
        ));
    }

    Ok(BaseElement::new(result))
}
