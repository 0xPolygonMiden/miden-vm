use super::{compile, test_script_execution};

// SIMPLE FLOW CONTROL TESTS
// ================================================================================================

#[test]
fn conditional_execution() {
    // --- if without else ------------------------------------------------------------------------
    let script = compile("begin dup.1 dup.1 eq if.true add end end");

    test_script_execution(&script, &[1, 2], &[2, 1]);
    test_script_execution(&script, &[3, 3], &[6]);

    // --- if with else ------------------------------------------------------------------------
    let script = compile("begin dup.1 dup.1 eq if.true add else mul end end");

    test_script_execution(&script, &[2, 3], &[6]);
    test_script_execution(&script, &[3, 3], &[6]);
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

    test_script_execution(&script, &[10], &[55]);

    // --- skipping the loop ----------------------------------------------------------------------
    let script = compile("begin dup eq.0 while.true add end end");

    test_script_execution(&script, &[10], &[10]);
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

    test_script_execution(&script, &[], &[1024]);
}

// NESTED CONTROL FLOW
// ================================================================================================

#[test]
fn if_in_loop() {
    let script = compile(
        "
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
            end",
    );

    test_script_execution(&script, &[10], &[210]);
}

#[test]
fn if_in_loop_in_if() {
    let script = compile(
        "
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
            end",
    );

    test_script_execution(&script, &[10], &[210]);
    test_script_execution(&script, &[11], &[121]);
}
