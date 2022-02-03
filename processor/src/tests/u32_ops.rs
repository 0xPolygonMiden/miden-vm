use std::cmp::Ordering;

use super::{
    super::StarkField, build_inputs, compile, execute, rand_word, test_compilation_failure,
    test_execution, test_execution_failure, test_param_out_of_bounds, Felt,
};
use proptest::prelude::*;
use rand_utils::rand_value;

// CONSTANTS
// ================================================================================================
const U32_BOUND: u64 = u32::MAX as u64 + 1;
const WORD_LEN: usize = 4;

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
    test_execution(asm_op, &[smaller], &[1, smaller]);

    // --- a = 2^32 -------------------------------------------------------------------------------
    test_execution(asm_op, &[equal], &[0, equal]);

    // --- a > 2^32 -------------------------------------------------------------------------------
    test_execution(asm_op, &[larger], &[0, larger]);
}

#[test]
// issue: https://github.com/maticnetwork/miden/issues/71
fn u32testw() {
    let asm_op = "u32testw";

    // --- all elements in range ------------------------------------------------------------------
    let values = vec![1; WORD_LEN];
    let mut expected = values.clone();
    expected.insert(0, 1);
    test_execution(asm_op, &values, &expected);

    // --- 1st element >= 2^32 --------------------------------------------------------------------
    let values = vec![U32_BOUND, 0, 0, 0];
    let mut expected = values.clone();
    expected.insert(0, 0);
    test_execution(asm_op, &values, &expected);

    // --- 2nd element >= 2^32 --------------------------------------------------------------------
    let values = vec![0, U32_BOUND, 0, 0];
    let mut expected = values.clone();
    expected.insert(0, 0);
    test_execution(asm_op, &values, &expected);

    // --- 3rd element >= 2^32 --------------------------------------------------------------------
    let values = vec![0, 0, U32_BOUND, 0];
    let mut expected = values.clone();
    expected.insert(0, 0);
    test_execution(asm_op, &values, &expected);

    // --- 4th element >= 2^32 --------------------------------------------------------------------
    let values = vec![0, 0, 0, U32_BOUND];
    let mut expected = values.clone();
    expected.insert(0, 0);
    test_execution(asm_op, &values, &expected);

    // --- all elements out of range --------------------------------------------------------------
    let values = vec![U32_BOUND; WORD_LEN];
    let mut expected = values.clone();
    expected.insert(0, 0);
    test_execution(asm_op, &values, &expected);
}

#[test]
fn u32assert() {
    // assertion passes and leaves the stack unchanged if a < 2^32
    let asm_op = "u32assert";
    let value = 1_u64;
    test_execution(asm_op, &[value], &[value]);
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
    test_execution(asm_op, &[2, 3, 4, 5], &[2, 3, 4, 5]);
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
    test_execution(asm_op, &[1], &[1]);

    // --- a > 2^32 -------------------------------------------------------------------------------
    test_execution(asm_op, &[U32_BOUND], &[0]);

    // --- rest of stack isn't affected -----------------------------------------------------------
    let a = rand_value();
    let b = rand_value();
    test_execution(asm_op, &[a, b], &[a % U32_BOUND, b]);
}

#[test]
fn u32split() {
    let asm_op = "u32split";

    // --- low bits set, no high bits set ---------------------------------------------------------
    test_execution(asm_op, &[1], &[0, 1]);

    // --- high bits set, no low bits set ---------------------------------------------------------
    test_execution(asm_op, &[U32_BOUND], &[1, 0]);

    // --- high bits and low bits set -------------------------------------------------------------
    test_execution(asm_op, &[U32_BOUND + 1], &[1, 1]);

    // --- rest of stack isn't affected -----------------------------------------------------------
    let a = rand_value();
    let b = rand_value();
    let expected_hi = a >> 32;
    let expected_lo = a % U32_BOUND;
    test_execution(asm_op, &[a, b], &[expected_hi, expected_lo, b]);
}

// U32 OPERATIONS TESTS - MANUAL - ARITHMETIC OPERATIONS
// ================================================================================================

#[test]
fn u32add() {
    let asm_op = "u32add";

    // --- simple case ----------------------------------------------------------------------------
    test_execution(asm_op, &[2, 1], &[3]);

    // --- random values --------------------------------------------------------------------------
    // test using u16 values to ensure there's no overflow so the result is valid
    let a = rand_value::<u64>() as u16;
    let b = rand_value::<u64>() as u16;
    let expected = a as u64 + b as u64;

    test_execution(asm_op, &[b as u64, a as u64], &[expected]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_execution(asm_op, &[b as u64, a as u64, c], &[expected, c]);
}

#[test]
fn u32add_fail() {
    let asm_op = "u32add";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[0, U32_BOUND], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 0], "FailedAssertion");

    // should fail if a + b >= 2^32
    let a = u32::MAX;
    let b = 1_u64;
    test_execution_failure(asm_op, &[b, a as u64], "FailedAssertion");
}

#[test]
fn u32add_b() {
    let build_asm_op = |param: u16| format!("u32add.{}", param);

    // --- simple case ----------------------------------------------------------------------------
    test_execution(build_asm_op(2).as_str(), &[1], &[3]);

    // --- random values --------------------------------------------------------------------------
    // test using u16 values to ensure there's no overflow so the result is valid
    let a = rand_value::<u64>() as u16;
    let b = rand_value::<u64>() as u16;
    let expected = a as u64 + b as u64;
    test_execution(build_asm_op(b).as_str(), &[a as u64], &[expected]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_execution(build_asm_op(b).as_str(), &[a as u64, c], &[expected, c]);
}

#[test]
fn u32add_b_fail() {
    let build_asm_op = |param: u64| format!("u32add.{}", param);

    // should fail during execution if a >= 2^32
    test_execution_failure(build_asm_op(0).as_str(), &[U32_BOUND], "FailedAssertion");

    // should fail during compilation if b >= 2^32
    test_param_out_of_bounds(build_asm_op(U32_BOUND).as_str(), U32_BOUND);

    // should fail if a + b >= 2^32
    let a = u32::MAX;
    let b = 1_u64;
    test_execution_failure(build_asm_op(b).as_str(), &[a as u64], "FailedAssertion");
}

#[test]
fn u32add_full() {
    let asm_op = "u32add.full";

    // should push c = (a + b) % 2^32 onto the stack
    // should push overflow flag d, where d = 1 if (a + b) >= 2^32 and d = 0 otherwise
    test_add_full(asm_op);
}

#[test]
fn u32add_full_fail() {
    let asm_op = "u32add.full";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[0, U32_BOUND], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 0], "FailedAssertion");
}

#[test]
fn u32add_unsafe() {
    let asm_op = "u32add.unsafe";

    // should push c = (a + b) % 2^32 onto the stack
    // should push overflow flag d, where d = 1 if (a + b) >= 2^32 and d = 0 otherwise
    test_add_full(asm_op);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
fn u32addc() {
    let asm_op = "u32addc";

    // should push d = (a + b + c) % 2^32 onto the stack, where c is 1 or 0
    // should push overflow flag e, where e = 1 if (a + b + c) >= 2^32 and e = 0 otherwise
    test_addc(asm_op);
}

#[test]
fn u32addc_fail() {
    let asm_op = "u32addc";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[0, U32_BOUND, 0], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 0, 0], "FailedAssertion");

    // should fail if c > 1
    test_execution_failure(asm_op, &[0, 0, 2], "NotBinaryValue");
}

#[test]
fn u32addc_unsafe() {
    let asm_op = "u32addc.unsafe";

    // --- test correct execution -----------------------------------------------------------------
    // should push d = (a + b + c) % 2^32 onto the stack, where c is 1 or 0
    // should push overflow flag e, where e = 1 if (a + b + c) >= 2^32 and e = 0 otherwise
    test_addc(asm_op);

    // --- test that out of bounds inputs do not cause a failure ----------------------------------
    let script = compile(format!("begin {} end", asm_op).as_str());

    // should not fail if a >= 2^32
    let inputs = build_inputs(&[0, U32_BOUND, 0]);
    assert!(execute(&script, &inputs).is_ok());

    // should not fail if b >= 2^32
    let inputs = build_inputs(&[U32_BOUND, 0, 0]);
    assert!(execute(&script, &inputs).is_ok());
}

