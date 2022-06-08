use super::{
    build_op_test, test_input_out_of_bounds, test_param_out_of_bounds, TestError, U32_BOUND,
};
use proptest::prelude::*;
use rand_utils::rand_value;

// U32 OPERATIONS TESTS - MANUAL - BITWISE OPERATIONS
// ================================================================================================

#[test]
fn u32and() {
    let asm_op = "u32and";

    // --- simple cases ---------------------------------------------------------------------------
    let test = build_op_test!(asm_op, &[1, 1]);
    test.expect_stack(&[1]);

    let test = build_op_test!(asm_op, &[0, 1]);
    test.expect_stack(&[0]);

    let test = build_op_test!(asm_op, &[1, 0]);
    test.expect_stack(&[0]);

    let test = build_op_test!(asm_op, &[0, 0]);
    test.expect_stack(&[0]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[(a & b) as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>() as u32;
    let d = rand_value::<u64>() as u32;

    let test = build_op_test!(asm_op, &[c as u64, d as u64, a as u64, b as u64]);
    test.expect_stack(&[(a & b) as u64, d as u64, c as u64]);
}

#[test]
// issue: https://github.com/maticnetwork/miden/issues/74
fn u32and_fail() {
    let asm_op = "u32and";

    let test = build_op_test!(asm_op, &[U32_BOUND, 0]);
    test.expect_error(TestError::ExecutionError("NotU32Value"));

    let test = build_op_test!(asm_op, &[0, U32_BOUND]);
    test.expect_error(TestError::ExecutionError("NotU32Value"));
}

#[test]
fn u32or() {
    let asm_op = "u32or";

    // --- simple cases ---------------------------------------------------------------------------
    let test = build_op_test!(asm_op, &[1, 1]);
    test.expect_stack(&[1]);

    let test = build_op_test!(asm_op, &[0, 1]);
    test.expect_stack(&[1]);

    let test = build_op_test!(asm_op, &[1, 0]);
    test.expect_stack(&[1]);

    let test = build_op_test!(asm_op, &[0, 0]);
    test.expect_stack(&[0]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[(a | b) as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>() as u32;
    let d = rand_value::<u64>() as u32;

    let test = build_op_test!(asm_op, &[c as u64, d as u64, a as u64, b as u64]);
    test.expect_stack(&[(a | b) as u64, d as u64, c as u64]);
}

#[test]
// issue: https://github.com/maticnetwork/miden/issues/74
fn u32or_fail() {
    let asm_op = "u32or";

    let test = build_op_test!(asm_op, &[U32_BOUND, 0]);
    test.expect_error(TestError::ExecutionError("NotU32Value"));

    let test = build_op_test!(asm_op, &[0, U32_BOUND]);
    test.expect_error(TestError::ExecutionError("NotU32Value"));
}

#[test]
fn u32xor() {
    let asm_op = "u32xor";

    // --- simple cases ---------------------------------------------------------------------------
    let test = build_op_test!(asm_op, &[1, 1]);
    test.expect_stack(&[0]);

    let test = build_op_test!(asm_op, &[0, 1]);
    test.expect_stack(&[1]);

    let test = build_op_test!(asm_op, &[1, 0]);
    test.expect_stack(&[1]);

    let test = build_op_test!(asm_op, &[0, 0]);
    test.expect_stack(&[0]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[(a ^ b) as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>() as u32;
    let d = rand_value::<u64>() as u32;
    let test = build_op_test!(asm_op, &[c as u64, d as u64, a as u64, b as u64]);
    test.expect_stack(&[(a ^ b) as u64, d as u64, c as u64]);
}

#[test]
// issue: https://github.com/maticnetwork/miden/issues/74
fn u32xor_fail() {
    let asm_op = "u32xor";

    let test = build_op_test!(asm_op, &[U32_BOUND, 0]);
    test.expect_error(TestError::ExecutionError("NotU32Value"));

    let test = build_op_test!(asm_op, &[0, U32_BOUND]);
    test.expect_error(TestError::ExecutionError("NotU32Value"));
}

#[test]
fn u32not() {
    let asm_op = "u32not";

    // --- simple cases ---------------------------------------------------------------------------
    let test = build_op_test!(asm_op, &[U32_BOUND - 1]);
    test.expect_stack(&[0]);

    let test = build_op_test!(asm_op, &[0]);
    test.expect_stack(&[U32_BOUND - 1]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;

    let test = build_op_test!(asm_op, &[a as u64]);
    test.expect_stack(&[!a as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let b = rand_value::<u64>() as u32;

    let test = build_op_test!(asm_op, &[b as u64, a as u64]);
    test.expect_stack(&[!a as u64, b as u64]);
}

#[test]
fn u32not_fail() {
    let asm_op = "u32not";
    test_input_out_of_bounds(asm_op);
}

#[test]
fn u32shl() {
    // left shift: pops a from the stack and pushes (a * 2^b) mod 2^32 for a provided value b
    let asm_op = "u32shl";

    // --- test simple case -----------------------------------------------------------------------
    let a = 1_u32;
    let b = 1_u32;
    let test = build_op_test!(asm_op, &[5, a as u64, b as u64]);
    test.expect_stack(&[2, 5]);

    // --- test max values of a and b -------------------------------------------------------------
    let a = (U32_BOUND - 1) as u32;
    let b = 31;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[a.wrapping_shl(b) as u64]);

    // --- test b = 0 -----------------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = 0;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[a.wrapping_shl(b) as u64]);

    // --- test random values ---------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = (rand_value::<u64>() % 32) as u32;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[a.wrapping_shl(b) as u64]);
}

#[test]
fn u32shl_fail() {
    let asm_op = "u32shl";

    // should fail if a >= 2^32
    let test = build_op_test!(asm_op, &[U32_BOUND, 1]);
    test.expect_error(TestError::ExecutionError("NotU32Value"));

    // should fail if b >= 32
    let test = build_op_test!(asm_op, &[1, 32]);
    // if b >= 32, 2^b >= 2^32 or not a u32
    test.expect_error(TestError::ExecutionError("NotU32Value"));
}

#[test]
fn u32shl_b() {
    // left shift: pops a from the stack and pushes (a * 2^b) mod 2^32 for a provided value b
    let op_base = "u32shl";
    let get_asm_op = |b: u32| format!("{}.{}", op_base, b);

    // --- test simple case -----------------------------------------------------------------------
    let a = 1_u32;
    let b = 1_u32;
    let test = build_op_test!(get_asm_op(b).as_str(), &[5, a as u64]);
    test.expect_stack(&[2, 5]);

    // --- test max values of a and b -------------------------------------------------------------
    let a = (U32_BOUND - 1) as u32;
    let b = 31;

    let test = build_op_test!(get_asm_op(b).as_str(), &[a as u64]);
    test.expect_stack(&[a.wrapping_shl(b) as u64]);

    // --- test b = 0 -----------------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = 0;

    let test = build_op_test!(get_asm_op(b).as_str(), &[a as u64]);
    test.expect_stack(&[a.wrapping_shl(b) as u64]);

    // --- test random values ---------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = (rand_value::<u64>() % 32) as u32;

    let test = build_op_test!(get_asm_op(b).as_str(), &[a as u64]);
    test.expect_stack(&[a.wrapping_shl(b) as u64]);
}

#[test]
fn u32shl_b_fail() {
    let op_base = "u32shl";

    test_input_out_of_bounds(format!("{}.{}", op_base, 1).as_str());
    test_param_out_of_bounds(op_base, 32);
}

#[test]
fn u32shl_unsafe() {
    // left shift: pops a from the stack and pushes (a * 2^b) mod 2^32 for a provided value b
    let asm_op = "u32shl.unsafe";

    // --- test simple case -----------------------------------------------------------------------
    let a = 1_u32;
    let b = 1_u32;
    let test = build_op_test!(asm_op, &[5, a as u64, b as u64]);
    test.expect_stack(&[0, 2, 5]);

    // --- test max values of a and b -------------------------------------------------------------
    let a = (U32_BOUND - 1) as u32;
    let b = 31;
    let c = a.wrapping_shl(b);
    let d = a.wrapping_shr(32 - b);

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[d as u64, c as u64]);

    // --- test b = 0 -----------------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = 0;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[0, a as u64]);

    // --- test random values ---------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = (rand_value::<u64>() % 32) as u32;
    let c = a.wrapping_shl(b);
    let d = if b == 0 { 0 } else { a.wrapping_shr(32 - b) };

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[d as u64, c as u64]);

    // --- test out of bounds input (should not fail) --------------------------------------------
    let test = build_op_test!(asm_op, &[U32_BOUND, 1]);
    assert!(test.execute().is_ok());
}

#[test]
fn u32shr() {
    // right shift: pops a from the stack and pushes a / 2^b for a provided value b
    let asm_op = "u32shr";

    // --- test simple case -----------------------------------------------------------------------
    let a = 4_u32;
    let b = 2_u32;
    let test = build_op_test!(asm_op, &[5, a as u64, b as u64]);
    test.expect_stack(&[1, 5]);

    // --- test max values of a and b -------------------------------------------------------------
    let a = (U32_BOUND - 1) as u32;
    let b = 31;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[a.wrapping_shr(b) as u64]);

    // --- test b = 0 ---------------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = 0;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[a.wrapping_shr(b) as u64]);

    // --- test random values ---------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = (rand_value::<u64>() % 32) as u32;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[a.wrapping_shr(b) as u64]);
}

#[test]
fn u32shr_fail() {
    let asm_op = "u32shr";

    // should fail if a >= 2^32
    let test = build_op_test!(asm_op, &[U32_BOUND, 1]);
    test.expect_error(TestError::ExecutionError("NotU32Value"));

    // should fail if b >= 32
    let test = build_op_test!(asm_op, &[1, 32]);
    test.expect_error(TestError::ExecutionError("NotU32Value"));
}

#[test]
fn u32shr_b() {
    // right shift: pops a from the stack and pushes a / 2^b for a provided value b
    let op_base = "u32shr";
    let get_asm_op = |b: u32| format!("{}.{}", op_base, b);

    // --- test simple case -----------------------------------------------------------------------
    let a = 4_u32;
    let b = 2_u32;
    let test = build_op_test!(get_asm_op(b).as_str(), &[5, a as u64]);
    test.expect_stack(&[1, 5]);

    // --- test max values of a and b -------------------------------------------------------------
    let a = (U32_BOUND - 1) as u32;
    let b = 31;

    let test = build_op_test!(get_asm_op(b).as_str(), &[a as u64]);
    test.expect_stack(&[a.wrapping_shr(b) as u64]);

    // --- test b = 0 ---------------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = 0;

    let test = build_op_test!(get_asm_op(b).as_str(), &[a as u64]);
    test.expect_stack(&[a.wrapping_shr(b) as u64]);

    // --- test random values ---------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = (rand_value::<u64>() % 32) as u32;

    let test = build_op_test!(get_asm_op(b).as_str(), &[a as u64]);
    test.expect_stack(&[a.wrapping_shr(b) as u64]);
}

#[test]
fn u32shr_b_fail() {
    let op_base = "u32shr";

    test_input_out_of_bounds(format!("{}.{}", op_base, 1).as_str());
    test_param_out_of_bounds(op_base, 32);
}

#[test]
fn u32shr_unsafe() {
    // right shift: pops a from the stack and pushes a / 2^b for a provided value b
    let asm_op = "u32shr.unsafe";

    // --- test simple case -----------------------------------------------------------------------
    let a = 4_u32;
    let b = 2_u32;
    let test = build_op_test!(asm_op, &[5, a as u64, b as u64]);
    test.expect_stack(&[0, 1, 5]);

    // --- test max values of a and b -------------------------------------------------------------
    let a = (U32_BOUND - 1) as u32;
    let b = 31;
    let c = a.wrapping_shr(b);
    let d = a.wrapping_shl(32 - b);

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[d as u64, c as u64]);

    // --- test b = 0 ---------------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = 0;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[0, a as u64]);

    // --- test random values ---------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = (rand_value::<u64>() % 32) as u32;
    let c = a.wrapping_shr(b);
    let d = if b == 0 { 0 } else { a.wrapping_shl(32 - b) };

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[d as u64, c as u64]);

    // --- test out of bounds inputs (should not fail) --------------------------------------------
    let test = build_op_test!(asm_op, &[U32_BOUND, 1]);
    assert!(test.execute().is_ok());
}

#[test]
fn u32rotl() {
    // Computes c by rotating a 32-bit representation of a to the left by b bits.
    let asm_op = "u32rotl";

    // --- test simple case -----------------------------------------------------------------------
    let a = 1_u32;
    let b = 1_u32;
    let test = build_op_test!(asm_op, &[5, a as u64, b as u64]);
    test.expect_stack(&[2, 5]);

    // --- test simple wraparound case with large a -----------------------------------------------
    let a = (1_u64 << 31) as u32;
    let b: u32 = 1;
    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[1]);

    // --- test simple case wraparound case with max b --------------------------------------------
    let a = 2_u32;
    let b: u32 = 31;
    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[1]);

    // --- no change when a is max value (all 1s) -------------------------------------------------
    let a = (U32_BOUND - 1) as u32;
    let b = 2;
    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[a as u64]);

    // --- test b = 0 -----------------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = 0;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[a.rotate_left(b) as u64]);

    // --- test random values ---------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = (rand_value::<u64>() % 32) as u32;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[a.rotate_left(b) as u64]);
}

