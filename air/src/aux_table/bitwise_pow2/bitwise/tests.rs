use super::{
    super::{get_periodic_values, OP_CYCLE_LEN},
    agg_bits, bitwise_and, bitwise_or, bitwise_xor, enforce_constraints, EvaluationFrame,
    A_COL_IDX, A_COL_RANGE, BITWISE_TRACE_OFFSET, B_COL_IDX, B_COL_RANGE, NUM_CONSTRAINTS,
    NUM_DECOMP_BITS, OUTPUT_COL_IDX, SELECTOR_COL_RANGE,
};
use vm_core::{
    bitwise::{
        Selectors, BITWISE_AND, BITWISE_A_COL_IDX, BITWISE_B_COL_IDX, BITWISE_OR,
        BITWISE_OUTPUT_COL_IDX, BITWISE_XOR, NUM_SELECTORS, TRACE_WIDTH as BITWISE_TRACE_WIDTH,
    },
    Felt, FieldElement, TRACE_WIDTH,
};

use proptest::prelude::*;

// CONSTANTS
// ================================================================================================

/// The index of the column where the input decomposition starts in the bitwise execution trace.
const BITWISE_DECOMPOSITION_OFFSET: usize = BITWISE_B_COL_IDX + 1;
/// Number of columns needed for input decomposition of `a` & `b` in the bitwise execution trace.
const BITWISE_DECOMPOSITION_LEN: usize = 2 * NUM_DECOMP_BITS;

// UNIT TESTS
// ================================================================================================

/// Tests that the bitwise constraints do not all evaluate to zero if the operation selectors change
/// within a cycle.
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
    let expected = [Felt::ZERO; NUM_CONSTRAINTS];

    let frame =
        get_test_frame_with_two_selectors(&current_bitwise, &next_bitwise, BITWISE_AND, BITWISE_OR);
    let result = get_constraint_evaluation(frame, cycle);
    assert_ne!(result, expected);

    let frame = get_test_frame_with_two_selectors(
        &current_bitwise,
        &next_bitwise,
        BITWISE_XOR,
        BITWISE_AND,
    );
    let result = get_constraint_evaluation(frame, cycle);
    assert_ne!(result, expected);

    let frame =
        get_test_frame_with_two_selectors(&current_bitwise, &next_bitwise, BITWISE_OR, BITWISE_XOR);
    let result = get_constraint_evaluation(frame, cycle);
    assert_ne!(result, expected);
}

// RANDOMIZED TESTS
// ================================================================================================

proptest! {
    /// Tests that the bitwise constraints evaluate to zero on valid frames within a cycle which
    /// compute the bitwise AND operation.
    #[test]
    fn test_bitwise_and(a in any::<u32>(), b in any::<u32>(), cycle_row in 0..(OP_CYCLE_LEN - 1)) {
        let expected = [Felt::ZERO; NUM_CONSTRAINTS];
        let frame = get_test_frame(BITWISE_AND, a, b, cycle_row);
        let result = get_constraint_evaluation(frame, cycle_row);
        assert_eq!(expected, result);
    }

    /// Tests that the bitwise constraints evaluate to zero on valid frames within a cycle which
    /// compute the bitwise OR operation.
    #[test]
    fn test_bitwise_or(a in any::<u32>(), b in any::<u32>(), cycle_row in 0..(OP_CYCLE_LEN - 1)) {
        let expected = [Felt::ZERO; NUM_CONSTRAINTS];
        let frame = get_test_frame(BITWISE_OR, a, b, cycle_row);
        let result = get_constraint_evaluation(frame, cycle_row);
        assert_eq!(expected, result);
    }

    /// Tests that the bitwise constraints evaluate to zero on valid frames within a cycle which
    /// compute the bitwise XOR operation.
    #[test]
    fn test_bitwise_xor(a in any::<u32>(), b in any::<u32>(), cycle_row in 0..(OP_CYCLE_LEN - 1)) {
        let expected = [Felt::ZERO; NUM_CONSTRAINTS];
        let frame = get_test_frame(BITWISE_XOR, a, b, cycle_row);
        let result = get_constraint_evaluation(frame, cycle_row);
        assert_eq!(expected, result);
    }
}

// TEST HELPERS
// ================================================================================================

/// Returns the result of Bitwise constraint evaluations on the provided frame starting at the
/// specified row.
fn get_constraint_evaluation(frame: EvaluationFrame<Felt>, row: usize) -> [Felt; NUM_CONSTRAINTS] {
    let periodic_values = get_periodic_values(row);
    let mut result = [Felt::ZERO; NUM_CONSTRAINTS];

    enforce_constraints(&frame, &periodic_values, &mut result, Felt::ONE);

    result
}

