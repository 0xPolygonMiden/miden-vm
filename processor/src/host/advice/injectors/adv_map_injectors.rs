use alloc::vec::Vec;

use vm_core::{
    crypto::hash::{Rpo256, RpoDigest},
    EMPTY_WORD, WORD_SIZE,
};

use super::super::{AdviceProvider, ExecutionError, Felt, HostResponse};
use crate::ProcessState;

// ADVICE MAP INJECTORS
// ================================================================================================

/// Reads words from memory at the specified range and inserts them into the advice map under
/// the key `KEY` located at the top of the stack.
///
/// Inputs:
///   Operand stack: [KEY, start_addr, end_addr, ...]
///   Advice map: {...}
///
/// Outputs:
///   Operand stack: [KEY, start_addr, end_addr, ...]
///   Advice map: {KEY: values}
///
/// Where `values` are the elements located in memory[start_addr..end_addr].
///
/// # Errors
/// Returns an error:
/// - `start_addr` is greater than or equal to 2^32.
/// - `end_addr` is greater than or equal to 2^32.
/// - `start_addr` > `end_addr`.
pub(crate) fn insert_mem_values_into_adv_map<A: AdviceProvider>(
    advice_provider: &mut A,
    process: ProcessState,
) -> Result<HostResponse, ExecutionError> {
    let (start_addr, end_addr) = get_mem_addr_range(process, 4, 5)?;
    let ctx = process.ctx();

    let mut values = Vec::with_capacity(((end_addr - start_addr) as usize) * WORD_SIZE);
    for addr in start_addr..end_addr {
        let mem_value = process.get_mem_value(ctx, addr).unwrap_or(EMPTY_WORD);
        values.extend_from_slice(&mem_value);
    }

    let key = process.get_stack_word(0);
    advice_provider.insert_into_map(key, values)?;

    Ok(HostResponse::None)
}

/// Reads two word from the operand stack and inserts them into the advice map under the key
/// defined by the hash of these words.
///
/// Inputs:
///   Operand stack: [B, A, ...]
///   Advice map: {...}
///
/// Outputs:
///   Operand stack: [B, A, ...]
///   Advice map: {KEY: [a0, a1, a2, a3, b0, b1, b2, b3]}
///
/// Where KEY is computed as hash(A || B, domain), where domain is provided via the immediate
/// value.
pub(crate) fn insert_hdword_into_adv_map<A: AdviceProvider>(
    advice_provider: &mut A,
    process: ProcessState,
    domain: Felt,
) -> Result<HostResponse, ExecutionError> {
    // get the top two words from the stack and hash them to compute the key value
    let word0 = process.get_stack_word(0);
    let word1 = process.get_stack_word(1);
    let key = Rpo256::merge_in_domain(&[word1.into(), word0.into()], domain);

    // build a vector of values from the two word and insert it into the advice map under the
    // computed key
    let mut values = Vec::with_capacity(2 * WORD_SIZE);
    values.extend_from_slice(&word1);
    values.extend_from_slice(&word0);
    advice_provider.insert_into_map(key.into(), values)?;

    Ok(HostResponse::None)
}

/// Reads three words from the operand stack and inserts the top two words into the advice map
/// under the key defined by applying an RPO permutation to all three words.
///
/// Inputs:
///   Operand stack: [B, A, C, ...]
///   Advice map: {...}
///
/// Outputs:
///   Operand stack: [B, A, C, ...]
///   Advice map: {KEY: [a0, a1, a2, a3, b0, b1, b2, b3]}
///
/// Where KEY is computed by extracting the digest elements from hperm([C, A, B]). For example,
/// if C is [0, d, 0, 0], KEY will be set as hash(A || B, d).
pub(crate) fn insert_hperm_into_adv_map<A: AdviceProvider>(
    advice_provider: &mut A,
    process: ProcessState,
) -> Result<HostResponse, ExecutionError> {
    // read the state from the stack
    let mut state = [
        process.get_stack_item(11),
        process.get_stack_item(10),
        process.get_stack_item(9),
        process.get_stack_item(8),
        process.get_stack_item(7),
        process.get_stack_item(6),
        process.get_stack_item(5),
        process.get_stack_item(4),
        process.get_stack_item(3),
        process.get_stack_item(2),
        process.get_stack_item(1),
        process.get_stack_item(0),
    ];

    // get the values to be inserted into the advice map from the state
    let values = state[Rpo256::RATE_RANGE].to_vec();

    // apply the permutation to the state and extract the key from it
    Rpo256::apply_permutation(&mut state);
    let key = RpoDigest::new(
        state[Rpo256::DIGEST_RANGE]
            .try_into()
            .expect("failed to extract digest from state"),
    );

    advice_provider.insert_into_map(key.into(), values)?;

    Ok(HostResponse::None)
}

/// Creates a new Merkle tree in the advice provider by combining Merkle trees with the
/// specified roots. The root of the new tree is defined as `Hash(LEFT_ROOT, RIGHT_ROOT)`.
///
/// Inputs:
///   Operand stack: [RIGHT_ROOT, LEFT_ROOT, ...]
///   Merkle store: {RIGHT_ROOT, LEFT_ROOT}
///
/// Outputs:
///   Operand stack: [RIGHT_ROOT, LEFT_ROOT, ...]
///   Merkle store: {RIGHT_ROOT, LEFT_ROOT, hash(LEFT_ROOT, RIGHT_ROOT)}
///
/// After the operation, both the original trees and the new tree remains in the advice
/// provider (i.e., the input trees are not removed).
///
/// It is not checked whether the provided roots exist as Merkle trees in the advide providers.
pub(crate) fn merge_merkle_nodes<A: AdviceProvider>(
    advice_provider: &mut A,
    process: ProcessState,
) -> Result<HostResponse, ExecutionError> {
    // fetch the arguments from the stack
    let lhs = process.get_stack_word(1);
    let rhs = process.get_stack_word(0);

    // perform the merge
    advice_provider.merge_roots(lhs, rhs)?;

    Ok(HostResponse::None)
}

// HELPER METHODS
// --------------------------------------------------------------------------------------------

/// Reads (start_addr, end_addr) tuple from the specified elements of the operand stack (
/// without modifying the state of the stack), and verifies that memory range is valid.
fn get_mem_addr_range(
    process: ProcessState,
    start_idx: usize,
    end_idx: usize,
) -> Result<(u32, u32), ExecutionError> {
    let start_addr = process.get_stack_item(start_idx).as_int();
    let end_addr = process.get_stack_item(end_idx).as_int();

    if start_addr > u32::MAX as u64 {
        return Err(ExecutionError::MemoryAddressOutOfBounds(start_addr));
    }
    if end_addr > u32::MAX as u64 {
        return Err(ExecutionError::MemoryAddressOutOfBounds(end_addr));
    }

    if start_addr > end_addr {
        return Err(ExecutionError::InvalidMemoryRange { start_addr, end_addr });
    }

    Ok((start_addr as u32, end_addr as u32))
}