#[test]
fn u32rotl_b() {
    // Computes c by rotating a 32-bit representation of a to the left by b bits.
    let op_base = "u32rotl";
    let get_asm_op = |b: u32| format!("{}.{}", op_base, b);

    // --- test simple case -----------------------------------------------------------------------
    let a = 1_u32;
    let b = 1_u32;
    let test = build_op_test!(get_asm_op(b).as_str(), &[5, a as u64]);
    test.expect_stack(&[2, 5]);

    // --- test simple wraparound case with large a -----------------------------------------------
    let a = (1_u64 << 31) as u32;
    let b: u32 = 1;
    let test = build_op_test!(get_asm_op(b).as_str(), &[a as u64]);
    test.expect_stack(&[1]);

    // --- test simple case wraparound case with max b --------------------------------------------
    let a = 2_u32;
    let b: u32 = 31;
    let test = build_op_test!(get_asm_op(b).as_str(), &[a as u64]);
    test.expect_stack(&[1]);

    // --- no change when a is max value (all 1s) -------------------------------------------------
    let a = (U32_BOUND - 1) as u32;
    let b = 2;
    let test = build_op_test!(get_asm_op(b).as_str(), &[a as u64]);
    test.expect_stack(&[a as u64]);

    // --- test b = 0 ---------------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = 0;

    let test = build_op_test!(get_asm_op(b).as_str(), &[a as u64]);
    test.expect_stack(&[a.rotate_left(b) as u64]);

    // --- test random values ---------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = (rand_value::<u64>() % 32) as u32;

    let test = build_op_test!(get_asm_op(b).as_str(), &[a as u64]);
    test.expect_stack(&[a.rotate_left(b) as u64]);
}

