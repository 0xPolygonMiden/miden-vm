use crate::{build_op_test, build_test};

#[test]
fn bitwise_and() {
    // Test all bit input combinations: (1, 1), (1, 0), (0, 0). Then test larger numbers.
    // the last drop at the end is added to make sure stack overflow table is empty at the end
    let asm_op = "u32checked_and push.0 u32checked_and push.0 u32checked_and push.65535 push.137 u32checked_and drop";
    let pub_inputs = vec![1, 1];

    build_op_test!(&asm_op, &pub_inputs).prove_and_verify(pub_inputs, 1, false);
}

#[test]
fn bitwise_or() {
    // Test all bit input combinations: (1, 1), (1, 0), (0, 0). Then test larger numbers.
    // the last drop at the end is added to make sure stack overflow table is empty at the end
    let asm_op = "u32checked_or push.0 u32checked_or not push.0 u32checked_or push.65535 push.137 u32checked_or drop";
    let pub_inputs = vec![1, 1];

    build_op_test!(&asm_op, &pub_inputs).prove_and_verify(pub_inputs, 1, false);
}

#[test]
fn bitwise_xor() {
    // Test all bit input combinations: (1, 1), (0, 0), (1, 0). Then test larger numbers
    // the last drop at the end is added to make sure stack overflow table is empty at the end
    let asm_op = "u32checked_xor push.0 u32checked_xor push.1 u32checked_xor push.65535 push.137 u32checked_xor drop";
    let pub_inputs = vec![1, 1];

    build_op_test!(&asm_op, &pub_inputs).prove_and_verify(pub_inputs, 1, false);
}

#[test]
fn pow2() {
    // Test powers of two significant to the construction: each power decomposed in the first row
    // of the pow2 trace; the first element of a  middle row; and the maximum exponent value.
    // the drop's at the end are added to make sure stack overflow table is empty at the end
    let source = "begin
        push.0 pow2
        push.1 pow2
        push.2 pow2
        push.3 pow2
        push.4 pow2
        push.5 pow2
        push.6 pow2
        push.7 pow2
        push.8 pow2
        push.9 pow2
        push.63 pow2
        drop drop drop drop drop drop drop drop drop drop drop
    end";
    let pub_inputs = vec![];

    build_test!(source, &pub_inputs).prove_and_verify(pub_inputs, 1, false);
}

#[test]
fn all_operations() {
    let source =
        "begin u32checked_and push.0 u32checked_or push.0 u32checked_xor push.9 pow2 drop end";
    let pub_inputs = vec![1, 1];

    build_test!(source, &pub_inputs).prove_and_verify(pub_inputs, 1, false);
}
