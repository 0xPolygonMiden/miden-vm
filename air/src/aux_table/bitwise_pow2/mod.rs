use super::{EvaluationFrame, FieldElement, TransitionConstraintDegree, BITWISE_TRACE_OFFSET};
use crate::utils::is_binary;
use core::ops::Range;
use vm_core::{bitwise::NUM_SELECTORS, utils::range as create_range, Felt};

mod bitwise;
mod pow2;

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

/// The length of a periodic cycle in the Bitwise & Power of Two co-processor.
pub const PERIODIC_CYCLE_LEN: usize = 8;
/// The number of shared constraints on the combined trace of the Bitwise and Power of Two
/// operation execution traces.
pub const NUM_CONSTRAINTS: usize = 2;
/// The degrees of constraints on the combined trace of the Bitwise and Power of Two traces. The
/// degree of all constraints is increased by 2 due to the co-processor selector flag from the
/// Auxiliary Table.
pub const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [
    4, 4, // Selector flags must be binary.
];
/// The range of indices for selector columns in the trace.
const SELECTOR_COL_RANGE: Range<usize> = create_range(BITWISE_TRACE_OFFSET, NUM_SELECTORS);
/// The starting index of the trace for the power of two operation.
const POW2_TRACE_OFFSET: usize = SELECTOR_COL_RANGE.end;

// AUXILIARY TABLE TRANSITION CONSTRAINTS
// ================================================================================================

/// Builds the transition constraint degrees for the combined bitwise and power of two trace,
/// including the shared constraints and the constraints for each of the co-processors.
pub fn get_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    let mut degrees: Vec<TransitionConstraintDegree> = CONSTRAINT_DEGREES
        .iter()
        .map(|&degree| TransitionConstraintDegree::new(degree))
        .collect();

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
    enforce_selectors(frame, result, processor_flag);

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
/// must be binary.
fn enforce_selectors<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    processor_flag: E,
) {
    // All selectors must be binary for the entire table.
    for (idx, result) in result.iter_mut().enumerate().take(NUM_SELECTORS) {
        *result = processor_flag * is_binary(frame.selector(idx));
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

    // --- Co-processor selector flags ------------------------------------------------------------

    /// Flag to indicate if the frame is executing a power of two operation.
    fn pow2_flag(&self) -> E;
}

impl<E: FieldElement> EvaluationFrameExt<E> for &EvaluationFrame<E> {
    // --- Column accessors -----------------------------------------------------------------------

    fn selector(&self, index: usize) -> E {
        self.current()[SELECTOR_COL_RANGE.start + index]
    }

    // --- Co-processor selector flags ------------------------------------------------------------

    fn pow2_flag(&self) -> E {
        self.selector(0) * self.selector(1)
    }
}

// CYCLE MASKS
// ================================================================================================
pub const BITWISE_POW2_K0_MASK: [Felt; PERIODIC_CYCLE_LEN] = [
    Felt::ONE,
    Felt::ZERO,
    Felt::ZERO,
    Felt::ZERO,
    Felt::ZERO,
    Felt::ZERO,
    Felt::ZERO,
    Felt::ZERO,
];

pub const BITWISE_POW2_K1_MASK: [Felt; PERIODIC_CYCLE_LEN] = [
    Felt::ONE,
    Felt::ONE,
    Felt::ONE,
    Felt::ONE,
    Felt::ONE,
    Felt::ONE,
    Felt::ONE,
    Felt::ZERO,
];
