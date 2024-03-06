use super::{
    parse_error_code,
    Instruction::*,
    LocalConstMap,
    Node::{self, Instruction},
    ParsingError, Token,
};

// INSTRUCTION PARSERS
// ================================================================================================

/// Returns `MTreeVerify` instruction node if no error code value is provided, or
/// `MTreeVerifyWithError` instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains a wrong number of parameters, or if
/// the provided parameter is not a u32 value.
pub fn parse_mtree_verify(op: &Token, constants: &LocalConstMap) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "mtree_verify");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(MTreeVerify)),
        2 => {
            let err_code = parse_error_code(op, constants)?;
            if err_code == 0 {
                Ok(Instruction(MTreeVerify))
            } else {
                Ok(Instruction(MTreeVerifyWithError(err_code)))
            }
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}
