use super::{
    check_div_by_zero, parse_checked_param, parse_param,
    Instruction::*,
    Node::{self, Instruction},
    ParsingError, Token,
};
use crate::{MAX_U32_ROTATE_VALUE, MAX_U32_SHIFT_VALUE};

// INSTRUCTION PARSERS
// ================================================================================================

/// Returns `U32Assert` instruction node if no immediate value is provided or the immediate value
/// is 1. Returns instruction `U32Assert2` if immediate value is equal 2.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not 1 or 2.
pub fn parse_u32assert(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "u32assert");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32Assert)),
        2 => match op.parts()[1] {
            "1" => Ok(Instruction(U32Assert)),
            "2" => Ok(Instruction(U32Assert2)),
            _ => Err(ParsingError::invalid_param(op, 1)),
        },
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `U32CheckedAdd` instruction node if no immediate value is provided or
/// `U32CheckedAddImm` instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32checked_add(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "u32checked_add");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32CheckedAdd)),
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Ok(Instruction(U32CheckedAddImm(value)))
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

/// Returns `U32CheckedSub` instruction node if no immediate value is provided or
/// `U32CheckedSubImm` instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32checked_sub(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "u32checked_sub");
    match op.num_parts() {
        1 => Ok(Instruction(U32CheckedSub)),
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Ok(Instruction(U32CheckedSubImm(value)))
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

/// Returns `U32CheckedMul` instruction node if no immediate value is provided or
/// `U32CheckedMulImm` instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32checked_mul(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "u32checked_mul");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32CheckedMul)),
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Ok(Instruction(U32CheckedMulImm(value)))
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

/// Returns one of four possible instructions:
/// - checked without parameter: `U32CheckedDiv`
/// - unchecked without parameter: `U32UncheckedDiv`
/// - checked with parameter: `U32CheckedDivImm`
/// - unchecked with parameter: `U32UncheckedDivImm`
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32_div(op: &Token, checked: bool) -> Result<Node, ParsingError> {
    //debug_assert_eq!("u32checked_div", op.parts()[0], "not a u32checked_div");
    match op.num_parts() {
        0 => unreachable!(),
        1 => {
            if checked {
                Ok(Instruction(U32CheckedDiv))
            } else {
                Ok(Instruction(U32UncheckedDiv))
            }
        }
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            check_div_by_zero(value.into(), op, 1)?;
            if checked {
                Ok(Instruction(U32CheckedDivImm(value)))
            } else {
                Ok(Instruction(U32UncheckedDivImm(value)))
            }
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns one of four possible instructions:
/// - checked without parameter: `U32CheckedMod`
/// - unchecked without parameter: `U32UncheckedMod`
/// - checked with parameter: `U32CheckedModImm`
/// - unchecked with parameter: `U32UncheckedModImm`
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32_mod(op: &Token, checked: bool) -> Result<Node, ParsingError> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => {
            if checked {
                Ok(Instruction(U32CheckedMod))
            } else {
                Ok(Instruction(U32UncheckedMod))
            }
        }
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            check_div_by_zero(value.into(), op, 1)?;
            if checked {
                Ok(Instruction(U32CheckedModImm(value)))
            } else {
                Ok(Instruction(U32UncheckedModImm(value)))
            }
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns one of four possible instructions:
/// - checked without parameter: `U32CheckedDivMod`
/// - unchecked without parameter: `U32UncheckedDivMod`
/// - checked with parameter: `U32CheckedDivModImm`
/// - unchecked with parameter: `U32UncheckedDivModImm`
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32_divmod(op: &Token, checked: bool) -> Result<Node, ParsingError> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => {
            if checked {
                Ok(Instruction(U32CheckedDivMod))
            } else {
                Ok(Instruction(U32UncheckedDivMod))
            }
        }
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            check_div_by_zero(value.into(), op, 1)?;
            if checked {
                Ok(Instruction(U32CheckedDivModImm(value)))
            } else {
                Ok(Instruction(U32UncheckedDivModImm(value)))
            }
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns one of four possible instructions:
/// - checked without parameter: `U32CheckedShr`
/// - unchecked without parameter: `U32UncheckedShr`
/// - checked with parameter: `U32CheckedShrImm`
/// - unchecked with parameter: `U32UncheckedShrImm`
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is greater than 31.
pub fn parse_u32_shr(op: &Token, checked: bool) -> Result<Node, ParsingError> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => {
            if checked {
                Ok(Instruction(U32CheckedShr))
            } else {
                Ok(Instruction(U32UncheckedShr))
            }
        }
        2 => {
            let n = parse_checked_param::<u8, _>(op, 1, 0..=MAX_U32_SHIFT_VALUE)?;
            if checked {
                Ok(Instruction(U32CheckedShrImm(n)))
            } else {
                Ok(Instruction(U32UncheckedShrImm(n)))
            }
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns one of four possible instructions:
/// - checked without parameter: `U32CheckedShl`
/// - unchecked without parameter: `U32UncheckedShl`
/// - checked with parameter: `U32CheckedShlImm`
/// - unchecked with parameter: `U32UncheckedShlImm`
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is greater than 31.
pub fn parse_u32_shl(op: &Token, checked: bool) -> Result<Node, ParsingError> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => {
            if checked {
                Ok(Instruction(U32CheckedShl))
            } else {
                Ok(Instruction(U32UncheckedShl))
            }
        }
        2 => {
            let n = parse_checked_param::<u8, _>(op, 1, 0..=MAX_U32_SHIFT_VALUE)?;
            if checked {
                Ok(Instruction(U32CheckedShlImm(n)))
            } else {
                Ok(Instruction(U32UncheckedShlImm(n)))
            }
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns one of four possible instructions:
/// - checked without parameter: `U32CheckedRotr`
/// - unchecked without parameter: `U32UncheckedRotr`
/// - checked with parameter: `U32CheckedRotrImm`
/// - unchecked with parameter: `U32UncheckedRotrImm`
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is greater than 31.
pub fn parse_u32_rotr(op: &Token, checked: bool) -> Result<Node, ParsingError> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => {
            if checked {
                Ok(Instruction(U32CheckedRotr))
            } else {
                Ok(Instruction(U32UncheckedRotr))
            }
        }
        2 => {
            let n = parse_checked_param::<u8, _>(op, 1, 0..=MAX_U32_ROTATE_VALUE)?;
            if checked {
                Ok(Instruction(U32CheckedRotrImm(n)))
            } else {
                Ok(Instruction(U32UncheckedRotrImm(n)))
            }
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns one of four possible instructions:
/// - checked without parameter: `U32CheckedRotl`
/// - unchecked without parameter: `U32UncheckedRotl`
/// - checked with parameter: `U32CheckedRotlImm`
/// - unchecked with parameter: `U32UncheckedRotlImm`
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is greater than 31.
pub fn parse_u32_rotl(op: &Token, checked: bool) -> Result<Node, ParsingError> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => {
            if checked {
                Ok(Instruction(U32CheckedRotl))
            } else {
                Ok(Instruction(U32UncheckedRotl))
            }
        }
        2 => {
            let n = parse_checked_param::<u8, _>(op, 1, 0..=MAX_U32_ROTATE_VALUE)?;
            if checked {
                Ok(Instruction(U32CheckedRotlImm(n)))
            } else {
                Ok(Instruction(U32UncheckedRotlImm(n)))
            }
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `U32CheckedEq` instruction node if no immediate value is provided or
/// `U32CheckedEqImm` instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32checked_eq(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "u32checked_eq");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32CheckedEq)),
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Ok(Instruction(U32CheckedEqImm(value)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `U32CheckedNeq` instruction node if no immediate value is provided or
/// `U32CheckedNeqImm` instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not a u32 value.
pub fn parse_u32checked_neq(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "u32checked_neq");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(U32CheckedNeq)),
        2 => {
            let value = parse_param::<u32>(op, 1)?;
            Ok(Instruction(U32CheckedNeqImm(value)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}
