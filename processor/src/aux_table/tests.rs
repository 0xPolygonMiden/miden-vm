use super::{
    super::{ExecutionTrace, Operation, Process},
    AuxTableTrace,
};
use vm_core::{
    bitwise::{BITWISE_OR, OP_CYCLE_LEN},
    hasher::{HASH_CYCLE_LEN, LINEAR_HASH, RETURN_STATE},
    program::blocks::CodeBlock,
    Felt, FieldElement, ProgramInputs, AUX_TRACE_RANGE,
};

#[test]
fn hasher_aux_trace() {
    // --- single hasher permutation with no stack manipulation -----------------------------------
    let stack = [2, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0];
    let operations = vec![Operation::RpPerm];
    let (aux_table_trace, trace_len) = build_trace(&stack, operations);

    // Skip the hash of the span block generated while building the trace to check only the RpPerm.
    let hasher_start = HASH_CYCLE_LEN;
    let hasher_end = hasher_start + HASH_CYCLE_LEN;
    validate_hasher_trace(&aux_table_trace, hasher_start, hasher_end);

    // Validate that the table was padded correctly.
    validate_padding(&aux_table_trace, hasher_end, trace_len);
}

#[test]
fn bitwise_aux_trace() {
    // --- single bitwise operation with no stack manipulation ------------------------------------
    let stack = [4, 8];
    let operations = vec![Operation::U32or];
    let (aux_table_trace, trace_len) = build_trace(&stack, operations);

    let bitwise_end = HASH_CYCLE_LEN + OP_CYCLE_LEN;
    validate_bitwise_trace(&aux_table_trace, HASH_CYCLE_LEN, bitwise_end);

    // Validate that the table was padded correctly.
    validate_padding(&aux_table_trace, bitwise_end, trace_len - 1);
}

#[test]
fn memory_aux_trace() {
    // --- single memory operation with no stack manipulation -------------------------------------
    let stack = [1, 2, 3, 4];
    let operations = vec![Operation::Push(Felt::new(2)), Operation::StoreW];
    let (aux_table_trace, trace_len) = build_trace(&stack, operations);
    let memory_trace_len = 1;

    // Validate that the table was padded correctly.
    let padding_end = trace_len - memory_trace_len;
    // Skip the hash cycle created by the span block when building the trace.
    validate_padding(&aux_table_trace, HASH_CYCLE_LEN, padding_end);

    // Check the memory trace.
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
    let (aux_table_trace, trace_len) = build_trace(&stack, operations);
    let memory_len = 1;

    // Skip the hash of the span block generated while building the trace to check only the RpPerm.
    let hasher_start = HASH_CYCLE_LEN;
    let hasher_end = hasher_start + HASH_CYCLE_LEN;
    validate_hasher_trace(&aux_table_trace, hasher_start, hasher_end);

    // Expect 1 operation cycle in the bitwise trace
    let bitwise_end = hasher_end + OP_CYCLE_LEN;
    validate_bitwise_trace(&aux_table_trace, hasher_end, bitwise_end);

    // Validate that the table was padded correctly.
    let padding_end = trace_len - memory_len;
    validate_padding(&aux_table_trace, bitwise_end, padding_end);

    // expect 1 row of memory trace
    validate_memory_trace(&aux_table_trace, padding_end, padding_end + memory_len, 0);
}

// HELPER FUNCTIONS
// ================================================================================================

/// Builds a sample trace by executing a span block containing the specified operations. This
/// results in 1 additional hash cycle at the beginning of the hasher coprocessor.
fn build_trace(stack: &[u64], operations: Vec<Operation>) -> (AuxTableTrace, usize) {
    let inputs = ProgramInputs::new(stack, &[], vec![]).unwrap();
    let mut process = Process::new(inputs);
    let program = CodeBlock::new_span(operations);
    process.execute_code_block(&program).unwrap();

    let (trace, _) = ExecutionTrace::test_finalize_trace(process);
    let trace_len = trace[0].len() - ExecutionTrace::NUM_RAND_ROWS;

    (
        trace[AUX_TRACE_RANGE]
            .to_vec()
            .try_into()
            .expect("failed to convert vector to array"),
        trace_len,
    )
}

/// Validate the hasher trace output by the rpperm operation. The full hasher trace is tested in
/// the Hasher module, so this just tests the AuxTableTrace selectors and the initial columns
/// of the hasher trace.
fn validate_hasher_trace(aux_table: &AuxTableTrace, start: usize, end: usize) {
    // The selectors should match the hasher selectors
    for row in start..end {
        // The selectors should match the selectors for the hasher segment
        assert_eq!(Felt::ZERO, aux_table[0][row]);

        match row % HASH_CYCLE_LEN {
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

/// Checks that the middle section of the auxiliary trace table before the memory coprocessor is
/// padded and has the correct selectors.
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
