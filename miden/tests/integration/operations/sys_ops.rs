use test_utils::{build_op_test, TestError};

// SYSTEM OPS ASSERTIONS - MANUAL TESTS
// ================================================================================================

#[test]
fn assert() {
    let asm_op = "assert";

    let test = build_op_test!(asm_op, &[1]);
    test.expect_stack(&[]);
}

#[test]
fn assert_with_code() {
    let asm_op = "assert.err=123";

    let test = build_op_test!(asm_op, &[1]);
    test.expect_stack(&[]);

    // triggered assertion captures both the VM cycle and error code
    let expected_err = "FailedAssertion { clk: 1, err_code: 123, err_msg: None }";
    let test = build_op_test!(asm_op, &[0]);
    test.expect_error(TestError::ExecutionError(&expected_err));
}

#[test]
fn assert_fail() {
    let asm_op = "assert";

    let test = build_op_test!(asm_op, &[2]);
    test.expect_error(TestError::ExecutionError("FailedAssertion"));
}

#[test]
fn assert_eq() {
    let asm_op = "assert_eq";

    let test = build_op_test!(asm_op, &[1, 1]);
    test.expect_stack(&[]);

    let test = build_op_test!(asm_op, &[3, 3]);
    test.expect_stack(&[]);
}

#[test]
fn assert_eq_fail() {
    let asm_op = "assert_eq";

    let test = build_op_test!(asm_op, &[2, 1]);
    test.expect_error(TestError::ExecutionError("FailedAssertion"));

    let test = build_op_test!(asm_op, &[1, 4]);
    test.expect_error(TestError::ExecutionError("FailedAssertion"));
}
