use super::{parse_int_param, AssemblyError, BaseElement, Operation, Token};

// CONVERSIONS AND TESTS
// ================================================================================================

/// Translates u32test assembly instruction to VM operation U32SPLIT EQZ.
pub fn parse_u32test(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            span_ops.push(Operation::Dup0);
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::Drop);
            span_ops.push(Operation::Eqz);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// TODO: implement
pub fn parse_u32testw(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// Translates u32assert assembly instruction to VM operation U32SPLIT EQZ ASSERT.
pub fn parse_u32assert(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Eqz);
            span_ops.push(Operation::Assert);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// TODO: implement
pub fn parse_u32assertw(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// Translates u32cast assembly instruction to VM operation U32SPLIT DROP.
pub fn parse_u32cast(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Drop);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32split assembly instruction to VM operation U32SPLIT.
pub fn parse_u32split(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32split),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

// ARITHMETIC OPERATIONS
// ================================================================================================

/// Translates u32add assembly instruction to VM operation U32ADD.
pub fn parse_u32add(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32add),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32addc assembly instruction to VM operation U32ADDC.
pub fn parse_u32addc(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32addc),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32sub assembly instruction to VM operation U32SUB.
pub fn parse_u32sub(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32sub),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32mul assembly instruction to VM operation U32MUL.
pub fn parse_u32mul(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32mul),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32madd assembly instruction to VM operation U32MADD.
pub fn parse_u32madd(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32madd),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32div assembly instruction to VM operation U32DIV.
pub fn parse_u32div(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32div),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32mod assembly instruction to VM operation U32DIV DROP.
pub fn parse_u32mod(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            span_ops.push(Operation::U32div);
            span_ops.push(Operation::Drop);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

// BITWISE OPERATIONS
// ================================================================================================

/// Translates u32and assembly instruction to VM operation U32AND.
pub fn parse_u32and(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32and),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32or assembly instruction to VM operation U32OR.
pub fn parse_u32or(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32or),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32xor assembly instruction to VM operation U32XOR.
pub fn parse_u32xor(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32xor),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32not assembly instruction to VM operation PUSH(2^32) SWAP INCR U32SUB.
/// The reason this works is because 2^32 provides a bit mask of ones, which after
/// subtracting the element, flips the bits of the original value to perform a bitwise NOT.
pub fn parse_u32not(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            // Assert the value is u32
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Not);
            span_ops.push(Operation::Assert);

            span_ops.push(Operation::Push(BaseElement::new(2u64.pow(32))));
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::Incr);
            span_ops.push(Operation::U32sub);

            // Drop the underflow flag
            span_ops.push(Operation::Drop);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32shl.x assembly instruction to VM operation PUSH(2^x) MUL U32SPLIT DROP.
pub fn parse_u32shl(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0..=1 => return Err(AssemblyError::missing_param(op)),
        2 => {
            // Assert the value is u32
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Not);
            span_ops.push(Operation::Assert);

            let x = parse_int_param(op, 1, 1, 31)?;
            span_ops.push(Operation::Push(BaseElement::new(2u64.pow(x))));
            span_ops.push(Operation::Mul);
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Drop);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32shl.x assembly instruction to VM operation PUSH(2^x) U32DIV SWAP DROP.
pub fn parse_u32shr(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0..=1 => return Err(AssemblyError::missing_param(op)),
        2 => {
            // Assert the value is u32
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Not);
            span_ops.push(Operation::Assert);

            let x = parse_int_param(op, 1, 1, 31)?;
            span_ops.push(Operation::Push(BaseElement::new(2u64.pow(x))));
            span_ops.push(Operation::U32div);
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::Drop);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// TODO: implement
pub fn parse_u32rotl(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_u32rotr(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_u32revb(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

// COMPARISON OPERATIONS
// ================================================================================================

/// Translates u32eq assembly instruction to VM operations.
/// Specifically we test the first two numbers to be u32 (U32SPLIT NOT ASSERT),
/// then perform a EQ to check the equality.
pub fn parse_u32eq(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            // Check first number is u32
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Not);
            span_ops.push(Operation::Assert);

            span_ops.push(Operation::Swap);

            // Check second number is u32
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Not);
            span_ops.push(Operation::Assert);

            span_ops.push(Operation::Eq);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32neq assembly instruction to VM operations.
/// Specifically we test the first two numbers to be u32 (U32SPLIT NOT ASSERT),
/// then perform a EQ NOT to check the equality.
pub fn parse_u32neq(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            // Check first number is u32
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Not);
            span_ops.push(Operation::Assert);

            span_ops.push(Operation::Swap);

            // Check second number is u32
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Not);
            span_ops.push(Operation::Assert);

            span_ops.push(Operation::Eq);
            span_ops.push(Operation::Not);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// TODO: implement
pub fn parse_u32lt(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_u32lte(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_u32gt(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_u32gte(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_u32min(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_u32max(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}
