use super::{
    super::{
        bitwise::BITWISE_OR,
        build_op_test, build_test,
        hasher::{LINEAR_HASH, RETURN_STATE},
        AuxiliaryTableTrace, FieldElement,
    },
    Felt,
};

#[test]
fn trace_len() {
    // --- final trace lengths are equal when stack trace is longer than aux trace ----------------
    let test = build_op_test!("popw.mem.2", &[1, 2, 3, 4]);
    let trace = test.execute().unwrap();

    assert_eq!(trace.aux_table()[0].len(), trace.stack()[0].len());

    // --- final trace lengths are equal when aux trace is longer than stack trace ----------------
    let test = build_op_test!("u32and", &[4, 8]);
    let trace = test.execute().unwrap();

    assert_eq!(trace.aux_table()[0].len(), trace.stack()[0].len());

    // --- stack and aux trace lengths are equal after multi-processor aux trace ------------------
    let source = "begin u32and pop.mem.0 u32or end";
    let test = build_test!(source, &[1, 2, 3, 4]);
    let trace = test.execute().unwrap();

    assert_eq!(trace.aux_table()[0].len(), trace.stack()[0].len());

    // --- trace len is power of 2 after multi-processor aux trace --------------------------------
    assert!(trace.aux_table()[0].len().is_power_of_two());
}

#[test]
fn hasher_aux_trace() {
    // --- single hasher permutation with no stack manipulation -----------------------------------
    let test = build_op_test!("rpperm", &[2, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0]);
    let trace = test.execute().unwrap();
    let aux_table = trace.aux_table();

    let expected_len = 8;
    validate_hasher_trace(aux_table, 0, expected_len);

    // expect  no bitwise trace and no memory trace
    // so the trace only consists of the hasher trace with length 8
    assert_eq!(aux_table[0].len(), expected_len);
}

#[test]
fn bitwise_aux_trace() {
    // --- single bitwise operation with no stack manipulation ------------------------------------
    let test = build_op_test!("u32or", &[4, 8]);
    let trace = test.execute().unwrap();
    let aux_table = trace.aux_table();

    let expected_len = 8;
    validate_bitwise_trace(aux_table, 0, expected_len);

    // expect no hasher trace and no memory trace,
    // so the trace only consists of the bitwise trace
    assert_eq!(aux_table[0].len(), expected_len);
}

#[test]
fn memory_aux_trace() {
    // --- single memory operation with no stack manipulation -------------------------------------
    let test = build_op_test!("storew.mem.2", &[1, 2, 3, 4]);
    let trace = test.execute().unwrap();
    let aux_table = trace.aux_table();

    // the memory trace is only one row, so the length should match the stack trace length
    let expected_len = trace.stack()[0].len();

    // check the memory trace
    validate_memory_trace(aux_table, 0, 1, 2);

    // check that it was padded correctly
    validate_padding(aux_table, 1, expected_len);

    // expect no bitwise trace and no hasher trace,
    // so the trace consists of the memory trace and final section of padding
    assert_eq!(aux_table[0].len(), expected_len);
}

#[test]
fn stacked_aux_trace() {
    // --- operations in hasher, bitwise, and memory processors without stack manipulation --------
    let source = "begin u32or storew.mem.0 rpperm end";
    let test = build_test!(source, &[8, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 1]);
    let trace = test.execute().unwrap();
    let aux_table = trace.aux_table();

    // expect 8 rows of hasher trace
    validate_hasher_trace(aux_table, 0, 8);

    // expect 8 rows of bitwise trace
    validate_bitwise_trace(aux_table, 8, 16);

    // expect 1 row of memory trace
    validate_memory_trace(aux_table, 16, 17, 0);

    // expect 15 rows of padding, to pad to next power of 2
    validate_padding(aux_table, 17, 32);

    // expect aux table trace length to be 32
    assert_eq!(aux_table[0].len(), 32);

    // expect the stack trace to be the same
    assert_eq!(aux_table[0].len(), trace.stack()[0].len());
}

