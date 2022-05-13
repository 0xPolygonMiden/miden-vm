use crate::utils::{binary_not, is_binary};

use super::{EvaluationFrame, FieldElement, POW2_TRACE_OFFSET};
use core::ops::Range;
use vm_core::{bitwise::POW2_POWERS_PER_ROW, utils::range as create_range};
use winter_air::TransitionConstraintDegree;

// CONSTANTS
// ================================================================================================

/// The number of constraints on the power of two co-processor.
pub const NUM_CONSTRAINTS: usize = 24;
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
    6, 7, // Ensure that the output is aggregated correctly.
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
/// The index of the output column `z`.
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

/// Enforces that the value in column `a` equals the aggregation of all powers that have been
/// decomposed into the decomposition columns a0 to a7 since the beginning of the periodic cycle.
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

/// Enforces that column `p` contains powers of 256 starting from 1 in the first row of each
/// periodic cycle and increasing by a factor of 256 in each subsequent row of the cycle.
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

/// The constraints to enforce correct aggregation of the output value. The constraints enforce:
/// - If the power decomposition ends in the first row, then the aggregated output of the decomposed
///   powers equals the value in the output column.
/// - For all rows except the last, the output value in the next row will equal the value in the
///   current row plus the next row's power of 256 times the aggregation of its decomposed powers.
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
    /// The aggregated output value in the current row.
    fn z(&self) -> E;
    /// The aggregated output value in the next row.
    fn z_next(&self) -> E;

    // --- Intermediate variables & helpers -------------------------------------------------------

    /// The aggregated output value at the specified index in the "virtual row" for the current row,
    /// where the virtual row is composed of the differences of adjacent values in the trace row and
    /// is used to track the end of the power decomposition, where the output should be aggregated.
    fn a_output(&self, index: usize) -> E;
    /// The aggregated output value at the specified index in the "virtual row" for the next row,
    /// where the virtual row is composed of the differences of adjacent values in the trace row and
    /// is used to track the end of the power decomposition, where the output should be aggregated.
    fn a_output_next(&self, index: usize) -> E;
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
}

// TEST HELPER FUNCTIONS
// ================================================================================================
#[cfg(test)]
use super::{NUM_SELECTORS, PERIODIC_CYCLE_LEN, SELECTOR_COL_RANGE};
#[cfg(test)]
use vm_core::{bitwise::POWER_OF_TWO, Felt, TRACE_WIDTH};

/// Generates the correct current and next rows for the power of two operation with the specified
/// input exponent at the specified cycle row number, then returns an EvaluationFrame for testing.
/// It only tests frames within a cycle.
///
/// # Errors
/// It expects the specified `cycle_row_num` for the current row to be such that the next row will
/// still be in the same cycle. It will fail with a row number input.
#[cfg(test)]
pub fn get_test_frame(exponent: u32, cycle_row_num: usize) -> EvaluationFrame<Felt> {
    let mut current = vec![Felt::ZERO; TRACE_WIDTH];
    let mut next = vec![Felt::ZERO; TRACE_WIDTH];

    // Set the operation selectors.
    for idx in 0..NUM_SELECTORS {
        current[SELECTOR_COL_RANGE.start + idx] = POWER_OF_TWO[idx];
        next[SELECTOR_COL_RANGE.start + idx] = POWER_OF_TWO[idx];
    }

    // The index of the final decomposition row.
    let mut final_decomp_row = exponent as usize / POW2_POWERS_PER_ROW;
    if exponent != 0 && exponent % POW2_POWERS_PER_ROW as u32 == 0 {
        // Multiples of 8 are aggregated in the previous row, except for 0.
        // Both 0 and 8 are aggregated in the first row.
        final_decomp_row -= 1;
    }

    if (cycle_row_num + 1) < final_decomp_row {
        // Both rows in this frame come before the decomposition finishes.
        set_pre_output_agg_row(cycle_row_num, &mut current);
        set_pre_output_agg_row(cycle_row_num + 1, &mut next);
    } else if cycle_row_num > final_decomp_row {
        // Both rows in this frame come after the decomposition & aggregation have finished.
        set_post_output_agg_row(exponent, cycle_row_num, &mut current);
        set_post_output_agg_row(exponent, cycle_row_num + 1, &mut next);
    } else if cycle_row_num == final_decomp_row {
        // The current row is the output aggregation row.
        set_output_agg_row(exponent, cycle_row_num, &mut current);
        set_post_output_agg_row(exponent, cycle_row_num + 1, &mut next);
    } else {
        // The next row is the output aggregation row.
        set_pre_output_agg_row(cycle_row_num, &mut current);
        set_output_agg_row(exponent, cycle_row_num + 1, &mut next);
    }

    EvaluationFrame::<Felt>::from_rows(current, next)
}

/// Fill a frame row with the expected values for a power of two operation row before the input
/// decomposition finishes. In this case, all input decomposition columns in the row will be set
/// to ONE. This function expects that the values in the frame_row have been initialized to
/// ZERO.
#[cfg(test)]
fn set_pre_output_agg_row(row_num: usize, frame_row: &mut [Felt]) {
    // Set the power decomposition and helper columns to one while decomposition is continuing.
    for cell in frame_row
        .iter_mut()
        .take(H_COL_IDX + 1)
        .skip(A_POWERS_COL_RANGE.start)
    {
        *cell = Felt::ONE;
    }

    // set the input aggregation column
    frame_row[A_AGG_COL_IDX] = Felt::new((POW2_POWERS_PER_ROW * (row_num + 1)) as u64);

    // Set the power of 256 column value.
    frame_row[P_COL_IDX] = Felt::new(256_u64.pow((row_num % PERIODIC_CYCLE_LEN) as u32));

    // The output is zero before it is aggregated.
}

/// Fill a frame row with the expected values for a power of two operation row where the input
/// decomposition finishes and the output is aggregated. This function expects that the values
/// in the frame_row have been initialized to ZERO.
#[cfg(test)]
fn set_output_agg_row(exponent: u32, row_num: usize, frame_row: &mut [Felt]) {
    let final_decomp_row_power = exponent as usize - row_num * POW2_POWERS_PER_ROW;

    for idx in 0..final_decomp_row_power {
        frame_row[A_POWERS_COL_RANGE.start + idx] = Felt::ONE;
    }
    // The rest of the helper and power decomposition columns should be left as zero.

    // After decomposition is finished, the value of a is the input exponent.
    frame_row[A_AGG_COL_IDX] = Felt::new(exponent as u64);

    // Set the power of 256 column value.
    frame_row[P_COL_IDX] = Felt::new(256_u64.pow((row_num % PERIODIC_CYCLE_LEN) as u32));

    // After aggregation, the output is the result.
    frame_row[Z_AGG_COL_IDX] = Felt::new(2_u64.pow(exponent));
}

/// Fill a frame row with the expected values for a power of two operation row after the output
/// aggregation has been done. This function expects that the values in the frame_row have been
/// initialized to ZERO.
#[cfg(test)]
fn set_post_output_agg_row(exponent: u32, row_num: usize, frame_row: &mut [Felt]) {
    // The power decomposition and helper columns are zero after decomposition is finished.

    // After decomposition is finished, the value of a is the input exponent.
    frame_row[A_AGG_COL_IDX] = Felt::new(exponent as u64);

    // Set the power of 256 column value.
    frame_row[P_COL_IDX] = Felt::new(256_u64.pow((row_num % PERIODIC_CYCLE_LEN) as u32));

    // After aggregation, the output is the result.
    frame_row[Z_AGG_COL_IDX] = Felt::new(2_u64.pow(exponent));
}
