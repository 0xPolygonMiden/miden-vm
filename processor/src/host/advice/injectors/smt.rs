use super::super::{AdviceSource, ExecutionError, Felt, HostResponse, Word};
use crate::{ AdviceProvider, ProcessState};
use alloc::vec::Vec;
use vm_core::{
    crypto::{
        hash::RpoDigest,
        merkle::{EmptySubtreeRoots, Smt, SMT_DEPTH},
    },
    WORD_SIZE,
};

// SMT INJECTORS
// ================================================================================================

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

/// Given a key and root associated with a Sparse Merkle Tree, pushes the value associated with the key onto the
/// advice stack along with the length of the leaf node containing the key-value pair.
/// Inputs:
///  Operand stack: [KEY, ROOT, ...]
///  Advice stack: [...]
/// 
/// Outputs:
///  Operand stack: [KEY, ROOT, ...]
///  Advice stack: [ZERO/ONE(empty or non_empty_leaf), LEAF_LENGTH, VALUE, ...]
/// 
/// Errors:
/// Returns an error if the provided Merkle root doesn't exist on the advice provider.
pub(crate) fn push_smtget<S: ProcessState, A: AdviceProvider>(
    advice_provider: &mut A,
    process: &S,
) -> Result<HostResponse, ExecutionError> {
    let empty_leaf = EmptySubtreeRoots::entry(SMT_DEPTH, SMT_DEPTH);

    let key = process.get_stack_word(0);
    let root = process.get_stack_word(1);

    let node = advice_provider.get_tree_node(root, &Felt::new(SMT_DEPTH as u64), &key[3])?; 

    if node == Word::from(empty_leaf) {
        // if the node is a root of an empty subtree, then there is no value associated with
        // the specified key

        // advice stack: [ZERO(empty_leaf), ZERO(leaf_length), VALUE(empty_value), ...]
        advice_provider.push_stack(AdviceSource::Word(Smt::EMPTY_VALUE))?;
        // advice_provider.push_stack(AdviceSource::Value(Felt::new(0)))?;
        // advice_provider.push_stack(AdviceSource::Value(Felt::new(0)))?;
    } else {
        let leaf_preimage = get_smt_leaf_preimage(advice_provider, node)?;

        for (key_in_leaf, value_in_leaf) in &leaf_preimage {
            if key == *key_in_leaf {
                // Found key - push value associated with key, and return

                // advice stack: [ONE(non_empty_leaf), LEAF_LENGTH, VALUE(value), ...]
                advice_provider.push_stack(AdviceSource::Word(*value_in_leaf))?;
                // advice_provider.push_stack(AdviceSource::Value(Felt::new(leaf_preimage.len() as u64)))?;
                // advice_provider.push_stack(AdviceSource::Value(Felt::new(1)))?;

                return Ok(HostResponse::None);
            } 
        }

        // if we can't find any key in the leaf that matches `key`, it means no value is associated
        // with `key`

        // advice stack: [ONE(non_empty_leaf), LEAF_LENGTH, VALUE(empty_value), ...]
        advice_provider.push_stack(AdviceSource::Word(Smt::EMPTY_VALUE))?;
        advice_provider.push_stack(AdviceSource::Value(Felt::new(leaf_preimage.len() as u64)))?;
        advice_provider.push_stack(AdviceSource::Value(Felt::new(1)))?;
    
    }
    Ok(HostResponse::None)
}

/// Pushes indicators onto the advice stack required for inserting
/// a new key-value pair into a Sparse Merkle Tree associated with the specified root.
/// Inputs:
///  Operand stack: [VALUE, KEY, ROOT, ...]
///  Advice stack: [...]
/// 
/// Outputs:
///  Operand stack: [VALUE, KEY, ROOT, ...]
///  Advice stack: [LEAF_LENGTH, ZERO(is_empty_subtree)/ONE(is_update), ...]
/// 
/// Errors:
/// Returns an error if the provided Merkle root doesn't exist on the advice provider.
pub(crate) fn push_smtset<S: ProcessState, A: AdviceProvider>(
    advice_provider: &mut A,
    process: &S,
) -> Result<HostResponse, ExecutionError> {
    let empty_leaf = EmptySubtreeRoots::entry(SMT_DEPTH, SMT_DEPTH);

    let key = process.get_stack_word(1);
    let root = process.get_stack_word(2);

    let node = advice_provider.get_tree_node(root, &Felt::new(SMT_DEPTH as u64), &key[3])?; 

 
    // - if the node is a root of an empty subtree, we need to insert a new leaf node
    // - otherwise, we need to update the value associated with the key in the leaf node

    if node == Word::from(empty_leaf) {
        // advice stack: [ZERO(leaf_length), ZERO(is_empty_subtree), ...]
        advice_provider.push_stack(AdviceSource::Value(Felt::new(0)))?;
        advice_provider.push_stack(AdviceSource::Value(Felt::new(0)))?;
    } else {
        let leaf_preimage = get_smt_leaf_preimage(advice_provider, node)?;
        for (key_in_leaf, _value_in_leaf) in &leaf_preimage {
            if key == *key_in_leaf {
                // Found key - push value associated with key, and return
                // advice stack: [LEAF_LENGTH, ONE(is_update), ...]
                advice_provider.push_stack(AdviceSource::Value(Felt::new(1)))?;
                advice_provider.push_stack(AdviceSource::Value(Felt::new(leaf_preimage.len() as u64)))?;

                return Ok(HostResponse::None);
            } 
        }

    }
    Ok(HostResponse::None)

}

// HELPER METHODS
// --------------------------------------------------------------------------------------------

fn get_smt_leaf_preimage<A: AdviceProvider>(
    advice_provider: &A,
    node: Word,
) -> Result<Vec<(Word, Word)>, ExecutionError> {
    let node_bytes = RpoDigest::from(node);

    let kv_pairs = advice_provider
        .get_mapped_values(&node_bytes)
        .ok_or(ExecutionError::SmtNodeNotFound(node))?;

    if kv_pairs.len() % WORD_SIZE * 2 != 0 {
        return Err(ExecutionError::SmtNodePreImageNotValid(node, kv_pairs.len()));
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
