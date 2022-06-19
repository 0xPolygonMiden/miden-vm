use super::{
    super::{
        get_periodic_values, EvaluationFrame, NUM_SELECTORS, OP_CYCLE_LEN, SELECTOR_COL_RANGE,
    },
    enforce_constraints, A_AGG_COL_IDX, A_POWERS_COL_RANGE, H_COL_IDX, NUM_CONSTRAINTS,
    POW2_POWERS_PER_ROW, P_COL_IDX, Z_AGG_COL_IDX, Z_AGG_PREV_COL_IDX,
};
use vm_core::{bitwise::POWER_OF_TWO, Felt, FieldElement, TRACE_WIDTH};

use proptest::prelude::*;

// RANDOMIZED TESTS
// ================================================================================================

proptest! {
    #[test]
    fn test_pow2(exponent in 0_u32..64, cycle_row in 0..(OP_CYCLE_LEN - 1)) {
        let expected = [Felt::ZERO; NUM_CONSTRAINTS];
        let frame = get_test_frame(exponent, cycle_row);
        let result = get_constraint_evaluation(frame, cycle_row);
        assert_eq!(expected, result);
    }
}

// TEST HELPERS
// ================================================================================================

/// Returns the result of Power of Two constraint evaluations on the provided frame starting at the
/// specified row.
fn get_constraint_evaluation(frame: EvaluationFrame<Felt>, row: usize) -> [Felt; NUM_CONSTRAINTS] {
    let periodic_values = get_periodic_values(row);
    let mut result = [Felt::ZERO; NUM_CONSTRAINTS];

    enforce_constraints(&frame, &periodic_values, &mut result, Felt::ONE);

    result
}

/// Generates the correct current and next rows for the power of two operation with the specified
/// input exponent at the specified cycle row number, then returns an EvaluationFrame for testing.
/// It only tests frames within a cycle.
///
/// # Errors
/// It expects the specified `cycle_row_num` for the current row to be such that the next row will
/// still be in the same cycle. It will fail if the row number input is >= OP_CYCLE_LEN - 1.
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
    frame_row[P_COL_IDX] = Felt::new(256_u64.pow((row_num % OP_CYCLE_LEN) as u32));

    // The output is zero before it is aggregated.
}

/// Fill a frame row with the expected values for a power of two operation row where the input
/// decomposition finishes and the output is aggregated. This function expects that the values
/// in the frame_row have been initialized to ZERO.
fn set_output_agg_row(exponent: u32, row_num: usize, frame_row: &mut [Felt]) {
    let final_decomp_row_power = exponent as usize - row_num * POW2_POWERS_PER_ROW;

    for idx in 0..final_decomp_row_power {
        frame_row[A_POWERS_COL_RANGE.start + idx] = Felt::ONE;
    }
    // The rest of the helper and power decomposition columns should be left as zero.

    // After decomposition is finished, the value of a is the input exponent.
    frame_row[A_AGG_COL_IDX] = Felt::new(exponent as u64);

    // Set the power of 256 column value.
    frame_row[P_COL_IDX] = Felt::new(256_u64.pow((row_num % OP_CYCLE_LEN) as u32));

    // The output at the previous row is zero before it is aggregated.

    // After aggregation, the output is the result.
    frame_row[Z_AGG_COL_IDX] = Felt::new(2_u64.pow(exponent));
}

/// Fill a frame row with the expected values for a power of two operation row after the output
/// aggregation has been done. This function expects that the values in the frame_row have been
/// initialized to ZERO.
fn set_post_output_agg_row(exponent: u32, row_num: usize, frame_row: &mut [Felt]) {
    // The power decomposition and helper columns are zero after decomposition is finished.

    // After decomposition is finished, the value of a is the input exponent.
    frame_row[A_AGG_COL_IDX] = Felt::new(exponent as u64);

    // Set the power of 256 column value.
    frame_row[P_COL_IDX] = Felt::new(256_u64.pow((row_num % OP_CYCLE_LEN) as u32));

    // After aggregation, the output at previous row should be equal to output.
    frame_row[Z_AGG_PREV_COL_IDX] = Felt::new(2_u64.pow(exponent));

    // After aggregation, the output is the result.
    frame_row[Z_AGG_COL_IDX] = Felt::new(2_u64.pow(exponent));
}
