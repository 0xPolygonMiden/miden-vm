use std::cmp::Ordering;

use super::{
    super::StarkField, build_inputs, compile, execute, rand_word, test_execution,
    test_execution_failure, test_param_out_of_bounds, Felt,
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
#[ignore = "unimplemented"]
fn u32add() {
    unimplemented!();
}

#[test]
#[ignore = "unimplemented"]
fn u32add_b() {
    unimplemented!();
}

#[test]
#[ignore = "unimplemented"]
fn u32add_full() {
    unimplemented!();
}

#[test]
#[ignore = "unimplemented"]
fn u32add_unsafe() {
    unimplemented!();
}

#[test]
#[ignore = "unimplemented"]
fn u32addc() {
    unimplemented!();
}

#[test]
#[ignore = "unimplemented"]
fn u32sub() {
    unimplemented!();
}

#[test]
#[ignore = "unimplemented"]
fn u32mul() {
    unimplemented!();
}

#[test]
#[ignore = "unimplemented"]
fn u32madd() {
    unimplemented!();
}

#[test]
#[ignore = "unimplemented"]
fn u32div() {
    unimplemented!();
}

#[test]
#[ignore = "unimplemented"]
fn u32mod() {
    unimplemented!();
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
    let get_asm_op = |b: u64| format!("u32shl.{}", b);
    let get_result = |a, b| (a << b) % U32_BOUND;

    // --- test simple case -----------------------------------------------------------------------
    let a = 1_u64;
    let b = 1_u64;
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
    let a = rand_value::<u64>() as u64;
    test_execution(get_asm_op(0).as_str(), &[a], &[a]);

    // --- test random values ---------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() % 32;
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
    let get_asm_op = |b: u64| format!("u32shr.{}", b);
    let get_result = |a, b| a >> b;

    // --- test simple case -----------------------------------------------------------------------
    let a = 4_u64;
    let b = 2_u64;
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
    let a = rand_value::<u64>() as u64;
    test_execution(get_asm_op(0).as_str(), &[a], &[a]);

    // --- test random values ---------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() % 32;
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
    let a = rand_value::<u64>() as u64;
    test_execution(get_asm_op(0).as_str(), &[a], &[a]);

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
    let a = rand_value::<u64>() as u64;
    test_execution(get_asm_op(0).as_str(), &[a], &[a]);

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
    // issue: https://github.com/maticnetwork/miden/issues/71
    fn u32testw_proptest(word in rand_word()) {
        let mut values = Vec::new();

        // test each element in the word to see if the values are all valid u32 values
        let mut elements_are_u32 = true;
        for a in word.iter(){
            if *a % Felt::MODULUS >= U32_BOUND {
                elements_are_u32 = false;
            }
            // add the values to the vector
            values.push(*a);
        }

        let mut expected = values.clone();
        // add the expected result to get the expected state
        let expected_result = if elements_are_u32 { 1 } else { 0 };
        expected.insert(0, expected_result);

        test_execution("u32testw", &values, &expected);
    }

    #[test]
    fn u32assert_proptest(value in any::<u64>()) {
        // assertion passes and leaves the stack unchanged if a < 2^32
        let asm_op = "u32assert";
        if value < U32_BOUND {
            test_execution(asm_op, &[value], &[value]);
        } else {
            test_execution_failure(asm_op, &[value], "FailedAssertion");
        }
    }

    #[test]
    fn u32assertw_proptest(word in rand_word()) {
        // assertion passes and leaves the stack unchanged if a < 2^32
        let asm_op = "u32assertw";
        let mut values = Vec::new();

        // test each element in the word to see if the values are all valid u32 values
        let mut elements_are_u32 = true;
        for a in word.iter(){
            if *a % Felt::MODULUS >= U32_BOUND {
                elements_are_u32 = false;
            }
            // add the values to the vector
            values.push(*a);
        }

        if elements_are_u32 {
            test_execution(asm_op, &values, &values);
        } else {
            test_execution_failure(asm_op, &values, "FailedAssertion");
        }
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

// U32 OPERATIONS TESTS - RANDOMIZED - BITWISE OPERATIONS
// ================================================================================================

proptest! {
    #[test]
    // issue: https://github.com/maticnetwork/miden/issues/74
    fn u32and_proptest(a in any::<u64>(), b in any::<u64>()) {
        let asm_opcode = "u32and";
        let values = [b, a];

        if a >= U32_BOUND || b >= U32_BOUND {
            // should fail
            test_execution_failure(asm_opcode, &values, "FailedAssertion");
        } else {
            // should result in bitwise AND
            test_execution(asm_opcode, &values, &[a & b])
        }
    }

    #[test]
    // issue: https://github.com/maticnetwork/miden/issues/74
    fn u32or_proptest(a in any::<u64>(), b in any::<u64>()) {
        let asm_opcode = "u32or";
        let values = [b, a];

        if a >= U32_BOUND || b >= U32_BOUND {
            // should fail
            test_execution_failure(asm_opcode, &values, "FailedAssertion");
        } else {
            // should result in bitwise OR
            test_execution(asm_opcode, &values, &[a | b])
        }
    }

    #[test]
    // issue: https://github.com/maticnetwork/miden/issues/74
    fn u32xor_proptest(a in any::<u64>(), b in any::<u64>()) {
        let asm_opcode = "u32xor";
        let values = [b, a];

        if a >= U32_BOUND || b >= U32_BOUND {
            // should fail
            test_execution_failure(asm_opcode, &values, "FailedAssertion");
        } else {
            // should result in bitwise XOR
            test_execution(asm_opcode, &values, &[a ^ b])
        }
    }

    #[test]
    fn u32not_proptest(value in any::<u64>()) {
        let asm_opcode = "u32not";

        if value >= U32_BOUND {
            // should fail
            test_execution_failure(asm_opcode, &[value], "FailedAssertion");
        } else {
            // should result in bitwise NOT
            test_execution(asm_opcode, &[value], &[!value])
        }
    }

    #[test]
    // https://github.com/maticnetwork/miden/issues/76
    fn u32shl_proptest(a in any::<u64>(), b in 0_u32..32) {
        let asm_opcode = format!("u32shl.{}", b);

        if a >= U32_BOUND {
            // should fail
            test_execution_failure(&asm_opcode, &[a], "FailedAssertion");
        } else {
            // should execute left shift
            let result =  (a << b) % U32_BOUND;
            test_execution(&asm_opcode, &[a], &[result])
        }
    }

    #[test]
    // https://github.com/maticnetwork/miden/issues/76
    fn u32shr_proptest(a in any::<u64>(), b in 0_u32..32) {
        let asm_opcode = format!("u32shr.{}", b);

        if a >= U32_BOUND {
            // should fail
            test_execution_failure(&asm_opcode, &[a], "FailedAssertion");
        } else {
            // should execute right shift
            let result =  a >> b;
            test_execution(&asm_opcode, &[a], &[result])
        }
    }

    #[test]
    // https://github.com/maticnetwork/miden/issues/76
    fn u32rotl_proptest(a in any::<u64>(), b in 0_u32..32) {
        let op_base = "u32rotl";
        let asm_opcode = format!("{}.{}", op_base, b);

        if a >= U32_BOUND {
            // should fail
            test_execution_failure(&asm_opcode, &[a], "FailedAssertion");
        } else {
            // should execute left bit rotation
            test_execution(&asm_opcode, &[a], &[(a as u32).rotate_left(b) as u64])
        }
    }

    #[test]
    // https://github.com/maticnetwork/miden/issues/76
    fn u32rotr_proptest(a in any::<u64>(), b in 0_u32..32) {
        let op_base = "u32rotr";
        let asm_opcode = format!("{}.{}", op_base, b);

        if a >= U32_BOUND {
            // should fail
            test_execution_failure(&asm_opcode, &[a], "FailedAssertion");
        } else {
            // should execute right bit rotation
            test_execution(&asm_opcode, &[a], &[(a as u32).rotate_right(b) as u64])
        }
    }
}

// U32 OPERATIONS TESTS - RANDOMIZED - COMPARISON OPERATIONS
// ================================================================================================

proptest! {
    #[test]
    #[ignore = "unimplemented"]
    fn u32eq_proptest(a in any::<u64>(), b in any::<u64>()) {
        let asm_opcode = "u32and";
        let values = [b, a];

        if a >= U32_BOUND {
            // should fail when a is out of bounds
            test_execution_failure(asm_opcode, &values, "FailedAssertion");
        } else if b >= U32_BOUND {
            // should fail when b is out of bounds and provided as a parameter
            test_param_out_of_bounds(asm_opcode, b);
            // should fail when b is out of bounds and provided via the stack
            test_execution_failure(asm_opcode, &values, "FailedAssertion");
        } else {
            // should test for equality
            let expected = if a==b { 1 } else { 0 };
            // b provided via the stack
            test_execution(asm_opcode, &values, &[expected]);
            // b provided as a parameter
            test_execution(format!("{}.{}", asm_opcode, b).as_str(), &[a], &[expected]);
        }
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
    let values = vec![0_u64; input_count];

    for i in 0..input_count {
        let mut i_values = values.clone();
        // should execute successfully when the value of the input at index i is out of bounds
        i_values[i] = U32_BOUND;
        let inputs = build_inputs(&i_values);
        assert!(execute(&script, &inputs).is_ok());
    }
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
