use super::{
    Felt, OverflowTableRow, ProgramInputs, Stack, StackTopState, MIN_STACK_DEPTH, ONE, ZERO,
};
use vm_core::{FieldElement, StarkField, NUM_STACK_HELPER_COLS};

#[test]
fn initialize() {
    // initialize a new stack with some initial values
    let mut stack_inputs = [1, 2, 3, 4];
    let inputs = ProgramInputs::new(&stack_inputs, &[], vec![]).unwrap();
    let stack = Stack::new(&inputs, 4, false);

    // Prepare the expected results.
    stack_inputs.reverse();
    let expected_stack = build_stack(&stack_inputs);
    let expected_helpers = [Felt::new(MIN_STACK_DEPTH as u64), ZERO, ZERO];

    // Check the stack state.
    assert_eq!(stack.trace_state(), expected_stack);

    // Check the helper columns.
    assert_eq!(
        stack.trace.get_helpers_state_at(stack.current_clk()),
        expected_helpers
    );
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
    let expected_stack = build_stack(&stack_inputs[..MIN_STACK_DEPTH]);
    let expected_depth = stack_inputs.len() as u64;
    let expected_helpers = [
        Felt::new(expected_depth),
        -ONE,
        Felt::new(expected_depth - MIN_STACK_DEPTH as u64).inv(),
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
    assert_eq!(
        stack.trace.get_helpers_state_at(stack.current_clk()),
        expected_helpers
    );

    // Check the overflow table state.
    assert_eq!(stack.overflow.active_rows(), expected_overflow_active_rows);
    assert_eq!(stack.overflow.all_rows(), expected_overflow_rows);
}

#[test]
fn shift_left() {
    let inputs = ProgramInputs::new(&[1, 2, 3, 4], &[], vec![]).unwrap();
    let mut stack = Stack::new(&inputs, 4, false);

    // ---- left shift an entire stack of minimum depth -------------------------------------------
    // Prepare the expected results.
    let expected_stack = build_stack(&[3, 2, 1]);
    let expected_helpers = build_helpers_left(0, 0);

    // Perform the left shift.
    stack.shift_left(1);
    stack.advance_clock();

    // Check the stack state.
    assert_eq!(stack.trace_state(), expected_stack);

    // Check the helper columns.
    assert_eq!(
        stack.trace.get_helpers_state_at(stack.current_clk()),
        expected_helpers
    );

    // ---- left shift an entire stack with multiple overflow items -------------------------------
    let mut stack = Stack::new(&inputs, 4, false);
    // Shift right twice to add 2 items to the overflow table.
    stack.shift_right(0);
    let prev_overflow_addr = stack.current_clk() as usize;
    stack.advance_clock();
    stack.shift_right(0);
    stack.advance_clock();

    // Prepare the expected results.
    let expected_stack = build_stack(&[0, 4, 3, 2, 1]);
    let expected_helpers = build_helpers_left(1, prev_overflow_addr);

    // Perform the left shift.
    stack.shift_left(1);
    stack.advance_clock();

    // Check the stack state.
    assert_eq!(stack.trace_state(), expected_stack);

    // Check the helper columns.
    assert_eq!(
        stack.trace.get_helpers_state_at(stack.current_clk()),
        expected_helpers
    );

    // ---- left shift an entire stack with one overflow item -------------------------------------
    // Prepare the expected results.
    let expected_stack = build_stack(&[4, 3, 2, 1]);
    let expected_helpers = build_helpers_left(0, 0);

    // Perform the left shift.
    stack.ensure_trace_capacity();
    stack.shift_left(1);
    stack.advance_clock();

    // Check the stack state.
    assert_eq!(stack.trace_state(), expected_stack);

    // Check the helper columns.
    assert_eq!(
        stack.trace.get_helpers_state_at(stack.current_clk()),
        expected_helpers
    );
}

#[test]
fn shift_right() {
    let inputs = ProgramInputs::new(&[1, 2, 3, 4], &[], vec![]).unwrap();
    let mut stack = Stack::new(&inputs, 4, false);

    // ---- right shift an entire stack of minimum depth ------------------------------------------
    let expected_stack = build_stack(&[0, 4, 3, 2, 1]);
    let expected_helpers = build_helpers_right(1, stack.current_clk());

    stack.shift_right(0);
    stack.advance_clock();

    // Check the stack state.
    assert_eq!(stack.trace_state(), expected_stack);

    // Check the helper columns.
    assert_eq!(
        stack.trace.get_helpers_state_at(stack.current_clk()),
        expected_helpers
    );

    // ---- right shift when the overflow table is non-empty --------------------------------------
    let expected_stack = build_stack(&[0, 0, 4, 3, 2, 1]);
    let expected_helpers = build_helpers_right(2, stack.current_clk());

    stack.shift_right(0);
    stack.advance_clock();

    // Check the stack state.
    assert_eq!(stack.trace_state(), expected_stack);

    // Check the helper columns.
    assert_eq!(
        stack.trace.get_helpers_state_at(stack.current_clk()),
        expected_helpers
    );
}

// HELPERS
// ================================================================================================

/// Builds the trace row of stack helpers expected as the result of a right shift at clock cycle
/// `clk` when there are `num_overflow` items in the overflow table.
fn build_helpers_right(num_overflow: usize, clk: u32) -> [Felt; NUM_STACK_HELPER_COLS] {
    let b0 = Felt::new((MIN_STACK_DEPTH + num_overflow) as u64);
    let b1 = Felt::new(clk as u64);
    let h0 = ONE / (b0 - Felt::new(MIN_STACK_DEPTH as u64));

    [b0, b1, h0]
}

/// Builds the trace row of stack helpers expected as the result of a left shift when there are
/// `num_overflow` items in the overflow table and the top row in the table has address
/// `next_overflow_addr`.
fn build_helpers_left(
    num_overflow: usize,
    next_overflow_addr: usize,
) -> [Felt; NUM_STACK_HELPER_COLS] {
    let depth = MIN_STACK_DEPTH + num_overflow;
    let b0 = Felt::new(depth as u64);
    let b1 = Felt::new(next_overflow_addr as u64);
    let h0 = if depth > MIN_STACK_DEPTH {
        ONE / (b0 - Felt::new(MIN_STACK_DEPTH as u64))
    } else {
        ZERO
    };

    [b0, b1, h0]
}

/// Builds a [StackTopState] that starts with the provided stack inputs and is padded with zeros
/// until the minimum stack depth.
fn build_stack(stack_inputs: &[u64]) -> StackTopState {
    let mut expected_stack = [ZERO; MIN_STACK_DEPTH];
    for (idx, &input) in stack_inputs.iter().enumerate() {
        expected_stack[idx] = Felt::new(input);
    }

    expected_stack
}