// HELPER FUNCTIONS
// ================================================================================================

/// Validate the hasher trace output by the rpperm operation. The full hasher trace is tested in
/// the Hasher module, so this just tests the AuxiliaryTableTrace selectors and the initial columns
/// of the hasher trace.
fn validate_hasher_trace(aux_table: &AuxiliaryTableTrace, start: usize, end: usize) {
    // The selectors should match the hasher selectors
    for row in start..end {
        // The selectors should match the selectors for the hasher segment
        assert_eq!(Felt::ZERO, aux_table[0][row]);

        match row {
            0 => {
                // in the first row, the expected start of the trace should hold the initial selectors
                assert_eq!(
                    LINEAR_HASH,
                    [aux_table[1][row], aux_table[2][row], aux_table[3][row]]
                );
            }
            7 => {
                // in the last row, the expected start of the trace should hold the final selectors
                assert_eq!(
                    RETURN_STATE,
                    [aux_table[1][row], aux_table[2][row], aux_table[3][row]]
                );
            }
            _ => {
                // in the other rows, the expected start of the trace should hold the mid selectors
                assert_eq!(
                    [Felt::ZERO, LINEAR_HASH[1], LINEAR_HASH[2]],
                    [aux_table[1][row], aux_table[2][row], aux_table[3][row]]
                );
            }
        }
    }
}

/// Validate the bitwise trace output by the u32or operation. The full bitwise trace is tested in
/// the Bitwise module, so this just tests the AuxiliaryTableTrace selectors, the initial columns
/// of the bitwise trace, and the final columns after the bitwise trace.
fn validate_bitwise_trace(aux_table: &AuxiliaryTableTrace, start: usize, end: usize) {
    // The selectors should match the bitwise selectors
    for row in start..end {
        // The selectors should match the selectors for the bitwise segment
        assert_eq!(Felt::ONE, aux_table[0][row]);
        assert_eq!(Felt::ZERO, aux_table[1][row]);

        // the expected start of the bitwise trace should hold the expected bitwise op selectors
        assert_eq!(BITWISE_OR, [aux_table[2][row], aux_table[3][row]]);

        // the final columns should be padded
        assert_eq!(Felt::ZERO, aux_table[15][row]);
        assert_eq!(Felt::ZERO, aux_table[16][row]);
        assert_eq!(Felt::ZERO, aux_table[17][row]);
    }
}

/// Validate the bitwise trace output by the storew operation. The full memory trace is tested in
/// the Memory module, so this just tests the AuxiliaryTableTrace selectors, the initial columns
/// of the memory trace, and the final column after the memory trace.
fn validate_memory_trace(aux_table: &AuxiliaryTableTrace, start: usize, end: usize, addr: u64) {
    for row in start..end {
        // The selectors in the first row should match the memory selectors
        assert_eq!(Felt::ONE, aux_table[0][row]);
        assert_eq!(Felt::ONE, aux_table[1][row]);
        assert_eq!(Felt::ZERO, aux_table[2][row]);

        // the expected start of the memory trace should hold the memory ctx and addr
        assert_eq!(Felt::ZERO, aux_table[3][row]);
        assert_eq!(Felt::new(addr), aux_table[4][row]);

        // the final column should be padded
        assert_eq!(Felt::ZERO, aux_table[17][row]);
    }
}

/// Checks that the end of the auxiliary trace table is padded and has the correct selectors.
fn validate_padding(aux_table: &AuxiliaryTableTrace, start: usize, end: usize) {
    for row in start..end {
        // selectors
        assert_eq!(Felt::ONE, aux_table[0][row]);
        assert_eq!(Felt::ONE, aux_table[1][row]);
        assert_eq!(Felt::ONE, aux_table[2][row]);

        // padding
        aux_table.iter().skip(3).for_each(|column| {
            assert_eq!(Felt::ZERO, column[row]);
        });
    }
}
