use super::{AssemblyError, Operation, Token};

// STACK MANIPULATION
// ================================================================================================

/// Translates drop assembly instruction to VM operation DROP.
pub fn parse_drop(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::Drop),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates dropw assembly instruction to VM operations DROP DROP DROP DROP.
pub fn parse_dropw(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            for _ in 0..4 {
                span_ops.push(Operation::Drop);
            }
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates padw assembly instruction to VM operations PAD PAD PAD PAD.
pub fn parse_padw(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            for _ in 0..4 {
                span_ops.push(Operation::Pad);
            }
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates dup.n assembly instruction to VM operations DUPN.
pub fn parse_dup(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::Dup0),
        2 => match op.parts()[1] {
            "0" => span_ops.push(Operation::Dup0),
            "1" => span_ops.push(Operation::Dup1),
            "2" => span_ops.push(Operation::Dup2),
            "3" => span_ops.push(Operation::Dup3),
            "4" => span_ops.push(Operation::Dup4),
            "5" => span_ops.push(Operation::Dup5),
            "6" => span_ops.push(Operation::Dup6),
            "7" => span_ops.push(Operation::Dup7),
            "8" => span_ops.push(Operation::Dup8),
            "9" => span_ops.push(Operation::Dup9),
            "10" => span_ops.push(Operation::Dup10),
            "11" => span_ops.push(Operation::Dup11),
            "12" => span_ops.push(Operation::Dup12),
            "13" => span_ops.push(Operation::Dup13),
            "14" => span_ops.push(Operation::Dup14),
            "15" => span_ops.push(Operation::Dup15),
            _ => return Err(AssemblyError::invalid_param(op, 1)),
        },
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// TODO: implement
pub fn parse_dupw(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_swap(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_swapw(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_movup(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// Translates movupw.x assembly instruction to VM operations.
///
/// Specifically:
/// * movupw.2 is translated into SWAPW SWAPW2
/// * movupw.3 is translated into SWAPW SWAPW2 SWAPW3
pub fn parse_movupw(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0..=1 => return Err(AssemblyError::missing_param(op)),
        2 => match op.parts()[1] {
            "2" => {
                span_ops.push(Operation::SwapW);
                span_ops.push(Operation::SwapW2);
            }
            "3" => {
                span_ops.push(Operation::SwapW);
                span_ops.push(Operation::SwapW2);
                span_ops.push(Operation::SwapW3);
            }
            _ => return Err(AssemblyError::invalid_param(op, 1)),
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    Ok(())
}

/// TODO: implement
pub fn parse_movdn(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// Translates movdnw.x assembly instruction to VM operations.
///
/// Specifically:
/// * movdnw.2 is translated into SWAPW2 SWAPW
/// * movdnw.3 is translated into SWAPW3 SWAPW2 SWAPW
pub fn parse_movdnw(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0..=1 => return Err(AssemblyError::missing_param(op)),
        2 => match op.parts()[1] {
            "2" => {
                span_ops.push(Operation::SwapW2);
                span_ops.push(Operation::SwapW);
            }
            "3" => {
                span_ops.push(Operation::SwapW3);
                span_ops.push(Operation::SwapW2);
                span_ops.push(Operation::SwapW);
            }
            _ => return Err(AssemblyError::invalid_param(op, 1)),
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    Ok(())
}

// CONDITIONAL MANIPULATION
// ================================================================================================

/// TODO: implement
pub fn parse_cswap(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_cswapw(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_cdrop(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_cdropw(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}
