use miden_air::{Felt, FieldElement, StarkField};
use processor::ZERO;
use test_utils::rand::rand_value;

#[test]
fn test_elgamal_keygen() {
    let value = rand_value();
    let mut stack = vec![value];

    let source = "
        use.std::crypto::elgamal_mock

        begin
            exec.elgamal_mock::gen_privatekey
        end
    ";

    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();
    let mut expected = [ZERO; 16];
    expected[0] = Felt::GENERATOR.exp(value);
    assert_eq!(strace, expected);
}

#[test]
fn test_elgamal_encrypt() {
    //inputs r, M, H
    let private_key = gen_random_private_key();
    let r = gen_random_private_key();

    let gen = get_generator();
    let plaintext_scalar = rand_value();

    let pm = scalar_mul(gen, plaintext_scalar);

    let ca = scalar_mul(gen, r);
    let h = scalar_mul(gen, private_key);
    let rh = scalar_mul(h, r);
    let cb = add(pm, rh);

    let source = "
        use.std::crypto::elgamal_mock

        begin
            exec.elgamal_mock::encrypt_ca
        end
    ";

    let mut stack = [r.as_int()];

    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    assert_eq!(strace[0], ca);

    let mut stack = [h.as_int(), r.as_int(), pm.as_int()];

    let source = "
        use.std::crypto::elgamal_mock

        begin
            exec.elgamal_mock::encrypt_cb
        end
    ";

    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    assert_eq!(strace[0], cb);
}

#[test]
fn test_elgamal_remask() {
    // Also known as rerandomisation
    // inputs r, H, Cb, Ca
    let private_key = gen_random_private_key();
    let r = gen_random_private_key();
    let r_prime = gen_random_private_key();

    let gen = get_generator();
    let plaintext_scalar = rand_value();

    let pm = scalar_mul(gen, plaintext_scalar);

    let ca = scalar_mul(gen, r);
    let h = scalar_mul(gen, private_key);
    let rh = scalar_mul(h, r);
    let cb = add(pm, rh);

    // ca and cb are the original plaintext
    let r_prime_g = scalar_mul(gen, r_prime);
    let r_prime_h = scalar_mul(h, r_prime);
    let c_prime_a = add(ca, r_prime_g);
    let c_prime_b = add(cb, r_prime_h);

    let source = "
        use.std::crypto::elgamal_mock

        begin
            exec.elgamal_mock::remask_ca
        end
    ";

    let mut stack = [r_prime.as_int(), ca.as_int()];
    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    assert_eq!(strace[0], c_prime_a);

    let source = "
        use.std::crypto::elgamal_mock

        begin
            exec.elgamal_mock::remask_cb
        end
    ";

    let mut stack = [h.as_int(), r_prime.as_int(), cb.as_int()];
    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    assert_eq!(strace[0], c_prime_b);
}

#[test]
fn test_elgamal_decrypt() {
    // inputs sk, Cb, Ca
    let private_key = gen_random_private_key();
    let r = gen_random_private_key();

    let gen = get_generator();
    let plaintext_scalar = Felt::from(64565664u32);

    let pm = scalar_mul(gen, plaintext_scalar);

    let ca = scalar_mul(gen, r);
    let h = scalar_mul(gen, private_key);
    let rh = scalar_mul(h, r);
    let cb = add(pm, rh);

    let source = "
        use.std::crypto::elgamal_mock

        begin
            exec.elgamal_mock::decrypt
        end
    ";

    let mut stack = [private_key.as_int(), cb.as_int(), ca.as_int()];
    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    assert_eq!(strace[0], pm);
}

// Helpers
fn scalar_mul(gen: Felt, scalar: Felt) -> Felt {
    gen.exp(scalar.as_int())
}

fn get_generator() -> Felt {
    Felt::from(7_u8)
}

fn gen_random_private_key() -> Felt {
    rand_value()
}

fn add(x: Felt, y: Felt) -> Felt {
    x * y
}
