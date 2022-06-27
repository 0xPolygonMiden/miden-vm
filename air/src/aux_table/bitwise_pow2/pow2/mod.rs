use crate::utils::{are_equal, binary_not, is_binary, is_zero};

use super::{EvaluationFrame, FieldElement, Vec, OP_CYCLE_LEN, POW2_TRACE_OFFSET};
use core::ops::Range;
use vm_core::{bitwise::POW2_POWERS_PER_ROW, utils::range as create_range};
use winter_air::TransitionConstraintDegree;

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

/// The number of constraints on the power of two co-processor.
pub const NUM_CONSTRAINTS: usize = 25;
/// The degrees of constraints on the power of two co-processor. The degree of all constraints is
/// increased by 4 due to the co-processor selector flag from the auxiliary table (degree 2) and the
/// power of two operation selector flag (degree 2).
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
    6, 6, 7, // Ensure that the output is aggregated correctly.
];
/// Index of CONSTRAINT_DEGREES array after which all constraints use periodic columns.
const PERIODIC_CONSTRAINTS_START: usize = 17;

/// The index range of the columns containing the power decomposition of the input exponent a. These
/// are the columns a0 to a7.
const A_POWERS_COL_RANGE: Range<usize> = create_range(POW2_TRACE_OFFSET, POW2_POWERS_PER_ROW);
/// The index of the helper column `h` used to validate correct input decomposition and correctly
/// aggregate the output result.
const H_COL_IDX: usize = A_POWERS_COL_RANGE.end;
/// The index of the column containing the aggregated value of the powers of the input exponent that
/// have been decomposed into the trace.
const A_AGG_COL_IDX: usize = H_COL_IDX + 1;
/// The column `p` holding increasing powers of 256.
const P_COL_IDX: usize = A_AGG_COL_IDX + 1;
/// The index of the output column `z` at previous row.
const Z_AGG_PREV_COL_IDX: usize = P_COL_IDX + 1;
/// The index of the output column `z`.
const Z_AGG_COL_IDX: usize = Z_AGG_PREV_COL_IDX + 1;

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
            .map(|&degree| TransitionConstraintDegree::with_cycles(degree - 1, vec![OP_CYCLE_LEN]))
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

/// Enforces that the input value specifying the exponent for the power of two operation is
/// decomposed correctly into individual powers of two in the columns a0 to a7. It also enforces
/// constraints on the helper column h, used for output aggregation, and on the maximum exponent
/// value allowed as an input to the operation.
fn enforce_input_decomposition<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
    processor_flag: E,
) -> usize {
    let mut constraint_offset = 0;
    let k1 = periodic_values[1];

    // Values in power decomposition columns are all binary.
    for (idx, result) in result.iter_mut().take(POW2_POWERS_PER_ROW).enumerate() {
        *result = processor_flag * is_binary(frame.a_power(idx));
    }
    constraint_offset += POW2_POWERS_PER_ROW;

    // Adjacent cells in decomposition columns can stay the same or transition from 1 -> 0.
    for (idx, result) in result[constraint_offset..]
        .iter_mut()
        .take(POW2_POWERS_PER_ROW - 1)
        .enumerate()
    {
        *result = processor_flag * binary_not(frame.a_power(idx)) * frame.a_power(idx + 1);
    }
    constraint_offset += POW2_POWERS_PER_ROW - 1;

    // The helper column is binary.
    result[constraint_offset] = processor_flag * is_binary(frame.h());
    constraint_offset += 1;

    // The transition from a7 to the helper column can stay the same or change from 1 -> 0.
    result[constraint_offset] = processor_flag * binary_not(frame.a_power(7)) * frame.h();
    constraint_offset += 1;

    // The helper column value equals the value of a0 in the next row for all rows except the last.
    result[constraint_offset] = processor_flag * k1 * (frame.a_power_next(0) - frame.h());
    constraint_offset += 1;

    // Ensure input is never greater than the maximum exponent of the table by enforcing that the
    // last power decomposition cell is always zero.
    result[constraint_offset] = processor_flag * binary_not(k1) * frame.a_power(7);
    constraint_offset += 1;

    constraint_offset
}

/// Enforces that the value in column `a` equals the aggregation of all powers that have been
/// decomposed into the decomposition columns a0 to a7 since the beginning of the periodic cycle.
fn enforce_input_aggregation<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
    processor_flag: E,
) -> usize {
    let mut constraint_offset = 0;
    let k1 = periodic_values[1];

    // Aggregate the next row's decomposed powers.
    let agg_row_powers: E =
        (0..POW2_POWERS_PER_ROW).fold(E::ZERO, |r, idx| r + frame.a_power_next(idx));

    // Ensure the decomposed input is aggregated into column a.
    result[constraint_offset] =
        processor_flag * (frame.a_next() - (agg_row_powers + k1 * frame.a()));
    constraint_offset += 1;

    constraint_offset
}

/// Enforces that column `p` contains powers of 256 starting from 1 in the first row of each
/// periodic cycle and increasing by a factor of 256 in each subsequent row of the cycle.
fn enforce_powers_of_256<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
    processor_flag: E,
) -> usize {
    let mut constraint_offset = 0;
    let k0 = periodic_values[0];
    let k1 = periodic_values[1];

    result[constraint_offset] = processor_flag * k0 * (frame.p() - E::ONE);
    constraint_offset += 1;

    result[constraint_offset] =
        processor_flag * k1 * (frame.p_next() - E::from(256_u16) * frame.p());
    constraint_offset += 1;

    constraint_offset
}

