use super::{op_flags::OpFlags, EvaluationFrame, Vec};
use crate::{stack::EvaluationFrameExt, utils::are_equal};
use vm_core::FieldElement;
use winter_air::TransitionConstraintDegree;

#[cfg(test)]
pub mod tests;

// CONSTANTS
// ================================================================================================

/// The number of unique transition constraints in stack manipulation operations.
pub const NUM_CONSTRAINTS: usize = 2;

/// The degrees of constraints in individual stack manipulation operations.
pub const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [
    // Given it is a degree 7 operation, 7 is added to all the individual constraints
    // degree.

    // There are two unique constraints in SWAP stack manipulation operation in which the first
    // element in the current frame is swapped with the second element.
    8, 8,
];

// STACK MANIPULATION OPERATIONS TRANSITION CONSTRAINTS
// ================================================================================================

/// Builds the transition constraint degrees for the stack manipulation operations.
pub fn get_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    CONSTRAINT_DEGREES
        .iter()
        .map(|&degree| TransitionConstraintDegree::new(degree))
        .collect()
}

/// Returns the number of transition constraints for the stack manipulation operations.
pub fn get_transition_constraint_count() -> usize {
    NUM_CONSTRAINTS
}

/// Enforces constraints for the stack manipulation operations.
pub fn enforce_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: &OpFlags<E>,
) -> usize {
    let mut index = 0;

    // Enforce constaints of the SWAP operations.
    index += enforce_swap_constraints(frame, result, op_flag.swap());

    index
}

// TRANSITION CONSTRAINT HELPERS
// ================================================================================================

/// Enforces constraints of the SWAP operation. The SWAP operation swaps the first
/// two elements in the stack. Therefore, the following constraints are enforced:
/// - The first element in the current frame should be equal to the second element in the
///   next frame.
/// - The second element in the current frame should be equal to the first element in the
///   next frame.
pub fn enforce_swap_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: E,
) -> usize {
    // Enforces the first element in the current frame is same as to the second element in the
    // next frame
    result[0] = op_flag * are_equal(frame.stack_item(0), frame.stack_item_next(1));

    // Enforces the second element in the current frame is same as to the first element in the
    // next frame
    result[1] = op_flag * are_equal(frame.stack_item(1), frame.stack_item_next(0));

    2
}
