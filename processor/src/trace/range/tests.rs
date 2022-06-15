use super::{
    super::{ExecutionTrace, Process, NUM_RAND_ROWS},
    Felt, FieldElement, P0_COL_IDX, P1_COL_IDX,
};
use rand_utils::rand_value;
use vm_core::{program::blocks::CodeBlock, Operation, ProgramInputs};
use winterfell::Trace;

#[test]
fn p0_trace() {
    // --- Range check 256_u32 (4 16-bit range checks: 0, 256 and 0, 0) ---------------------------
    let stack = [1, 255];
    let operations = vec![Operation::U32add];
    let mut trace = build_trace(&stack, operations);

    let alpha = rand_value::<Felt>();
    let rand_elements = vec![alpha];
    let aux_columns = trace.build_aux_segment(&[], &rand_elements).unwrap();
    let p0 = aux_columns.get_column(P0_COL_IDX);

    assert_eq!(trace.length(), p0.len());

    // 256 8-bit rows are needed to for each value 0-255. 64 8-bit rows are needed to check 256
    // increments of 255 in the 16-bit portion of the table, for a total of 256 + 63 = 319 rows.
    let len_8bit = 319;
    // 260 16-bit rows are needed for 0, 0, 255, 256, ... 255 increments of 255 ..., 65535. (0 is
    // range-checked in 2 rows for a total of 3 lookups. 256 is range-checked in one row. 65535 is
    // the max, and the rest are "bridge" values.)
    let len_16bit = 260;
    // The range checker is padded at the beginning, so the padding must be skipped.
    let start_8bit = trace.length() - len_8bit - len_16bit - NUM_RAND_ROWS;
    let start_16bit = trace.length() - len_16bit - NUM_RAND_ROWS;

    // The padded portion of the column should be all ones.
    let expected_padding = vec![Felt::ONE; start_8bit];
    assert_eq!(expected_padding, p0[..start_8bit]);

    // The first value in the 8-bit portion should be one.
    assert_eq!(Felt::ONE, p0[start_8bit]);

    // At the start of the 16-bit portion, the value of `p0` should include all the 8-bit lookups:
    // 1 lookup of zero; 1 lookup of one; 1 lookup of 254; 256 lookups of 255.
    // Therefore, the value should be: alpha * (alpha + 1) * (alpha + 254) + (alpha + 255)^256
    let mut acc_255 = alpha + Felt::new(255);
    for _ in 0..8 {
        acc_255 *= acc_255;
    }
    let expected_acc = alpha * (alpha + Felt::ONE) * (alpha + Felt::new(254)) * acc_255;
    assert_eq!(expected_acc, p0[start_16bit]);

    // The final value at the end of the 16-bit portion should be 1. This will be the last row
    // before the random row.
    assert_eq!(Felt::ONE, p0[p0.len() - 1 - NUM_RAND_ROWS]);
}

#[test]
fn p1_trace() {
    // --- Range check 256_u32 (4 16-bit range checks: 0, 256 and 0, 0) ---------------------------
    let stack = [1, 255];
    let operations = vec![Operation::U32add];
    let mut trace = build_trace(&stack, operations);

    let alpha = rand_value::<Felt>();
    let rand_elements = vec![alpha];
    let aux_columns = trace.build_aux_segment(&[], &rand_elements).unwrap();
    let p1 = aux_columns.get_column(P1_COL_IDX);

    assert_eq!(trace.length(), p1.len());

    // 256 8-bit rows are needed to for each value 0-255. 64 8-bit rows are needed to check 256
    // increments of 255 in the 16-bit portion of the table, for a total of 256 + 63 = 319 rows.
    let len_8bit = 319;
    // 260 16-bit rows are needed for 0, 0, 255, 256, ... 255 increments of 255 ..., 65535. (0 is
    // range-checked in 2 rows for a total of 3 lookups. 256 is range-checked in one row. 65535 is
    // the max, and the rest are "bridge" values.)
    let len_16bit = 260;
    // The range checker is padded at the beginning, so the padding must be skipped.
    let padding_end = trace.length() - len_8bit - len_16bit - NUM_RAND_ROWS;
    let start_16bit = trace.length() - len_16bit - NUM_RAND_ROWS;

    // The values in p1 should be one for the length of the 8-bit table.
    let expected_8bit = vec![Felt::ONE; len_8bit];
    assert_eq!(expected_8bit, p1[padding_end..start_16bit]);

    // Once the 16-bit portion of the table starts, the first value will be one.
    assert_eq!(p1[start_16bit], Felt::ONE);
    // We include 2 lookups of 0, so the next value should be multiplied by alpha squared.
    let mut expected = alpha.square();
    assert_eq!(expected, p1[start_16bit + 1]);
    // Then we include our third lookup of 0, so the next value should be multiplied by alpha.
    expected *= alpha;
    assert_eq!(expected, p1[start_16bit + 2]);
    // Then we have a bridge row for 255 where the value does not change
    assert_eq!(expected, p1[start_16bit + 3]);
    // Then we include 1 lookup of 256, so it should be multiplied by alpha + 256.
    expected *= alpha + Felt::new(256);
    assert_eq!(expected, p1[start_16bit + 4]);
}

// HELPER FUNCTIONS
// ================================================================================================

/// Builds a sample trace by executing a span block containing the specified operations. This
/// results in 1 additional hash cycle at the beginning of the hasher coprocessor.
fn build_trace(stack: &[u64], operations: Vec<Operation>) -> ExecutionTrace {
    let inputs = ProgramInputs::new(stack, &[], vec![]).unwrap();
    let mut process = Process::new(inputs);
    let program = CodeBlock::new_span(operations);
    process.execute_code_block(&program).unwrap();

    ExecutionTrace::new(process)
}
