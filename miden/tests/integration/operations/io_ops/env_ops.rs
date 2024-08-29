use assembly::SourceManager;
use processor::FMP_MIN;
use test_utils::{build_op_test, build_test, StackInputs, Test, Word, MIN_STACK_DEPTH};
use vm_core::{
    mast::{MastForest, MastNode},
    Operation,
};

// SDEPTH INSTRUCTION
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
    let source = format!(
        "
    use.std::sys

    begin 
        push.1 
        push.1 
        {test_op} 
        
        exec.sys::truncate_stack 
    end
    "
    );
    let test = build_test!(&source, &[0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7]);
    test.expect_stack(&[18, 1, 1, 7, 6, 5, 4, 3, 2, 1, 0, 7, 6, 5, 4, 3]);
}

// LOCADDR INSTRUCTION
// ================================================================================================

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
            swapw dropw
        end";

    let test = build_test!(source, &[10]);
    test.expect_stack(&[FMP_MIN + 2, FMP_MIN + 1, 10]);

    // --- accessing mem via locaddr updates the correct variables --------------------------------
    let source = "
        proc.foo.2
            locaddr.0
            mem_store
            locaddr.1
            mem_storew
            dropw
            loc_load.0
            push.0.0.0.0
            loc_loadw.1
        end
        begin
            exec.foo
            swapdw dropw dropw
        end";

    let test = build_test!(source, &[10, 1, 2, 3, 4, 5]);
    test.expect_stack(&[4, 3, 2, 1, 5, 10]);

    // --- locaddr returns expected addresses in nested procedures --------------------------------
    let source = "
        use.std::sys

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

            exec.sys::truncate_stack
        end";

    let test = build_test!(source, &[10]);
    test.expect_stack(&[
        FMP_MIN + 3,
        FMP_MIN + 2,
        FMP_MIN + 1,
        FMP_MIN + 2,
        FMP_MIN + 5,
        FMP_MIN + 4,
        FMP_MIN + 3,
        FMP_MIN + 1,
        10,
    ]);

    // --- accessing mem via locaddr in nested procedures updates the correct variables -----------
    let source = "
        proc.foo.2
            locaddr.0
            mem_store
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
            loc_store.1
            exec.foo
            locaddr.1
            mem_load
            loc_load.0
        end
        begin
            exec.bar
            swapdw dropw dropw
        end";

    let test = build_test!(source, &[10, 1, 2, 3, 4, 5, 6, 7]);
    test.expect_stack(&[7, 6, 5, 4, 3, 2, 1, 10]);
}

// CALLER INSTRUCTION
// ================================================================================================

#[test]
fn caller() {
    let kernel_source = "
        export.foo
            caller
        end
    ";

    let program_source = "
        proc.bar
            syscall.foo
        end

        begin
            call.bar
        end";

    // TODO: update and use macro?
    let mut test = Test::new(&format!("test{}", line!()), program_source, false);
    test.stack_inputs = StackInputs::try_from_ints([1, 2, 3, 4, 5]).unwrap();
    test.kernel_source = Some(
        test.source_manager
            .load(&format!("kernel{}", line!()), kernel_source.to_string()),
    );

    // top 4 elements should be overwritten with the hash of `bar` procedure, but the 5th
    // element should remain untouched
    let bar_hash = build_bar_hash();
    test.expect_stack(&[bar_hash[3], bar_hash[2], bar_hash[1], bar_hash[0], 1]);

    test.prove_and_verify(vec![1, 2, 3, 4, 5], false);
}

fn build_bar_hash() -> [u64; 4] {
    let mut mast_forest = MastForest::new();

    let foo_root_id = mast_forest.add_block(vec![Operation::Caller], None).unwrap();

    let bar_root = MastNode::new_syscall(foo_root_id, &mast_forest).unwrap();
    let bar_hash: Word = bar_root.digest().into();
    [
        bar_hash[0].as_int(),
        bar_hash[1].as_int(),
        bar_hash[2].as_int(),
        bar_hash[3].as_int(),
    ]
}

// CLK INSTRUCTION
// ================================================================================================

#[test]
fn clk() {
    let test = build_op_test!("clk");
    test.expect_stack(&[2]);

    let source = "
        proc.foo
            push.5
            push.4
            clk
        end

        begin
            exec.foo
            swapw dropw
        end";

    let test = build_test!(source, &[]);
    test.expect_stack(&[3, 4, 5]);
}
