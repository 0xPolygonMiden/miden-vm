use crate::{
    build_test,
    helpers::{AdviceInputs, Test, TestError},
};
use vm_core::StackInputs;

// SIMPLE FLOW CONTROL TESTS
// ================================================================================================

#[test]
fn conditional_execution() {
    // --- if without else ------------------------------------------------------------------------
    let source = "begin dup.1 dup.1 eq if.true add end end";

    let test = build_test!(source, &[1, 2]);
    test.expect_stack(&[2, 1]);

    let test = build_test!(source, &[3, 3]);
    test.expect_stack(&[6]);

    // --- if with else ------------------------------------------------------------------------
    let source = "begin dup.1 dup.1 eq if.true add else mul end end";

    let test = build_test!(source, &[2, 3]);
    test.expect_stack(&[6]);

    let test = build_test!(source, &[3, 3]);
    test.expect_stack(&[6]);
}

#[test]
fn conditional_loop() {
    // --- entering the loop ----------------------------------------------------------------------
    // computes sum of values from 0 to the value at the top of the stack
    let source = "
        begin
            dup push.0 movdn.2 neq.0
            while.true
                dup movup.2 add swap push.1 sub dup neq.0
            end
            drop
        end";

    let test = build_test!(source, &[10]);
    test.expect_stack(&[55]);

    // --- skipping the loop ----------------------------------------------------------------------
    let source = "begin dup eq.0 while.true add end end";

    let test = build_test!(source, &[10]);
    test.expect_stack(&[10]);
}

#[test]
fn counter_controlled_loop() {
    // --- entering the loop ----------------------------------------------------------------------
    // compute 2^10
    let source = "
        begin
            push.2
            push.1
            repeat.10
                dup.1 mul
            end
            swap drop
        end";

    let test = build_test!(source);
    test.expect_stack(&[1024]);
}

// NESTED CONTROL FLOW
// ================================================================================================

#[test]
fn if_in_loop() {
    let source = "
        begin
            dup push.0 movdn.2 neq.0
            while.true
                dup movup.2 dup.1 eq.5
                if.true
                    mul
                else
                    add
                end
                swap push.1 sub dup neq.0
            end
            drop
        end";

    let test = build_test!(source, &[10]);
    test.expect_stack(&[210]);
}

#[test]
fn if_in_loop_in_if() {
    let source = "
        begin
            dup eq.10
            if.true
                dup push.0 movdn.2 neq.0
                while.true
                    dup movup.2 dup.1 eq.5
                    if.true
                        mul
                    else
                        add
                    end
                    swap push.1 sub dup neq.0
                end
                drop
            else
                dup mul
            end
        end";

    let test = build_test!(source, &[10]);
    test.expect_stack(&[210]);

    let test = build_test!(source, &[11]);
    test.expect_stack(&[121]);
}

// FUNCTION CALLS
// ================================================================================================

#[test]
fn local_fn_call() {
    // returning from a function with non-empty overflow table should result in an error
    let source = "
        proc.foo
            push.1
        end

        begin
            call.foo
        end";

    let expected_err = TestError::ExecutionError("InvalidStackDepthOnReturn(17)");
    build_test!(source, &[1, 2]).expect_error(expected_err);

    // dropping values from the stack in the current execution context should not affect values
    // in the overflow table from the parent execution context
    let source = "
        proc.foo
            repeat.20
                drop
            end
        end

        begin
            push.18
            call.foo
            repeat.16
                drop
            end
        end";

    let inputs = (1_u64..18).collect::<Vec<_>>();

    let test = build_test!(source, &inputs);
    test.expect_stack(&[2, 1]);

    test.prove_and_verify(inputs, false);
}

#[test]
fn local_fn_call_with_mem_access() {
    // foo should be executed in a different memory context; thus, when we read from memory after
    // calling foo, the value saved into memory[0] before calling foo should still be there.
    let source = "
        proc.foo
            mem_store.0
        end

        begin
            mem_store.0
            call.foo
            mem_load.0
            eq.7
        end";

    let test = build_test!(source, &[3, 7]);
    test.expect_stack(&[1]);

    test.prove_and_verify(vec![3, 7], false);
}

#[test]
fn simple_syscall() {
    let kernel_source = "
        export.foo
            add
        end
    ";

    let program_source = "
        begin
            syscall.foo
        end";

    // TODO: update and use macro?
    let test = Test {
        source: program_source.to_string(),
        kernel: Some(kernel_source.to_string()),
        stack_inputs: StackInputs::try_from_values([1, 2]).unwrap(),
        advice_inputs: AdviceInputs::default(),
        in_debug_mode: false,
    };
    test.expect_stack(&[3]);

    test.prove_and_verify(vec![1, 2], false);
}
