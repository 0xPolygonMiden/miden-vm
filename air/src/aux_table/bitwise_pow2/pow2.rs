use crate::utils::{binary_not, is_binary};

use super::{EvaluationFrame, FieldElement, POW2_TRACE_OFFSET};
use core::ops::Range;
use vm_core::{bitwise::POW2_POWERS_PER_ROW, utils::range as create_range};
use winter_air::TransitionConstraintDegree;

// CONSTANTS
// ================================================================================================

/// The number of constraints on the management of the power of two co-processor.
pub const NUM_CONSTRAINTS: usize = 24;
/// The degrees of constraints on the management of the power of two co-processor. The degree of all
/// constraints is increased by 4 due to the co-processor selector flag from the auxiliary table
/// (degree 2) and the power of two operation selector flag (degree 2).
pub const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [
    // --- INPUT DECOMPOSITION --------------------------------------------------------------------
    6, 6, 6, 6, 6, 6, 6, 6, // All power decomposition columns are binary.
    6, 6, 6, 6, 6, 6,
    6, // Adjacent power decomposition cells can only transition from 1 -> 1 or 1 -> 0.
    6, // The helper column must be binary.
    6, // The transition between a7 and helper column can only be 1 -> 1 or 1 -> 0.
    6, // The value of the helper column must equal the next value of a0, except in the last row.
    6, // Ensure the input value a never exceeds the maximum allowed exponent value for the table.
    6, // Constrain the input aggregation.
    6, 6, // Enforce that the powers of 256 start at 1 and increase by 256 with each row.
    6, 7, // Ensure that the output is aggregated correctly.
];
/// Index of CONSTRAINT_DEGREES array after which all constraints use periodic columns.
const PERIODIC_CONSTRAINTS_START: usize = 17;

const A_POWERS_COL_RANGE: Range<usize> = create_range(POW2_TRACE_OFFSET, POW2_POWERS_PER_ROW);
const H_COL_IDX: usize = A_POWERS_COL_RANGE.end;
const A_AGG_COL_IDX: usize = H_COL_IDX + 1;
const P_COL_IDX: usize = A_AGG_COL_IDX + 1;
const Z_AGG_COL_IDX: usize = P_COL_IDX + 1;

// POWER OF TWO TRANSITION CONSTRAINTS
// ================================================================================================

/// Builds the transition constraint degrees for the power of two co-processor.
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
            .map(|&degree| TransitionConstraintDegree::with_cycles(degree - 1, vec![8]))
            .collect(),
    );

    degrees
}

/// Returns the number of transition constraints for the power of two co-processor.
pub fn get_transition_constraint_count() -> usize {
    NUM_CONSTRAINTS
}

/// Enforces constraints for the power of two co-processor.
pub fn enforce_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
    processor_flag: E,
) {
    let mut index = enforce_input_decomposition(frame, periodic_values, result, processor_flag);

    index +=
        enforce_input_aggregation(frame, periodic_values, &mut result[index..], processor_flag);

    index += enforce_powers_of_256(frame, periodic_values, &mut result[index..], processor_flag);

    enforce_output_aggregation(frame, periodic_values, &mut result[index..], processor_flag);
}

// TRANSITION CONSTRAINT HELPERS
// ================================================================================================

fn enforce_input_decomposition<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
    processor_flag: E,
) -> usize {
    let num_constraints = 19;
    let k1 = periodic_values[1];

    // Values in power decomposition columns are all binary.
    for (idx, result) in result.iter_mut().enumerate().take(POW2_POWERS_PER_ROW) {
        *result = processor_flag * is_binary(frame.a_power(idx));
    }
    let mut constraint_index = POW2_POWERS_PER_ROW;

    // Adjacent cells in decomposition columns can stay the same or transition from 1 -> 0.
    for (idx, result) in result[constraint_index..]
        .iter_mut()
        .enumerate()
        .take(POW2_POWERS_PER_ROW - 1)
    {
        *result = processor_flag * binary_not(frame.a_power(idx)) * frame.a_power(idx + 1);
    }
    constraint_index += POW2_POWERS_PER_ROW - 1;

    // The helper column is binary.
    result[constraint_index] = processor_flag * is_binary(frame.h());
    constraint_index += 1;

    // The transition from a7 to the helper column can stay the same or change from 1 -> 0.
    result[constraint_index] = processor_flag * binary_not(frame.a_power(7)) * frame.h();
    constraint_index += 1;

    // The helper column value equals the value of a0 in the next row for all rows except the last.
    result[constraint_index] = processor_flag * k1 * (frame.a_power_next(0) - frame.h());
    constraint_index += 1;

    // Ensure input is never greater than the maximum exponent of the table by enforcing that the
    // last power decomposition cell is always zero.
    result[constraint_index] = processor_flag * binary_not(k1) * frame.a_power(7);

    num_constraints
}

