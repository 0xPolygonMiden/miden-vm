extern crate alloc;

use test_utils::build_test;

mod air;
mod cli;
mod exec_iters;
mod flow_control;
mod operations;

// TESTS
// ================================================================================================

#[test]
fn simple_program() {
    build_test!("begin push.1 push.2 add swap drop end").expect_stack(&[3]);
}

#[test]
fn multi_output_program() {
    let test = build_test!("begin mul movup.2 drop end", &[1, 2, 3]);
    test.prove_and_verify(vec![1, 2, 3], false);
}