/// The constraints to enforce correct aggregation of the output value. The constraints enforce:
/// - In the first row, `z_prev` should be set to 0.
/// - For all the rows except the last one, the next value of `z_prev` should be the same as
///   the current value of `z`.
/// - For all rows except the last one, the current output value (`z`) should equal the output value
///   copied from the previous row (`z_prev`) plus the aggregation of current row's decomposed power.
fn enforce_output_aggregation<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
    processor_flag: E,
) -> usize {
    let mut constraint_offset = 0;
    let k0 = periodic_values[0];
    let k1 = periodic_values[1];

    // Enforce value of `z_prev` output to 0 for the first row.
    result[constraint_offset] = processor_flag * k0 * is_zero(frame.z_prev());
    constraint_offset += 1;

    // For all rows except the last one, enforce the next value of `z_prev` is the same as
    // the current value of `z`.
    result[constraint_offset] = processor_flag * k1 * are_equal(frame.z_prev_next(), frame.z());
    constraint_offset += 1;

    // For all rows, enforce the current row's output is the aggregation of the current row's
    // decomposed powers and the current value of `z_prev`(the previous row's output value).
    let agg_row_output =
        (0..=POW2_POWERS_PER_ROW).fold(E::ZERO, |r, idx| r + frame.a_output(idx, k0));

    result[constraint_offset] =
        processor_flag * are_equal(frame.z(), frame.p() * agg_row_output + frame.z_prev());

    constraint_offset += 1;

    constraint_offset
}

// BITWISE FRAME EXTENSION TRAIT
// ================================================================================================
trait EvaluationFrameExt<E: FieldElement> {
    // --- Column accessors -----------------------------------------------------------------------

    /// The value in the power decomposition column with the specified index in the current row.
    fn a_power(&self, index: usize) -> E;
    /// The value in the power decomposition column with the specified index in the next row.
    fn a_power_next(&self, index: usize) -> E;
    /// The value in the helper column in the current row.
    fn h(&self) -> E;
    /// The value in the helper column in the next row.
    fn h_next(&self) -> E;
    /// The aggregated value of the input that has been decomposed so far in the current row.
    fn a(&self) -> E;
    /// The aggregated value of the input that has been decomposed so far in the next row.
    fn a_next(&self) -> E;
    /// The power of 256 in the current row.
    fn p(&self) -> E;
    /// The power of 256 in the next row.
    fn p_next(&self) -> E;
    /// The aggregated output value in the previous row.
    fn z_prev(&self) -> E;
    /// The aggregated output value in the next row's previous row.
    fn z_prev_next(&self) -> E;
    /// The aggregated output value in the current row.
    fn z(&self) -> E;
    /// The aggregated output value in the next row.
    fn z_next(&self) -> E;

    // --- Intermediate variables & helpers -------------------------------------------------------

    /// The aggregated output value at the specified index in the "virtual row" for the current row,
    /// where the virtual row is composed of the differences of adjacent values in the trace row and
    /// is used to track the end of the power decomposition, where the output should be aggregated.
    fn a_output(&self, index: usize, k0: E) -> E;
}

impl<E: FieldElement> EvaluationFrameExt<E> for &EvaluationFrame<E> {
    // --- Column accessors -----------------------------------------------------------------------

    #[inline(always)]
    fn a_power(&self, index: usize) -> E {
        self.current()[A_POWERS_COL_RANGE.start + index]
    }
    #[inline(always)]
    fn a_power_next(&self, index: usize) -> E {
        self.next()[A_POWERS_COL_RANGE.start + index]
    }
    #[inline(always)]
    fn h(&self) -> E {
        self.current()[H_COL_IDX]
    }
    #[inline(always)]
    fn h_next(&self) -> E {
        self.next()[H_COL_IDX]
    }
    #[inline(always)]
    fn a(&self) -> E {
        self.current()[A_AGG_COL_IDX]
    }
    #[inline(always)]
    fn a_next(&self) -> E {
        self.next()[A_AGG_COL_IDX]
    }
    #[inline(always)]
    fn p(&self) -> E {
        self.current()[P_COL_IDX]
    }
    #[inline(always)]
    fn p_next(&self) -> E {
        self.next()[P_COL_IDX]
    }
    #[inline(always)]
    fn z_prev(&self) -> E {
        self.current()[Z_AGG_PREV_COL_IDX]
    }
    #[inline(always)]
    fn z_prev_next(&self) -> E {
        self.next()[Z_AGG_PREV_COL_IDX]
    }
    #[inline(always)]
    fn z(&self) -> E {
        self.current()[Z_AGG_COL_IDX]
    }
    #[inline(always)]
    fn z_next(&self) -> E {
        self.next()[Z_AGG_COL_IDX]
    }

    // --- Intermediate variables & helpers -------------------------------------------------------

    #[inline(always)]
    fn a_output(&self, index: usize, k0: E) -> E {
        match index {
            // If the value of the zeroth index of the first row is 0 then it returns 1 and for all other
            // cases the the output of zeroth index is zero due to the selector column being 0 for rows other
            // than the first row.
            0 => binary_not(self.a_power(0)) * k0,
            8 => (self.a_power(index - 1) - self.h()) * E::from(2_u64.pow(index as u32)),
            _ => (self.a_power(index - 1) - self.a_power(index)) * E::from(2_u64.pow(index as u32)),
        }
    }
}
