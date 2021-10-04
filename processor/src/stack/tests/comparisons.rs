use super::{get_stack_state, init_stack, OpCode, OpHint, Stack, TRACE_LENGTH};
use rand_utils::rand_value;
use winterfell::math::{fields::f128::BaseElement, FieldElement, StarkField};

// EQUALITY OPERATION
// ================================================================================================

#[test]
fn eq() {
    let inv_diff = (BaseElement::ONE - BaseElement::new(4)).inv().as_int();
    let mut stack = init_stack(&[3, 3, 4, 5], &[0, inv_diff], &[], TRACE_LENGTH);

    stack.execute(OpCode::Read, OpHint::None);
    stack.execute(OpCode::Eq, OpHint::None);
    assert_eq!(vec![1, 4, 5, 0, 0, 0, 0, 0], get_stack_state(&stack, 2));

    assert_eq!(3, stack.depth);
    assert_eq!(5, stack.max_depth);

    stack.execute(OpCode::Read, OpHint::None);
    stack.execute(OpCode::Eq, OpHint::None);
    assert_eq!(vec![0, 5, 0, 0, 0, 0, 0, 0], get_stack_state(&stack, 4));

    assert_eq!(2, stack.depth);
    assert_eq!(5, stack.max_depth);
}

#[test]
fn eq_with_hint() {
    let mut stack = init_stack(&[3, 3, 4, 5], &[], &[], TRACE_LENGTH);

    stack.execute(OpCode::Read, OpHint::EqStart);
    stack.execute(OpCode::Eq, OpHint::None);
    assert_eq!(vec![1, 4, 5, 0, 0, 0, 0, 0], get_stack_state(&stack, 2));

    assert_eq!(3, stack.depth);
    assert_eq!(5, stack.max_depth);

    stack.execute(OpCode::Read, OpHint::EqStart);
    stack.execute(OpCode::Eq, OpHint::None);
    assert_eq!(vec![0, 5, 0, 0, 0, 0, 0, 0], get_stack_state(&stack, 4));

    assert_eq!(2, stack.depth);
    assert_eq!(5, stack.max_depth);
}

// COMPARISON OPERATION
// ================================================================================================

#[test]
fn cmp_128() {
    let a = rand_value();
    let b = rand_value();
    let p127 = BaseElement::new(2).exp(127);

    // initialize the stack
    let (inputs_a, inputs_b) = build_inputs_for_cmp(a, b, 128);
    let mut stack = init_stack(&[0, 0, 0, 0, 0, a, b], &inputs_a, &inputs_b, 256);
    stack.execute(OpCode::Pad2, OpHint::None);
    stack.execute(OpCode::Push, OpHint::PushValue(p127));

    // execute CMP operations
    for i in 2..130 {
        stack.execute(OpCode::Cmp, OpHint::None);

        let state = get_stack_state(&stack, i);
        let next = get_stack_state(&stack, i + 1);

        let gt = BaseElement::new(state[4]);
        let lt = BaseElement::new(state[5]);
        let not_set = (BaseElement::ONE - gt) * (BaseElement::ONE - lt);
        assert_eq!(not_set, BaseElement::new(next[3]));
    }

    // check the result
    let lt = if a < b {
        BaseElement::ONE
    } else {
        BaseElement::ZERO
    };
    let gt = if a < b {
        BaseElement::ZERO
    } else {
        BaseElement::ONE
    };

    let state = get_stack_state(&stack, 130);
    assert_eq!([gt.as_int(), lt.as_int(), b, a], state[4..8]);
}

