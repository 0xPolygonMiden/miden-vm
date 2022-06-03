use super::{EvaluationFrame, FieldElement, TransitionConstraintDegree, BITWISE_TRACE_OFFSET};
use crate::utils::is_binary;
use core::ops::Range;
use vm_core::{bitwise::NUM_SELECTORS, utils::range as create_range, Felt};

mod bitwise;
mod pow2;

#[cfg(test)]
pub mod tests;

// CONSTANTS
// ================================================================================================

/// The number of rows required to compute an operation in the Bitwise & Power of Two co-processor.
pub const OP_CYCLE_LEN: usize = 8;
/// Index of CONSTRAINT_DEGREES array after which all constraints use periodic columns.
const PERIODIC_CONSTRAINTS_START: usize = 2;
/// The number of shared constraints on the combined trace of the Bitwise and Power of Two
/// operation execution traces.
pub const NUM_CONSTRAINTS: usize = 4;
/// The degrees of constraints on the combined trace of the Bitwise and Power of Two traces. The
/// degree of all constraints is increased by 2 due to the co-processor selector flag from the
/// Auxiliary Table.
pub const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [
    4, 4, // Selector flags must be binary.
    4, 4, // Selector flags should remain the same throughout the cycle.
];
/// The range of indices for selector columns in the trace.
const SELECTOR_COL_RANGE: Range<usize> = create_range(BITWISE_TRACE_OFFSET, NUM_SELECTORS);
/// The starting index of the trace for the power of two operation.
const POW2_TRACE_OFFSET: usize = SELECTOR_COL_RANGE.end;

// PERIODIC COLUMNS
// ================================================================================================

/// Returns the set of periodic columns required by the Bitwise & Power of Two co-processor.
///
/// The columns consist of:
/// - k0 column, which has a repeating pattern of a single one followed by 7 zeros.
/// - k1 column, which has a repeating pattern of a 7 ones followed by a single zero.
pub fn get_periodic_column_values() -> Vec<Vec<Felt>> {
    vec![BITWISE_POW2_K0_MASK.to_vec(), BITWISE_POW2_K1_MASK.to_vec()]
}

// AUXILIARY TABLE TRANSITION CONSTRAINTS
// ================================================================================================

/// Builds the transition constraint degrees for the combined bitwise and power of two trace,
/// including the shared constraints and the constraints for each of the co-processors.
pub fn get_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    // Add the degrees of non-periodic constraints.
    let mut degrees: Vec<TransitionConstraintDegree> = CONSTRAINT_DEGREES
        [..PERIODIC_CONSTRAINTS_START]
        .iter()
        .map(|&degree| TransitionConstraintDegree::new(degree))
        .collect();

    // Add the degrees of periodic constraints.
    degrees.append(
        &mut CONSTRAINT_DEGREES[PERIODIC_CONSTRAINTS_START..]
            .iter()
            .map(|&degree| TransitionConstraintDegree::with_cycles(degree - 1, vec![OP_CYCLE_LEN]))
            .collect(),
    );

    degrees.append(&mut bitwise::get_transition_constraint_degrees());

    degrees.append(&mut pow2::get_transition_constraint_degrees());

    degrees
}

/// Returns the number of transition constraints for the combined bitwise and power of two trace,
/// including the shared constraints and the constraints for each of the co-processors.
pub fn get_transition_constraint_count() -> usize {
    NUM_CONSTRAINTS
        + bitwise::get_transition_constraint_count()
        + pow2::get_transition_constraint_count()
}

/// Enforces constraints for the combined bitwise and power of two trace, including the shared
/// constraints and the constraints for each of the co-processors.
pub fn enforce_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
    processor_flag: E,
) {
    // Selector transition constraints applying to both bitwise and power of two operations.
    enforce_selectors(frame, periodic_values, result, processor_flag);

    let mut constraint_offset = NUM_CONSTRAINTS;
    // bitwise transition constraints
    bitwise::enforce_constraints(
        frame,
        periodic_values,
        &mut result[constraint_offset..],
        processor_flag,
    );
    constraint_offset += bitwise::get_transition_constraint_count();

    // power of two transition constraints
    pow2::enforce_constraints(
        frame,
        periodic_values,
        &mut result[constraint_offset..],
        processor_flag * frame.pow2_flag(),
    );
}

