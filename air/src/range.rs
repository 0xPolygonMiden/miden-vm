use vm_core::RANGE_CHECK_TRACE_OFFSET;

use super::{
    utils::{binary_not, is_binary},
    Assertion, EvaluationFrame, Felt, FieldElement, TransitionConstraintDegree,
};

// CONSTANTS
// ================================================================================================

/// The number of constraints required by the Range Checker.
pub const NUM_CONSTRAINTS: usize = 7;
/// The degrees of the range checker's constraints, in the order they'll be added to the the result
/// array when a transition is evaluated.
pub const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [
    2, 2, 2, // Selector flags must be binary: t, s0, s1.
    3, // Constrain the row transitions in the 8-bit section of the table.
    2, // Transition from 8-bit to 16-bit section of range check table occurs at most once.
    3, 3, // Enforce values of column v before and after 8-bit to 16-bit transition.
];
/// A binary selector column to track whether a transition currently in the 8-bit or 16-bit portion
/// of the range checker table.
pub const T_COL_IDX: usize = RANGE_CHECK_TRACE_OFFSET;
/// A binary selector column to help specify whether or not the value should be included in the
/// running product.
pub const S0_COL_IDX: usize = RANGE_CHECK_TRACE_OFFSET + 1;
/// A binary selector column to help specify whether or not the value should be included in the
/// running product.
pub const S1_COL_IDX: usize = RANGE_CHECK_TRACE_OFFSET + 2;
/// A column to hold the values being range-checked.
pub const V_COL_IDX: usize = RANGE_CHECK_TRACE_OFFSET + 3;

pub const NUM_ASSERTIONS: usize = 2;

// BOUNDARY CONSTRAINTS
// ================================================================================================

/// Returns the range checker's boundary assertions for the first step.
pub fn get_assertions_first_step(result: &mut Vec<Assertion<Felt>>) {
    let step = 0;
    result.push(Assertion::single(V_COL_IDX, step, Felt::ZERO));
}

/// Returns the range checker's boundary assertions for the last step.
pub fn get_assertions_last_step(result: &mut Vec<Assertion<Felt>>, step: usize) {
    result.push(Assertion::single(V_COL_IDX, step, Felt::new(65535)));
}

// TRANSITION CONSTRAINTS
// ================================================================================================

/// Builds the transition constraint degrees for the range checker.
pub fn get_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    CONSTRAINT_DEGREES
        .iter()
        .map(|&degree| TransitionConstraintDegree::new(degree))
        .collect()
}

/// Returns the number of transition constraints for the range checker.
pub fn get_transition_constraint_count() -> usize {
    NUM_CONSTRAINTS
}

/// Enforces constraints for the range checker.
pub fn enforce_constraints<E: FieldElement>(frame: &EvaluationFrame<E>, result: &mut [E]) {
    // Constrain the selector flags.
    let mut index = enforce_flags(frame, result);

    // Constrain the row transitions in the 8-bit section of the table.
    index += enforce_8bit(frame, &mut result[index..]);

    // Constrain the transition from 8-bit to 16-bit section of the table.
    enforce_16bit(frame, &mut result[index..]);
}

// TRANSITION CONSTRAINT HELPERS
// ================================================================================================

/// Constrain the selector flags to binary values.
fn enforce_flags<E: FieldElement>(frame: &EvaluationFrame<E>, result: &mut [E]) -> usize {
    let constraint_count = 3;

    result[0] = is_binary(frame.t());
    result[1] = is_binary(frame.s0());
    result[2] = is_binary(frame.s1());

    constraint_count
}

/// Constrain the row transitions in the 8-bit section of the table.
fn enforce_8bit<E: FieldElement>(frame: &EvaluationFrame<E>, result: &mut [E]) -> usize {
    let constraint_count = 1;
    let v_change = frame.change(V_COL_IDX);

    result[0] = binary_not(frame.t_next()) * (v_change) * (v_change - E::ONE);

    constraint_count
}

/// Constrain the transition from 8-bit to 16-bit section of the table.
fn enforce_16bit<E: FieldElement>(frame: &EvaluationFrame<E>, result: &mut [E]) -> usize {
    let constraint_count = 3;

    // Values in column t can "flip" from 0 to 1 only once.
    result[0] = frame.t() * binary_not(frame.t_next());

    // When column t "flips", column v must equal 255.
    result[1] = frame.flip_to_16bit_flag() * (frame.v() - E::from(255_u8));

    // When column t "flips", the next value column v must be reset to 0.
    result[2] = frame.flip_to_16bit_flag() * frame.v_next();

    constraint_count
}

// RANGE CHECKER FRAME EXTENSION TRAIT
// ================================================================================================

/// Trait to allow easy access to column values and intermediate variables used in constraint
/// calculations for the Range Checker.
trait EvaluationFrameExt<E: FieldElement> {
    // --- Column accessors -----------------------------------------------------------------------

    /// The current value in column T.
    fn t(&self) -> E;
    /// The next value in column T.
    fn t_next(&self) -> E;
    /// The current value in column s0.
    fn s0(&self) -> E;
    /// The current value in column s1.
    fn s1(&self) -> E;
    /// The current value in column V.
    fn v(&self) -> E;
    /// The next value in column V.
    fn v_next(&self) -> E;

    // --- Intermediate variables & helpers -------------------------------------------------------

    /// The change between the current value in the specified column and the next value, calculated
    /// as `next - current`.
    fn change(&self, column: usize) -> E;

    /// A flag set to 1 when column t changes for 0 to 1 indicating the transition from the 8-bit to
    /// 16-bit sections of the range checker table.
    fn flip_to_16bit_flag(&self) -> E;
}

impl<E: FieldElement> EvaluationFrameExt<E> for &EvaluationFrame<E> {
    // --- Column accessors -----------------------------------------------------------------------

    #[inline(always)]
    fn t(&self) -> E {
        self.current()[T_COL_IDX]
    }

    #[inline(always)]
    fn t_next(&self) -> E {
        self.next()[T_COL_IDX]
    }

    #[inline(always)]
    fn s0(&self) -> E {
        self.current()[S0_COL_IDX]
    }

    #[inline(always)]
    fn s1(&self) -> E {
        self.current()[S1_COL_IDX]
    }

    #[inline(always)]
    fn v(&self) -> E {
        self.current()[V_COL_IDX]
    }

    #[inline(always)]
    fn v_next(&self) -> E {
        self.next()[V_COL_IDX]
    }

    // --- Intermediate variables & helpers -------------------------------------------------------

    #[inline(always)]
    fn change(&self, column: usize) -> E {
        self.next()[column] - self.current()[column]
    }

    // --- Flags -------------------------------------------------------------------------

    #[inline(always)]
    fn flip_to_16bit_flag(&self) -> E {
        binary_not(self.t()) * self.t_next()
    }
}
