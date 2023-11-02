use super::{
    check_div_by_zero, parse_checked_param, parse_error_code, parse_param,
    Instruction::*,
    LocalConstMap,
    Node::{self, Instruction},
    ParsingError, Token,
};
use crate::{MAX_U32_ROTATE_VALUE, MAX_U32_SHIFT_VALUE};

// INSTRUCTION PARSERS
// ================================================================================================

/// Returns `U32Assert` instruction node if no error code value is provided, or
/// `U32AssertWithError` instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32assert(op: &Token, constants: &LocalConstMap) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "u32assert");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32Assert)),
        2 => {
            let err_code = parse_error_code(op, constants)?;
            if err_code == 0 {
                Ok(Instruction(U32Assert))
            } else {
                Ok(Instruction(U32AssertWithError(err_code)))
            }
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `U32Assert2` instruction node if no error code value is provided, or
/// `U32Assert2WithError` instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32assert2(op: &Token, constants: &LocalConstMap) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "u32assert2");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32Assert2)),
        2 => {
            let err_code = parse_error_code(op, constants)?;
            if err_code == 0 {
                Ok(Instruction(U32Assert2))
            } else {
                Ok(Instruction(U32Assert2WithError(err_code)))
            }
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `U32AssertW` instruction node if no error code value is provided, or
/// `U32AssertWWithError` instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32assertw(op: &Token, constants: &LocalConstMap) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "u32assertw");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32AssertW)),
        2 => {
            let err_code = parse_error_code(op, constants)?;
            if err_code == 0 {
                Ok(Instruction(U32AssertW))
            } else {
                Ok(Instruction(U32AssertWWithError(err_code)))
            }
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `U32WrappingAdd` instruction node if no immediate value is provided or
/// `U32WrappingAddImm` instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32wrapping_add(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "u32wrapping_add");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32WrappingAdd)),
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Ok(Instruction(U32WrappingAddImm(value)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `U32OverflowingAdd` instruction node if no immediate value is provided or
/// `U32OverflowingAddImm` instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32overflowing_add(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "u32overflowing_add");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32OverflowingAdd)),
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Ok(Instruction(U32OverflowingAddImm(value)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `U32WrappingSub` instruction node if no immediate value is provided or
/// `U32WrappingSubImm` instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32wrapping_sub(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "u32wrapping_sub");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32WrappingSub)),
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Ok(Instruction(U32WrappingSubImm(value)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `U32OverflowingSub` instruction node if no immediate value is provided or
/// `U32OverflowingSubImm` instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32overflowing_sub(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "u32overflowing_sub");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32OverflowingSub)),
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Ok(Instruction(U32OverflowingSubImm(value)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `U32WrappingMul` instruction node if no immediate value is provided or
/// `U32WrappingMulImm` instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32wrapping_mul(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "u32wrapping_mul");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32WrappingMul)),
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Ok(Instruction(U32WrappingMulImm(value)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `U32OverflowingMul` instruction node if no immediate value is provided or
/// `U32OverflowingMulImm` instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32overflowing_mul(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "u32overflowing_mul",);
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32OverflowingMul)),
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Ok(Instruction(U32OverflowingMulImm(value)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns one of two possible instructions:
/// - division without parameter: `U32Div`
/// - division with parameter: `U32DivImm`
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32_div(op: &Token) -> Result<Node, ParsingError> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32Div)),
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            check_div_by_zero(value.into(), op, 1)?;
            Ok(Instruction(U32DivImm(value)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns one of two possible instructions:
/// - module without parameter: `U32Mod`
/// - module with parameter: `U32ModImm`
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32_mod(op: &Token) -> Result<Node, ParsingError> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32Mod)),
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            check_div_by_zero(value.into(), op, 1)?;
            Ok(Instruction(U32ModImm(value)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns one of two possible instructions:
/// - DivMod without parameter: `U32DivMod`
/// - DivMod with parameter: `U32DivModImm`
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32_divmod(op: &Token) -> Result<Node, ParsingError> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32DivMod)),
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            check_div_by_zero(value.into(), op, 1)?;
            Ok(Instruction(U32DivModImm(value)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns one of two possible instructions:
/// - shift right without parameter: `U32Shr`
/// - shift right with parameter: `U32ShrImm`
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is greater than 31.
pub fn parse_u32_shr(op: &Token) -> Result<Node, ParsingError> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32Shr)),
        2 => {
            let n = parse_checked_param::<u8, _>(op, 1, 0..=MAX_U32_SHIFT_VALUE)?;
            Ok(Instruction(U32ShrImm(n)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns one of two possible instructions:
/// - shift left without parameter: `U32Shl`
/// - shift left with parameter: `U32ShlImm`
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is greater than 31.
pub fn parse_u32_shl(op: &Token) -> Result<Node, ParsingError> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32Shl)),
        2 => {
            let n = parse_checked_param::<u8, _>(op, 1, 0..=MAX_U32_SHIFT_VALUE)?;
            Ok(Instruction(U32ShlImm(n)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns one of two possible instructions:
/// - rotation right without parameter: `U32Rotr`
/// - rotation right with parameter: `U32RotrImm`
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is greater than 31.
pub fn parse_u32_rotr(op: &Token) -> Result<Node, ParsingError> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32Rotr)),
        2 => {
            let n = parse_checked_param::<u8, _>(op, 1, 0..=MAX_U32_ROTATE_VALUE)?;
            Ok(Instruction(U32RotrImm(n)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns one of two possible instructions:
/// - rotation left without parameter: `U32Rotl`
/// - rotation left with parameter: `U32RotlImm`
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is greater than 31.
pub fn parse_u32_rotl(op: &Token) -> Result<Node, ParsingError> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32Rotl)),
        2 => {
            let n = parse_checked_param::<u8, _>(op, 1, 0..=MAX_U32_ROTATE_VALUE)?;
            Ok(Instruction(U32RotlImm(n)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}
