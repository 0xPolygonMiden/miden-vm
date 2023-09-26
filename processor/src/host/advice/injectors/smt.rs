use super::super::{AdviceSource, ExecutionError, Felt, HostResponse, StarkField, Word};
use crate::{AdviceProvider, ProcessState};
use vm_core::{
    crypto::{
        hash::{Rpo256, RpoDigest},
        merkle::{EmptySubtreeRoots, NodeIndex, TieredSmt},
    },
    utils::collections::{btree_map::Entry, BTreeMap, Vec},
    ONE, WORD_SIZE, ZERO,
};

// CONSTANTS
// ================================================================================================

/// Maximum depth of a Sparse Merkle Tree
const SMT_MAX_TREE_DEPTH: Felt = Felt::new(64);

/// Lookup table for Sparse Merkle Tree depth normalization
const SMT_NORMALIZED_DEPTHS: [u8; 65] = [
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 32, 32, 32, 32, 32, 32, 32,
    32, 32, 32, 32, 32, 32, 32, 32, 32, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48,
    48, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
];

// SMT INJECTORS
// ================================================================================================

/// Pushes values onto the advice stack which are required for successful retrieval of a
/// value from a Sparse Merkle Tree data structure.
///
/// The Sparse Merkle Tree is tiered, meaning it will have leaf depths in `{16, 32, 48, 64}`.
/// The depth flags define the tier on which the leaf is located.
///
/// Inputs:
///   Operand stack: [KEY, ROOT, ...]
///   Advice stack: [...]
///
/// Outputs:
///   Operand stack: [KEY, ROOT, ...]
///   Advice stack: [f0, f1, K, V, f2]
///
/// Where:
/// - f0 is a boolean flag set to `1` if the depth is `16` or `48`.
/// - f1 is a boolean flag set to `1` if the depth is `16` or `32`.
/// - K is the key; will be zeroed if the tree don't contain a mapped value for the key.
/// - V is the value word; will be zeroed if the tree don't contain a mapped value for the key.
/// - f2 is a boolean flag set to `1` if the key is not zero.
///
/// # Errors
/// Returns an error if the provided Merkle root doesn't exist on the advice provider.
///
/// # Panics
/// Will panic as unimplemented if the target depth is `64`.
pub(crate) fn push_smtget_inputs<S: ProcessState, A: AdviceProvider>(
    advice_provider: &mut A,
    process: &S,
) -> Result<HostResponse, ExecutionError> {
    // fetch the arguments from the operand stack
    let key = process.get_stack_word(0);
    let root = process.get_stack_word(1);

    // get the node from the SMT for the specified key; this node can be either a leaf node,
    // or a root of an empty subtree at the returned depth
    let (node, depth, _) = get_smt_node(advice_provider, root, key)?;

    // set the node value; zeroed if empty sub-tree
    let empty = EmptySubtreeRoots::empty_hashes(64);
    if Word::from(empty[depth as usize]) == node {
        // push zeroes for remaining key, value & empty remaining key flag
        for _ in 0..9 {
            advice_provider.push_stack(AdviceSource::Value(ZERO))?;
        }
    } else {
        // push a flag indicating that a remaining key exists
        advice_provider.push_stack(AdviceSource::Value(ONE))?;

        // map is expected to contain `node |-> {K, V}`
        advice_provider.push_stack(AdviceSource::Map {
            key: node,
            include_len: false,
        })?;
    }

    // set the flags
    let is_16_or_32 = if depth == 16 || depth == 32 { ONE } else { ZERO };
    let is_16_or_48 = if depth == 16 || depth == 48 { ONE } else { ZERO };
    advice_provider.push_stack(AdviceSource::Value(is_16_or_32))?;
    advice_provider.push_stack(AdviceSource::Value(is_16_or_48))?;

    Ok(HostResponse::Unit)
}

