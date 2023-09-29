use super::{
    parse_error_code,
    Instruction::*,
    LocalConstMap,
    Node::{self, Instruction},
    ParsingError, Token,
};

/// Returns `Assert` instruction node if no error code value is provided, or `AssertWithError`
/// instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains a wrong number of parameters, or if
/// the provided parameter is not a u32 value.
pub fn parse_assert(op: &Token, constants: &LocalConstMap) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "assert");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(Assert)),
        2 => {
            let err_code = parse_error_code(op, constants)?;
            if err_code == 0 {
                Ok(Instruction(Assert))
            } else {
                Ok(Instruction(AssertWithError(err_code)))
            }
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `Assertz` instruction node if no error code value is provided, or `AssertzWithError`
/// instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains a wrong number of parameters, or if
/// the provided parameter is not a u32 value.
pub fn parse_assertz(op: &Token, constants: &LocalConstMap) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "assertz");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(Assertz)),
        2 => {
            let err_code = parse_error_code(op, constants)?;
            if err_code == 0 {
                Ok(Instruction(Assertz))
            } else {
                Ok(Instruction(AssertzWithError(err_code)))
            }
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `AssertEq` instruction node if no error code value is provided, or `AssertEqWithError`
/// instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains a wrong number of parameters, or if
/// the provided parameter is not a u32 value.
pub fn parse_assert_eq(op: &Token, constants: &LocalConstMap) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "assert_eq");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(AssertEq)),
        2 => {
            let err_code = parse_error_code(op, constants)?;
            if err_code == 0 {
                Ok(Instruction(AssertEq))
            } else {
                Ok(Instruction(AssertEqWithError(err_code)))
            }
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `AssertEqw` instruction node if no error code value is provided, or `AssertEqwWithCode`
/// instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains a wrong number of parameters, or if
/// the provided parameter is not a u32 value.
pub fn parse_assert_eqw(op: &Token, constants: &LocalConstMap) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "assert_eqw");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(AssertEqw)),
        2 => {
            let err_code = parse_error_code(op, constants)?;
            if err_code == 0 {
                Ok(Instruction(AssertEqw))
            } else {
                Ok(Instruction(AssertEqwWithError(err_code)))
            }
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}
