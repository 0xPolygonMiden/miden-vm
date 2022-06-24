use super::{
    super::{utils::build_trace_from_ops, LookupTableRow, Trace, NUM_RAND_ROWS},
    Felt,
};
use crate::decoder::{build_op_group, BlockStackTableRow, OpGroupTableRow};
use rand_utils::rand_array;
use vm_core::{
    decoder::{P1_COL_IDX, P3_COL_IDX},
    utils::ToElements,
    FieldElement, Operation, AUX_TRACE_RAND_ELEMENTS, ONE, ZERO,
};

// BLOCK STACK TABLE TESTS
// ================================================================================================

#[test]
#[allow(clippy::needless_range_loop)]
fn decoder_p1_span_with_respan() {
    let iv = [1, 3, 5, 7, 9, 11, 13, 15, 17].to_elements();
    let ops = vec![
        Operation::Push(iv[0]),
        Operation::Push(iv[1]),
        Operation::Push(iv[2]),
        Operation::Push(iv[3]),
        Operation::Push(iv[4]),
        Operation::Push(iv[5]),
        Operation::Push(iv[6]),
        // next batch
        Operation::Push(iv[7]),
        Operation::Push(iv[8]),
        Operation::Add,
    ];
    let mut trace = build_trace_from_ops(&[], ops.clone());
    let alphas = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let aux_columns = trace.build_aux_segment(&[], &alphas).unwrap();
    let p1 = aux_columns.get_column(P1_COL_IDX);

    let row_values = [
        BlockStackTableRow::new_test(ONE, ZERO, false).to_value(&alphas),
        BlockStackTableRow::new_test(Felt::new(9), ZERO, false).to_value(&alphas),
    ];

    // make sure the first entry is ONE
    assert_eq!(ONE, p1[0]);

    // when SPAN operation is executed, entry for span block is added to the table
    let expected_value = row_values[0];
    assert_eq!(expected_value, p1[1]);

    // for the next 8 cycles (as we execute user ops), the table is not affected
    for i in 2..10 {
        assert_eq!(expected_value, p1[i]);
    }

    // when RESPAN is executed, the first entry is replaced with a new entry
    let expected_value = expected_value * row_values[0].inv() * row_values[1];
    assert_eq!(expected_value, p1[10]);

    // for the next 4 cycles (as we execute user ops), the table is not affected
    for i in 11..15 {
        assert_eq!(expected_value, p1[i]);
    }

    // at cycle 14, the END operation is executed and the table is cleared
    let expected_value = expected_value * row_values[1].inv();
    assert_eq!(expected_value, ONE);
    for i in 15..(p1.len() - NUM_RAND_ROWS) {
        assert_eq!(ONE, p1[i]);
    }
}

// OP GROUP TABLE TESTS
// ================================================================================================

#[test]
fn decoder_p3_trace_empty_table() {
    let stack = [1, 2];
    let operations = vec![Operation::Add];
    let mut trace = build_trace_from_ops(&stack, operations);

    let rand_elements = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let aux_columns = trace.build_aux_segment(&[], &rand_elements).unwrap();

    // no rows should have been added or removed from the op group table, and thus, all values
    // in the column must be ONE
    let p3 = aux_columns.get_column(P3_COL_IDX);
    for &value in p3.iter().take(p3.len() - NUM_RAND_ROWS) {
        assert_eq!(ONE, value);
    }
}

#[test]
#[allow(clippy::needless_range_loop)]
fn decoder_p3_trace_one_batch() {
    let stack = [1, 2, 3, 4, 5, 6, 7, 8];
    let ops = vec![
        Operation::Add,
        Operation::Mul,
        Operation::Add,
        Operation::Push(ONE),
        Operation::Add,
        Operation::Mul,
        Operation::Add,
        Operation::Push(Felt::new(2)),
        Operation::Add,
        Operation::Swap,
        Operation::Mul,
        Operation::Add,
    ];
    let mut trace = build_trace_from_ops(&stack, ops.clone());
    let alphas = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let aux_columns = trace.build_aux_segment(&[], &alphas).unwrap();
    let p3 = aux_columns.get_column(P3_COL_IDX);

    // make sure the first entry is ONE
    assert_eq!(ONE, p3[0]);

    // make sure 3 groups were inserted at clock cycle 1; these entries are for the two immediate
    // values and the second operation group consisting of [SWAP, MUL, ADD]
    let g1_value = OpGroupTableRow::new(ONE, Felt::new(3), ONE).to_value(&alphas);
    let g2_value = OpGroupTableRow::new(ONE, Felt::new(2), Felt::new(2)).to_value(&alphas);
    let g3_value =
        OpGroupTableRow::new(ONE, Felt::new(1), build_op_group(&ops[9..])).to_value(&alphas);
    let expected_value = g1_value * g2_value * g3_value;
    assert_eq!(expected_value, p3[1]);

    // for the next 3 cycles (2, 3, 4), op group table doesn't change
    for i in 2..5 {
        assert_eq!(expected_value, p3[i]);
    }

    // at cycle 5, when PUSH(1) is executed, the entry for the first group is removed from the
    // table
    let expected_value = expected_value / g1_value;
    assert_eq!(expected_value, p3[5]);

    // for the next 3 cycles (6, 7, 8), op group table doesn't change
    for i in 6..9 {
        assert_eq!(expected_value, p3[i]);
    }

    // at cycle 9, when PUSH(2) is executed, the entry for the second group is removed from the
    // table
    let expected_value = expected_value / g2_value;
    assert_eq!(expected_value, p3[9]);

    // at cycle 10, op group 0 is completed, and the entry for the next op group is removed from
    // the table
    let expected_value = expected_value / g3_value;
    assert_eq!(expected_value, p3[10]);

    // at this point, the table should be empty and thus, running product should be ONE
    assert_eq!(expected_value, ONE);
    for i in 11..(p3.len() - NUM_RAND_ROWS) {
        assert_eq!(ONE, p3[i]);
    }
}

