use assembly::utils::Serializable;
use miden_air::{Felt, StarkField};
use processor::Digest;
use rand::Rng;

use std::vec;
use test_utils::{crypto::{rpo_falcon512::KeyPair, MerkleStore}, FieldElement, ONE, QuadFelt, rand::rand_vector, Test, Word, ZERO};
use test_utils::rand::rand_value;

const M: u64 = 12289;
const Q: u64 = (M - 1) / 2;
const J: u64 = 512 * M * M;

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

    let test1 = build_test!(source, &[u, v, w]);
    let expected_answer =  (v + w + J - u).rem_euclid(M);

    test1.expect_stack(&[expected_answer]);
}

#[test]
fn test_falcon_powers_of_tau() {
    let source = "
    use.std::crypto::dsa::rpo_falcon512

    begin
        exec.rpo_falcon512::powers_of_tau
    end
    ";

    let tau = rand_value::<QuadFelt>();
    let tau_ptr = 0_u32;
    let (tau_0, tau_1) = ext_element_to_ints(tau);

    let mut tau_power: QuadFelt;
    let mut elem_0: u64;
    let mut elem_1: u64;
    let mut expected_memory = vec![0; 513 * 4];
    expected_memory[0] = 1;

    for i in (1..513usize) {
        tau_power = tau.exp(i as u64);
        (elem_0, elem_1) = ext_element_to_ints(tau_power);
        expected_memory[i * 4] = elem_0;
        expected_memory[i * 4 + 1] = elem_1;
        expected_memory[i * 4 + 2] = expected_memory[i * 4 - 4];
        expected_memory[i * 4 + 3] = expected_memory[i * 4 - 3];
    }

    let stack_init = [tau_ptr.into(), tau_0, tau_1];

    let test = build_test!(source, &stack_init);

    let expected_stack = &[<u32 as Into<u64>>::into(tau_ptr) + 513];

    test.expect_stack_and_memory(expected_stack, tau_ptr, &expected_memory);
}

#[test]
fn test_falcon_probabilistic_product() {
    let source = "
    use.std::crypto::dsa::rpo_falcon512

    begin
        exec.rpo_falcon512::probabilistic_product
    end
    ";

    let tau = rand_value::<QuadFelt>();
    let tau_ptr = 0_u32;
    let (tau_0, tau_1) = ext_element_to_ints(tau);

    let mut tau_power: QuadFelt;
    let mut elem_0: u64;
    let mut elem_1: u64;
    let mut expected_memory = vec![0; 513 * 4];
    expected_memory[0] = 1;

    for i in (1..513usize) {
        tau_power = tau.exp(i as u64);
        (elem_0, elem_1) = ext_element_to_ints(tau_power);
        expected_memory[i * 4] = elem_0;
        expected_memory[i * 4 + 1] = elem_1;
        expected_memory[i * 4 + 2] = expected_memory[i * 4 - 4];
        expected_memory[i * 4 + 3] = expected_memory[i * 4 - 3];
    }

    let stack_init = [tau_ptr.into(), tau_0, tau_1];

    let test = build_test!(source, &stack_init);

    let expected_stack = &[<u32 as Into<u64>>::into(tau_ptr) + 513];

    test.expect_stack_and_memory(expected_stack, tau_ptr, &expected_memory);
}

#[test]
fn test_falcon_verify() {
    let keypair = KeyPair::new().unwrap();

    let message = rand_vector::<Felt>(4).try_into().unwrap();

    let test = generate_test_verify(keypair, message);
    test.expect_stack(&[])
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

// HELPER FUNCTIONS
// ================================================================================================
/// Helper function to convert a quadratic extension field element into a tuple of elements in the
/// underlying base field and convert them into integers.
fn ext_element_to_ints(ext_elem: QuadFelt) -> (u64, u64) {
    let base_elements = ext_elem.to_base_elements();
    (base_elements[0].as_int(), base_elements[1].as_int())
}
