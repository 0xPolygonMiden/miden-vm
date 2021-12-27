use super::{AssemblyError, Operation, Token};

// STACK MANIPULATION
// ================================================================================================

/// TODO: implement
pub fn parse_drop(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_dropw(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_padw(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_dup(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
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

pub fn parse_movupw(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    if _op.num_parts() != 2 {
        return Err(AssemblyError::extra_param(_op))
    }

    match _op.parts()[1] {
        "2" => {
            _span_ops.push(Operation::SwapW);
            _span_ops.push(Operation::SwapW2);
        }
        "3" => {
            _span_ops.push(Operation::SwapW);
            _span_ops.push(Operation::SwapW2);
            _span_ops.push(Operation::SwapW3);
        }
        _ => return Err(AssemblyError::extra_param(_op)),
    }

    Ok(())
}

/// TODO: implement
pub fn parse_movdn(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_movdnw(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
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
