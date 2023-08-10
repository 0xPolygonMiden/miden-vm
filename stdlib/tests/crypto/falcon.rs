use core::slice;
use miden_air::{Felt, StarkField};

use std::vec;
use test_utils::{
    crypto::{KeyPair, Polynomial, Rpo256},
    rand::rand_vector,
};

#[test]
fn test_falcon_final() {
    let source = "
    use.std::crypto::dsa::falcon

    begin
        exec.falcon::verify
    end
    ";

    let keypair = KeyPair::keygen();
    let sk = keypair.secret_key();
    let pk = keypair.public_key();

    let msg = rand_vector::<u64>(4);
    let message = elements_as_bytes(&msg);

    let signature = sk.sign(message, keypair.expanded_public_key());
    assert!(pk.verify(message, &signature));

    let h = signature.pk();
    let s2: Polynomial = (&signature).into();
    let nonce = slice_u8_to_slice_u64(signature.nonce());

    let prod = Polynomial::mul_modulo_p(&h, &s2)
        .into_iter()
        .map(|a| a as u64)
        .collect::<Vec<u64>>();
    let s2 = s2.inner().into_iter().map(|a| a as u64).collect::<Vec<u64>>();

    let h_felt = h.inner().into_iter().map(|a| Felt::new(a as u64)).collect::<Vec<Felt>>();
    let h_digest = Rpo256::hash_elements(&h_felt)
        .as_elements()
        .iter()
        .map(|a| a.as_int())
        .collect::<Vec<u64>>();
    let h = h.inner().into_iter().map(|a| a as u64).collect::<Vec<u64>>();

    let mut adv_stack = vec![];
    adv_stack.extend_from_slice(&h);
    adv_stack.extend_from_slice(&s2);
    adv_stack.extend_from_slice(&prod);

    let mut op_stack = vec![];
    op_stack.extend_from_slice(&nonce);
    op_stack.extend_from_slice(&msg);
    op_stack.extend_from_slice(&h_digest);

    let test = build_test!(source, &op_stack, &adv_stack);

    test.expect_stack(&[]);
}

// HELPER FUNCTIONS
// ================================================================================================

fn elements_as_bytes(elements: &[u64]) -> &[u8] {
    let p = elements.as_ptr();
    let len = elements.len() * 8;
    unsafe { slice::from_raw_parts(p as *const u8, len) }
}

fn slice_u8_to_slice_u64(slice: &[u8]) -> Vec<u64> {
    assert_eq!(slice.len() % 8, 0);
    let len = slice.len() / 8;
    let mut result = vec![0; len];
    for i in 0..len {
        let start = i * 8;
        let end = start + 8;
        let bytes = &slice[start..end];
        result[i] = u64::from_le_bytes(bytes.try_into().unwrap());
    }
    result
}
