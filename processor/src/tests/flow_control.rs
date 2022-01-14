use super::{super::execute, build_inputs, build_stack_state, compile};

// SIMPLE FLOW CONTROL TESTS
// ================================================================================================

#[test]
fn conditional_execution() {
    // --- if without else ------------------------------------------------------------------------
    let script = compile("begin dup.1 dup.1 eq if.true add end end");

    let inputs = build_inputs(&[2, 1]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    let expected_state = build_stack_state(&[2, 1]);
    assert_eq!(expected_state, last_state);

    let inputs = build_inputs(&[3, 3]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    let expected_state = build_stack_state(&[6]);
    assert_eq!(expected_state, last_state);

    // --- if with else ------------------------------------------------------------------------
    let script = compile("begin dup.1 dup.1 eq if.true add else mul end end");

    let inputs = build_inputs(&[3, 2]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    let expected_state = build_stack_state(&[6]);
    assert_eq!(expected_state, last_state);

    let inputs = build_inputs(&[3, 3]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    let expected_state = build_stack_state(&[6]);
    assert_eq!(expected_state, last_state);
}

#[test]
fn conditional_loop() {
    // --- entering the loop ----------------------------------------------------------------------
    // computes sum of values from 0 to the value at the top of the stack
    let script = compile(
        "
        begin
            dup push.0 movdn.2 neq.0
            while.true
                dup movup.2 add swap push.1 sub dup neq.0
            end
            drop
        end",
    );

    let inputs = build_inputs(&[10]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    let expected_state = build_stack_state(&[55]);
    assert_eq!(expected_state, last_state);

    // --- skipping the loop ----------------------------------------------------------------------
    let script = compile("begin dup eq.0 while.true add end end");

    let inputs = build_inputs(&[10]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    let expected_state = build_stack_state(&[10]);
    assert_eq!(expected_state, last_state);
}

#[test]
fn counter_controlled_loop() {
    // --- entering the loop ----------------------------------------------------------------------
    // compute 2^10
    let script = compile(
        "
        begin
            push.2
            push.1
            repeat.10
                dup.1 mul
            end
            swap drop
        end",
    );

    let inputs = build_inputs(&[]);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    let expected_state = build_stack_state(&[1024]);
    assert_eq!(expected_state, last_state);
}
