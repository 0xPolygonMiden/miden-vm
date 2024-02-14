use assembly::utils::Serializable;
use miden_air::Felt;
use processor::Digest;

use std::vec;
use test_utils::{
    crypto::{rpo_falcon512::KeyPair, MerkleStore},
    rand::rand_vector,
    Test, Word,
};

#[test]
fn test_falcon() {
    let keypair = KeyPair::new().unwrap();

    let message = rand_vector::<Felt>(4).try_into().unwrap();

    let test = generate_test(keypair, message);
    test.expect_stack(&[])
}

fn generate_test(keypair: KeyPair, message: Word) -> Test {
    let source = "
    use.std::crypto::dsa::rpo_falcon512

    begin
        exec.rpo_falcon512::verify
    end
    ";

    let pk: Word = keypair.public_key().into();
    let pk: Digest = pk.into();
    let pk_sk_bytes = keypair.to_bytes();
    let to_adv_map = pk_sk_bytes.iter().map(|a| Felt::new(*a as u64)).collect::<Vec<Felt>>();

    let advice_map: Vec<([u8; 32], Vec<Felt>)> = vec![(pk.as_bytes(), to_adv_map.into())];

    let message = message.into_iter().map(|a| a.as_int() as u64).collect::<Vec<u64>>();

    let mut op_stack = vec![];
    op_stack.extend_from_slice(&message);
    op_stack.extend_from_slice(&pk.as_elements().iter().map(|a| a.as_int()).collect::<Vec<u64>>());
    let adv_stack = vec![];
    let store = MerkleStore::new();
    let test = build_test!(source, &op_stack, &adv_stack, store, advice_map.into_iter());

    test
}
