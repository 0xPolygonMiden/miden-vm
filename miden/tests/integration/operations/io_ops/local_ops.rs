use super::build_test;

// PUSHING VALUES ONTO THE STACK (PUSH)
// ================================================================================================

#[test]
fn push_local() {
    let source = "
        proc.foo.1
            loc_load.0
        end
        begin
            exec.foo
        end";

    // --- 1 value is pushed & the rest of the stack is unchanged ---------------------------------
    let inputs = [1, 2, 3, 4];
    // In general, there is no guarantee that reading from uninitialized memory will result in ZEROs
    // but in this case since no other operations are executed, we do know it will push a ZERO.
    let final_stack = [0, 4, 3, 2, 1];

    build_test!(source, &inputs).expect_stack(&final_stack);
}

// REMOVING VALUES FROM THE STACK (POP)
// ================================================================================================

#[test]
fn pop_local() {
    // --- test write to local memory -------------------------------------------------------------
    let source = "
        proc.foo.2
            loc_store.0
            loc_store.1
            loc_load.0
            loc_load.1
        end
        begin
            exec.foo
        end";

    let test = build_test!(source, &[1, 2, 3, 4]);
    test.expect_stack(&[3, 4, 2, 1]);

    // --- test existing memory is not affected ---------------------------------------------------
    let source = "
        proc.foo.1
            loc_store.0
        end
        begin
            mem_store.0
            mem_store.1
            exec.foo
        end";
    let mem_addr = 1;

    let test = build_test!(source, &[1, 2, 3, 4]);
    test.expect_stack_and_memory(&[1], mem_addr, &[3, 0, 0, 0]);
}

// OVERWRITING VALUES ON THE STACK (LOAD)
// ================================================================================================

#[test]
fn loadw_local() {
    let source = "
        proc.foo.1
            loc_loadw.0
        end
        begin
            exec.foo
        end";

    // --- the top 4 values are overwritten & the rest of the stack is unchanged ------------------
    let inputs = [1, 2, 3, 4, 5, 6, 7, 8];
    // In general, there is no guarantee that reading from uninitialized memory will result in ZEROs
    // but in this case since no other operations are executed, we do know it will load ZEROs.
    let final_stack = [0, 0, 0, 0, 4, 3, 2, 1];

    build_test!(source, &inputs).expect_stack(&final_stack);
}

// SAVING STACK VALUES WITHOUT REMOVING THEM (STORE)
// ================================================================================================

