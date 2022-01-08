use super::{parse_element_param, AssemblyError, BaseElement, FieldElement, Operation, Token};

// ASSERTIONS AND TESTS
// ================================================================================================

/// Appends ASSERT operation to the span block.
///
/// In cases when 'eq' parameter is specified, the sequence of appended operations is: EQ ASSERT
pub fn parse_assert(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        1 => span_ops.push(Operation::Assert),
        2 => {
            if op.parts()[1] == "eq" {
                span_ops.push(Operation::Eq);
                span_ops.push(Operation::Assert);
            } else {
                return Err(AssemblyError::invalid_param(op, 1));
            }
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }
    Ok(())
}

// ARITHMETIC OPERATIONS
// ================================================================================================

/// Appends ADD operation to the span block.
///
/// In cases when one of the parameters is provided via immediate value, the sequence of
/// operations is: PUSH(imm) ADD, unless the imm value is 1, then the operation is just: INCR
pub fn parse_add(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        1 => span_ops.push(Operation::Add),
        2 => {
            let imm = parse_element_param(op, 1)?;
            if imm == BaseElement::ONE {
                span_ops.push(Operation::Incr);
            } else {
                span_ops.push(Operation::Push(imm));
                span_ops.push(Operation::Add);
            }
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }
    Ok(())
}

/// Appends NEG ADD operations to the span block.
///
/// In cases when one of the parameters is provided via immediate value, the sequence of
/// operations is: PUSH(-imm) ADD
pub fn parse_sub(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        1 => {
            span_ops.push(Operation::Neg);
            span_ops.push(Operation::Add);
        }
        2 => {
            let imm = parse_element_param(op, 1)?;
            span_ops.push(Operation::Push(-imm));
            span_ops.push(Operation::Add);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }
    Ok(())
}

/// Appends MUL operation to the span block.
///
/// In cases when one of the parameters is provided via immediate value, the sequence of
/// operations is: PUSH(imm) MUL
pub fn parse_mul(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        1 => span_ops.push(Operation::Mul),
        2 => {
            let imm = parse_element_param(op, 1)?;
            span_ops.push(Operation::Push(imm));
            span_ops.push(Operation::Mul);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }
    Ok(())
}

/// Appends INV MUL operations to the span block.
///
/// In cases when one of the parameters is provided via immediate value, the sequence of
/// operations is: PUSH(imm) INV MUL
pub fn parse_div(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        1 => {
            span_ops.push(Operation::Inv);
            span_ops.push(Operation::Mul);
        }
        2 => {
            let imm = parse_element_param(op, 1)?;
            span_ops.push(Operation::Push(imm.inv()));
            span_ops.push(Operation::Mul);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }
    Ok(())
}

/// Appends NEG operation to the span block.
pub fn parse_neg(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    span_ops.push(Operation::Neg);
    Ok(())
}

/// Appends INV operation to the span block.
pub fn parse_inv(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    span_ops.push(Operation::Inv);
    Ok(())
}

// BOOLEAN OPERATIONS
// ================================================================================================

/// Appends NOT operation to the span block.
pub fn parse_not(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    span_ops.push(Operation::Not);
    Ok(())
}

/// Appends AND operation to the span block.
pub fn parse_and(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    span_ops.push(Operation::And);
    Ok(())
}

/// Appends OR operation to the span block.
pub fn parse_or(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    span_ops.push(Operation::Or);
    Ok(())
}

/// Appends a sequence of operations emulating an XOR operation to the span block.
///
/// The sequence is: DUP0 DUP2 OR MOVDN2 AND NOT AND
pub fn parse_xor(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    span_ops.extend_from_slice(&[
        Operation::Dup0,
        Operation::Dup2,
        Operation::Or,
        Operation::MovDn2,
        Operation::And,
        Operation::Not,
        Operation::And,
    ]);
    Ok(())
}

// COMPARISON OPERATIONS
// ================================================================================================

/// Appends EQ operation to the span block.
///
/// In cases when an immediate values is supplied:
/// - If the immediate value is zero, the appended operation is EQZ
/// - Otherwise, the appended operations are: PUSH(imm) EQ
pub fn parse_eq(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        1 => span_ops.push(Operation::Eq),
        2 => {
            let imm = parse_element_param(op, 1)?;
            if imm == BaseElement::ZERO {
                span_ops.push(Operation::Eqz);
            } else {
                span_ops.push(Operation::Push(imm));
                span_ops.push(Operation::Eq);
            }
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }
    Ok(())
}

/// Appends EQ NOT operation to the span block.
///
/// In cases when an immediate values is supplied:
/// - If the immediate value is zero, the appended operations are: EQZ NOT
/// - Otherwise, the appended operations are: PUSH(imm) EQ NOT
pub fn parse_neq(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        1 => span_ops.push(Operation::Eq),
        2 => {
            let imm = parse_element_param(op, 1)?;
            if imm == BaseElement::ZERO {
                span_ops.push(Operation::Eqz);
            } else {
                span_ops.push(Operation::Push(imm));
                span_ops.push(Operation::Eq);
            }
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }
    span_ops.push(Operation::Not);
    Ok(())
}

// TODO: implement
pub fn parse_eqw(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

// TODO: implement
pub fn parse_lt(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

// TODO: implement
pub fn parse_lte(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

// TODO: implement
pub fn parse_gt(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

// TODO: implement
pub fn parse_gte(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}
