use super::{
    super::{ExecutionTrace, Operation, Process},
    AuxTableTrace,
};
use vm_core::{
    bitwise::BITWISE_OR,
    hasher::{LINEAR_HASH, RETURN_STATE},
    Felt, FieldElement, ProgramInputs, AUX_TRACE_RANGE,
};

#[test]
fn hasher_aux_trace() {
    // --- single hasher permutation with no stack manipulation -----------------------------------
    let stack = [2, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0];
    let operations = vec![Operation::RpPerm];
    let aux_table_trace = build_trace(&stack, operations);
    let trace_len = aux_table_trace[0].len();

    let expected_len = 8;
    validate_hasher_trace(&aux_table_trace, 0, expected_len);

    // validate that the table was padded correctly (accounting for random row)
    validate_padding(&aux_table_trace, 8, trace_len - 1);
}

#[test]
fn bitwise_aux_trace() {
    // --- single bitwise operation with no stack manipulation ------------------------------------
    let stack = [4, 8];
    let operations = vec![Operation::U32or];
    let aux_table_trace = build_trace(&stack, operations);
    let trace_len = aux_table_trace[0].len();

    let expected_len = 8;
    validate_bitwise_trace(&aux_table_trace, 0, expected_len);

    // validate that the table was padded correctly (accounting for random row)
    validate_padding(&aux_table_trace, 8, trace_len - 1);
}

#[test]
fn memory_aux_trace() {
    // --- single memory operation with no stack manipulation -------------------------------------
    let stack = [1, 2, 3, 4];
    let operations = vec![Operation::Push(Felt::new(2)), Operation::StoreW];
    let aux_table_trace = build_trace(&stack, operations);
    let trace_len = aux_table_trace[0].len();
    let memory_trace_len = 1;

    // validate that the table was padded correctly (accounting for random row)
    let padding_end = trace_len - memory_trace_len - 1;
    validate_padding(&aux_table_trace, 0, padding_end);

    // check the memory trace
    validate_memory_trace(
        &aux_table_trace,
        padding_end,
        padding_end + memory_trace_len,
        2,
    );
}

#[test]
fn stacked_aux_trace() {
    // --- operations in hasher, bitwise, and memory processors without stack manipulation --------
    let stack = [8, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 1];
    let operations = vec![
        Operation::U32or,
        Operation::Push(Felt::ZERO),
        Operation::StoreW,
        Operation::RpPerm,
    ];
    let aux_table_trace = build_trace(&stack, operations);
    let trace_len = aux_table_trace[0].len();
    let memory_len = 1;

    // expect 8 rows of hasher trace
    let hasher_end = 8;
    validate_hasher_trace(&aux_table_trace, 0, hasher_end);

    // expect 8 rows of bitwise trace
    let bitwise_end = hasher_end + 8;
    validate_bitwise_trace(&aux_table_trace, hasher_end, bitwise_end);

    // validate that the table was padded correctly (accounting for random row)
    let padding_end = trace_len - memory_len - 1;
    validate_padding(&aux_table_trace, bitwise_end, padding_end);

    // expect 1 row of memory trace
    validate_memory_trace(&aux_table_trace, padding_end, padding_end + memory_len, 0);
}

// HELPER FUNCTIONS
// ================================================================================================

fn build_trace(stack: &[u64], operations: Vec<Operation>) -> AuxTableTrace {
    let inputs = ProgramInputs::new(stack, &[], vec![]).unwrap();
    let mut process = Process::new(inputs);

    for operation in operations.iter() {
        process.execute_op(*operation).unwrap();
    }

    let (trace, _) = ExecutionTrace::test_finalize_trace(process);
    trace[AUX_TRACE_RANGE]
        .to_vec()
        .try_into()
        .expect("failed to convert vector to array")
}

/// Validate the hasher trace output by the rpperm operation. The full hasher trace is tested in
/// the Hasher module, so this just tests the AuxTableTrace selectors and the initial columns
/// of the hasher trace.
fn validate_hasher_trace(aux_table: &AuxTableTrace, start: usize, end: usize) {
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
/// the Bitwise module, so this just tests the AuxTableTrace selectors, the initial columns
/// of the bitwise trace, and the final columns after the bitwise trace.
fn validate_bitwise_trace(aux_table: &AuxTableTrace, start: usize, end: usize) {
    // The selectors should match the bitwise selectors
    for row in start..end {
        // The selectors should match the selectors for the bitwise segment
        assert_eq!(Felt::ONE, aux_table[0][row]);
        assert_eq!(Felt::ZERO, aux_table[1][row]);

        // the expected start of the bitwise trace should hold the expected bitwise op selectors
        assert_eq!(BITWISE_OR, [aux_table[2][row], aux_table[3][row]]);

        // the final columns should be padded
        assert_eq!(Felt::ZERO, aux_table[16][row]);
        assert_eq!(Felt::ZERO, aux_table[17][row]);
    }
}

/// Validate the bitwise trace output by the storew operation. The full memory trace is tested in
/// the Memory module, so this just tests the AuxTableTrace selectors, the initial columns
/// of the memory trace, and the final column after the memory trace.
fn validate_memory_trace(aux_table: &AuxTableTrace, start: usize, end: usize, addr: u64) {
    for row in start..end {
        // The selectors in the first row should match the memory selectors
        assert_eq!(Felt::ONE, aux_table[0][row]);
        assert_eq!(Felt::ONE, aux_table[1][row]);
        assert_eq!(Felt::ONE, aux_table[2][row]);

        // the expected start of the memory trace should hold the memory ctx and addr
        assert_eq!(Felt::ZERO, aux_table[3][row]);
        assert_eq!(Felt::new(addr), aux_table[4][row]);

        // the final column should be padded
        assert_eq!(Felt::ZERO, aux_table[17][row]);
    }
}

/// Checks that the end of the auxiliary trace table is padded and has the correct selectors.
fn validate_padding(aux_table: &AuxTableTrace, start: usize, end: usize) {
    for row in start..end {
        // selectors
        assert_eq!(Felt::ONE, aux_table[0][row]);
        assert_eq!(Felt::ONE, aux_table[1][row]);
        assert_eq!(Felt::ZERO, aux_table[2][row]);

        // padding
        aux_table.iter().skip(3).for_each(|column| {
            assert_eq!(Felt::ZERO, column[row]);
        });
    }
}
