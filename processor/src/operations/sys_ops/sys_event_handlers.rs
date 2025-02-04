use alloc::vec::Vec;

use vm_core::{
    crypto::hash::{Rpo256, RpoDigest},
    sys_events::SystemEvent,
    Felt, FieldElement, WORD_SIZE, ZERO,
};

use crate::{AdviceProvider, AdviceSource, ExecutionError, Host, Process, ProcessState, QuadFelt};

/// The offset of the domain value on the stack in the `hdword_to_map_with_domain` system event.
const HDWORD_TO_MAP_WITH_DOMAIN_DOMAIN_OFFSET: usize = 8;

impl Process {
    pub(super) fn handle_system_event(
        &self,
        system_event: SystemEvent,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        let advice_provider = host.advice_provider_mut();
        let process_state: ProcessState = self.into();
        match system_event {
            SystemEvent::MerkleNodeMerge => merge_merkle_nodes(advice_provider, process_state),
            SystemEvent::MerkleNodeToStack => {
                copy_merkle_node_to_adv_stack(advice_provider, process_state)
            },
            SystemEvent::MapValueToStack => {
                copy_map_value_to_adv_stack(advice_provider, process_state, false)
            },
            SystemEvent::MapValueToStackN => {
                copy_map_value_to_adv_stack(advice_provider, process_state, true)
            },
            SystemEvent::Ext2Inv => push_ext2_inv_result(advice_provider, process_state),
            SystemEvent::U32Clz => push_leading_zeros(advice_provider, process_state),
            SystemEvent::U32Ctz => push_trailing_zeros(advice_provider, process_state),
            SystemEvent::U32Clo => push_leading_ones(advice_provider, process_state),
            SystemEvent::U32Cto => push_trailing_ones(advice_provider, process_state),
            SystemEvent::ILog2 => push_ilog2(advice_provider, process_state),

            SystemEvent::MemToMap => insert_mem_values_into_adv_map(advice_provider, process_state),
            SystemEvent::HdwordToMap => {
                insert_hdword_into_adv_map(advice_provider, process_state, ZERO)
            },
            SystemEvent::HdwordToMapWithDomain => {
                let domain = self.stack.get(HDWORD_TO_MAP_WITH_DOMAIN_DOMAIN_OFFSET);
                insert_hdword_into_adv_map(advice_provider, process_state, domain)
            },
            SystemEvent::HpermToMap => insert_hperm_into_adv_map(advice_provider, process_state),
        }
    }
}

