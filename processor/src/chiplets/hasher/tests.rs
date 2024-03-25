use super::{
    init_state_from_words, Digest, Felt, Hasher, HasherState, MerklePath, Selectors, TraceFragment,
    Word, LINEAR_HASH, MP_VERIFY, MR_UPDATE_NEW, MR_UPDATE_OLD, RETURN_HASH, RETURN_STATE,
    TRACE_WIDTH,
};
use alloc::vec::Vec;

use miden_air::trace::chiplets::hasher::{
    DIGEST_LEN, HASH_CYCLE_LEN, NUM_ROUNDS, NUM_SELECTORS, STATE_COL_RANGE,
};
use test_utils::rand::rand_array;
use vm_core::{
    chiplets::hasher,
    code_blocks::CodeBlock,
    crypto::merkle::{MerkleTree, NodeIndex},
    Operation, ONE, ZERO,
};

// LINEAR HASH TESTS
// ================================================================================================

#[test]
fn hasher_permute() {
    // --- test one permutation -----------------------------------------------

    // initialize the hasher and perform one permutation
    let mut hasher = Hasher::default();
    let init_state: HasherState = rand_array();

    let (addr, final_state) = hasher.permute(init_state);

    // address of the permutation should be ONE (as hasher address starts at ONE)
    assert_eq!(ONE, addr);

    // make sure the result is correct
    let expected_state = apply_permutation(init_state);
    assert_eq!(expected_state, final_state);

    // build the trace
    let trace = build_trace(hasher, 8);

    // make sure the trace is correct
    check_selector_trace(&trace, 0, LINEAR_HASH, RETURN_STATE);
    check_hasher_state_trace(&trace, 0, init_state);
    assert_eq!(trace.last().unwrap(), &[ZERO; 8]);

    // --- test two permutations ----------------------------------------------

    // initialize the hasher and perform two permutations
    let mut hasher = Hasher::default();
    let init_state1: HasherState = rand_array();

    let (addr1, final_state1) = hasher.permute(init_state1);

    let init_state2: HasherState = rand_array();
    let (addr2, final_state2) = hasher.permute(init_state2);

    // make sure the returned addresses are correct (they must be 8 rows apart)
    assert_eq!(ONE, addr1);
    assert_eq!(Felt::new(9), addr2);

    // make sure the results are correct
    let expected_state1 = apply_permutation(init_state1);
    assert_eq!(expected_state1, final_state1);

    let expected_state2 = apply_permutation(init_state2);
    assert_eq!(expected_state2, final_state2);

    // build the trace
    let trace = build_trace(hasher, 16);

    // make sure the trace is correct
    check_selector_trace(&trace, 0, LINEAR_HASH, RETURN_STATE);
    check_selector_trace(&trace, 8, LINEAR_HASH, RETURN_STATE);
    check_hasher_state_trace(&trace, 0, init_state1);
    check_hasher_state_trace(&trace, 8, init_state2);
    assert_eq!(trace.last().unwrap(), &[ZERO; 16]);
}

// MERKLE TREE TESTS
// ================================================================================================

