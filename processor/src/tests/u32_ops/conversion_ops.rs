use super::{
    rand_word, test_execution_failure, test_inputs_out_of_bounds, test_op_execution,
    test_op_execution_proptest, Felt, StarkField, U32_BOUND, WORD_LEN,
};
use proptest::prelude::*;
use rand_utils::rand_value;

// U32 OPERATIONS TESTS - MANUAL - CONVERSIONS AND TESTS
// ================================================================================================

#[test]
fn u32test() {
    // pushes 1 onto the stack if a < 2^32 and 0 otherwise
    let asm_op = "u32test";

    // vars to test
    let smaller = 1_u64;
    let equal = 1_u64 << 32;
    let larger = equal + 1;

    // --- a < 2^32 -------------------------------------------------------------------------------
    test_op_execution(asm_op, &[smaller], &[1, smaller]);

    // --- a = 2^32 -------------------------------------------------------------------------------
    test_op_execution(asm_op, &[equal], &[0, equal]);

    // --- a > 2^32 -------------------------------------------------------------------------------
    test_op_execution(asm_op, &[larger], &[0, larger]);
}

#[test]
fn u32testw() {
    let asm_op = "u32testw";

    // --- all elements in range ------------------------------------------------------------------
    let values = [1, 1, 1, 1];
    let expected = [1, 1, 1, 1, 1];
    test_op_execution(asm_op, &values, &expected);

    // --- 1st element >= 2^32 --------------------------------------------------------------------
    let values = [U32_BOUND, 0, 0, 0];
    let expected = [0, 0, 0, 0, U32_BOUND];
    test_op_execution(asm_op, &values, &expected);

    // --- 2nd element >= 2^32 --------------------------------------------------------------------
    let values = [0, U32_BOUND, 0, 0];
    let expected = [0, 0, 0, U32_BOUND, 0];
    test_op_execution(asm_op, &values, &expected);

    // --- 3rd element >= 2^32 --------------------------------------------------------------------
    let values = [0, 0, U32_BOUND, 0];
    let expected = [0, 0, U32_BOUND, 0, 0];
    test_op_execution(asm_op, &values, &expected);

    // --- 4th element >= 2^32 --------------------------------------------------------------------
    let values = [0, 0, 0, U32_BOUND];
    let expected = [0, U32_BOUND, 0, 0, 0];
    test_op_execution(asm_op, &values, &expected);

    // --- all elements out of range --------------------------------------------------------------
    let values = [U32_BOUND, U32_BOUND, U32_BOUND, U32_BOUND];
    let expected = [0, U32_BOUND, U32_BOUND, U32_BOUND, U32_BOUND];
    test_op_execution(asm_op, &values, &expected);
}

#[test]
fn u32assert() {
    // assertion passes and leaves the stack unchanged if a < 2^32
    let asm_op = "u32assert";
    let value = 1_u64;
    test_op_execution(asm_op, &[value], &[value]);
}

#[test]
fn u32assert_fail() {
    // assertion fails if a >= 2^32
    let asm_op = "u32assert";
    let err = "FailedAssertion";

    // vars to test
    let equal = 1_u64 << 32;
    let larger = equal + 1;

    // --- test when a = 2^32 ---------------------------------------------------------------------
    test_execution_failure(asm_op, &[equal], err);

    // --- test when a > 2^32 ---------------------------------------------------------------------
    test_execution_failure(asm_op, &[larger], err);
}

#[test]
fn u32assertw() {
    // assertion passes and leaves the stack unchanged if each element of the word < 2^32
    let asm_op = "u32assertw";
    test_op_execution(asm_op, &[2, 3, 4, 5], &[5, 4, 3, 2]);
}

#[test]
fn u32assertw_fail() {
    // fails if any element in the word >= 2^32
    let asm_op = "u32assertw";
    let err = "FailedAssertion";

    // --- any one of the inputs inputs >= 2^32 (out of bounds) -----------------------------------
    test_inputs_out_of_bounds(asm_op, WORD_LEN);

    // --- all elements out of range --------------------------------------------------------------
    test_execution_failure(asm_op, &[U32_BOUND; WORD_LEN], err);
}

