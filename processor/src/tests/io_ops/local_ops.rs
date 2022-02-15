use super::{compile, test_memory_write, test_script_execution, test_script_execution_failure};

// PUSHING VALUES ONTO THE STACK (PUSH)
// ================================================================================================

#[test]
fn push_local() {
    // --- read from uninitialized memory ---------------------------------------------------------
    let script = compile(
        "
        proc.foo.1 
            push.local.0
        end 
        begin
            exec.foo
        end",
    );

    test_script_execution(&script, &[], &[0]);

    // --- the rest of the stack is unchanged -----------------------------------------------------
    test_script_execution(&script, &[1, 2, 3, 4], &[0, 4, 3, 2, 1]);
}

#[test]
fn pushw_local() {
    // --- read from uninitialized memory ---------------------------------------------------------
    let script = compile(
        "
        proc.foo.1 
            pushw.local.0
        end 
        begin
            exec.foo
        end",
    );

    test_script_execution(&script, &[], &[0, 0, 0, 0]);

    // --- the rest of the stack is unchanged -----------------------------------------------------
    test_script_execution(&script, &[1, 2, 3, 4], &[0, 0, 0, 0, 4, 3, 2, 1]);
}

// REMOVING VALUES FROM THE STACK (POP)
// ================================================================================================

#[test]
fn pop_local() {
    // --- test write to local memory -------------------------------------------------------------
    let script = compile(
        "
        proc.foo.2
            pop.local.0
            pop.local.1
            push.local.0
            push.local.1
        end
        begin
            exec.foo
        end",
    );

    test_script_execution(&script, &[1, 2, 3, 4], &[3, 4, 2, 1]);

    // --- test existing memory is not affected ---------------------------------------------------
    let script = compile(
        "
        proc.foo.1
            pop.local.0
        end
        begin
            pop.mem.0
            pop.mem.1
            exec.foo
        end",
    );
    let mem_addr = 1;

    test_memory_write(&script, &[1, 2, 3, 4], &[1], mem_addr, &[3, 0, 0, 0]);
}

#[test]
fn pop_local_invalid() {
    let script = compile(
        "
        proc.foo.1 
            pop.local.0
        end 
        begin
            exec.foo
        end",
    );

    // --- pop fails when stack is empty ----------------------------------------------------------
    test_script_execution_failure(&script, &[], "StackUnderflow");
}

#[test]
fn popw_local() {
    // --- test write to local memory -------------------------------------------------------------
    let script = compile(
        "
        proc.foo.2 
            popw.local.0
            popw.local.1
            pushw.local.0
            pushw.local.1
        end 
        begin
            exec.foo
        end",
    );

    test_script_execution(
        &script,
        &[1, 2, 3, 4, 5, 6, 7, 8],
        &[4, 3, 2, 1, 8, 7, 6, 5],
    );

    // --- test existing memory is not affected ---------------------------------------------------
    let script = compile(
        "
        proc.foo.1 
            popw.local.0
        end 
        begin
            popw.mem.0
            popw.mem.1
            exec.foo
        end",
    );
    let mem_addr = 1;

    test_memory_write(
        &script,
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        &[],
        mem_addr,
        &[5, 6, 7, 8],
    );
}

#[test]
fn popw_local_invalid() {
    let script = compile(
        "
        proc.foo.1 
            popw.local.0
        end 
        begin
            exec.foo
        end",
    );

    // --- pop fails when stack is empty ----------------------------------------------------------
    test_script_execution_failure(&script, &[1, 2], "StackUnderflow");
}

// OVERWRITING VALUES ON THE STACK (LOAD)
// ================================================================================================

#[test]
fn loadw_local() {
    // --- read from uninitialized memory ---------------------------------------------------------
    let script = compile(
        "
        proc.foo.1 
            loadw.local.0
        end 
        begin
            exec.foo
        end",
    );

    test_script_execution(&script, &[5, 6, 7, 8], &[0, 0, 0, 0]);

    // --- the rest of the stack is unchanged -----------------------------------------------------
    test_script_execution(
        &script,
        &[1, 2, 3, 4, 5, 6, 7, 8],
        &[0, 0, 0, 0, 4, 3, 2, 1],
    );
}

// SAVING STACK VALUES WITHOUT REMOVING THEM (STORE)
// ================================================================================================

#[test]
fn storew_local() {
    // --- test write to local memory -------------------------------------------------------------
    let script = compile(
        "
        proc.foo.2 
            storew.local.0
            swapw
            storew.local.1
            swapw
            pushw.local.0
            pushw.local.1
        end 
        begin
            exec.foo
        end",
    );

    test_script_execution(
        &script,
        &[1, 2, 3, 4, 5, 6, 7, 8],
        &[4, 3, 2, 1, 8, 7, 6, 5, 8, 7, 6, 5, 4, 3, 2, 1],
    );

    // --- test existing memory is not affected ---------------------------------------------------
    let script = compile(
        "
        proc.foo.1 
            storew.local.0
        end 
        begin
            popw.mem.0
            popw.mem.1
            exec.foo
        end",
    );
    let mem_addr = 1;

    test_memory_write(
        &script,
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        &[4, 3, 2, 1],
        mem_addr,
        &[5, 6, 7, 8],
    );
}

