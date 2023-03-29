use super::{
    init_state_from_words, AuxTraceBuilder, Digest, Felt, Hasher, HasherState, Selectors,
    SiblingTableRow, SiblingTableUpdate, TraceFragment, Vec, Word, LINEAR_HASH, MP_VERIFY,
    MR_UPDATE_NEW, MR_UPDATE_OLD, RETURN_HASH, RETURN_STATE, TRACE_WIDTH,
};
use crate::chiplets::hasher::{lookups::HasherLookupContext, HasherLookup};
use rand_utils::rand_array;
use vm_core::{
    chiplets::hasher::{
        self, DIGEST_LEN, HASH_CYCLE_LEN, LINEAR_HASH_LABEL, MP_VERIFY_LABEL, MR_UPDATE_NEW_LABEL,
        MR_UPDATE_OLD_LABEL, NUM_ROUNDS, NUM_SELECTORS, RETURN_HASH_LABEL, RETURN_STATE_LABEL,
        STATE_COL_RANGE,
    },
    code_blocks::CodeBlock,
    crypto::merkle::{MerkleTree, NodeIndex},
    Operation, StarkField, ONE, ZERO,
};

// LINEAR HASH TESTS
// ================================================================================================

#[test]
fn hasher_permute() {
    // --- test one permutation -----------------------------------------------

    // initialize the hasher and perform one permutation
    let mut hasher = Hasher::default();
    let init_state: HasherState = rand_array();
    let mut lookups = Vec::new();
    let (addr, final_state) = hasher.permute(init_state, &mut lookups);

    let lookup_start_addr = 1;
    // there should be two lookups for start and end rows of hasher operation
    let expected_lookups_len = 2;
    // make sure the lookups have correct labels, addresses, indices and contexts.
    let expected_lookup_start =
        HasherLookup::new(LINEAR_HASH_LABEL, lookup_start_addr, ZERO, HasherLookupContext::Start);

    let expected_lookup_end = HasherLookup::new(
        RETURN_STATE_LABEL,
        lookup_start_addr + HASH_CYCLE_LEN as u32 - 1,
        ZERO,
        HasherLookupContext::Return,
    );
    check_lookups_validity(
        lookups,
        expected_lookups_len,
        vec![expected_lookup_start, expected_lookup_end],
    );

    // address of the permutation should be ONE (as hasher address starts at ONE)
    assert_eq!(ONE, addr);

    // make sure the result is correct
    let expected_state = apply_permutation(init_state);
    assert_eq!(expected_state, final_state);

    // build the trace
    let (trace, aux_hints) = build_trace(hasher, 8);

    // make sure the trace is correct
    check_row_addr_trace(&trace);
    check_selector_trace(&trace, 0, LINEAR_HASH, RETURN_STATE);
    check_hasher_state_trace(&trace, 0, init_state);
    assert_eq!(trace.last().unwrap(), &[ZERO; 8]);

    // make sure aux hints for sibling table are empty
    assert!(aux_hints.sibling_hints.is_empty());
    assert!(aux_hints.sibling_rows.is_empty());

    // --- test two permutations ----------------------------------------------

    // initialize the hasher and perform two permutations
    let mut hasher = Hasher::default();
    let init_state1: HasherState = rand_array();
    let mut lookups1 = Vec::new();
    let (addr1, final_state1) = hasher.permute(init_state1, &mut lookups1);

    let mut lookups2 = Vec::new();
    let init_state2: HasherState = rand_array();
    let (addr2, final_state2) = hasher.permute(init_state2, &mut lookups2);

    // make sure the returned addresses are correct (they must be 8 rows apart)
    assert_eq!(ONE, addr1);
    assert_eq!(Felt::new(9), addr2);

    // make sure the results are correct
    let expected_state1 = apply_permutation(init_state1);
    assert_eq!(expected_state1, final_state1);

    let expected_state2 = apply_permutation(init_state2);
    assert_eq!(expected_state2, final_state2);

    // build the trace
    let (trace, aux_hints) = build_trace(hasher, 16);

    // make sure the trace is correct
    check_row_addr_trace(&trace);
    check_selector_trace(&trace, 0, LINEAR_HASH, RETURN_STATE);
    check_selector_trace(&trace, 8, LINEAR_HASH, RETURN_STATE);
    check_hasher_state_trace(&trace, 0, init_state1);
    check_hasher_state_trace(&trace, 8, init_state2);
    assert_eq!(trace.last().unwrap(), &[ZERO; 16]);

    // make sure aux hints for sibling table are empty
    assert!(aux_hints.sibling_hints.is_empty());
    assert!(aux_hints.sibling_rows.is_empty());
}

