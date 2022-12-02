use super::{build_test, Felt};
use vm_core::chiplets::hasher::hash_elements;
use vm_core::StarkField;

#[test]
fn rescue_prime_hash_32_bytes() {
    let source = "
    use.std::crypto::hashes::rescue_prime

    begin
        exec.rescue_prime::hash_1to1
    end
    ";

    let input = rand_utils::rand_array::<Felt, 4>();
    let digest = hash_elements(&input);
    let output = digest
        .as_elements()
        .iter()
        .map(|v| v.as_int())
        .rev()
        .collect::<Vec<u64>>();

    let mut stack = [0u64; 4];
    stack.copy_from_slice(&input.map(|v| v.as_int()));

    let test = build_test!(source, &stack);
    test.expect_stack(&output);
}
