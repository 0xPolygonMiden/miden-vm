use super::{AssemblyError, Instruction, Node, Vec};
use crate::{validate_operation, Token};

/// Returns `Drop` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_drop(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::Drop))
}

/// Returns `DropW` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_dropw(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::DropW))
}

/// Returns `PadW` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_padw(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::PadW))
}

/// Returns `Dup0` node instruction if no immediate vaule is provided or one of the
/// `Dup1` — `Dup15` instructions according to the immediate value
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param or more than one param
pub fn parse_dup(op: &Token) -> Result<Node, AssemblyError> {
    let node = match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => Node::Instruction(Instruction::Dup0),
        2 => match op.parts()[1] {
            "0" => Node::Instruction(Instruction::Dup0),
            "1" => Node::Instruction(Instruction::Dup1),
            "2" => Node::Instruction(Instruction::Dup2),
            "3" => Node::Instruction(Instruction::Dup3),
            "4" => Node::Instruction(Instruction::Dup4),
            "5" => Node::Instruction(Instruction::Dup5),
            "6" => Node::Instruction(Instruction::Dup6),
            "7" => Node::Instruction(Instruction::Dup7),
            "8" => Node::Instruction(Instruction::Dup8),
            "9" => Node::Instruction(Instruction::Dup9),
            "10" => Node::Instruction(Instruction::Dup10),
            "11" => Node::Instruction(Instruction::Dup11),
            "12" => Node::Instruction(Instruction::Dup12),
            "13" => Node::Instruction(Instruction::Dup13),
            "14" => Node::Instruction(Instruction::Dup14),
            "15" => Node::Instruction(Instruction::Dup15),
            _ => return Err(AssemblyError::invalid_param(op, 1)),
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    Ok(node)
}

/// Returns `DupW0` node instruction if no immediate vaule is provided or one of the
/// `DupW1` — `DupW3` instructions according to the immediate value
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param or more than one param
pub fn parse_dupw(op: &Token) -> Result<Node, AssemblyError> {
    let node = match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => Node::Instruction(Instruction::DupW0),
        2 => match op.parts()[1] {
            "0" => Node::Instruction(Instruction::DupW0),
            "1" => Node::Instruction(Instruction::DupW1),
            "2" => Node::Instruction(Instruction::DupW2),
            "3" => Node::Instruction(Instruction::DupW3),
            _ => return Err(AssemblyError::invalid_param(op, 1)),
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    Ok(node)
}

/// Returns `Swap` node instruction if no immediate vaule is provided or one of the
/// `Swap` — `Swap15` instructions according to the immediate value
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param or more than one param
pub fn parse_swap(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "swap", 0..1);

    let node = match op.num_parts() {
        1 => Node::Instruction(Instruction::Swap),
        2 => match op.parts()[1] {
            "1" => Node::Instruction(Instruction::Swap),
            "2" => Node::Instruction(Instruction::Swap2),
            "3" => Node::Instruction(Instruction::Swap3),
            "4" => Node::Instruction(Instruction::Swap4),
            "5" => Node::Instruction(Instruction::Swap5),
            "6" => Node::Instruction(Instruction::Swap6),
            "7" => Node::Instruction(Instruction::Swap7),
            "8" => Node::Instruction(Instruction::Swap8),
            "9" => Node::Instruction(Instruction::Swap9),
            "10" => Node::Instruction(Instruction::Swap10),
            "11" => Node::Instruction(Instruction::Swap11),
            "12" => Node::Instruction(Instruction::Swap12),
            "13" => Node::Instruction(Instruction::Swap13),
            "14" => Node::Instruction(Instruction::Swap14),
            "15" => Node::Instruction(Instruction::Swap15),
            _ => return Err(AssemblyError::invalid_param(op, 1)),
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    Ok(node)
}

/// Returns `SwapW` node instruction if no immediate vaule is provided or one of the
/// `SwapW` — `SwapW3` instructions according to the immediate value
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param or more than one param
pub fn parse_swapw(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "swapw", 0..1);

    let node = match op.num_parts() {
        1 => Node::Instruction(Instruction::SwapW),
        2 => match op.parts()[1] {
            "1" => Node::Instruction(Instruction::SwapW),
            "2" => Node::Instruction(Instruction::SwapW2),
            "3" => Node::Instruction(Instruction::SwapW3),
            _ => return Err(AssemblyError::invalid_param(op, 1)),
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    Ok(node)
}

/// Returns `SwapDW` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_swapdw(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::SwapDW))
}

/// Returns one of the `MovUp2` — `MovUp15` instructions according to the immediate value
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param, no params or more than one
/// param
pub fn parse_movup(op: &Token) -> Result<Node, AssemblyError> {
    let node = match op.num_parts() {
        0..=1 => return Err(AssemblyError::missing_param(op)),
        2 => match op.parts()[1] {
            "2" => Node::Instruction(Instruction::MovUp2),
            "3" => Node::Instruction(Instruction::MovUp3),
            "4" => Node::Instruction(Instruction::MovUp4),
            "5" => Node::Instruction(Instruction::MovUp5),
            "6" => Node::Instruction(Instruction::MovUp6),
            "7" => Node::Instruction(Instruction::MovUp7),
            "8" => Node::Instruction(Instruction::MovUp8),
            "9" => Node::Instruction(Instruction::MovUp9),
            "10" => Node::Instruction(Instruction::MovUp10),
            "11" => Node::Instruction(Instruction::MovUp11),
            "12" => Node::Instruction(Instruction::MovUp12),
            "13" => Node::Instruction(Instruction::MovUp13),
            "14" => Node::Instruction(Instruction::MovUp14),
            "15" => Node::Instruction(Instruction::MovUp15),
            _ => return Err(AssemblyError::invalid_param(op, 1)),
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    Ok(node)
}

/// Returns one of the `MovDn2` — `MovDn15` instructions according to the immediate value
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param, no params or more than one
/// param
pub fn parse_movdn(op: &Token) -> Result<Node, AssemblyError> {
    let node = match op.num_parts() {
        0..=1 => return Err(AssemblyError::missing_param(op)),
        2 => match op.parts()[1] {
            "2" => Node::Instruction(Instruction::MovDn2),
            "3" => Node::Instruction(Instruction::MovDn3),
            "4" => Node::Instruction(Instruction::MovDn4),
            "5" => Node::Instruction(Instruction::MovDn5),
            "6" => Node::Instruction(Instruction::MovDn6),
            "7" => Node::Instruction(Instruction::MovDn7),
            "8" => Node::Instruction(Instruction::MovDn8),
            "9" => Node::Instruction(Instruction::MovDn9),
            "10" => Node::Instruction(Instruction::MovDn10),
            "11" => Node::Instruction(Instruction::MovDn11),
            "12" => Node::Instruction(Instruction::MovDn12),
            "13" => Node::Instruction(Instruction::MovDn13),
            "14" => Node::Instruction(Instruction::MovDn14),
            "15" => Node::Instruction(Instruction::MovDn15),
            _ => return Err(AssemblyError::invalid_param(op, 1)),
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    Ok(node)
}

/// Returns `MovUpW2` or `MovUpW3` instructions according to the immediate value
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param, no params or more than one
/// param
pub fn parse_movupw(op: &Token) -> Result<Node, AssemblyError> {
    let node = match op.num_parts() {
        0..=1 => return Err(AssemblyError::missing_param(op)),
        2 => match op.parts()[1] {
            "2" => Node::Instruction(Instruction::MovUpW2),
            "3" => Node::Instruction(Instruction::MovUpW3),
            _ => return Err(AssemblyError::invalid_param(op, 1)),
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    Ok(node)
}

/// Returns `MovDnW2` or `MovDnW3` instructions according to the immediate value
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param, no params or more than one
/// param
pub fn parse_movdnw(op: &Token) -> Result<Node, AssemblyError> {
    let node = match op.num_parts() {
        0..=1 => return Err(AssemblyError::missing_param(op)),
        2 => match op.parts()[1] {
            "2" => Node::Instruction(Instruction::MovDnW2),
            "3" => Node::Instruction(Instruction::MovDnW3),
            _ => return Err(AssemblyError::invalid_param(op, 1)),
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    Ok(node)
}

/// Returns `CSwap` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_cswap(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::CSwap))
}

/// Returns `CSwapW` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_cswapw(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::CSwapW))
}

/// Returns `CDrop` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_cdrop(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::CDrop))
}

/// Returns `CDropW` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_cdropw(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::CDropW))
}
