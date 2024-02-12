use assembly::utils::Serializable;
use miden_air::{Felt, StarkField};
use processor::{Digest, ExecutionError};
use rand::Rng;

use std::vec;
use test_utils::{crypto::{rpo_falcon512::KeyPair, MerkleStore}, FieldElement, QuadFelt, rand::rand_vector, Test, TestError, Word, WORD_SIZE};
use test_utils::crypto::rpo_falcon512::Polynomial;
use miden_crypto::hash::rpo::Rpo256;
use test_utils::rand::rand_value;

// Modulus used for rpo falcon 512.
const M: u64 = 12289;
const Q: u64 = (M - 1) / 2;
const N: usize = 512;
const J: u64 = (N * M as usize * M as usize) as u64;


#[test]
fn test_falcon512_norm_sq() {
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
fn test_falcon512_diff_mod_q() {
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
fn test_falcon512_powers_of_tau() {
    let source = "
    use.std::crypto::dsa::rpo_falcon512

    begin
        exec.rpo_falcon512::powers_of_tau
    end
    ";

    let tau = rand_value::<QuadFelt>();
    let tau_ptr = 0_u32;
    let (tau_0, tau_1) = ext_element_to_ints(tau);
    println!("tau_0 is {}, tau_1 is: {}", tau_0, tau_1);

    let expected_memory = powers_of_tau(tau);
    println!("The expected memory length is: {}", expected_memory.len());

    let stack_init = [tau_ptr.into(), tau_0, tau_1];

    let test = build_test!(source, &stack_init);

    let expected_stack = &[<u32 as Into<u64>>::into(tau_ptr) + N as u64 + 1];

    test.expect_stack_and_memory(expected_stack, tau_ptr, &expected_memory);
}

#[test]
fn test_falcon512_probabilistic_product() {
    let source = "
    use.std::crypto::dsa::rpo_falcon512

    begin
        exec.rpo_falcon512::load_h_s2_and_product
        exec.rpo_falcon512::powers_of_tau
        exec.rpo_falcon512::set_to_zero

        locaddr.512     # tau_ptr
        locaddr.1025    # z_ptr
        locaddr.0       # h ptr
        #=> [h_ptr, zeros_ptr, tau_ptr, ...]

        exec.rpo_falcon512::probablistic_product
    end
    ";

    // Set the pointer to where h, s2 and pi = h * s2 will be stored.
    let h_ptr = 0_u32;

    // Create two polynomials and compute their product.
    let s2: Polynomial = unsafe { Polynomial::new(random_coefficients()) };
    let h: Polynomial = unsafe { Polynomial::new(random_coefficients()) };
    let pi = Polynomial::mul_modulo_p(&h, &s2);

    // Lay the polynomials in the advice stack.
    let mut advice_stack: Vec<u64> = h.to_elements().iter().map(|&e| e.into()).collect();
    let s2_64: Vec<u64> = s2.to_elements().iter().map(|&e| e.into()).collect();
    let pi_64: Vec<u64> = pi.iter().map(|&e| e.into()).collect();

    // advice_stack.extend(s2_64);
    // advice_stack.extend(pi_64);

    // TODO: Computer hash of h and set it as second stack elements.
    let h_rpo_hash: Word = Rpo256::hash(&*advice_stack.clone().to_bytes()).into();

    advice_stack.extend(s2_64);
    advice_stack.extend(pi_64);

    let stack_init: Vec<u64> = vec![h_ptr.into(), h_rpo_hash.into()];

    let test = test_utils::build_test!(source, &stack_init, &advice_stack);

    let expected_stack = &[];

    test.expect_stack(expected_stack);
}

#[test]
fn test_falcon512_probabilistic_product_failure() {
    let source = "
    use.std::crypto::dsa::rpo_falcon512

    begin
        exec.rpo_falcon512::load_h_s2_and_product
        exec.rpo_falcon512::powers_of_tau
        exec.rpo_falcon512::set_to_zero

        locaddr.512     # tau_ptr
        locaddr.1025    # z_ptr
        locaddr.0       # h ptr
        #=> [h_ptr, zeros_ptr, tau_ptr, ...]

        exec.rpo_falcon512::probablistic_product
    end
    ";

    // Set the pointer to where h, s2 and pi = h * s2 will be stored.
    let h_ptr = 0_u32;

    // Create three polynomials where pi != h * s2.
    let s2: Polynomial = unsafe { Polynomial::new(random_coefficients()) };
    let h: Polynomial = unsafe { Polynomial::new(random_coefficients()) };
    let pi = unsafe { Polynomial::new(random_coefficients()) };

    // Lay the polynomials in the advice stack.
    let mut advice_stack: Vec<u64> = h.to_elements().iter().map(|&e| e.into()).collect();
    let s2_64: Vec<u64> = s2.to_elements().iter().map(|&e| e.into()).collect();
    let pi_64: Vec<u64> = pi.iter().map(|&e| e.into()).collect();

    advice_stack.extend(s2_64);
    advice_stack.extend(pi_64);

    // TODO: Computer hash of h and set it as second stack element.

    let stack_init = [h_ptr, ];

    let expected_error = TestError::ExecutionError(ExecutionError::FailedAssertion {clk: 0, err_code: 0, err_msg: Option::from(String::from("")) });

    build_test!(source, &stack_init, &advice_stack).expect_error(expected_error);
}


#[test]
fn test_falcon512_verify() {
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

fn powers_of_tau(tau: QuadFelt) -> Vec<u64> {
    let mut tau_power: QuadFelt;
    let mut elem_0: u64;
    let mut elem_1: u64;
    let mut expected_memory = vec![0; (N + 1) * WORD_SIZE];
    expected_memory[0] = 1;

    for i in 1..N + 1 {
        tau_power = tau.exp(i as u64);
        (elem_0, elem_1) = ext_element_to_ints(tau_power);
        expected_memory[i * WORD_SIZE] = elem_0;
        expected_memory[i * WORD_SIZE+ 1] = elem_1;
        expected_memory[i * WORD_SIZE + 2] = expected_memory[i * WORD_SIZE - WORD_SIZE];
        expected_memory[i * WORD_SIZE + 3] = expected_memory[i * WORD_SIZE - 3];
    }
    expected_memory
}

fn random_coefficients() -> [u16; N] {
    let mut res = [u16::default(); N];
    for i in res.iter_mut() {
        *i = rand::thread_rng().gen_range(0..M) as u16;
    }
    res
}

fn key_to_bytes(key: [u64; 4]) -> [u8; 32] {
    let mut result = [0; 32];

    result[..8].copy_from_slice(&key[0].to_le_bytes());
    result[8..16].copy_from_slice(&key[1].to_le_bytes());
    result[16..24].copy_from_slice(&key[2].to_le_bytes());
    result[24..].copy_from_slice(&key[3].to_le_bytes());

    result
}
