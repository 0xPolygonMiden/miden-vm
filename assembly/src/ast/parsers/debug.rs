use super::{
    parse_checked_param,
    Instruction::*,
    Node::{self, Instruction},
    ParsingError, Token,
};
use vm_core::DebugOptions;

// INSTRUCTION PARSERS
// ================================================================================================

/// Returns `Debug` instruction node.
///
/// # Errors
/// Returns an error if the instruction token contains a wrong number of parameters, or if
/// the provided parameters are not valid.
pub fn parse_debug(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "debug");
    if op.num_parts() < 2 {
        return Err(ParsingError::missing_param(op, "debug.stack.<debug_params?>"));
    }

    let options = match op.parts()[1] {
        "stack" => match op.num_parts() {
            2 => DebugOptions::StackAll,
            3 => {
                let n: u16 = parse_checked_param(op, 2, 1..=u16::MAX)?;
                DebugOptions::StackTop(n)
            }
            _ => return Err(ParsingError::extra_param(op)),
        },
        _ => return Err(ParsingError::invalid_op(op)),
    };

    Ok(Instruction(Debug(options)))
}
