use crate::{
    chiplets::{ChipletsFrameExt, MemoryFrameExt},
    trace::range::{B_RANGE_COL_IDX, Q_COL_IDX, S0_COL_IDX, S1_COL_IDX, V_COL_IDX},
    utils::{are_equal, binary_not, is_binary},
    Assertion, EvaluationFrame, Felt, FieldElement, TransitionConstraintDegree,
};
use vm_core::{utils::collections::Vec, ExtensionOf};
use winter_air::AuxTraceRandElements;

// CONSTANTS
// ================================================================================================

// --- Main constraints ---------------------------------------------------------------------------

/// The number of boundary constraints required by the Range Checker
pub const NUM_ASSERTIONS: usize = 2;
/// The number of transition constraints required by the Range Checker.
pub const NUM_CONSTRAINTS: usize = 3;
/// The degrees of the range checker's constraints, in the order they'll be added to the the result
/// array when a transition is evaluated.
pub const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [
    2, 2, // Selector flags must be binary: s0, s1.
    9, // Enforce values of column v transition.
];

// --- Auxiliary column constraints for multiset checks -------------------------------------------

/// The number of auxiliary assertions for multiset checks.
pub const NUM_AUX_ASSERTIONS: usize = 2;
/// The number of transition constraints required by multiset checks for the Range Checker.
pub const NUM_AUX_CONSTRAINTS: usize = 1;
/// The degrees of the Range Checker's auxiliary column constraints, used for multiset checks.
pub const AUX_CONSTRAINT_DEGREES: [usize; NUM_AUX_CONSTRAINTS] = [7];

// BOUNDARY CONSTRAINTS
// ================================================================================================

// --- MAIN TRACE ---------------------------------------------------------------------------------

/// Returns the range checker's boundary assertions for the main trace at the first step.
pub fn get_assertions_first_step(result: &mut Vec<Assertion<Felt>>) {
    let step = 0;
    result.push(Assertion::single(V_COL_IDX, step, Felt::ZERO));
}

/// Returns the range checker's boundary assertions for the main trace at the last step.
pub fn get_assertions_last_step(result: &mut Vec<Assertion<Felt>>, step: usize) {
    result.push(Assertion::single(V_COL_IDX, step, Felt::new(65535)));
}

// --- AUXILIARY COLUMNS (FOR MULTISET CHECKS) ----------------------------------------------------

/// Returns the range checker's boundary assertions for auxiliary columns at the first step.
pub fn get_aux_assertions_first_step<E: FieldElement>(result: &mut Vec<Assertion<E>>) {
    let step = 0;
    result.push(Assertion::single(B_RANGE_COL_IDX, step, E::ONE));
}

/// Returns the range checker's boundary assertions for auxiliary columns at the last step.
pub fn get_aux_assertions_last_step<E: FieldElement>(result: &mut Vec<Assertion<E>>, step: usize) {
    result.push(Assertion::single(B_RANGE_COL_IDX, step, E::ONE));
}

// TRANSITION CONSTRAINTS
// ================================================================================================

// --- MAIN TRACE ---------------------------------------------------------------------------------

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
    let index = enforce_flags(frame, result);

    // Constrain the transition between rows of the range checker table.
    enforce_delta(frame, &mut result[index..]);
}

// --- AUXILIARY COLUMNS (FOR MULTISET CHECKS) ----------------------------------------------------

/// Returns the transition constraint degrees for the range checker's auxiliary columns, used for
/// multiset checks.
pub fn get_aux_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    AUX_CONSTRAINT_DEGREES
        .iter()
        .map(|&degree| TransitionConstraintDegree::new(degree))
        .collect()
}

/// Enforces constraints on the range checker's auxiliary columns.
pub fn enforce_aux_constraints<F, E>(
    main_frame: &EvaluationFrame<F>,
    aux_frame: &EvaluationFrame<E>,
    aux_rand_elements: &AuxTraceRandElements<E>,
    result: &mut [E],
) where
    F: FieldElement<BaseField = Felt>,
    E: FieldElement<BaseField = Felt> + ExtensionOf<F>,
{
    // Get the first random element for this segment.
    let alpha = aux_rand_elements.get_segment_elements(0)[0];

    // Enforce b_range.
    enforce_running_product_b_range(main_frame, aux_frame, alpha, &mut result[..]);
}

// TRANSITION CONSTRAINT HELPERS
// ================================================================================================

// --- MAIN TRACE ---------------------------------------------------------------------------------

/// Constrain the selector flags to binary values.
fn enforce_flags<E: FieldElement>(frame: &EvaluationFrame<E>, result: &mut [E]) -> usize {
    let constraint_count = 2;

    result[0] = is_binary(frame.s0());
    result[1] = is_binary(frame.s1());

    constraint_count
}

