use super::{op_flags::OpFlags, EvaluationFrame, Vec};
use crate::{
    stack::EvaluationFrameExt,
    utils::{are_equal, is_binary},
};
use vm_core::FieldElement;
use winter_air::TransitionConstraintDegree;

#[cfg(test)]
pub mod tests;

// CONSTANTS
// ================================================================================================

/// The number of transition constraints in all the field operations.
pub const NUM_CONSTRAINTS: usize = 5;

/// The degrees of constraints in individual stack operations of the field operations.
pub const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [
    // Given it is a degree 7 operation, 7 is added to all the constraints
    // constraints.

    // There's one constraint in INCR field operation where 1 is added to the top
    // element of the stack.
    8,
    // There's one constraint in INV field operation where the top element in the
    // stack is replaced with it's inverse.
    9,
    // There's one constraint in NEG field operation where the top element in the
    // stack is replaced with it's negation.
    8,
    // There are two  operations in NOT field operation. The first one checks if the
    // top element in the stack is a boolean or not whereas the second one verifies if
    // the first element in the next trace is a boolean not of first element in the current trace.
    9, 8,
];

// FIELD OPERATIONS TRANSITION CONSTRAINTS
// ================================================================================================

/// Builds the transition constraint degrees for the field operations.
pub fn get_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    CONSTRAINT_DEGREES
        .iter()
        .map(|&degree| TransitionConstraintDegree::new(degree))
        .collect()
}

/// Returns the number of transition constraints of the Field operations.
pub fn get_transition_constraint_count() -> usize {
    NUM_CONSTRAINTS
}

/// Enforces constraints of the Field operations.
pub fn enforce_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: &OpFlags<E>,
) -> usize {
    let mut index = 0;

    // Enforce constaints of the INCR operation.
    index += enforce_incr_constraints(frame, result, op_flag.incr());
    // Enforce constaints of the INV operation.
    index += enforce_inv_constraints(frame, &mut result[index..], op_flag.inv());
    // Enforce constaints of the NEG operation.
    index += enforce_neg_constraints(frame, &mut result[index..], op_flag.neg());
    // Enforce constaints of the NOT operation.
    index += enforce_not_constraints(frame, &mut result[index..], op_flag.not());

    index
}

// TRANSITION CONSTRAINT HELPERS
// ================================================================================================

/// Enforces constraints of the INCR operation. The INCR operation increments the
/// top element in the stack by 1. Therefore, the following constraints are enforced:
/// - The next element in the next frame should be equal to the addition of first element in the
///   current frame with 1.
pub fn enforce_incr_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: E,
) -> usize {
    // Enforces the first element in the next frame is incremented by 1.
    result[0] = op_flag * are_equal(frame.stack_item(0) + E::ONE, frame.stack_item_next(0));

    1
}

/// Enforces constraints of the INV operation. The INV operation updates the top element
/// in the stack with its inverse. Therefore, the following constraints are enforced:
/// - The next element in the next frame should be the inverse of frist element in the
///   current frame.
pub fn enforce_inv_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: E,
) -> usize {
    // Enforces the first element in the next frame is an inverse of the first element in the
    // current frame.
    result[0] = op_flag * are_equal(frame.stack_item(0) * frame.stack_item_next(0), E::ONE);

    1
}

/// Enforces constraints of the NEG operation. The NEG operation updates the top element
/// in the stack with its inverse. Therefore, the following constraints are enforced:
/// - The next element in the next frame should be the negation of first element in the
///   current frame, therefore, their sum should be 0.
pub fn enforce_neg_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: E,
) -> usize {
    // Enforces the first element in the current frame is a negation of the first element in the
    // next frame
    result[0] = op_flag * are_equal(frame.stack_item(0) + frame.stack_item_next(0), E::ZERO);

    1
}

/// Enforces constraints of the NOT operation. The NOT operation updates the top element
/// in the stack with its bitwise not value. Therefore, the following constraints are
/// enforced:
/// - The top element in the stack should be a binary.
/// - The first element of the next frame should be a binary not of the first element of
/// the current frame.
pub fn enforce_not_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: E,
) -> usize {
    // Enforces the top element in the current frame is a binary or not.
    result[0] = op_flag * is_binary(frame.stack_item(0));

    // Enforces the first element in the current frame is a binary not of the first element
    // in the next frame.
    result[1] = op_flag * are_equal(frame.stack_item_next(0) + frame.stack_item(0), E::ONE);

    2
}
