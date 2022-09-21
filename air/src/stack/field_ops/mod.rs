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
pub const NUM_CONSTRAINTS: usize = 17;

/// The degrees of constraints in individual stack operations of the field operations.
pub const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [
    // Given it is a degree 7 operation, 7 is added to all the individual constraints
    // degree.
    8, // constraint for ADD field operation.
    8, // constraint for NEG field operation.
    9, // constraint for MUL field operation.
    9, // constraint for INV field operation.
    8, // constraint for INCR field operation.
    9, 8, // two constraints for NOT field operation.
    9, 9, 9, // three constraints for AND field operation.
    9, 9, 9, // three constraints for OR field operation.
    9, 9, // two constraints for EQ field operation.
    9, 9, // two constraints for EQZ field operation.
];

// FIELD OPERATIONS TRANSITION CONSTRAINTS
// ================================================================================================

/// Builds the transition constraint degrees of the field operations.
pub fn get_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    CONSTRAINT_DEGREES
        .iter()
        .map(|&degree| TransitionConstraintDegree::new(degree))
        .collect()
}

/// Returns the number of transition constraints of the field operations.
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

    // Enforce constaints of the ADD operation.
    index += enforce_add_constraints(frame, result, op_flag.add());

    // Enforce constaints of the NEG operation.
    index += enforce_neg_constraints(frame, &mut result[index..], op_flag.neg());

    // Enforce constaints of the MUL operation.
    index += enforce_mul_constraints(frame, &mut result[index..], op_flag.mul());

    // Enforce constaints of the INV operation.
    index += enforce_inv_constraints(frame, &mut result[index..], op_flag.inv());

    // Enforce constaints of the INCR operation.
    index += enforce_incr_constraints(frame, &mut result[index..], op_flag.incr());

    // Enforce constaints of the NOT operation.
    index += enforce_not_constraints(frame, &mut result[index..], op_flag.not());

    // Enforce constaints of the AND operation.
    index += enforce_and_constraints(frame, &mut result[index..], op_flag.and());

    // Enforce constaints of the OR operation.
    index += enforce_or_constraints(frame, &mut result[index..], op_flag.or());

    // Enforce constaints of the EQ operation.
    index += enforce_eq_constraints(frame, &mut result[index..], op_flag.eq());

    // Enforce constaints of the EQZ operation.
    index += enforce_eqz_constraints(frame, &mut result[index..], op_flag.eqz());

    index
}

// TRANSITION CONSTRAINT HELPERS
// ================================================================================================

/// Enforces constraints of the ADD operation. The ADD operation adds the first two elements
/// in the current trace. Therefore, the following constraints are enforced:
/// - The first element in the trace frame should be the addition of the first two elements in
///   the current trace. s0` - s0 - s1 = 0.
pub fn enforce_add_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: E,
) -> usize {
    let a = frame.stack_item(0);
    let b = frame.stack_item(1);
    let c = frame.stack_item_next(0);

    // Enforce that c is equal to the sum of a and b.
    result[0] = op_flag * are_equal(a + b, c);

    1
}

/// Enforces constraints of the NEG operation. The NEG operation updates the top element
/// in the stack with its inverse. Therefore, the following constraints are enforced:
/// - The first element in the next frame should be the negation of first element in the
///   current frame, therefore, their sum should be 0. s0` + s0 = 0.
pub fn enforce_neg_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: E,
) -> usize {
    // Enforces the first element in the current frame is a negation of the first element in the
    // next frame.
    result[0] = op_flag * are_equal(frame.stack_item(0) + frame.stack_item_next(0), E::ZERO);

    1
}

/// Enforces constraints of the MUL operation. The MUL operation multiplies the first two elements
/// in the current trace. Therefore, the following constraints are enforced:
/// - The first element in the next frame should be the product of the first two elements in
///   the current frame. s0` - s0 * s1 = 0
pub fn enforce_mul_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: E,
) -> usize {
    let a = frame.stack_item(0);
    let b = frame.stack_item(1);
    let c = frame.stack_item_next(0);

    // Enforce that c is the product of a and b.
    result[0] = op_flag * are_equal(a * b, c);

    1
}

/// Enforces constraints of the INV operation. The INV operation updates the top element
/// in the stack with its inverse. Therefore, the following constraints are enforced:
/// - The next element in the next frame should be the inverse of first element in the
///   current frame. s0` * s0 = 1.
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

/// Enforces constraints of the INCR operation. The INCR operation increments the
/// top element in the stack by 1. Therefore, the following constraints are enforced:
/// - The next element in the next frame should be equal to the addition of first element in the
///   current frame with 1. s0` - s0 - 1 = 0.
pub fn enforce_incr_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: E,
) -> usize {
    // Enforces the first element in the next frame is incremented by 1.
    result[0] = op_flag * are_equal(frame.stack_item(0) + E::ONE, frame.stack_item_next(0));

    1
}

