use alloc::{boxed::Box, vec::Vec};
use core::error::Error;

use processor::{EventHandler, ExecutionError, ProcessState, SMT_DEPTH};
use vm_core::{
    crypto::{
        hash::RpoDigest,
        merkle::{EmptySubtreeRoots, Smt},
    },
    fft, AdviceProvider, AdviceProviderError, AdviceSource, Felt, FieldElement, QuadExtension,
    Word, WORD_SIZE, ZERO,
};

use crate::{
    dsa, Ext2InttError, EVENT_EXT2_INTT, EVENT_FALCON_DIV, EVENT_FALCON_SIG_TO_STACK,
    EVENT_SMT_PEEK, EVENT_U64_DIV,
};

// CONSTANTS
// ==============================================================================================

/// Falcon signature prime.
const M: u64 = 12289;

// SIGNATURE TO STACK EVENT HANDLER
// ==============================================================================================

/// An event handler which verifies a Falcon signature and pushes the result onto the stack.
pub struct FalconSigToStackEventHandler<A> {
    signer: Box<dyn FalconSigner<A>>,
}

impl<A> FalconSigToStackEventHandler<A> {
    /// Creates a new instance of the Falcon signature to stack event handler, given a specified
    /// Falcon signer.
    pub fn new(signer: Box<dyn FalconSigner<A>>) -> Self {
        Self { signer }
    }
}

impl<A> Default for FalconSigToStackEventHandler<A> {
    fn default() -> Self {
        Self { signer: Box::new(DefaultFalconSigner) }
    }
}

impl<A> EventHandler<A> for FalconSigToStackEventHandler<A>
where
    A: AdviceProvider,
{
    fn id(&self) -> u32 {
        EVENT_FALCON_SIG_TO_STACK
    }

    fn on_event(
        &mut self,
        process: ProcessState,
        advice_provider: &mut A,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let pub_key = process.get_stack_word(0);
        let msg = process.get_stack_word(1);

        let signature = self.signer.sign_message(pub_key, msg, advice_provider)?;

        for r in signature {
            advice_provider.push_stack(AdviceSource::Value(r))?;
        }

        Ok(())
    }
}

/// A trait for signing messages using the Falcon signature scheme.
///
/// This trait is used by [FalconSigToStackEventHandler] to sign messages using the Falcon signature
/// scheme.
///
/// It is recommended to use [dsa::falcon_sign] to implement this trait once the private key has
/// been fetched from a user-defined location.
pub trait FalconSigner<A>: Send + Sync {
    /// Signs the message using the Falcon signature scheme, and returns the signature as a
    /// `Vec<Felt>`.
    fn sign_message(
        &self,
        pub_key: Word,
        msg: Word,
        advice_provider: &A,
    ) -> Result<Vec<Felt>, Box<dyn Error + Send + Sync + 'static>>
    where
        A: AdviceProvider;
}

/// The default Falcon signer.
///
/// This signer reads the private key from the advice provider's map using `pub_key` as the map key,
/// and signs the message.
#[derive(Debug, Clone)]
pub struct DefaultFalconSigner;

impl<A> FalconSigner<A> for DefaultFalconSigner {
    fn sign_message(
        &self,
        pub_key: Word,
        msg: Word,
        advice_provider: &A,
    ) -> Result<Vec<Felt>, Box<dyn Error + Send + Sync + 'static>>
    where
        A: AdviceProvider,
    {
        let priv_key = advice_provider
            .get_mapped_values(&pub_key.into())
            .ok_or(AdviceProviderError::AdviceMapKeyNotFound(pub_key))?;

        dsa::falcon_sign(priv_key, msg)
    }
}

// DIVISION EVENT HANDLER
// ==============================================================================================

pub struct FalconDivEventHandler;

impl<A> EventHandler<A> for FalconDivEventHandler
where
    A: AdviceProvider,
{
    fn id(&self) -> u32 {
        EVENT_FALCON_DIV
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
    fn on_event(
        &mut self,
        process: ProcessState,
        advice_provider: &mut A,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let dividend_hi = process.get_stack_item(0).as_int();
        let dividend_lo = process.get_stack_item(1).as_int();
        let dividend = (dividend_hi << 32) + dividend_lo;

        let (quotient, remainder) = (dividend / M, dividend % M);

        let (q_hi, q_lo) = u64_to_u32_elements(quotient);
        let (r_hi, r_lo) = u64_to_u32_elements(remainder);
        assert_eq!(r_hi, ZERO);

        advice_provider.push_stack(AdviceSource::Value(r_lo))?;
        advice_provider.push_stack(AdviceSource::Value(q_lo))?;
        advice_provider.push_stack(AdviceSource::Value(q_hi))?;

        Ok(())
    }
}

// U64 DIVISION EVENT HANDLER
// ==============================================================================================

pub struct U64DivEventHandler;

impl<A> EventHandler<A> for U64DivEventHandler
where
    A: AdviceProvider,
{
    fn id(&self) -> u32 {
        EVENT_U64_DIV
    }

    fn on_event(
        &mut self,
        process: ProcessState,
        advice_provider: &mut A,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let divisor_hi = process.get_stack_item(0).as_int();
        let divisor_lo = process.get_stack_item(1).as_int();
        let divisor = (divisor_hi << 32) + divisor_lo;

        if divisor == 0 {
            return Err(ExecutionError::DivideByZero(process.clk()).into());
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

        Ok(())
    }
}

// EXT2 iNTT EVENT HANDLER
// ==============================================================================================

pub struct Ext2iNTTEventHandler;

impl<A> EventHandler<A> for Ext2iNTTEventHandler
where
    A: AdviceProvider,
{
    fn id(&self) -> u32 {
        EVENT_EXT2_INTT
    }

    fn on_event(
        &mut self,
        process: ProcessState,
        advice_provider: &mut A,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
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

            poly.push(QuadExtension::<Felt>::new(word[0], word[1]));
            poly.push(QuadExtension::<Felt>::new(word[2], word[3]));
        }

        let twiddles = fft::get_inv_twiddles::<Felt>(input_size);
        fft::interpolate_poly::<Felt, QuadExtension<Felt>>(&mut poly, &twiddles);

        for element in
            QuadExtension::<Felt>::slice_as_base_elements(&poly[..output_size]).iter().rev()
        {
            advice_provider.push_stack(AdviceSource::Value(*element))?;
        }

        Ok(())
    }
}

// PUSH SMT PEEK EVENT HANDLER
// ==============================================================================================

pub struct PushSmtPeekEventHandler;

impl<A> EventHandler<A> for PushSmtPeekEventHandler
where
    A: AdviceProvider,
{
    fn id(&self) -> u32 {
        EVENT_SMT_PEEK
    }

    fn on_event(
        &mut self,
        process: ProcessState,
        advice_provider: &mut A,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
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

                    return Ok(());
                }
            }

            // if we can't find any key in the leaf that matches `key`, it means no value is
            // associated with `key`
            advice_provider.push_stack(AdviceSource::Word(Smt::EMPTY_VALUE))?;
        }
        Ok(())
    }
}

// HELPERS
// ==============================================================================================

fn u64_to_u32_elements(value: u64) -> (Felt, Felt) {
    let hi = Felt::from((value >> 32) as u32);
    let lo = Felt::from(value as u32);
    (hi, lo)
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
