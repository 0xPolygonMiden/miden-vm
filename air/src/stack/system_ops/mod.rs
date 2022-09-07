use super::{op_flags::OpFlags, EvaluationFrame, Vec};
use crate::{stack::EvaluationFrameExt, utils::are_equal};
use vm_core::FieldElement;
use winter_air::TransitionConstraintDegree;

#[cfg(test)]
pub mod tests;

// CONSTANTS
// ================================================================================================

/// The number of unique transition constraints in system operations.
pub const NUM_CONSTRAINTS: usize = 1;

/// The degrees of constraints in individual system operations of the system ops.
pub const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [
    // Given it is a degree 7 operation, 7 is added to all the individual constraints
    // degree.

    // There's one unique constraint in FMPADD system operation in which the top element in the
    // stack is is incremented by fmp register value.
    8,
];

// SYSTEM OPERATIONS TRANSITION CONSTRAINTS
// ================================================================================================

/// Builds the transition constraint degrees for the system operations.
pub fn get_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    CONSTRAINT_DEGREES
        .iter()
        .map(|&degree| TransitionConstraintDegree::new(degree))
        .collect()
}

/// Returns the number of transition constraints for the system operations.
pub fn get_transition_constraint_count() -> usize {
    NUM_CONSTRAINTS
}

/// Enforces constraints for all the system operations.
pub fn enforce_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: &OpFlags<E>,
) -> usize {
    let mut index = 0;

    index += enforce_fmpadd_constraints(frame, result, op_flag.fmpadd());

    index
}

// TRANSITION CONSTRAINT HELPERS
// ================================================================================================

/// Enforces unique constraints of the FMPADD operation. The FMPADD operation increments the
/// top element in the stack by fmp value. Therefore, the following constraints are enforced:
/// - The first element in the next frame should be equal to the addition of the first element
///   in the current frame and the fmp value.
pub fn enforce_fmpadd_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: E,
) -> usize {
    // Enforces the first element in the next frame is incremented by fmp value.
    result[0] = op_flag * are_equal(frame.stack_item(0) + frame.fmp(), frame.stack_item_next(0));

    1
}