/// Generates the correct current and next rows for the specified operation, inputs, and current
/// cycle row number and returns an EvaluationFrame for testing. It only tests frames within a
/// cycle.
///
/// # Errors
/// It expects the specified `cycle_row_num` for the current row to be such that the next row will
/// still be in the same cycle. It will fail if the row number input is >= OP_CYCLE_LEN - 1.
pub fn get_test_frame(
    operation: Selectors,
    a: u32,
    b: u32,
    cycle_row_num: usize,
) -> EvaluationFrame<Felt> {
    assert!(
        cycle_row_num < OP_CYCLE_LEN - 1,
        "Failed to build test EvaluationFrame for bitwise operation. The next row would be in a new cycle."
    );

    // Initialize the rows.
    let mut current = vec![Felt::ZERO; TRACE_WIDTH];
    let mut next = vec![Felt::ZERO; TRACE_WIDTH];

    // Define the shift amounts for the specified rows.
    let current_shift = NUM_DECOMP_BITS * (OP_CYCLE_LEN - cycle_row_num - 1);
    let next_shift = current_shift - NUM_DECOMP_BITS;

    // Set the operation selectors.
    for idx in 0..NUM_SELECTORS {
        current[SELECTOR_COL_RANGE.start + idx] = operation[idx];
        next[SELECTOR_COL_RANGE.start + idx] = operation[idx];
    }

    // Set the input aggregation values.
    let current_a = (a >> current_shift) as u64;
    let current_b = (b >> current_shift) as u64;
    let next_a = (a >> next_shift) as u64;
    let next_b = (b >> next_shift) as u64;

    current[A_COL_IDX] = Felt::new(current_a);
    next[A_COL_IDX] = Felt::new(next_a);
    current[B_COL_IDX] = Felt::new(current_b);
    next[B_COL_IDX] = Felt::new(next_b);

    // Set the input decomposition values.
    for idx in 0..NUM_DECOMP_BITS {
        current[A_COL_RANGE.start + idx] = Felt::new((current_a >> idx) & 1);
        current[B_COL_RANGE.start + idx] = Felt::new((current_b >> idx) & 1);
        next[A_COL_RANGE.start + idx] = Felt::new((next_a >> idx) & 1);
        next[B_COL_RANGE.start + idx] = Felt::new((next_b >> idx) & 1);
    }

    // Set the output.
    let output = if operation == BITWISE_AND {
        a & b
    } else if operation == BITWISE_OR {
        a | b
    } else if operation == BITWISE_XOR {
        a ^ b
    } else {
        panic!("Test bitwise EvaluationFrame requested for unrecognized operation.");
    };
    current[OUTPUT_COL_IDX] = Felt::new((output >> current_shift) as u64);
    next[OUTPUT_COL_IDX] = Felt::new((output >> next_shift) as u64);

    EvaluationFrame::<Felt>::from_rows(current, next)
}

/// Generates the trace frame of an customised two consecutive cycle row. Accepts decomposed input
/// containing bits wise component of both a and b for the consecutive cycle row and two selectors.
/// Returns the frame with these two consecutive cycle row.  
pub fn get_test_frame_with_two_selectors(
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
    let output_agg_curr_op1 = aggregate_row_output(curr, operation1, prev_output);
    let output_agg_next_op1 = aggregate_row_output(next, operation1, output_agg_curr_op1);

    // First two columns would be selector.
    // Assign operation2 and operation1 to the current and next cycle row respectively.
    current_bitstate[..NUM_SELECTORS].copy_from_slice(&operation2);
    next_bitstate[..NUM_SELECTORS].copy_from_slice(&operation1);

    current_bitstate[BITWISE_A_COL_IDX] = input_curr_a;
    current_bitstate[BITWISE_B_COL_IDX] = input_curr_b;
    next_bitstate[BITWISE_A_COL_IDX] = input_next_a;
    next_bitstate[BITWISE_B_COL_IDX] = input_next_b;

    current_bitstate
        [BITWISE_DECOMPOSITION_OFFSET..BITWISE_DECOMPOSITION_OFFSET + BITWISE_DECOMPOSITION_LEN]
        .copy_from_slice(curr);
    next_bitstate
        [BITWISE_DECOMPOSITION_OFFSET..BITWISE_DECOMPOSITION_OFFSET + BITWISE_DECOMPOSITION_LEN]
        .copy_from_slice(next);

    current_bitstate[BITWISE_OUTPUT_COL_IDX] = output_agg_curr_op1;
    next_bitstate[BITWISE_OUTPUT_COL_IDX] = output_agg_next_op1;

    current_cycle_row[BITWISE_TRACE_OFFSET..BITWISE_TRACE_OFFSET + BITWISE_TRACE_WIDTH]
        .copy_from_slice(&current_bitstate);
    next_cycle_row[BITWISE_TRACE_OFFSET..BITWISE_TRACE_OFFSET + BITWISE_TRACE_WIDTH]
        .copy_from_slice(&next_bitstate);

    EvaluationFrame::<Felt>::from_rows(current_cycle_row, next_cycle_row)
}

/// Aggregates the decomposed value of input bits.
fn aggregate_input_bits(bits_array: &[Felt], prev_input: Felt) -> Felt {
    agg_bits(bits_array, 0) + prev_input * Felt::new(2_u64.pow(4))
}

/// Generates the output value from decomposed input using a specified input operation (AND, OR, XOR)
fn aggregate_row_output(bits_array: &[Felt], operation: Selectors, prev_output: Felt) -> Felt {
    let output = if operation == BITWISE_AND {
        bitwise_and(bits_array) + prev_output * Felt::new(2_u64.pow(4))
    } else if operation == BITWISE_OR {
        bitwise_or(bits_array) + prev_output * Felt::new(2_u64.pow(4))
    } else if operation == BITWISE_XOR {
        bitwise_xor(bits_array) + prev_output * Felt::new(2_u64.pow(4))
    } else {
        panic!("unrecognized operation.");
    };

    output
}
