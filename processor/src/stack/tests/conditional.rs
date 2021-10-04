use super::{get_stack_state, init_stack, OpCode, OpHint, TRACE_LENGTH};

// CHOOSE OPERATIONS
// ================================================================================================

#[test]
fn choose() {
    // choose on false
    let mut stack = init_stack(&[2, 3, 0], &[], &[], TRACE_LENGTH);
    stack.execute(OpCode::Choose, OpHint::None);
    assert_eq!(vec![3, 0, 0, 0, 0, 0, 0, 0], get_stack_state(&stack, 1));

    assert_eq!(1, stack.depth);
    assert_eq!(3, stack.max_depth);

    let mut stack = init_stack(&[2, 3, 0, 4], &[], &[], TRACE_LENGTH);
    stack.execute(OpCode::Choose, OpHint::None);
    assert_eq!(vec![3, 4, 0, 0, 0, 0, 0, 0], get_stack_state(&stack, 1));

    assert_eq!(2, stack.depth);
    assert_eq!(4, stack.max_depth);

    // choose on true
    let mut stack = init_stack(&[2, 3, 1, 4], &[], &[], TRACE_LENGTH);
    stack.execute(OpCode::Choose, OpHint::None);
    assert_eq!(vec![2, 4, 0, 0, 0, 0, 0, 0], get_stack_state(&stack, 1));

    assert_eq!(2, stack.depth);
    assert_eq!(4, stack.max_depth);
}

#[test]
#[should_panic(expected = "CHOOSE on a non-binary condition at step 1")]
fn choose_fail() {
    let mut stack = init_stack(&[2, 3, 4], &[], &[], TRACE_LENGTH);
    stack.execute(OpCode::Choose, OpHint::None);
}

#[test]
fn choose2() {
    // choose on false
    let mut stack = init_stack(&[2, 3, 4, 5, 0, 6, 7], &[], &[], TRACE_LENGTH);
    stack.execute(OpCode::Choose2, OpHint::None);
    assert_eq!(vec![4, 5, 7, 0, 0, 0, 0, 0], get_stack_state(&stack, 1));

    assert_eq!(3, stack.depth);
    assert_eq!(7, stack.max_depth);

    // choose on true
    let mut stack = init_stack(&[2, 3, 4, 5, 1, 6, 7], &[], &[], TRACE_LENGTH);
    stack.execute(OpCode::Choose2, OpHint::None);
    assert_eq!(vec![2, 3, 7, 0, 0, 0, 0, 0], get_stack_state(&stack, 1));

    assert_eq!(3, stack.depth);
    assert_eq!(7, stack.max_depth);
}

#[test]
#[should_panic(expected = "CHOOSE2 on a non-binary condition at step 1")]
fn choose2_fail() {
    let mut stack = init_stack(&[2, 3, 4, 5, 6, 8, 8], &[], &[], TRACE_LENGTH);
    stack.execute(OpCode::Choose2, OpHint::None);
}

// OTHER CONDITIONAL OPERATIONS
// ================================================================================================

#[test]
fn cswap2() {
    // don't swap on false
    let mut stack = init_stack(&[2, 3, 4, 5, 0, 6, 7], &[], &[], TRACE_LENGTH);
    stack.execute(OpCode::CSwap2, OpHint::None);
    assert_eq!(vec![2, 3, 4, 5, 7, 0, 0, 0], get_stack_state(&stack, 1));

    assert_eq!(5, stack.depth);
    assert_eq!(7, stack.max_depth);

    // swap on true
    let mut stack = init_stack(&[2, 3, 4, 5, 1, 6, 7], &[], &[], TRACE_LENGTH);
    stack.execute(OpCode::CSwap2, OpHint::None);
    assert_eq!(vec![4, 5, 2, 3, 7, 0, 0, 0], get_stack_state(&stack, 1));

    assert_eq!(5, stack.depth);
    assert_eq!(7, stack.max_depth);
}

#[test]
#[should_panic(expected = "CSWAP2 on a non-binary condition at step 1")]
fn cswap2_fail() {
    let mut stack = init_stack(&[2, 3, 4, 5, 6, 8, 8], &[], &[], TRACE_LENGTH);
    stack.execute(OpCode::CSwap2, OpHint::None);
}
