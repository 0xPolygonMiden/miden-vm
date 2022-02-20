use super::{compile, test_execution_failure, test_memory_write, test_script_execution};

// PUSHING VALUES ONTO THE STACK (PUSH)
// ================================================================================================

#[test]
fn push_mem() {
    let addr = 1;
    let asm_op = "push.mem";

    // --- read from uninitialized memory - address provided via the stack ------------------------
    let script = compile(format!("begin {} end", asm_op).as_str());
    test_script_execution(&script, &[addr], &[0]);

    // --- read from uninitialized memory - address provided as a parameter -----------------------
    let script = compile(format!("begin {}.{} end", asm_op, addr).as_str());
    test_script_execution(&script, &[], &[0]);

    // --- the rest of the stack is unchanged -----------------------------------------------------
    test_script_execution(&script, &[1, 2, 3, 4], &[0, 4, 3, 2, 1]);
}

#[test]
fn pushw_mem() {
    let addr = 1;
    let asm_op = "pushw.mem";

    // --- read from uninitialized memory - address provided via the stack ------------------------
    let script = compile(format!("begin {} end", asm_op).as_str());
    test_script_execution(&script, &[addr], &[0, 0, 0, 0]);

    // --- read from uninitialized memory - address provided as a parameter -----------------------
    let script = compile(format!("begin {}.{} end", asm_op, addr).as_str());
    test_script_execution(&script, &[], &[0, 0, 0, 0]);

    // --- the rest of the stack is unchanged -----------------------------------------------------
    test_script_execution(&script, &[1, 2, 3, 4], &[0, 0, 0, 0, 4, 3, 2, 1]);
}

// REMOVING VALUES FROM THE STACK (POP)
// ================================================================================================

#[test]
fn pop_mem() {
    let asm_op = "pop.mem";
    let addr = 0;

    // --- address provided via the stack ---------------------------------------------------------
    let script = compile(format!("begin {} end", asm_op).as_str());
    test_memory_write(
        &script,
        &[1, 2, 3, 4, addr],
        &[3, 2, 1],
        addr,
        &[4, 0, 0, 0],
    );

    // --- address provided as a parameter --------------------------------------------------------
    let script = compile(format!("begin {}.{} end", asm_op, addr).as_str());
    test_memory_write(&script, &[1, 2, 3, 4], &[3, 2, 1], addr, &[4, 0, 0, 0]);
}

#[test]
fn pop_mem_invalid() {
    let asm_op = "pop.mem.0";

    // --- pop fails when stack is empty ----------------------------------------------------------
    test_execution_failure(asm_op, &[], "StackUnderflow");
}

#[test]
fn popw_mem() {
    let asm_op = "popw.mem";
    let addr = 0;

    // --- address provided via the stack ---------------------------------------------------------
    let script = compile(format!("begin {} end", asm_op).as_str());
    test_memory_write(&script, &[1, 2, 3, 4, addr], &[], addr, &[1, 2, 3, 4]);

    // --- address provided as a parameter --------------------------------------------------------
    let script = compile(format!("begin {}.{} end", asm_op, addr).as_str());
    test_memory_write(&script, &[1, 2, 3, 4], &[], addr, &[1, 2, 3, 4]);

    // --- the rest of the stack is unchanged -----------------------------------------------------
    let script = compile(format!("begin {}.{} end", asm_op, addr).as_str());
    test_memory_write(&script, &[0, 1, 2, 3, 4], &[0], addr, &[1, 2, 3, 4]);
}

#[test]
fn popw_mem_invalid() {
    let asm_op = "popw.mem.0";

    // --- popw fails when the stack doesn't contain a full word ----------------------------------------------------------
    test_execution_failure(asm_op, &[1, 2], "StackUnderflow");
}

// OVERWRITING VALUES ON THE STACK (LOAD)
// ================================================================================================

#[test]
fn loadw_mem() {
    let addr = 1;
    let asm_op = "loadw.mem";

    // --- read from uninitialized memory - address provided via the stack ------------------------
    let script = compile(format!("begin {} end", asm_op).as_str());
    test_script_execution(&script, &[addr, 5, 6, 7, 8], &[0, 0, 0, 0]);

    // --- read from uninitialized memory - address provided as a parameter -----------------------
    let script = compile(format!("begin {}.{} end", asm_op, addr).as_str());
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
fn storew_mem() {
    let asm_op = "storew.mem";
    let addr = 0;

    // --- address provided via the stack ---------------------------------------------------------
    let script = compile(format!("begin {} end", asm_op).as_str());
    test_memory_write(
        &script,
        &[1, 2, 3, 4, addr],
        &[4, 3, 2, 1],
        addr,
        &[1, 2, 3, 4],
    );

    // --- address provided as a parameter --------------------------------------------------------
    let script = compile(format!("begin {}.{} end", asm_op, addr).as_str());
    test_memory_write(&script, &[1, 2, 3, 4], &[4, 3, 2, 1], addr, &[1, 2, 3, 4]);

    // --- the rest of the stack is unchanged -----------------------------------------------------
    let script = compile(format!("begin {}.{} end", asm_op, addr).as_str());
    test_memory_write(
        &script,
        &[0, 1, 2, 3, 4],
        &[4, 3, 2, 1, 0],
        addr,
        &[1, 2, 3, 4],
    );
}

// PAIRED OPERATIONS - ABSOLUTE MEMORY (push/pop, pushw/popw, loadw/storew)
// ================================================================================================

#[test]
fn inverse_operations() {
    // --- pop and push are inverse operations, so the stack should be left unchanged -------------
    let script = compile(
        "
        begin
            push.0
            pop.mem
            pop.mem.1
            push.1
            push.mem
            push.mem.0
        end",
    );
    let inputs = [0, 1, 2, 3, 4];
    let mut final_stack = inputs;
    final_stack.reverse();

    test_script_execution(&script, &inputs, &final_stack);

    // --- popw and pushw are inverse operations, so the stack should be left unchanged -----------
    let script = compile(
        "
        begin
            push.0
            popw.mem
            popw.mem.1
            push.1
            pushw.mem
            pushw.mem.0
        end",
    );
    let inputs = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let mut final_stack = inputs;
    final_stack.reverse();

    test_script_execution(&script, &inputs, &final_stack);

    // --- storew and loadw are inverse operations, so the stack should be left unchanged ---------
    let script = compile(
        "
        begin
            push.0
            storew.mem
            storew.mem.1
            push.1
            loadw.mem
            loadw.mem.0
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
    let script = compile("begin storew.mem.0 push.mem.0 end");
    test_script_execution(&script, &[1, 2, 3, 4], &[1, 4, 3, 2, 1]);

    // --- write to memory first, then test read with pushw --------------------------------------
    let script = compile("begin storew.mem.0 pushw.mem.0 end");
    test_script_execution(&script, &[1, 2, 3, 4], &[4, 3, 2, 1, 4, 3, 2, 1]);

    // --- write to memory first, then test read with loadw --------------------------------------
    let script = compile("begin popw.mem.0 loadw.mem.0 end");
    test_script_execution(&script, &[1, 2, 3, 4, 5, 6, 7, 8], &[8, 7, 6, 5]);
}
