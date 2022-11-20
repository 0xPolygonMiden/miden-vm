use super::{
    parse_element_param,
    Instruction::*,
    Node::{self, Instruction},
    ParsingError, Token,
};

// INSTRUCTION PARSERS
// ================================================================================================

/// Returns `Add` instruction node if no immediate value is provided or `AddImm` instruction
/// node otherwise.
///
/// # Errors
/// Returns an error if the instruction token has invalid param or more than one param.
pub fn parse_add(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "add");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(Add)),
        2 => {
            let imm = parse_element_param(op, 1)?;
            Ok(Instruction(AddImm(imm)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `Sub` instruction node if no immediate value is provided or `SubImm` instruction
/// node otherwise.
///
/// # Errors
/// Returns an error if the instruction token has invalid param or more than one param.
pub fn parse_sub(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "sub");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(Sub)),
        2 => {
            let imm = parse_element_param(op, 1)?;
            Ok(Instruction(SubImm(imm)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `Mul` instruction node if no immediate value is provided or `MulImm` instruction
/// node otherwise.
///
/// # Errors
/// Returns an error if the instruction token has invalid param or more than one param.
pub fn parse_mul(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "mul");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(Mul)),
        2 => {
            let imm = parse_element_param(op, 1)?;
            Ok(Instruction(MulImm(imm)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `Div` instruction node if no immediate value is provided or `DivImm` instruction
/// node otherwise.
///
/// # Errors
/// Returns an error if the instruction token has invalid param or more than one param
pub fn parse_div(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "div");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(Div)),
        2 => {
            let imm = parse_element_param(op, 1)?;
            Ok(Instruction(DivImm(imm)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `Exp` instruction node if no immediate value is provided, otherwise returns `ExpImm`
/// or `ExpBitLength` instruction node depending on the immediate value provided.
///
/// # Errors
/// Returns an error if the instruction token has invalid param or more than one param
pub fn parse_exp(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "exp");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(Exp)),
        2 => {
            let param_value = op.parts()[1];

            if param_value.strip_prefix('u').is_some() {
                // parse the bits length of the exponent from the immediate value.
                let bits_len = parse_bit_len_param(op, 1)?;

                // the specified bits length can not be more than 64 bits.
                if bits_len > 64 {
                    return Err(ParsingError::invalid_param_with_reason(
                        op,
                        1,
                        format!("parameter can at max be a u64 but found u{}", bits_len).as_str(),
                    ));
                }

                Ok(Instruction(ExpBitLength(bits_len)))
            } else {
                // parse immediate value.
                let imm = parse_element_param(op, 1)?;
                Ok(Instruction(ExpImm(imm)))
            }
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `Eq` instruction node, if no immediate value is provided or `EqImm` instruction
/// node otherwise.
///
/// # Errors
/// Returns an error if the instruction token has invalid param or more than one param.
pub fn parse_eq(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "eq");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(Eq)),
        2 => {
            let imm = parse_element_param(op, 1)?;
            Ok(Instruction(EqImm(imm)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `Neq` instruction node if no immediate value is provided or `NeqImm` instruction
/// node otherwise.
///
/// # Errors
/// Returns an error if the instruction token has invalid param or more than one param.
pub fn parse_neq(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "neq");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(Neq)),
        2 => {
            let imm = parse_element_param(op, 1)?;
            Ok(Instruction(NeqImm(imm)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Parses the bits length in `exp` assembly operation into usize.
fn parse_bit_len_param(op: &Token, param_idx: usize) -> Result<u8, ParsingError> {
    let param_value = op.parts()[param_idx];

    if let Some(param) = param_value.strip_prefix('u') {
        // parse bits len param
        match param.parse::<u8>() {
            Ok(value) => Ok(value),
            Err(_) => Err(ParsingError::invalid_param(op, param_idx)),
        }
    } else {
        Err(ParsingError::invalid_param(op, param_idx))
    }
}