#[test]
fn hasher_build_merkle_root() {
    // --- Merkle tree with 2 leaves ------------------------------------------

    // build a Merkle tree
    let leaves = init_leaves(&[1, 2]);
    let tree = MerkleTree::new(&leaves).unwrap();

    // initialize the hasher and perform two Merkle branch verifications
    let mut hasher = Hasher::default();
    let path0 = tree.get_path(NodeIndex::new(1, 0).unwrap()).unwrap();

    hasher.build_merkle_root(leaves[0], &path0, ZERO);

    let path1 = tree.get_path(NodeIndex::new(1, 1).unwrap()).unwrap();

    hasher.build_merkle_root(leaves[1], &path1, ONE);

    // build the trace
    let trace = build_trace(hasher, 16);

    // make sure the trace is correct
    check_selector_trace(&trace, 0, MP_VERIFY, RETURN_HASH);
    check_selector_trace(&trace, 8, MP_VERIFY, RETURN_HASH);
    check_hasher_state_trace(&trace, 0, init_state_from_words(&leaves[0], &path0[0]));
    check_hasher_state_trace(&trace, 0, init_state_from_words(&path1[0], &leaves[1]));
    let node_idx_column = trace.last().unwrap();
    assert_eq!(&node_idx_column[..8], &[ZERO; 8]);
    assert_eq!(node_idx_column[8], ONE);
    assert_eq!(&node_idx_column[9..], &[ZERO; 7]);

    // --- Merkle tree with 8 leaves ------------------------------------------

    // build a Merkle tree
    let leaves = init_leaves(&[1, 2, 3, 4, 5, 6, 7, 8]);
    let tree = MerkleTree::new(&leaves).unwrap();

    // initialize the hasher and perform one Merkle branch verifications
    let mut hasher = Hasher::default();
    let path = tree.get_path(NodeIndex::new(3, 5).unwrap()).unwrap();
    hasher.build_merkle_root(leaves[5], &path, Felt::new(5));

    // build and check the trace for validity
    let trace = build_trace(hasher, 24);
    check_merkle_path(&trace, 0, leaves[5], &path, 5, MP_VERIFY);

    // --- Merkle tree with 8 leaves (multiple branches) ----------------------

    // initialize the hasher and perform one Merkle branch verifications
    let mut hasher = Hasher::default();

    let path0 = tree.get_path(NodeIndex::new(3, 0).unwrap()).unwrap();

    hasher.build_merkle_root(leaves[0], &path0, ZERO);

    let path3 = tree.get_path(NodeIndex::new(3, 3).unwrap()).unwrap();

    hasher.build_merkle_root(leaves[3], &path3, Felt::new(3));

    let path7 = tree.get_path(NodeIndex::new(3, 7).unwrap()).unwrap();

    hasher.build_merkle_root(leaves[7], &path7, Felt::new(7));

    // path3 again

    hasher.build_merkle_root(leaves[3], &path3, Felt::new(3));

    // build and check the trace for validity
    let trace = build_trace(hasher, 96);
    check_merkle_path(&trace, 0, leaves[0], &path0, 0, MP_VERIFY);
    check_merkle_path(&trace, 24, leaves[3], &path3, 3, MP_VERIFY);
    check_merkle_path(&trace, 48, leaves[7], &path7, 7, MP_VERIFY);
    check_merkle_path(&trace, 72, leaves[3], &path3, 3, MP_VERIFY);
}

