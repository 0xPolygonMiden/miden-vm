use super::{parse_param, Instruction, Node};
use crate::{validate_operation, AssemblyError, Token, Vec};

pub fn parse_u32assert(op: &Token) -> Result<Node, AssemblyError> {
    let instruction = match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => Instruction::U32Assert,
        2 => match op.parts()[1] {
            "1" => Instruction::U32Assert,
            "2" => Instruction::U32Assert2,
            _ => return Err(AssemblyError::invalid_param(op, 1)),
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    Ok(Node::Instruction(instruction))
}

pub fn parse_u32checked_add(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "u32checked_add", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Node::Instruction(Instruction::U32CheckedAddImm(value))
        }
        _ => Node::Instruction(Instruction::U32CheckedAdd),
    };

    Ok(node)
}

pub fn parse_u32wrapping_add(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "u32wrapping_add", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Node::Instruction(Instruction::U32WrappingAddImm(value))
        }
        _ => Node::Instruction(Instruction::U32WrappingAdd),
    };

    Ok(node)
}

pub fn parse_u32overflowing_add(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "u32overflowing_add", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Node::Instruction(Instruction::U32OverflowingAddImm(value))
        }
        _ => Node::Instruction(Instruction::U32OverflowingAdd),
    };

    Ok(node)
}

pub fn parse_u32checked_sub(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "u32checked_sub", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Node::Instruction(Instruction::U32CheckedSubImm(value))
        }
        _ => Node::Instruction(Instruction::U32CheckedSub),
    };

    Ok(node)
}

pub fn parse_u32wrapping_sub(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "u32wrapping_sub", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Node::Instruction(Instruction::U32WrappingSubImm(value))
        }
        _ => Node::Instruction(Instruction::U32WrappingSub),
    };

    Ok(node)
}

pub fn parse_u32overflowing_sub(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "u32overflowing_sub", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Node::Instruction(Instruction::U32OverflowingSubImm(value))
        }
        _ => Node::Instruction(Instruction::U32OverflowingSub),
    };

    Ok(node)
}

pub fn parse_u32checked_mul(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "u32checked_mul", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Node::Instruction(Instruction::U32CheckedMulImm(value))
        }
        _ => Node::Instruction(Instruction::U32CheckedMul),
    };

    Ok(node)
}

pub fn parse_u32wrapping_mul(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "u32wrapping_mul", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Node::Instruction(Instruction::U32WrappingMulImm(value))
        }
        _ => Node::Instruction(Instruction::U32WrappingMul),
    };

    Ok(node)
}

pub fn parse_u32overflowing_mul(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "u32overflowing_mul", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Node::Instruction(Instruction::U32OverflowingMulImm(value))
        }
        _ => Node::Instruction(Instruction::U32OverflowingMul),
    };

    Ok(node)
}

pub fn parse_u32_div(op: &Token, checked: bool) -> Result<Node, AssemblyError> {
    validate_operation!(op, "u32checked_div|u32unchecked_div", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            if checked {
                Node::Instruction(Instruction::U32CheckedDivImm(value))
            } else {
                Node::Instruction(Instruction::U32UncheckedDivImm(value))
            }
        }
        _ => {
            if checked {
                Node::Instruction(Instruction::U32CheckedDiv)
            } else {
                Node::Instruction(Instruction::U32UncheckedDiv)
            }
        }
    };

    Ok(node)
}

pub fn parse_u32_mod(op: &Token, checked: bool) -> Result<Node, AssemblyError> {
    validate_operation!(op, "u32checked_mod|u32unchecked_mod", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            if checked {
                Node::Instruction(Instruction::U32CheckedModImm(value))
            } else {
                Node::Instruction(Instruction::U32UncheckedModImm(value))
            }
        }
        _ => {
            if checked {
                Node::Instruction(Instruction::U32CheckedMod)
            } else {
                Node::Instruction(Instruction::U32UncheckedMod)
            }
        }
    };

    Ok(node)
}

pub fn parse_u32_divmod(op: &Token, checked: bool) -> Result<Node, AssemblyError> {
    validate_operation!(op, "u32checked_divmod|u32unchecked_divmod", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            if checked {
                Node::Instruction(Instruction::U32CheckedDivModImm(value))
            } else {
                Node::Instruction(Instruction::U32UncheckedDivModImm(value))
            }
        }
        _ => {
            if checked {
                Node::Instruction(Instruction::U32CheckedDivMod)
            } else {
                Node::Instruction(Instruction::U32UncheckedDivMod)
            }
        }
    };

    Ok(node)
}

pub fn parse_u32_shr(op: &Token, checked: bool) -> Result<Node, AssemblyError> {
    validate_operation!(op, "u32checked_shr|u32unchecked_shr", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_param::<u8>(op, 1)?;
            if checked {
                Node::Instruction(Instruction::U32CheckedShrImm(value))
            } else {
                Node::Instruction(Instruction::U32UncheckedShrImm(value))
            }
        }
        _ => {
            if checked {
                Node::Instruction(Instruction::U32CheckedShr)
            } else {
                Node::Instruction(Instruction::U32UncheckedShr)
            }
        }
    };

    Ok(node)
}

pub fn parse_u32_shl(op: &Token, checked: bool) -> Result<Node, AssemblyError> {
    validate_operation!(op, "u32checked_shl|u32unchecked_shl", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_param::<u8>(op, 1)?;
            if checked {
                Node::Instruction(Instruction::U32CheckedShlImm(value))
            } else {
                Node::Instruction(Instruction::U32UncheckedShlImm(value))
            }
        }
        _ => {
            if checked {
                Node::Instruction(Instruction::U32CheckedShl)
            } else {
                Node::Instruction(Instruction::U32UncheckedShl)
            }
        }
    };

    Ok(node)
}

pub fn parse_u32_rotr(op: &Token, checked: bool) -> Result<Node, AssemblyError> {
    validate_operation!(op, "u32checked_rotr|u32unchecked_rotr", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_param::<u8>(op, 1)?;
            if checked {
                Node::Instruction(Instruction::U32CheckedRotrImm(value))
            } else {
                Node::Instruction(Instruction::U32UncheckedRotrImm(value))
            }
        }
        _ => {
            if checked {
                Node::Instruction(Instruction::U32CheckedRotr)
            } else {
                Node::Instruction(Instruction::U32UncheckedRotr)
            }
        }
    };

    Ok(node)
}

pub fn parse_u32_rotl(op: &Token, checked: bool) -> Result<Node, AssemblyError> {
    validate_operation!(op, "u32checked_rotl|u32unchecked_rotl", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_param::<u8>(op, 1)?;
            if checked {
                Node::Instruction(Instruction::U32CheckedRotlImm(value))
            } else {
                Node::Instruction(Instruction::U32UncheckedRotlImm(value))
            }
        }
        _ => {
            if checked {
                Node::Instruction(Instruction::U32CheckedRotl)
            } else {
                Node::Instruction(Instruction::U32UncheckedRotl)
            }
        }
    };

    Ok(node)
}