#[test]
fn u32cast() {
    let asm_op = "u32cast";

    // --- a < 2^32 -------------------------------------------------------------------------------
    test_op_execution(asm_op, &[1], &[1]);

    // --- a > 2^32 -------------------------------------------------------------------------------
    test_op_execution(asm_op, &[U32_BOUND], &[0]);

    // --- rest of stack isn't affected -----------------------------------------------------------
    let a = rand_value();
    let b = rand_value();
    test_op_execution(asm_op, &[a, b], &[b % U32_BOUND, a]);
}

#[test]
fn u32split() {
    let asm_op = "u32split";

    // --- low bits set, no high bits set ---------------------------------------------------------
    test_op_execution(asm_op, &[1], &[0, 1]);

    // --- high bits set, no low bits set ---------------------------------------------------------
    test_op_execution(asm_op, &[U32_BOUND], &[1, 0]);

    // --- high bits and low bits set -------------------------------------------------------------
    test_op_execution(asm_op, &[U32_BOUND + 1], &[1, 1]);

    // --- rest of stack isn't affected -----------------------------------------------------------
    let a = rand_value();
    let b = rand_value();
    let expected_hi = b >> 32;
    let expected_lo = b % U32_BOUND;
    test_op_execution(asm_op, &[a, b], &[expected_hi, expected_lo, a]);
}

// U32 OPERATIONS TESTS - RANDOMIZED - CONVERSIONS AND TESTS
// ================================================================================================
proptest! {
    #[test]
    fn u32test_proptest(value in any::<u64>()) {
        // check to see if the value of the element will be a valid u32
        let expected_result = if value % Felt::MODULUS < U32_BOUND { 1 } else { 0 };

        test_op_execution_proptest("u32test", &[value], &[expected_result, value])?;
    }

    #[test]
    fn u32testw_proptest(word in rand_word::<u32>()) {
        // should leave a 1 on the stack since all values in the word are valid u32 values
        let values: Vec<u64> = word.iter().map(|a| *a as u64).collect();
        let mut expected = values.clone();
        // push the expected result
        expected.push(1);
        // reverse the values to put the expected array in stack order
        expected.reverse();

        test_op_execution_proptest("u32testw", &values, &expected)?;
    }

    #[test]
    fn u32assert_proptest(value in any::<u32>()) {
        // assertion passes and leaves the stack unchanged if a < 2^32
        let asm_op = "u32assert";
        test_op_execution_proptest(asm_op, &[value as u64], &[value as u64])?;
    }

    #[test]
    fn u32assertw_proptest(word in rand_word::<u32>()) {
        // should pass and leave the stack unchanged if a < 2^32 for all values in the word
        let asm_op = "u32assertw";
        let values: Vec<u64> = word.iter().map(|a| *a as u64).collect();
        let mut expected = values.clone();
        // reverse the values to put the expected array in stack order
        expected.reverse();

        test_op_execution_proptest(asm_op, &values, &expected)?;
    }

    #[test]
    fn u32cast_proptest(value in any::<u64>()) {
        // expected result will be mod 2^32 applied to a field element
        // so the field modulus should be applied first
        let expected_result = value % Felt::MODULUS % U32_BOUND;

        test_op_execution_proptest("u32cast", &[value], &[expected_result])?;
    }

    #[test]
    fn u32split_proptest(value in any::<u64>()) {
        // expected result will be mod 2^32 applied to a field element
        // so the field modulus must be applied first
        let felt_value = value % Felt::MODULUS;
        let expected_b = felt_value >> 32;
        let expected_c = felt_value as u32 as u64;

        test_op_execution_proptest("u32split", &[value, value], &[expected_b, expected_c, value])?;
    }
}
