use super::{
    Felt, FieldElement, Hasher, HasherState, Selectors, TraceFragment, Word, LINEAR_HASH,
    MP_VERIFY, MR_UPDATE_NEW, MR_UPDATE_OLD, RETURN_HASH, RETURN_STATE, TRACE_WIDTH,
};
use rand_utils::rand_array;
use vm_core::{
    hasher::{self, NUM_ROUNDS},
    AdviceSet,
};

// LINEAR HASH TESTS
// ================================================================================================

#[test]
fn hasher_permute() {
    // --- test one permutation -----------------------------------------------

    // initialize the hasher and perform one permutation
    let mut hasher = Hasher::new();
    let init_state: HasherState = rand_array();
    let (addr, final_state) = hasher.permute(init_state);

    // address of the permutation should be ONE (as hasher address starts at ONE)
    assert_eq!(Felt::ONE, addr);

    // make sure the result is correct
    let expected_state = apply_permutation(init_state);
    assert_eq!(expected_state, final_state);

    // build the trace
    let trace = build_trace(hasher, 8);

    // make sure the trace is correct
    check_row_addr_trace(&trace);
    check_selector_trace(&trace, 0, LINEAR_HASH, RETURN_STATE);
    check_hasher_state_trace(&trace, 0, init_state);
    assert_eq!(trace.last().unwrap(), &[Felt::ZERO; 8]);

    // --- test two permutations ----------------------------------------------

    // initialize the hasher and perform two permutations
    let mut hasher = Hasher::new();
    let init_state1: HasherState = rand_array();
    let (addr1, final_state1) = hasher.permute(init_state1);

    let init_state2: HasherState = rand_array();
    let (addr2, final_state2) = hasher.permute(init_state2);

    // make sure the returned addresses are correct (they must be 8 rows apart)
    assert_eq!(Felt::ONE, addr1);
    assert_eq!(Felt::new(9), addr2);

    // make sure the results are correct
    let expected_state1 = apply_permutation(init_state1);
    assert_eq!(expected_state1, final_state1);

    let expected_state2 = apply_permutation(init_state2);
    assert_eq!(expected_state2, final_state2);

    // build the trace
    let trace = build_trace(hasher, 16);

    // make sure the trace is correct
    check_row_addr_trace(&trace);
    check_selector_trace(&trace, 0, LINEAR_HASH, RETURN_STATE);
    check_selector_trace(&trace, 8, LINEAR_HASH, RETURN_STATE);
    check_hasher_state_trace(&trace, 0, init_state1);
    check_hasher_state_trace(&trace, 8, init_state2);
    assert_eq!(trace.last().unwrap(), &[Felt::ZERO; 16]);
}

// MERKLE TREE TESTS
// ================================================================================================