#[test]
fn storew_local_invalid() {
    let script = compile(
        "
        proc.foo.1 
            storew.local.0
        end 
        begin
            exec.foo
        end",
    );

    // --- pop fails when stack is empty ----------------------------------------------------------
    test_script_execution_failure(&script, &[1, 2], "StackUnderflow");
}

// NESTED PROCEDURES & PAIRED OPERATIONS (push/pop, pushw/popw, loadw/storew)
// ================================================================================================

#[test]
fn inverse_operations() {
    // --- pop and push are inverse operations, so the stack should be left unchanged -------------
    let script = compile(
        "
        proc.foo.1
            pop.local.0
            push.local.0
        end
        begin
            exec.foo
        end",
    );
    let inputs = [0, 1, 2, 3, 4];
    let mut final_stack = inputs;
    final_stack.reverse();

    test_script_execution(&script, &inputs, &final_stack);

    // --- popw and pushw are inverse operations, so the stack should be left unchanged -----------
    let script = compile(
        "
        proc.foo.1
            popw.local.0
            pushw.local.0
        end
        begin
            exec.foo
        end",
    );
    let inputs = [0, 1, 2, 3, 4];
    let mut final_stack = inputs;
    final_stack.reverse();

    test_script_execution(&script, &inputs, &final_stack);

    // --- storew and loadw are inverse operations, so the stack should be left unchanged ---------
    let script = compile(
        "
        proc.foo.1
            storew.local.0
            loadw.local.0
        end
        begin
            exec.foo
        end",
    );
    let inputs = [0, 1, 2, 3, 4];
    let mut final_stack = inputs;
    final_stack.reverse();

    test_script_execution(&script, &inputs, &final_stack);
}

#[test]
fn read_after_write() {
    // --- write to memory first, then test read with push --------------------------------------
    let script = compile(
        "
        proc.foo.1 
            storew.local.0
            push.local.0
        end 
        begin
            exec.foo
        end",
    );

    test_script_execution(&script, &[1, 2, 3, 4], &[1, 4, 3, 2, 1]);

    // --- write to memory first, then test read with pushw --------------------------------------
    let script = compile(
        "
        proc.foo.1 
            storew.local.0
            pushw.local.0
        end 
        begin
            exec.foo
        end",
    );

    test_script_execution(&script, &[1, 2, 3, 4], &[4, 3, 2, 1, 4, 3, 2, 1]);

    // --- write to memory first, then test read with loadw --------------------------------------
    let script = compile(
        "
        proc.foo.1 
            popw.local.0
            loadw.local.0
        end 
        begin
            exec.foo
        end",
    );

    test_script_execution(&script, &[1, 2, 3, 4, 5, 6, 7, 8], &[8, 7, 6, 5]);
}

#[test]
fn nested_procedures() {
    // --- test nested procedures - pop/push ------------------------------------------------------
    let script = compile(
        "
        proc.foo.1
            pop.local.0
        end
        proc.bar.1
            pop.local.0
            exec.foo
            push.local.0
        end
        begin
            exec.bar
        end",
    );
    let inputs = [0, 1, 2, 3];

    test_script_execution(&script, &inputs, &[3, 1, 0]);

    // --- test nested procedures - popw/pushw ----------------------------------------------------
    let script = compile(
        "
        proc.foo.1
            popw.local.0
        end
        proc.bar.1
            popw.local.0
            exec.foo
            pushw.local.0
        end
        begin
            exec.bar
        end",
    );
    let inputs = [0, 1, 2, 3, 4, 5, 6, 7];

    test_script_execution(&script, &inputs, &[7, 6, 5, 4]);

    // --- test nested procedures - storew/loadw --------------------------------------------------
    let script = compile(
        "
        proc.foo.1
            push.0 push.0
            storew.local.0
        end
        proc.bar.1
            storew.local.0
            exec.foo
            loadw.local.0
        end
        begin
            exec.bar
        end",
    );
    let inputs = [0, 1, 2, 3];

    test_script_execution(&script, &inputs, &[3, 2, 1, 0, 1, 0]);
}

#[test]
fn free_memory_pointer() {
    // ensure local procedure memory doesn't overwrite memory from outer scope
    let script = compile(
        "
        proc.bar.2
            pop.local.0
            pop.local.1
        end
        begin
            pop.mem.0
            pop.mem.1
            pop.mem.2
            pop.mem.3
            exec.bar
            push.mem.3
            push.mem.2
            push.mem.1
            push.mem.0
        end",
    );
    let inputs = [1, 2, 3, 4, 5, 6, 7];

    test_script_execution(&script, &inputs, &[7, 6, 5, 4, 1]);
}
