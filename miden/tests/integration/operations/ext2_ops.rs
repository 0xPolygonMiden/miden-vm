use crate::build_op_test;
use rand_utils::rand_value;
use vm_core::{Felt, FieldElement, QuadExtension, StarkField};

type Ext2Element = QuadExtension<Felt>;

// EXT2 OPS ASSERTIONS - MANUAL TESTS
// ================================================================================================

#[test]
fn ext2add() {
    let asm_op = "ext2add";

    let a = rand_value::<Ext2Element>();
    let b = rand_value::<Ext2Element>();
    let c = a + b;

    let (a0, a1) = ext_element_to_ints(a);
    let (b0, b1) = ext_element_to_ints(b);
    let (c0, c1) = ext_element_to_ints(c);

    let stack_init = [a0, a1, b0, b1];
    let expected = [c1, c0];

    let test = build_op_test!(asm_op, &stack_init);
    test.expect_stack(&expected);
}

#[test]
fn ext2sub() {
    let asm_op = "ext2sub";

    let a = rand_value::<Ext2Element>();
    let b = rand_value::<Ext2Element>();
    let c = a - b;

    let (a0, a1) = ext_element_to_ints(a);
    let (b0, b1) = ext_element_to_ints(b);
    let (c0, c1) = ext_element_to_ints(c);

    let stack_init = [a0, a1, b0, b1];
    let expected = [c1, c0];

    let test = build_op_test!(asm_op, &stack_init);
    test.expect_stack(&expected);
}

#[test]
fn ext2mul() {
    let asm_op = "ext2mul";

    let a = rand_value::<Ext2Element>();
    let b = rand_value::<Ext2Element>();
    let c = b * a;

    let (a0, a1) = ext_element_to_ints(a);
    let (b0, b1) = ext_element_to_ints(b);
    let (c0, c1) = ext_element_to_ints(c);

    let stack_init = [a0, a1, b0, b1];
    let expected = [c1, c0];

    let test = build_op_test!(asm_op, &stack_init);
    test.expect_stack(&expected);
}

#[test]
fn ext2div() {
    let asm_op = "ext2div";

    let a = rand_value::<Ext2Element>();
    let b = rand_value::<Ext2Element>();
    let c = a * b.inv();
    let (a0, a1) = ext_element_to_ints(a);
    let (b0, b1) = ext_element_to_ints(b);
    let (c0, c1) = ext_element_to_ints(c);

    let stack_init = [a0, a1, b0, b1];
    let expected = [c1, c0];

    let test = build_op_test!(asm_op, &stack_init);
    test.expect_stack(&expected);
}

#[test]
fn ext2neg() {
    let asm_op = "ext2neg";

    let a = rand_value::<Ext2Element>();
    let b = -a;
    let (a0, a1) = ext_element_to_ints(a);
    let (b0, b1) = ext_element_to_ints(b);

    let stack_init = [a0, a1];
    let expected = [b1, b0];

    let test = build_op_test!(asm_op, &stack_init);
    test.expect_stack(&expected);
}

#[test]
fn ext2inv() {
    let asm_op = "ext2inv";

    let a = rand_value::<Ext2Element>();
    let b = a.inv();

    let (a0, a1) = ext_element_to_ints(a);
    let (b0, b1) = ext_element_to_ints(b);

    let stack_init = [a0, a1];
    let expected = [b1, b0];

    let test = build_op_test!(asm_op, &stack_init);
    test.expect_stack(&expected);
}

// HELPER FUNCTIONS
// ================================================================================================
/// Helper function to convert a list of field elements into a list of elements in the underlying
/// base field and convert them into integers. Returns a tuple of integers.
fn ext_element_to_ints(ext_elem: Ext2Element) -> (u64, u64) {
    let ext_elem_arr = [ext_elem];
    let ext_elem_to_base_field = Ext2Element::as_base_elements(&ext_elem_arr);
    (ext_elem_to_base_field[0].as_int(), ext_elem_to_base_field[1].as_int())
}
