use crate::{build_op_test, build_test};

#[test]
fn push() {
    // drop's are added at the end to make sure stack overflow is empty on exit
    let asm_op = "push.mem.0 swap drop";

    build_op_test!(asm_op).prove_and_verify(vec![], 0, false);
}

#[test]
fn pushw() {
    // drop's are added at the end to make sure stack overflow is empty on exit
    let asm_op = "pushw.mem.0 swapw drop drop drop drop";

    build_op_test!(asm_op).prove_and_verify(vec![], 0, false);
}

#[test]
fn pop() {
    let asm_op = "pop.mem.0";
    let pub_inputs = vec![1];

    build_op_test!(asm_op, &pub_inputs).prove_and_verify(pub_inputs, 0, false);
}

#[test]
fn popw() {
    let asm_op = "popw.mem.0";
    let pub_inputs = vec![1, 2, 3, 4];

    build_op_test!(asm_op, &pub_inputs).prove_and_verify(pub_inputs, 0, false);
}

#[test]
fn load() {
    let asm_op = "loadw.mem.0";

    build_op_test!(asm_op).prove_and_verify(vec![], 0, false);
}

#[test]
fn store() {
    let asm_op = "storew.mem.0";
    let pub_inputs = vec![1, 2, 3, 4];

    build_op_test!(asm_op, &pub_inputs).prove_and_verify(pub_inputs, 0, false);
}

#[test]
fn write_read() {
    // drop's are added at the end to make sure stack overflow is empty on exit
    let source = "begin popw.mem.0 pushw.mem.0 swapw drop drop drop drop end";
    let pub_inputs = vec![4, 3, 2, 1];

    build_test!(source, &pub_inputs).prove_and_verify(pub_inputs, 1, false);
}

#[test]
fn update() {
    // drop's are added at the end to make sure stack overflow is empty on exit
    let source = "begin pushw.mem.0 storew.mem.0 swapw drop drop drop drop end";
    let pub_inputs = vec![8, 7, 6, 5, 4, 3, 2, 1];

    build_test!(source, &pub_inputs).prove_and_verify(pub_inputs, 1, false);
}

#[test]
fn incr_write_addr() {
    let source = "begin storew.mem.0 storew.mem.1 end";
    let pub_inputs = vec![4, 3, 2, 1];

    build_test!(source, &pub_inputs).prove_and_verify(pub_inputs, 1, false);
}
