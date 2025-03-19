use alloc::vec::Vec;

// RE-EXPORTS
// ================================================================================================
pub use vm_core::utils::*;

use super::Felt;

// FALCON SIGNATURE
// ================================================================================================

/// Event ID for pushing a Falcon signature to the advice stack.
/// This event is used for testing purposes only.
#[cfg(any(test, feature = "testing"))]
pub const EVENT_FALCON_SIG_TO_STACK: u32 = 3419226139;

/// Gets as input a vector containing a secret key, and a word representing a message and outputs a
/// vector of values to be pushed onto the advice stack.
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
///
/// # Return
///
/// Signs the provided message with the provided secret key and returns the resulting signature
/// encoded in the format required by the rpo_faclcon512::verify procedure, or `None` if the secret
/// key is malformed due to either incorrect length or failed decoding.
#[cfg(feature = "std")]
pub fn falcon_sign(sk: &[Felt], msg: vm_core::Word) -> Option<Vec<Felt>> {
    use vm_core::{
        crypto::{
            dsa::rpo_falcon512::{Polynomial, SecretKey},
            hash::Rpo256,
        },
        utils::Deserializable,
    };

    // Create the corresponding secret key
    let mut sk_bytes = Vec::with_capacity(sk.len());
    for element in sk {
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
pub fn falcon_sign(_pk_sk: &[Felt], _msg: vm_core::Word) -> Option<Vec<Felt>> {
    None
}

// HELPER FUNCTIONS
// ================================================================================================

/// Returns the number of rows in the provided execution trace assumed to be in column-major form
/// and contain at least one column.
pub(crate) fn get_trace_len(trace: &[Vec<Felt>]) -> usize {
    trace[0].len()
}

/// Splits an element into two field elements containing 32-bit integer values
#[inline(always)]
pub(crate) fn split_element(value: Felt) -> (Felt, Felt) {
    let value = value.as_int();
    let lo = (value as u32) as u64;
    let hi = value >> 32;
    (Felt::new(hi), Felt::new(lo))
}

/// Splits an element into two 16 bit integer limbs. It assumes that the field element contains a
/// valid 32-bit integer value.
pub(crate) fn split_element_u32_into_u16(value: Felt) -> (Felt, Felt) {
    let (hi, lo) = split_u32_into_u16(value.as_int());
    (Felt::new(hi as u64), Felt::new(lo as u64))
}

/// Splits a u64 integer assumed to contain a 32-bit value into two u16 integers.
///
/// # Errors
/// Fails in debug mode if the provided value is not a 32-bit value.
pub(crate) fn split_u32_into_u16(value: u64) -> (u16, u16) {
    const U32MAX: u64 = u32::MAX as u64;
    debug_assert!(value <= U32MAX, "not a 32-bit value");

    let lo = value as u16;
    let hi = (value >> 16) as u16;

    (hi, lo)
}