#[test]
fn hasher_build_merkle_root() {
    // --- Merkle tree with 2 leaves ------------------------------------------

    // build a Merkle tree
    let leaves = inti_leaves(&[1, 2]);
    let tree = AdviceSet::new_merkle_tree(leaves.to_vec()).unwrap();

    // initialize the hasher and perform two Merkle branch verifications
    let mut hasher = Hasher::new();
    let path0 = tree.get_path(1, 0).unwrap();
    hasher.build_merkle_root(leaves[0], &path0, Felt::ZERO);
    let path1 = tree.get_path(1, 1).unwrap();
    hasher.build_merkle_root(leaves[1], &path1, Felt::ONE);

    // build the trace
    let trace = build_trace(hasher, 16);

    // make sure the trace is correct
    check_row_addr_trace(&trace);
    check_selector_trace(&trace, 0, MP_VERIFY, RETURN_HASH);
    check_selector_trace(&trace, 8, MP_VERIFY, RETURN_HASH);
    check_hasher_state_trace(&trace, 0, hasher_merge_state(leaves[0], path0[0]));
    check_hasher_state_trace(&trace, 0, hasher_merge_state(path1[0], leaves[1]));
    let node_idx_column = trace.last().unwrap();
    assert_eq!(&node_idx_column[..8], &[Felt::ZERO; 8]);
    assert_eq!(node_idx_column[8], Felt::ONE);
    assert_eq!(&node_idx_column[9..], &[Felt::ZERO; 7]);

    // --- Merkle tree with 8 leaves ------------------------------------------

    // build a Merkle tree
    let leaves = inti_leaves(&[1, 2, 3, 4, 5, 6, 7, 8]);
    let tree = AdviceSet::new_merkle_tree(leaves.to_vec()).unwrap();

    // initialize the hasher and perform one Merkle branch verifications
    let mut hasher = Hasher::new();
    let path = tree.get_path(3, 5).unwrap();
    hasher.build_merkle_root(leaves[5], &path, Felt::new(5));

    // build and check the trace for validity
    let trace = build_trace(hasher, 24);
    check_merkle_path(&trace, 0, leaves[5], &path, 5, MP_VERIFY);

    // --- Merkle tree with 8 leaves (multiple branches) ----------------------

    // initialize the hasher and perform one Merkle branch verifications
    let mut hasher = Hasher::new();

    let path0 = tree.get_path(3, 0).unwrap();
    hasher.build_merkle_root(leaves[0], &path0, Felt::ZERO);

    let path3 = tree.get_path(3, 3).unwrap();
    hasher.build_merkle_root(leaves[3], &path3, Felt::new(3));

    let path7 = tree.get_path(3, 7).unwrap();
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
    let leaves = inti_leaves(&[1, 2]);
    let mut tree = AdviceSet::new_merkle_tree(leaves.to_vec()).unwrap();

    // initialize the hasher and update both leaves
    let mut hasher = Hasher::new();

    let path0 = tree.get_path(1, 0).unwrap();
    let new_leaf0 = init_leaf(3);
    hasher.update_merkle_root(leaves[0], new_leaf0, &path0, Felt::ZERO);
    tree.update_leaf(0, new_leaf0).unwrap();

    let path1 = tree.get_path(1, 1).unwrap();
    let new_leaf1 = init_leaf(4);
    hasher.update_merkle_root(leaves[1], new_leaf1, &path1, Felt::ONE);
    tree.update_leaf(1, new_leaf1).unwrap();

    // build the trace
    let trace = build_trace(hasher, 32);

    // make sure the trace is correct
    check_row_addr_trace(&trace);
    check_selector_trace(&trace, 0, MR_UPDATE_OLD, RETURN_HASH);
    check_selector_trace(&trace, 8, MR_UPDATE_NEW, RETURN_HASH);
    check_selector_trace(&trace, 16, MR_UPDATE_OLD, RETURN_HASH);
    check_selector_trace(&trace, 24, MR_UPDATE_NEW, RETURN_HASH);
    check_hasher_state_trace(&trace, 0, hasher_merge_state(leaves[0], path0[0]));
    check_hasher_state_trace(&trace, 8, hasher_merge_state(new_leaf0, path0[0]));
    check_hasher_state_trace(&trace, 16, hasher_merge_state(path1[0], leaves[1]));
    check_hasher_state_trace(&trace, 24, hasher_merge_state(path1[0], new_leaf1));
    let node_idx_column = trace.last().unwrap();
    assert_eq!(&node_idx_column[..16], &[Felt::ZERO; 16]);
    assert_eq!(node_idx_column[16], Felt::ONE);
    assert_eq!(&node_idx_column[17..24], &[Felt::ZERO; 7]);
    assert_eq!(node_idx_column[24], Felt::ONE);
    assert_eq!(&node_idx_column[25..], &[Felt::ZERO; 7]);

    // --- Merkle tree with 8 leaves ------------------------------------------

    // build a Merkle tree
    let leaves = inti_leaves(&[1, 2, 3, 4, 5, 6, 7, 8]);
    let mut tree = AdviceSet::new_merkle_tree(leaves.to_vec()).unwrap();

    // initialize the hasher
    let mut hasher = Hasher::new();

    let path3 = tree.get_path(3, 3).unwrap();
    let new_leaf3 = init_leaf(23);
    hasher.update_merkle_root(leaves[3], new_leaf3, &path3, Felt::new(3));
    tree.update_leaf(3, new_leaf3).unwrap();

    let path6 = tree.get_path(3, 6).unwrap();
    let new_leaf6 = init_leaf(25);
    hasher.update_merkle_root(leaves[6], new_leaf6, &path6, Felt::new(6));
    tree.update_leaf(6, new_leaf6).unwrap();

    // update leaf 3 again
    let path3_2 = tree.get_path(3, 3).unwrap();
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

// HELPER FUNCTIONS
// ================================================================================================

/// Builds an execution trace for the provided hasher. The trace must have the number of rows
/// specified by num_rows.
fn build_trace(hasher: Hasher, num_rows: usize) -> Vec<Vec<Felt>> {
    let mut trace = (0..TRACE_WIDTH)
        .map(|_| vec![Felt::new(0); num_rows])
        .collect::<Vec<_>>();
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
    path: &[Word],
    node_index: u64,
    init_selectors: Selectors,
) {
    // make sure row address is correct
    check_row_addr_trace(&trace);

    // make sure selectors were set correctly
    let mid_selectors = [Felt::ZERO, init_selectors[1], init_selectors[2]];
    check_selector_trace(&trace, row_idx, init_selectors, init_selectors);
    for i in 1..path.len() - 1 {
        check_selector_trace(&trace, row_idx + i * 8, mid_selectors, init_selectors);
    }
    let last_perm_row_addr = row_idx + (path.len() - 1) * 8;
    check_selector_trace(&trace, last_perm_row_addr, mid_selectors, RETURN_HASH);

    // make sure hasher states are correct
    let mut root = leaf;
    for (i, &node) in path.iter().enumerate() {
        let index_bit = (node_index >> i) & 1;
        let old_root = root;
        let init_state = if index_bit == 0 {
            root = hasher::merge(&[root.into(), node.into()]).into();
            hasher_merge_state(old_root, node)
        } else {
            root = hasher::merge(&[node.into(), root.into()]).into();
            hasher_merge_state(node, old_root)
        };
        check_hasher_state_trace(&trace, row_idx + i * 8, init_state);
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

/// Makes sure that values in the row address column (column 3) start out at 1 and are incremented
/// by 1 with every row.
fn check_row_addr_trace(trace: &[Vec<Felt>]) {
    for (i, &addr) in trace[3].iter().enumerate() {
        assert_eq!(Felt::new(i as u64 + 1), addr);
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
    let mid_selectors = [Felt::ZERO, init_selectors[1], init_selectors[2]];

    assert_row_equal(trace, row_idx, &init_selectors);
    for i in 0..NUM_ROUNDS - 1 {
        assert_row_equal(trace, row_idx + i + 1, &mid_selectors);
    }
    assert_row_equal(trace, row_idx + NUM_ROUNDS, &final_selectors);
}

/// Makes sure hasher state columns (columns 4 through 15) are valid for an 8-row cycle starting
/// with row_idx.
fn check_hasher_state_trace(trace: &[Vec<Felt>], row_idx: usize, init_state: HasherState) {
    let trace = &trace[4..16];
    let mut state = init_state;

    assert_row_equal(trace, row_idx, &state);
    for i in 0..NUM_ROUNDS {
        hasher::apply_round(&mut state, i);
        assert_row_equal(trace, row_idx + i + 1, &state);
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
    vm_core::hasher::apply_permutation(&mut state);
    state
}

fn inti_leaves(values: &[u64]) -> Vec<Word> {
    values.iter().map(|&v| init_leaf(v)).collect()
}

fn init_leaf(value: u64) -> Word {
    [Felt::new(value), Felt::ZERO, Felt::ZERO, Felt::ZERO]
}

fn hasher_merge_state(a: Word, b: Word) -> HasherState {
    [
        Felt::new(8),
        Felt::ZERO,
        Felt::ZERO,
        Felt::ZERO,
        a[0],
        a[1],
        a[2],
        a[3],
        b[0],
        b[1],
        b[2],
        b[3],
    ]
}
