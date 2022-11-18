use super::{
    super::parse_bit_len_param, super::parse_element_param, AssemblyError, Instruction, Node,
    Token, Vec,
};
use crate::validate_operation;

/// Returns `Assert` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_assert(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::Assert))
}

/// Returns `AssertEq` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_assert_eq(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::AssertEq))
}

/// Returns `Assertz` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_assertz(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::Assertz))
}

/// Returns `Add` node instruction if no immediate vaule is provided or `AddImm` otherwise
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param or more than one param
pub fn parse_add(op: &Token) -> Result<Node, AssemblyError> {
    let instruction = match op.num_parts() {
        1 => Instruction::Add,
        2 => {
            let imm = parse_element_param(op, 1)?;
            Instruction::AddImm(imm)
        }
        _ => return Err(AssemblyError::extra_param(op)),
    };
    Ok(Node::Instruction(instruction))
}

/// Returns `Sub` node instruction if no immediate vaule is provided or `SubImm` otherwise
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param or more than one param
pub fn parse_sub(op: &Token) -> Result<Node, AssemblyError> {
    let instruction = match op.num_parts() {
        1 => Instruction::Sub,
        2 => {
            let imm = parse_element_param(op, 1)?;
            Instruction::SubImm(imm)
        }
        _ => return Err(AssemblyError::extra_param(op)),
    };
    Ok(Node::Instruction(instruction))
}

/// Returns `Mul` node instruction if no immediate vaule is provided or `MulImm` otherwise
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param or more than one param
pub fn parse_mul(op: &Token) -> Result<Node, AssemblyError> {
    let instruction = match op.num_parts() {
        1 => Instruction::Mul,
        2 => {
            let imm = parse_element_param(op, 1)?;
            Instruction::MulImm(imm)
        }
        _ => return Err(AssemblyError::extra_param(op)),
    };
    Ok(Node::Instruction(instruction))
}

/// Returns `Div` node instruction if no immediate vaule is provided or `DivImm` otherwise
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param or more than one param
pub fn parse_div(op: &Token) -> Result<Node, AssemblyError> {
    let instruction = match op.num_parts() {
        1 => Instruction::Div,
        2 => {
            let imm = parse_element_param(op, 1)?;
            Instruction::DivImm(imm)
        }
        _ => return Err(AssemblyError::extra_param(op)),
    };
    Ok(Node::Instruction(instruction))
}

/// Returns `Neg` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_neg(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::Neg))
}

/// Returns `Inv` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_inv(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::Inv))
}

/// Returns `Pow2` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_pow2(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::Pow2))
}

/// Returns `Exp` node instruction if no immediate vaule is provided, otherwise returns instruction
/// `ExpImm` or `ExpBitLength` depending on what immediate value was passed
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param or more than one param
pub fn parse_exp(op: &Token) -> Result<Node, AssemblyError> {
    let instruction = match op.num_parts() {
        1 => Instruction::Exp,
        2 => {
            let param_value = op.parts()[1];

            if param_value.strip_prefix('u').is_some() {
                // parse the bits length of the exponent from the immediate value.
                let bits_len = parse_bit_len_param(op, 1)?;

                // the specified bits length can not be more than 64 bits.
                if bits_len > 64 {
                    return Err(AssemblyError::invalid_param_with_reason(
                        op,
                        1,
                        format!("parameter can at max be a u64 but found u{}", bits_len).as_str(),
                    ));
                }

                Instruction::ExpBitLength(bits_len as u8)
            } else {
                // parse immediate value.
                let imm = parse_element_param(op, 1)?;
                Instruction::ExpImm(imm)
            }
        }
        _ => return Err(AssemblyError::extra_param(op)),
    };

    Ok(Node::Instruction(instruction))
}

/// Returns `Not` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_not(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::Not))
}

/// Returns `And` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_and(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::And))
}

/// Returns `Or` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_or(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::Or))
}

/// Returns `Xor` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_xor(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::Xor))
}

/// Returns `Eq` node instruction if no immediate vaule is provided or `EqImm` otherwise
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param or more than one param
pub fn parse_eq(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "eq", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_element_param(op, 1)?;
            Node::Instruction(Instruction::EqImm(value))
        }
        _ => Node::Instruction(Instruction::Eq),
    };

    Ok(node)
}

/// Returns `Neq` node instruction if no immediate vaule is provided or `NeqImm` otherwise
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param or more than one param
pub fn parse_neq(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "neq", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_element_param(op, 1)?;
            Node::Instruction(Instruction::NeqImm(value))
        }
        _ => Node::Instruction(Instruction::Neq),
    };

    Ok(node)
}

/// Returns `Lt` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_lt(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::Lt))
}

/// Returns `Lte` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_lte(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::Lte))
}

/// Returns `Gt` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_gt(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::Gt))
}

/// Returns `Gte` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_gte(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::Gte))
}

/// Returns `Eqw` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_eqw(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::Eqw))
}
