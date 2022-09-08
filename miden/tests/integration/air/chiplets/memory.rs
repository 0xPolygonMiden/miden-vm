use crate::{build_op_test, build_test};
use vm_core::utils::ToElements;

#[test]
fn mem_load() {
    let asm_op = "mem_load.0 swap";

    build_op_test!(asm_op).prove_and_verify(vec![], false);
}

#[test]
fn mem_store() {
    let asm_op = "mem_store.0 drop";
    let pub_inputs = vec![1];

    build_op_test!(asm_op, &pub_inputs).prove_and_verify(pub_inputs, false);
}

#[test]
fn helper_mem_store() {
    // Sequence of operations: [Span, Pad, MStore, Drop, Pad, Mstore, Drop, Pad, Mstore, Drop]
    let asm_op = "begin mem_store.0 drop mem_store.0 drop mem_store.0 drop end";
    let pub_inputs = vec![1, 2];

    let trace = build_test!(asm_op, &pub_inputs).execute().unwrap();
    // Since the MStore operation stores in helper registers the value that was previously in
    // memory, after the first call to MStore, the helper registers will be filled with zeros
    // (since memory is initialized with zeros by default). And an element taken from the
    // top of the stack, that is, 2, will be written to memory. On the second call, the memory will
    // be overwritten by 1, and the previous value that was in memory, that is, 2, will be written
    // to the helper registers.
    let helper_regs = [2, 0, 0, 0, 0, 0].to_elements();
    // We need to check helper registers state after second MStore, which index is 5
    assert_eq!(helper_regs, trace.get_user_op_helpers_at(5));
    // The next time the MStore operation is called, the memory will be overwritten again, and the
    // 1 lying there before that will be written to the helper register
    let helper_regs = [1, 0, 0, 0, 0, 0].to_elements();
    // We need to check helper registers state after third MStore, which index is 8
    assert_eq!(helper_regs, trace.get_user_op_helpers_at(8));
}

#[test]
fn mem_loadw() {
    let asm_op = "mem_loadw.0";

    build_op_test!(asm_op).prove_and_verify(vec![], false);
}

#[test]
fn mem_storew() {
    let asm_op = "mem_storew.0";
    let pub_inputs = vec![1, 2, 3, 4];

    build_op_test!(asm_op, &pub_inputs).prove_and_verify(pub_inputs, false);
}

#[test]
fn write_read() {
    let source = "begin mem_storew.0 mem_loadw.0 swapw end";

    let pub_inputs = vec![4, 3, 2, 1];

    build_test!(source, &pub_inputs).prove_and_verify(pub_inputs, false);
}

#[test]
fn helper_write_read() {
    // Sequence of operations: [Span, Pad, MStorew, Drop, Drop, Drop, Drop, Pad, MLoad, ... ]
    let source = "begin mem_storew.0 dropw mem_load.0 swapw end";
    let pub_inputs = vec![4, 3, 2, 1];

    let trace = build_test!(source, &pub_inputs).execute().unwrap();
    // When the MLoad operation is called, word elements that were not pushed on the stack
    // are written to helper registers. So, 3, 2 and 1 will be written after this operation
    let helper_regs = [3, 2, 1, 0, 0, 0].to_elements();
    // We need to check helper registers state after first MLoad, which index is 8
    assert_eq!(helper_regs, trace.get_user_op_helpers_at(8));
}

#[test]
fn update() {
    let source = "begin push.0.0.0.0 mem_loadw.0 mem_storew.0 swapw end";
    let pub_inputs = vec![8, 7, 6, 5, 4, 3, 2, 1];

    build_test!(source, &pub_inputs).prove_and_verify(pub_inputs, false);
}

#[test]
fn incr_write_addr() {
    let source = "begin mem_storew.0 mem_storew.1 end";
    let pub_inputs = vec![4, 3, 2, 1];

    build_test!(source, &pub_inputs).prove_and_verify(pub_inputs, false);
}
