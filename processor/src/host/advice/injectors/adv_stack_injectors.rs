use alloc::vec::Vec;

use vm_core::{QuadExtension, SignatureKind};
use winter_prover::math::fft;

use super::super::{AdviceSource, ExecutionError, Felt, HostResponse};
use crate::{AdviceProvider, Ext2InttError, FieldElement, ProcessState, ZERO};

// TYPE ALIASES
// ================================================================================================
type QuadFelt = QuadExtension<Felt>;

// ADVICE STACK INJECTORS
// ================================================================================================

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
pub(crate) fn copy_merkle_node_to_adv_stack<A: AdviceProvider>(
    advice_provider: &mut A,
    process: ProcessState,
) -> Result<HostResponse, ExecutionError> {
    // read node depth, node index, and tree root from the stack
    let depth = process.get_stack_item(0);
    let index = process.get_stack_item(1);
    let root = [
        process.get_stack_item(5),
        process.get_stack_item(4),
        process.get_stack_item(3),
        process.get_stack_item(2),
    ];

    // look up the node in the advice provider
    let node = advice_provider.get_tree_node(root, &depth, &index)?;

    // push the node onto the advice stack with the first element pushed last so that it can
    // be popped first (i.e. stack behavior for word)
    advice_provider.push_stack(AdviceSource::Value(node[3]))?;
    advice_provider.push_stack(AdviceSource::Value(node[2]))?;
    advice_provider.push_stack(AdviceSource::Value(node[1]))?;
    advice_provider.push_stack(AdviceSource::Value(node[0]))?;

    Ok(HostResponse::None)
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
pub(crate) fn copy_map_value_to_adv_stack<A: AdviceProvider>(
    advice_provider: &mut A,
    process: ProcessState,
    include_len: bool,
    key_offset: usize,
) -> Result<HostResponse, ExecutionError> {
    if key_offset > 12 {
        return Err(ExecutionError::InvalidStackWordOffset(key_offset));
    }

    let key = [
        process.get_stack_item(key_offset + 3),
        process.get_stack_item(key_offset + 2),
        process.get_stack_item(key_offset + 1),
        process.get_stack_item(key_offset),
    ];
    advice_provider.push_stack(AdviceSource::Map { key, include_len })?;

    Ok(HostResponse::None)
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
pub(crate) fn push_u64_div_result<A: AdviceProvider>(
    advice_provider: &mut A,
    process: ProcessState,
) -> Result<HostResponse, ExecutionError> {
    let divisor_hi = process.get_stack_item(0).as_int();
    let divisor_lo = process.get_stack_item(1).as_int();
    let divisor = (divisor_hi << 32) + divisor_lo;

    if divisor == 0 {
        return Err(ExecutionError::DivideByZero(process.clk()));
    }

    let dividend_hi = process.get_stack_item(2).as_int();
    let dividend_lo = process.get_stack_item(3).as_int();
    let dividend = (dividend_hi << 32) + dividend_lo;

    let quotient = dividend / divisor;
    let remainder = dividend - quotient * divisor;

    let (q_hi, q_lo) = u64_to_u32_elements(quotient);
    let (r_hi, r_lo) = u64_to_u32_elements(remainder);

    advice_provider.push_stack(AdviceSource::Value(r_hi))?;
    advice_provider.push_stack(AdviceSource::Value(r_lo))?;
    advice_provider.push_stack(AdviceSource::Value(q_hi))?;
    advice_provider.push_stack(AdviceSource::Value(q_lo))?;

    Ok(HostResponse::None)
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
pub(crate) fn push_ext2_inv_result<A: AdviceProvider>(
    advice_provider: &mut A,
    process: ProcessState,
) -> Result<HostResponse, ExecutionError> {
    let coef0 = process.get_stack_item(1);
    let coef1 = process.get_stack_item(0);

    let element = QuadFelt::new(coef0, coef1);
    if element == QuadFelt::ZERO {
        return Err(ExecutionError::DivideByZero(process.clk()));
    }
    let result = element.inv().to_base_elements();

    advice_provider.push_stack(AdviceSource::Value(result[1]))?;
    advice_provider.push_stack(AdviceSource::Value(result[0]))?;

    Ok(HostResponse::None)
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
/// - `input_ptr` is greater than 2^32.
/// - `input_ptr + input_size / 2` is greater than 2^32.
pub(crate) fn push_ext2_intt_result<A: AdviceProvider>(
    advice_provider: &mut A,
    process: ProcessState,
) -> Result<HostResponse, ExecutionError> {
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
    if input_size > u32::MAX as usize {
        return Err(Ext2InttError::InputSizeTooBig(input_size as u64).into());
    }

    let input_end_ptr = input_start_ptr + (input_size / 2) as u64;
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
    for addr in (input_start_ptr as u32)..(input_end_ptr as u32) {
        let word = process
            .get_mem_value(process.ctx(), addr)
            .ok_or(Ext2InttError::UninitializedMemoryAddress(addr))?;

        poly.push(QuadFelt::new(word[0], word[1]));
        poly.push(QuadFelt::new(word[2], word[3]));
    }

    let twiddles = fft::get_inv_twiddles::<Felt>(input_size);
    fft::interpolate_poly::<Felt, QuadFelt>(&mut poly, &twiddles);

    for element in QuadFelt::slice_as_base_elements(&poly[..output_size]).iter().rev() {
        advice_provider.push_stack(AdviceSource::Value(*element))?;
    }

    Ok(HostResponse::None)
}

/// Pushes values onto the advice stack which are required for verification of a DSA in Miden VM.
///
/// Inputs:
///   Operand stack: [PK, MSG, ...]
///   Advice stack: [...]
///
/// Outputs:
///   Operand stack: [PK, MSG, ...]
///   Advice stack: [DATA]
///
/// Where:
/// - PK is the digest of an expanded public.
/// - MSG is the digest of the message to be signed.
/// - DATA is the needed data for signature verification in the VM.
///
/// The advice provider is expected to contain the private key associated to the public key PK.
pub(crate) fn push_signature<A: AdviceProvider>(
    advice_provider: &mut A,
    process: ProcessState,
    kind: SignatureKind,
) -> Result<HostResponse, ExecutionError> {
    let pub_key = process.get_stack_word(0);
    let msg = process.get_stack_word(1);
    let result: Vec<Felt> = advice_provider.get_signature(kind, pub_key, msg)?;
    for r in result {
        advice_provider.push_stack(AdviceSource::Value(r))?;
    }
    Ok(HostResponse::None)
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
pub(crate) fn push_leading_zeros<A: AdviceProvider>(
    advice_provider: &mut A,
    process: ProcessState,
) -> Result<HostResponse, ExecutionError> {
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
pub(crate) fn push_trailing_zeros<A: AdviceProvider>(
    advice_provider: &mut A,
    process: ProcessState,
) -> Result<HostResponse, ExecutionError> {
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
pub(crate) fn push_leading_ones<A: AdviceProvider>(
    advice_provider: &mut A,
    process: ProcessState,
) -> Result<HostResponse, ExecutionError> {
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
pub(crate) fn push_trailing_ones<A: AdviceProvider>(
    advice_provider: &mut A,
    process: ProcessState,
) -> Result<HostResponse, ExecutionError> {
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
pub(crate) fn push_ilog2<A: AdviceProvider>(
    advice_provider: &mut A,
    process: ProcessState,
) -> Result<HostResponse, ExecutionError> {
    let n = process.get_stack_item(0).as_int();
    if n == 0 {
        return Err(ExecutionError::LogArgumentZero(process.clk()));
    }
    let ilog2 = Felt::from(n.ilog2());
    advice_provider.push_stack(AdviceSource::Value(ilog2))?;
    Ok(HostResponse::None)
}

// HELPER FUNCTIONS
// ================================================================================================

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
) -> Result<HostResponse, ExecutionError> {
    let stack_top = process.get_stack_item(0);
    let stack_top: u32 = stack_top
        .as_int()
        .try_into()
        .map_err(|_| ExecutionError::NotU32Value(stack_top, ZERO))?;
    let transformed_stack_top = f(stack_top);
    advice_provider.push_stack(AdviceSource::Value(transformed_stack_top))?;
    Ok(HostResponse::None)
}
