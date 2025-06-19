use alloc::vec::Vec;

use processor::{
    AdviceProvider, AdviceSource, ProcessState,
    handlers::{EventError, EventHandler},
    utils::to_hex,
};
use vm_core::{
    Felt, FieldElement, Word, crypto::dsa::rpo_falcon512::SecretKey, utils::Deserializable,
};

/// Event ID for pushing a Falcon signature to the advice stack.
/// This event is used for testing purposes only.
/// TODO: Do we want to make make this the default?
pub const EVENT_FALCON_SIG_TO_STACK: u32 = 3419226139;

#[derive(Default)]
pub struct FalconEventHandler;

impl<A: AdviceProvider> EventHandler<A> for FalconEventHandler {
    fn id(&self) -> u32 {
        EVENT_FALCON_SIG_TO_STACK
    }

    /// Pushes values onto the advice stack which are required for verification of a DSA in Miden
    /// VM.
    ///
    /// Inputs:
    ///   Operand stack: [PK, MSG, ...]
    ///   Advice stack: \[ SIGNATURE \]
    ///
    /// Outputs:
    ///   Operand stack: [PK, MSG, ...]
    ///   Advice stack: [...]
    ///
    /// Where:
    /// - PK is the digest of an expanded public.
    /// - MSG is the digest of the message to be signed.
    /// - SIGNATURE is the signature being verified.
    ///
    /// The advice provider is expected to contain the private key associated to the public key PK.
    fn on_event(
        &mut self,
        advice_provider: &mut A,
        process: ProcessState,
    ) -> Result<(), EventError> {
        let pub_key = process.get_stack_word(0);
        let msg = process.get_stack_word(1);

        let sk_felts = advice_provider
            .get_mapped_values(&pub_key.into())
            .ok_or_else(|| FalconError::NoSecretKey { key: pub_key }.into())?;

        let sk = secret_key_from_felts(sk_felts).expect("TODO");

        let result = falcon_sign(&sk, msg).ok_or_else(|| {
            FalconError::MalformedSignatureKey { key_type: "RPO Falcon512" }.into()
        })?;

        for r in result {
            advice_provider.push_stack(AdviceSource::Value(r)).expect("stack push failed");
        }
        Ok(())
    }
}

// FALCON SIGNATURE
// ================================================================================================

pub fn secret_key_from_felts(sk: &[Felt]) -> Option<SecretKey> {
    // Create the corresponding secret key
    let mut sk_bytes = Vec::with_capacity(sk.len());
    for element in sk {
        let value = element.as_int();
        if value > u8::MAX as u64 {
            return None;
        }
        sk_bytes.push(value as u8);
    }

    SecretKey::read_from_bytes(&sk_bytes).ok()
}

/// Signs the provided message with the provided secret key and returns the resulting signature
/// encoded in the format required by the rpo_faclcon512::verify procedure, or `None` if the secret
/// key is malformed due to either incorrect length or failed decoding.
///
/// The values are the ones required for a Falcon signature verification inside the VM and they are:
///
/// 1. The challenge point, a tuple of elements representing an element in the quadratic extension
///    field, at which we evaluate the polynomials in the subsequent three points to check the
///    product relationship.
/// 2. The expanded public key represented as the coefficients of a polynomial of degree < 512.
/// 3. The signature represented as the coefficients of a polynomial of degree < 512.
/// 4. The product of the above two polynomials in the ring of polynomials with coefficients in the
///    Miden field.
/// 5. The nonce represented as 8 field elements.
#[cfg(feature = "std")]
pub fn falcon_sign(sk: &SecretKey, msg: Word) -> Option<Vec<Felt>> {
    use alloc::{vec, vec::Vec};

    use vm_core::crypto::{dsa::rpo_falcon512::Polynomial, hash::Rpo256};

    // We can now generate the signature
    let sig = sk.sign(msg);

    // The signature is composed of a nonce and a polynomial s2

    // The nonce is represented as 8 field elements.
    let nonce = sig.nonce();

    // We convert the signature to a polynomial
    let s2 = sig.sig_poly();

    // We also need in the VM the expanded key corresponding to the public key the was provided
    // via the operand stack
    let h = sk.compute_pub_key_poly().0;

    // Lastly, for the probabilistic product routine that is part of the verification procedure,
    // we need to compute the product of the expanded key and the signature polynomial in
    // the ring of polynomials with coefficients in the Miden field.
    let pi = Polynomial::mul_modulo_p(&h, s2);

    // We now push the expanded key, the signature polynomial, and the product of the
    // expanded key and the signature polynomial to the advice stack. We also push
    // the challenge point at which the previous polynomials will be evaluated.
    // Finally, we push the nonce needed for the hash-to-point algorithm.

    let mut polynomials: Vec<Felt> =
        h.coefficients.iter().map(|a| Felt::from(a.value() as u32)).collect();
    polynomials.extend(s2.coefficients.iter().map(|a| Felt::from(a.value() as u32)));
    polynomials.extend(pi.iter().map(|a| Felt::new(*a)));

    let digest_polynomials = Rpo256::hash_elements(&polynomials);
    let challenge = (digest_polynomials[0], digest_polynomials[1]);

    let mut result: Vec<Felt> = vec![challenge.0, challenge.1];
    result.extend_from_slice(&polynomials);
    result.extend_from_slice(&nonce.to_elements());

    result.reverse();
    Some(result)
}

#[cfg(not(feature = "std"))]
pub fn falcon_sign(_sk: &SecretKey, _msg: Word) -> Option<alloc::vec::Vec<Felt>> {
    None
}

// EVENT ERROR
// ================================================================================================

#[derive(Debug, thiserror::Error)]
pub enum FalconError {
    #[error("public key {} not present in the event handler", to_hex(Felt::elements_as_bytes(.key)))]
    NoSecretKey { key: Word },
    #[error("malformed signature key: {key_type}")]
    MalformedSignatureKey { key_type: &'static str },
}

impl FalconError {
    fn into(self) -> EventError {
        EventError::from(self)
    }
}
