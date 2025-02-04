use alloc::{boxed::Box, vec::Vec};
use core::error::Error;

use processor::{EventHandler, ProcessState};
use vm_core::{AdviceProvider, AdviceProviderError, AdviceSource, Felt, Word, ZERO};

use crate::{dsa, EVENT_FALCON_DIV, EVENT_FALCON_SIG_TO_STACK};

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

// HELPERS
// ==============================================================================================

fn u64_to_u32_elements(value: u64) -> (Felt, Felt) {
    let hi = Felt::from((value >> 32) as u32);
    let lo = Felt::from(value as u32);
    (hi, lo)
}