/// Pushes onto the advice stack the value associated with the specified key in a Sparse
/// Merkle Tree defined by the specified root.
///
/// If no value was previously associated with the specified key, [ZERO; 4] is pushed onto
/// the advice stack.
///
/// Inputs:
///   Operand stack: [KEY, ROOT, ...]
///   Advice stack: [...]
///
/// Outputs:
///   Operand stack: [KEY, ROOT, ...]
///   Advice stack: [VALUE, ...]
///
/// # Errors
/// Returns an error if the provided Merkle root doesn't exist on the advice provider.
///
/// # Panics
/// Will panic as unimplemented if the target depth is `64`.
pub(crate) fn push_smtpeek_result<S: ProcessState, A: AdviceProvider>(
    advice_provider: &mut A,
    process: &S,
) -> Result<HostResponse, ExecutionError> {
    // fetch the arguments from the operand stack
    let key = process.get_stack_word(0);
    let root = process.get_stack_word(1);

    // get the node from the SMT for the specified key; this node can be either a leaf node,
    // or a root of an empty subtree at the returned depth
    let (node, depth, _) = get_smt_node(advice_provider, root, key)?;

    let empty = EmptySubtreeRoots::empty_hashes(64)[depth as usize];
    if node == Word::from(empty) {
        // if the node is a root of an empty subtree, then there is no value associated with
        // the specified key
        advice_provider.push_stack(AdviceSource::Word(TieredSmt::EMPTY_VALUE))?;
    } else {
        // get the key and value stored in the current leaf
        let (leaf_key, leaf_value) = get_smt_upper_leaf_preimage(advice_provider, node)?;

        // if the leaf is for a different key, then there is no value associated with the
        // specified key
        if leaf_key == key {
            advice_provider.push_stack(AdviceSource::Word(leaf_value))?;
        } else {
            advice_provider.push_stack(AdviceSource::Word(TieredSmt::EMPTY_VALUE))?;
        }
    }

    Ok(HostResponse::Unit)
}

/// Pushes values onto the advice stack which are required for successful insertion of a
/// key-value pair into a Sparse Merkle Tree data structure.
///
/// The Sparse Merkle Tree is tiered, meaning it will have leaf depths in `{16, 32, 48, 64}`.
///
/// Inputs:
///   Operand stack: [VALUE, KEY, ROOT, ...]
///   Advice stack: [...]
///
/// Outputs:
///   Operand stack: [OLD_VALUE, NEW_ROOT, ...]
///   Advice stack: see comments for specialized handlers below.
///
/// Where:
/// - ROOT and NEW_ROOT are the roots of the TSMT before and after the insert respectively.
/// - VALUE is the value to be inserted.
/// - OLD_VALUE is the value previously associated with the specified KEY.
///
/// # Errors
/// Returns an error if:
/// - The Merkle store does not contain a node with the specified root.
/// - The Merkle store does not contain all nodes needed to validate the path between the root
///   and the relevant TSMT nodes.
/// - The advice map does not contain required data about TSMT leaves to be modified.
///
/// # Panics
/// Will panic as unimplemented if the target depth is `64`.
pub(crate) fn push_smtset_inputs<S: ProcessState, A: AdviceProvider>(
    advice_provider: &mut A,
    process: &S,
) -> Result<HostResponse, ExecutionError> {
    // get the key, value, and tree root from the stack
    let value = process.get_stack_word(0);
    let key = process.get_stack_word(1);
    let root = process.get_stack_word(2);

    // get the node from the SMT for the specified key; this node can be either a leaf node,
    // or a root of an empty subtree at the returned depth
    let (node, depth, index) = get_smt_node(advice_provider, root, key)?;

    // if the value to be inserted is an empty word, we need to process it as a delete
    if value == TieredSmt::EMPTY_VALUE {
        return handle_smt_delete(advice_provider, root, node, depth, index, key);
    }

    // figure out what kind of insert we are doing; possible options are:
    // - if the node is a root of an empty subtree, this is a simple insert.
    // - if the node is a leaf, this could be either an update (for the same key), or a
    //   complex insert (i.e., the existing leaf needs to be moved to a lower tier).
    let empty = EmptySubtreeRoots::empty_hashes(64)[depth as usize];
    if node == Word::from(empty) {
        handle_smt_simple_insert(advice_provider, root, depth, index)
    } else {
        // get the key and value stored in the current leaf
        let (leaf_key, leaf_value) = get_smt_upper_leaf_preimage(advice_provider, node)?;

        // if the key for the value to be inserted is the same as the leaf's key, we are
        // dealing with a simple update; otherwise, we are dealing with a complex insert
        if leaf_key == key {
            handle_smt_update(advice_provider, depth, leaf_value)
        } else {
            handle_smt_complex_insert(advice_provider, depth, key, leaf_key, leaf_value)
        }
    }
}

