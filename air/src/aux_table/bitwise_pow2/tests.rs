use super::bitwise::{NUM_CONSTRAINTS, NUM_DECOMP_BITS};
use super::{bitwise, pow2, EvaluationFrame, BITWISE_TRACE_OFFSET, PERIODIC_CYCLE_LEN};
use crate::{Felt, FieldElement};
use vm_core::{
    bitwise::{
        Selectors, BITWISE_AND, BITWISE_OR, BITWISE_XOR, NUM_SELECTORS,
        TRACE_WIDTH as BITWISE_TRACE_WIDTH,
    },
    TRACE_WIDTH,
};

use proptest::prelude::*;

// CONSTANTS
// ================================================================================================

/// The index of the column holding the aggregated value of input `a`.
const A_COL_IDX: usize = 2;
/// The index of the column holding the aggregated value of input `b`.
const B_COL_IDX: usize = 3;
/// The index of the column containing the aggregated output value.
const OUTPUT_COL_IDX: usize = 12;
/// The index of the column from where the input decomposition starts in a cycle row.
const BITWISE_DECOMPOSITION_OFFSET: usize = 4;
/// Number of columns needed for input decomposition of `a` & `b` in a cycle row.
const BITWISE_DECOMPOSITION_SIZE: usize = 8;

// UNIT TESTS
// ================================================================================================

/// Takes two custom consecutive row cycle state of bitwise decompostion with two distinct selectors. Outputs failure
/// as all constraint are not zero (not satisfied)
#[test]
fn test_bitwise_selectors_fail() {
    let current_bitwise = vec![
        // decomposed a
        Felt::ONE,
        Felt::ONE,
        Felt::ZERO,
        Felt::ZERO,
        // decomposed b
        Felt::ONE,
        Felt::ZERO,
        Felt::ONE,
        Felt::ONE,
    ];

    let next_bitwise = vec![
        Felt::ONE,
        Felt::ONE,
        Felt::ONE,
        Felt::ZERO,
        Felt::ZERO,
        Felt::ONE,
        Felt::ONE,
        Felt::ONE,
    ];
    let cycle = 1;
    let expected = [Felt::ZERO; bitwise::NUM_CONSTRAINTS];

    let frame =
        get_test_frame_with_two_selectors(&current_bitwise, &next_bitwise, BITWISE_AND, BITWISE_OR);
    let result = get_bitwise_frame_result(frame, cycle);
    assert_ne!(result, expected);

    let frame = get_test_frame_with_two_selectors(
        &current_bitwise,
        &next_bitwise,
        BITWISE_XOR,
        BITWISE_AND,
    );
    let result = get_bitwise_frame_result(frame, cycle);
    assert_ne!(result, expected);

    let frame =
        get_test_frame_with_two_selectors(&current_bitwise, &next_bitwise, BITWISE_OR, BITWISE_XOR);
    let result = get_bitwise_frame_result(frame, cycle);
    assert_ne!(result, expected);
}

proptest! {
    #[test]
    fn test_bitwise_and(a in any::<u32>(), b in any::<u32>(), cycle_row in 0..(PERIODIC_CYCLE_LEN - 1)) {
        test_bitwise_frame(BITWISE_AND, a, b, cycle_row);
    }

    #[test]
    fn test_bitwise_or(a in any::<u32>(), b in any::<u32>(), cycle_row in 0..(PERIODIC_CYCLE_LEN - 1)) {
        test_bitwise_frame(BITWISE_OR, a, b, cycle_row);
    }

    #[test]
    fn test_bitwise_xor(a in any::<u32>(), b in any::<u32>(), cycle_row in 0..(PERIODIC_CYCLE_LEN - 1)) {
        test_bitwise_frame(BITWISE_XOR, a, b, cycle_row);
    }

    #[test]
    fn test_pow2(exponent in 0_u32..64, cycle_row in 0..(PERIODIC_CYCLE_LEN - 1)) {
        test_pow2_frame(exponent, cycle_row);
    }
}

// TEST HELPERS
// ================================================================================================

/// Returns the values from the shared bitwise & power of two processor's periodic columns for the
/// specified cycle row.
fn get_test_periodic_values(cycle_row: usize) -> [Felt; 2] {
    match cycle_row {
        0 => [Felt::ONE, Felt::ONE],
        8 => [Felt::ZERO, Felt::ZERO],
        _ => [Felt::ZERO, Felt::ONE],
    }
}

/// Generates the specified trace frame for the specified bitwise operation and inputs, then asserts
/// that applying the constraints to this frame yields valid results (all zeros).
fn test_bitwise_frame(operation: Selectors, a: u32, b: u32, cycle_row: usize) {
    let frame = bitwise::get_test_frame(operation, a, b, cycle_row);
    let periodic_values = get_test_periodic_values(cycle_row);
    let mut result = [Felt::ZERO; bitwise::NUM_CONSTRAINTS];
    let expected = result;

    bitwise::enforce_constraints(&frame, &periodic_values, &mut result, Felt::ONE);

    assert_eq!(expected, result);
}

