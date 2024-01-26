use super::super::{AdviceSource, ExecutionError, Felt, HostResponse, Word};
use crate::{AdviceProvider, ProcessState};
use vm_core::{
    crypto::{
        hash::RpoDigest,
        merkle::{EmptySubtreeRoots, Smt, SMT_DEPTH},
    },
    utils::collections::Vec,
    WORD_SIZE,
};

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
    _advice_provider: &mut A,
    _process: &S,
) -> Result<HostResponse, ExecutionError> {
    unimplemented!()
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
pub(crate) fn push_smtpeek_result<S: ProcessState, A: AdviceProvider>(
    advice_provider: &mut A,
    process: &S,
) -> Result<HostResponse, ExecutionError> {
    let empty_leaf = EmptySubtreeRoots::entry(SMT_DEPTH, SMT_DEPTH);
    // fetch the arguments from the operand stack
    let key = process.get_stack_word(0);
    let root = process.get_stack_word(1);

    // get the node from the SMT for the specified key; this node can be either a leaf node,
    // or a root of an empty subtree at the returned depth
    let node = advice_provider.get_tree_node(root, &Felt::new(SMT_DEPTH as u64), &key[3])?;

    if node == Word::from(empty_leaf) {
        // if the node is a root of an empty subtree, then there is no value associated with
        // the specified key
        advice_provider.push_stack(AdviceSource::Word(Smt::EMPTY_VALUE))?;
    } else {
        let leaf_preimage = get_smt_leaf_preimage(advice_provider, node)?;

        for (key_in_leaf, value_in_leaf) in leaf_preimage {
            if key == key_in_leaf {
                // Found key - push value associated with key, and return
                advice_provider.push_stack(AdviceSource::Word(value_in_leaf))?;

                return Ok(HostResponse::None);
            }
        }

        // if we can't find any key in the leaf that matches `key`, it means no value is associated
        // with `key`
        advice_provider.push_stack(AdviceSource::Word(Smt::EMPTY_VALUE))?;
    }

    Ok(HostResponse::None)
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
    _advice_provider: &mut A,
    _process: &S,
) -> Result<HostResponse, ExecutionError> {
    unimplemented!()
}

// HELPER METHODS
// --------------------------------------------------------------------------------------------

fn get_smt_leaf_preimage<A: AdviceProvider>(
    advice_provider: &A,
    node: Word,
) -> Result<Vec<(Word, Word)>, ExecutionError> {
    let node_bytes = RpoDigest::from(node).as_bytes();

    let kv_pairs = advice_provider
        .get_mapped_values(&node_bytes)
        .ok_or(ExecutionError::AdviceMapKeyNotFound(node))?;

    if kv_pairs.len() % WORD_SIZE * 2 != 0 {
        return Err(ExecutionError::AdviceMapValueInvalidLength(node, kv_pairs.len()));
    }

    Ok(kv_pairs
        .chunks_exact(WORD_SIZE * 2)
        .map(|kv_chunk| {
            let key = [kv_chunk[0], kv_chunk[1], kv_chunk[2], kv_chunk[3]];
            let value = [kv_chunk[4], kv_chunk[5], kv_chunk[6], kv_chunk[7]];

            (key, value)
        })
        .collect())
}