// MERKLE TREE TESTS
// ================================================================================================

#[test]
fn hasher_build_merkle_root() {
    // --- Merkle tree with 2 leaves ------------------------------------------

    // build a Merkle tree
    let leaves = init_leaves(&[1, 2]);
    let tree = MerkleTree::new(leaves.to_vec()).unwrap();

    // initialize the hasher and perform two Merkle branch verifications
    let mut hasher = Hasher::default();
    let path0 = tree.get_path(NodeIndex::new(1, 0)).unwrap();
    let mut lookups = Vec::new();
    hasher.build_merkle_root(leaves[0], &path0, ZERO, &mut lookups);

    // there should be two lookups for start and end rows of hasher operation
    let expected_lookups_len = 2;
    // make sure the lookups have correct labels, addresses, indices and contexts.
    let lookup_start_addr = 1;
    let expected_lookup_start =
        HasherLookup::new(MP_VERIFY_LABEL, lookup_start_addr, ZERO, HasherLookupContext::Start);
    let expected_lookup_end = HasherLookup::new(
        RETURN_HASH_LABEL,
        lookup_start_addr + HASH_CYCLE_LEN as u32 - 1,
        ZERO,
        HasherLookupContext::Return,
    );
    check_lookups_validity(
        lookups,
        expected_lookups_len,
        vec![expected_lookup_start, expected_lookup_end],
    );

    let path1 = tree.get_path(NodeIndex::new(1, 1)).unwrap();
    let mut lookups = Vec::new();
    hasher.build_merkle_root(leaves[1], &path1, ONE, &mut lookups);

    let lookup_start_addr = 9;
    // there should be two lookups for start and end rows of hasher operation
    let expected_lookups_len = 2;
    // make sure the lookups have correct labels, addresses, indices and contexts.
    let expected_lookup_start =
        HasherLookup::new(MP_VERIFY_LABEL, lookup_start_addr, ONE, HasherLookupContext::Start);
    let expected_lookup_end = HasherLookup::new(
        RETURN_HASH_LABEL,
        lookup_start_addr + HASH_CYCLE_LEN as u32 - 1,
        ZERO,
        HasherLookupContext::Return,
    );
    check_lookups_validity(
        lookups,
        expected_lookups_len,
        vec![expected_lookup_start, expected_lookup_end],
    );

    // build the trace
    let (trace, aux_hints) = build_trace(hasher, 16);

    // make sure the trace is correct
    check_row_addr_trace(&trace);
    check_selector_trace(&trace, 0, MP_VERIFY, RETURN_HASH);
    check_selector_trace(&trace, 8, MP_VERIFY, RETURN_HASH);
    check_hasher_state_trace(&trace, 0, init_state_from_words(&leaves[0], &path0[0]));
    check_hasher_state_trace(&trace, 0, init_state_from_words(&path1[0], &leaves[1]));
    let node_idx_column = trace.last().unwrap();
    assert_eq!(&node_idx_column[..8], &[ZERO; 8]);
    assert_eq!(node_idx_column[8], ONE);
    assert_eq!(&node_idx_column[9..], &[ZERO; 7]);

    // make sure aux hints for sibling table are empty
    assert!(aux_hints.sibling_hints.is_empty());
    assert!(aux_hints.sibling_rows.is_empty());

    // --- Merkle tree with 8 leaves ------------------------------------------

    // build a Merkle tree
    let leaves = init_leaves(&[1, 2, 3, 4, 5, 6, 7, 8]);
    let tree = MerkleTree::new(leaves.to_vec()).unwrap();

    // initialize the hasher and perform one Merkle branch verifications
    let mut hasher = Hasher::default();
    let path = tree.get_path(NodeIndex::new(3, 5)).unwrap();
    let mut lookups = Vec::new();
    hasher.build_merkle_root(leaves[5], &path, Felt::new(5), &mut lookups);

    let lookup_start_addr = 1;
    // there should be two lookups for start and end rows of hasher operation
    let expected_lookups_len = 2;
    // make sure the lookups have correct labels, addresses, indices and contexts.
    let expected_lookup_start = HasherLookup::new(
        MP_VERIFY_LABEL,
        lookup_start_addr,
        Felt::new(5),
        HasherLookupContext::Start,
    );
    let expected_lookup_end = HasherLookup::new(
        RETURN_HASH_LABEL,
        lookup_start_addr + 3 * HASH_CYCLE_LEN as u32 - 1,
        ZERO,
        HasherLookupContext::Return,
    );
    check_lookups_validity(
        lookups,
        expected_lookups_len,
        vec![expected_lookup_start, expected_lookup_end],
    );

    // build and check the trace for validity
    let (trace, aux_hints) = build_trace(hasher, 24);
    check_merkle_path(&trace, 0, leaves[5], &path, 5, MP_VERIFY);

    // make sure aux hints for sibling table are empty
    assert!(aux_hints.sibling_hints.is_empty());
    assert!(aux_hints.sibling_rows.is_empty());

    // --- Merkle tree with 8 leaves (multiple branches) ----------------------

    // initialize the hasher and perform one Merkle branch verifications
    let mut hasher = Hasher::default();

    let path0 = tree.get_path(NodeIndex::new(3, 0)).unwrap();
    let mut lookups = Vec::new();
    hasher.build_merkle_root(leaves[0], &path0, ZERO, &mut lookups);

    let lookup_start_addr = 1;
    // there should be two lookups for start and end rows of hasher operation
    let expected_lookups_len = 2;
    // make sure the lookups have correct labels, addresses, indices and contexts.
    let expected_lookup_start =
        HasherLookup::new(MP_VERIFY_LABEL, lookup_start_addr, ZERO, HasherLookupContext::Start);
    let expected_lookup_end = HasherLookup::new(
        RETURN_HASH_LABEL,
        lookup_start_addr + 3 * HASH_CYCLE_LEN as u32 - 1,
        ZERO,
        HasherLookupContext::Return,
    );
    check_lookups_validity(
        lookups,
        expected_lookups_len,
        vec![expected_lookup_start, expected_lookup_end],
    );

    let path3 = tree.get_path(NodeIndex::new(3, 3)).unwrap();
    let mut lookups = Vec::new();
    hasher.build_merkle_root(leaves[3], &path3, Felt::new(3), &mut lookups);

    let lookup_start_addr = 25;
    // there should be two lookups for start and end rows of hasher operation
    let expected_lookups_len = 2;
    // make sure the lookups have correct labels, addresses, indices and contexts.
    let expected_lookup_start = HasherLookup::new(
        MP_VERIFY_LABEL,
        lookup_start_addr,
        Felt::new(3),
        HasherLookupContext::Start,
    );
    let expected_lookup_end = HasherLookup::new(
        RETURN_HASH_LABEL,
        lookup_start_addr + 3 * HASH_CYCLE_LEN as u32 - 1,
        ZERO,
        HasherLookupContext::Return,
    );
    check_lookups_validity(
        lookups,
        expected_lookups_len,
        vec![expected_lookup_start, expected_lookup_end],
    );

    let path7 = tree.get_path(NodeIndex::new(3, 7)).unwrap();
    let mut lookups = Vec::new();
    hasher.build_merkle_root(leaves[7], &path7, Felt::new(7), &mut lookups);

    let lookup_start_addr = 49;
    // there should be two lookups for start and end rows of hasher operation
    let expected_lookups_len = 2;
    // make sure the lookups have correct labels, addresses, indices and contexts.
    let expected_lookup_start = HasherLookup::new(
        MP_VERIFY_LABEL,
        lookup_start_addr,
        Felt::new(7),
        HasherLookupContext::Start,
    );
    let expected_lookup_end = HasherLookup::new(
        RETURN_HASH_LABEL,
        lookup_start_addr + 3 * HASH_CYCLE_LEN as u32 - 1,
        ZERO,
        HasherLookupContext::Return,
    );
    check_lookups_validity(
        lookups,
        expected_lookups_len,
        vec![expected_lookup_start, expected_lookup_end],
    );

    // path3 again
    let mut lookups = Vec::new();
    hasher.build_merkle_root(leaves[3], &path3, Felt::new(3), &mut lookups);

    let lookup_start_addr = 73;
    // there should be two lookups for start and end rows of hasher operation
    let expected_lookups_len = 2;
    // make sure the lookups have correct labels, addresses, indices and contexts.
    let expected_lookup_start = HasherLookup::new(
        MP_VERIFY_LABEL,
        lookup_start_addr,
        Felt::new(3),
        HasherLookupContext::Start,
    );
    let expected_lookup_end = HasherLookup::new(
        RETURN_HASH_LABEL,
        lookup_start_addr + 3 * HASH_CYCLE_LEN as u32 - 1,
        ZERO,
        HasherLookupContext::Return,
    );
    check_lookups_validity(
        lookups,
        expected_lookups_len,
        vec![expected_lookup_start, expected_lookup_end],
    );

    // build and check the trace for validity
    let (trace, aux_hints) = build_trace(hasher, 96);
    check_merkle_path(&trace, 0, leaves[0], &path0, 0, MP_VERIFY);
    check_merkle_path(&trace, 24, leaves[3], &path3, 3, MP_VERIFY);
    check_merkle_path(&trace, 48, leaves[7], &path7, 7, MP_VERIFY);
    check_merkle_path(&trace, 72, leaves[3], &path3, 3, MP_VERIFY);

    // make sure aux hints for sibling table are empty
    assert!(aux_hints.sibling_hints.is_empty());
    assert!(aux_hints.sibling_rows.is_empty());
}

