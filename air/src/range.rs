use vm_core::RANGE_CHECK_TRACE_OFFSET;

use super::{
    utils::{is_binary, ColumnTransition},
    Assertion, EvaluationFrame, Felt, FieldElement, TransitionConstraintDegree,
};

// CONSTANTS
// ================================================================================================
const NUM_CONSTRAINTS: usize = 7;
const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [
    2, 2, 2, // Selector flags must be binary: t, s0, s1.
    3, // Constrain the row transitions in the 8-bit section of the table.
    2, // Transition from 8-bit to 16-bit section of range check table occurs at most once.
    3, 3, // Enforce values of column v before and after 8-bit to 16-bit transition.
];

pub const T_COL_IDX: usize = RANGE_CHECK_TRACE_OFFSET;
pub const S0_COL_IDX: usize = RANGE_CHECK_TRACE_OFFSET + 1;
pub const S1_COL_IDX: usize = RANGE_CHECK_TRACE_OFFSET + 2;
pub const V_COL_IDX: usize = RANGE_CHECK_TRACE_OFFSET + 3;

pub const NUM_ASSERTIONS: usize = 2;

// CONSTRAINT DEGREES
// ================================================================================================

pub fn get_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    let mut result = Vec::new();

    for &degree in CONSTRAINT_DEGREES.iter() {
        result.push(TransitionConstraintDegree::new(degree));
    }

    result
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
    // --- Constrain the selector flags. ----------------------------------------------------------
    enforce_flags(frame, result);

    // --- Constrain the row transitions in the 8-bit section of the table. -----------------------
    enforce_8bit(frame, &mut result[3..]);

    // --- Constrain the transition from 8-bit to 16-bit section of the table. --------------------
    enforce_16bit(frame, &mut result[4..]);
}

// TRANSITION CONSTRAINT HELPERS
// ================================================================================================

/// Constrain the selector flags.
fn enforce_flags<E: FieldElement<BaseField = Felt>>(frame: &EvaluationFrame<E>, result: &mut [E]) {
    let current = frame.current();

    result[0] = is_binary(current[T_COL_IDX]);
    result[1] = is_binary(current[S0_COL_IDX]);
    result[2] = is_binary(current[S1_COL_IDX]);
}

/// Constrain the row transitions in the 8-bit section of the table.
fn enforce_8bit<E: FieldElement<BaseField = Felt>>(frame: &EvaluationFrame<E>, result: &mut [E]) {
    let next = frame.next();

    let change = frame.change(V_COL_IDX);
    result[0] = (E::ONE - next[T_COL_IDX]) * (change) * (change - E::ONE);
}

/// Constrain the transition from 8-bit to 16-bit section of the table.
fn enforce_16bit<E: FieldElement<BaseField = Felt>>(frame: &EvaluationFrame<E>, result: &mut [E]) {
    let current = frame.current();
    let next = frame.next();
    let t = current[T_COL_IDX];
    let t_next = next[T_COL_IDX];
    let flip = (E::ONE - t) * t_next;

    // Values in column t can "flip" from 0 to 1 only once.
    result[0] = t * (E::ONE - t_next);

    // When column t "flips", column v must equal 255.
    result[1] = flip * (current[V_COL_IDX] - E::from(255_u8));

    // When column t "flips", the next value column v must be reset to 0.
    result[2] = flip * next[V_COL_IDX];
}