#[test]
fn u32rotl_fail_b() {
    let op_base = "u32rotl";

    test_input_out_of_bounds(format!("{}.{}", op_base, 1).as_str());
    test_param_out_of_bounds(op_base, 32);
}

#[test]
fn u32rotl_unsafe() {
    // Computes c by rotating a 32-bit representation of a to the left by b bits.
    let asm_op = "u32rotl.unsafe";

    // --- test simple case -----------------------------------------------------------------------
    let a = 1_u32;
    let b = 1_u32;
    let test = build_op_test!(asm_op, &[5, a as u64, b as u64]);
    test.expect_stack(&[2, 5]);

    // --- test simple wraparound case with large a -----------------------------------------------
    let a = (1_u64 << 31) as u32;
    let b: u32 = 1;
    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[1]);

    // --- test simple case wraparound case with max b --------------------------------------------
    let a = 2_u32;
    let b: u32 = 31;
    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[1]);

    // --- no change when a is max value (all 1s) -------------------------------------------------
    let a = (U32_BOUND - 1) as u32;
    let b = 2;
    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[a as u64]);

    // --- test b = 0 -----------------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = 0;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[a.rotate_left(b) as u64]);

    // --- test random values ---------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = (rand_value::<u64>() % 32) as u32;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[a.rotate_left(b) as u64]);

    // --- test out of bounds inputs (should not fail) --------------------------------------------
    let test = build_op_test!(asm_op, &[U32_BOUND, 1]);
    assert!(test.execute().is_ok());
}

