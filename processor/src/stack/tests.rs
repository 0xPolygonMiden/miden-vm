use super::{Felt, OverflowTableRow, ProgramInputs, Stack, ONE, STACK_TOP_SIZE, ZERO};
use crate::StackTopState;
use vm_core::{
    stack::{B0_COL_IDX, B1_COL_IDX, H0_COL_IDX, NUM_STACK_HELPER_COLS},
    FieldElement, StarkField, STACK_TRACE_WIDTH,
};

// TYPE ALIASES
// ================================================================================================

type StackHelpersState = [Felt; NUM_STACK_HELPER_COLS];

// INITIALIZATION TESTS
// ================================================================================================

#[test]
fn initialize() {
    // initialize a new stack with some initial values
    let mut stack_inputs = [1, 2, 3, 4];
    let inputs = ProgramInputs::new(&stack_inputs, &[], vec![]).unwrap();
    let stack = Stack::new(&inputs, 4, false);

    // Prepare the expected results.
    stack_inputs.reverse();
    let expected_stack = build_stack(&stack_inputs);
    let expected_helpers = [Felt::new(STACK_TOP_SIZE as u64), ZERO, ZERO];

    // Check the stack state.
    assert_eq!(stack.trace_state(), expected_stack);

    // Check the helper columns.
    assert_eq!(stack.helpers_state(), expected_helpers);
}

#[test]
fn initialize_overflow() {
    // Initialize a new stack with enough initial values that the overflow table is non-empty.
    let mut stack_inputs = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ];
    let inputs = ProgramInputs::new(&stack_inputs, &[], vec![]).unwrap();
    let stack = Stack::new(&inputs, 4, false);

    // Prepare the expected results.
    stack_inputs.reverse();
    let expected_stack = build_stack(&stack_inputs[..STACK_TOP_SIZE]);
    let expected_depth = stack_inputs.len() as u64;
    let expected_helpers = [
        Felt::new(expected_depth),
        -ONE,
        Felt::new(expected_depth - STACK_TOP_SIZE as u64),
    ];
    let init_addr = Felt::MODULUS - 3;
    let expected_overflow_rows = vec![
        OverflowTableRow::new(init_addr, Felt::new(1), ZERO),
        OverflowTableRow::new(init_addr + 1, Felt::new(2), Felt::new(init_addr)),
        OverflowTableRow::new(init_addr + 2, Felt::new(3), Felt::new(init_addr + 1)),
    ];
    let expected_overflow_active_rows = vec![0, 1, 2];

    // Check the stack state.
    assert_eq!(stack.trace_state(), expected_stack);

    // Check the helper columns.
    assert_eq!(stack.helpers_state(), expected_helpers);

    // Check the overflow table state.
    assert_eq!(stack.overflow.active_rows(), expected_overflow_active_rows);
    assert_eq!(stack.overflow.all_rows(), expected_overflow_rows);
}

#[test]
fn shift_left() {
    let inputs = ProgramInputs::new(&[1, 2, 3, 4], &[], vec![]).unwrap();
    let mut stack = Stack::new(&inputs, 4, false);

    // ---- left shift an entire stack of minimum depth -------------------------------------------
    // Perform the left shift.
    stack.shift_left(1);
    stack.advance_clock();

    // Check the state of stack item and helper columns.
    assert_eq!(stack.trace_state(), build_stack(&[3, 2, 1]));
    assert_eq!(stack.helpers_state(), build_helpers_partial(0, 0));

    // ---- left shift an entire stack with multiple overflow items -------------------------------
    let mut stack = Stack::new(&inputs, 4, false);

    // make sure the first right shift is not executed at clk = 0
    stack.copy_state(0);
    stack.advance_clock();

    // Shift right twice to add 2 items to the overflow table.
    stack.shift_right(0);
    let prev_overflow_addr = stack.current_clk() as usize;
    stack.advance_clock();
    stack.shift_right(0);
    stack.advance_clock();

    // Perform the left shift.
    stack.ensure_trace_capacity();
    stack.shift_left(1);
    stack.advance_clock();

    // Check the state of stack item and helper columns.
    assert_eq!(stack.trace_state(), build_stack(&[0, 4, 3, 2, 1]));
    assert_eq!(
        stack.helpers_state(),
        build_helpers_partial(1, prev_overflow_addr)
    );

    // ---- left shift an entire stack with one overflow item -------------------------------------

    // Perform the left shift.
    stack.shift_left(1);
    stack.advance_clock();

    // Check the state of stack item and helper columns.
    assert_eq!(stack.trace_state(), build_stack(&[4, 3, 2, 1]));
    assert_eq!(stack.helpers_state(), build_helpers_partial(0, 0));
}

