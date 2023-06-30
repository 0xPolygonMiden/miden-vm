use super::{build_trace_from_ops, Felt, FieldElement, Trace, NUM_RAND_ROWS, ONE, ZERO};
use miden_air::trace::{
    chiplets::hasher::HASH_CYCLE_LEN,
    range::{B_RANGE_COL_IDX, Q_COL_IDX},
    AUX_TRACE_RAND_ELEMENTS,
};
use rand_utils::rand_array;
use vm_core::Operation;

#[test]
#[allow(clippy::needless_range_loop)]
fn q_trace() {
    let stack = [1, 255];
    let operations = vec![
        Operation::U32add,
        Operation::MStoreW,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
    ];
    let mut trace = build_trace_from_ops(operations, &stack);

    let rand_elements = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let alpha = rand_elements[0];
    let aux_columns = trace.build_aux_segment(&[], &rand_elements).unwrap();
    let q = aux_columns.get_column(Q_COL_IDX);

    assert_eq!(trace.length(), q.len());

    // --- Check the stack processor's range check lookups. ---------------------------------------

    // Before any range checks are executed, the value in b_range should be one.
    assert_eq!(Felt::ONE, q[0]);

    // The first range check lookup from the stack will happen when the add operation is executed,
    // at cycle 1. (The trace begins by executing `span`). It must be divided out of `b_range`.
    // The range-checked values are 0, 256, 0, 0.
    let expected = (alpha) * (Felt::new(256) + alpha) * alpha.square();
    assert_eq!(expected, q[1]);

    // --- Check the last value of the q column is one. ------------------------------------------

    for row in 2..(q.len() - NUM_RAND_ROWS) {
        assert_eq!(Felt::ONE, q[row]);
    }
}

/// This test checks that range check lookups from stack operations are balanced by the range checks
/// processed in the Range Checker.
///
/// The `U32add` operation results in 4 16-bit range checks of 256, 0, 0, 0.
#[test]
fn b_range_trace_stack() {
    let stack = [1, 255];
    let operations = vec![Operation::U32add];
    let mut trace = build_trace_from_ops(operations, &stack);

    let rand_elements = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let alpha = rand_elements[0];
    let aux_columns = trace.build_aux_segment(&[], &rand_elements).unwrap();
    let b_range = aux_columns.get_column(B_RANGE_COL_IDX);

    assert_eq!(trace.length(), b_range.len());

    // --- Check the stack processor's range check lookups. ---------------------------------------

    // Before any range checks are executed, the value in b_range should be one.
    assert_eq!(Felt::ONE, b_range[0]);
    assert_eq!(Felt::ONE, b_range[1]);

    // The first range check lookup from the stack will happen when the add operation is executed,
    // at cycle 1. (The trace begins by executing `span`). It must be divided out of `b_range`.
    // The range-checked values are 0, 256, 0, 0.
    let lookup_product = (alpha) * (Felt::new(256) + alpha) * alpha.square();
    let mut expected = lookup_product.inv();
    assert_eq!(expected, b_range[2]);

    // --- Check the range checker's lookups. -----------------------------------------------------

    // 45 rows are needed for 0, 0, 243, 252, 255, 256, ... 38 additional bridge rows of
    // powers of 3 ..., 65535. (0 is range-checked in 2 rows for a total of 3 lookups. 256 is
    // range-checked in one row. 65535 is the max, and the rest are "bridge" values.) An extra row
    // is added to pad the u16::MAX value.
    let len_16bit = 45 + 1;
    // The start of the values in the range checker table.
    let values_start = trace.length() - len_16bit - NUM_RAND_ROWS;

    // After the padded rows, the first value will be unchanged.
    assert_eq!(expected, b_range[values_start]);
    // We include 2 lookups of 0, so the next value should be multiplied by alpha squared.
    expected *= alpha.square();
    assert_eq!(expected, b_range[values_start + 1]);
    // Then we include our third lookup of 0, so the next value should be multiplied by alpha.
    expected *= alpha;
    assert_eq!(expected, b_range[values_start + 2]);
    // Then we have 3 bridge rows between 0 and 255 where the value does not change
    assert_eq!(expected, b_range[values_start + 3]);
    assert_eq!(expected, b_range[values_start + 4]);
    assert_eq!(expected, b_range[values_start + 5]);
    // Then we include 1 lookup of 256, so it should be multiplied by alpha + 256.
    expected *= alpha + Felt::new(256);
    assert_eq!(expected, b_range[values_start + 6]);

    // --- Check the last value of the b_range column is one. ------------------------------------------

    let last_row = b_range.len() - NUM_RAND_ROWS - 1;
    assert_eq!(Felt::ONE, b_range[last_row]);
}