#[test]
fn storew_local() {
    // --- test write to local memory -------------------------------------------------------------
    let source = "
        proc.foo.2
            loc_storew.0
            swapw
            loc_storew.1
            swapw
            push.0.0.0.0
            loc_loadw.0
            push.0.0.0.0
            loc_loadw.1
        end
        begin
            exec.foo
        end";

    let test = build_test!(source, &[1, 2, 3, 4, 5, 6, 7, 8]);
    test.expect_stack(&[4, 3, 2, 1, 8, 7, 6, 5, 8, 7, 6, 5, 4, 3, 2, 1]);

    // --- test existing memory is not affected ---------------------------------------------------
    let source = "
        proc.foo.1
            loc_storew.0
        end
        begin
            mem_storew.0
            dropw
            mem_storew.1
            dropw
            exec.foo
        end";
    let mem_addr = 1;

    let test = build_test!(source, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    test.expect_stack_and_memory(&[4, 3, 2, 1], mem_addr, &[5, 6, 7, 8]);
}

// NESTED PROCEDURES & PAIRED OPERATIONS (push/pop, pushw/popw, loadw/storew)
// ================================================================================================

#[test]
fn inverse_operations() {
    // --- pop and push are inverse operations, so the stack should be left unchanged -------------
    let source = "
        proc.foo.1
            loc_store.0
            loc_load.0
        end
        begin
            exec.foo
        end";
    let inputs = [0, 1, 2, 3, 4];
    let mut final_stack = inputs;
    final_stack.reverse();

    let test = build_test!(source, &inputs);
    test.expect_stack(&final_stack);

    // --- popw and pushw are inverse operations, so the stack should be left unchanged -----------
    let source = "
        proc.foo.1
            loc_storew.0
            dropw
            push.0.0.0.0
            loc_loadw.0
        end
        begin
            exec.foo
        end";
    let inputs = [0, 1, 2, 3, 4];
    let mut final_stack = inputs;
    final_stack.reverse();

    let test = build_test!(source, &inputs);
    test.expect_stack(&final_stack);

    // --- storew and loadw are inverse operations, so the stack should be left unchanged ---------
    let source = "
        proc.foo.1
            loc_storew.0
            loc_loadw.0
        end
        begin
            exec.foo
        end";
    let inputs = [0, 1, 2, 3, 4];
    let mut final_stack = inputs;
    final_stack.reverse();

    let test = build_test!(source, &inputs);
    test.expect_stack(&final_stack);
}

#[test]
fn read_after_write() {
    // --- write to memory first, then test read with push --------------------------------------
    let source = "
        proc.foo.1
            loc_storew.0
            loc_load.0
        end
        begin
            exec.foo
        end";

    let test = build_test!(source, &[1, 2, 3, 4]);
    test.expect_stack(&[1, 4, 3, 2, 1]);

    // --- write to memory first, then test read with pushw --------------------------------------
    let source = "
        proc.foo.1
            loc_storew.0
            push.0.0.0.0
            loc_loadw.0
        end
        begin
            exec.foo
        end";

    let test = build_test!(source, &[1, 2, 3, 4]);
    test.expect_stack(&[4, 3, 2, 1, 4, 3, 2, 1]);

    // --- write to memory first, then test read with loadw --------------------------------------
    let source = "
        proc.foo.1
            loc_storew.0
            dropw
            loc_loadw.0
        end
        begin
            exec.foo
        end";

    let test = build_test!(source, &[1, 2, 3, 4, 5, 6, 7, 8]);
    test.expect_stack(&[8, 7, 6, 5]);
}

#[test]
fn nested_procedures() {
    // --- test nested procedures - pop/push ------------------------------------------------------
    let source = "
        proc.foo.1
            loc_store.0
        end
        proc.bar.1
            loc_store.0
            exec.foo
            loc_load.0
        end
        begin
            exec.bar
        end";
    let inputs = [0, 1, 2, 3];

    let test = build_test!(source, &inputs);
    test.expect_stack(&[3, 1, 0]);

    // --- test nested procedures - popw/pushw ----------------------------------------------------
    let source = "
        proc.foo.1
            loc_storew.0
            dropw
        end
        proc.bar.1
            loc_storew.0
            dropw
            exec.foo
            push.0.0.0.0
            loc_loadw.0
        end
        begin
            exec.bar
        end";
    let inputs = [0, 1, 2, 3, 4, 5, 6, 7];

    let test = build_test!(source, &inputs);
    test.expect_stack(&[7, 6, 5, 4]);

    // --- test nested procedures - storew/loadw --------------------------------------------------
    let source = "
        proc.foo.1
            push.0 push.0
            loc_storew.0
        end
        proc.bar.1
            loc_storew.0
            exec.foo
            loc_loadw.0
        end
        begin
            exec.bar
        end";
    let inputs = [0, 1, 2, 3];

    let test = build_test!(source, &inputs);
    test.expect_stack(&[3, 2, 1, 0, 1, 0]);
}

#[test]
fn free_memory_pointer() {
    // ensure local procedure memory doesn't overwrite memory from outer scope
    let source = "
        proc.bar.2
            loc_store.0
            loc_store.1
        end
        begin
            mem_store.0
            mem_store.1
            mem_store.2
            mem_store.3
            exec.bar
            mem_load.3
            mem_load.2
            mem_load.1
            mem_load.0
        end";
    let inputs = [1, 2, 3, 4, 5, 6, 7];

    let test = build_test!(source, &inputs);
    test.expect_stack(&[7, 6, 5, 4, 1]);
}
