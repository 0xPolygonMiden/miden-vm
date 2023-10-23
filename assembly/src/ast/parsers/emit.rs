use super::{
    parse_param_with_constant_lookup,
    Instruction::*,
    LocalConstMap,
    Node::{self, Instruction},
    ParsingError, Token,
};

// EMIT PARSER
// ================================================================================================

/// Returns `Emit` instruction node with the parsed `event_id`.
///
/// The `event_id` can be provided as a constant label or as a u32 value.
///
/// # Errors
/// Returns an error if the constant does not exist or if the value is not a u32.
pub fn parse_emit(op: &Token, constants: &LocalConstMap) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "emit");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Err(ParsingError::missing_param(op, "emit.<event_id>")),
        2 => {
            let event_id = parse_param_with_constant_lookup(op, 1, constants)?;
            Ok(Instruction(Emit(event_id)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}
