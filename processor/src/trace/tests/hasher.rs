use super::{
    super::{Trace, NUM_RAND_ROWS},
    build_trace_from_ops_with_inputs, rand_array, AdviceInputs, Felt, LookupTableRow, Operation,
    Vec, Word, ONE, ZERO,
};
use crate::{chiplets::SiblingTableRow, StackInputs};
use vm_core::{
    chiplets::hasher::P1_COL_IDX,
    crypto::merkle::{MerkleStore, MerkleTree, NodeIndex},
    FieldElement, StarkField, AUX_TRACE_RAND_ELEMENTS,
};

// SIBLING TABLE TESTS
// ================================================================================================

#[test]
#[allow(clippy::needless_range_loop)]
fn hasher_p1_mp_verify() {
    let (tree, leaves) = build_merkle_tree();
    let store = MerkleStore::new().with_merkle_tree(leaves).unwrap();
    let node = tree.get_node(NodeIndex::new(3, 1)).unwrap();

    // build program inputs
    let mut init_stack = vec![];
    append_word(&mut init_stack, node);
    init_stack.extend_from_slice(&[3, 1]);
    append_word(&mut init_stack, tree.root());
    init_stack.reverse();
    let stack_inputs = StackInputs::try_from_values(init_stack).unwrap();
    let advice_inputs = AdviceInputs::default().with_merkle_store(store);

    // build execution trace and extract the sibling table column from it
    let ops = vec![Operation::MpVerify];
    let mut trace = build_trace_from_ops_with_inputs(ops, stack_inputs, advice_inputs);
    let alphas = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let aux_columns = trace.build_aux_segment(&[], &alphas).unwrap();
    let p1 = aux_columns.get_column(P1_COL_IDX);

    // executing MPVERIFY does not affect the sibling table - so, all values in the column must be
    // ONE
    for i in 0..(p1.len() - NUM_RAND_ROWS) {
        assert_eq!(ONE, p1[i]);
    }
}

#[test]
#[allow(clippy::needless_range_loop)]
fn hasher_p1_mr_update() {
    let (tree, leaves) = build_merkle_tree();
    let index = 5_u64;
    let old_node = tree.get_node(NodeIndex::new(3, index)).unwrap();
    let new_node = init_leaf(11);
    let path = tree.get_path(NodeIndex::new(3, index)).unwrap();

    // build program inputs
    let mut init_stack = vec![];
    append_word(&mut init_stack, old_node);
    init_stack.extend_from_slice(&[3, index]);
    append_word(&mut init_stack, tree.root());
    append_word(&mut init_stack, new_node);

    init_stack.reverse();
    let stack_inputs = StackInputs::try_from_values(init_stack).unwrap();
    let store = MerkleStore::new().with_merkle_tree(leaves).unwrap();
    let advice_inputs = AdviceInputs::default().with_merkle_store(store);

    // build execution trace and extract the sibling table column from it
    let ops = vec![Operation::MrUpdate];
    let mut trace = build_trace_from_ops_with_inputs(ops, stack_inputs, advice_inputs);
    let alphas = rand_array::<Felt, AUX_TRACE_RAND_ELEMENTS>();
    let aux_columns = trace.build_aux_segment(&[], &alphas).unwrap();
    let p1 = aux_columns.get_column(P1_COL_IDX);

    let row_values = [
        SiblingTableRow::new(Felt::new(index), path[0]).to_value(&trace.main_trace, &alphas),
        SiblingTableRow::new(Felt::new(index >> 1), path[1]).to_value(&trace.main_trace, &alphas),
        SiblingTableRow::new(Felt::new(index >> 2), path[2]).to_value(&trace.main_trace, &alphas),
    ];

    // make sure the first entry is ONE
    let mut expected_value = ONE;
    assert_eq!(expected_value, p1[0]);

    // the running product does not change for the next 7 steps because the hasher computes the
    // hash of the SPAN block
    for i in 1..8 {
        assert_eq!(expected_value, p1[i]);
    }

    // on step 8, computations of the "old Merkle root" is started and the first sibling is added
    // to the table in the following row (step 9)
    expected_value *= row_values[0];
    assert_eq!(expected_value, p1[9]);

    // and then again for the next 7 steps the value remains the same
    for i in 10..17 {
        assert_eq!(expected_value, p1[i]);
    }

    // on step 16, the next sibling is added to the table in the following row (step 17)
    expected_value *= row_values[1];
    assert_eq!(expected_value, p1[17]);

    // and then again for the next 7 steps the value remains the same
    for i in 18..25 {
        assert_eq!(expected_value, p1[i]);
    }

    // on step 24, the last sibling is added to the table in the following row (step 25)
    expected_value *= row_values[2];
    assert_eq!(expected_value, p1[25]);

    // and then again for the next 7 steps the value remains the same
    for i in 25..33 {
        assert_eq!(expected_value, p1[i]);
    }

    // on step 32, computations of the "new Merkle root" is started and the first sibling is
    // removed from the table in the following row (step 33)
    expected_value *= row_values[0].inv();
    assert_eq!(expected_value, p1[33]);

    // then, for the next 7 steps the value remains the same
    for i in 33..41 {
        assert_eq!(expected_value, p1[i]);
    }

    // on step 40, the next sibling is removed from the table in the following row (step 41)
    expected_value *= row_values[1].inv();
    assert_eq!(expected_value, p1[41]);

    // and then again for the next 7 steps the value remains the same
    for i in 41..49 {
        assert_eq!(expected_value, p1[i]);
    }

    // on step 48, the last sibling is removed from the table in the following row (step 49)
    expected_value *= row_values[2].inv();
    assert_eq!(expected_value, p1[49]);

    // at this point the table should be empty again, and it should stay empty until the end
    assert_eq!(expected_value, ONE);
    for i in 50..(p1.len() - NUM_RAND_ROWS) {
        assert_eq!(ONE, p1[i]);
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn build_merkle_tree() -> (MerkleTree, Vec<Word>) {
    // build a Merkle tree
    let leaves = init_leaves(&[1, 2, 3, 4, 5, 6, 7, 8]);
    (MerkleTree::new(leaves.clone()).unwrap(), leaves)
}

fn init_leaves(values: &[u64]) -> Vec<Word> {
    values.iter().map(|&v| init_leaf(v)).collect()
}

fn init_leaf(value: u64) -> Word {
    [Felt::new(value), ZERO, ZERO, ZERO]
}

fn append_word(target: &mut Vec<u64>, word: Word) {
    word.iter().rev().for_each(|v| target.push(v.as_int()));
}
