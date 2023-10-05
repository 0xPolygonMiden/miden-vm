use miden_air::{Felt, StarkField};

use std::vec;
use test_utils::{
    crypto::{
        rpo_falcon512::{KeyPair, Polynomial, Signature},
        Rpo256,
    },
    rand::rand_vector,
    Test, Word,
};

#[test]
fn test_falcon() {
    let keypair = KeyPair::new().unwrap();

    let message = rand_vector::<Felt>(4).try_into().unwrap();

    let signature = keypair.sign(message).unwrap();
    let test = generate_test(keypair, message, signature);
    test.expect_stack(&[])
}

#[test]
fn test_falcon_wrong_pub_key() {
    let keypair = KeyPair::new().unwrap();

    let keypair_wrong = KeyPair::new().unwrap();

    let message = rand_vector::<Felt>(4).try_into().unwrap();
    let signature = keypair.sign(message).unwrap();
    let test = generate_test(keypair_wrong, message, signature);
    assert!(test.execute().is_err());
}

#[test]
fn test_falcon_wrong_message() {
    let keypair = KeyPair::new().unwrap();

    let message = rand_vector::<Felt>(4).try_into().unwrap();
    let message_wrong = rand_vector::<Felt>(4).try_into().unwrap();
    let signature = keypair.sign(message).unwrap();
    let test = generate_test(keypair, message_wrong, signature);
    assert!(test.execute().is_err());
}

fn generate_test(keypair: KeyPair, message: Word, signature: Signature) -> Test {
    let source = "
    use.std::crypto::dsa::rpo_falcon512

    begin
        exec.rpo_falcon512::verify
    end
    ";

    let h = Polynomial::from_pub_key(&keypair.expanded_public_key()).unwrap();
    let s2: Polynomial = (signature).sig_poly();
    let nonce = signature.nonce();

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
    let nonce = nonce.into_iter().map(|a| a.as_int() as u64).collect::<Vec<u64>>();
    let message = message.into_iter().map(|a| a.as_int() as u64).collect::<Vec<u64>>();

    let mut adv_stack = vec![];
    adv_stack.extend_from_slice(&h);
    adv_stack.extend_from_slice(&s2);
    adv_stack.extend_from_slice(&prod);

    let mut op_stack = vec![];
    op_stack.extend_from_slice(&nonce);
    op_stack.extend_from_slice(&message);
    op_stack.extend_from_slice(&h_digest);
    let test = build_test!(source, &op_stack, &adv_stack);

    test
}