fn enforce_input_aggregation<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
    processor_flag: E,
) -> usize {
    let num_constraints = 1;
    let k1 = periodic_values[1];

    // Aggregate the next row's decomposed powers.
    let agg_row_powers: E =
        (0..POW2_POWERS_PER_ROW).fold(E::ZERO, |r, idx| r + frame.a_power_next(idx));

    // Ensure the decomposed input is aggregated into column a.
    result[0] = processor_flag * (frame.a_next() - (agg_row_powers + k1 * frame.a()));

    num_constraints
}

fn enforce_powers_of_256<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
    processor_flag: E,
) -> usize {
    let num_constraints = 2;
    let k0 = periodic_values[0];
    let k1 = periodic_values[1];

    result[0] = processor_flag * k0 * (frame.p() - E::ONE);
    result[1] = processor_flag * k1 * (frame.p_next() - E::from(256_u16) * frame.p());

    num_constraints
}

fn enforce_output_aggregation<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
    processor_flag: E,
) -> usize {
    let num_constraints = 2;
    let k0 = periodic_values[0];
    let k1 = periodic_values[1];

    // Enforce correct output aggregation in the first row.
    let agg_row_output = (0..=POW2_POWERS_PER_ROW).fold(E::ZERO, |r, idx| r + frame.a_output(idx));
    result[0] = processor_flag * k0 * (frame.z() - agg_row_output);

    // For all rows except the last, enforce the next row's output is the aggregation of the
    // decomposed powers in the next row and the output value in the current row.
    let agg_next_row_output =
        (0..=POW2_POWERS_PER_ROW).fold(E::ZERO, |r, idx| r + frame.a_output_next(idx));
    result[1] =
        processor_flag * k1 * (frame.z_next() - (frame.p_next() * agg_next_row_output + frame.z()));

    num_constraints
}

// BITWISE FRAME EXTENSION TRAIT
// ================================================================================================
trait EvaluationFrameExt<E: FieldElement> {
    // --- Column accessors -----------------------------------------------------------------------

    fn a_power(&self, index: usize) -> E;
    fn a_power_next(&self, index: usize) -> E;
    fn h(&self) -> E;
    fn h_next(&self) -> E;
    fn a(&self) -> E;
    fn a_next(&self) -> E;
    fn p(&self) -> E;
    fn p_next(&self) -> E;
    fn z(&self) -> E;
    fn z_next(&self) -> E;

    // --- Intermediate variables & helpers -------------------------------------------------------
    fn a_output(&self, index: usize) -> E;
    fn a_output_next(&self, index: usize) -> E;

    // --- Flags ----------------------------------------------------------------------------------
}

impl<E: FieldElement> EvaluationFrameExt<E> for &EvaluationFrame<E> {
    // --- Column accessors -----------------------------------------------------------------------

    fn a_power(&self, index: usize) -> E {
        self.current()[A_POWERS_COL_RANGE.start + index]
    }
    fn a_power_next(&self, index: usize) -> E {
        self.next()[A_POWERS_COL_RANGE.start + index]
    }
    fn h(&self) -> E {
        self.current()[H_COL_IDX]
    }
    fn h_next(&self) -> E {
        self.next()[H_COL_IDX]
    }
    fn a(&self) -> E {
        self.current()[A_AGG_COL_IDX]
    }
    fn a_next(&self) -> E {
        self.next()[A_AGG_COL_IDX]
    }
    fn p(&self) -> E {
        self.current()[P_COL_IDX]
    }
    fn p_next(&self) -> E {
        self.next()[P_COL_IDX]
    }
    fn z(&self) -> E {
        self.current()[Z_AGG_COL_IDX]
    }
    fn z_next(&self) -> E {
        self.next()[Z_AGG_COL_IDX]
    }

    // --- Intermediate variables & helpers -------------------------------------------------------

    fn a_output(&self, index: usize) -> E {
        match index {
            0 => binary_not(self.a_power(0)),
            8 => (self.a_power(index - 1) - self.h()) * E::from(2_u64.pow(index as u32)),
            _ => (self.a_power(index - 1) - self.a_power(index)) * E::from(2_u64.pow(index as u32)),
        }
    }
    fn a_output_next(&self, index: usize) -> E {
        match index {
            0 => E::ZERO,
            8 => (self.a_power_next(7) - self.h_next()) * E::from(2_u64.pow(index as u32)),
            _ => {
                (self.a_power_next(index - 1) - self.a_power_next(index))
                    * E::from(2_u64.pow(index as u32))
            }
        }
    }

    // --- Flags ----------------------------------------------------------------------------------
}