/// Constrain the transition between rows in the range checker table.
fn enforce_delta<E: FieldElement>(frame: &EvaluationFrame<E>, result: &mut [E]) -> usize {
    let constraint_count = 1;

    result[0] = frame.change(V_COL_IDX)
        * (frame.change(V_COL_IDX) - E::ONE)
        * (frame.change(V_COL_IDX) - E::from(3_u8))
        * (frame.change(V_COL_IDX) - E::from(9_u8))
        * (frame.change(V_COL_IDX) - E::from(27_u8))
        * (frame.change(V_COL_IDX) - E::from(81_u8))
        * (frame.change(V_COL_IDX) - E::from(243_u8))
        * (frame.change(V_COL_IDX) - E::from(729_u16))
        * (frame.change(V_COL_IDX) - E::from(2187_u16));

    constraint_count
}

// --- AUXILIARY COLUMNS (FOR MULTISET CHECKS) ----------------------------------------------------

/// Ensures that the running product is computed correctly in the column `b_range`. It enforces
/// that the value only changes after the padded rows, where the value of `z` is included at each
/// step, ensuring that the values in the range checker table are multiplied into `b_range` 0, 1,
/// 2, or 4 times, according to the selector flags.
fn enforce_running_product_b_range<E, F>(
    main_frame: &EvaluationFrame<F>,
    aux_frame: &EvaluationFrame<E>,
    alpha: E,
    result: &mut [E],
) where
    F: FieldElement<BaseField = Felt>,
    E: FieldElement<BaseField = Felt> + ExtensionOf<F>,
{
    // The running product column must enforce that the next step has the values from the range
    // checker multiplied in (z) and the values from the stack (q) and the memory divided out. This
    // is enforced by ensuring that b_range_next multiplied by the stack and memory lookups at this step
    // is equal to the combination of b_range and the range checker's values for this step.
    let lookups = aux_frame.q() * get_memory_lookups(main_frame, alpha);
    let range_checks = get_z(main_frame, alpha);

    result[0] = are_equal(aux_frame.b_range_next() * lookups, aux_frame.b_range() * range_checks);
}

/// The value to be included in the running product column for memory lookups at this row. These are
/// only included for steps in the memory section of the trace (when the memory_flag is one).
fn get_memory_lookups<E, F>(main_frame: &EvaluationFrame<F>, alpha: E) -> E
where
    F: FieldElement<BaseField = Felt>,
    E: FieldElement<BaseField = Felt> + ExtensionOf<F>,
{
    let memory_flag: E = main_frame.chiplets_memory_flag().into();
    let d0: E = main_frame.memory_d0().into();
    let d1: E = main_frame.memory_d1().into();

    E::ONE + memory_flag * ((d0 + alpha) * (d1 + alpha) - E::ONE)
}

/// Returns the value `z` which is included in the running product columns at each step. `z` causes
/// the row's value to be included 0, 1, 2, or 4 times, according to the row's selector flags row.
fn get_z<E, F>(main_frame: &EvaluationFrame<F>, alpha: E) -> E
where
    F: FieldElement<BaseField = Felt>,
    E: FieldElement<BaseField = Felt> + ExtensionOf<F>,
{
    // Get the selectors and the value from the main frame.
    let s0: E = main_frame.s0().into();
    let s1: E = main_frame.s1().into();
    let v: E = main_frame.v().into();

    // Define the flags.
    let f0: E = binary_not(s0) * binary_not(s1);
    let f1: E = s0 * binary_not(s1);
    let f2: E = binary_not(s0) * s1;
    let f3: E = s0 * s1;

    // Compute z.
    let v_alpha = v + alpha;
    let v_alpha2 = v_alpha.square();
    let v_alpha4 = v_alpha2.square();
    f3 * v_alpha4 + f2 * v_alpha2 + f1 * v_alpha + f0
}

// RANGE CHECKER FRAME EXTENSION TRAIT
// ================================================================================================

/// Trait to allow easy access to column values and intermediate variables used in constraint
/// calculations for the Range Checker.
trait EvaluationFrameExt<E: FieldElement> {
    // --- Column accessors -----------------------------------------------------------------------

    fn s0(&self) -> E;
    /// The current value in column s1.
    fn s1(&self) -> E;
    /// The current value in column V.
    fn v(&self) -> E;
    /// The next value in column V.
    fn v_next(&self) -> E;
    /// The current value in auxiliary column b_range.
    fn b_range(&self) -> E;

    /// The next value in auxiliary column b_range.
    fn b_range_next(&self) -> E;

    /// The current value in auxiliary column q.
    fn q(&self) -> E;

    // --- Intermediate variables & helpers -------------------------------------------------------

    /// The change between the current value in the specified column and the next value, calculated
    /// as `next - current`.
    fn change(&self, column: usize) -> E;
}

impl<E: FieldElement> EvaluationFrameExt<E> for &EvaluationFrame<E> {
    // --- Column accessors -----------------------------------------------------------------------

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

    #[inline(always)]
    fn b_range(&self) -> E {
        self.current()[B_RANGE_COL_IDX]
    }

    #[inline(always)]
    fn b_range_next(&self) -> E {
        self.next()[B_RANGE_COL_IDX]
    }

    #[inline(always)]
    fn q(&self) -> E {
        self.current()[Q_COL_IDX]
    }

    // --- Intermediate variables & helpers -------------------------------------------------------

    #[inline(always)]
    fn change(&self, column: usize) -> E {
        self.next()[column] - self.current()[column]
    }
}