// TSMT UPDATE HELPER METHODS
// --------------------------------------------------------------------------------------------

/// Returns first leaf or an empty tree node for the provided key in the Sparse Merkle tree
/// with the specified root.
///
/// Also returns the depth and index of the returned node at this depth.
fn get_smt_node<A: AdviceProvider>(
    advice_provider: &A,
    root: Word,
    key: Word,
) -> Result<(Word, u8, Felt), ExecutionError> {
    // determine the depth of the first leaf or an empty tree node
    let index = &key[3];
    let depth = advice_provider.get_leaf_depth(root, &SMT_MAX_TREE_DEPTH, index)?;
    debug_assert!(depth < 65);

    // map the depth value to its tier; this rounds up depth to 16, 32, 48, or 64
    let depth = SMT_NORMALIZED_DEPTHS[depth as usize];
    if depth == 64 {
        unimplemented!("handling of depth=64 tier hasn't been implemented yet");
    }

    // get the value of the node at this index/depth
    let index = index.as_int() >> (64 - depth);
    let index = Felt::new(index);
    let node = advice_provider.get_tree_node(root, &Felt::from(depth), &index)?;

    Ok((node, depth, index))
}

/// Retrieves a key-value pair for the specified leaf node from the advice map.
///
/// # Errors
/// Returns an error if the value under the specified node does not exist or does not consist
/// of exactly 8 elements.
fn get_smt_upper_leaf_preimage<A: AdviceProvider>(
    advice_provider: &A,
    node: Word,
) -> Result<(Word, Word), ExecutionError> {
    let node_bytes = RpoDigest::from(node).as_bytes();
    let kv = advice_provider
        .get_mapped_values(&node_bytes)
        .ok_or(ExecutionError::AdviceMapKeyNotFound(node))?;

    if kv.len() != WORD_SIZE * 2 {
        return Err(ExecutionError::AdviceMapValueInvalidLength(node, WORD_SIZE * 2, kv.len()));
    }

    let key = [kv[0], kv[1], kv[2], kv[3]];
    let val = [kv[4], kv[5], kv[6], kv[7]];
    Ok((key, val))
}

/// Prepares the advice stack for a TSMT update operation. Specifically, the advice stack will
/// be arranged as follows:
///
/// - [ZERO (padding), d0, d1, ONE (is_update), OLD_VALUE]
///
/// Where:
/// - d0 is a boolean flag set to `1` if the depth is `16` or `48`.
/// - d1 is a boolean flag set to `1` if the depth is `16` or `32`.
/// - OLD_VALUE is the current value in the leaf to be updated.
fn handle_smt_update<A: AdviceProvider>(
    advice_provider: &mut A,
    depth: u8,
    old_value: Word,
) -> Result<HostResponse, ExecutionError> {
    // put the old value onto the advice stack
    advice_provider.push_stack(AdviceSource::Word(old_value))?;

    // set is_update flag to ONE
    advice_provider.push_stack(AdviceSource::Value(ONE))?;

    // set depth flags based on leaf's depth
    let (is_16_or_32, is_16_or_48) = get_depth_flags(depth);
    advice_provider.push_stack(AdviceSource::Value(is_16_or_32))?;
    advice_provider.push_stack(AdviceSource::Value(is_16_or_48))?;

    // pad the advice stack with an extra value to make it consistent with other cases when
    // we expect 4 flag values on the top of the advice stack
    advice_provider.push_stack(AdviceSource::Value(ZERO))?;

    Ok(HostResponse::Unit)
}