#[test]
fn hasher_update_merkle_root() {
    // --- Merkle tree with 2 leaves ------------------------------------------

    // build a Merkle tree
    let leaves = init_leaves(&[1, 2]);
    let mut tree = MerkleTree::new(&leaves).unwrap();

    // initialize the hasher and update both leaves
    let mut hasher = Hasher::default();

    let path0 = tree.get_path(NodeIndex::new(1, 0).unwrap()).unwrap();
    let new_leaf0 = init_leaf(3);

    hasher.update_merkle_root(leaves[0], new_leaf0, &path0, ZERO);
    tree.update_leaf(0, new_leaf0).unwrap();

    let path1 = tree.get_path(NodeIndex::new(1, 1).unwrap()).unwrap();
    let new_leaf1 = init_leaf(4);

    hasher.update_merkle_root(leaves[1], new_leaf1, &path1, ONE);
    tree.update_leaf(1, new_leaf1).unwrap();

    // build the trace
    let trace = build_trace(hasher, 32);

    // make sure the trace is correct
    check_selector_trace(&trace, 0, MR_UPDATE_OLD, RETURN_HASH);
    check_selector_trace(&trace, 8, MR_UPDATE_NEW, RETURN_HASH);
    check_selector_trace(&trace, 16, MR_UPDATE_OLD, RETURN_HASH);
    check_selector_trace(&trace, 24, MR_UPDATE_NEW, RETURN_HASH);
    check_hasher_state_trace(&trace, 0, init_state_from_words(&leaves[0], &path0[0]));
    check_hasher_state_trace(&trace, 8, init_state_from_words(&new_leaf0, &path0[0]));
    check_hasher_state_trace(&trace, 16, init_state_from_words(&path1[0], &leaves[1]));
    check_hasher_state_trace(&trace, 24, init_state_from_words(&path1[0], &new_leaf1));
    let node_idx_column = trace.last().unwrap();
    assert_eq!(&node_idx_column[..16], &[ZERO; 16]);
    assert_eq!(node_idx_column[16], ONE);
    assert_eq!(&node_idx_column[17..24], &[ZERO; 7]);
    assert_eq!(node_idx_column[24], ONE);
    assert_eq!(&node_idx_column[25..], &[ZERO; 7]);

    // --- Merkle tree with 8 leaves ------------------------------------------

    // build a Merkle tree
    let leaves = init_leaves(&[1, 2, 3, 4, 5, 6, 7, 8]);
    let mut tree = MerkleTree::new(&leaves).unwrap();

    // initialize the hasher
    let mut hasher = Hasher::default();

    let path3 = tree.get_path(NodeIndex::new(3, 3).unwrap()).unwrap();
    let new_leaf3 = init_leaf(23);

    hasher.update_merkle_root(leaves[3], new_leaf3, &path3, Felt::new(3));
    tree.update_leaf(3, new_leaf3).unwrap();

    let path6 = tree.get_path(NodeIndex::new(3, 6).unwrap()).unwrap();
    let new_leaf6 = init_leaf(25);
    hasher.update_merkle_root(leaves[6], new_leaf6, &path6, Felt::new(6));
    tree.update_leaf(6, new_leaf6).unwrap();

    // update leaf 3 again
    let path3_2 = tree.get_path(NodeIndex::new(3, 3).unwrap()).unwrap();
    let new_leaf3_2 = init_leaf(27);
    hasher.update_merkle_root(new_leaf3, new_leaf3_2, &path3_2, Felt::new(3));
    tree.update_leaf(3, new_leaf3_2).unwrap();
    assert_ne!(path3, path3_2);

    // build and check the trace for validity
    let trace = build_trace(hasher, 144);
    check_merkle_path(&trace, 0, leaves[3], &path3, 3, MR_UPDATE_OLD);
    check_merkle_path(&trace, 24, new_leaf3, &path3, 3, MR_UPDATE_NEW);
    check_merkle_path(&trace, 48, leaves[6], &path6, 6, MR_UPDATE_OLD);
    check_merkle_path(&trace, 72, new_leaf6, &path6, 6, MR_UPDATE_NEW);
    check_merkle_path(&trace, 96, new_leaf3, &path3_2, 3, MR_UPDATE_OLD);
    check_merkle_path(&trace, 120, new_leaf3_2, &path3_2, 3, MR_UPDATE_NEW);
}

// MEMOIZATION TESTS
// ================================================================================================

