use alloc::vec::Vec;

use vm_core::{
    Felt, FieldElement, WORD_SIZE, Word, ZERO,
    crypto::{
        hash::{Rpo256, RpoDigest},
        merkle::{EmptySubtreeRoots, SMT_DEPTH, Smt},
    },
    mast::MastNodeExt,
    sys_events::SystemEvent,
};
use winter_prover::math::fft;

use crate::{
    AdviceProvider, AdviceSource, ExecutionError, Ext2InttError, Host, MemoryError, Process,
    ProcessState, QuadFelt, errors::ErrorContext,
};

/// The offset of the domain value on the stack in the `hdword_to_map_with_domain` system event.
pub const HDWORD_TO_MAP_WITH_DOMAIN_DOMAIN_OFFSET: usize = 8;

/// Falcon signature prime.
const M: u64 = 12289;

impl Process {
    pub(super) fn handle_system_event(
        &self,
        system_event: SystemEvent,
        host: &mut impl Host,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Result<(), ExecutionError> {
        let advice_provider = host.advice_provider_mut();
        let process_state: ProcessState = self.into();
        match system_event {
            SystemEvent::MerkleNodeMerge => {
                merge_merkle_nodes(advice_provider, process_state, err_ctx)
            },
            SystemEvent::MerkleNodeToStack => {
                copy_merkle_node_to_adv_stack(advice_provider, process_state, err_ctx)
            },
            SystemEvent::MapValueToStack => {
                copy_map_value_to_adv_stack(advice_provider, process_state, false, err_ctx)
            },
            SystemEvent::MapValueToStackN => {
                copy_map_value_to_adv_stack(advice_provider, process_state, true, err_ctx)
            },
            SystemEvent::U64Div => push_u64_div_result(advice_provider, process_state, err_ctx),
            SystemEvent::FalconDiv => {
                push_falcon_mod_result(advice_provider, process_state, err_ctx)
            },
            SystemEvent::Ext2Inv => push_ext2_inv_result(advice_provider, process_state, err_ctx),
            SystemEvent::Ext2Intt => push_ext2_intt_result(advice_provider, process_state, err_ctx),
            SystemEvent::SmtPeek => push_smtpeek_result(advice_provider, process_state, err_ctx),
            SystemEvent::U32Clz => push_leading_zeros(advice_provider, process_state, err_ctx),
            SystemEvent::U32Ctz => push_trailing_zeros(advice_provider, process_state, err_ctx),
            SystemEvent::U32Clo => push_leading_ones(advice_provider, process_state, err_ctx),
            SystemEvent::U32Cto => push_trailing_ones(advice_provider, process_state, err_ctx),
            SystemEvent::ILog2 => push_ilog2(advice_provider, process_state, err_ctx),

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
    err_ctx: &ErrorContext<'_, impl MastNodeExt>,
) -> Result<(), ExecutionError> {
    // fetch the arguments from the stack
    let lhs = process.get_stack_word(1);
    let rhs = process.get_stack_word(0);

    // perform the merge
    advice_provider.merge_roots(lhs, rhs, err_ctx)?;

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
    err_ctx: &ErrorContext<'_, impl MastNodeExt>,
) -> Result<(), ExecutionError> {
    let depth = process.get_stack_item(0);
    let index = process.get_stack_item(1);
    let root = [
        process.get_stack_item(5),
        process.get_stack_item(4),
        process.get_stack_item(3),
        process.get_stack_item(2),
    ];

    let node = advice_provider.get_tree_node(root, &depth, &index, err_ctx)?;

    advice_provider.push_stack(AdviceSource::Value(node[3]), err_ctx)?;
    advice_provider.push_stack(AdviceSource::Value(node[2]), err_ctx)?;
    advice_provider.push_stack(AdviceSource::Value(node[1]), err_ctx)?;
    advice_provider.push_stack(AdviceSource::Value(node[0]), err_ctx)?;

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
    err_ctx: &ErrorContext<'_, impl MastNodeExt>,
) -> Result<(), ExecutionError> {
    let key = [
        process.get_stack_item(3),
        process.get_stack_item(2),
        process.get_stack_item(1),
        process.get_stack_item(0),
    ];
    advice_provider.push_stack(AdviceSource::Map { key, include_len }, err_ctx)?;

    Ok(())
}

/// Pushes the result of [u64] division (both the quotient and the remainder) onto the advice
/// stack.
///
/// Inputs:
///   Operand stack: [b1, b0, a1, a0, ...]
///   Advice stack: [...]
///
/// Outputs:
///   Operand stack: [b1, b0, a1, a0, ...]
///   Advice stack: [q0, q1, r0, r1, ...]
///
/// Where (a0, a1) and (b0, b1) are the 32-bit limbs of the dividend and the divisor
/// respectively (with a0 representing the 32 lest significant bits and a1 representing the
/// 32 most significant bits). Similarly, (q0, q1) and (r0, r1) represent the quotient and
/// the remainder respectively.
///
/// # Errors
/// Returns an error if the divisor is ZERO.
pub fn push_u64_div_result(
    advice_provider: &mut impl AdviceProvider,
    process: ProcessState,
    err_ctx: &ErrorContext<'_, impl MastNodeExt>,
) -> Result<(), ExecutionError> {
    let divisor = {
        let divisor_hi = process.get_stack_item(0).as_int();
        let divisor_lo = process.get_stack_item(1).as_int();

        // Ensure the divisor is a pair of u32 values
        if divisor_hi > u32::MAX.into() {
            return Err(ExecutionError::NotU32Value(Felt::new(divisor_hi), ZERO));
        }
        if divisor_lo > u32::MAX.into() {
            return Err(ExecutionError::NotU32Value(Felt::new(divisor_lo), ZERO));
        }

        let divisor = (divisor_hi << 32) + divisor_lo;

        if divisor == 0 {
            return Err(ExecutionError::divide_by_zero(process.clk(), err_ctx));
        }

        divisor
    };

    let dividend = {
        let dividend_hi = process.get_stack_item(2).as_int();
        let dividend_lo = process.get_stack_item(3).as_int();

        // Ensure the dividend is a pair of u32 values
        if dividend_hi > u32::MAX.into() {
            return Err(ExecutionError::NotU32Value(Felt::new(dividend_hi), ZERO));
        }
        if dividend_lo > u32::MAX.into() {
            return Err(ExecutionError::NotU32Value(Felt::new(dividend_lo), ZERO));
        }

        (dividend_hi << 32) + dividend_lo
    };

    let quotient = dividend / divisor;
    let remainder = dividend - quotient * divisor;

    let (q_hi, q_lo) = u64_to_u32_elements(quotient);
    let (r_hi, r_lo) = u64_to_u32_elements(remainder);

    advice_provider.push_stack(AdviceSource::Value(r_hi), err_ctx)?;
    advice_provider.push_stack(AdviceSource::Value(r_lo), err_ctx)?;
    advice_provider.push_stack(AdviceSource::Value(q_hi), err_ctx)?;
    advice_provider.push_stack(AdviceSource::Value(q_lo), err_ctx)?;

    Ok(())
}

/// Pushes the result of divison (both the quotient and the remainder) of a [u64] by the Falcon
/// prime (M = 12289) onto the advice stack.
///
/// Inputs:
///   Operand stack: [a1, a0, ...]
///   Advice stack: [...]
///
/// Outputs:
///   Operand stack: [a1, a0, ...]
///   Advice stack: [q1, q0, r, ...]
///
/// Where (a0, a1) are the 32-bit limbs of the dividend (with a0 representing the 32 least
/// significant bits and a1 representing the 32 most significant bits).
/// Similarly, (q0, q1) represent the quotient and r the remainder.
///
/// # Errors
/// Returns an error if the divisor is ZERO.
pub fn push_falcon_mod_result(
    advice_provider: &mut impl AdviceProvider,
    process: ProcessState,
    err_ctx: &ErrorContext<'_, impl MastNodeExt>,
) -> Result<(), ExecutionError> {
    let dividend_hi = process.get_stack_item(0).as_int();
    let dividend_lo = process.get_stack_item(1).as_int();
    let dividend = (dividend_hi << 32) + dividend_lo;

    let (quotient, remainder) = (dividend / M, dividend % M);

    let (q_hi, q_lo) = u64_to_u32_elements(quotient);
    let (r_hi, r_lo) = u64_to_u32_elements(remainder);
    assert_eq!(r_hi, ZERO);

    advice_provider.push_stack(AdviceSource::Value(r_lo), err_ctx)?;
    advice_provider.push_stack(AdviceSource::Value(q_lo), err_ctx)?;
    advice_provider.push_stack(AdviceSource::Value(q_hi), err_ctx)?;

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
    err_ctx: &ErrorContext<'_, impl MastNodeExt>,
) -> Result<(), ExecutionError> {
    let coef0 = process.get_stack_item(1);
    let coef1 = process.get_stack_item(0);

    let element = QuadFelt::new(coef0, coef1);
    if element == QuadFelt::ZERO {
        return Err(ExecutionError::divide_by_zero(process.clk(), err_ctx));
    }
    let result = element.inv().to_base_elements();

    advice_provider.push_stack(AdviceSource::Value(result[1]), err_ctx)?;
    advice_provider.push_stack(AdviceSource::Value(result[0]), err_ctx)?;

    Ok(())
}

/// Given evaluations of a polynomial over some specified domain, interpolates the evaluations
///  into a polynomial in coefficient form and pushes the result into the advice stack.
///
/// The interpolation is performed using the iNTT algorithm. The evaluations are expected to be
/// in the quadratic extension.
///
/// Inputs:
///   Operand stack: [output_size, input_size, input_start_ptr, ...]
///   Advice stack: [...]
///
/// Outputs:
///   Operand stack: [output_size, input_size, input_start_ptr, ...]
///   Advice stack: [coefficients...]
///
/// - `input_size` is the number of evaluations (each evaluation is 2 base field elements). Must be
///   a power of 2 and greater 1.
/// - `output_size` is the number of coefficients in the interpolated polynomial (each coefficient
///   is 2 base field elements). Must be smaller than or equal to the number of input evaluations.
/// - `input_start_ptr` is the memory address of the first evaluation.
/// - `coefficients` are the coefficients of the interpolated polynomial such that lowest degree
///   coefficients are located at the top of the advice stack.
///
/// # Errors
/// Returns an error if:
/// - `input_size` less than or equal to 1, or is not a power of 2.
/// - `output_size` is 0 or is greater than the `input_size`.
/// - `input_ptr` is greater than 2^32, or is not aligned on a word boundary.
/// - `input_ptr + input_size * 2` is greater than 2^32.
pub fn push_ext2_intt_result(
    advice_provider: &mut impl AdviceProvider,
    process: ProcessState,
    err_ctx: &ErrorContext<'_, impl MastNodeExt>,
) -> Result<(), ExecutionError> {
    let output_size = process.get_stack_item(0).as_int() as usize;
    let input_size = process.get_stack_item(1).as_int() as usize;
    let input_start_ptr = process.get_stack_item(2).as_int();

    if input_size <= 1 {
        return Err(Ext2InttError::DomainSizeTooSmall(input_size as u64).into());
    }
    if !input_size.is_power_of_two() {
        return Err(Ext2InttError::DomainSizeNotPowerOf2(input_size as u64).into());
    }
    if input_start_ptr >= u32::MAX as u64 {
        return Err(Ext2InttError::InputStartAddressTooBig(input_start_ptr).into());
    }
    if input_start_ptr % WORD_SIZE as u64 != 0 {
        return Err(Ext2InttError::InputStartNotWordAligned(input_start_ptr).into());
    }
    if input_size > u32::MAX as usize {
        return Err(Ext2InttError::InputSizeTooBig(input_size as u64).into());
    }

    let input_end_ptr = input_start_ptr + (input_size * 2) as u64;
    if input_end_ptr > u32::MAX as u64 {
        return Err(Ext2InttError::InputEndAddressTooBig(input_end_ptr).into());
    }

    if output_size == 0 {
        return Err(Ext2InttError::OutputSizeIsZero.into());
    }
    if output_size > input_size {
        return Err(Ext2InttError::OutputSizeTooBig(output_size, input_size).into());
    }

    let mut poly = Vec::with_capacity(input_size);
    for addr in ((input_start_ptr as u32)..(input_end_ptr as u32)).step_by(4) {
        let word = process
            .get_mem_word(process.ctx(), addr)?
            .ok_or(Ext2InttError::UninitializedMemoryAddress(addr))?;

        poly.push(QuadFelt::new(word[0], word[1]));
        poly.push(QuadFelt::new(word[2], word[3]));
    }

    let twiddles = fft::get_inv_twiddles::<Felt>(input_size);
    fft::interpolate_poly::<Felt, QuadFelt>(&mut poly, &twiddles);

    for element in QuadFelt::slice_as_base_elements(&poly[..output_size]).iter().rev() {
        advice_provider.push_stack(AdviceSource::Value(*element), err_ctx)?;
    }

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
    err_ctx: &ErrorContext<'_, impl MastNodeExt>,
) -> Result<(), ExecutionError> {
    push_transformed_stack_top(
        advice_provider,
        process,
        |stack_top| Felt::from(stack_top.leading_zeros()),
        err_ctx,
    )
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
    err_ctx: &ErrorContext<'_, impl MastNodeExt>,
) -> Result<(), ExecutionError> {
    push_transformed_stack_top(
        advice_provider,
        process,
        |stack_top| Felt::from(stack_top.trailing_zeros()),
        err_ctx,
    )
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
    err_ctx: &ErrorContext<'_, impl MastNodeExt>,
) -> Result<(), ExecutionError> {
    push_transformed_stack_top(
        advice_provider,
        process,
        |stack_top| Felt::from(stack_top.leading_ones()),
        err_ctx,
    )
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
    err_ctx: &ErrorContext<'_, impl MastNodeExt>,
) -> Result<(), ExecutionError> {
    push_transformed_stack_top(
        advice_provider,
        process,
        |stack_top| Felt::from(stack_top.trailing_ones()),
        err_ctx,
    )
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
    err_ctx: &ErrorContext<'_, impl MastNodeExt>,
) -> Result<(), ExecutionError> {
    let n = process.get_stack_item(0).as_int();
    if n == 0 {
        return Err(ExecutionError::log_argument_zero(process.clk(), err_ctx));
    }
    let ilog2 = Felt::from(n.ilog2());
    advice_provider.push_stack(AdviceSource::Value(ilog2), err_ctx)?;
    Ok(())
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
pub fn push_smtpeek_result(
    advice_provider: &mut impl AdviceProvider,
    process: ProcessState,
    err_ctx: &ErrorContext<'_, impl MastNodeExt>,
) -> Result<(), ExecutionError> {
    let empty_leaf = EmptySubtreeRoots::entry(SMT_DEPTH, SMT_DEPTH);
    // fetch the arguments from the operand stack
    let key = process.get_stack_word(0);
    let root = process.get_stack_word(1);

    // get the node from the SMT for the specified key; this node can be either a leaf node,
    // or a root of an empty subtree at the returned depth
    let node =
        advice_provider.get_tree_node(root, &Felt::new(SMT_DEPTH as u64), &key[3], err_ctx)?;

    if node == Word::from(empty_leaf) {
        // if the node is a root of an empty subtree, then there is no value associated with
        // the specified key
        advice_provider.push_stack(AdviceSource::Word(Smt::EMPTY_VALUE), err_ctx)?;
    } else {
        let leaf_preimage = get_smt_leaf_preimage(advice_provider, node)?;

        for (key_in_leaf, value_in_leaf) in leaf_preimage {
            if key == key_in_leaf {
                // Found key - push value associated with key, and return
                advice_provider.push_stack(AdviceSource::Word(value_in_leaf), err_ctx)?;

                return Ok(());
            }
        }

        // if we can't find any key in the leaf that matches `key`, it means no value is
        // associated with `key`
        advice_provider.push_stack(AdviceSource::Word(Smt::EMPTY_VALUE), err_ctx)?;
    }
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
        return Err(ExecutionError::MemoryError(MemoryError::address_out_of_bounds(
            start_addr,
            &ErrorContext::default(),
        )));
    }
    if end_addr > u32::MAX as u64 {
        return Err(ExecutionError::MemoryError(MemoryError::address_out_of_bounds(
            end_addr,
            &ErrorContext::default(),
        )));
    }

    if start_addr > end_addr {
        return Err(ExecutionError::MemoryError(MemoryError::InvalidMemoryRange {
            start_addr,
            end_addr,
        }));
    }

    Ok((start_addr as u32, end_addr as u32))
}

fn u64_to_u32_elements(value: u64) -> (Felt, Felt) {
    let hi = Felt::from((value >> 32) as u32);
    let lo = Felt::from(value as u32);
    (hi, lo)
}

/// Gets the top stack element, applies a provided function to it and pushes it to the advice
/// provider.
fn push_transformed_stack_top<A: AdviceProvider>(
    advice_provider: &mut A,
    process: ProcessState,
    f: impl FnOnce(u32) -> Felt,
    err_ctx: &ErrorContext<'_, impl MastNodeExt>,
) -> Result<(), ExecutionError> {
    let stack_top = process.get_stack_item(0);
    let stack_top: u32 = stack_top
        .as_int()
        .try_into()
        .map_err(|_| ExecutionError::NotU32Value(stack_top, ZERO))?;
    let transformed_stack_top = f(stack_top);
    advice_provider.push_stack(AdviceSource::Value(transformed_stack_top), err_ctx)?;
    Ok(())
}

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