// TRANSITION CONSTRAINT HELPERS
// ================================================================================================

/// Constraint evaluation function to enforce that the Bitwise and Power of Two selector columns
/// must be binary and they remain the same throughout the cycle.
fn enforce_selectors<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
    processor_flag: E,
) {
    let k1 = periodic_values[1];
    let mut constraint_offset = 0;

    // All selectors must be binary for the entire table.
    for (idx, result) in result.iter_mut().enumerate().take(NUM_SELECTORS) {
        *result = processor_flag * is_binary(frame.selector(idx));
    }

    constraint_offset += NUM_SELECTORS;

    // Selector values should stay the same for the entire cycle. In other words, the value can
    // only change when there is a transition to a new cycle i.e. from last row of a cycle and the
    // first row of the new cycle, when periodic column k1=0.
    for (idx, result) in result[constraint_offset..]
        .iter_mut()
        .enumerate()
        .take(NUM_SELECTORS)
    {
        *result = processor_flag * k1 * (frame.selector(idx) - frame.selector_next(idx));
    }
}

// BITWISE & POWER OF TWO FRAME EXTENSION TRAIT
// ================================================================================================

/// Trait to allow easy access to column values and intermediate variables used in constraint
/// calculations for the shared contraints and the Power of Two co-processor.
trait EvaluationFrameExt<E: FieldElement> {
    // --- Column accessors -----------------------------------------------------------------------

    /// Accessor for the selector column of the current row at the specified index.
    fn selector(&self, index: usize) -> E;

    /// Accessor for the selector column of the next row at the specified index.
    fn selector_next(&self, index: usize) -> E;

    // --- Co-processor selector flags ------------------------------------------------------------

    /// Flag to indicate if the frame is executing a power of two operation.
    fn pow2_flag(&self) -> E;
}

impl<E: FieldElement> EvaluationFrameExt<E> for &EvaluationFrame<E> {
    // --- Column accessors -----------------------------------------------------------------------

    #[inline(always)]
    fn selector(&self, index: usize) -> E {
        self.current()[SELECTOR_COL_RANGE.start + index]
    }

    #[inline(always)]
    fn selector_next(&self, index: usize) -> E {
        self.next()[SELECTOR_COL_RANGE.start + index]
    }

    // --- Co-processor selector flags ------------------------------------------------------------

    #[inline(always)]
    fn pow2_flag(&self) -> E {
        self.selector(0) * self.selector(1)
    }
}

// CYCLE MASKS
// ================================================================================================
pub const BITWISE_POW2_K0_MASK: [Felt; OP_CYCLE_LEN] = [
    Felt::ONE,
    Felt::ZERO,
    Felt::ZERO,
    Felt::ZERO,
    Felt::ZERO,
    Felt::ZERO,
    Felt::ZERO,
    Felt::ZERO,
];

pub const BITWISE_POW2_K1_MASK: [Felt; OP_CYCLE_LEN] = [
    Felt::ONE,
    Felt::ONE,
    Felt::ONE,
    Felt::ONE,
    Felt::ONE,
    Felt::ONE,
    Felt::ONE,
    Felt::ZERO,
];

// TEST HELPERS
// ================================================================================================

/// Returns the values from the shared bitwise & power of two processor's periodic columns for the
/// specified cycle row.
#[cfg(test)]
fn get_periodic_values(cycle_row: usize) -> [Felt; 2] {
    match cycle_row {
        0 => [Felt::ONE, Felt::ONE],
        8 => [Felt::ZERO, Felt::ZERO],
        _ => [Felt::ZERO, Felt::ONE],
    }
}