#[test]
fn cmp_64() {
    let a: u128 = (rand_value::<u64>()) as u128;
    let b: u128 = (rand_value::<u64>()) as u128;
    let p63 = BaseElement::new(2).exp(63);

    // initialize the stack
    let (inputs_a, inputs_b) = build_inputs_for_cmp(a, b, 64);
    let mut stack = init_stack(&[0, 0, 0, 0, 0, a, b], &inputs_a, &inputs_b, 256);
    stack.execute(OpCode::Pad2, OpHint::None);
    stack.execute(OpCode::Push, OpHint::PushValue(p63));

    // execute CMP operations
    for i in 2..66 {
        stack.execute(OpCode::Cmp, OpHint::None);

        let state = get_stack_state(&stack, i);
        let next = get_stack_state(&stack, i + 1);

        let gt = BaseElement::new(state[4]);
        let lt = BaseElement::new(state[5]);
        let not_set = (BaseElement::ONE - gt) * (BaseElement::ONE - lt);
        assert_eq!(not_set, BaseElement::new(next[3]));
    }

    // check the result
    let lt = if a < b {
        BaseElement::ONE
    } else {
        BaseElement::ZERO
    };
    let gt = if a < b {
        BaseElement::ZERO
    } else {
        BaseElement::ONE
    };

    let state = get_stack_state(&stack, 66);
    assert_eq!([gt.as_int(), lt.as_int(), b, a], state[4..8]);
}

// COMPARISON PROGRAMS
// ================================================================================================

