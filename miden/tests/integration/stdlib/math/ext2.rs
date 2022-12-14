use super::build_test;
use rand_utils::rand_value;
use vm_core::{Felt, FieldElement, QuadExtension, StarkField};

type ExtElement = QuadExtension<Felt>;

#[test]
fn mul_extension_2() {
    let source = "
    use.std::math::ext2
    begin
        exec.ext2::mul
    end";

    // --- simple case ----------------------------------------------------------------------------
    let test = build_test!(source, &[4, 3, 2, 1]);
    test.expect_stack(&[13, 2]);

    // --- random values --------------------------------------------------------------------------
    let a0 = Felt::new(rand_value::<u64>());
    let b0 = Felt::new(rand_value::<u64>());
    let a1 = Felt::new(rand_value::<u64>());
    let b1 = Felt::new(rand_value::<u64>());

    let a = ExtElement::new(a0, a1);
    let b = ExtElement::new(b0, b1);
    let arr = vec![a * b];
    let c = ExtElement::as_base_elements(&arr);

    let test = build_test!(source, &[b0.as_int(), b1.as_int(), a0.as_int(), a1.as_int()]);
    test.expect_stack(&[c[1].as_int(), c[0].as_int()]);
}

#[test]
fn mul_base() {
    let source = "
    use.std::math::ext2
    begin
        exec.ext2::mul_base
    end";

    // --- simple case ----------------------------------------------------------------------------
    let test = build_test!(source, &[4, 3, 2]);
    test.expect_stack(&[6, 8]);

    // --- random values --------------------------------------------------------------------------
    let a0 = Felt::new(rand_value::<u64>());
    let a1 = Felt::new(rand_value::<u64>());
    let x = Felt::new(rand_value::<u64>());

    let c0 = x * a0;
    let c1 = x * a1;

    let test = build_test!(source, &[a0.as_int(), a1.as_int(), x.as_int()]);
    test.expect_stack(&[c1.as_int(), c0.as_int()]);
}

#[test]
fn inv_extension_2() {
    let source = "
    use.std::math::ext2
    
    begin
        exec.ext2::inv
    end";

    let a0 = Felt::new(rand_value::<u64>() + 1u64);
    let a1 = Felt::new(rand_value::<u64>());

    let a = ExtElement::new(a0, a1);
    let a_inv = a.inv();

    let arr = vec![a_inv];
    let b = ExtElement::as_base_elements(&arr);

    let istack = [a0.as_int(), a1.as_int()];
    let ostack = [b[1].as_int(), b[0].as_int()];

    let test = build_test!(source, &istack);
    test.expect_stack(&ostack);
}