#[test]
fn u32rotr() {
    // Computes c by rotating a 32-bit representation of a to the right by b bits.
    let asm_op = "u32rotr";

    // --- test simple case -----------------------------------------------------------------------
    let a = 2_u32;
    let b = 1_u32;
    let test = build_op_test!(asm_op, &[5, a as u64, b as u64]);
    test.expect_stack(&[1, 5]);

    // --- test simple wraparound case with small a -----------------------------------------------
    let a = 1_u32;
    let b = 1_u32;
    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[U32_BOUND >> 1]);

    // --- test simple case wraparound case with max b --------------------------------------------
    let a = 1_u32;
    let b: u32 = 31;
    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[2]);

    // --- no change when a is max value (all 1s) -------------------------------------------------
    let a = (U32_BOUND - 1) as u32;
    let b = 2;
    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[a as u64]);

    // --- test b = 0 ---------------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = 0;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[a.rotate_right(b) as u64]);

    // --- test random values ---------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = (rand_value::<u64>() % 32) as u32;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[a.rotate_right(b) as u64]);
}

#[test]
fn u32rotr_fail() {
    let asm_op = "u32rotr";

    // should fail if a >= 2^32
    let test = build_op_test!(asm_op, &[U32_BOUND, 1]);
    test.expect_error(TestError::ExecutionError("NotU32Value"));

    // should fail if b >= 32
    let test = build_op_test!(asm_op, &[1, 32]);
    test.expect_error(TestError::ExecutionError("FailedAssertion"));
}

