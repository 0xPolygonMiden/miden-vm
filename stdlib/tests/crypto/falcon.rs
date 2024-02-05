use assembly::utils::Serializable;
use miden_air::{Felt, StarkField};
use processor::{Digest, ExecutionError, ONE, ZERO};
use rand::Rng;

use std::vec;
use processor::math::fft;
use test_utils::{crypto::{rpo_falcon512::KeyPair, MerkleStore}, FieldElement, QuadFelt, rand::rand_vector, Test, TestError, Word, WORD_SIZE};
use test_utils::math::{polynom, QuadExtension};
use test_utils::rand::rand_value;

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

    let expected_memory = powers_of_tau(tau);

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
        exec.rpo_falcon512::probablistic_product
    end
    ";

    let h_ptr = 0_u32;

    // Create an array of the powers of a random quadratic extension field element from 0 to N.
    let tau = rand_value::<QuadFelt>();
    let powers_of_tau = powers_of_tau(tau);

    // Create zeros array.
    let zeros_ptr: Vec<u64> = vec![1024];

    // Create the polynomials h and s2.
    let mut h = random_polynomial_coefficients();
    let s2 = random_polynomial_coefficients();

    // Multiply them to get pi.
    let pi = polynom::mul(&*h, &*s2);

    // Evaluate s2(tau)
    let h_tau = polynom::eval(&*h, tau);

    // Evaluate s2(tau)
    let s2_tau = polynom::eval(&*s2, tau);

    // Evaluate pi(tau)
    let pi_tau = polynom::eval(&*pi, tau);
    assert_eq!(pi_tau, h_tau * s2_tau);

    println!("h_tau is: {:?}", h_tau);
    println!("s2_tau is: {:?}", s2_tau);
    println!("The product of h_tau and s2_tau is: {:?}", h_tau * s2_tau);
    println!("pi_tau is: {:?}", pi_tau);

    let mut h_64: Vec<u64> = h.iter().map(|&e| e.into()).collect();
    let s2_64: Vec<u64> = s2.iter().map(|&e| e.into()).collect();
    let pi_64: Vec<u64> = pi.iter().map(|&e| e.into()).collect();

    h_64.extend(s2_64);
    h_64.extend(pi_64);
    h_64.extend(zeros_ptr);
    h_64.extend(powers_of_tau);

    // Stack should be empty and memory shouldn't change.

    let stack_init = [<u32 as Into<u64>>::into(h_ptr) + (N as u64 * 7), <u32 as Into<u64>>::into(h_ptr) + (N as u64 * 6), h_ptr.into()];

    let test = build_test!(source, &stack_init);

    let expected_stack = &[];

    test.expect_stack(expected_stack);
}

#[test]
fn test_falcon512_probabilistic_product_failure() {
    let source = "
    use.std::crypto::dsa::rpo_falcon512

    begin
        exec.rpo_falcon512::probablistic_product
    end
    ";

    // Create an array of the powers of a random quadratic extension field element from 0 to N.
    let tau = rand_value::<QuadFelt>();
    let powers_of_tau = powers_of_tau(tau);

    // Create zeros array.
    let zeros_ptr: Vec<u64> = vec![1024];

    // Create the polynomials h and s2.
    let mut h = random_polynomial_coefficients();
    let h_ptr = 0_u32;

    let s2 = random_polynomial_coefficients();

    // Multiply them to get pi.
    let pi = random_polynomial_coefficients();

    // Evaluate h(tau)
    let h_tau = polynom::eval(&*h, tau);

    // Evaluate s2(tau)
    let s2_tau = polynom::eval(&*s2, tau);

    // Evaluate pi(tau)
    let pi_tau = polynom::eval(&*pi, tau);
    assert_ne!(pi_tau, h_tau * s2_tau);

    println!("h_tau is: {:?}", h_tau);
    println!("s2_tau is: {:?}", s2_tau);
    println!("The product of h_tau and s2_tau is: {:?}", h_tau * s2_tau);
    println!("pi_tau is: {:?}", pi_tau);


    let mut h_64: Vec<u64> = h.iter().map(|&e| e.into()).collect();
    let s2_64: Vec<u64> = s2.iter().map(|&e| e.into()).collect();
    let pi_64: Vec<u64> = pi.iter().map(|&e| e.into()).collect();

    h_64.extend(s2_64);
    h_64.extend(pi_64);
    h_64.extend(zeros_ptr);
    h_64.extend(powers_of_tau);

    // Equality assertion should throw exception.

    let stack_init = [<u32 as Into<u64>>::into(h_ptr) + (N as u64 * 7), <u32 as Into<u64>>::into(h_ptr) + (N as u64 * 6), h_ptr.into()];
    let expected_error = TestError::ExecutionError(ExecutionError::FailedAssertion {clk: 0, err_code: 0, err_msg: Option::from(String::from("")) });

    build_test!(source, &stack_init).expect_error(expected_error);
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

fn random_polynomial_coefficients() -> Vec<Felt> {
    let mut array:Vec<Felt> = vec![ZERO; N];
    for felt in array.iter_mut() {
        *felt = rand_value::<Felt>();
    }
    array
}