use miden_air::trace::{
    decoder::{P1_COL_IDX, P2_COL_IDX},
    AUX_TRACE_RAND_ELEMENTS,
};
use test_utils::rand::rand_array;
use vm_core::{
    mast::{MastForest, MastNode},
    FieldElement, Operation, Program, Word, ONE, ZERO,
};

use super::{
    super::{
        tests::{build_trace_from_ops, build_trace_from_program},
        utils::build_span_with_respan_ops,
    },
    Felt,
};
use crate::{decoder::BlockHashTableRow, ContextId};

// BLOCK STACK TABLE TESTS
// ================================================================================================

#[test]
#[allow(clippy::needless_range_loop)]
fn decoder_p1_span_with_respan() {
    let (ops, _) = build_span_with_respan_ops();
    let trace = build_trace_from_ops(ops, &[]);
    let alphas = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let aux_columns = trace.build_aux_trace(&alphas).unwrap();
    let p1 = aux_columns.get_column(P1_COL_IDX);

    let row_values = [
        BlockStackTableRow::new(ONE, ZERO, false).to_value(&alphas),
        BlockStackTableRow::new(Felt::new(9), ZERO, false).to_value(&alphas),
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

    // for the next 11 cycles (as we execute user ops), the table is not affected
    for i in 11..22 {
        assert_eq!(expected_value, p1[i]);
    }

    // at cycle 22, the END operation is executed and the table is cleared
    let expected_value = expected_value * row_values[1].inv();
    assert_eq!(expected_value, ONE);
    for i in 22..p1.len() {
        assert_eq!(ONE, p1[i]);
    }
}

#[test]
#[allow(clippy::needless_range_loop)]
fn decoder_p1_join() {
    let program = {
        let mut mast_forest = MastForest::new();

        let basic_block_1_id = mast_forest.add_block(vec![Operation::Mul], None).unwrap();
        let basic_block_2_id = mast_forest.add_block(vec![Operation::Add], None).unwrap();
        let join_id = mast_forest.add_join(basic_block_1_id, basic_block_2_id).unwrap();
        mast_forest.make_root(join_id);

        Program::new(mast_forest.into(), join_id)
    };

    let trace = build_trace_from_program(&program, &[]);
    let alphas = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let aux_columns = trace.build_aux_trace(&alphas).unwrap();
    let p1 = aux_columns.get_column(P1_COL_IDX);

    let a_9 = Felt::new(9);
    let a_17 = Felt::new(17);
    let row_values = [
        BlockStackTableRow::new(ONE, ZERO, false).to_value(&alphas),
        BlockStackTableRow::new(a_9, ONE, false).to_value(&alphas),
        BlockStackTableRow::new(a_17, ONE, false).to_value(&alphas),
    ];

    // make sure the first entry is ONE
    assert_eq!(ONE, p1[0]);

    // when JOIN operation is executed, entry for the JOIN block is added to the table
    let mut expected_value = row_values[0];
    assert_eq!(expected_value, p1[1]);

    // when the first SPAN is executed, its entry is added to the table
    expected_value *= row_values[1];
    assert_eq!(expected_value, p1[2]);

    // when the user op is executed, the table is not affected
    assert_eq!(expected_value, p1[3]);

    // when the first SPAN block ends, its entry is removed from the table
    expected_value *= row_values[1].inv();
    assert_eq!(expected_value, p1[4]);

    // when the second SPAN is executed, its entry is added to the table
    expected_value *= row_values[2];
    assert_eq!(expected_value, p1[5]);

    // when the user op is executed, the table is not affected
    assert_eq!(expected_value, p1[6]);

    // when the second SPAN block ends, its entry is removed from the table
    expected_value *= row_values[2].inv();
    assert_eq!(expected_value, p1[7]);

    // when the JOIN block ends, its entry is removed from the table
    expected_value *= row_values[0].inv();
    assert_eq!(expected_value, p1[8]);

    // at this point the table should be empty, and thus, all subsequent values must be ONE
    assert_eq!(expected_value, ONE);
    for i in 9..p1.len() {
        assert_eq!(ONE, p1[i]);
    }
}

#[test]
#[allow(clippy::needless_range_loop)]
fn decoder_p1_split() {
    let program = {
        let mut mast_forest = MastForest::new();

        let basic_block_1_id = mast_forest.add_block(vec![Operation::Mul], None).unwrap();
        let basic_block_2_id = mast_forest.add_block(vec![Operation::Add], None).unwrap();
        let split_id = mast_forest.add_split(basic_block_1_id, basic_block_2_id).unwrap();
        mast_forest.make_root(split_id);

        Program::new(mast_forest.into(), split_id)
    };

    let trace = build_trace_from_program(&program, &[1]);
    let alphas = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let aux_columns = trace.build_aux_trace(&alphas).unwrap();
    let p1 = aux_columns.get_column(P1_COL_IDX);

    let a_9 = Felt::new(9);
    let row_values = [
        BlockStackTableRow::new(ONE, ZERO, false).to_value(&alphas),
        BlockStackTableRow::new(a_9, ONE, false).to_value(&alphas),
    ];

    // make sure the first entry is ONE
    assert_eq!(ONE, p1[0]);

    // when SPLIT operation is executed, entry for the SPLIT block is added to the table
    let mut expected_value = row_values[0];
    assert_eq!(expected_value, p1[1]);

    // when the true branch SPAN is executed, its entry is added to the table
    expected_value *= row_values[1];
    assert_eq!(expected_value, p1[2]);

    // when the user op is executed, the table is not affected
    assert_eq!(expected_value, p1[3]);

    // when the SPAN block ends, its entry is removed from the table
    expected_value *= row_values[1].inv();
    assert_eq!(expected_value, p1[4]);

    // when the SPLIT block ends, its entry is removed from the table
    expected_value *= row_values[0].inv();
    assert_eq!(expected_value, p1[5]);

    // at this point the table should be empty, and thus, all subsequent values must be ONE
    assert_eq!(expected_value, ONE);
    for i in 6..p1.len() {
        assert_eq!(ONE, p1[i]);
    }
}

#[test]
#[allow(clippy::needless_range_loop)]
fn decoder_p1_loop_with_repeat() {
    let program = {
        let mut mast_forest = MastForest::new();

        let basic_block_1_id = mast_forest.add_block(vec![Operation::Pad], None).unwrap();
        let basic_block_2_id = mast_forest.add_block(vec![Operation::Drop], None).unwrap();
        let join_id = mast_forest.add_join(basic_block_1_id, basic_block_2_id).unwrap();
        let loop_node_id = mast_forest.add_loop(join_id).unwrap();
        mast_forest.make_root(loop_node_id);

        Program::new(mast_forest.into(), loop_node_id)
    };

    let trace = build_trace_from_program(&program, &[0, 1, 1]);
    let alphas = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let aux_columns = trace.build_aux_trace(&alphas).unwrap();
    let p1 = aux_columns.get_column(P1_COL_IDX);

    let a_9 = Felt::new(9); // address of the JOIN block in the first iteration
    let a_17 = Felt::new(17); // address of the first SPAN block in the first iteration
    let a_25 = Felt::new(25); // address of the second SPAN block in the first iteration
    let a_33 = Felt::new(33); // address of the JOIN block in the second iteration
    let a_41 = Felt::new(41); // address of the first SPAN block in the second iteration
    let a_49 = Felt::new(49); // address of the second SPAN block in the second iteration
    let row_values = [
        BlockStackTableRow::new(ONE, ZERO, true).to_value(&alphas),
        BlockStackTableRow::new(a_9, ONE, false).to_value(&alphas),
        BlockStackTableRow::new(a_17, a_9, false).to_value(&alphas),
        BlockStackTableRow::new(a_25, a_9, false).to_value(&alphas),
        BlockStackTableRow::new(a_33, ONE, false).to_value(&alphas),
        BlockStackTableRow::new(a_41, a_33, false).to_value(&alphas),
        BlockStackTableRow::new(a_49, a_33, false).to_value(&alphas),
    ];

    // make sure the first entry is ONE
    assert_eq!(ONE, p1[0]);

    // --- first iteration ----------------------------------------------------

    // when LOOP operation is executed, entry for the LOOP block is added to the table
    let mut expected_value = row_values[0];
    assert_eq!(expected_value, p1[1]);

    // when JOIN operation is executed, entry for the JOIN block is added to the table
    expected_value *= row_values[1];
    assert_eq!(expected_value, p1[2]);

    // when the first SPAN is executed, its entry is added to the table
    expected_value *= row_values[2];
    assert_eq!(expected_value, p1[3]);

    // when the user op is executed, the table is not affected
    assert_eq!(expected_value, p1[4]);

    // when the first SPAN block ends, its entry is removed from the table
    expected_value *= row_values[2].inv();
    assert_eq!(expected_value, p1[5]);

    // when the second SPAN is executed, its entry is added to the table
    expected_value *= row_values[3];
    assert_eq!(expected_value, p1[6]);

    // when the user op is executed, the table is not affected
    assert_eq!(expected_value, p1[7]);

    // when the second SPAN block ends, its entry is removed from the table
    expected_value *= row_values[3].inv();
    assert_eq!(expected_value, p1[8]);

    // when the JOIN block ends, its entry is removed from the table
    expected_value *= row_values[1].inv();
    assert_eq!(expected_value, p1[9]);

    // --- second iteration ---------------------------------------------------

    // when REPEAT operation is executed, the table is not affected
    assert_eq!(expected_value, p1[10]);

    // when JOIN operation is executed, entry for the JOIN block is added to the table
    expected_value *= row_values[4];
    assert_eq!(expected_value, p1[11]);

    // when the first SPAN is executed, its entry is added to the table
    expected_value *= row_values[5];
    assert_eq!(expected_value, p1[12]);

    // when the user op is executed, the table is not affected
    assert_eq!(expected_value, p1[13]);

    // when the first SPAN block ends, its entry is removed from the table
    expected_value *= row_values[5].inv();
    assert_eq!(expected_value, p1[14]);

    // when the second SPAN is executed, its entry is added to the table
    expected_value *= row_values[6];
    assert_eq!(expected_value, p1[15]);

    // when the user op is executed, the table is not affected
    assert_eq!(expected_value, p1[16]);

    // when the second SPAN block ends, its entry is removed from the table
    expected_value *= row_values[6].inv();
    assert_eq!(expected_value, p1[17]);

    // when the JOIN block ends, its entry is removed from the table
    expected_value *= row_values[4].inv();
    assert_eq!(expected_value, p1[18]);

    // when the LOOP block ends, its entry is removed from the table
    expected_value *= row_values[0].inv();
    assert_eq!(expected_value, p1[19]);

    // at this point the table should be empty, and thus, all subsequent values must be ONE
    assert_eq!(expected_value, ONE);
    for i in 20..p1.len() {
        assert_eq!(ONE, p1[i]);
    }
}

// BLOCK HASH TABLE TESTS
// ================================================================================================

#[test]
#[allow(clippy::needless_range_loop)]
fn decoder_p2_span_with_respan() {
    let program = {
        let mut mast_forest = MastForest::new();

        let (ops, _) = build_span_with_respan_ops();
        let basic_block_id = mast_forest.add_block(ops, None).unwrap();
        mast_forest.make_root(basic_block_id);

        Program::new(mast_forest.into(), basic_block_id)
    };
    let trace = build_trace_from_program(&program, &[]);
    let alphas = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let aux_columns = trace.build_aux_trace(&alphas).unwrap();
    let p2 = aux_columns.get_column(P2_COL_IDX);

    let row_values = [
        BlockHashTableRow::new_test(ZERO, program.hash().into(), false, false).collapse(&alphas)
    ];

    // make sure the first entry is initialized to program hash
    let mut expected_value = row_values[0];
    assert_eq!(expected_value, p2[0]);

    // as operations inside the span execute (including RESPAN), the table is not affected
    for i in 1..22 {
        assert_eq!(expected_value, p2[i]);
    }

    // at cycle 22, the END operation is executed and the table is cleared
    expected_value *= row_values[0].inv();
    assert_eq!(expected_value, ONE);
    for i in 22..p2.len() {
        assert_eq!(ONE, p2[i]);
    }
}

#[test]
#[allow(clippy::needless_range_loop)]
fn decoder_p2_join() {
    let mut mast_forest = MastForest::new();

    let basic_block_1 = MastNode::new_basic_block(vec![Operation::Mul], None).unwrap();
    let basic_block_1_id = mast_forest.add_node(basic_block_1.clone()).unwrap();

    let basic_block_2 = MastNode::new_basic_block(vec![Operation::Add], None).unwrap();
    let basic_block_2_id = mast_forest.add_node(basic_block_2.clone()).unwrap();

    let join = MastNode::new_join(basic_block_1_id, basic_block_2_id, &mast_forest).unwrap();
    let join_id = mast_forest.add_node(join.clone()).unwrap();
    mast_forest.make_root(join_id);

    let program = Program::new(mast_forest.into(), join_id);

    let trace = build_trace_from_program(&program, &[]);
    let alphas = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let aux_columns = trace.build_aux_trace(&alphas).unwrap();
    let p2 = aux_columns.get_column(P2_COL_IDX);

    let row_values = [
        BlockHashTableRow::new_test(ZERO, join.digest().into(), false, false).collapse(&alphas),
        BlockHashTableRow::new_test(ONE, basic_block_1.digest().into(), true, false)
            .collapse(&alphas),
        BlockHashTableRow::new_test(ONE, basic_block_2.digest().into(), false, false)
            .collapse(&alphas),
    ];

    // make sure the first entry is initialized to program hash
    let mut expected_value = row_values[0];
    assert_eq!(expected_value, p2[0]);

    // when JOIN operation is executed, entries for both children are added to the table
    expected_value *= row_values[1] * row_values[2];
    assert_eq!(expected_value, p2[1]);

    // for the next 2 cycles, the table is not affected
    assert_eq!(expected_value, p2[2]);
    assert_eq!(expected_value, p2[3]);

    // when the first SPAN block ends, its entry is removed from the table
    expected_value *= row_values[1].inv();
    assert_eq!(expected_value, p2[4]);

    // for the next 2 cycles, the table is not affected
    assert_eq!(expected_value, p2[5]);
    assert_eq!(expected_value, p2[6]);

    // when the second SPAN block ends, its entry is removed from the table
    expected_value *= row_values[2].inv();
    assert_eq!(expected_value, p2[7]);

    // when the JOIN block ends, its entry is removed from the table
    expected_value *= row_values[0].inv();
    assert_eq!(expected_value, p2[8]);

    // at this point the table should be empty, and thus, all subsequent values must be ONE
    assert_eq!(expected_value, ONE);
    for i in 9..p2.len() {
        assert_eq!(ONE, p2[i]);
    }
}

#[test]
#[allow(clippy::needless_range_loop)]
fn decoder_p2_split_true() {
    // build program
    let mut mast_forest = MastForest::new();

    let basic_block_1 = MastNode::new_basic_block(vec![Operation::Mul], None).unwrap();
    let basic_block_1_id = mast_forest.add_node(basic_block_1.clone()).unwrap();
    let basic_block_2_id = mast_forest.add_block(vec![Operation::Add], None).unwrap();
    let split_id = mast_forest.add_split(basic_block_1_id, basic_block_2_id).unwrap();
    mast_forest.make_root(split_id);

    let program = Program::new(mast_forest.into(), split_id);

    // build trace from program
    let trace = build_trace_from_program(&program, &[1]);
    let alphas = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let aux_columns = trace.build_aux_trace(&alphas).unwrap();
    let p2 = aux_columns.get_column(P2_COL_IDX);

    let row_values = [
        BlockHashTableRow::new_test(ZERO, program.hash().into(), false, false).collapse(&alphas),
        BlockHashTableRow::new_test(ONE, basic_block_1.digest().into(), false, false)
            .collapse(&alphas),
    ];

    // make sure the first entry is initialized to program hash
    let mut expected_value = row_values[0];
    assert_eq!(expected_value, p2[0]);

    // when SPLIT operation is executed, entry for the true branch is added to the table
    expected_value *= row_values[1];
    assert_eq!(expected_value, p2[1]);

    // for the next 2 cycles, the table is not affected
    assert_eq!(expected_value, p2[2]);
    assert_eq!(expected_value, p2[3]);

    // when the SPAN block ends, its entry is removed from the table
    expected_value *= row_values[1].inv();
    assert_eq!(expected_value, p2[4]);

    // when the SPLIT block ends, its entry is removed from the table
    expected_value *= row_values[0].inv();
    assert_eq!(expected_value, p2[5]);

    // at this point the table should be empty, and thus, all subsequent values must be ONE
    assert_eq!(expected_value, ONE);
    for i in 6..p2.len() {
        assert_eq!(ONE, p2[i]);
    }
}

#[test]
#[allow(clippy::needless_range_loop)]
fn decoder_p2_split_false() {
    // build program
    let mut mast_forest = MastForest::new();

    let basic_block_1 = MastNode::new_basic_block(vec![Operation::Mul], None).unwrap();
    let basic_block_1_id = mast_forest.add_node(basic_block_1.clone()).unwrap();

    let basic_block_2 = MastNode::new_basic_block(vec![Operation::Add], None).unwrap();
    let basic_block_2_id = mast_forest.add_node(basic_block_2.clone()).unwrap();

    let split_id = mast_forest.add_split(basic_block_1_id, basic_block_2_id).unwrap();
    mast_forest.make_root(split_id);

    let program = Program::new(mast_forest.into(), split_id);

    // build trace from program
    let trace = build_trace_from_program(&program, &[0]);
    let alphas = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let aux_columns = trace.build_aux_trace(&alphas).unwrap();
    let p2 = aux_columns.get_column(P2_COL_IDX);

    let row_values = [
        BlockHashTableRow::new_test(ZERO, program.hash().into(), false, false).collapse(&alphas),
        BlockHashTableRow::new_test(ONE, basic_block_2.digest().into(), false, false)
            .collapse(&alphas),
    ];

    // make sure the first entry is initialized to program hash
    let mut expected_value = row_values[0];
    assert_eq!(expected_value, p2[0]);

    // when SPLIT operation is executed, entry for the false branch is added to the table
    expected_value *= row_values[1];
    assert_eq!(expected_value, p2[1]);

    // for the next 2 cycles, the table is not affected
    assert_eq!(expected_value, p2[2]);
    assert_eq!(expected_value, p2[3]);

    // when the SPAN block ends, its entry is removed from the table
    expected_value *= row_values[1].inv();
    assert_eq!(expected_value, p2[4]);

    // when the SPLIT block ends, its entry is removed from the table
    expected_value *= row_values[0].inv();
    assert_eq!(expected_value, p2[5]);

    // at this point the table should be empty, and thus, all subsequent values must be ONE
    assert_eq!(expected_value, ONE);
    for i in 6..p2.len() {
        assert_eq!(ONE, p2[i]);
    }
}

#[test]
#[allow(clippy::needless_range_loop)]
fn decoder_p2_loop_with_repeat() {
    // build program
    let mut mast_forest = MastForest::new();

    let basic_block_1 = MastNode::new_basic_block(vec![Operation::Pad], None).unwrap();
    let basic_block_1_id = mast_forest.add_node(basic_block_1.clone()).unwrap();

    let basic_block_2 = MastNode::new_basic_block(vec![Operation::Drop], None).unwrap();
    let basic_block_2_id = mast_forest.add_node(basic_block_2.clone()).unwrap();

    let join = MastNode::new_join(basic_block_1_id, basic_block_2_id, &mast_forest).unwrap();
    let join_id = mast_forest.add_node(join.clone()).unwrap();

    let loop_node_id = mast_forest.add_loop(join_id).unwrap();
    mast_forest.make_root(loop_node_id);

    let program = Program::new(mast_forest.into(), loop_node_id);

    // build trace from program
    let trace = build_trace_from_program(&program, &[0, 1, 1]);
    let alphas = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let aux_columns = trace.build_aux_trace(&alphas).unwrap();
    let p2 = aux_columns.get_column(P2_COL_IDX);

    let a_9 = Felt::new(9); // address of the JOIN block in the first iteration
    let a_33 = Felt::new(33); // address of the JOIN block in the second iteration
    let row_values = [
        BlockHashTableRow::new_test(ZERO, program.hash().into(), false, false).collapse(&alphas),
        BlockHashTableRow::new_test(ONE, join.digest().into(), false, true).collapse(&alphas),
        BlockHashTableRow::new_test(a_9, basic_block_1.digest().into(), true, false)
            .collapse(&alphas),
        BlockHashTableRow::new_test(a_9, basic_block_2.digest().into(), false, false)
            .collapse(&alphas),
        BlockHashTableRow::new_test(a_33, basic_block_1.digest().into(), true, false)
            .collapse(&alphas),
        BlockHashTableRow::new_test(a_33, basic_block_2.digest().into(), false, false)
            .collapse(&alphas),
    ];

    // make sure the first entry is initialized to program hash
    let mut expected_value = row_values[0];
    assert_eq!(expected_value, p2[0]);

    // --- first iteration ----------------------------------------------------

    // when LOOP operation is executed, entry for loop body is added to the table
    expected_value *= row_values[1];
    assert_eq!(expected_value, p2[1]);

    // when JOIN operation is executed, entries for both children are added to the table
    expected_value *= row_values[2] * row_values[3];
    assert_eq!(expected_value, p2[2]);

    // for the next 2 cycles, the table is not affected
    assert_eq!(expected_value, p2[3]);
    assert_eq!(expected_value, p2[4]);

    // when the first SPAN block ends, its entry is removed from the table
    expected_value *= row_values[2].inv();
    assert_eq!(expected_value, p2[5]);

    // for the next 2 cycles, the table is not affected
    assert_eq!(expected_value, p2[6]);
    assert_eq!(expected_value, p2[7]);

    // when the second SPAN block ends, its entry is removed from the table
    expected_value *= row_values[3].inv();
    assert_eq!(expected_value, p2[8]);

    // when the JOIN block ends, its entry is removed from the table
    expected_value *= row_values[1].inv();
    assert_eq!(expected_value, p2[9]);

    // --- second iteration ---------------------------------------------------

    // when REPEAT operation is executed, entry for loop body is again added to the table
    expected_value *= row_values[1];
    assert_eq!(expected_value, p2[10]);

    // when JOIN operation is executed, entries for both children are added to the table
    expected_value *= row_values[4] * row_values[5];
    assert_eq!(expected_value, p2[11]);

    // for the next 2 cycles, the table is not affected
    assert_eq!(expected_value, p2[12]);
    assert_eq!(expected_value, p2[13]);

    // when the first SPAN block ends, its entry is removed from the table
    expected_value *= row_values[4].inv();
    assert_eq!(expected_value, p2[14]);

    // for the next 2 cycles, the table is not affected
    assert_eq!(expected_value, p2[15]);
    assert_eq!(expected_value, p2[16]);

    // when the second SPAN block ends, its entry is removed from the table
    expected_value *= row_values[5].inv();
    assert_eq!(expected_value, p2[17]);

    // when the JOIN block ends, its entry is removed from the table
    expected_value *= row_values[1].inv();
    assert_eq!(expected_value, p2[18]);

    // when the LOOP block ends, its entry is removed from the table
    expected_value *= row_values[0].inv();
    assert_eq!(expected_value, p2[19]);

    // at this point the table should be empty, and thus, all subsequent values must be ONE
    assert_eq!(expected_value, ONE);
    for i in 20..p2.len() {
        assert_eq!(ONE, p2[i]);
    }
}

// HELPER STRUCTS AND METHODS
// ================================================================================================

/// Describes a single entry in the block stack table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockStackTableRow {
    block_id: Felt,
    parent_id: Felt,
    is_loop: bool,
    parent_ctx: ContextId,
    parent_fn_hash: Word,
    parent_fmp: Felt,
    parent_stack_depth: u32,
    parent_next_overflow_addr: Felt,
}

