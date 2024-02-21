use rand::Rng;

use assembly::{utils::Serializable, Assembler};
use miden_air::{Felt, ProvingOptions};
use miden_stdlib::StdLibrary;
use processor::{
    AdviceInputs, DefaultHost, Digest, ExecutionError, MemAdviceProvider, StackInputs,
};

use std::vec;
use test_utils::crypto::rpo_falcon512::Polynomial;
use test_utils::crypto::Rpo256;
use test_utils::rand::rand_value;
use test_utils::{
    crypto::{rpo_falcon512::KeyPair, MerkleStore},
    rand::rand_vector,
    FieldElement, ProgramInfo, QuadFelt, TestError, Word, WORD_SIZE,
};

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
    let expected_answer = (v + w + J - u).rem_euclid(M);

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
        #=> [PK, ...]
        mem_load.0
        #=> [h_ptr, PK, ...]

        exec.rpo_falcon512::load_h_s2_and_product
        #=> [tau1, tau0, tau_ptr, ...]

        exec.rpo_falcon512::powers_of_tau
        #=> [zeros_ptr, ...]

        exec.rpo_falcon512::set_to_zero
        #=> [c_ptr, ...]

        drop
        #=> [...]

        push.512    # tau_ptr
        push.1025   # z_ptr
        push.0      # h ptr

        #=> [h_ptr, zeros_ptr, tau_ptr, ...]

        exec.rpo_falcon512::probabilistic_product
    end
    ";

    // Create two random polynomials and multiply them.
    let h: Polynomial = unsafe { Polynomial::new(random_coefficients()) };
    let s2: Polynomial = unsafe { Polynomial::new(random_coefficients()) };
    let pi = Polynomial::mul_modulo_p(&h, &s2);

    // Lay the polynomials in the advice stack, h then s2 then pi = h * s2.
    let mut h_array = h.to_elements();
    h_array.extend(s2.to_elements());
    h_array.extend(pi.iter().map(|a| Felt::new(*a)));
    let advice_stack: Vec<u64> = h_array.iter().map(|&e| e.into()).collect();

    // Compute hash of h and place it on the stack.
    let binding = Rpo256::hash_elements(&*h.clone().to_elements());
    let h_hash = binding.as_elements();
    let h_hash_copy: Vec<u64> = h_hash.into_iter().map(|felt| (*felt).into()).collect();

    let stack_init = vec![h_hash_copy[0], h_hash_copy[1], h_hash_copy[2], h_hash_copy[3]];

    let test = build_test!(source, &stack_init, &advice_stack);

    let expected_stack = &[];

    test.expect_stack(expected_stack);
}

#[test]
fn test_falcon512_probabilistic_product_failure() {
    let source = "
    use.std::crypto::dsa::rpo_falcon512

    begin
        #=> [PK, ...]
        mem_load.0
        #=> [h_ptr, PK, ...]

        exec.rpo_falcon512::load_h_s2_and_product
        #=> [tau1, tau0, tau_ptr, ...]

        exec.rpo_falcon512::powers_of_tau
        #=> [zeros_ptr, ...]

        exec.rpo_falcon512::set_to_zero
        #=> [c_ptr, ...]

        drop
        #=> [...]

        push.512    # tau_ptr
        push.1025   # z_ptr
        push.0      # h ptr

        #=> [h_ptr, zeros_ptr, tau_ptr, ...]

        exec.rpo_falcon512::probabilistic_product
    end
    ";

    // Create a polynomial pi that is not equal to h * s2.
    let h: Polynomial = unsafe { Polynomial::new(random_coefficients()) };
    let s2: Polynomial = unsafe { Polynomial::new(random_coefficients()) };
    let h_wrong: Polynomial = unsafe { Polynomial::new(random_coefficients()) };

    let pi = Polynomial::mul_modulo_p(&h_wrong, &s2);

    // Lay the polynomials in the advice stack, h then s2 then pi = h * s2.
    let mut h_array = h.to_elements();
    h_array.extend(s2.to_elements());
    h_array.extend(pi.iter().map(|a| Felt::new(*a)));
    let advice_stack: Vec<u64> = h_array.iter().map(|&e| e.into()).collect();

    // Compute hash of h and place it on the stack.
    let binding = Rpo256::hash_elements(&*h.clone().to_elements());
    let h_hash = binding.as_elements();
    let h_hash_copy: Vec<u64> = h_hash.into_iter().map(|felt| (*felt).into()).collect();

    let stack_init = vec![h_hash_copy[0], h_hash_copy[1], h_hash_copy[2], h_hash_copy[3]];

    let test = build_test!(source, &stack_init, &advice_stack);

    test.expect_error(TestError::ExecutionError(ExecutionError::FailedAssertion {
        clk: 17472,
        err_code: 0,
        err_msg: None,
    }));
}