#[test]
fn hash_memoization_control_blocks() {
    // --- Join block with 2 same split blocks as children, having the same hasher execution trace.
    //           Join
    //          /    \
    //         /     \
    //        /      \
    //      Split1     Split2 (memoized)

    let t_branch = CodeBlock::new_span(vec![Operation::Push(ZERO)]);
    let f_branch = CodeBlock::new_span(vec![Operation::Push(ONE)]);
    let split1_block = CodeBlock::new_split(t_branch.clone(), f_branch.clone());
    let split2_block = CodeBlock::new_split(t_branch.clone(), f_branch.clone());
    let join_block = CodeBlock::new_join([split1_block.clone(), split2_block.clone()]);

    let mut hasher = Hasher::default();
    let h1: [Felt; DIGEST_LEN] = split1_block
        .hash()
        .as_elements()
        .try_into()
        .expect("Could not convert slice to array");
    let h2: [Felt; DIGEST_LEN] = split2_block
        .hash()
        .as_elements()
        .try_into()
        .expect("Could not convert slice to array");

    let expected_hash = join_block.hash();

    // builds the trace of the join block.
    let (_, final_state) = hasher.hash_control_block(h1, h2, join_block.domain(), expected_hash);

    // make sure the hash of the final state is the same as the expected hash.
    assert_eq!(Digest::new(final_state), expected_hash);

    let h1: [Felt; DIGEST_LEN] = t_branch
        .hash()
        .as_elements()
        .try_into()
        .expect("Could not convert slice to array");
    let h2: [Felt; DIGEST_LEN] = f_branch
        .hash()
        .as_elements()
        .try_into()
        .expect("Could not convert slice to array");

    let expected_hash = split1_block.hash();

    // builds the hash execution trace of the first split block from scratch.
    let (addr, final_state) =
        hasher.hash_control_block(h1, h2, split1_block.domain(), expected_hash);

    let first_block_final_state = final_state;

    // make sure the hash of the final state of the first split block is the same as the expected
    // hash.
    assert_eq!(Digest::new(final_state), expected_hash);

    let start_row = addr.as_int() as usize - 1;
    let end_row = hasher.trace_len() - 1;

    let h1: [Felt; DIGEST_LEN] = t_branch
        .hash()
        .as_elements()
        .try_into()
        .expect("Could not convert slice to array");
    let h2: [Felt; DIGEST_LEN] = f_branch
        .hash()
        .as_elements()
        .try_into()
        .expect("Could not convert slice to array");
    let expected_hash = split2_block.hash();

    // builds the hash execution trace of the second split block by copying it from the trace of
    // the first split block.
    let (addr, final_state) =
        hasher.hash_control_block(h1, h2, split2_block.domain(), expected_hash);

    // make sure the hash of the final state of the second split block is the same as the expected
    // hash.
    assert_eq!(Digest::new(final_state), expected_hash);
    // make sure the hash of the first and second split blocks is the same.
    assert_eq!(first_block_final_state, final_state);

    let copied_start_row = addr.as_int() as usize - 1;
    let copied_end_row = hasher.trace_len() - 1;

    let trace = build_trace(hasher, copied_end_row + 1);

    //  check the row address at which memoized block starts.
    let hash_cycle_len: u64 = HASH_CYCLE_LEN.try_into().expect("Could not convert usize to u64");
    assert_eq!(Felt::new(hash_cycle_len * 2 + 1), addr);
    // check the trace length of the final trace.
    assert_eq!(trace.last().unwrap(), &[ZERO; HASH_CYCLE_LEN * 3]);

    // check correct copy of the memoized trace.
    check_memoized_trace(&trace, start_row, end_row, copied_start_row, copied_end_row);
}

#[test]
fn hash_memoization_span_blocks() {
    // --- span block with 1 batch ----------------------------------------------------------------
    let span_block = CodeBlock::new_span(vec![Operation::Push(Felt::new(10)), Operation::Drop]);

    hash_memoization_span_blocks_check(span_block);

    // --- span block with multiple batches -------------------------------------------------------
    let span_block = CodeBlock::new_span(vec![
        Operation::Push(ONE),
        Operation::Push(Felt::new(2)),
        Operation::Push(Felt::new(3)),
        Operation::Push(Felt::new(4)),
        Operation::Push(Felt::new(5)),
        Operation::Push(Felt::new(6)),
        Operation::Push(Felt::new(7)),
        Operation::Push(Felt::new(8)),
        Operation::Push(Felt::new(9)),
        Operation::Push(Felt::new(10)),
        Operation::Push(Felt::new(11)),
        Operation::Push(Felt::new(12)),
        Operation::Push(Felt::new(13)),
        Operation::Push(Felt::new(14)),
        Operation::Push(Felt::new(15)),
        Operation::Push(Felt::new(16)),
        Operation::Push(Felt::new(17)),
        Operation::Push(Felt::new(18)),
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
    ]);

    hash_memoization_span_blocks_check(span_block);
}

