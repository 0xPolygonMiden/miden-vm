use super::{build_inputs, compile, execute, push_to_stack, test_op_execution};
use proptest::prelude::*;
use vm_core::{Felt, StarkField};

// FIELD OPS COMPARISON - MANUAL TESTS
// ================================================================================================

#[test]
fn eq() {
    // let script = compile("begin eq end");
    let asm_op = "eq";

    // --- test when two elements are equal ------------------------------------------------------
    test_op_execution(asm_op, &[100, 100], &[1]);

    // --- test when two elements are unequal ----------------------------------------------------
    test_op_execution(asm_op, &[25, 100], &[0]);

    // --- test when two u64s are unequal but their felts are equal ------------------------------
    let a = Felt::MODULUS + 1;
    let b = 1;
    test_op_execution(asm_op, &[a, b], &[1]);
}

#[test]
fn eqw() {
    let asm_op = "eqw";

    // --- test when top two words are equal ------------------------------------------------------
    let values = vec![5, 4, 3, 2, 5, 4, 3, 2];
    let mut expected = values.clone();
    // push the result
    expected.push(1);
    // put it in stack order
    expected.reverse();
    test_op_execution(asm_op, &values, &expected);

    // --- test when top two words are not equal --------------------------------------------------
    let values = vec![8, 7, 6, 5, 4, 3, 2, 1];
    let mut expected = values.clone();
    // push the result
    expected.push(0);
    // put it in stack order
    expected.reverse();
    test_op_execution(asm_op, &values, &expected);
}

#[test]
fn lt() {
    // Results in 1 if a < b for a starting stack of [b, a, ...] and 0 otherwise
    test_felt_comparison_op("lt", 1, 0, 0);
}

#[test]
fn lte() {
    // Results in 1 if a <= b for a starting stack of [b, a, ...] and 0 otherwise
    test_felt_comparison_op("lte", 1, 1, 0);
}

#[test]
fn gt() {
    // Results in 1 if a > b for a starting stack of [b, a, ...] and 0 otherwise
    test_felt_comparison_op("gt", 0, 0, 1);
}

#[test]
fn gte() {
    // Results in 1 if a >= b for a starting stack of [b, a, ...] and 0 otherwise
    test_felt_comparison_op("gte", 0, 1, 1);
}

// HELPER FUNCTIONS FOR MANUAL TESTS
// ================================================================================================

/// This helper function runs an assembly field comparison operation (lt, lte, gt, gte) against a
/// variety of field element pairs.
//
/// The assembly ops which compare multiple field elements work by splitting both elements and
/// performing a comparison of the upper and lower 32-bit values for each element.
/// Since we're working with a 64-bit field modulus, we need to ensure that valid field elements
/// represented by > 32 bits are still compared properly, with high-bit values prioritized over low
/// when they disagree.
//
/// In order for an encoded 64-bit value to be a valid field element while having bits set in
/// both the high and low 32 bits, the upper 32 bits must not be all 1s. Therefore, for testing
/// it's sufficient to use elements with one high bit and one low bit set.
fn test_felt_comparison_op(asm_op: &str, expect_if_lt: u64, expect_if_eq: u64, expect_if_gt: u64) {
    // create vars with a variety of high and low bit relationships for testing
    let low_bit = 1;
    let high_bit = 1 << 48;

    // a smaller field element with both a high and a low bit set
    let smaller = high_bit + low_bit;
    // element with high bits equal to "smaller" and low bits bigger
    let hi_eq_lo_gt = smaller + low_bit;
    // element with high bits bigger than "smaller" and low bits smaller
    let hi_gt_lo_lt = high_bit << 1;
    // element with high bits bigger than "smaller" and low bits equal
    let hi_gt_lo_eq = hi_gt_lo_lt + low_bit;

    // unequal integers expected to be equal as field elements
    let a = Felt::MODULUS + 1;
    let a_mod = 1_u64;

    // --- a < b ----------------------------------------------------------------------------------
    // a is smaller in the low bits (equal in high bits)
    test_op_execution(asm_op, &[smaller, hi_eq_lo_gt], &[expect_if_lt]);

    // a is smaller in the high bits and equal in the low bits
    test_op_execution(asm_op, &[smaller, hi_gt_lo_eq], &[expect_if_lt]);

    // a is smaller in the high bits but bigger in the low bits
    test_op_execution(asm_op, &[smaller, hi_gt_lo_lt], &[expect_if_lt]);

    // compare values above and below the field modulus
    test_op_execution(asm_op, &[a_mod, a + 1], &[expect_if_lt]);

    // --- a = b ----------------------------------------------------------------------------------
    // high and low bits are both set
    test_op_execution(asm_op, &[hi_gt_lo_eq, hi_gt_lo_eq], &[expect_if_eq]);

    // compare values above and below the field modulus
    test_op_execution(asm_op, &[a_mod, a], &[expect_if_eq]);

    // --- a > b ----------------------------------------------------------------------------------
    // a is bigger in the low bits (equal in high bits)
    test_op_execution(asm_op, &[hi_eq_lo_gt, smaller], &[expect_if_gt]);

    // a is bigger in the high bits and equal in the low bits
    test_op_execution(asm_op, &[hi_gt_lo_eq, smaller], &[expect_if_gt]);

    // a is bigger in the high bits but smaller in the low bits
    test_op_execution(asm_op, &[hi_gt_lo_lt, smaller], &[expect_if_gt]);

    // compare values above and below the field modulus
    test_op_execution(asm_op, &[a_mod + 1, a], &[expect_if_gt]);
}

