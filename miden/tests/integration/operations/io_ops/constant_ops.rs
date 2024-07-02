use super::build_op_test;

// PUSHING VALUES ONTO THE STACK (PUSH)
// ================================================================================================

#[test]
fn push_one() {
    let asm_op_base = "push";

    // --- test zero ------------------------------------------------------------------------------
    let asm_op = format!("{}.{}", asm_op_base, "0");
    let test = build_op_test!(&asm_op);
    test.expect_stack(&[0]);

    // --- single decimal input -------------------------------------------------------------------
    let asm_op = format!("{}.{}", asm_op_base, "5");
    let test = build_op_test!(&asm_op);
    test.expect_stack(&[5]);

    // --- single hexadecimal input ---------------------------------------------------------------
    let asm_op = format!("{}.{}", asm_op_base, "0xAF");
    let test = build_op_test!(&asm_op);
    test.expect_stack(&[175]);
}

#[test]
fn push_many() {
    let base_op = "push";

    // --- multiple values with separators --------------------------------------------------------
    let asm_op = format!("{base_op}.17.0x13.23");
    let test = build_op_test!(asm_op);
    test.expect_stack(&[23, 19, 17]);

    // --- push the maximum number of decimal values (16) -------------------------------------
    let asm_op = format!("{base_op}.16.17.18.19.20.21.22.23.24.25.26.27.28.29.30.31");
    let mut expected = Vec::with_capacity(16);
    for i in (16..32).rev() {
        expected.push(i);
    }

    let test = build_op_test!(asm_op);
    test.expect_stack(&expected);

    // --- push hexadecimal values with period separators between values ----------------------
    let asm_op = format!("{base_op}.0x0A.0x64.0x03E8.0x2710.0x0186A0");
    let mut expected = Vec::with_capacity(5);
    for i in (1..=5).rev() {
        expected.push(10_u64.pow(i));
    }

    let test = build_op_test!(asm_op);
    test.expect_stack(&expected);

    // --- push a mixture of decimal and single-element hexadecimal values --------------------
    let asm_op = format!("{base_op}.2.4.8.0x10.0x20.0x40.128.0x0100");
    let mut expected = Vec::with_capacity(8);
    for i in (1_u32..=8).rev() {
        expected.push(2_u64.pow(i));
    }

    let test = build_op_test!(asm_op);
    test.expect_stack(&expected);
}

#[test]
fn push_without_separator() {
    // --- push the maximum allowed number of hexadecimal values without separators (4) -----------
    let asm_op = "push.0x\
    0000000000000000\
    0100000000000000\
    0200000000000000\
    0300000000000000";
    let expected = vec![3, 2, 1, 0];

    let test = build_op_test!(asm_op);
    test.expect_stack(&expected);
}