#[test]
fn lt() {
    let a: u128 = rand_value();
    let b: u128 = rand_value();
    let p127 = BaseElement::new(2).exp(127);

    // initialize the stack
    let (inputs_a, inputs_b) = build_inputs_for_cmp(a, b, 128);
    let mut stack = init_stack(&[0, 0, 0, a, b, 7, 11], &inputs_a, &inputs_b, 256);
    stack.execute(OpCode::Pad2, OpHint::None);
    stack.execute(OpCode::Pad2, OpHint::None);
    stack.execute(OpCode::Push, OpHint::PushValue(p127));

    // execute CMP operations
    for _ in 3..131 {
        stack.execute(OpCode::Cmp, OpHint::None);
    }

    // execute program finale
    lt_finale(&mut stack);

    // check the result
    let state = get_stack_state(&stack, stack.current_step());
    let expected = if a < b {
        BaseElement::ONE
    } else {
        BaseElement::ZERO
    };
    assert_eq!(
        vec![expected.as_int(), 7, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        state
    );
}

#[test]
fn gt() {
    let a: u128 = rand_value();
    let b: u128 = rand_value();
    let p127 = BaseElement::new(2).exp(127);

    // initialize the stack
    let (inputs_a, inputs_b) = build_inputs_for_cmp(a, b, 128);
    let mut stack = init_stack(&[0, 0, 0, a, b, 7, 11], &inputs_a, &inputs_b, 256);
    stack.execute(OpCode::Pad2, OpHint::None);
    stack.execute(OpCode::Pad2, OpHint::None);
    stack.execute(OpCode::Push, OpHint::PushValue(p127));

    // execute CMP operations
    for _ in 3..131 {
        stack.execute(OpCode::Cmp, OpHint::None);
    }

    // execute program finale
    gt_finale(&mut stack);

    // check the result
    let state = get_stack_state(&stack, stack.current_step());
    let expected = if a > b {
        BaseElement::ONE
    } else {
        BaseElement::ZERO
    };
    assert_eq!(
        vec![expected.as_int(), 7, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        state
    );
}

// BINARY DECOMPOSITION
// ================================================================================================

#[test]
fn binacc_128() {
    let x: u128 = rand_value();

    // initialize the stack
    let mut inputs_a = Vec::new();
    for i in 0..128 {
        inputs_a.push((x >> (127 - i)) & 1);
    }
    inputs_a.reverse();

    let mut stack = init_stack(&[0, 0, 1, 0, x, 7, 11], &inputs_a, &[], 256);

    // execute binary aggregation operations
    for _ in 0..128 {
        stack.execute(OpCode::BinAcc, OpHint::None);
    }

    // check the result
    stack.execute(OpCode::Drop, OpHint::None);
    stack.execute(OpCode::Drop, OpHint::None);
    stack.execute(OpCode::Drop, OpHint::None);
    let state = get_stack_state(&stack, 131);
    assert_eq!(vec![x, x, 7, 11, 0, 0, 0, 0], state);
}

#[test]
fn binacc_64() {
    let x: u128 = (rand_value::<u64>()) as u128;

    // initialize the stack
    let mut inputs_a = Vec::new();
    for i in 0..64 {
        inputs_a.push((x >> (63 - i)) & 1);
    }
    inputs_a.reverse();

    let mut stack = init_stack(&[0, 0, 1, 0, x, 7, 11], &inputs_a, &[], 256);

    // execute binary aggregation operations
    for _ in 0..64 {
        stack.execute(OpCode::BinAcc, OpHint::None);
    }

    // check the result
    stack.execute(OpCode::Drop, OpHint::None);
    stack.execute(OpCode::Drop, OpHint::None);
    stack.execute(OpCode::Drop, OpHint::None);
    let state = get_stack_state(&stack, 67);
    assert_eq!(vec![x, x, 7, 11, 0, 0, 0, 0], state);
}

#[test]
fn isodd_128() {
    let x: u128 = rand_value();
    let is_odd = x & 1;

    // initialize the stack
    let mut inputs_a = Vec::new();
    for i in 0..128 {
        inputs_a.push((x >> (127 - i)) & 1);
    }
    inputs_a.reverse();

    let mut stack = init_stack(&[0, 0, 1, 0, x, 7, 11], &inputs_a, &[], 256);

    // read the first bit and make sure it is saved at the end of the stack
    stack.execute(OpCode::BinAcc, OpHint::None);
    stack.execute(OpCode::Swap2, OpHint::None);
    stack.execute(OpCode::Roll4, OpHint::None);
    stack.execute(OpCode::Dup, OpHint::None);

    // execute remaining binary aggregation operations
    for _ in 0..127 {
        stack.execute(OpCode::BinAcc, OpHint::None);
    }

    // check the result
    stack.execute(OpCode::Drop, OpHint::None);
    stack.execute(OpCode::Drop, OpHint::None);
    stack.execute(OpCode::Swap, OpHint::None);
    stack.execute(OpCode::Roll4, OpHint::None);
    stack.execute(OpCode::AssertEq, OpHint::None);
    stack.execute(OpCode::Drop, OpHint::None);
    let state = get_stack_state(&stack, 137);
    assert_eq!(vec![is_odd, 7, 11, 0, 0, 0, 0, 0], state);
}

// HELPER FUNCTIONS
// ================================================================================================
fn build_inputs_for_cmp(a: u128, b: u128, size: usize) -> (Vec<u128>, Vec<u128>) {
    let mut inputs_a = Vec::new();
    let mut inputs_b = Vec::new();
    for i in 0..size {
        inputs_a.push((a >> i) & 1);
        inputs_b.push((b >> i) & 1);
    }
    inputs_a.reverse();
    inputs_b.reverse();

    return (inputs_a, inputs_b);
}

fn lt_finale(stack: &mut Stack) {
    stack.execute(OpCode::Drop4, OpHint::None);
    stack.execute(OpCode::Pad2, OpHint::None);
    stack.execute(OpCode::Swap4, OpHint::None);
    stack.execute(OpCode::Roll4, OpHint::None);
    stack.execute(OpCode::AssertEq, OpHint::None);
    stack.execute(OpCode::AssertEq, OpHint::None);
    stack.execute(OpCode::Dup, OpHint::None);
    stack.execute(OpCode::Drop4, OpHint::None);
}

fn gt_finale(stack: &mut Stack) {
    stack.execute(OpCode::Drop4, OpHint::None);
    stack.execute(OpCode::Pad2, OpHint::None);
    stack.execute(OpCode::Swap4, OpHint::None);
    stack.execute(OpCode::Roll4, OpHint::None);
    stack.execute(OpCode::AssertEq, OpHint::None);
    stack.execute(OpCode::AssertEq, OpHint::None);
    stack.execute(OpCode::Roll4, OpHint::None);
    stack.execute(OpCode::Dup, OpHint::None);
    stack.execute(OpCode::Drop4, OpHint::None);
}
