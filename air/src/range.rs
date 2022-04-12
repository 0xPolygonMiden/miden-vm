use vm_core::RANGE_CHECK_TRACE_OFFSET;

use super::{
    utils::{
        binary_not, is_binary, ColumnBoundary, ColumnTransition, ProcessorConstraints,
        TransitionConstraints,
    },
    EvaluationFrame, Felt, FieldElement,
};

// CONSTANTS
// ================================================================================================
pub const NUM_CONSTRAINTS: usize = 7;
pub const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [
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
pub struct RangeCheckerConstraints {
    first_step: Vec<ColumnBoundary>,
    last_step: Vec<ColumnBoundary>,
    transitions: TransitionConstraints,
}

impl RangeCheckerConstraints {
    pub fn new() -> Self {
        // Define the boundary constraints.
        let first_step = vec![ColumnBoundary::new(V_COL_IDX, Felt::ZERO)];
        let last_step = vec![ColumnBoundary::new(V_COL_IDX, Felt::new(65535))];

        // Define the transition constraints.
        let transitions = TransitionConstraints::new(NUM_CONSTRAINTS, CONSTRAINT_DEGREES.to_vec());

        Self {
            first_step,
            last_step,
            transitions,
        }
    }
}

impl ProcessorConstraints for RangeCheckerConstraints {
    // BOUNDARY CONSTRAINTS
    // ============================================================================================

    fn first_step(&self) -> &[ColumnBoundary] {
        &self.first_step
    }

    fn last_step(&self) -> &[ColumnBoundary] {
        &self.last_step
    }

    // TRANSITION CONSTRAINTS
    // ============================================================================================

    fn transitions(&self) -> &TransitionConstraints {
        &self.transitions
    }

    fn enforce_constraints<E: FieldElement>(&self, frame: &EvaluationFrame<E>, result: &mut [E]) {
        // --- Constrain the selector flags. ------------------------------------------------------
        enforce_flags::<E>(frame, result);

        // --- Constrain the row transitions in the 8-bit section of the table. -------------------
        enforce_8bit::<E>(frame, &mut result[3..]);

        // --- Constrain the transition from 8-bit to 16-bit section of the table. ----------------
        enforce_16bit::<E>(frame, &mut result[4..]);
    }
}

// TRANSITION CONSTRAINT HELPERS
// ================================================================================================

/// Constrain the selector flags.
fn enforce_flags<E: FieldElement>(frame: &EvaluationFrame<E>, result: &mut [E]) {
    let current = frame.current();

    result[0] = is_binary(current[T_COL_IDX]);
    result[1] = is_binary(current[S0_COL_IDX]);
    result[2] = is_binary(current[S1_COL_IDX]);
}

/// Constrain the row transitions in the 8-bit section of the table.
fn enforce_8bit<E: FieldElement>(frame: &EvaluationFrame<E>, result: &mut [E]) {
    let next = frame.next();

    let change = frame.change(V_COL_IDX);
    result[0] = (E::ONE - next[T_COL_IDX]) * (change) * (change - E::ONE);
}

/// Constrain the transition from 8-bit to 16-bit section of the table.
fn enforce_16bit<E: FieldElement>(frame: &EvaluationFrame<E>, result: &mut [E]) {
    let current = frame.current();
    let next = frame.next();
    let t = current[T_COL_IDX];
    let t_next = next[T_COL_IDX];
    let flip = binary_not(t) * t_next;

    // Values in column t can "flip" from 0 to 1 only once.
    result[0] = t * binary_not(t_next);

    // When column t "flips", column v must equal 255.
    result[1] = flip * (current[V_COL_IDX] - E::from(255_u8));

    // When column t "flips", the next value column v must be reset to 0.
    result[2] = flip * next[V_COL_IDX];
}
