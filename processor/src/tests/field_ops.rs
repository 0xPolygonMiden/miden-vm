use super::{super::execute, build_inputs, build_stack_state, compile};
use proptest::prelude::*;
use vm_core::{Felt, StarkField};

// FIELD OPS COMPARISON - MANUAL TESTS
// ================================================================================================

#[test]
fn eq() {
    let script = compile("begin eq end");

    // --- test when two elements are equal ------------------------------------------------------
    let inputs = build_inputs(&[100, 100]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    let expected_state = build_stack_state(&[1]);
    assert_eq!(expected_state, last_state);

    // --- test when two elements are unequal ----------------------------------------------------
    let inputs = build_inputs(&[100, 25]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    let expected_state = build_stack_state(&[0]);
    assert_eq!(expected_state, last_state);

    // --- test when two u64s are unequal but their felts are equal ------------------------------
    let a = Felt::MODULUS + 1;
    let b = 1;

    let inputs = build_inputs(&[b, a]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    let expected_state = build_stack_state(&[1]);
    assert_eq!(expected_state, last_state);
}

#[test]
fn eqw() {
    let script = compile("begin eqw end");

    // --- test when top two words are equal ------------------------------------------------------
    let mut values = vec![2, 3, 4, 5, 2, 3, 4, 5];
    let inputs = build_inputs(&values);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    values.insert(0, 1);
    let expected_state = build_stack_state(&values);
    assert_eq!(expected_state, last_state);

    // --- test when top two words are not equal --------------------------------------------------
    let mut values = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let inputs = build_inputs(&values);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    values.insert(0, 0);
    let expected_state = build_stack_state(&values);
    assert_eq!(expected_state, last_state);
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
    let script = compile(format!("begin {} end", asm_op).as_str());

    // prepare the expected states with the provided values
    let expected_state_lt = build_stack_state(&[expect_if_lt]);
    let expected_state_eq = build_stack_state(&[expect_if_eq]);
    let expected_state_gt = build_stack_state(&[expect_if_gt]);

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
    let inputs = build_inputs(&[hi_eq_lo_gt, smaller]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    assert_eq!(expected_state_lt, last_state);

    // a is smaller in the high bits and equal in the low bits
    let inputs = build_inputs(&[hi_gt_lo_eq, smaller]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    assert_eq!(expected_state_lt, last_state);

    // a is smaller in the high bits but bigger in the low bits
    let inputs = build_inputs(&[hi_gt_lo_lt, smaller]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    assert_eq!(expected_state_lt, last_state);

    // compare values above and below the field modulus
    let inputs = build_inputs(&[a + 1, a_mod]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    assert_eq!(expected_state_lt, last_state);

    // --- a = b ----------------------------------------------------------------------------------
    // high and low bits are both set
    let inputs = build_inputs(&[hi_gt_lo_eq, hi_gt_lo_eq]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    assert_eq!(expected_state_eq, last_state);

    // compare values above and below the field modulus
    let inputs = build_inputs(&[a, a_mod]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    assert_eq!(expected_state_eq, last_state);

    // --- a > b ----------------------------------------------------------------------------------
    // a is bigger in the low bits (equal in high bits)
    let inputs = build_inputs(&[smaller, hi_eq_lo_gt]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    assert_eq!(expected_state_gt, last_state);

    // a is bigger in the high bits and equal in the low bits
    let inputs = build_inputs(&[smaller, hi_gt_lo_eq]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    assert_eq!(expected_state_gt, last_state);

    // a is bigger in the high bits but smaller in the low bits
    let inputs = build_inputs(&[smaller, hi_gt_lo_lt]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    assert_eq!(expected_state_gt, last_state);

    // compare values above and below the field modulus
    let inputs = build_inputs(&[a, a_mod + 1]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    assert_eq!(expected_state_gt, last_state);
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

        let inputs = build_inputs(&[b, a]);
        let trace = execute(&script, &inputs).unwrap();
        let last_state = trace.last_stack_state();

        // compare the random a & b values modulo the field modulus to get the expected result
        let expected_result = if a % Felt::MODULUS == b % Felt::MODULUS { 1 } else { 0 };
        let expected_state = build_stack_state(&[expected_result]);

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
        values.insert(0, expected_result);
        let expected_state = build_stack_state(&values);

        prop_assert_eq!(expected_state, last_state);
    }

    #[test]
    fn lt_proptest(a in any::<u64>(), b in any::<u64>()) {
        // test the less-than assembly operation with randomized inputs
        let script = compile("begin lt end");

        let inputs = build_inputs(&[b, a]);
        let trace = execute(&script, &inputs).unwrap();
        let last_state = trace.last_stack_state();

        // compare the random a & b values modulo the field modulus to get the expected result
        let expected_result = if a % Felt::MODULUS < b % Felt::MODULUS { 1 } else { 0 };
        let expected_state = build_stack_state(&[expected_result]);

        prop_assert_eq!(expected_state, last_state);
    }

    #[test]
    fn lte_proptest(a in any::<u64>(), b in any::<u64>()) {
        // test the less-than-or-equal assembly operation with randomized inputs
        let script = compile("begin lte end");

        let inputs = build_inputs(&[b, a]);
        let trace = execute(&script, &inputs).unwrap();
        let last_state = trace.last_stack_state();

        // compare the random a & b values modulo the field modulus to get the expected result
        let expected_result = if a % Felt::MODULUS <= b % Felt::MODULUS { 1 } else { 0 };
        let expected_state = build_stack_state(&[expected_result]);

        prop_assert_eq!(expected_state, last_state);
    }

    #[test]
    fn gt_proptest(a in any::<u64>(), b in any::<u64>()) {
        // test the greater-than assembly operation with randomized inputs
        let script = compile("begin gt end");

        let inputs = build_inputs(&[b, a]);
        let trace = execute(&script, &inputs).unwrap();
        let last_state = trace.last_stack_state();

        // compare the random a & b values modulo the field modulus to get the expected result
        let expected_result = if a % Felt::MODULUS > b % Felt::MODULUS { 1 } else { 0 };
        let expected_state = build_stack_state(&[expected_result]);

        prop_assert_eq!(expected_state, last_state);
    }

    #[test]
    fn gte_proptest(a in any::<u64>(), b in any::<u64>()) {
        // test the greater-than-or-equal assembly operation with randomized inputs
        let script = compile("begin gte end");

        let inputs = build_inputs(&[b, a]);
        let trace = execute(&script, &inputs).unwrap();
        let last_state = trace.last_stack_state();

        // compare the random a & b values modulo the field modulus to get the expected result
        let expected_result = if a % Felt::MODULUS >= b % Felt::MODULUS { 1 } else { 0 };
        let expected_state = build_stack_state(&[expected_result]);

        prop_assert_eq!(expected_state, last_state);
    }
}
