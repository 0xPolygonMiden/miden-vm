use alloc::vec::Vec;

use processor::{AdviceProvider, AdviceSource, ProcessState, handlers::EventError};
use vm_core::{Felt, Word};

/// Host handler function which pushes values onto the advice stack which are required for
/// verification of a DSA in Miden VM.
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
///
/// # TODO:
/// Eventually, we might want to make this a handler stateful, containing the signing keys.
/// These could be loaded into the advice provider by the host when loaded into the Host.
pub fn falcon_sig_to_stack_handler(
    advice_provider: &mut dyn AdviceProvider,
    process: ProcessState,
) -> Result<(), EventError> {
    let pub_key = process.get_stack_word(0);
    let msg = process.get_stack_word(1);

    let sk_felts = advice_provider
        .get_mapped_values(&pub_key)
        .ok_or_else(|| FalconError::NoSecretKey { key: pub_key }.into())?;

    let result = falcon_sign(sk_felts, msg)
        .ok_or_else(|| FalconError::MalformedSignatureKey { key_type: "RPO Falcon512" }.into())?;

    for r in result {
        advice_provider.push_stack(AdviceSource::Value(r)).expect("stack push failed");
    }
    Ok(())
}

// FALCON SIGNATURE
// ================================================================================================

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
pub fn falcon_sign(sk_felt: &[Felt], msg: Word) -> Option<Vec<Felt>> {
    use alloc::{vec, vec::Vec};

    use vm_core::{
        crypto::{
            dsa::rpo_falcon512::{Polynomial, SecretKey},
            hash::Rpo256,
        },
        utils::Deserializable,
    };

    // Create the corresponding secret key
    let mut sk_bytes = Vec::with_capacity(sk_felt.len());
    for element in sk_felt {
        let value = element.as_int();
        if value > u8::MAX as u64 {
            return None;
        }
        sk_bytes.push(value as u8);
    }

    let sk = SecretKey::read_from_bytes(&sk_bytes).ok()?;

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
pub fn falcon_sign(_sk: &[Felt], _msg: Word) -> Option<Vec<Felt>> {
    None
}

// EVENT ERROR
// ================================================================================================

#[derive(Debug, thiserror::Error)]
pub enum FalconError {
    #[error("public key {} not present in the event handler", .key.to_hex())]
    NoSecretKey { key: Word },
    #[error("malformed signature key: {key_type}")]
    MalformedSignatureKey { key_type: &'static str },
}

impl FalconError {
    fn into(self) -> EventError {
        EventError::from(self)
    }
}
