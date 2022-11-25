use crate::{build_op_test, build_test};
use vm_core::utils::ToElements;

#[test]
fn mem_load() {
    let asm_op = "mem_load.0 swap";

    build_op_test!(asm_op).prove_and_verify(vec![], false);
}

#[test]
fn mem_store() {
    let asm_op = "mem_store.0";
    let pub_inputs = vec![1];

    build_op_test!(asm_op, &pub_inputs).prove_and_verify(pub_inputs, false);
}

#[test]
fn helper_mem_store() {
    // Sequence of operations: [Span, Pad, MStoreW, Drop, Drop, Drop, Drop, Pad, Mstore, Drop, Pad, MStoreW, Drop, Pad, Mstore, Drop]
    let asm_op =
        "begin mem_storew.0 drop drop drop drop mem_store.0 mem_storew.0 drop mem_store.0 end";
    let pub_inputs = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let trace = build_test!(asm_op, &pub_inputs).execute().unwrap();
    // Since MStore only writes 1 element to memory, the 3 elements in the word at that location
    // that are not touched are placed in the helper registers.
    let helper_regs = [10, 9, 8, 0, 0, 0].to_elements();
    // We need to check helper registers state after the MStore operation at clock cycle 8.
    assert_eq!(helper_regs, trace.get_user_op_helpers_at(8));
    // After the second MStoreW call, the helper registers should be zero.
    let helper_regs = [0, 0, 0, 0, 0, 0].to_elements();
    assert_eq!(helper_regs, trace.get_user_op_helpers_at(11));

    // We need to check helper registers state after the MStore operation at clock cycle 14.
    let helper_regs = [5, 4, 3, 0, 0, 0].to_elements();
    assert_eq!(helper_regs, trace.get_user_op_helpers_at(14));
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
    let helper_regs = [1, 2, 3, 0, 0, 0].to_elements();
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
