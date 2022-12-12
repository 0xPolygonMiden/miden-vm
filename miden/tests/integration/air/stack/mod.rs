use crate::build_op_test;

mod field_ops;
mod stack_manipualtion_ops;

/// Test empty starting stack with no overflow outputs.
#[test]
fn empty_input() {
    let asm_op = "push.1 drop";
    let pub_inputs = vec![];

    build_op_test!(&asm_op, &pub_inputs).prove_and_verify(pub_inputs, false);
}

/// Test an empty starting stack but enough outputs that the overflow table is non-empty at the end.
#[test]
fn empty_input_overflow_output() {
    let asm_ops = "push.17 push.18";
    let pub_inputs = vec![];

    build_op_test!(&asm_ops, &pub_inputs).prove_and_verify(pub_inputs, false);
}

/// Test starting stack with some inputs but not full with no overflow outputs.
#[test]
fn some_inputs() {
    let asm_op = "push.5 drop";
    let pub_inputs = vec![1, 2, 3, 4];

    build_op_test!(&asm_op, &pub_inputs).prove_and_verify(pub_inputs, false);
}

/// Test full starting stack with no overflow outputs.
#[test]
fn full_inputs() {
    let asm_op = "push.17 drop";
    let pub_inputs = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    build_op_test!(&asm_op, &pub_inputs).prove_and_verify(pub_inputs, false);
}

/// Test a script that finishes with enough outputs that the overflow table is non-empty at the end.
#[test]
fn full_inputs_overflow_outputs() {
    let asm_ops = "push.17 push.18";
    let pub_inputs = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    build_op_test!(&asm_ops, &pub_inputs).prove_and_verify(pub_inputs, false);
}

/// Test a script initialized with enough inputs that the overflow table is non-empty at the start
/// but there's no overflow output at the end.
#[test]
fn overflow_inputs() {
    let asm_op = "push.19 drop";

    let pub_inputs = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18];

    build_op_test!(&asm_op, &pub_inputs).prove_and_verify(pub_inputs, false);
}

/// Test a script initialized with enough inputs that the overflow table is non-empty at the start
/// and at the end.
#[test]
fn overflow_inputs_overflow_outputs() {
    let asm_op = "push.19 push.20";
    let pub_inputs = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18];

    build_op_test!(&asm_op, &pub_inputs).prove_and_verify(pub_inputs, false);
}