fn hash_memoization_span_blocks_check(span_block: CodeBlock) {
    // Join block with a join and span block as children. The span child of the first join
    // child block is the same as the span child of root join block. Here the hash execution
    // trace of the second span block is built by copying the trace built for the first same
    // span block.
    //           Join1
    //          /    \
    //         /     \
    //        /      \
    //      Join2     Span2 (memoized)
    //       / \
    //      /   \
    //     /     \
    //  Span1   Loop

    let span1_block = span_block.clone();
    let loop_body = CodeBlock::new_span(vec![Operation::Pad, Operation::Eq, Operation::Not]);
    let loop_block = CodeBlock::new_loop(loop_body);
    let join2_block = CodeBlock::new_join([span1_block.clone(), loop_block.clone()]);
    let span2_block = span_block;
    let join1_block = CodeBlock::new_join([join2_block.clone(), span2_block.clone()]);

    let mut hasher = Hasher::default();
    let h1: [Felt; DIGEST_LEN] = join2_block
        .hash()
        .as_elements()
        .try_into()
        .expect("Could not convert slice to array");
    let h2: [Felt; DIGEST_LEN] = span2_block
        .hash()
        .as_elements()
        .try_into()
        .expect("Could not convert slice to array");
    let expected_hash = join1_block.hash();

    // builds the trace of the Join1 block.
    let (_, final_state) = hasher.hash_control_block(h1, h2, join1_block.domain(), expected_hash);

    // make sure the hash of the final state of Join1 is the same as the expected hash.
    assert_eq!(Digest::new(final_state), expected_hash);

    let h1: [Felt; DIGEST_LEN] = span1_block
        .hash()
        .as_elements()
        .try_into()
        .expect("Could not convert slice to array");
    let h2: [Felt; DIGEST_LEN] = loop_block
        .hash()
        .as_elements()
        .try_into()
        .expect("Could not convert slice to array");
    let expected_hash = join2_block.hash();

    let (_, final_state) = hasher.hash_control_block(h1, h2, join2_block.domain(), expected_hash);

    // make sure the hash of the final state of Join2 is the same as the expected hash.
    assert_eq!(Digest::new(final_state), expected_hash);

    let span1_block_val = if let CodeBlock::Span(span) = span1_block.clone() {
        span
    } else {
        unreachable!()
    };

    // builds the hash execution trace of the first span block from scratch.
    let (addr, final_state) =
        hasher.hash_span_block(span1_block_val.op_batches(), span1_block.hash());

    let _num_batches = span1_block_val.op_batches().len();

    let first_span_block_final_state = final_state;

    // make sure the hash of the final state of Span1 block is the same as the expected hash.
    let expected_hash = span1_block.hash();
    assert_eq!(Digest::new(final_state), expected_hash);

    let start_row = addr.as_int() as usize - 1;
    let end_row = hasher.trace_len() - 1;

    let span2_block_val = if let CodeBlock::Span(span) = span2_block.clone() {
        span
    } else {
        unreachable!()
    };

    // builds the hash execution trace of the second span block by copying the sections of the
    // trace corresponding to the first span block with the same hash.
    let (addr, final_state) =
        hasher.hash_span_block(span2_block_val.op_batches(), span2_block.hash());

    let _num_batches = span2_block_val.op_batches().len();

    let expected_hash = span2_block.hash();
    // make sure the hash of the final state of Span2 block is the same as the expected hash.
    assert_eq!(Digest::new(final_state), expected_hash);

    // make sure the hash of the first and second span blocks is the same.
    assert_eq!(first_span_block_final_state, final_state);

    let copied_start_row = addr.as_int() as usize - 1;
    let copied_end_row = hasher.trace_len() - 1;

    let trace = build_trace(hasher, copied_end_row + 1);

    // check correct copy after memoization
    check_memoized_trace(&trace, start_row, end_row, copied_start_row, copied_end_row);
}

// HELPER FUNCTIONS
// ================================================================================================

/// Builds an execution trace for the provided hasher. The trace must have the number of rows
/// specified by num_rows.
fn build_trace(hasher: Hasher, num_rows: usize) -> Vec<Vec<Felt>> {
    let mut trace = (0..TRACE_WIDTH).map(|_| vec![ZERO; num_rows]).collect::<Vec<_>>();
    let mut fragment = TraceFragment::trace_to_fragment(&mut trace);
    hasher.fill_trace(&mut fragment);
    trace
}