// FIELD OPS COMPARISON - RANDOMIZED TESTS
// ================================================================================================

const WORD_LEN: usize = 4;

// This is a proptest strategy for generating a random word of u64 values.
fn rand_word() -> impl Strategy<Value = Vec<u64>> {
    prop::collection::vec(any::<u64>(), WORD_LEN)
}

proptest! {
    #[test]
    fn eq_proptest(a in any::<u64>(), b in any::<u64>()) {
        // test the eq assembly operation with randomized inputs
        let script = compile("begin eq end");

        let inputs = build_inputs(&[a, b]);
        let trace = execute(&script, &inputs).unwrap();
        let last_state = trace.last_stack_state();

        // compare the random a & b values modulo the field modulus to get the expected result
        let expected_result = if a % Felt::MODULUS == b % Felt::MODULUS { 1 } else { 0 };
        let expected_state = push_to_stack(&[expected_result]);

        prop_assert_eq!(expected_state, last_state);
    }

    #[test]
    fn eqw_proptest(w1 in rand_word(), w2 in rand_word()) {
        // test the eqw assembly operation with randomized inputs
        let script = compile("begin eqw end");

        // 2 words (8 values) for comparison and 1 for the result
        let mut values = vec![0; 2 * WORD_LEN + 1];

        // check the inputs for equality in the field
        let mut inputs_equal = true;
        for (i, (a, b)) in w1.iter().zip(w2.iter()).enumerate() {
            // if any of the values are unequal in the field, then the words will be unequal
            if *a % Felt::MODULUS != *b % Felt::MODULUS {
                inputs_equal = false;
            }
            // add the values to the vector
            values[i] = *a;
            values[i + WORD_LEN] = *b;
        }

        let inputs = build_inputs(&values);
        let trace = execute(&script, &inputs).unwrap();
        let last_state = trace.last_stack_state();

        // add the expected result to get the expected state
        let expected_result = if inputs_equal { 1 } else { 0 };
        values.push(expected_result);
        let expected_state = push_to_stack(&values);

        prop_assert_eq!(expected_state, last_state);
    }

    #[test]
    fn lt_proptest(a in any::<u64>(), b in any::<u64>()) {
        // test the less-than assembly operation with randomized inputs
        let script = compile("begin lt end");

        let inputs = build_inputs(&[a, b]);
        let trace = execute(&script, &inputs).unwrap();
        let last_state = trace.last_stack_state();

        // compare the random a & b values modulo the field modulus to get the expected result
        let expected_result = if a % Felt::MODULUS < b % Felt::MODULUS { 1 } else { 0 };
        let expected_state = push_to_stack(&[expected_result]);

        prop_assert_eq!(expected_state, last_state);
    }

    #[test]
    fn lte_proptest(a in any::<u64>(), b in any::<u64>()) {
        // test the less-than-or-equal assembly operation with randomized inputs
        let script = compile("begin lte end");

        let inputs = build_inputs(&[a, b]);
        let trace = execute(&script, &inputs).unwrap();
        let last_state = trace.last_stack_state();

        // compare the random a & b values modulo the field modulus to get the expected result
        let expected_result = if a % Felt::MODULUS <= b % Felt::MODULUS { 1 } else { 0 };
        let expected_state = push_to_stack(&[expected_result]);

        prop_assert_eq!(expected_state, last_state);
    }

    #[test]
    fn gt_proptest(a in any::<u64>(), b in any::<u64>()) {
        // test the greater-than assembly operation with randomized inputs
        let script = compile("begin gt end");

        let inputs = build_inputs(&[a, b]);
        let trace = execute(&script, &inputs).unwrap();
        let last_state = trace.last_stack_state();

        // compare the random a & b values modulo the field modulus to get the expected result
        let expected_result = if a % Felt::MODULUS > b % Felt::MODULUS { 1 } else { 0 };
        let expected_state = push_to_stack(&[expected_result]);

        prop_assert_eq!(expected_state, last_state);
    }

    #[test]
    fn gte_proptest(a in any::<u64>(), b in any::<u64>()) {
        // test the greater-than-or-equal assembly operation with randomized inputs
        let script = compile("begin gte end");

        let inputs = build_inputs(&[a, b]);
        let trace = execute(&script, &inputs).unwrap();
        let last_state = trace.last_stack_state();

        // compare the random a & b values modulo the field modulus to get the expected result
        let expected_result = if a % Felt::MODULUS >= b % Felt::MODULUS { 1 } else { 0 };
        let expected_state = push_to_stack(&[expected_result]);

        prop_assert_eq!(expected_state, last_state);
    }
}