#[test]
fn hasher_update_merkle_root() {
    // --- Merkle tree with 2 leaves ------------------------------------------

    // build a Merkle tree
    let leaves = init_leaves(&[1, 2]);
    let mut tree = MerkleTree::new(leaves.to_vec()).unwrap();

    // initialize the hasher and update both leaves
    let mut hasher = Hasher::default();

    let path0 = tree.get_path(NodeIndex::new(1, 0)).unwrap();
    let new_leaf0 = init_leaf(3);
    let mut lookups = Vec::new();
    let lookup_start_addr = 1;
    hasher.update_merkle_root(leaves[0], new_leaf0, &path0, ZERO, &mut lookups);
    tree.update_leaf(0, new_leaf0).unwrap();

    let expected_lookups_len = 4;
    // make sure the lookups have correct labels, addresses, indices and contexts.
    let expected_lookups = vec![
        HasherLookup::new(MR_UPDATE_OLD_LABEL, lookup_start_addr, ZERO, HasherLookupContext::Start),
        HasherLookup::new(
            RETURN_HASH_LABEL,
            lookup_start_addr + HASH_CYCLE_LEN as u32 - 1,
            ZERO,
            HasherLookupContext::Return,
        ),
        HasherLookup::new(
            MR_UPDATE_NEW_LABEL,
            lookup_start_addr + HASH_CYCLE_LEN as u32,
            ZERO,
            HasherLookupContext::Start,
        ),
        HasherLookup::new(
            RETURN_HASH_LABEL,
            lookup_start_addr + 2 * HASH_CYCLE_LEN as u32 - 1,
            ZERO,
            HasherLookupContext::Return,
        ),
    ];
    check_lookups_validity(lookups, expected_lookups_len, expected_lookups);

    let path1 = tree.get_path(NodeIndex::new(1, 1)).unwrap();
    let new_leaf1 = init_leaf(4);
    let mut lookups = Vec::new();

    hasher.update_merkle_root(leaves[1], new_leaf1, &path1, ONE, &mut lookups);
    tree.update_leaf(1, new_leaf1).unwrap();

    let lookup_start_addr = 17;
    let expected_lookups_len = 4;
    // make sure the lookups have correct labels, addresses, indices and contexts.
    let expected_lookups = vec![
        HasherLookup::new(MR_UPDATE_OLD_LABEL, lookup_start_addr, ONE, HasherLookupContext::Start),
        HasherLookup::new(
            RETURN_HASH_LABEL,
            lookup_start_addr + HASH_CYCLE_LEN as u32 - 1,
            ZERO,
            HasherLookupContext::Return,
        ),
        HasherLookup::new(
            MR_UPDATE_NEW_LABEL,
            lookup_start_addr + HASH_CYCLE_LEN as u32,
            ONE,
            HasherLookupContext::Start,
        ),
        HasherLookup::new(
            RETURN_HASH_LABEL,
            lookup_start_addr + 2 * HASH_CYCLE_LEN as u32 - 1,
            ZERO,
            HasherLookupContext::Return,
        ),
    ];
    check_lookups_validity(lookups, expected_lookups_len, expected_lookups);

    // build the trace
    let (trace, aux_hints) = build_trace(hasher, 32);

    // make sure the trace is correct
    check_row_addr_trace(&trace);
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

    // make sure sibling table hints were built correctly
    let expected_sibling_hints = vec![
        // first update
        (0, SiblingTableUpdate::SiblingAdded(0)),
        (8, SiblingTableUpdate::SiblingRemoved(0)),
        // second update
        (16, SiblingTableUpdate::SiblingAdded(1)),
        (24, SiblingTableUpdate::SiblingRemoved(1)),
    ];
    assert_eq!(expected_sibling_hints, aux_hints.sibling_hints);

    let expected_sibling_rows =
        vec![SiblingTableRow::new(ZERO, path0[0]), SiblingTableRow::new(ONE, path1[0])];
    assert_eq!(expected_sibling_rows, aux_hints.sibling_rows);

    // --- Merkle tree with 8 leaves ------------------------------------------

    // build a Merkle tree
    let leaves = init_leaves(&[1, 2, 3, 4, 5, 6, 7, 8]);
    let mut tree = MerkleTree::new(leaves.to_vec()).unwrap();

    // initialize the hasher
    let mut hasher = Hasher::default();

    let path3 = tree.get_path(NodeIndex::new(3, 3)).unwrap();
    let new_leaf3 = init_leaf(23);
    let mut lookups = Vec::new();
    hasher.update_merkle_root(leaves[3], new_leaf3, &path3, Felt::new(3), &mut lookups);
    tree.update_leaf(3, new_leaf3).unwrap();

    let lookup_start_addr = 1;
    let expected_lookups_len = 4;
    // make sure the lookups have correct labels, addresses, indices and contexts.
    let expected_lookups = vec![
        HasherLookup::new(
            MR_UPDATE_OLD_LABEL,
            lookup_start_addr,
            Felt::new(3),
            HasherLookupContext::Start,
        ),
        HasherLookup::new(
            RETURN_HASH_LABEL,
            lookup_start_addr + 3 * HASH_CYCLE_LEN as u32 - 1,
            ZERO,
            HasherLookupContext::Return,
        ),
        HasherLookup::new(
            MR_UPDATE_NEW_LABEL,
            lookup_start_addr + 3 * HASH_CYCLE_LEN as u32,
            Felt::new(3),
            HasherLookupContext::Start,
        ),
        HasherLookup::new(
            RETURN_HASH_LABEL,
            lookup_start_addr + 3 * HASH_CYCLE_LEN as u32 + 3 * HASH_CYCLE_LEN as u32 - 1,
            ZERO,
            HasherLookupContext::Return,
        ),
    ];
    check_lookups_validity(lookups, expected_lookups_len, expected_lookups);

    let path6 = tree.get_path(NodeIndex::new(3, 6)).unwrap();
    let new_leaf6 = init_leaf(25);
    let mut lookups = Vec::new();
    hasher.update_merkle_root(leaves[6], new_leaf6, &path6, Felt::new(6), &mut lookups);
    tree.update_leaf(6, new_leaf6).unwrap();

    let lookup_start_addr = 49;
    let expected_lookups_len = 4;
    // make sure the lookups have correct labels, addresses, indices and contexts.
    let expected_lookups = vec![
        HasherLookup::new(
            MR_UPDATE_OLD_LABEL,
            lookup_start_addr,
            Felt::new(6),
            HasherLookupContext::Start,
        ),
        HasherLookup::new(
            RETURN_HASH_LABEL,
            lookup_start_addr + 3 * HASH_CYCLE_LEN as u32 - 1,
            ZERO,
            HasherLookupContext::Return,
        ),
        HasherLookup::new(
            MR_UPDATE_NEW_LABEL,
            lookup_start_addr + 3 * HASH_CYCLE_LEN as u32,
            Felt::new(6),
            HasherLookupContext::Start,
        ),
        HasherLookup::new(
            RETURN_HASH_LABEL,
            lookup_start_addr + 3 * HASH_CYCLE_LEN as u32 + 3 * HASH_CYCLE_LEN as u32 - 1,
            ZERO,
            HasherLookupContext::Return,
        ),
    ];
    check_lookups_validity(lookups, expected_lookups_len, expected_lookups);

    // update leaf 3 again
    let path3_2 = tree.get_path(NodeIndex::new(3, 3)).unwrap();
    let new_leaf3_2 = init_leaf(27);
    let mut lookups = Vec::new();
    hasher.update_merkle_root(new_leaf3, new_leaf3_2, &path3_2, Felt::new(3), &mut lookups);
    tree.update_leaf(3, new_leaf3_2).unwrap();
    assert_ne!(path3, path3_2);

    let lookup_start_addr = 97;
    let expected_lookups_len = 4;
    // make sure the lookups have correct labels, addresses, indices and contexts.
    let expected_lookups = vec![
        HasherLookup::new(
            MR_UPDATE_OLD_LABEL,
            lookup_start_addr,
            Felt::new(3),
            HasherLookupContext::Start,
        ),
        HasherLookup::new(
            RETURN_HASH_LABEL,
            lookup_start_addr + 3 * HASH_CYCLE_LEN as u32 - 1,
            ZERO,
            HasherLookupContext::Return,
        ),
        HasherLookup::new(
            MR_UPDATE_NEW_LABEL,
            lookup_start_addr + 3 * HASH_CYCLE_LEN as u32,
            Felt::new(3),
            HasherLookupContext::Start,
        ),
        HasherLookup::new(
            RETURN_HASH_LABEL,
            lookup_start_addr + 3 * HASH_CYCLE_LEN as u32 + 3 * HASH_CYCLE_LEN as u32 - 1,
            ZERO,
            HasherLookupContext::Return,
        ),
    ];
    check_lookups_validity(lookups, expected_lookups_len, expected_lookups);

    // build and check the trace for validity
    let (trace, aux_hints) = build_trace(hasher, 144);
    check_merkle_path(&trace, 0, leaves[3], &path3, 3, MR_UPDATE_OLD);
    check_merkle_path(&trace, 24, new_leaf3, &path3, 3, MR_UPDATE_NEW);
    check_merkle_path(&trace, 48, leaves[6], &path6, 6, MR_UPDATE_OLD);
    check_merkle_path(&trace, 72, new_leaf6, &path6, 6, MR_UPDATE_NEW);
    check_merkle_path(&trace, 96, new_leaf3, &path3_2, 3, MR_UPDATE_OLD);
    check_merkle_path(&trace, 120, new_leaf3_2, &path3_2, 3, MR_UPDATE_NEW);

    // make sure sibling table hints were built correctly
    let expected_sibling_hints = vec![
        // first update
        (0, SiblingTableUpdate::SiblingAdded(0)),
        (8, SiblingTableUpdate::SiblingAdded(1)),
        (16, SiblingTableUpdate::SiblingAdded(2)),
        (24, SiblingTableUpdate::SiblingRemoved(0)),
        (32, SiblingTableUpdate::SiblingRemoved(1)),
        (40, SiblingTableUpdate::SiblingRemoved(2)),
        // second update
        (48, SiblingTableUpdate::SiblingAdded(3)),
        (56, SiblingTableUpdate::SiblingAdded(4)),
        (64, SiblingTableUpdate::SiblingAdded(5)),
        (72, SiblingTableUpdate::SiblingRemoved(3)),
        (80, SiblingTableUpdate::SiblingRemoved(4)),
        (88, SiblingTableUpdate::SiblingRemoved(5)),
        // third update
        (96, SiblingTableUpdate::SiblingAdded(6)),
        (104, SiblingTableUpdate::SiblingAdded(7)),
        (112, SiblingTableUpdate::SiblingAdded(8)),
        (120, SiblingTableUpdate::SiblingRemoved(6)),
        (128, SiblingTableUpdate::SiblingRemoved(7)),
        (136, SiblingTableUpdate::SiblingRemoved(8)),
    ];
    assert_eq!(expected_sibling_hints, aux_hints.sibling_hints);

    let expected_sibling_rows = vec![
        // first update
        SiblingTableRow::new(Felt::new(3), path3[0]),
        SiblingTableRow::new(Felt::new(3 >> 1), path3[1]),
        SiblingTableRow::new(Felt::new(3 >> 2), path3[2]),
        // second update
        SiblingTableRow::new(Felt::new(6), path6[0]),
        SiblingTableRow::new(Felt::new(6 >> 1), path6[1]),
        SiblingTableRow::new(Felt::new(6 >> 2), path6[2]),
        // third update
        SiblingTableRow::new(Felt::new(3), path3_2[0]),
        SiblingTableRow::new(Felt::new(3 >> 1), path3_2[1]),
        SiblingTableRow::new(Felt::new(3 >> 2), path3_2[2]),
    ];
    assert_eq!(expected_sibling_rows, aux_hints.sibling_rows);
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

    let mut lookups = Vec::new();
    // builds the trace of the join block.
    let (_, final_state) =
        hasher.hash_control_block(h1, h2, join_block.domain(), expected_hash, &mut lookups);

    let lookup_start_addr = 1;
    let expected_lookups_len = 2;
    // make sure the lookups have correct labels, addresses, indices and contexts.
    let lookup_start =
        HasherLookup::new(LINEAR_HASH_LABEL, lookup_start_addr, ZERO, HasherLookupContext::Start);
    let lookup_end = HasherLookup::new(
        RETURN_HASH_LABEL,
        lookup_start_addr + HASH_CYCLE_LEN as u32 - 1,
        ZERO,
        HasherLookupContext::Return,
    );
    check_lookups_validity(lookups, expected_lookups_len, vec![lookup_start, lookup_end]);

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

    let mut lookups = Vec::new();
    // builds the hash execution trace of the first split block from scratch.
    let (addr, final_state) =
        hasher.hash_control_block(h1, h2, split1_block.domain(), expected_hash, &mut lookups);

    let lookup_start_addr = 9;
    let expected_lookups_len = 2;
    // make sure the lookups have correct labels, addresses, indices and contexts.
    let lookup_start =
        HasherLookup::new(LINEAR_HASH_LABEL, lookup_start_addr, ZERO, HasherLookupContext::Start);
    let lookup_end = HasherLookup::new(
        RETURN_HASH_LABEL,
        lookup_start_addr + HASH_CYCLE_LEN as u32 - 1,
        ZERO,
        HasherLookupContext::Return,
    );
    check_lookups_validity(lookups, expected_lookups_len, vec![lookup_start, lookup_end]);

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

    let mut lookups = Vec::new();
    // builds the hash execution trace of the second split block by copying it from the trace of
    // the first split block.
    let (addr, final_state) =
        hasher.hash_control_block(h1, h2, split2_block.domain(), expected_hash, &mut lookups);

    let lookup_start_addr = 17;
    let expected_lookups_len = 2;
    // make sure the lookups have correct labels, addresses, indices and contexts.
    let lookup_start =
        HasherLookup::new(LINEAR_HASH_LABEL, lookup_start_addr, ZERO, HasherLookupContext::Start);
    let lookup_end = HasherLookup::new(
        RETURN_HASH_LABEL,
        lookup_start_addr + HASH_CYCLE_LEN as u32 - 1,
        ZERO,
        HasherLookupContext::Return,
    );
    check_lookups_validity(lookups, expected_lookups_len, vec![lookup_start, lookup_end]);

    // make sure the hash of the final state of the second split block is the same as the expected
    // hash.
    assert_eq!(Digest::new(final_state), expected_hash);
    // make sure the hash of the first and second split blocks is the same.
    assert_eq!(first_block_final_state, final_state);

    let copied_start_row = addr.as_int() as usize - 1;
    let copied_end_row = hasher.trace_len() - 1;

    let (trace, _) = build_trace(hasher, copied_end_row + 1);

    // check row addresses of trace to make sure they start from 1 and incremented by 1 each row.
    check_row_addr_trace(&trace);
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
        Operation::Push(Felt::new(1)),
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

    let mut lookups = Vec::new();
    // builds the trace of the Join1 block.
    let (_, final_state) =
        hasher.hash_control_block(h1, h2, join1_block.domain(), expected_hash, &mut lookups);

    let lookup_start_addr = 1;
    let expected_lookups_len = 2;
    // make sure the lookups have correct labels, addresses, indices and contexts.
    let lookup_start =
        HasherLookup::new(LINEAR_HASH_LABEL, lookup_start_addr, ZERO, HasherLookupContext::Start);
    let lookup_end = HasherLookup::new(
        RETURN_HASH_LABEL,
        lookup_start_addr + HASH_CYCLE_LEN as u32 - 1,
        ZERO,
        HasherLookupContext::Return,
    );
    check_lookups_validity(lookups, expected_lookups_len, vec![lookup_start, lookup_end]);
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

    let mut lookups = Vec::new();
    let (_, final_state) =
        hasher.hash_control_block(h1, h2, join2_block.domain(), expected_hash, &mut lookups);

    let lookup_start_addr = 9;
    let expected_lookups_len = 2;
    // make sure the lookups have correct labels, addresses, indices and contexts.
    let lookup_start =
        HasherLookup::new(LINEAR_HASH_LABEL, lookup_start_addr, ZERO, HasherLookupContext::Start);
    let lookup_end = HasherLookup::new(
        RETURN_HASH_LABEL,
        lookup_start_addr + HASH_CYCLE_LEN as u32 - 1,
        ZERO,
        HasherLookupContext::Return,
    );
    check_lookups_validity(lookups, expected_lookups_len, vec![lookup_start, lookup_end]);

    // make sure the hash of the final state of Join2 is the same as the expected hash.
    assert_eq!(Digest::new(final_state), expected_hash);

    let span1_block_val = if let CodeBlock::Span(span) = span1_block.clone() {
        span
    } else {
        unreachable!()
    };

    // builds the hash execution trace of the first span block from scratch.
    let mut lookups = Vec::new();
    let (addr, final_state) =
        hasher.hash_span_block(span1_block_val.op_batches(), span1_block.hash(), &mut lookups);

    let num_batches = span1_block_val.op_batches().len();
    let lookup_start_addr = 17;

    let expected_lookups_len = 2 + num_batches - 1;

    let mut expected_lookups = Vec::new();

    // add lookup for start of span block
    let lookup_start =
        HasherLookup::new(LINEAR_HASH_LABEL, lookup_start_addr, ZERO, HasherLookupContext::Start);
    expected_lookups.push(lookup_start);

    // add lookups for absorbed batches
    for i in 1..num_batches {
        let lookup = HasherLookup::new(
            LINEAR_HASH_LABEL,
            lookup_start_addr + (i * HASH_CYCLE_LEN) as u32 - 1,
            ZERO,
            HasherLookupContext::Absorb,
        );
        expected_lookups.push(lookup);
    }

    let last_lookup_addr_memoized_block =
        lookup_start_addr + (num_batches * HASH_CYCLE_LEN) as u32 - 1;

    // add lookup for end of span block
    let lookup_end = HasherLookup::new(
        RETURN_HASH_LABEL,
        last_lookup_addr_memoized_block,
        ZERO,
        HasherLookupContext::Return,
    );
    expected_lookups.push(lookup_end);

    check_lookups_validity(lookups, expected_lookups_len, expected_lookups);

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

    let mut lookups = Vec::new();
    // builds the hash execution trace of the second span block by copying the sections of the
    // trace corresponding to the first span block with the same hash.
    let (addr, final_state) =
        hasher.hash_span_block(span2_block_val.op_batches(), span2_block.hash(), &mut lookups);

    let num_batches = span2_block_val.op_batches().len();
    let lookup_start_addr = last_lookup_addr_memoized_block + 1;

    let expected_lookups_len = 2 + num_batches - 1;

    let mut expected_lookups = Vec::new();

    // add lookup for start of span block
    let lookup_start =
        HasherLookup::new(LINEAR_HASH_LABEL, lookup_start_addr, ZERO, HasherLookupContext::Start);
    expected_lookups.push(lookup_start);

    // add lookups for absorbed batches
    for i in 1..num_batches {
        let lookup = HasherLookup::new(
            LINEAR_HASH_LABEL,
            lookup_start_addr + (i * HASH_CYCLE_LEN) as u32 - 1,
            ZERO,
            HasherLookupContext::Absorb,
        );
        expected_lookups.push(lookup);
    }

    // add lookup for end of span block
    let lookup_end = HasherLookup::new(
        RETURN_HASH_LABEL,
        lookup_start_addr + (num_batches * HASH_CYCLE_LEN) as u32 - 1,
        ZERO,
        HasherLookupContext::Return,
    );
    expected_lookups.push(lookup_end);

    check_lookups_validity(lookups, expected_lookups_len, expected_lookups);

    let expected_hash = span2_block.hash();
    // make sure the hash of the final state of Span2 block is the same as the expected hash.
    assert_eq!(Digest::new(final_state), expected_hash);

    // make sure the hash of the first and second span blocks is the same.
    assert_eq!(first_span_block_final_state, final_state);

    let copied_start_row = addr.as_int() as usize - 1;
    let copied_end_row = hasher.trace_len() - 1;

    let (trace, _) = build_trace(hasher, copied_end_row + 1);

    // check row addresses of trace to make sure they start from 1 and incremented by 1 each row.
    check_row_addr_trace(&trace);

    // check correct copy after memoization
    check_memoized_trace(&trace, start_row, end_row, copied_start_row, copied_end_row);
}