impl BlockStackTableRow {
    /// Returns a new [BlockStackTableRow] instantiated with the specified parameters. This is
    /// used for test purpose only.
    #[cfg(test)]
    pub fn new(block_id: Felt, parent_id: Felt, is_loop: bool) -> Self {
        Self {
            block_id,
            parent_id,
            is_loop,
            parent_ctx: ContextId::root(),
            parent_fn_hash: vm_core::EMPTY_WORD,
            parent_fmp: ZERO,
            parent_stack_depth: 0,
            parent_next_overflow_addr: ZERO,
        }
    }
}

impl BlockStackTableRow {
    /// Reduces this row to a single field element in the field specified by E. This requires
    /// at least 12 alpha values.
    pub fn to_value<E: FieldElement<BaseField = Felt>>(&self, alphas: &[E]) -> E {
        let is_loop = if self.is_loop { ONE } else { ZERO };
        alphas[0]
            + alphas[1].mul_base(self.block_id)
            + alphas[2].mul_base(self.parent_id)
            + alphas[3].mul_base(is_loop)
            + alphas[4].mul_base(Felt::from(self.parent_ctx))
            + alphas[5].mul_base(self.parent_fmp)
            + alphas[6].mul_base(Felt::from(self.parent_stack_depth))
            + alphas[7].mul_base(self.parent_next_overflow_addr)
            + alphas[8].mul_base(self.parent_fn_hash[0])
            + alphas[9].mul_base(self.parent_fn_hash[1])
            + alphas[10].mul_base(self.parent_fn_hash[2])
            + alphas[11].mul_base(self.parent_fn_hash[3])
    }
}
