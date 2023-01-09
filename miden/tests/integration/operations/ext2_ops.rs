use crate::build_op_test;
use rand_utils::rand_value;
use vm_core::{Felt, FieldElement, QuadExtension, StarkField};

type ExtElement = QuadExtension<Felt>;

// EXT2 OPS ASSERTIONS - MANUAL TESTS
// ================================================================================================

#[test]
fn ext2add() {
    let asm_op = "ext2add";

    let a0 = Felt::new(rand_value::<u64>());
    let a1 = Felt::new(rand_value::<u64>());
    let b0 = Felt::new(rand_value::<u64>());
    let b1 = Felt::new(rand_value::<u64>());

    let a = ExtElement::new(a0, a1);
    let b = ExtElement::new(b0, b1);
    let arr = vec![a + b];
    let c = ExtElement::as_base_elements(&arr);

    let test = build_op_test!(asm_op, &[a0.as_int(), a1.as_int(), b0.as_int(), b1.as_int()]);
    test.expect_stack(&[c[1].as_int(), c[0].as_int()]);
}

#[test]
fn ext2sub() {
    let asm_op = "ext2sub";

    let a0 = Felt::new(rand_value::<u64>());
    let a1 = Felt::new(rand_value::<u64>());
    let b0 = Felt::new(rand_value::<u64>());
    let b1 = Felt::new(rand_value::<u64>());

    let a = ExtElement::new(a0, a1);
    let b = ExtElement::new(b0, b1);
    let arr = vec![b - a];
    let c = ExtElement::as_base_elements(&arr);

    let test = build_op_test!(asm_op, &[a0.as_int(), a1.as_int(), b0.as_int(), b1.as_int()]);
    test.expect_stack(&[c[1].as_int(), c[0].as_int()]);
}

#[test]
fn ext2neg() {
    let asm_op = "ext2neg";

    let a0 = Felt::new(rand_value::<u64>());
    let a1 = Felt::new(rand_value::<u64>());

    let a = ExtElement::new(a0, a1);
    let arr = vec![-a];
    let b = ExtElement::as_base_elements(&arr);

    let test = build_op_test!(asm_op, &[a0.as_int(), a1.as_int()]);
    test.expect_stack(&[b[1].as_int(), b[0].as_int()]);
}

#[test]
fn ext2mul() {
    let asm_op = "ext2mul";

    let a0 = Felt::new(rand_value::<u64>());
    let b0 = Felt::new(rand_value::<u64>());
    let a1 = Felt::new(rand_value::<u64>());
    let b1 = Felt::new(rand_value::<u64>());

    let a = ExtElement::new(a0, a1);
    let b = ExtElement::new(b0, b1);
    let arr = vec![a * b];
    let c = ExtElement::as_base_elements(&arr);

    let test = build_op_test!(asm_op, &[b0.as_int(), b1.as_int(), a0.as_int(), a1.as_int()]);
    test.expect_stack(&[c[1].as_int(), c[0].as_int()]);
}

#[test]
fn ext2div() {
    let asm_op = "ext2div";

    let a0 = Felt::new(rand_value::<u64>());
    let a1 = Felt::new(rand_value::<u64>());

    let b0 = Felt::new(rand_value::<u64>());
    let b1 = Felt::new(rand_value::<u64>());

    let a = ExtElement::new(a0, a1);
    let b = ExtElement::new(b0, b1);
    let b_inv = b.inv();
    let arr = vec![a * b_inv];

    let c = ExtElement::as_base_elements(&arr);

    let istack = [a0.as_int(), a1.as_int(), b0.as_int(), b1.as_int()];
    let ostack = [c[1].as_int(), c[0].as_int()];

    let test = build_op_test!(asm_op, &istack);
    test.expect_stack(&ostack);
}

#[test]
fn ext2inv() {
    let asm_op = "ext2inv";

    let a0 = Felt::new(rand_value::<u64>() + 1u64);
    let a1 = Felt::new(rand_value::<u64>());

    let a = ExtElement::new(a0, a1);
    let a_inv = a.inv();

    let arr = vec![a_inv];
    let b = ExtElement::as_base_elements(&arr);

    let istack = [a0.as_int(), a1.as_int()];
    let ostack = [b[1].as_int(), b[0].as_int()];

    let test = build_op_test!(asm_op, &istack);
    test.expect_stack(&ostack);
}