/// Generates the trace frame of an customised two consecutive cycle row. Accepts decomposed input
/// containing bits wise component of both a and b for the consecutive cycle row and two selectors.
/// Returns the frame with these two consecutive cycle row.  
fn get_test_frame_with_two_selectors(
    curr: &[Felt],
    next: &[Felt],
    operation1: Selectors,
    operation2: Selectors,
) -> EvaluationFrame<Felt> {
    // Hardcoded values of `a`,`b` and `output` values of previous cycle row.
    let prev_input_a = Felt::new(10);
    let prev_input_b = Felt::new(9);
    let prev_output = Felt::new(8);

    let mut current_cycle_row = vec![Felt::ZERO; TRACE_WIDTH];
    let mut next_cycle_row = vec![Felt::ZERO; TRACE_WIDTH];

    let mut current_bitstate = vec![Felt::ZERO; BITWISE_TRACE_WIDTH];
    let mut next_bitstate = vec![Felt::ZERO; BITWISE_TRACE_WIDTH];

    // calculating the input values of current and next cycle row.
    let input_curr_a = aggregate_input_bits(&curr[0..NUM_DECOMP_BITS], prev_input_a);
    let input_curr_b =
        aggregate_input_bits(&curr[NUM_DECOMP_BITS..NUM_DECOMP_BITS * 2], prev_input_b);
    let input_next_a = aggregate_input_bits(&next[0..NUM_DECOMP_BITS], input_curr_a);
    let input_next_b =
        aggregate_input_bits(&next[NUM_DECOMP_BITS..NUM_DECOMP_BITS * 2], input_curr_b);

    // calculating the ouptut values of current and next cycle row.
    let output_agg_curr_op1 = aggregate_row_output(&curr, operation1, prev_output);
    let output_agg_next_op1 = aggregate_row_output(&next, operation1, output_agg_curr_op1);

    // First two columns would be selector.
    // Assign operation2 and operation1 to the current and next cycle row respectively.
    for idx in 0..NUM_SELECTORS {
        current_bitstate[idx] = operation2[idx];
        next_bitstate[idx] = operation1[idx];
    }

    current_bitstate[A_COL_IDX] = input_curr_a;
    current_bitstate[B_COL_IDX] = input_curr_b;
    next_bitstate[A_COL_IDX] = input_next_a;
    next_bitstate[B_COL_IDX] = input_next_b;

    current_bitstate
        [BITWISE_DECOMPOSITION_OFFSET..BITWISE_DECOMPOSITION_OFFSET + BITWISE_DECOMPOSITION_SIZE]
        .copy_from_slice(&curr);
    next_bitstate
        [BITWISE_DECOMPOSITION_OFFSET..BITWISE_DECOMPOSITION_OFFSET + BITWISE_DECOMPOSITION_SIZE]
        .copy_from_slice(&next);

    current_bitstate[OUTPUT_COL_IDX] = output_agg_curr_op1;
    next_bitstate[OUTPUT_COL_IDX] = output_agg_next_op1;

    current_cycle_row[BITWISE_TRACE_OFFSET..BITWISE_TRACE_OFFSET + BITWISE_TRACE_WIDTH]
        .copy_from_slice(&current_bitstate);
    next_cycle_row[BITWISE_TRACE_OFFSET..BITWISE_TRACE_OFFSET + BITWISE_TRACE_WIDTH]
        .copy_from_slice(&next_bitstate);

    let frame = EvaluationFrame::<Felt>::from_rows(current_cycle_row, next_cycle_row);

    frame
}

/// Generates the expected and calculated constraint result on an input frame.
fn get_bitwise_frame_result(frame: EvaluationFrame<Felt>, row: usize) -> [Felt; NUM_CONSTRAINTS] {
    let cycle_row = get_test_periodic_values(row);
    let mut result = [Felt::ZERO; NUM_CONSTRAINTS];

    bitwise::enforce_constraints(&frame, &cycle_row, &mut result, Felt::ONE);

    result
}

/// Aggregates the decomposed value of input bits.
fn aggregate_input_bits(bits_array: &[Felt], prev_input: Felt) -> Felt {
    bitwise::agg_bits(bits_array, 0) + prev_input * Felt::new(2_u64.pow(4))
}

/// Generates the output value from decomposed input using a specified input operation (AND, OR, XOR)
fn aggregate_row_output(bits_array: &[Felt], operation: Selectors, prev_output: Felt) -> Felt {
    let output = if operation == BITWISE_AND {
        bitwise::bitwise_and(bits_array) + prev_output * Felt::new(2_u64.pow(4))
    } else if operation == BITWISE_OR {
        bitwise::bitwise_or(bits_array) + prev_output * Felt::new(2_u64.pow(4))
    } else if operation == BITWISE_XOR {
        bitwise::bitwise_xor(bits_array) + prev_output * Felt::new(2_u64.pow(4))
    } else {
        panic!("unrecognized operation.");
    };

    output
}

/// Generates the specified trace frame for the specified power of two operation, then asserts
/// that applying the constraints to this frame yields valid results (all zeros).
fn test_pow2_frame(exponent: u32, cycle_row: usize) {
    let frame = pow2::get_test_frame(exponent, cycle_row);
    let periodic_values = get_test_periodic_values(cycle_row);
    let mut result = [Felt::ZERO; pow2::NUM_CONSTRAINTS];
    let expected = result;

    pow2::enforce_constraints(&frame, &periodic_values, &mut result, Felt::ONE);

    assert_eq!(expected, result);
}