#[test]
#[allow(clippy::needless_range_loop)]
fn decoder_p3_trace_two_batches() {
    let iv = [1, 3, 5, 7, 9, 11, 13, 15, 17].to_elements();
    let ops = vec![
        Operation::Push(iv[0]),
        Operation::Push(iv[1]),
        Operation::Push(iv[2]),
        Operation::Push(iv[3]),
        Operation::Push(iv[4]),
        Operation::Push(iv[5]),
        Operation::Push(iv[6]),
        // next batch
        Operation::Push(iv[7]),
        Operation::Push(iv[8]),
        Operation::Add,
    ];
    let mut trace = build_trace_from_ops(&[], ops.clone());
    let alphas = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let aux_columns = trace.build_aux_segment(&[], &alphas).unwrap();
    let p3 = aux_columns.get_column(P3_COL_IDX);

    // make sure the first entry is ONE
    assert_eq!(ONE, p3[0]);

    // --- first batch ----------------------------------------------------------------------------
    // make sure entries for 7 groups were inserted at clock cycle 1
    let b0_values = [
        OpGroupTableRow::new(ONE, Felt::new(11), iv[0]).to_value(&alphas),
        OpGroupTableRow::new(ONE, Felt::new(10), iv[1]).to_value(&alphas),
        OpGroupTableRow::new(ONE, Felt::new(9), iv[2]).to_value(&alphas),
        OpGroupTableRow::new(ONE, Felt::new(8), iv[3]).to_value(&alphas),
        OpGroupTableRow::new(ONE, Felt::new(7), iv[4]).to_value(&alphas),
        OpGroupTableRow::new(ONE, Felt::new(6), iv[5]).to_value(&alphas),
        OpGroupTableRow::new(ONE, Felt::new(5), iv[6]).to_value(&alphas),
    ];
    let mut expected_value: Felt = b0_values.iter().fold(ONE, |acc, &val| acc * val);
    assert_eq!(expected_value, p3[1]);

    // for the next 7 cycles (2, 3, 4, 5, 6, 7, 8), an entry for an op group is removed from the
    // table
    for (i, clk) in (2..9).enumerate() {
        expected_value /= b0_values[i];
        assert_eq!(expected_value, p3[clk]);
    }

    // at cycle 9, when we execute a NOOP to finish the first batch, op group table doesn't change;
    // also, at this point op group table must be empty
    assert_eq!(expected_value, p3[9]);
    assert_eq!(expected_value, ONE);

    // --- second batch ---------------------------------------------------------------------------
    // make sure entries for 3 groups (two immediate values and NOOP for the padding group) are
    // inserted at clock cycle 10 (when RESPAN is executed)
    let batch1_addr = ONE + Felt::new(8);
    let b1_values = [
        OpGroupTableRow::new(batch1_addr, Felt::new(3), iv[7]).to_value(&alphas),
        OpGroupTableRow::new(batch1_addr, Felt::new(2), iv[8]).to_value(&alphas),
        OpGroupTableRow::new(batch1_addr, Felt::new(1), ZERO).to_value(&alphas),
    ];
    let mut expected_value: Felt = b1_values.iter().fold(ONE, |acc, &val| acc * val);
    assert_eq!(expected_value, p3[10]);

    // for the next 2 cycles (11, 12), an entry for an op group is removed from the table
    for (i, clk) in (11..13).enumerate() {
        expected_value /= b1_values[i];
        assert_eq!(expected_value, p3[clk]);
    }

    // at cycle 13, when ADD is executed, the entry for the last op group is removed from the
    // table
    expected_value /= b1_values[2];
    assert_eq!(expected_value, p3[13]);

    // at this point, the table should be empty and thus, running product should be ONE
    assert_eq!(expected_value, ONE);
    for i in 14..(p3.len() - NUM_RAND_ROWS) {
        assert_eq!(ONE, p3[i]);
    }
}
