use crate::build_op_test;
use rand_utils::rand_value;
use vm_core::{Felt, FieldElement, QuadExtension, StarkField};

type ExtElement = QuadExtension<Felt>;

// EXT2 OPS ASSERTIONS - MANUAL TESTS
// ================================================================================================

#[test]
fn ext2add() {
    let asm_op = "ext2add";

    let a_ext = rand_value::<ExtElement>();
    let b_ext = rand_value::<ExtElement>();
    let a_ext_arr = [a_ext];
    let b_ext_arr = [b_ext];
    let c_ext_arr = vec![a_ext + b_ext];
    let a = ExtElement::as_base_elements(&a_ext_arr);
    let b = ExtElement::as_base_elements(&b_ext_arr);
    let c = ExtElement::as_base_elements(&c_ext_arr);

    let test =
        build_op_test!(asm_op, &[a[0].as_int(), a[1].as_int(), b[0].as_int(), b[1].as_int()]);
    test.expect_stack(&[c[1].as_int(), c[0].as_int()]);
}

#[test]
fn ext2sub() {
    let asm_op = "ext2sub";

    let a_ext = rand_value::<ExtElement>();
    let b_ext = rand_value::<ExtElement>();
    let a_ext_arr = [a_ext];
    let b_ext_arr = [b_ext];
    let c_ext_arr = vec![b_ext - a_ext];
    let a = ExtElement::as_base_elements(&a_ext_arr);
    let b = ExtElement::as_base_elements(&b_ext_arr);
    let c = ExtElement::as_base_elements(&c_ext_arr);

    let test =
        build_op_test!(asm_op, &[a[0].as_int(), a[1].as_int(), b[0].as_int(), b[1].as_int()]);
    test.expect_stack(&[c[1].as_int(), c[0].as_int()]);
}

#[test]
fn ext2mul() {
    let asm_op = "ext2mul";

    let a_ext = rand_value::<ExtElement>();
    let b_ext = rand_value::<ExtElement>();
    let a_ext_arr = [a_ext];
    let b_ext_arr = [b_ext];
    let c_ext_arr = vec![b_ext * a_ext];
    let a = ExtElement::as_base_elements(&a_ext_arr);
    let b = ExtElement::as_base_elements(&b_ext_arr);
    let c = ExtElement::as_base_elements(&c_ext_arr);

    let test =
        build_op_test!(asm_op, &[a[0].as_int(), a[1].as_int(), b[0].as_int(), b[1].as_int()]);
    test.expect_stack(&[c[1].as_int(), c[0].as_int()]);
}

#[test]
fn ext2div() {
    let asm_op = "ext2div";

    let a_ext = rand_value::<ExtElement>();
    let b_ext = rand_value::<ExtElement>();
    let b_inv = b_ext.inv();
    let a_ext_arr = [a_ext];
    let b_ext_arr = [b_ext];
    let c_ext_arr = vec![a_ext * b_inv];
    let a = ExtElement::as_base_elements(&a_ext_arr);
    let b = ExtElement::as_base_elements(&b_ext_arr);
    let c = ExtElement::as_base_elements(&c_ext_arr);

    let istack = [a[0].as_int(), a[1].as_int(), b[0].as_int(), b[1].as_int()];
    let ostack = [c[1].as_int(), c[0].as_int()];

    let test = build_op_test!(asm_op, &istack);
    test.expect_stack(&ostack);
}

#[test]
fn ext2neg() {
    let asm_op = "ext2neg";

    let a_ext = rand_value::<ExtElement>();
    let a_ext_arr = [a_ext];
    let b_ext_arr = vec![-a_ext];
    let a = ExtElement::as_base_elements(&a_ext_arr);
    let b = ExtElement::as_base_elements(&b_ext_arr);

    let test = build_op_test!(asm_op, &[a[0].as_int(), a[1].as_int()]);
    test.expect_stack(&[b[1].as_int(), b[0].as_int()]);
}

#[test]
fn ext2inv() {
    let asm_op = "ext2inv";

    let a_ext = rand_value::<ExtElement>();
    let a_ext_arr = [a_ext];
    let a_inv = a_ext.inv();

    let b_ext_arr = vec![a_inv];
    let a = ExtElement::as_base_elements(&a_ext_arr);
    let b = ExtElement::as_base_elements(&b_ext_arr);

    let istack = [a[0].as_int(), a[1].as_int()];
    let ostack = [b[1].as_int(), b[0].as_int()];

    let test = build_op_test!(asm_op, &istack);
    test.expect_stack(&ostack);
}
