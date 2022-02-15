use super::test_op_execution;

// PUSHING VALUES ONTO THE STACK (PUSH)
// ================================================================================================

#[test]
fn push_one() {
    let asm_op = "push";

    // --- test zero ------------------------------------------------------------------------------
    test_op_execution(format!("{}.{}", asm_op, "0").as_str(), &[], &[0]);

    // --- single decimal input -------------------------------------------------------------------
    test_op_execution(format!("{}.{}", asm_op, "5").as_str(), &[], &[5]);

    // --- single hexadecimal input ---------------------------------------------------------------
    test_op_execution(format!("{}.{}", asm_op, "0xAF").as_str(), &[], &[175]);
}

#[test]
fn push_many() {
    let asm_op = "push";

    // --- multiple values with separators --------------------------------------------------------
    test_op_execution(
        format!("{}.17.0x13.23", asm_op).as_str(),
        &[],
        &[23, 19, 17],
    );

    // --- push the maximum number of decimal values (16) -------------------------------------
    let mut expected = Vec::with_capacity(16);
    for i in (16..32).rev() {
        expected.push(i);
    }
    test_op_execution(
        format!("{}.16.17.18.19.20.21.22.23.24.25.26.27.28.29.30.31", asm_op).as_str(),
        &[],
        &expected,
    );

    // --- push hexadecimal values with period separators between values ----------------------
    let mut expected = Vec::with_capacity(5);
    for i in (1..=5).rev() {
        expected.push(10_u64.pow(i));
    }
    test_op_execution(
        format!("{}.0xA.0x64.0x3E8.0x2710.0x186A0", asm_op).as_str(),
        &[],
        &expected,
    );

    // --- push a mixture of decimal and single-element hexadecimal values --------------------
    let mut expected = Vec::with_capacity(8);
    for i in (1_u32..=8).rev() {
        expected.push(2_u64.pow(i));
    }
    test_op_execution(
        format!("{}.2.4.8.0x10.0x20.0x40.128.0x100", asm_op).as_str(),
        &[],
        &expected,
    );
}

#[test]
fn push_without_separator() {
    let asm_op = "push";

    // --- multiple values as a hexadecimal string ------------------------------------------------
    test_op_execution(
        format!("{}.0x0000000000004321000000000000dcba", asm_op).as_str(),
        &[],
        &[56506, 17185],
    );

    // --- push the maximum number of hexadecimal values without separators (16) ------------------
    let mut expected = Vec::with_capacity(16);
    for i in (0..16).rev() {
        expected.push(i);
    }
    test_op_execution(
        format!("{}.0x0000000000000000000000000000000100000000000000020000000000000003000000000000000400000000000000050000000000000006000000000000000700000000000000080000000000000009000000000000000A000000000000000B000000000000000C000000000000000D000000000000000E000000000000000F", asm_op).as_str(),
        &[],
        &expected,
    )
}