/// Prepares the advice stack for a TSMT simple insert operation (i.e., when we are replacing
/// an empty node). Specifically, the advice stack will be arranged as follows:
///
/// - Simple insert at depth 16: [d0, d1, ONE (is_simple_insert), ZERO (is_update)]
/// - Simple insert at depth 32 or 48: [d0, d1, ONE (is_simple_insert), ZERO (is_update), P_NODE]
///
/// Where:
/// - d0 is a boolean flag set to `1` if the depth is `16` or `48`.
/// - d1 is a boolean flag set to `1` if the depth is `16` or `32`.
/// - P_NODE is an internal node located at the tier above the insert tier.
fn handle_smt_simple_insert<A: AdviceProvider>(
    advice_provider: &mut A,
    root: Word,
    depth: u8,
    index: Felt,
) -> Result<HostResponse, ExecutionError> {
    // put additional data onto the advice stack as needed
    match depth {
        16 => (), // nothing to do; all the required data is already in the VM
        32 | 48 => {
            // for depth 32 and 48, we need to provide the internal node located on the tier
            // above the insert tier
            let p_index = Felt::from(index.as_int() >> 16);
            let p_depth = Felt::from(depth - 16);
            let p_node = advice_provider.get_tree_node(root, &p_depth, &p_index)?;
            advice_provider.push_stack(AdviceSource::Word(p_node))?;
        }
        64 => unimplemented!("insertions at depth 64 are not yet implemented"),
        _ => unreachable!("invalid depth {depth}"),
    }

    // push is_update and is_simple_insert flags onto the advice stack
    advice_provider.push_stack(AdviceSource::Value(ZERO))?;
    advice_provider.push_stack(AdviceSource::Value(ONE))?;

    // set depth flags based on node's depth
    let (is_16_or_32, is_16_or_48) = get_depth_flags(depth);
    advice_provider.push_stack(AdviceSource::Value(is_16_or_32))?;
    advice_provider.push_stack(AdviceSource::Value(is_16_or_48))?;

    Ok(HostResponse::Unit)
}

/// Prepares the advice stack for a TSMT complex insert operation (i.e., when a leaf node needs
/// to be replaced with a subtree of nodes at a lower tier). Specifically, the advice stack
/// will be arranged as follows:
///
///  - [d0, d1, ZERO (is_simple_insert), ZERO (is_update), E_KEY, E_VALUE]
///
/// Where:
/// - d0 and d1 are boolean flags a combination of which determines the source and the target
///   tiers as follows:
///   - (0, 0): depth 16 -> 32
///   - (0, 1): depth 16 -> 48
///   - (1, 0): depth 32 -> 48
///   - (1, 1): depth 16, 32, or 48 -> 64
/// - E_KEY and E_VALUE are the key-value pair for a leaf which is to be replaced by a subtree.
fn handle_smt_complex_insert<A: AdviceProvider>(
    advice_provider: &mut A,
    depth: u8,
    key: Word,
    leaf_key: Word,
    leaf_value: Word,
) -> Result<HostResponse, ExecutionError> {
    // push the key and value onto the advice stack
    advice_provider.push_stack(AdviceSource::Word(leaf_value))?;
    advice_provider.push_stack(AdviceSource::Word(leaf_key))?;

    // push is_update and is_simple_insert flags onto the advice stack
    advice_provider.push_stack(AdviceSource::Value(ZERO))?;
    advice_provider.push_stack(AdviceSource::Value(ZERO))?;

    // determine the combination of the source and target tiers for the insert
    // and populate the depth flags accordingly
    let common_prefix = get_common_prefix(&key, &leaf_key);
    let target_depth = SMT_NORMALIZED_DEPTHS[common_prefix as usize + 1];
    match target_depth {
        32 if depth == 16 => {
            advice_provider.push_stack(AdviceSource::Value(ONE))?;
            advice_provider.push_stack(AdviceSource::Value(ONE))?;
        }
        48 if depth == 16 => {
            advice_provider.push_stack(AdviceSource::Value(ONE))?;
            advice_provider.push_stack(AdviceSource::Value(ZERO))?;
        }
        48 if depth == 32 => {
            advice_provider.push_stack(AdviceSource::Value(ZERO))?;
            advice_provider.push_stack(AdviceSource::Value(ONE))?;
        }
        64 => unimplemented!("insertions at depth 64 are not yet implemented"),
        _ => unreachable!("invalid source/target tier combination: {depth} -> {target_depth}"),
    }

    Ok(HostResponse::Unit)
}