/// Enforces constraints of the NOT operation. The NOT operation updates the top element
/// in the stack with its bitwise not value. Therefore, the following constraints are
/// enforced:
/// - The top element in the stack should be a binary. s0*2 - s0 = 0.
/// - The first element of the next frame should be a binary not of the first element of
/// the current frame. s0` + s0 = 1.
pub fn enforce_not_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: E,
) -> usize {
    let a = frame.stack_item(0);
    let b = frame.stack_item_next(0);

    // Enforce that a is a binary.
    result[0] = op_flag * is_binary(a);

    // Enforce that b is the binary not of a.
    result[1] = op_flag * are_equal(a + b, E::ONE);

    2
}

/// Enforces constraints of the AND operation. The AND operation computes the bitwise and of the
/// first two elements in the current trace. Therefore, the following constraints are enforced:
/// - The top two elements in the current frame of the stack should be binary. s0*2 - s0 = 0,
///   s1*2 - s1 = 0.
/// - The first element of the next frame should be a binary and of the first two elements in the
///   current frame. s0` - s0 * s1 = 0.
pub fn enforce_and_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: E,
) -> usize {
    let a = frame.stack_item(0);
    let b = frame.stack_item(1);
    let c = frame.stack_item_next(0);

    // Enforce that a and b are binary values.
    result[0] = op_flag * is_binary(a);
    result[1] = op_flag * is_binary(b);

    // bitwise and of a and b.
    let and_value = a * b;

    // Enforce that c is the bitwise and of a & b.
    result[2] = op_flag * are_equal(c, and_value);

    3
}

/// Enforces constraints of the OR operation. The OR operation computes the bitwise or of the
/// first two elements in the current trace. Therefore, the following constraints are enforced:
/// - The top two elements in the current frame of the stack should be binary. s0*2 - s0 = 0,
///   s1*2 - s1 = 0.
/// - The first element of the next frame should be a binary or of the first two elements in the
///   current frame. s0` - ( s0 + s1 - s0 * s1 ) = 0.
pub fn enforce_or_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: E,
) -> usize {
    let a = frame.stack_item(0);
    let b = frame.stack_item(1);
    let c = frame.stack_item_next(0);

    // Enforce that a and b are binary values.
    result[0] = op_flag * is_binary(a);
    result[1] = op_flag * is_binary(b);

    // bitwise or of a and b.
    let or_value = a + b - a * b;

    // Enforce that c is the bitwise or of a and b.
    result[2] = op_flag * are_equal(c, or_value);

    3
}

/// Enforces constraints of the EQ operation. The EQ operation checks if the top two elements in the
/// current frame are equal or not. Therefore, the following constraints are enforced:
/// - (s0 - s1) * s0' = 0
/// - s0` - (1 - (s0 - s1) * h0) = 0
pub fn enforce_eq_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: E,
) -> usize {
    let a = frame.stack_item(0);
    let b = frame.stack_item(1);
    let c = frame.stack_item_next(0);

    // difference between a and b.
    let diff_top_elements = a - b;

    // Enforce that either c or difference between a & b is zero.
    result[0] = op_flag * are_equal(diff_top_elements * c, E::ZERO);

    // It is the inverse of the diff of the top two element in the current frame if the diff is not ZERO;
    // otherwise it could be anything. The value is fetched from the first register in the user op helper.
    let diff_inv = frame.user_op_helper(0);

    let helper_agg_value = E::ONE - diff_top_elements * diff_inv;

    // Enforce that if the difference between a & b is zero, c cannot be zero and vice versa.
    result[1] = op_flag * are_equal(c, helper_agg_value);

    2
}

/// Enforces constraints of the EQZ operation. The EQZ operation checks if the top element in the
/// current frame is 0 or not. Therefore, the following constraints are enforced:
/// - s0 * s0` = 0.
/// - s0` - (1 - h0 * s0) = 0.
pub fn enforce_eqz_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: E,
) -> usize {
    let a = frame.stack_item(0);
    let b = frame.stack_item_next(0);

    // Enforce that either a or b is zero.
    result[0] = op_flag * are_equal(a * b, E::ZERO);

    // It is the inverse of the top element in the current frame if the top element is not ZERO; it
    // could be anything otherwise. The value is fetched from the first register in the user op helper.
    let inv = frame.user_op_helper(0);

    let helper_agg_value = E::ONE - a * inv;

    // Enforce that if a is zero, b cannot be zero and vice versa.
    result[1] = op_flag * are_equal(b, helper_agg_value);

    2
}
