use assembly::utils::Serializable;
use miden_air::{Felt, StarkField};
use processor::Digest;
use rand::Rng;

use std::vec;
use test_utils::{
    crypto::{rpo_falcon512::KeyPair, MerkleStore},
    rand::rand_vector,
    Test, Word,
};

const M: u64 = 12289;
const Q: u64 = (M - 1) / 2;
const J: u64 = 512 * M * M;

#[test]
fn test_falcon_verify() {
    let keypair = KeyPair::new().unwrap();

    let message = rand_vector::<Felt>(4).try_into().unwrap();

    let test = generate_test_verify(keypair, message);
    test.expect_stack(&[])
}

#[test]
fn test_falcon_norm_sq() {
    let source = "
    use.std::crypto::dsa::rpo_falcon512

    begin
        exec.rpo_falcon512::norm_sq
    end
    ";

    let num1 = rand::thread_rng().gen_range(Q..M);

    let test1 = build_test!(source, &[num1]);

    test1.expect_stack(&[(M - num1) * (M - num1)]);

    let num2 = rand::thread_rng().gen_range(0..Q);


    let test2 = build_test!(source, &[num2]);

    test2.expect_stack(&[num2 * num2])
}

#[test]
fn test_falcon_diff_mod_q() {
    let source = "
    use.std::crypto::dsa::rpo_falcon512

    begin
        exec.rpo_falcon512::diff_mod_q
    end
    ";

    let u = rand::thread_rng().gen_range(0..J);
    let v = rand::thread_rng().gen_range(Q..M);
    let w = rand::thread_rng().gen_range(0..J);
    let neg_w: i64 = (-1 * w) as i64;


    let test1 = build_test!(source, &[u, v, w]);
    let expected_answer1 =  v - (u + ((neg_w).rem_euclid(M)).rem_euclid(M)).rem_euclid(M);

    let expected_answer = (v + w + J - u).rem_euclid(M);
    test1.expect_stack(&[expected_answer1]);
}

fn generate_test_verify(keypair: KeyPair, message: Word) -> Test {
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

fn generate_test_norm_sq(keypair: KeyPair, message: Word) -> Test {
    let source = "
    use.std::crypto::dsa::rpo_falcon512

    begin
        exec.rpo_falcon512::norm_sq
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
