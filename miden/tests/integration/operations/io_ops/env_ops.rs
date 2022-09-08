use super::{build_op_test, build_test};
use processor::FMP_MIN;
use vm_core::MIN_STACK_DEPTH;

// PUSHING VALUES ONTO THE STACK (PUSH)
// ================================================================================================

#[test]
fn sdepth() {
    let test_op = "sdepth";

    // --- empty stack ----------------------------------------------------------------------------
    let test = build_op_test!(test_op);
    test.expect_stack(&[MIN_STACK_DEPTH as u64]);

    // --- multi-element stack --------------------------------------------------------------------
    let test = build_op_test!(test_op, &[2, 4, 6, 8, 10]);
    test.expect_stack(&[MIN_STACK_DEPTH as u64, 10, 8, 6, 4, 2]);

    // --- overflowed stack -----------------------------------------------------------------------
    // push 2 values to increase the lenth of the stack beyond 16
    let source = format!("begin push.1 push.1 {} end", test_op);
    let test = build_test!(&source, &[0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7]);
    test.expect_stack(&[18, 1, 1, 7, 6, 5, 4, 3, 2, 1, 0, 7, 6, 5, 4, 3]);
}

#[test]
fn locaddr() {
    // --- locaddr returns expected address -------------------------------------------------------
    let source = "
        proc.foo.2
            locaddr.0
            locaddr.1
        end
        begin
            exec.foo
        end";

    let test = build_test!(source, &[10]);
    test.expect_stack(&[FMP_MIN + 1, FMP_MIN + 2, 10]);

    // --- accessing mem via locaddr updates the correct variables --------------------------------
    let source = "
        proc.foo.2
            locaddr.0
            mem_store
            drop
            locaddr.1
            mem_storew
            dropw
            loc_load.0
            push.0.0.0.0
            loc_loadw.1
        end
        begin
            exec.foo
        end";

    let test = build_test!(source, &[10, 1, 2, 3, 4, 5]);
    test.expect_stack(&[4, 3, 2, 1, 5, 10]);

    // --- locaddr returns expected addresses in nested procedures --------------------------------
    let source = "
        proc.foo.3
            locaddr.0
            locaddr.1
            locaddr.2
        end
        proc.bar.2
            locaddr.0
            exec.foo
            locaddr.1
        end
        begin
            exec.bar
            exec.foo
        end";

    let test = build_test!(source, &[10]);
    test.expect_stack(&[
        FMP_MIN + 1,
        FMP_MIN + 2,
        FMP_MIN + 3,
        FMP_MIN + 1,
        FMP_MIN + 3,
        FMP_MIN + 4,
        FMP_MIN + 5,
        FMP_MIN + 2,
        10,
    ]);

    // --- accessing mem via locaddr in nested procedures updates the correct variables -----------
    let source = "
        proc.foo.2
            locaddr.0
            mem_store
            drop
            locaddr.1
            mem_storew
            dropw
            push.0.0.0.0
            loc_loadw.1
            loc_load.0
        end
        proc.bar.2
            locaddr.0
            mem_store
            drop
            loc_store.1
            drop
            exec.foo
            locaddr.1
            mem_load
            loc_load.0
        end
        begin
            exec.bar
        end";

    let test = build_test!(source, &[10, 1, 2, 3, 4, 5, 6, 7]);
    test.expect_stack(&[7, 6, 5, 4, 3, 2, 1, 10]);
}