#[test]
fn u32addc_unsafe_fail() {
    let asm_op = "u32addc.unsafe";

    // should fail if c > 1
    test_execution_failure(asm_op, &[U32_BOUND, 0, 2], "NotBinaryValue");
}

#[test]
fn u32sub() {
    let asm_op = "u32sub";

    // --- simple cases ---------------------------------------------------------------------------
    test_execution(asm_op, &[1, 1], &[0]);
    test_execution(asm_op, &[1, 2], &[1]);

    // --- random u32 values ----------------------------------------------------------------------
    let val1 = rand_value::<u64>() as u32;
    let val2 = rand_value::<u64>() as u32;
    // assign the larger value to a and the smaller value to b
    let (a, b) = if val1 >= val2 {
        (val1, val2)
    } else {
        (val2, val1)
    };
    let expected = a - b;

    test_execution(asm_op, &[b as u64, a as u64], &[expected as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_execution(asm_op, &[b as u64, a as u64, c], &[expected as u64, c]);
}

#[test]
fn u32sub_fail() {
    let asm_op = "u32sub";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[0, U32_BOUND], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 0], "FailedAssertion");

    // should fail if a < b
    let a = 1_u64;
    let b = 2_u64;
    test_execution_failure(asm_op, &[b, a], "FailedAssertion");
}

#[test]
fn u32sub_b() {
    let build_asm_op = |param: u32| format!("u32sub.{}", param);

    // --- simple cases ---------------------------------------------------------------------------
    test_execution(build_asm_op(1).as_str(), &[2], &[1]);
    test_execution(build_asm_op(1).as_str(), &[1], &[0]);

    // --- random u32 values ----------------------------------------------------------------------
    let val1 = rand_value::<u64>() as u32;
    let val2 = rand_value::<u64>() as u32;
    // assign the larger value to a and the smaller value to b
    let (a, b) = if val1 >= val2 {
        (val1, val2)
    } else {
        (val2, val1)
    };
    let expected = a - b;
    test_execution(build_asm_op(b).as_str(), &[a as u64], &[expected as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_execution(
        build_asm_op(b).as_str(),
        &[a as u64, c],
        &[expected as u64, c],
    );
}

#[test]
fn u32sub_b_fail() {
    let build_asm_op = |param: u64| format!("u32sub.{}", param);

    // should fail during execution if a >= 2^32
    test_execution_failure(build_asm_op(0).as_str(), &[U32_BOUND], "FailedAssertion");

    // should fail during compilation if b >= 2^32
    test_param_out_of_bounds(build_asm_op(U32_BOUND).as_str(), U32_BOUND);

    // should fail if a < b
    let a = 1_u64;
    let b = 2_u64;
    test_execution_failure(build_asm_op(b).as_str(), &[a], "FailedAssertion");
}

#[test]
fn u32sub_full() {
    let asm_op = "u32sub.full";

    // should push c = (a - b) % 2^32 onto the stack
    // should push underflow flag d, where d = 1 if a < b and d = 0 otherwise
    test_sub_full(asm_op);
}

#[test]
fn u32sub_full_fail() {
    let asm_op = "u32sub.full";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[0, U32_BOUND], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 0], "FailedAssertion");
}

#[test]
fn u32sub_unsafe() {
    let asm_op = "u32sub.unsafe";

    // should push c = (a - b) % 2^32 onto the stack
    // should push underflow flag d, where d = 1 if a < b and d = 0 otherwise
    test_sub_full(asm_op);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
fn u32mul() {
    let asm_op = "u32mul";

    // --- simple cases ---------------------------------------------------------------------------
    test_execution(asm_op, &[0, 1], &[0]);
    test_execution(asm_op, &[1, 5], &[5]);
    test_execution(asm_op, &[5, 2], &[10]);

    // --- random values --------------------------------------------------------------------------
    // test using u16 values to ensure there's no overflow so the result is valid
    let a = rand_value::<u64>() as u16;
    let b = rand_value::<u64>() as u16;

    let expected: u64 = a as u64 * b as u64;
    test_execution(asm_op, &[b as u64, a as u64], &[expected]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_execution(asm_op, &[b as u64, a as u64, c], &[expected, c]);
}

#[test]
fn u32mul_fail() {
    let asm_op = "u32mul";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[0, U32_BOUND], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 0], "FailedAssertion");

    // should fail if a * b  >= 2^32
    let a = u32::MAX as u64;
    let b = 2_u64;
    test_execution_failure(asm_op, &[b, a], "FailedAssertion");
}

#[test]
fn u32mul_b() {
    let build_asm_op = |param: u16| format!("u32mul.{}", param);

    // --- simple cases ---------------------------------------------------------------------------
    test_execution(build_asm_op(0).as_str(), &[1], &[0]);
    test_execution(build_asm_op(1).as_str(), &[5], &[5]);
    test_execution(build_asm_op(5).as_str(), &[2], &[10]);

    // --- random values --------------------------------------------------------------------------
    // test using u16 values to ensure there's no overflow so the result is valid
    let a = rand_value::<u64>() as u16;
    let b = rand_value::<u64>() as u16;

    let expected: u64 = a as u64 * b as u64;
    test_execution(build_asm_op(b).as_str(), &[a as u64], &[expected as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_execution(build_asm_op(5).as_str(), &[10, c], &[50, c]);
}

#[test]
fn u32mul_b_fail() {
    let build_asm_op = |param: u64| format!("u32mul.{}", param);

    // should fail during execution if a >= 2^32
    test_execution_failure(build_asm_op(0).as_str(), &[U32_BOUND], "FailedAssertion");

    // should fail during compilation if b >= 2^32
    test_param_out_of_bounds(build_asm_op(U32_BOUND).as_str(), U32_BOUND);

    // should fail if a * b >= 2^32
    let a = u32::MAX as u64;
    let b = u32::MAX as u64;
    test_execution_failure(build_asm_op(b).as_str(), &[a], "FailedAssertion");
}

#[test]
fn u32mul_full() {
    let asm_op = "u32mul.full";

    // should push c = (a * b) % 2^32 onto the stack
    // should push d = (a * b) / 2^32 onto the stack
    test_mul_full(asm_op);
}

#[test]
fn u32mul_full_fail() {
    let asm_op = "u32mul.full";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[0, U32_BOUND], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 0], "FailedAssertion");
}

#[test]
fn u32mul_unsafe() {
    let asm_op = "u32mul.unsafe";

    // should push c = (a * b) % 2^32 onto the stack
    // should push d = (a * b) / 2^32 onto the stack
    test_mul_full(asm_op);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
fn u32madd() {
    let asm_op = "u32madd";

    // should push d = (a * b + c) % 2^32 onto the stack
    // should push e = (a * b + c) / 2^32 onto the stack
    test_madd(asm_op);
}

#[test]
fn u32madd_fail() {
    let asm_op = "u32madd";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[0, U32_BOUND, 0], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 0, 0], "FailedAssertion");

    // should fail if c  >= 2^32
    test_execution_failure(asm_op, &[0, 0, U32_BOUND], "FailedAssertion");
}

#[test]
fn u32madd_unsafe() {
    let asm_op = "u32madd.unsafe";

    // should push d = (a * b + c) % 2^32 onto the stack
    // should push e = (a * b + c) / 2^32 onto the stack
    test_madd(asm_op);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 3);
}

