use super::{ExecutionError, Felt, StarkField, Vec, Word};
use vm_core::crypto::dsa::{
    elements_as_bytes, Polynomial, PublicKeyBytes, SecretKey, SecretKeyBytes, PK_LEN, SK_LEN,
};

/// Gets as input a vector containing an expanded public key and its associated secret key, and a
/// word representing a message and outputs a vector of values to be pushed onto the advice stack.
/// The values are the ones required for a Falcon signature verification inside the VM and they are:
///
/// 1. The nonce represented as 8 field elements.
/// 2. The expanded public key represented as the coefficients of a polynomial of degree < 512.
/// 3. The signature represented as the coefficients of a polynomial of degree < 512.
/// 4. The product of the above two polynomials in the ring of polynomials with coefficients
/// in the Miden field.
///
/// Inputs:
///   Operand stack: [PK, MSG, ...]
///   Advice stack: [...]
///   Advice map: {PK: [pk_raw, sk_raw]}
///
/// Outputs:
///   Operand stack: [PK, MSG, ...]
///   Advice stack: [NONCE1, NONCE0, h, s2, pi]
///   Advice map: {PK: [pk_raw, sk_raw]}
///
/// Where:
/// - PK is the digest of an expanded public.
/// - MSG is the digest of the message to be signed.
/// - [NONCE0, NONCE1] is a double-word representing a 40 bit nonce that is used in the Falcon
/// hash-to-point algorithm.
/// - h is the polynomial representing the expanded public key corresponding to the digest PK.
/// - s2 is the polynomial representing the signature with the secret key associated to PK on
/// the message MSG.
/// - pi is the product of the above two polynomials.
/// - pk_raw are raw bytes of the expanded public key.
/// - sk_raw are raw bytes of the secret key.
///
/// # Errors
/// Will return an error if either:
/// - The advice map does not contain an entry with key PK.
/// - The advice map entry under key PK is not a vector of the expected length.
/// ///
/// The function only generates non-deterministic input that is required for the Falcon verification
/// procedure inside the VM and as such does interact with the VM only through the advice provider.
pub fn falcon_sign(pk_sk: &[Felt], msg: Word) -> Result<Vec<Felt>, ExecutionError> {
    if pk_sk.len() != (PK_LEN + SK_LEN) {
        return Err(ExecutionError::AdviceStackReadFailed(0));
    }

    // To generate a signature, we need the expanded key as well as the secret key
    let pk_exp = pk_sk[..PK_LEN].to_vec();
    let pk_exp: PublicKeyBytes = vec_felt_to_u8(&pk_exp)
        .try_into()
        .expect("Should not fail as we've checked the length of the combined vector");

    let sk = pk_sk[PK_LEN..].to_vec();
    let sk: SecretKeyBytes = vec_felt_to_u8(&sk)
        .try_into()
        .expect("Should not fail as we've checked the length of the combined vector");
    let sk = SecretKey::new(sk);

    // We need to convert the message to a byte array
    let msg_u64 = vec_felt_to_u64(&msg);
    let msg = elements_as_bytes(&msg_u64);

    // We can now generate the signature
    let sig = sk.sign(msg, pk_exp);

    // The signature is composed of a nonce and a polynomial s2

    // We first convert the nonce, a [40; u8], to 8 field elements.
    let nonce = sig.nonce();
    let nonce = convert_nonce(nonce);

    // We convert the signature to a polynomial
    let s2: Polynomial = (&sig).into();

    // We also need in the VM the expanded key corresponding to the public key the was provided
    // via the operand stack
    let h: Polynomial = Polynomial::from_pubkey(&pk_exp);

    // Lastly, for the probabilistic product routine that is part of the verification procedure,
    // we need to compute the product of the expanded key and the signature polynomial in
    // the ring of polynomials with coefficients in the Miden field.
    let pi = Polynomial::mul_modulo_p(&h, &s2);

    // We now push the nonce, the expanded key, the signature polynomial, and the product of the
    // expanded key and the signature polynomial to the advice stack.
    let mut result: Vec<Felt> = nonce.iter().map(|a| Felt::new(*a)).collect::<Vec<Felt>>();
    result.extend(h.inner().iter().map(|a| Felt::new(*a as u64)).collect::<Vec<Felt>>());
    result.extend(s2.inner().iter().map(|&a| Felt::new(a as u64)).collect::<Vec<Felt>>());
    result.extend(pi.iter().map(|&a| Felt::new(a)).collect::<Vec<Felt>>());
    result.reverse();
    Ok(result)
}

// HELPERS
// ================================================================================================

fn vec_felt_to_u8(felts: &[Felt]) -> Vec<u8> {
    felts.iter().map(|f| f.as_int() as u8).collect()
}
fn vec_felt_to_u64(felts: &[Felt]) -> Vec<u64> {
    felts.iter().map(|f| f.as_int()).collect()
}
fn convert_nonce(nonce: &[u8]) -> Vec<u64> {
    let mut result = vec![];
    for i in 0..8 {
        result.push(u64::from_le_bytes([
            nonce[i * 5],
            nonce[i * 5 + 1],
            nonce[i * 5 + 2],
            nonce[i * 5 + 3],
            nonce[i * 5 + 4],
            0,
            0,
            0,
        ]));
    }
    result
}