#[test]
fn shift_right() {
    let inputs = ProgramInputs::new(&[1, 2, 3, 4], &[], vec![]).unwrap();
    let mut stack = Stack::new(&inputs, 4, false);

    // make sure the first right shift is not executed at clk = 0
    stack.copy_state(0);
    stack.advance_clock();

    // ---- right shift an entire stack of minimum depth ------------------------------------------
    let expected_stack = build_stack(&[0, 4, 3, 2, 1]);
    let expected_helpers = build_helpers_partial(1, stack.current_clk() as usize);

    stack.shift_right(0);
    stack.advance_clock();

    // Check the stack state.
    assert_eq!(stack.trace_state(), expected_stack);

    // Check the helper columns.
    assert_eq!(stack.helpers_state(), expected_helpers);

    // ---- right shift when the overflow table is non-empty --------------------------------------
    let expected_stack = build_stack(&[0, 0, 4, 3, 2, 1]);
    let expected_helpers = build_helpers_partial(2, stack.current_clk() as usize);

    stack.shift_right(0);
    stack.advance_clock();

    // Check the stack state.
    assert_eq!(stack.trace_state(), expected_stack);

    // Check the helper columns.
    assert_eq!(stack.helpers_state(), expected_helpers);
}

// TRACE GENERATION
// ================================================================================================

#[test]
fn generate_trace() {
    let inputs = ProgramInputs::new(&[1, 2, 3, 4], &[], vec![]).unwrap();
    let mut stack = Stack::new(&inputs, 16, false);

    stack.copy_state(0);
    stack.advance_clock();

    stack.shift_right(0);
    stack.advance_clock();

    stack.shift_right(0);
    stack.advance_clock();

    stack.shift_left(1);
    stack.advance_clock();

    stack.copy_state(0);
    stack.advance_clock();

    stack.shift_left(1);
    stack.advance_clock();

    stack.copy_state(0);
    stack.advance_clock();

    let trace = stack.into_trace(8, 1);
    let trace = trace.trace;

    assert_eq!(read_stack_top(&trace, 0), build_stack(&[4, 3, 2, 1]));
    assert_eq!(read_stack_top(&trace, 1), build_stack(&[4, 3, 2, 1]));
    assert_eq!(read_stack_top(&trace, 2), build_stack(&[0, 4, 3, 2, 1]));
    assert_eq!(read_stack_top(&trace, 3), build_stack(&[0, 0, 4, 3, 2, 1]));
    assert_eq!(read_stack_top(&trace, 4), build_stack(&[0, 4, 3, 2, 1]));
    assert_eq!(read_stack_top(&trace, 5), build_stack(&[0, 4, 3, 2, 1]));
    assert_eq!(read_stack_top(&trace, 6), build_stack(&[4, 3, 2, 1]));
    assert_eq!(read_stack_top(&trace, 7), build_stack(&[4, 3, 2, 1]));

    assert_eq!(read_helpers(&trace, 0), build_helpers(16, 0));
    assert_eq!(read_helpers(&trace, 1), build_helpers(16, 0));
    assert_eq!(read_helpers(&trace, 2), build_helpers(17, 1));
    assert_eq!(read_helpers(&trace, 3), build_helpers(18, 2));
    assert_eq!(read_helpers(&trace, 4), build_helpers(17, 1));
    assert_eq!(read_helpers(&trace, 5), build_helpers(17, 1));
    assert_eq!(read_helpers(&trace, 6), build_helpers(16, 0));
    assert_eq!(read_helpers(&trace, 7), build_helpers(16, 0));
}

// HELPERS
// ================================================================================================

/// Builds a [StackTopState] that starts with the provided stack inputs and is padded with zeros
/// until the minimum stack depth.
fn build_stack(stack_inputs: &[u64]) -> StackTopState {
    let mut result = [ZERO; STACK_TOP_SIZE];
    for (idx, &input) in stack_inputs.iter().enumerate() {
        result[idx] = Felt::new(input);
    }
    result
}

/// Builds expected values of stack helper registers for the specified parameters.
fn build_helpers(stack_depth: u64, next_overflow_addr: u64) -> StackHelpersState {
    let b0 = Felt::new(stack_depth as u64);
    let b1 = Felt::new(next_overflow_addr as u64);
    let h0 = (b0 - Felt::new(STACK_TOP_SIZE as u64)).inv();

    [b0, b1, h0]
}

/// Builds expected values of stack helper registers prior to finalization of execution trace.
/// The difference between this function and build_helpers() is that this function does not invert
/// h0 value.
fn build_helpers_partial(num_overflow: usize, next_overflow_addr: usize) -> StackHelpersState {
    let depth = STACK_TOP_SIZE + num_overflow;
    let b0 = Felt::new(depth as u64);
    let b1 = Felt::new(next_overflow_addr as u64);
    let h0 = b0 - Felt::new(STACK_TOP_SIZE as u64);

    [b0, b1, h0]
}

/// Returns values in stack top columns of the provided trace at the specified row.
fn read_stack_top(trace: &[Vec<Felt>; STACK_TRACE_WIDTH], row: usize) -> StackTopState {
    let mut result = [ZERO; STACK_TOP_SIZE];
    for (value, column) in result.iter_mut().zip(trace) {
        *value = column[row];
    }
    result
}

/// Returns values in the stack helper columns of the provided trace in the specified row.
fn read_helpers(trace: &[Vec<Felt>; STACK_TRACE_WIDTH], row: usize) -> StackHelpersState {
    [
        trace[B0_COL_IDX][row],
        trace[B1_COL_IDX][row],
        trace[H0_COL_IDX][row],
    ]
}
