use super::test_op_execution;

// PUSHING VALUES ONTO THE STACK (PUSH)
// ================================================================================================

#[test]
fn push_env() {
    let test_op = "push.env.sdepth";

    // --- empty stack ----------------------------------------------------------------------------
    test_op_execution(test_op, &[], &[0]);

    // --- multi-element stack --------------------------------------------------------------------
    test_op_execution(test_op, &[2, 4, 6, 8, 10], &[5, 10, 8, 6, 4, 2]);

    // --- overflowed stack -----------------------------------------------------------------------
    // push 2 values to increase the lenth of the stack beyond 16
    let setup_ops = "push.1 push.1";
    test_op_execution(
        format!("{} {}", setup_ops, test_op).as_str(),
        &[0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7],
        &[18, 1, 1, 7, 6, 5, 4, 3, 2, 1, 0, 7, 6, 5, 4, 3],
    );
}
