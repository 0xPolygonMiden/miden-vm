use processor::ExecutionError;
use test_utils::{build_op_test, Felt, TestError};

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
    let test = build_op_test!(asm_op, &[0]);
    test.expect_error(TestError::ExecutionError(ExecutionError::FailedAssertion(
        1,
        Felt::new(123),
    )));
}

#[test]
fn assert_fail() {
    let asm_op = "assert";

    let test = build_op_test!(asm_op, &[2]);
    test.expect_error(TestError::ExecutionError(ExecutionError::FailedAssertion(1, Felt::new(0))));
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
    test.expect_error(TestError::ExecutionError(ExecutionError::FailedAssertion(2, Felt::new(0))));

    let test = build_op_test!(asm_op, &[1, 4]);
    test.expect_error(TestError::ExecutionError(ExecutionError::FailedAssertion(2, Felt::new(0))));
}
