use super::{super::Instruction, push_value, AssemblyError, Felt, Operation, Vec};

// CONSTANTS
// ================================================================================================

/// The maximum number of constant inputs allowed by `push` operation.
const MAX_CONST_INPUTS: usize = 16;

// CONSTANT INPUTS
// ================================================================================================

/// Appends `PUSH` operations to the span block to push one or more provided constant values onto
/// the stack, up to a maximum of 16 values.
pub fn parse_push(
    instruction: &Instruction,
    span_ops: &mut Vec<Operation>,
    values: &Vec<Felt>,
) -> Result<(), AssemblyError> {
    if values.len() > MAX_CONST_INPUTS {
        return Err(AssemblyError::invalid_instruction(&instruction.to_string()));
    }

    for value in values {
        push_value(span_ops, *value);
    }

    Ok(())
}