/// Prepares the advice stack for a TSMT deletion operation. Specifically, the advice stack
/// will be arranged as follows (depending on the type of the node which occupies the location
/// at which the node for the specified key should be present):
///
/// - Root of empty subtree: [d0, d1, ZERO (is_leaf), ONE (key_not_set)]
/// - Leaf for another key: [d0, d1, ONE (is_leaf), ONE (key_not_set), KEY, VALUE]
/// - Leaf for the provided key: [ZERO, ZERO, ZERO, ZERO (key_not_set), NEW_ROOT, OLD_VALUE]
///
/// Where:
/// - d0 is a boolean flag set to `1` if the depth is `16` or `48`.
/// - d1 is a boolean flag set to `1` if the depth is `16` or `32`.
/// - KEY and VALUE is the key-value pair of a leaf node occupying the location of the node
///   for the specified key. Note that KEY may be the same as the specified key or different
///   from the specified key if the location is occupied by a different key-value pair.
/// - NEW_ROOT is the new root of the TSMT post deletion.
/// - OLD_VALUE is the value which is to be replaced with [ZERO; 4].
fn handle_smt_delete<A: AdviceProvider>(
    advice_provider: &mut A,
    root: Word,
    node: Word,
    depth: u8,
    index: Felt,
    key: Word,
) -> Result<HostResponse, ExecutionError> {
    let empty = EmptySubtreeRoots::empty_hashes(TieredSmt::MAX_DEPTH)[depth as usize];

    if node == Word::from(empty) {
        // if the node to be replaced is already an empty node, we set key_not_set = ONE,
        // and is_leaf = ZERO
        advice_provider.push_stack(AdviceSource::Value(ONE))?;
        advice_provider.push_stack(AdviceSource::Value(ZERO))?;

        // set depth flags based on node's depth
        let (is_16_or_32, is_16_or_48) = get_depth_flags(depth);
        advice_provider.push_stack(AdviceSource::Value(is_16_or_32))?;
        advice_provider.push_stack(AdviceSource::Value(is_16_or_48))?;

        Ok(HostResponse::Unit)
    } else {
        // if the node is not a root of an empty subtree, it must be a leaf; thus we can get
        // the key and the value stored in the leaf.
        let (leaf_key, leaf_value) = get_smt_upper_leaf_preimage(advice_provider, node)?;

        if leaf_key != key {
            // if the node to be replaced is a leaf for different key, we push that key-value
            // pair onto the advice stack and set key_not_set = ONE and is_leaf = ONE

            advice_provider.push_stack(AdviceSource::Word(leaf_value))?;
            advice_provider.push_stack(AdviceSource::Word(leaf_key))?;

            advice_provider.push_stack(AdviceSource::Value(ONE))?;
            advice_provider.push_stack(AdviceSource::Value(ONE))?;

            // set depth flags based on node's depth
            let (is_16_or_32, is_16_or_48) = get_depth_flags(depth);
            advice_provider.push_stack(AdviceSource::Value(is_16_or_32))?;
            advice_provider.push_stack(AdviceSource::Value(is_16_or_48))?;
        } else {
            // if the key which we want to set to [ZERO; 4] does have an associated value,
            // we update the tree in the advice provider to get the new root, then push the root
            // and the old value onto the advice stack, key_not_set = ZERO, and also push 3
            // ZERO values for padding
            let new_root = match find_lone_sibling(advice_provider, root, depth, &index)? {
                Some((sibling, new_index)) => {
                    // if the node to be deleted has a lone sibling, we need to move it to a
                    // higher tier.

                    // first, we compute the value of the new node on the higher tier
                    let (leaf_key, leaf_val) =
                        get_smt_upper_leaf_preimage(advice_provider, *sibling)?;
                    let new_node = Rpo256::merge_in_domain(
                        &[leaf_key.into(), leaf_val.into()],
                        new_index.depth().into(),
                    );

                    // then we insert the node and its pre-image into the advice provider
                    let mut elements = leaf_key.to_vec();
                    elements.extend_from_slice(&leaf_val);
                    advice_provider.insert_into_map(new_node.into(), elements)?;

                    // and finally we update the tree in the advice provider
                    let (_, new_root) = advice_provider.update_merkle_node(
                        root,
                        &new_index.depth().into(),
                        &new_index.value().into(),
                        new_node.into(),
                    )?;
                    new_root
                }
                None => {
                    // if the node does not have a lone sibling, we just replace it with an
                    // empty node
                    let (_, new_root) = advice_provider.update_merkle_node(
                        root,
                        &Felt::from(depth),
                        &index,
                        empty.into(),
                    )?;
                    new_root
                }
            };

            advice_provider.push_stack(AdviceSource::Word(leaf_value))?;
            advice_provider.push_stack(AdviceSource::Word(new_root))?;

            advice_provider.push_stack(AdviceSource::Value(ZERO))?;
            advice_provider.push_stack(AdviceSource::Value(ZERO))?;

            advice_provider.push_stack(AdviceSource::Value(ZERO))?;
            advice_provider.push_stack(AdviceSource::Value(ZERO))?;
        }
        Ok(HostResponse::Unit)
    }
}