/// Makes sure that the provided trace is consistent with verifying the specified Merkle path
/// in the context defined by init_selectors.
fn check_merkle_path(
    trace: &[Vec<Felt>],
    row_idx: usize,
    leaf: Word,
    path: &MerklePath,
    node_index: u64,
    init_selectors: Selectors,
) {
    // make sure selectors were set correctly
    let mid_selectors = [ZERO, init_selectors[1], init_selectors[2]];
    check_selector_trace(trace, row_idx, init_selectors, init_selectors);
    for i in 1..path.len() - 1 {
        check_selector_trace(trace, row_idx + i * 8, mid_selectors, init_selectors);
    }
    let last_perm_row_addr = row_idx + (path.len() - 1) * 8;
    check_selector_trace(trace, last_perm_row_addr, mid_selectors, RETURN_HASH);

    // make sure hasher states are correct
    let mut root = leaf;
    for (i, &node) in path.iter().enumerate() {
        let index_bit = (node_index >> i) & 1;
        let old_root = root;
        let init_state = if index_bit == 0 {
            root = hasher::merge(&[root.into(), node]).into();
            init_state_from_words(&old_root, &node)
        } else {
            root = hasher::merge(&[node, root.into()]).into();
            init_state_from_words(&node, &old_root)
        };
        check_hasher_state_trace(trace, row_idx + i * 8, init_state);
    }

    // make sure node index is set correctly
    let node_idx_column = trace.last().unwrap();
    assert_eq!(Felt::new(node_index), node_idx_column[row_idx]);
    let mut node_index = node_index >> 1;
    for i in 1..8 {
        assert_eq!(Felt::new(node_index), node_idx_column[row_idx + i])
    }

    for i in 1..path.len() {
        node_index >>= 1;
        for j in 0..8 {
            assert_eq!(Felt::new(node_index), node_idx_column[row_idx + i * 8 + j])
        }
    }
}

/// Makes sure that selector columns (columns 0, 1, 2) are valid for an 8-row cycle starting
/// with row_idx.
fn check_selector_trace(
    trace: &[Vec<Felt>],
    row_idx: usize,
    init_selectors: Selectors,
    final_selectors: Selectors,
) {
    let trace = &trace[0..3];
    let mid_selectors = [ZERO, init_selectors[1], init_selectors[2]];

    assert_row_equal(trace, row_idx, &init_selectors);
    for i in 0..NUM_ROUNDS - 1 {
        assert_row_equal(trace, row_idx + i + 1, &mid_selectors);
    }
    assert_row_equal(trace, row_idx + NUM_ROUNDS, &final_selectors);
}

/// Makes sure hasher state columns (columns 4 through 15) are valid for an 8-row cycle starting
/// with row_idx.
fn check_hasher_state_trace(trace: &[Vec<Felt>], row_idx: usize, init_state: HasherState) {
    let trace = &trace[STATE_COL_RANGE];
    let mut state = init_state;

    assert_row_equal(trace, row_idx, &state);
    for i in 0..NUM_ROUNDS {
        hasher::apply_round(&mut state, i);
        assert_row_equal(trace, row_idx + i + 1, &state);
    }
}

/// Makes sure that the trace is copied correctly on memoization
fn check_memoized_trace(
    trace: &[Vec<Felt>],
    start_row: usize,
    end_row: usize,
    copied_start_row: usize,
    copied_end_row: usize,
) {
    // make sure the number of copied rows are equal as the original.
    assert_eq!(end_row - start_row, copied_end_row - copied_start_row);

    // make sure selector trace is copied correctly
    let selector_trace = &trace[0..NUM_SELECTORS];
    for column in selector_trace.iter() {
        assert_eq!(column[start_row..end_row], column[copied_start_row..copied_end_row])
    }

    // make sure hasher state trace is copied correctly
    let hasher_state_trace = &trace[STATE_COL_RANGE];
    for column in hasher_state_trace.iter() {
        assert_eq!(column[start_row..end_row], column[copied_start_row..copied_end_row])
    }
}

/// Makes sure that a row in the provided trace is equal to the provided values at the specified
/// row index.
fn assert_row_equal(trace: &[Vec<Felt>], row_idx: usize, values: &[Felt]) {
    for (column, &value) in trace.iter().zip(values.iter()) {
        assert_eq!(column[row_idx], value);
    }
}

fn apply_permutation(mut state: HasherState) -> HasherState {
    hasher::apply_permutation(&mut state);
    state
}

fn init_leaves(values: &[u64]) -> Vec<Word> {
    values.iter().map(|&v| init_leaf(v)).collect()
}

fn init_leaf(value: u64) -> Word {
    [Felt::new(value), ZERO, ZERO, ZERO]
}
