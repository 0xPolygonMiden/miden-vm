use super::{
    parse_param_with_constant_lookup,
    Instruction::*,
    LocalConstMap,
    Node::{self, Instruction},
    ParsingError, Token,
};

// TRACE PARSER
// ================================================================================================

/// Returns `Trace` instruction node with the parsed `trace_id`.
///
/// The `trace_id` can be provided as a constant label or as a u32 value.
///
/// # Errors
/// Returns an error if the constant does not exist or if the value is not a u32.
pub fn parse_trace(op: &Token, constants: &LocalConstMap) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "trace");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Err(ParsingError::missing_param(op, "trace.<tace_id>")),
        2 => {
            let trace_id = parse_param_with_constant_lookup(op, 1, constants)?;
            Ok(Instruction(Trace(trace_id)))
        }
        _ => Err(ParsingError::extra_param(op)),
    }
}
