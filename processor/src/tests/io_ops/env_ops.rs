use super::{compile, test_op_execution, test_script_execution};
use crate::system::FMP_MIN;

// PUSHING VALUES ONTO THE STACK (PUSH)
// ================================================================================================

#[test]
fn push_env_sdepth() {
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

#[test]
fn push_env_locaddr() {
    // --- locaddr returns expected address -------------------------------------------------------
    let script = compile(
        "
        proc.foo.2
            push.env.locaddr.0
            push.env.locaddr.1
        end
        begin
            exec.foo
        end",
    );
    test_script_execution(&script, &[10], &[FMP_MIN + 1, FMP_MIN + 2, 10]);

    // --- accessing mem via locaddr updates the correct variables --------------------------------
    let script = compile(
        "
        proc.foo.2
            push.env.locaddr.0
            pop.mem
            push.env.locaddr.1
            popw.mem
            push.local.0
            pushw.local.1
        end
        begin
            exec.foo
        end",
    );
    test_script_execution(&script, &[10, 1, 2, 3, 4, 5], &[4, 3, 2, 1, 5, 10]);

    // --- locaddr returns expected addresses in nested procedures --------------------------------
    let script = compile(
        "
        proc.foo.3
            push.env.locaddr.0
            push.env.locaddr.1
            push.env.locaddr.2
        end
        proc.bar.2
            push.env.locaddr.0
            exec.foo
            push.env.locaddr.1
        end
        begin
            exec.bar
            exec.foo
        end",
    );

    test_script_execution(
        &script,
        &[10],
        &[
            FMP_MIN + 1,
            FMP_MIN + 2,
            FMP_MIN + 3,
            FMP_MIN + 1,
            FMP_MIN + 3,
            FMP_MIN + 4,
            FMP_MIN + 5,
            FMP_MIN + 2,
            10,
        ],
    );

    // --- accessing mem via locaddr in nested procedures updates the correct variables -----------
    let script = compile(
        "
        proc.foo.2
            push.env.locaddr.0
            pop.mem
            push.env.locaddr.1
            popw.mem
            pushw.local.1
            push.local.0
        end
        proc.bar.2
            push.env.locaddr.0
            pop.mem
            pop.local.1
            exec.foo
            push.env.locaddr.1
            push.mem
            push.local.0
        end
        begin
            exec.bar
        end",
    );

    test_script_execution(
        &script,
        &[10, 1, 2, 3, 4, 5, 6, 7],
        &[7, 6, 5, 4, 3, 2, 1, 10],
    );
}
