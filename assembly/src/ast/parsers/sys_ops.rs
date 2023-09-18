use super::{
    try_get_constant_value,
    Instruction::*,
    LocalConstMap,
    Node::{self, Instruction},
    ParsingError, Token, Vec,
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

// HELPER FUNCTIONS
// ================================================================================================

/// Parses the error code declaration for an assertion instruction, and returns the value of the
/// code.
///
/// The code is expected to be specified via the first instruction parameter and have the form
/// `err=<code>`.
fn parse_error_code(token: &Token, constants: &LocalConstMap) -> Result<u32, ParsingError> {
    let inst = token.parts()[0];
    let err_code_parts: Vec<&str> = token.parts()[1].split('=').collect();
    match err_code_parts.len() {
        0 => unreachable!(),
        1 => Err(ParsingError::missing_param(token, format!("{inst}.err=<code>").as_str())),
        2 => {
            if err_code_parts[0] != "err" {
                return Err(ParsingError::invalid_param(token, 1));
            }

            let err_code_str = err_code_parts[1];
            let err_code = match try_get_constant_value(token, err_code_str, constants)? {
                Some(val) => val.try_into().map_err(|_| ParsingError::invalid_param(token, 1))?,
                None => err_code_str.parse().map_err(|_| ParsingError::invalid_param(token, 1))?,
            };
            Ok(err_code)
        }
        _ => Err(ParsingError::extra_param(token)),
    }
}
