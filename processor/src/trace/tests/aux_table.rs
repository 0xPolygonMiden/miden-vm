use super::{
    super::{Trace, NUM_RAND_ROWS},
    build_trace_from_ops, rand_array, ExecutionTrace, Felt, FieldElement, Operation, Word, ONE,
    ZERO,
};
use vm_core::{
    aux_table::memory::{
        ADDR_COL_IDX, CLK_COL_IDX, CTX_COL_IDX, NUM_ELEMENTS, U_COL_RANGE, V_COL_RANGE,
    },
    AUX_TABLE_AUX_TRACE_OFFSET, AUX_TRACE_RAND_ELEMENTS,
};

/// Tests the generation of the `b_aux` bus column when only memory lookups are included. It ensures
/// that trace generation is correct when all of the following are true.
///
/// - All possible memory operations are called by the stack.
/// - Some requests from the Stack and responses from Memory occur at the same cycle.
/// - Multiple memory addresses are used.
#[test]
#[allow(clippy::needless_range_loop)]
fn b_aux_trace_mem() {
    let stack = [1, 2, 3, 4, 0];
    let word = [ONE, Felt::new(2), Felt::new(3), Felt::new(4)];
    let operations = vec![
        Operation::MStoreW, // store [1, 2, 3, 4]
        Operation::Drop,    // clear the stack
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::MLoad,     // read the first value of the word
        Operation::MovDn5,    // put address 0 and space for a full word at top of stack
        Operation::MLoadW,    // load word from address 0 to stack
        Operation::Push(ONE), // push a new value onto the stack
        Operation::Push(ONE), // push a new address on to the stack
        Operation::MStore,    // store 1 at address 1
        Operation::Drop,      // ensure the stack overflow table is empty
    ];
    let mut trace = build_trace_from_ops(operations, &stack);

    let rand_elements = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let aux_columns = trace.build_aux_segment(&[], &rand_elements).unwrap();
    let b_aux = aux_columns.get_column(AUX_TABLE_AUX_TRACE_OFFSET);

    assert_eq!(trace.length(), b_aux.len());
    assert_eq!(ONE, b_aux[0]);
    assert_eq!(ONE, b_aux[1]);

    // The first memory request from the stack is sent when the `MStoreW` operation is executed, at
    // cycle 1, so the request is included in the next row. (The trace begins by executing `span`).
    let value = build_expected_memory(&rand_elements, ZERO, ZERO, Felt::new(1), [ZERO; 4], word);
    let mut expected = value.inv();
    assert_eq!(expected, b_aux[2]);

    // Nothing changes after user operations that don't make requests to the Auxiliary Table.
    for row in 3..7 {
        assert_eq!(expected, b_aux[row]);
    }

    // The next memory request from the stack is sent when `MLoad` is executed at cycle 6 and
    // included at row 7
    let value = build_expected_memory(&rand_elements, ZERO, ZERO, Felt::new(6), word, word);
    expected *= value.inv();
    assert_eq!(expected, b_aux[7]);

    // Nothing changes in row 8.
    assert_eq!(expected, b_aux[8]);

    // Memory responses will be provided during the memory segment of the Auxiliary Table trace,
    // which starts after the hash for the span block at row 8. There will be 4 rows, corresponding
    // to the four Memory operations.

    // At cycle 8 `MLoadW` is requested by the stack and `MStoreW` is provided by memory
    let value = build_expected_memory(&rand_elements, ZERO, ZERO, Felt::new(8), word, word);
    expected *= value.inv();
    expected *= build_expected_memory_from_trace(&trace, &rand_elements, 8);
    assert_eq!(expected, b_aux[9]);

    // At cycle 9, `MLoad` is provided by memory.
    expected *= build_expected_memory_from_trace(&trace, &rand_elements, 9);
    assert_eq!(expected, b_aux[10]);

    // At cycle 10,  `MLoadW` is provided by memory.
    expected *= build_expected_memory_from_trace(&trace, &rand_elements, 10);
    assert_eq!(expected, b_aux[11]);

    // At cycle 11, `MStore` is requested by the stack and provided by memory.
    let value = build_expected_memory(
        &rand_elements,
        ZERO,
        ONE,
        Felt::new(11),
        [ZERO; 4],
        [ONE, ZERO, ZERO, ZERO],
    );
    expected *= value.inv();
    expected *= build_expected_memory_from_trace(&trace, &rand_elements, 11);
    assert_eq!(expected, b_aux[12]);

    // The value in b_aux should be ONE now and for the rest of the trace.
    for row in 12..trace.length() - NUM_RAND_ROWS {
        assert_eq!(ONE, b_aux[row]);
    }
}

// TEST HELPERS
// ================================================================================================

fn build_expected_memory(
    alphas: &[Felt],
    ctx: Felt,
    addr: Felt,
    clk: Felt,
    old_word: Word,
    new_word: Word,
) -> Felt {
    let mut old_word_value = ZERO;
    let mut new_word_value = ZERO;

    for i in 0..NUM_ELEMENTS {
        old_word_value += alphas[i + 4] * old_word[i];
        new_word_value += alphas[i + 8] * new_word[i];
    }

    alphas[0]
        + alphas[1] * ctx
        + alphas[2] * addr
        + alphas[3] * clk
        + old_word_value
        + new_word_value
}

fn build_expected_memory_from_trace(trace: &ExecutionTrace, alphas: &[Felt], row: usize) -> Felt {
    let ctx = trace.main_trace.get_column(CTX_COL_IDX)[row];
    let addr = trace.main_trace.get_column(ADDR_COL_IDX)[row];
    let clk = trace.main_trace.get_column(CLK_COL_IDX)[row];
    let mut old_word = [ZERO; NUM_ELEMENTS];
    let mut new_word = [ZERO; NUM_ELEMENTS];

    for i in 0..NUM_ELEMENTS {
        old_word[i] = trace.main_trace.get_column(U_COL_RANGE.start + i)[row];
        new_word[i] = trace.main_trace.get_column(V_COL_RANGE.start + i)[row];
    }

    build_expected_memory(alphas, ctx, addr, clk, old_word, new_word)
}