/// This test checks that range check lookups from memory operations are balanced by the
/// range checks processed in the Range Checker.
///
/// The `StoreW` memory operation results in 2 16-bit range checks of 0, 0.
/// The `LoadW` memory operation results in 2 16-bit range checks of 0, 0.
#[test]
#[allow(clippy::needless_range_loop)]
fn b_range_trace_mem() {
    let stack = [0, 1, 2, 3, 4, 0];
    let operations = vec![
        Operation::MStoreW,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::MLoadW,
    ];
    let mut trace = build_trace_from_ops(operations, &stack);

    let rand_elements = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let alpha = rand_elements[0];
    let aux_columns = trace.build_aux_segment(&[], &rand_elements).unwrap();
    let b_range = aux_columns.get_column(B_RANGE_COL_IDX);

    assert_eq!(trace.length(), b_range.len());

    // The memory section of the chiplets trace starts after the span hash.
    let memory_start = HASH_CYCLE_LEN;

    // 41 rows are needed for 0, 0, 3, 4, ... 36 bridge additional bridge rows of powers of
    // 3  ..., 65535. (0 is range-checked in 2 rows for a total of 3 lookups. Four is range
    // checked in one row for a total of one lookup. 65535 is the max, and the rest are "bridge"
    // values.) An extra row is added to pad the u16::MAX value.
    let len_16bit = 41 + 1;
    let values_start = trace.length() - len_16bit - NUM_RAND_ROWS;

    // The value should start at ONE and be unchanged until the memory processor section begins.
    let mut expected = ONE;
    for row in 0..=memory_start {
        assert_eq!(expected, b_range[row]);
    }

    // --- Check the memory processor's range check lookups. --------------------------------------

    // There are two memory lookups. For each memory lookup, the context and address are unchanged,
    // so the delta values indicated the clock cycle change i' - i - 1.
    // StoreW is executed at cycle 1 (after the initial span), so i' - i - 1 = 0.
    let (d0_store, d1_store) = (ZERO, ZERO);
    // LoadW is executed at cycle 6, so i' - i - 1 = 6 - 1 - 1 = 4.
    let (d0_load, d1_load) = (Felt::new(4), ZERO);

    // Include the lookups from the `MStoreW` operation at the next row.
    expected *= ((d0_store + alpha) * (d1_store + alpha)).inv();
    assert_eq!(expected, b_range[memory_start + 1]);
    // Include the lookup from the `MLoadW` operation at the next row.
    expected *= ((d0_load + alpha) * (d1_load + alpha)).inv();
    assert_eq!(expected, b_range[memory_start + 2]);

    // --- Check the range checker's lookups. -----------------------------------------------------

    // We include 2 lookups of ZERO in the next row.
    expected *= alpha.square();
    assert_eq!(expected, b_range[values_start + 1]);
    // We include 1 more lookup of ZERO in the next row.
    expected *= d0_store + alpha;
    assert_eq!(expected, b_range[values_start + 2]);

    // then we have one bridge row between 0 and 4 where the value does not change.
    assert_eq!(expected, b_range[values_start + 3]);

    // We include 1 lookup of 4 in the next row.
    expected *= d0_load + alpha;
    assert_eq!(expected, b_range[values_start + 4]);

    // --- The value should now be ONE for the rest of the trace. ---------------------------------
    assert_eq!(expected, ONE);
    for i in (values_start + 4)..(b_range.len() - NUM_RAND_ROWS) {
        assert_eq!(ONE, b_range[i]);
    }
}