#[test]
fn u32rotr_b() {
    // Computes c by rotating a 32-bit representation of a to the right by b bits.
    let op_base = "u32rotr";
    let get_asm_op = |b: u32| format!("{}.{}", op_base, b);

    // --- test simple case -----------------------------------------------------------------------
    let a = 2_u32;
    let b = 1_u32;
    let test = build_op_test!(get_asm_op(b).as_str(), &[5, a as u64]);
    test.expect_stack(&[1, 5]);

    // --- test simple wraparound case with small a -----------------------------------------------
    let a = 1_u32;
    let b = 1_u32;
    let test = build_op_test!(get_asm_op(b).as_str(), &[a as u64]);
    test.expect_stack(&[U32_BOUND >> 1]);

    // --- test simple case wraparound case with max b --------------------------------------------
    let a = 1_u32;
    let b: u32 = 31;
    let test = build_op_test!(get_asm_op(b).as_str(), &[a as u64]);
    test.expect_stack(&[2]);

    // --- no change when a is max value (all 1s) -------------------------------------------------
    let a = (U32_BOUND - 1) as u32;
    let b = 2;
    let test = build_op_test!(get_asm_op(b).as_str(), &[a as u64]);
    test.expect_stack(&[a as u64]);

    // --- test b = 0 ---------------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = 0;

    let test = build_op_test!(get_asm_op(b).as_str(), &[a as u64]);
    test.expect_stack(&[a.rotate_right(b) as u64]);

    // --- test random values ---------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = (rand_value::<u64>() % 32) as u32;

    let test = build_op_test!(get_asm_op(b).as_str(), &[a as u64]);
    test.expect_stack(&[a.rotate_right(b) as u64]);
}