/// Returns info about a lone sibling of a leaf specified by depth and index parameters in the
/// Tiered Sparse Merkle tree defined by the specified root. If no lone siblings exist for the
/// specified parameters, None is returned.
///
/// A lone sibling is defined as a leaf which has a common root with the specified leaf at a
/// higher tier such that the subtree starting at this root contains only these two leaves.
///
/// In addition to the leaf node itself, this also returns the index of the common root at a
/// higher tier.
fn find_lone_sibling<A: AdviceProvider>(
    advice_provider: &A,
    root: Word,
    depth: u8,
    index: &Felt,
) -> Result<Option<(RpoDigest, NodeIndex)>, ExecutionError> {
    debug_assert!(matches!(depth, 16 | 32 | 48));

    // if the leaf is on the first tier (depth=16), we don't care about lone siblings as they
    // cannot be moved to a higher tier.
    if depth == TieredSmt::TIER_SIZE {
        return Ok(None);
    }

    let empty = &EmptySubtreeRoots::empty_hashes(TieredSmt::MAX_DEPTH)[..=depth as usize];

    // get the path to the leaf node
    let path: Vec<_> = advice_provider.get_merkle_path(root, &depth.into(), index)?.into();

    // traverse the path from the leaf up to the root, keeping track of all non-empty nodes;
    // here we ignore the top 16 depths because lone siblings cannot be moved to a higher tier
    // from tier at depth 16.
    let mut non_empty_nodes = BTreeMap::new();
    for (depth, sibling) in (TieredSmt::TIER_SIZE..=depth).rev().zip(path.iter()) {
        // map the depth of each node to the tier it would "round up" to. For example, 17 maps
        // to tier 1, 32 also maps to tier 1, but 33 maps to tier 2.
        let tier = (depth - 1) / TieredSmt::TIER_SIZE;

        // if the node is non-empty, insert it into the map, but if a node for the same tier
        // is already in the map, stop traversing the tree. we do this because if two nodes in
        // a given tier are non-empty a lone sibling cannot exist at this tier or any higher
        // tier. to indicate the the tier cannot contain a lone sibling, we set the value in
        // the map to None.
        if sibling != &empty[depth as usize] {
            match non_empty_nodes.entry(tier) {
                Entry::Vacant(entry) => {
                    entry.insert(Some((depth, *sibling)));
                }
                Entry::Occupied(mut entry) => {
                    entry.insert(None);
                    break;
                }
            }
        }
    }

    // take the deepest non-empty node and check if its subtree contains just a single leaf
    if let Some((_, Some((node_depth, node)))) = non_empty_nodes.pop_last() {
        let mut node_index = NodeIndex::new(depth, index.as_int()).expect("invalid node index");
        node_index.move_up_to(node_depth);
        let node_index = node_index.sibling();

        if let Some((mut leaf_index, leaf)) =
            advice_provider.find_lone_leaf(node.into(), node_index, TieredSmt::MAX_DEPTH)?
        {
            // if the node's subtree does contain a single leaf, figure out to which depth
            // we can move it up to. we do this by taking the next tier down from the tier
            // which contained at least one non-empty node on the path from the original leaf
            // up to the root. if there were no non-empty nodes on this path, we default to
            // the first tier (i.e., depth 16).
            let target_tier = non_empty_nodes.keys().last().map(|&t| t + 1).unwrap_or(1);
            leaf_index.move_up_to(target_tier * TieredSmt::TIER_SIZE);

            return Ok(Some((leaf.into(), leaf_index)));
        }
    }

    Ok(None)
}

// HELPER FUNCTIONS
// ================================================================================================

fn get_common_prefix(key1: &Word, key2: &Word) -> u8 {
    let k1 = key1[3].as_int();
    let k2 = key2[3].as_int();
    (k1 ^ k2).leading_zeros() as u8
}

fn get_depth_flags(depth: u8) -> (Felt, Felt) {
    let is_16_or_32 = if depth == 16 || depth == 32 { ONE } else { ZERO };
    let is_16_or_48 = if depth == 16 || depth == 48 { ONE } else { ZERO };
    (is_16_or_32, is_16_or_48)
}