/// Reads elements from memory at the specified range and inserts them into the advice map under
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
pub fn insert_mem_values_into_adv_map(
    advice_provider: &mut impl AdviceProvider,
    process: ProcessState,
) -> Result<(), ExecutionError> {
    let (start_addr, end_addr) = get_mem_addr_range(process, 4, 5)?;
    let ctx = process.ctx();

    let mut values = Vec::with_capacity(((end_addr - start_addr) as usize) * WORD_SIZE);
    for addr in start_addr..end_addr {
        let mem_value = process.get_mem_value(ctx, addr).unwrap_or(ZERO);
        values.push(mem_value);
    }

    let key = process.get_stack_word(0);
    advice_provider.insert_into_map(key, values);

    Ok(())
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
pub fn insert_hdword_into_adv_map(
    advice_provider: &mut impl AdviceProvider,
    process: ProcessState,
    domain: Felt,
) -> Result<(), ExecutionError> {
    // get the top two words from the stack and hash them to compute the key value
    let word0 = process.get_stack_word(0);
    let word1 = process.get_stack_word(1);
    let key = Rpo256::merge_in_domain(&[word1.into(), word0.into()], domain);

    // build a vector of values from the two word and insert it into the advice map under the
    // computed key
    let mut values = Vec::with_capacity(2 * WORD_SIZE);
    values.extend_from_slice(&word1);
    values.extend_from_slice(&word0);
    advice_provider.insert_into_map(key.into(), values);

    Ok(())
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
pub fn insert_hperm_into_adv_map(
    advice_provider: &mut impl AdviceProvider,
    process: ProcessState,
) -> Result<(), ExecutionError> {
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

    advice_provider.insert_into_map(key.into(), values);

    Ok(())
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
pub fn merge_merkle_nodes(
    advice_provider: &mut impl AdviceProvider,
    process: ProcessState,
) -> Result<(), ExecutionError> {
    // fetch the arguments from the stack
    let lhs = process.get_stack_word(1);
    let rhs = process.get_stack_word(0);

    // perform the merge
    advice_provider.merge_roots(lhs, rhs)?;

    Ok(())
}

/// Pushes a node of the Merkle tree specified by the values on the top of the operand stack
/// onto the advice stack.
///
/// Inputs:
///   Operand stack: [depth, index, TREE_ROOT, ...]
///   Advice stack: [...]
///   Merkle store: {TREE_ROOT<-NODE}
///
/// Outputs:
///   Operand stack: [depth, index, TREE_ROOT, ...]
///   Advice stack: [NODE, ...]
///   Merkle store: {TREE_ROOT<-NODE}
///
/// # Errors
/// Returns an error if:
/// - Merkle tree for the specified root cannot be found in the advice provider.
/// - The specified depth is either zero or greater than the depth of the Merkle tree identified by
///   the specified root.
/// - Value of the node at the specified depth and index is not known to the advice provider.
pub fn copy_merkle_node_to_adv_stack(
    advice_provider: &mut impl AdviceProvider,
    process: ProcessState,
) -> Result<(), ExecutionError> {
    let depth = process.get_stack_item(0);
    let index = process.get_stack_item(1);
    let root = [
        process.get_stack_item(5),
        process.get_stack_item(4),
        process.get_stack_item(3),
        process.get_stack_item(2),
    ];

    let node = advice_provider.get_tree_node(root, &depth, &index)?;

    advice_provider.push_stack(AdviceSource::Value(node[3]))?;
    advice_provider.push_stack(AdviceSource::Value(node[2]))?;
    advice_provider.push_stack(AdviceSource::Value(node[1]))?;
    advice_provider.push_stack(AdviceSource::Value(node[0]))?;

    Ok(())
}

/// Pushes a list of field elements onto the advice stack. The list is looked up in the advice
/// map using the specified word from the operand stack as the key. If `include_len` is set to
/// true, the number of elements in the value is also pushed onto the advice stack.
///
/// Inputs:
///   Operand stack: [..., KEY, ...]
///   Advice stack: [...]
///   Advice map: {KEY: values}
///
/// Outputs:
///   Operand stack: [..., KEY, ...]
///   Advice stack: [values_len?, values, ...]
///   Advice map: {KEY: values}
///
/// The `key_offset` value specifies the location of the `KEY` on the stack. For example,
/// offset value of 0 indicates that the top word on the stack should be used as the key, the
/// offset value of 4, indicates that the second word on the stack should be used as the key
/// etc.
///
/// The valid values of `key_offset` are 0 through 12 (inclusive).
///
/// # Errors
/// Returns an error if the required key was not found in the key-value map or if stack offset
/// is greater than 12.
pub fn copy_map_value_to_adv_stack(
    advice_provider: &mut impl AdviceProvider,
    process: ProcessState,
    include_len: bool,
) -> Result<(), ExecutionError> {
    let key = [
        process.get_stack_item(3),
        process.get_stack_item(2),
        process.get_stack_item(1),
        process.get_stack_item(0),
    ];
    advice_provider.push_stack(AdviceSource::Map { key, include_len })?;

    Ok(())
}

/// Given an element in a quadratic extension field on the top of the stack (i.e., a0, b1),
/// computes its multiplicative inverse and push the result onto the advice stack.
///
/// Inputs:
///   Operand stack: [a1, a0, ...]
///   Advice stack: [...]
///
/// Outputs:
///   Operand stack: [a1, a0, ...]
///   Advice stack: [b0, b1...]
///
/// Where (b0, b1) is the multiplicative inverse of the extension field element (a0, a1) at the
/// top of the stack.
///
/// # Errors
/// Returns an error if the input is a zero element in the extension field.
pub fn push_ext2_inv_result(
    advice_provider: &mut impl AdviceProvider,
    process: ProcessState,
) -> Result<(), ExecutionError> {
    let coef0 = process.get_stack_item(1);
    let coef1 = process.get_stack_item(0);

    let element = QuadFelt::new(coef0, coef1);
    if element == QuadFelt::ZERO {
        return Err(ExecutionError::DivideByZero(process.clk()));
    }
    let result = element.inv().to_base_elements();

    advice_provider.push_stack(AdviceSource::Value(result[1]))?;
    advice_provider.push_stack(AdviceSource::Value(result[0]))?;

    Ok(())
}

/// Pushes the number of the leading zeros of the top stack element onto the advice stack.
///
/// Inputs:
///   Operand stack: [n, ...]
///   Advice stack: [...]
///
/// Outputs:
///   Operand stack: [n, ...]
///   Advice stack: [leading_zeros, ...]
pub fn push_leading_zeros(
    advice_provider: &mut impl AdviceProvider,
    process: ProcessState,
) -> Result<(), ExecutionError> {
    push_transformed_stack_top(advice_provider, process, |stack_top| {
        Felt::from(stack_top.leading_zeros())
    })
}

/// Pushes the number of the trailing zeros of the top stack element onto the advice stack.
///
/// Inputs:
///   Operand stack: [n, ...]
///   Advice stack: [...]
///
/// Outputs:
///   Operand stack: [n, ...]
///   Advice stack: [trailing_zeros, ...]
pub fn push_trailing_zeros(
    advice_provider: &mut impl AdviceProvider,
    process: ProcessState,
) -> Result<(), ExecutionError> {
    push_transformed_stack_top(advice_provider, process, |stack_top| {
        Felt::from(stack_top.trailing_zeros())
    })
}

/// Pushes the number of the leading ones of the top stack element onto the advice stack.
///
/// Inputs:
///   Operand stack: [n, ...]
///   Advice stack: [...]
///
/// Outputs:
///   Operand stack: [n, ...]
///   Advice stack: [leading_ones, ...]
pub fn push_leading_ones(
    advice_provider: &mut impl AdviceProvider,
    process: ProcessState,
) -> Result<(), ExecutionError> {
    push_transformed_stack_top(advice_provider, process, |stack_top| {
        Felt::from(stack_top.leading_ones())
    })
}

/// Pushes the number of the trailing ones of the top stack element onto the advice stack.
///
/// Inputs:
///   Operand stack: [n, ...]
///   Advice stack: [...]
///
/// Outputs:
///   Operand stack: [n, ...]
///   Advice stack: [trailing_ones, ...]
pub fn push_trailing_ones(
    advice_provider: &mut impl AdviceProvider,
    process: ProcessState,
) -> Result<(), ExecutionError> {
    push_transformed_stack_top(advice_provider, process, |stack_top| {
        Felt::from(stack_top.trailing_ones())
    })
}

/// Pushes the base 2 logarithm of the top stack element, rounded down.
/// Inputs:
///   Operand stack: [n, ...]
///   Advice stack: [...]
///
/// Outputs:
///   Operand stack: [n, ...]
///   Advice stack: [ilog2(n), ...]
///
/// # Errors
/// Returns an error if the logarithm argument (top stack element) equals ZERO.
pub fn push_ilog2(
    advice_provider: &mut impl AdviceProvider,
    process: ProcessState,
) -> Result<(), ExecutionError> {
    let n = process.get_stack_item(0).as_int();
    if n == 0 {
        return Err(ExecutionError::LogArgumentZero(process.clk()));
    }
    let ilog2 = Felt::from(n.ilog2());
    advice_provider.push_stack(AdviceSource::Value(ilog2))?;
    Ok(())
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

/// Gets the top stack element, applies a provided function to it and pushes it to the advice
/// provider.
fn push_transformed_stack_top<A: AdviceProvider>(
    advice_provider: &mut A,
    process: ProcessState,
    f: impl FnOnce(u32) -> Felt,
) -> Result<(), ExecutionError> {
    let stack_top = process.get_stack_item(0);
    let stack_top: u32 = stack_top
        .as_int()
        .try_into()
        .map_err(|_| ExecutionError::NotU32Value(stack_top, ZERO))?;
    let transformed_stack_top = f(stack_top);
    advice_provider.push_stack(AdviceSource::Value(transformed_stack_top))?;
    Ok(())
}
