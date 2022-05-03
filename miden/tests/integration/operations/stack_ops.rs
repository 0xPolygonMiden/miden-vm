use crate::build_op_test;
use rand_utils::rand_vector;

// STACK OPERATIONS TESTS
// ================================================================================================

#[test]
fn swapdw() {
    let asm_op = "swapdw";

    // --- simple case ----------------------------------------------------------------------------
    let test = build_op_test!(
        asm_op,
        &[16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
    );
    test.expect_stack(&[9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8]);

    // --- random values --------------------------------------------------------------------------
    let test_values = rand_vector::<u64>(16);

    let mut a = test_values[..8].to_vec();
    let mut b = test_values[8..].to_vec();
    a.reverse();
    b.reverse();

    a.append(&mut b);

    let test = build_op_test!(asm_op, test_values.as_slice());
    test.expect_stack(a.as_slice());
}
