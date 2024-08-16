use processor::ExecutionError;
use test_utils::{build_op_test, expect_exec_error};

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
    expect_exec_error!(
        test,
        ExecutionError::FailedAssertion {
            clk: 2.into(),
            err_code: 123,
            err_msg: None,
        }
    );
}

#[test]
fn assert_fail() {
    let asm_op = "assert";

    let test = build_op_test!(asm_op, &[2]);
    expect_exec_error!(
        test,
        ExecutionError::FailedAssertion {
            clk: 2.into(),
            err_code: 0,
            err_msg: None,
        }
    );
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
    expect_exec_error!(
        test,
        ExecutionError::FailedAssertion {
            clk: 3.into(),
            err_code: 0,
            err_msg: None,
        }
    );

    let test = build_op_test!(asm_op, &[1, 4]);
    expect_exec_error!(
        test,
        ExecutionError::FailedAssertion {
            clk: 3.into(),
            err_code: 0,
            err_msg: None,
        }
    );
}