// HELPER FUNCTIONS
// ================================================================================================

/// Builds an execution trace for the provided hasher. The trace must have the number of rows
/// specified by num_rows.
fn build_trace(hasher: Hasher, num_rows: usize) -> (Vec<Vec<Felt>>, AuxTraceBuilder) {
    let mut trace = (0..TRACE_WIDTH).map(|_| vec![Felt::new(0); num_rows]).collect::<Vec<_>>();
    let mut fragment = TraceFragment::trace_to_fragment(&mut trace);
    let aux_trace_builder = hasher.fill_trace(&mut fragment);
    (trace, aux_trace_builder)
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
    check_row_addr_trace(trace);

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
            root = hasher::merge(&[root.into(), node.into()]).into();
            init_state_from_words(&old_root, &node)
        } else {
            root = hasher::merge(&[node.into(), root.into()]).into();
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

/// Makes sure the lookups are built correctly.
fn check_lookups_validity(
    lookups: Vec<HasherLookup>,
    expected_lookups_length: usize,
    expected_lookups: Vec<HasherLookup>,
) {
    // make sure the length of the lookups is what we expect.
    assert_eq!(expected_lookups_length, lookups.len());

    // make sure the length of lookups and expected lookups is same.
    assert_eq!(expected_lookups.len(), lookups.len());

    for (lookup, expected_lookup) in lookups.iter().zip(expected_lookups) {
        // make sure the lookups match with what we expect.
        assert_eq!(expected_lookup, *lookup);
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