#[test]
fn falcon_execution() {
    let keypair = KeyPair::new().unwrap();
    let message = rand_vector::<Felt>(4).try_into().unwrap();
    let (source, op_stack, adv_stack, store, advice_map) = generate_test(keypair, message);

    let test = build_test!(source, &op_stack, &adv_stack, store, advice_map.into_iter());
    test.expect_stack(&[])
}

#[test]
#[ignore]
fn falcon_prove_verify() {
    let keypair = KeyPair::new().unwrap();
    let message = rand_vector::<Felt>(4).try_into().unwrap();
    let (source, op_stack, _, _, advice_map) = generate_test(keypair, message);

    let program = Assembler::default()
        .with_library(&StdLibrary::default())
        .expect("failed to load stdlib")
        .compile(&source)
        .expect("failed to compile test source");

    let stack_inputs =
        StackInputs::try_from_values(op_stack).expect("failed to create stack inputs");
    let advice_inputs = AdviceInputs::default().with_map(advice_map);
    let advice_provider = MemAdviceProvider::from(advice_inputs);
    let host = DefaultHost::new(advice_provider);

    let options = ProvingOptions::with_96_bit_security(false);
    let (stack_outputs, proof) = test_utils::prove(&program, stack_inputs.clone(), host, options)
        .expect("failed to generate proof");

    let program_info = ProgramInfo::from(program);
    let result = test_utils::verify(program_info, stack_inputs, stack_outputs, proof);

    assert!(result.is_ok(), "error: {result:?}");
}

fn generate_test(
    keypair: KeyPair,
    message: Word,
) -> (&'static str, Vec<u64>, Vec<u64>, MerkleStore, Vec<(Digest, Vec<Felt>)>) {
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

    let advice_map: Vec<(Digest, Vec<Felt>)> = vec![(pk, to_adv_map.into())];

    let mut op_stack = vec![];
    let message = message.into_iter().map(|a| a.as_int()).collect::<Vec<u64>>();
    op_stack.extend_from_slice(&message);
    op_stack.extend_from_slice(&pk.as_elements().iter().map(|a| a.as_int()).collect::<Vec<u64>>());

    let adv_stack = vec![];
    let store = MerkleStore::new();

    (source, op_stack, adv_stack, store, advice_map)
}

// HELPER FUNCTIONS
// ================================================================================================
/// Helper function to convert a quadratic extension field element into a tuple of elements in the
/// underlying base field and convert them into integers.
fn ext_element_to_ints(ext_elem: QuadFelt) -> (u64, u64) {
    let base_elements = ext_elem.to_base_elements();
    (base_elements[0].as_int(), base_elements[1].as_int())
}

/*
For an element `tau := (tau0, tau1)` in the quadratic extension field, computes all its powers
`tau^i` for `i = 0,..., 512` and store them in a vector of length 2048 (word size * N).  The first two
quadratic field elements of the word i are the elements of tau^i, and the second quadratic field
elements of the same word i, are the elements tau^(i - 1).  Used to test powers of tau procedure.
Ex:
[1, 0, 0, 0, tau_0, tau_1, 1, 0, (tau^2)_0, (tau^2)_1, tau_0, tau_1, (tau^3)_0, (tau^3)_1, (tau^2)_0,
(tau^2)_1, ...]
 */
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
        expected_memory[i * WORD_SIZE + 1] = elem_1;
        expected_memory[i * WORD_SIZE + 2] = expected_memory[i * WORD_SIZE - WORD_SIZE];
        expected_memory[i * WORD_SIZE + 3] = expected_memory[i * WORD_SIZE - 3];
    }
    expected_memory
}

// Create random coefficients in the range
fn random_coefficients() -> [u16; N] {
    let mut res = [u16::default(); N];
    for i in res.iter_mut() {
        *i = rand::thread_rng().gen_range(0..M) as u16;
    }
    res
}