#[test]
// issue: https://github.com/maticnetwork/miden/issues/94
fn u32div() {
    let asm_op = "u32div";

    // --- simple cases ---------------------------------------------------------------------------
    test_execution(asm_op, &[1, 0], &[0]);
    // division with no remainder
    test_execution(asm_op, &[1, 2], &[2]);
    // division with remainder
    test_execution(asm_op, &[2, 1], &[0]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let expected = a / b;
    test_execution(asm_op, &[b as u64, a as u64], &[expected as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_execution(asm_op, &[b as u64, a as u64, c], &[expected as u64, c]);
}

#[test]
fn u32div_fail() {
    let asm_op = "u32div";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[1, U32_BOUND], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 1], "FailedAssertion");
}

#[test]
#[should_panic = "divide by zero"]
fn u32div_panic() {
    let script = compile("begin u32div end");
    let inputs = build_inputs(&[0, 1]);

    // should panic if b = 0
    execute(&script, &inputs).unwrap();
}

#[test]
// issue: https://github.com/maticnetwork/miden/issues/94
fn u32div_b() {
    let build_asm_op = |param: u32| format!("u32div.{}", param);

    // --- simple cases ---------------------------------------------------------------------------
    test_execution(build_asm_op(1).as_str(), &[0], &[0]);
    // division with no remainder
    test_execution(build_asm_op(1).as_str(), &[2], &[2]);
    // division with remainder
    test_execution(build_asm_op(2).as_str(), &[1], &[0]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let expected = a / b;
    test_execution(build_asm_op(b).as_str(), &[a as u64], &[expected as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_execution(
        build_asm_op(b).as_str(),
        &[a as u64, c],
        &[expected as u64, c],
    );
}

#[test]
// issue: https://github.com/maticnetwork/miden/issues/94
fn u32div_b_fail() {
    let build_asm_op = |param: u64| format!("u32div.{}", param);

    // should fail during execution if a >= 2^32
    test_execution_failure(build_asm_op(1).as_str(), &[U32_BOUND], "FailedAssertion");

    // should fail during compilation if b >= 2^32
    test_param_out_of_bounds(build_asm_op(U32_BOUND).as_str(), U32_BOUND);

    // should fail during compilation if b = 0
    test_compilation_failure(build_asm_op(0).as_str(), "parameter");
}

#[test]
fn u32div_full() {
    let asm_op = "u32div.full";

    // should push the quotient c = a / b onto the stack
    // should push the remainder d = a % b onto the stack
    test_div_full(asm_op);
}

#[test]
fn u32div_full_fail() {
    let asm_op = "u32div.full";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[1, U32_BOUND], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 1], "FailedAssertion");
}

#[test]
#[should_panic = "divide by zero"]
fn u32div_full_panic() {
    let script = compile("begin u32div.full end");
    let inputs = build_inputs(&[0, 1]);

    // should panic if b = 0
    execute(&script, &inputs).unwrap();
}

#[test]
fn u32div_unsafe() {
    let asm_op = "u32div.unsafe";

    // should push c = (a * b) % 2^32 onto the stack
    // should push d = (a * b) / 2^32 onto the stack
    test_div_full(asm_op);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
#[should_panic = "divide by zero"]
fn u32div_unsafe_panic() {
    let script = compile("begin u32div.unsafe end");
    let inputs = build_inputs(&[0, 1]);

    // should panic if b = 0
    execute(&script, &inputs).unwrap();
}

#[test]
fn u32mod() {
    let asm_op = "u32mod";

    // should pop b, a off the stack and push the result of a % b onto the stack
    test_mod(asm_op);
}

#[test]
fn u32mod_fail() {
    let asm_op = "u32mod";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[1, U32_BOUND], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 1], "FailedAssertion");
}

#[test]
#[should_panic = "divide by zero"]
fn u32mod_panic() {
    let script = compile("begin u32mod end");
    let inputs = build_inputs(&[0, 1]);

    // should panic if b = 0
    execute(&script, &inputs).unwrap();
}

#[test]
fn u32mod_b() {
    let build_asm_op = |param: u32| format!("u32mod.{}", param);

    // --- simple cases ---------------------------------------------------------------------------
    test_execution(build_asm_op(5).as_str(), &[10], &[0]);
    test_execution(build_asm_op(5).as_str(), &[11], &[1]);
    test_execution(build_asm_op(11).as_str(), &[5], &[5]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let mut b = rand_value::<u64>() as u32;
    if b == 0 {
        // ensure we're not using a failure case
        b += 1;
    }
    let expected = a % b;
    test_execution(build_asm_op(b).as_str(), &[a as u64], &[expected as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_execution(
        build_asm_op(b).as_str(),
        &[a as u64, c],
        &[expected as u64, c],
    );
}

#[test]
fn u32mod_b_fail() {
    let build_asm_op = |param: u64| format!("u32mod.{}", param);

    // should fail during exeuction if a >= 2^32
    test_execution_failure(build_asm_op(1).as_str(), &[U32_BOUND], "FailedAssertion");

    // should fail during compilation if b >= 2^32
    test_param_out_of_bounds(build_asm_op(U32_BOUND).as_str(), U32_BOUND);

    // should fail during compilation if b = 0
    test_compilation_failure(build_asm_op(0).as_str(), "parameter");
}

#[test]
fn u32mod_unsafe() {
    let asm_op = "u32mod.unsafe";

    // should pop b, a off the stack and push the result of a % b onto the stack
    test_mod(asm_op);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
#[should_panic = "divide by zero"]
fn u32mod_unsafe_panic() {
    let script = compile("begin u32mod.unsafe end");
    let inputs = build_inputs(&[0, 1]);

    // should panic if b = 0
    execute(&script, &inputs).unwrap();
}

// U32 OPERATIONS TESTS - MANUAL - BITWISE OPERATIONS
// ================================================================================================

#[test]
fn u32and() {
    let asm_op = "u32and";

    // --- simple cases ---------------------------------------------------------------------------
    test_execution(asm_op, &[1, 1], &[1]);
    test_execution(asm_op, &[0, 1], &[0]);
    test_execution(asm_op, &[1, 0], &[0]);
    test_execution(asm_op, &[0, 0], &[0]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    test_execution(asm_op, &[a as u64, b as u64], &[(a & b) as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>() as u32;
    let d = rand_value::<u64>() as u32;
    test_execution(
        asm_op,
        &[a as u64, b as u64, c as u64, d as u64],
        &[(a & b) as u64, c as u64, d as u64],
    );
}

#[test]
// issue: https://github.com/maticnetwork/miden/issues/74
fn u32and_fail() {
    let asm_op = "u32and";

    test_execution_failure(asm_op, &[U32_BOUND, 0], "FailedAssertion");
    test_execution_failure(asm_op, &[0, U32_BOUND], "FailedAssertion");
}

#[test]
fn u32or() {
    let asm_op = "u32or";

    // --- simple cases ---------------------------------------------------------------------------
    test_execution(asm_op, &[1, 1], &[1]);
    test_execution(asm_op, &[0, 1], &[1]);
    test_execution(asm_op, &[1, 0], &[1]);
    test_execution(asm_op, &[0, 0], &[0]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    test_execution(asm_op, &[a as u64, b as u64], &[(a | b) as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>() as u32;
    let d = rand_value::<u64>() as u32;
    test_execution(
        asm_op,
        &[a as u64, b as u64, c as u64, d as u64],
        &[(a | b) as u64, c as u64, d as u64],
    );
}

#[test]
// issue: https://github.com/maticnetwork/miden/issues/74
fn u32or_fail() {
    let asm_op = "u32or";

    test_execution_failure(asm_op, &[U32_BOUND, 0], "FailedAssertion");
    test_execution_failure(asm_op, &[0, U32_BOUND], "FailedAssertion");
}

#[test]
fn u32xor() {
    let asm_op = "u32xor";

    // --- simple cases ---------------------------------------------------------------------------
    test_execution(asm_op, &[1, 1], &[0]);
    test_execution(asm_op, &[0, 1], &[1]);
    test_execution(asm_op, &[1, 0], &[1]);
    test_execution(asm_op, &[0, 0], &[0]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    test_execution(asm_op, &[a as u64, b as u64], &[(a ^ b) as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>() as u32;
    let d = rand_value::<u64>() as u32;
    test_execution(
        asm_op,
        &[a as u64, b as u64, c as u64, d as u64],
        &[(a ^ b) as u64, c as u64, d as u64],
    );
}

#[test]
// issue: https://github.com/maticnetwork/miden/issues/74
fn u32xor_fail() {
    let asm_op = "u32xor";

    test_execution_failure(asm_op, &[U32_BOUND, 0], "FailedAssertion");
    test_execution_failure(asm_op, &[0, U32_BOUND], "FailedAssertion");
}

#[test]
fn u32not() {
    let asm_op = "u32not";

    // --- simple cases ---------------------------------------------------------------------------
    test_execution(asm_op, &[U32_BOUND - 1], &[0]);
    test_execution(asm_op, &[0], &[U32_BOUND - 1]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    test_execution(asm_op, &[a as u64], &[!a as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let b = rand_value::<u64>() as u32;
    test_execution(asm_op, &[a as u64, b as u64], &[!a as u64, b as u64]);
}

#[test]
fn u32not_fail() {
    let asm_op = "u32not";
    test_input_out_of_bounds(asm_op);
}

#[test]
// https://github.com/maticnetwork/miden/issues/76
fn u32shl() {
    // left shift: pops a from the stack and pushes (a * 2^b) mod 2^32 for a provided value b
    let op_base = "u32shl";
    let get_asm_op = |b: u32| format!("{}.{}", op_base, b);
    let get_result = |a, b| (a << b) % U32_BOUND;

    // --- test simple case -----------------------------------------------------------------------
    let a = 1_u64;
    let b = 1_u32;
    test_execution(get_asm_op(b).as_str(), &[a], &[2]);

    // --- test max values of a and b -------------------------------------------------------------
    let a = U32_BOUND - 1;
    let b = 31;
    test_execution(
        get_asm_op(b).as_str(),
        &[U32_BOUND - 1],
        &[get_result(a, b)],
    );

    // --- test b = 0 -----------------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = 0;
    test_execution(
        get_asm_op(b).as_str(),
        &[a as u64],
        &[get_result(a as u64, b)],
    );

    // --- test random values ---------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = (rand_value::<u64>() % 32) as u32;
    test_execution(
        get_asm_op(b).as_str(),
        &[a as u64],
        &[get_result(a as u64, b)],
    );
}

#[test]
fn u32shl_fail() {
    let op_base = "u32shl";

    test_input_out_of_bounds(format!("{}.{}", op_base, 1).as_str());
    test_param_out_of_bounds(op_base, 32);
}

#[test]
// https://github.com/maticnetwork/miden/issues/76
fn u32shr() {
    // right shift: pops a from the stack and pushes a / 2^b for a provided value b
    let op_base = "u32shr";
    let get_asm_op = |b: u32| format!("{}.{}", op_base, b);
    let get_result = |a, b| a >> b;

    // --- test simple case -----------------------------------------------------------------------
    let a = 4_u64;
    let b = 2_u32;
    test_execution(get_asm_op(b).as_str(), &[a], &[1]);

    // --- test max values of a and b -------------------------------------------------------------
    let a = U32_BOUND - 1;
    let b = 31;
    test_execution(
        get_asm_op(b).as_str(),
        &[U32_BOUND - 1],
        &[get_result(a, b)],
    );

    // --- test b = 0 ---------------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = 0;
    test_execution(
        get_asm_op(b).as_str(),
        &[a as u64],
        &[get_result(a as u64, b)],
    );

    // --- test random values ---------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = (rand_value::<u64>() % 32) as u32;
    test_execution(
        get_asm_op(b).as_str(),
        &[a as u64],
        &[get_result(a as u64, b)],
    );
}

#[test]
fn u32shr_fail() {
    let op_base = "u32shr";

    test_input_out_of_bounds(format!("{}.{}", op_base, 1).as_str());
    test_param_out_of_bounds(op_base, 32);
}

#[test]
// https://github.com/maticnetwork/miden/issues/76
fn u32rotl() {
    // Computes c by rotating a 32-bit representation of a to the left by b bits.
    let op_base = "u32rotl";
    let get_asm_op = |b: u32| format!("{}.{}", op_base, b);

    // --- test simple case -----------------------------------------------------------------------
    let a = 1_u32;
    let b = 1_u32;
    test_execution(get_asm_op(b).as_str(), &[a as u64], &[2]);

    // --- test simple wraparound case with large a -----------------------------------------------
    let a = (1_u64 << 31) as u32;
    let b: u32 = 1;
    test_execution(get_asm_op(b).as_str(), &[a as u64], &[1]);

    // --- test simple case wraparound case with max b --------------------------------------------
    let a = 2_u32;
    let b: u32 = 31;
    test_execution(get_asm_op(b).as_str(), &[a as u64], &[1]);

    // --- no change when a is max value (all 1s) -------------------------------------------------
    let a = (U32_BOUND - 1) as u32;
    let b = 2;
    test_execution(get_asm_op(b).as_str(), &[a as u64], &[a as u64]);

    // --- test b = 0 ---------------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = 0 as u32;
    test_execution(
        get_asm_op(b).as_str(),
        &[a as u64],
        &[a.rotate_left(b) as u64],
    );

    // --- test random values ---------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = (rand_value::<u64>() % 32) as u32;
    test_execution(
        get_asm_op(b).as_str(),
        &[a as u64],
        &[a.rotate_left(b) as u64],
    );
}

#[test]
fn u32rotl_fail() {
    let op_base = "u32rotl";

    test_input_out_of_bounds(format!("{}.{}", op_base, 1).as_str());
    test_param_out_of_bounds(op_base, 32);
}

#[test]
// https://github.com/maticnetwork/miden/issues/76
fn u32rotr() {
    // Computes c by rotating a 32-bit representation of a to the right by b bits.
    let op_base = "u32rotr";
    let get_asm_op = |b: u32| format!("{}.{}", op_base, b);

    // --- test simple case -----------------------------------------------------------------------
    let a = 2_u32;
    let b = 1_u32;
    test_execution(get_asm_op(b).as_str(), &[a as u64], &[1]);

    // --- test simple wraparound case with small a -----------------------------------------------
    let a = 1_u32;
    let b = 1_u32;
    test_execution(get_asm_op(b).as_str(), &[a as u64], &[U32_BOUND >> 1]);

    // --- test simple case wraparound case with max b --------------------------------------------
    let a = 1_u32;
    let b: u32 = 31;
    test_execution(get_asm_op(b).as_str(), &[a as u64], &[2]);

    // --- no change when a is max value (all 1s) -------------------------------------------------
    let a = (U32_BOUND - 1) as u32;
    let b = 2;
    test_execution(get_asm_op(b).as_str(), &[a as u64], &[a as u64]);

    // --- test b = 0 ---------------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = 0 as u32;
    test_execution(
        get_asm_op(b).as_str(),
        &[a as u64],
        &[a.rotate_right(b) as u64],
    );

    // --- test random values ---------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = (rand_value::<u64>() % 32) as u32;
    test_execution(
        get_asm_op(b).as_str(),
        &[a as u64],
        &[a.rotate_right(b) as u64],
    );
}

#[test]
fn u32rotr_fail() {
    let op_base = "u32rotr";

    test_input_out_of_bounds(format!("{}.{}", op_base, 1).as_str());
    test_param_out_of_bounds(op_base, 32);
}

// U32 OPERATIONS TESTS - MANUAL - COMPARISON OPERATIONS
// ================================================================================================

#[test]
fn u32eq() {
    let asm_op = "u32eq";

    // --- simple cases ---------------------------------------------------------------------------
    test_execution(asm_op, &[1, 1], &[1]);
    test_execution(asm_op, &[1, 0], &[0]);

    // --- random u32: equality -------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    test_execution(asm_op, &[a as u64, a as u64], &[1]);

    // --- random u32: probable inequality --------------------------------------------------------
    let b = rand_value::<u64>() as u32;
    let expected = if a == b { 1 } else { 0 };
    test_execution(asm_op, &[a as u64, b as u64], &[expected]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_execution(asm_op, &[a as u64, b as u64, c], &[expected, c]);
}

#[test]
fn u32eq_fail() {
    let asm_op = "u32eq";

    // should fail if either one of 2 inputs is out of bounds
    test_inputs_out_of_bounds(asm_op, 2);
}

#[test]
fn u32eq_b() {
    let build_asm_op = |param: u32| format!("u32eq.{}", param);

    // --- simple cases ---------------------------------------------------------------------------
    test_execution(build_asm_op(1).as_str(), &[1], &[1]);
    test_execution(build_asm_op(0).as_str(), &[1], &[0]);

    // --- random u32: equality -------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    test_execution(build_asm_op(a).as_str(), &[a as u64], &[1]);

    // --- random u32: probable inequality --------------------------------------------------------
    let b = rand_value::<u64>() as u32;
    let expected = if a == b { 1 } else { 0 };
    test_execution(build_asm_op(b).as_str(), &[a as u64], &[expected]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_execution(build_asm_op(b).as_str(), &[a as u64, c], &[expected, c]);
}

#[test]
fn u32eq_b_fail() {
    let asm_op_base = "u32eq";

    // should fail when b is out of bounds and provided as a parameter
    test_param_out_of_bounds(asm_op_base, U32_BOUND);

    // should fail when b is a valid parameter but a is out of bounds
    test_execution_failure(
        format!("{}.{}", asm_op_base, 1).as_str(),
        &[U32_BOUND],
        "FailedAssertion",
    );
}

#[test]
fn u32neq() {
    let asm_op = "u32neq";

    // --- simple cases ---------------------------------------------------------------------------
    test_execution(asm_op, &[1, 1], &[0]);
    test_execution(asm_op, &[1, 0], &[1]);

    // --- random u32: equality -------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    test_execution(asm_op, &[a as u64, a as u64], &[0]);

    // --- random u32: probable inequality --------------------------------------------------------
    let b = rand_value::<u64>() as u32;
    let expected = if a != b { 1 } else { 0 };
    test_execution(asm_op, &[a as u64, b as u64], &[expected]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_execution(asm_op, &[a as u64, b as u64, c], &[expected, c]);
}

#[test]
fn u32neq_fail() {
    let asm_op = "u32neq";

    // should fail if either one of 2 inputs is out of bounds
    test_inputs_out_of_bounds(asm_op, 2);
}

#[test]
fn u32neq_b() {
    let build_asm_op = |param: u32| format!("u32neq.{}", param);

    // --- simple cases ---------------------------------------------------------------------------
    test_execution(build_asm_op(1).as_str(), &[1], &[0]);
    test_execution(build_asm_op(0).as_str(), &[1], &[1]);

    // --- random u32: equality -------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    test_execution(build_asm_op(a).as_str(), &[a as u64], &[0]);

    // --- random u32: probable inequality --------------------------------------------------------
    let b = rand_value::<u64>() as u32;
    let expected = if a != b { 1 } else { 0 };
    test_execution(build_asm_op(b).as_str(), &[a as u64], &[expected]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_execution(build_asm_op(b).as_str(), &[a as u64, c], &[expected, c]);
}

#[test]
fn u32neq_b_fail() {
    let asm_op_base = "u32neq";

    // should fail when b is out of bounds and provided as a parameter
    test_param_out_of_bounds(asm_op_base, U32_BOUND);

    // should fail when b is a valid parameter but a is out of bounds
    test_execution_failure(
        format!("{}.{}", asm_op_base, 1).as_str(),
        &[U32_BOUND],
        "FailedAssertion",
    );
}

#[test]
fn u32lt() {
    let asm_op = "u32lt";

    // should push 1 to the stack when a < b and 0 otherwise
    test_comparison_op(asm_op, 1, 0, 0);
}

#[test]
fn u32lt_fail() {
    let asm_op = "u32lt";

    // should fail if either one of 2 inputs is out of bounds
    test_inputs_out_of_bounds(asm_op, 2);
}

#[test]
fn u32lt_unsafe() {
    let asm_op = "u32lt.unsafe";

    // should push 1 to the stack when a < b and 0 otherwise
    test_comparison_op(asm_op, 1, 0, 0);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
fn u32lte() {
    let asm_op = "u32lte";

    // should push 1 to the stack when a <= b and 0 otherwise
    test_comparison_op(asm_op, 1, 1, 0);
}

#[test]
fn u32lte_fail() {
    let asm_op = "u32lte";

    // should fail if either one of 2 inputs is out of bounds
    test_inputs_out_of_bounds(asm_op, 2);
}

#[test]
fn u32lte_unsafe() {
    let asm_op = "u32lte.unsafe";

    // should push 1 to the stack when a <= b and 0 otherwise
    test_comparison_op(asm_op, 1, 1, 0);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
fn u32gt() {
    let asm_op = "u32gt";

    // should push 1 to the stack when a > b and 0 otherwise
    test_comparison_op(asm_op, 0, 0, 1);
}

#[test]
fn u32gt_fail() {
    let asm_op = "u32gt";

    // should fail if either one of 2 inputs is out of bounds
    test_inputs_out_of_bounds(asm_op, 2);
}

#[test]
fn u32gt_unsafe() {
    let asm_op = "u32gt.unsafe";

    // should push 1 to the stack when a > b and 0 otherwise
    test_comparison_op(asm_op, 0, 0, 1);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
fn u32gte() {
    let asm_op = "u32gte";

    // should push 1 to the stack when a >= b and 0 otherwise
    test_comparison_op(asm_op, 0, 1, 1);
}

#[test]
fn u32gte_fail() {
    let asm_op = "u32gte";

    // should fail if either one of 2 inputs is out of bounds
    test_inputs_out_of_bounds(asm_op, 2);
}

#[test]
fn u32gte_unsafe() {
    let asm_op = "u32gte.unsafe";

    // should push 1 to the stack when a >= b and 0 otherwise
    test_comparison_op(asm_op, 0, 1, 1);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
fn u32min() {
    let asm_op = "u32min";

    // should put the minimum of the 2 inputs on the stack
    test_min(asm_op);
}

#[test]
fn u32min_fail() {
    let asm_op = "u32min";

    // should fail if either one of 2 inputs is out of bounds
    test_inputs_out_of_bounds(asm_op, 2);
}

#[test]
fn u32min_unsafe() {
    let asm_op = "u32min.unsafe";

    // should put the minimum of the 2 inputs on the stack
    test_min(asm_op);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
fn u32max() {
    let asm_op = "u32max";

    // should put the maximum of the 2 inputs on the stack
    test_max(asm_op);
}

#[test]
fn u32max_fail() {
    let asm_op = "u32max";

    // should fail if either one of 2 inputs is out of bounds
    test_inputs_out_of_bounds(asm_op, 2);
}

#[test]
fn u32max_unsafe() {
    let asm_op = "u32max.unsafe";

    // should put the maximum of the 2 inputs on the stack
    test_max(asm_op);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

// U32 OPERATIONS TESTS - RANDOMIZED - CONVERSIONS AND TESTS
// ================================================================================================
proptest! {
    #[test]
    fn u32test_proptest(value in any::<u64>()) {
        // check to see if the value of the element will be a valid u32
        let expected_result = if value % Felt::MODULUS < U32_BOUND { 1 } else { 0 };

        test_execution("u32test", &[value], &[expected_result, value]);
    }

    #[test]
    fn u32testw_proptest(word in rand_word::<u32>()) {
        // should leave a 1 on the stack since all values in the word are valid u32 values
        let values: Vec<u64> = word.iter().map(|a| *a as u64).collect();
        let mut expected = values.clone();
        expected.insert(0, 1);

        test_execution("u32testw", &values, &expected);
    }

    #[test]
    fn u32assert_proptest(value in any::<u32>()) {
        // assertion passes and leaves the stack unchanged if a < 2^32
        let asm_op = "u32assert";
        test_execution(asm_op, &[value as u64], &[value as u64]);
    }

    #[test]
    fn u32assertw_proptest(word in rand_word::<u32>()) {
        // should pass and leave the stack unchanged if a < 2^32 for all values in the word
        let asm_op = "u32assertw";
        let values: Vec<u64> = word.iter().map(|a| *a as u64).collect();

        test_execution(asm_op, &values, &values);
    }

    #[test]
    fn u32cast_proptest(value in any::<u64>()) {
        // expected result will be mod 2^32 applied to a field element
        // so the field modulus should be applied first
        let expected_result = value % Felt::MODULUS % U32_BOUND;

        test_execution("u32cast", &[value], &[expected_result]);
    }

    #[test]
    fn u32split_proptest(value in any::<u64>()) {
        // expected result will be mod 2^32 applied to a field element
        // so the field modulus must be applied first
        let felt_value = value % Felt::MODULUS;
        let expected_b = felt_value >> 32;
        let expected_c = felt_value as u32 as u64;

        test_execution("u32split", &[value, value], &[expected_b, expected_c, value]);
    }
}

// U32 OPERATIONS TESTS - RANDOMIZED - ARITHMETIC OPERATIONS
// ================================================================================================
proptest! {
    #[test]
    fn u32add_proptest(a in any::<u16>(), b in any::<u16>()) {
        let asm_op = "u32add";

        let expected = a as u64 + b as u64;

        // b provided via the stack
        test_execution(asm_op, &[b as u64, a as u64], &[expected]);
        // b provided as a parameter
        test_execution(format!("{}.{}", asm_op, b).as_str(), &[a as u64], &[expected]);
    }

    #[test]
    fn u32add_full_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_op = "u32add";

        let (c, overflow) = a.overflowing_add(b);
        let d = if overflow { 1 } else { 0 };

        // full and unsafe should produce the same result for valid values
        test_execution(format!("{}.full", asm_op).as_str(), &[b as u64, a as u64], &[d, c as u64]);
        test_execution(format!("{}.unsafe", asm_op).as_str(), &[b as u64, a as u64], &[d, c as u64]);
    }

    #[test]
    fn u32addc_proptest(a in any::<u32>(), b in any::<u32>(), c in 0_u32..1) {
        let asm_op = "u32addc";

        let (d, overflow_b) = a.overflowing_add(b);
        let (d, overflow_c) = d.overflowing_add(c);
        let e = if overflow_b || overflow_c { 1_u64 } else { 0_u64 };

        // safe and unsafe should produce the same result for valid values
        test_execution(asm_op, &[b as u64, a as u64, c as u64], &[e, d as u64]);
        test_execution(format!("{}.unsafe", asm_op).as_str(), &[b as u64, a as u64, c as u64], &[e, d as u64]);
    }

    #[test]
    fn u32sub_proptest(val1 in any::<u32>(), val2 in any::<u32>()) {
        let asm_op = "u32sub";

        // assign the larger value to a and the smaller value to b so all parameters are valid
        let (a, b) = if val1 >= val2 {
            (val1, val2)
        } else {
            (val2, val1)
        };

        let expected = a - b;
        // b provided via the stack
        test_execution(asm_op, &[b as u64, a as u64], &[expected as u64]);
        // b provided as a parameter
        test_execution(format!("{}.{}", asm_op, b).as_str(), &[a as u64], &[expected as u64]);
    }

    #[test]
    fn u32sub_full_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_op = "u32sub";

        // assign the larger value to a and the smaller value to b so all parameters are valid
        let (c, overflow) = a.overflowing_sub(b);
        let d = if overflow { 1 } else { 0 };

        // full and unsafe should produce the same result for valid values
        test_execution(format!("{}.full", asm_op).as_str(), &[b as u64, a as u64], &[d, c as u64]);
        test_execution(format!("{}.unsafe", asm_op).as_str(), &[b as u64, a as u64], &[d, c as u64]);
    }

    #[test]
    fn u32mul_proptest(a in any::<u16>(), b in any::<u16>()) {
        let asm_op = "u32mul";

        let expected = a as u64 * b as u64;

        // b provided via the stack
        test_execution(asm_op, &[b as u64, a as u64], &[expected]);
        // b provided as a parameter
        test_execution(format!("{}.{}", asm_op, b).as_str(), &[a as u64], &[expected]);
    }

    #[test]
    fn u32mul_full_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_op = "u32mul";

        let (c, overflow) = a.overflowing_mul(b);
        let d = if !overflow {
            0
        } else {
            (a as u64 * b as u64) / U32_BOUND
        };

        // full and unsafe should produce the same result for valid values
        test_execution(format!("{}.full", asm_op).as_str(), &[b as u64, a as u64], &[d, c as u64]);
        test_execution(format!("{}.unsafe", asm_op).as_str(), &[b as u64, a as u64], &[d, c as u64]);
    }

    #[test]
    fn u32madd_proptest(a in any::<u32>(), b in any::<u32>(), c in any::<u32>()) {
        let asm_op = "u32madd";

        let madd = a as u64 * b as u64 + c as u64;
        let d = madd % U32_BOUND;
        let e = madd / U32_BOUND;

        // safe and unsafe should produce the same result for valid values
        test_execution(asm_op, &[b as u64, a as u64, c as u64], &[e, d as u64]);
        test_execution(format!("{}.unsafe", asm_op).as_str(), &[b as u64, a as u64, c as u64], &[e, d as u64]);
    }

    #[test]
    // issue: https://github.com/maticnetwork/miden/issues/94
    fn u32div_proptest(a in any::<u32>(), b in 1..u32::MAX) {
        let asm_op = "u32div";

        let expected = a / b;

        // b provided via the stack
        test_execution(asm_op, &[b as u64, a as u64], &[expected as u64]);
        // b provided as a parameter
        test_execution(format!("{}.{}", asm_op, b).as_str(), &[a as u64], &[expected as u64]);
    }

    #[test]
    fn u32div_full_proptest(a in any::<u32>(), b in 1..u32::MAX) {
        let asm_op = "u32div";

        let quot = (a / b) as u64;
        let rem = (a % b) as u64;

        // full and unsafe should produce the same result for valid values
        test_execution(format!("{}.full", asm_op).as_str(), &[b as u64, a as u64], &[rem, quot]);
        test_execution(format!("{}.unsafe", asm_op).as_str(), &[b as u64, a as u64], &[rem, quot]);
    }

    #[test]
    fn u32mod_proptest(a in any::<u32>(), b in 1..u32::MAX) {
        let asm_op = "u32mod";

        let expected = a % b;

        // b provided via the stack
        test_execution(asm_op, &[b as u64, a as u64], &[expected as u64]);
        // b provided as a parameter
        test_execution(format!("{}.{}", asm_op, b).as_str(), &[a as u64], &[expected as u64]);
        // safe and unsafe should produce the same result for valid values
        test_execution(format!("{}.unsafe", asm_op).as_str(), &[b as u64, a as u64], &[expected as u64]);
    }
}

// U32 OPERATIONS TESTS - RANDOMIZED - BITWISE OPERATIONS
// ================================================================================================

proptest! {
    #[test]
    fn u32and_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_opcode = "u32and";
        let values = [b as u64, a as u64];
        // should result in bitwise AND
        let expected = (a & b) as u64;

        test_execution(asm_opcode, &values, &[expected]);
    }

    #[test]
    fn u32or_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_opcode = "u32or";
        let values = [b as u64, a as u64];
        // should result in bitwise OR
        let expected = (a | b) as u64;

        test_execution(asm_opcode, &values, &[expected]);
    }

    #[test]
    fn u32xor_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_opcode = "u32xor";
        let values = [b as u64, a as u64];
        // should result in bitwise XOR
        let expected = (a ^ b) as u64;

        test_execution(asm_opcode, &values, &[expected]);
    }

    #[test]
    fn u32not_proptest(value in any::<u32>()) {
        let asm_opcode = "u32not";

        // should result in bitwise NOT
        test_execution(asm_opcode, &[value as u64], &[!value as u64]);
    }

    #[test]
    // https://github.com/maticnetwork/miden/issues/76
    fn u32shl_proptest(a in any::<u32>(), b in 0_u32..32) {
        let asm_opcode = format!("u32shl.{}", b);

        // should execute left shift
        let expected =  a << b;
        test_execution(&asm_opcode, &[a as u64], &[expected as u64]);
    }

    #[test]
    // https://github.com/maticnetwork/miden/issues/76
    fn u32shr_proptest(a in any::<u32>(), b in 0_u32..32) {
        let asm_opcode = format!("u32shr.{}", b);

        // should execute right shift
        let expected =  a >> b;
        test_execution(&asm_opcode, &[a as u64], &[expected as u64]);
    }

    #[test]
    // https://github.com/maticnetwork/miden/issues/76
    fn u32rotl_proptest(a in any::<u32>(), b in 0_u32..32) {
        let op_base = "u32rotl";
        let asm_opcode = format!("{}.{}", op_base, b);

        // should execute left bit rotation
        test_execution(&asm_opcode, &[a as u64], &[a.rotate_left(b) as u64]);
    }

    #[test]
    // https://github.com/maticnetwork/miden/issues/76
    fn u32rotr_proptest(a in any::<u32>(), b in 0_u32..32) {
        let op_base = "u32rotr";
        let asm_opcode = format!("{}.{}", op_base, b);

        // should execute right bit rotation
        test_execution(&asm_opcode, &[a as u64], &[a.rotate_right(b) as u64]);
    }
}

// U32 OPERATIONS TESTS - RANDOMIZED - COMPARISON OPERATIONS
// ================================================================================================

proptest! {
    #[test]
    fn u32eq_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_op = "u32eq";
        let values = [b as u64, a as u64];

        // should test for equality
        let expected = if a == b { 1 } else { 0 };
        // b provided via the stack
        test_execution(asm_op, &values, &[expected]);
        // b provided as a parameter
        test_execution(format!("{}.{}", asm_op, b).as_str(), &[a as u64], &[expected]);
    }

    #[test]
    fn u32neq_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_op = "u32neq";
        let values = [b as u64, a as u64];

        // should test for inequality
        let expected = if a != b { 1 } else { 0 };
        // b provided via the stack
        test_execution(asm_op, &values, &[expected]);
        // b provided as a parameter
        test_execution(format!("{}.{}", asm_op, b).as_str(), &[a as u64], &[expected]);
    }

    #[test]
    fn u32lt_proptest(a in any::<u32>(), b in any::<u32>()) {
        let expected = match a.cmp(&b) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => 0,
        };

        // safe and unsafe should produce the same result for valid values
        test_execution("u32lt", &[b as u64, a as u64], &[expected]);
        test_execution("u32lt.unsafe", &[b as u64, a as u64], &[expected]);
    }

    #[test]
    fn u32lte_proptest(a in any::<u32>(), b in any::<u32>()) {
        let expected = match a.cmp(&b) {
            Ordering::Less => 1,
            Ordering::Equal => 1,
            Ordering::Greater => 0,
        };

        // safe and unsafe should produce the same result for valid values
        test_execution("u32lte", &[b as u64, a as u64], &[expected]);
        test_execution("u32lte.unsafe", &[b as u64, a as u64], &[expected]);
    }

    #[test]
    fn u32gt_proptest(a in any::<u32>(), b in any::<u32>()) {
        let expected = match a.cmp(&b) {
            Ordering::Less => 0,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };

        // safe and unsafe should produce the same result for valid values
        test_execution("u32gt", &[b as u64, a as u64], &[expected]);
        test_execution("u32gt.unsafe", &[b as u64, a as u64], &[expected]);
    }

    #[test]
    fn u32gte_proptest(a in any::<u32>(), b in any::<u32>()) {
        let expected = match a.cmp(&b) {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => 1,
        };

        // safe and unsafe should produce the same result for valid values
        test_execution("u32gte", &[b as u64, a as u64], &[expected]);
        test_execution("u32gte.unsafe", &[b as u64, a as u64], &[expected]);
    }

    #[test]
    fn u32min_proptest(a in any::<u32>(), b in any::<u32>()) {
        let expected = if a < b { a } else { b };

        // safe and unsafe should produce the same result for valid values
        test_execution("u32min", &[b as u64, a as u64], &[expected as u64]);
        test_execution("u32min.unsafe", &[b as u64, a as u64], &[expected as u64]);
    }

    #[test]
    fn u32max_proptest(a in any::<u32>(), b in any::<u32>()) {
        let expected = if a > b { a } else { b };

        // safe and unsafe should produce the same result for valid values
        test_execution("u32max", &[b as u64, a as u64], &[expected as u64]);
        test_execution("u32max.unsafe", &[b as u64, a as u64], &[expected as u64]);
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// This helper function tests a provided u32 assembly operation, which takes a single input, to
/// ensure that it fails when the input is >= 2^32.
fn test_input_out_of_bounds(asm_op: &str) {
    test_execution_failure(asm_op, &[U32_BOUND], "FailedAssertion");
}

/// This helper function tests a provided u32 assembly operation, which takes multiple inputs, to
/// ensure that it fails when any one of the inputs is >= 2^32. Each input is tested independently.
fn test_inputs_out_of_bounds(asm_op: &str, input_count: usize) {
    let inputs = vec![0_u64; input_count];

    for i in 0..input_count {
        let mut i_inputs = inputs.clone();
        // should fail when the value of the input at index i is out of bounds
        i_inputs[i] = U32_BOUND;
        test_execution_failure(asm_op, &i_inputs, "FailedAssertion");
    }
}

/// This helper function tests that when the given u32 assembly instruction is executed on
/// out-of-bounds inputs it does not fail. Each input is tested independently.
fn test_unsafe_execution(asm_op: &str, input_count: usize) {
    let script = compile(format!("begin {} end", asm_op).as_str());
    let values = vec![1_u64; input_count];

    for i in 0..input_count {
        let mut i_values = values.clone();
        // should execute successfully when the value of the input at index i is out of bounds
        i_values[i] = U32_BOUND;
        let inputs = build_inputs(&i_values);
        assert!(execute(&script, &inputs).is_ok());
    }
}

/// This helper function tests overflowing addition for two u32 inputs for a number of simple cases
/// as well as for random values. It checks that a result of (a + b) % 2^32 is pushed to the stack,
/// as well as a flag indicating whether or not arithmetic overflow occurred. Finally, it ensures
/// that the rest of the stack was unaffected.
fn test_add_full(asm_op: &str) {
    // --- (a + b) < 2^32 -------------------------------------------------------------------------
    // c = a + b and d should be unset, since there was no overflow
    test_execution(asm_op, &[2, 1], &[0, 3]);

    // --- (a + b) = 2^32 -------------------------------------------------------------------------
    let a = u32::MAX;
    let b = 1_u64;
    // c should be the sum mod 2^32 and d should be set to signal overflow
    test_execution(asm_op, &[b, a as u64], &[1, 0]);

    // --- (a + b) > 2^32 -------------------------------------------------------------------------
    let a = 2_u64;
    let b = u32::MAX;
    // c should be the sum mod 2^32 and d should be set to signal overflow
    test_execution(asm_op, &[b as u64, a], &[1, 1]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let (c, overflow) = a.overflowing_add(b);
    let d = if overflow { 1 } else { 0 };
    test_execution(asm_op, &[b as u64, a as u64], &[d, c as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let e = rand_value::<u64>();
    test_execution(asm_op, &[b as u64, a as u64, e], &[d, c as u64, e]);
}

/// This helper function tests overflowing add with carry for two u32 inputs a, b and one binary
/// value c. It tests a number of simple cases as well as random u32 values. It checks that a
/// result of (a + b + c) % 2^32 is pushed to the stack, along with a flag indicating whether or
/// not arithmetic overflow occurred. Finally, it ensures that the rest of the stack was
/// unaffected.
fn test_addc(asm_op: &str) {
    // --- (a + b + c) < 2^32 where c = 0 ---------------------------------------------------------
    // d = a + b + c and e should be unset, since there was no overflow
    test_execution(asm_op, &[2, 1, 0], &[0, 3]);

    // --- (a + b + c) < 2^32 where c = 1 ---------------------------------------------------------
    // d = a + b + c and e should be unset, since there was no overflow
    test_execution(asm_op, &[3, 2, 1], &[0, 6]);

    // --- (a + b + c) = 2^32 ---------------------------------------------------------------------
    let a = u32::MAX;
    let b = 1_u64;
    // d should be the sum mod 2^32 and e should be set to signal overflow
    test_execution(asm_op, &[b, a as u64, 0], &[1, 0]);

    // --- (a + b + c) > 2^32 ---------------------------------------------------------------------
    let a = 1_u64;
    let b = u32::MAX;
    // d should be the sum mod 2^32 and e should be set to signal overflow
    test_execution(asm_op, &[b as u64, a, 1], &[1, 1]);

    // --- random u32 values with c = 0 -----------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let c = 0_u64;
    let (d, overflow) = a.overflowing_add(b);
    let e = if overflow { 1 } else { 0 };
    test_execution(asm_op, &[b as u64, a as u64, c], &[e, d as u64]);

    // --- random u32 values with c = 1 -----------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let c = 1_u32;
    let (d, overflow_b) = a.overflowing_add(b);
    let (d, overflow_c) = d.overflowing_add(c);
    let e = if overflow_b || overflow_c { 1 } else { 0 };
    test_execution(asm_op, &[b as u64, a as u64, c as u64], &[e, d as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let f = rand_value::<u64>();
    test_execution(
        asm_op,
        &[b as u64, a as u64, c as u64, f],
        &[e, d as u64, f],
    );
}

/// This helper function tests overflowing subtraction for two u32 inputs for a number of simple
/// cases as well as for random values. It checks that a result of (a + b) % 2^32 is pushed to the
/// stack, as well as a flag indicating whether or not arithmetic overflow occurred. Finally, it
/// ensures that the rest of the stack was unaffected.
fn test_sub_full(asm_op: &str) {
    // --- a > b -------------------------------------------------------------------------
    // c = a - b and d should be unset, since there was no arithmetic overflow
    test_execution(asm_op, &[1, 2], &[0, 1]);

    // --- a = b -------------------------------------------------------------------------
    // c = a - b and d should be unset, since there was no arithmetic overflow
    test_execution(asm_op, &[1, 1], &[0, 0]);

    // --- a < b -------------------------------------------------------------------------
    // c = a - b % 2^32 and d should be set, since there was arithmetic overflow
    test_execution(asm_op, &[2, 1], &[1, u32::MAX as u64]);

    // --- random u32 values: a >= b --------------------------------------------------------------
    let val1 = rand_value::<u64>() as u32;
    let val2 = rand_value::<u64>() as u32;
    let (a, b) = if val1 >= val2 {
        (val1, val2)
    } else {
        (val2, val1)
    };
    let c = a - b;
    test_execution(asm_op, &[b as u64, a as u64], &[0, c as u64]);

    // --- random u32 values: a < b ---------------------------------------------------------------
    let val1 = rand_value::<u64>() as u32;
    let val2 = rand_value::<u64>() as u32;
    let (a, b) = if val1 >= val2 {
        (val2, val1)
    } else {
        (val1, val2)
    };
    let (c, _) = a.overflowing_sub(b);
    let d = 1;
    test_execution(asm_op, &[b as u64, a as u64], &[d, c as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let e = rand_value::<u64>();
    test_execution(asm_op, &[b as u64, a as u64, e], &[d, c as u64, e]);
}

/// This helper function tests overflowing multiplication for two u32 inputs for a number of simple
/// cases as well as for random values. It checks that a result of (a * b) % 2^32 is pushed to the
/// stack, as well as a value of (a * b) / 2^32 indicating the number of times multiplication
/// overflowed. Finally, it ensures that the rest of the stack was unaffected.
fn test_mul_full(asm_op: &str) {
    // --- no overflow ----------------------------------------------------------------------------
    // c = a * b and d should be unset, since there was no arithmetic overflow
    test_execution(asm_op, &[1, 2], &[0, 2]);

    // --- overflow once --------------------------------------------------------------------------
    // c = a * b and d = 1, since it overflows once
    test_execution(asm_op, &[U32_BOUND / 2, 2], &[1, 0]);

    // --- multiple overflows ---------------------------------------------------------------------
    // c = a * b and d = 2, since it overflows twice
    test_execution(asm_op, &[U32_BOUND / 2, 4], &[2, 0]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let (c, overflow) = a.overflowing_mul(b);
    let d = if !overflow {
        0
    } else {
        (a as u64 * b as u64) / U32_BOUND
    };
    test_execution(asm_op, &[b as u64, a as u64], &[d, c as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let e = rand_value::<u64>();
    test_execution(asm_op, &[b as u64, a as u64, e], &[d, c as u64, e]);
}

/// This helper function tests multiply and add for three u32 inputs for a number of simple cases
/// as well as for random values. It checks that a result of (a * b + c) % 2^32 is pushed to the
/// stack, along with a value of (a * b + c) / 2^32 indicating the number of times the operation
/// overflowed. Finally, it ensures that the rest of the stack was unaffected.
fn test_madd(asm_op: &str) {
    // --- no overflow ----------------------------------------------------------------------------
    // d = a * b + c and e should be unset, since there was no arithmetic overflow
    test_execution(asm_op, &[0, 0, 1], &[0, 1]);
    test_execution(asm_op, &[2, 1, 3], &[0, 5]);

    // --- overflow once --------------------------------------------------------------------------
    // c = a * b and d = 1, since it overflows once
    test_execution(asm_op, &[U32_BOUND / 2, 2, 1], &[1, 1]);

    // --- multiple overflows ---------------------------------------------------------------------
    // c = a * b and d = 2, since it overflows twice
    test_execution(asm_op, &[U32_BOUND / 2, 4, 1], &[2, 1]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let c = rand_value::<u64>() as u32;
    let madd = a as u64 * b as u64 + c as u64;
    let d = madd % U32_BOUND;
    let e = madd / U32_BOUND;
    test_execution(asm_op, &[b as u64, a as u64, c as u64], &[e, d]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let f = rand_value::<u64>();
    test_execution(asm_op, &[b as u64, a as u64, c as u64, f], &[e, d, f]);
}

/// This helper function tests division with remainder for two u32 inputs for a number of simple
/// cases as well as for random values. It checks that the floor of a / b is pushed to the
/// stack, along with the remainder a % b. Finally, it ensures that the rest of the stack was
/// unaffected.
fn test_div_full(asm_op: &str) {
    // --- simple cases ---------------------------------------------------------------------------
    // division with no remainder
    test_execution(asm_op, &[1, 2], &[0, 2]);
    // division with remainder
    test_execution(asm_op, &[2, 1], &[1, 0]);
    test_execution(asm_op, &[2, 3], &[1, 1]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let quot = (a / b) as u64;
    let rem = (a % b) as u64;
    test_execution(asm_op, &[b as u64, a as u64], &[rem, quot]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let e = rand_value::<u64>();
    test_execution(asm_op, &[b as u64, a as u64, e], &[rem, quot, e]);
}

/// This helper function tests the modulus operation for two u32 inputs for a number of simple
/// cases as well as for random values. It checks that a % b is pushed to the stack. Finally, it
/// ensures that the rest of the stack was unaffected.
fn test_mod(asm_op: &str) {
    // --- simple cases ---------------------------------------------------------------------------
    test_execution(asm_op, &[5, 10], &[0]);
    test_execution(asm_op, &[5, 11], &[1]);
    test_execution(asm_op, &[11, 5], &[5]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let mut b = rand_value::<u64>() as u32;
    if b == 0 {
        // ensure we're not using a failure case
        b += 1;
    }
    let expected = a % b;
    test_execution(asm_op, &[b as u64, a as u64], &[expected as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_execution(asm_op, &[b as u64, a as u64, c], &[expected as u64, c]);
}

/// This helper function tests that the provided assembly comparison operation pushes the expected
/// value to the stack for each of the less than, equal to, or greater than comparisons tested.
fn test_comparison_op(asm_op: &str, expected_lt: u64, expected_eq: u64, expected_gt: u64) {
    // --- simple cases ---------------------------------------------------------------------------
    // a < b should put the expected value on the stack for the less-than case
    test_execution(asm_op, &[1, 0], &[expected_lt]);
    // a = b should put the expected value on the stack for the equal-to case
    test_execution(asm_op, &[0, 0], &[expected_eq]);
    // a > b should put the expected value on the stack for the greater-than case
    test_execution(asm_op, &[0, 1], &[expected_gt]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let expected = match a.cmp(&b) {
        Ordering::Less => expected_lt,
        Ordering::Equal => expected_eq,
        Ordering::Greater => expected_gt,
    };
    test_execution(asm_op, &[b as u64, a as u64], &[expected]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_execution(asm_op, &[b as u64, a as u64, c], &[expected, c]);
}

/// Tests a u32min assembly operation (u32min or u32min.unsafe) against a number of cases to ensure
/// that the operation puts the minimum of 2 input values on the stack.
fn test_min(asm_op: &str) {
    // --- simple cases ---------------------------------------------------------------------------
    // a < b should put a on the stack
    test_execution(asm_op, &[1, 0], &[0]);
    // a = b should put b on the stack
    test_execution(asm_op, &[0, 0], &[0]);
    // a > b should put b on the stack
    test_execution(asm_op, &[0, 1], &[0]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let expected = match a.cmp(&b) {
        Ordering::Less => a,
        Ordering::Equal => b,
        Ordering::Greater => b,
    };
    test_execution(asm_op, &[b as u64, a as u64], &[expected as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_execution(asm_op, &[b as u64, a as u64, c], &[expected as u64, c]);
}

/// Tests a u32max assembly operation (u32max or u32max.unsafe) against a number of cases to ensure
/// that the operation puts the maximum of 2 input values on the stack.
fn test_max(asm_op: &str) {
    // --- simple cases ---------------------------------------------------------------------------
    // a < b should put b on the stack
    test_execution(asm_op, &[1, 0], &[1]);
    // a = b should put b on the stack
    test_execution(asm_op, &[0, 0], &[0]);
    // a > b should put a on the stack
    test_execution(asm_op, &[0, 1], &[1]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let expected = match a.cmp(&b) {
        Ordering::Less => b,
        Ordering::Equal => b,
        Ordering::Greater => a,
    };
    test_execution(asm_op, &[b as u64, a as u64], &[expected as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_execution(asm_op, &[b as u64, a as u64, c], &[expected as u64, c]);
}