#[test]
fn u32rotr_b_fail() {
    let op_base = "u32rotr";

    test_input_out_of_bounds(format!("{}.{}", op_base, 1).as_str());
    test_param_out_of_bounds(op_base, 32);
}

#[test]
fn u32rotr_unsafe() {
    // Computes c by rotating a 32-bit representation of a to the right by b bits.
    let asm_op = "u32rotr.unsafe";

    // --- test simple case -----------------------------------------------------------------------
    let a = 2_u32;
    let b = 1_u32;
    let test = build_op_test!(asm_op, &[5, a as u64, b as u64]);
    test.expect_stack(&[1, 5]);

    // --- test simple wraparound case with small a -----------------------------------------------
    let a = 1_u32;
    let b = 1_u32;
    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[U32_BOUND >> 1]);

    // --- test simple case wraparound case with max b --------------------------------------------
    let a = 1_u32;
    let b: u32 = 31;
    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[2]);

    // --- no change when a is max value (all 1s) -------------------------------------------------
    let a = (U32_BOUND - 1) as u32;
    let b = 2;
    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[a as u64]);

    // --- test b = 0 ---------------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = 0;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[a.rotate_right(b) as u64]);

    // --- test random values ---------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = (rand_value::<u64>() % 32) as u32;

    let test = build_op_test!(asm_op, &[a as u64, b as u64]);
    test.expect_stack(&[a.rotate_right(b) as u64]);

    // --- test out of bounds inputs (should not fail) --------------------------------------------
    let test = build_op_test!(asm_op, &[U32_BOUND, 1]);
    assert!(test.execute().is_ok());
}

// U32 OPERATIONS TESTS - RANDOMIZED - BITWISE OPERATIONS
// ================================================================================================

