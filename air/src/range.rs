use vm_core::RANGE_CHECK_TRACE_OFFSET;

use super::{
    utils::is_binary, Assertion, EvaluationFrame, Felt, FieldElement, TransitionConstraintDegree,
};

// CONSTANTS
// ================================================================================================

pub const T_COL_IDX: usize = RANGE_CHECK_TRACE_OFFSET;
pub const S0_COL_IDX: usize = RANGE_CHECK_TRACE_OFFSET + 1;
pub const S1_COL_IDX: usize = RANGE_CHECK_TRACE_OFFSET + 2;
pub const V_COL_IDX: usize = RANGE_CHECK_TRACE_OFFSET + 3;

// CONSTRAINT DEGREES
// ================================================================================================

pub fn get_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    vec![
        // Selector flags must be binary: t, s0, s1
        TransitionConstraintDegree::new(2),
        TransitionConstraintDegree::new(2),
        TransitionConstraintDegree::new(2),
        // Constrain the row transitions in the 8-bit section of the table.
        TransitionConstraintDegree::new(3),
        // Transition from 8-bit to 16-bit section of range check table occurs at most once.
        TransitionConstraintDegree::new(2),
        // Enforce values of column v before and after 8-bit to 16-bit transition.
        TransitionConstraintDegree::new(3),
        TransitionConstraintDegree::new(3),
    ]
}

// BOUNDARY CONSTRAINTS
// ================================================================================================

pub fn get_assertions_first_step(result: &mut Vec<Assertion<Felt>>) {
    result.push(Assertion::single(V_COL_IDX, 0, Felt::ZERO));
}

pub fn get_assertions_last_step(result: &mut Vec<Assertion<Felt>>, step: usize) {
    result.push(Assertion::single(V_COL_IDX, step, Felt::new(65535)));
}

// TRANSITION CONSTRAINTS
// ================================================================================================

pub fn enforce_constraints<E: FieldElement<BaseField = Felt>>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
) {
    let current = frame.current();
    let next = frame.next();

    // --- Constrain the selector flags. ----------------------------------------------------------
    result[0] = is_binary(current[T_COL_IDX]);
    result[1] = is_binary(current[S0_COL_IDX]);
    result[2] = is_binary(current[S1_COL_IDX]);

    // --- Constrain the row transitions in the 8-bit section of the table. -----------------------
    let row_diff = next[V_COL_IDX] - current[V_COL_IDX];
    result[3] = (E::ONE - next[T_COL_IDX]) * (row_diff) * (row_diff - E::ONE);

    // --- Constrain the transition from 8-bit to 16-bit section of the table. --------------------
    let flip = (E::ONE - current[T_COL_IDX]) * next[T_COL_IDX];

    // Values in column t can "flip" from 0 to 1 only once.
    result[4] = current[T_COL_IDX] * (E::ONE - next[T_COL_IDX]);

    // When column t "flips", column v must equal 255.
    result[5] = flip * (current[V_COL_IDX] - E::from(255_u8));

    // When column t "flips", the next value column v must be reset to 0.
    result[6] = flip * next[V_COL_IDX];
}