proptest! {
    #[test]
    fn u32and_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_opcode = "u32and";
        let values = [a as u64, b as u64];
        // should result in bitwise AND
        let expected = (a & b) as u64;

        let test = build_op_test!(asm_opcode, &values);
        test.prop_expect_stack(&[expected])?;
    }

    #[test]
    fn u32or_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_opcode = "u32or";
        let values = [a as u64, b as u64];
        // should result in bitwise OR
        let expected = (a | b) as u64;

        let test = build_op_test!(asm_opcode, &values);
        test.prop_expect_stack(&[expected])?;
    }

    #[test]
    fn u32xor_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_opcode = "u32xor";
        let values = [a as u64, b as u64];
        // should result in bitwise XOR
        let expected = (a ^ b) as u64;

        let test = build_op_test!(asm_opcode, &values);
        test.prop_expect_stack(&[expected])?;
    }

    #[test]
    fn u32not_proptest(value in any::<u32>()) {
        let asm_opcode = "u32not";

        // should result in bitwise NOT
        let test = build_op_test!(asm_opcode, &[value as u64]);
        test.prop_expect_stack(&[!value as u64])?;
    }

    #[test]
    fn u32shl_proptest(a in any::<u32>(), b in 0_u32..32) {
        let asm_opcode = "u32shl";

        // should execute left shift
        let expected =  a << b;
        let test = build_op_test!(asm_opcode, &[a as u64, b as u64]);
        test.prop_expect_stack(&[expected as u64])?;
    }

    #[test]
    fn u32shl_b_proptest(a in any::<u32>(), b in 0_u32..32) {
        let asm_opcode = format!("u32shl.{}", b);

        // should execute left shift
        let expected =  a << b;
        let test = build_op_test!(asm_opcode, &[a as u64]);
        test.prop_expect_stack(&[expected as u64])?;
    }

    #[test]
    fn u32shl_unsafe_proptest(a in any::<u32>(), b in 0_u32..32) {
        let asm_opcode = "u32shl.unsafe";

        // should execute left shift
        let c =  a.wrapping_shl(b);
        // and leave the result that was shifted off
        let d = if b==0 { 0} else { a.wrapping_shr(32-b) };
        let test = build_op_test!(asm_opcode, &[a as u64, b as u64]);
        test.prop_expect_stack(&[d as u64, c as u64])?;
    }

    #[test]
    fn u32shr_proptest(a in any::<u32>(), b in 0_u32..32) {
        let asm_opcode = "u32shr";

        // should execute right shift
        let expected =  a >> b;
        let test = build_op_test!(asm_opcode, &[a as u64, b as u64]);
        test.prop_expect_stack(&[expected as u64])?;
    }

    #[test]
    fn u32shr_b_proptest(a in any::<u32>(), b in 0_u32..32) {
        let asm_opcode = format!("u32shr.{}", b);

        // should execute right shift
        let expected =  a >> b;
        let test = build_op_test!(asm_opcode, &[a as u64]);
        test.prop_expect_stack(&[expected as u64])?;
    }

    #[test]
    fn u32shr_unsafe_proptest(a in any::<u32>(), b in 0_u32..32) {
        let asm_opcode = "u32shr.unsafe";

        // should execute right shift
        let c =  a.wrapping_shr(b);
        // and leave the result that was shifted off
        let d = if b==0 { 0} else { a.wrapping_shl(32-b) };
        let test = build_op_test!(asm_opcode, &[a as u64, b as u64]);
        test.prop_expect_stack(&[d as u64, c as u64])?;
    }

    #[test]
    fn u32rotl_proptest(a in any::<u32>(), b in 0_u32..32) {
        let asm_opcode = "u32rotl";

        // should execute left bit rotation
        let test = build_op_test!(asm_opcode, &[a as u64, b as u64]);
        test.prop_expect_stack(&[a.rotate_left(b) as u64])?;
    }

    #[test]
    fn u32rotl_b_proptest(a in any::<u32>(), b in 0_u32..32) {
        let op_base = "u32rotl";
        let asm_opcode = format!("{}.{}", op_base, b);

        // should execute left bit rotation
        let test = build_op_test!(asm_opcode, &[a as u64]);
        test.prop_expect_stack(&[a.rotate_left(b) as u64])?;
    }

    #[test]
    fn u32rotl_unsafe_proptest(a in any::<u32>(), b in 0_u32..32) {
        let asm_opcode = "u32rotl.unsafe";

        // should execute left bit rotation
        let test = build_op_test!(asm_opcode, &[a as u64, b as u64]);
        test.prop_expect_stack(&[a.rotate_left(b) as u64])?;
    }

    #[test]
    fn u32rotr_proptest(a in any::<u32>(), b in 0_u32..32) {
        let asm_opcode = "u32rotr";

        // should execute right bit rotation
        let test = build_op_test!(asm_opcode, &[a as u64, b as u64]);
        test.prop_expect_stack(&[a.rotate_right(b) as u64])?;
    }

    #[test]
    fn u32rotr_b_proptest(a in any::<u32>(), b in 0_u32..32) {
        let op_base = "u32rotr";
        let asm_opcode = format!("{}.{}", op_base, b);

        // should execute right bit rotation
        let test = build_op_test!(asm_opcode, &[a as u64]);
        test.prop_expect_stack(&[a.rotate_right(b) as u64])?;
    }

    #[test]
    fn u32rotr_unsafe_proptest(a in any::<u32>(), b in 0_u32..32) {
        let asm_opcode = "u32rotr.unsafe";

        // should execute right bit rotation
        let test = build_op_test!(asm_opcode, &[a as u64, b as u64]);
        test.prop_expect_stack(&[a.rotate_right(b) as u64])?;
    }
}
