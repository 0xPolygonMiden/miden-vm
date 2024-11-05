use miden_vm::{Assembler, DefaultHost, MemAdviceProvider, Program, StackInputs};
use stdlib::StdLibrary;
use vm_core::{utils::group_slice_elements, Felt};

use super::Example;

// CONSTANTS
// ================================================================================================

const INITIAL_HASH_VALUE: [u32; 8] = [u32::MAX; 8];

// EXAMPLE BUILDER
// ================================================================================================

pub fn get_example(n: usize) -> Example<DefaultHost<MemAdviceProvider>> {
    // generate the program and expected results
    let program = generate_blake3_program(n);
    let expected_result = compute_hash_chain(n);
    println!(
        "Generated a program to compute {}-th iteration of BLAKE3 1-to-1 hash; expected result: {:?}",
        n, expected_result
    );

    let mut host = DefaultHost::default();
    host.load_mast_forest(StdLibrary::default().mast_forest().clone());

    let stack_inputs =
        StackInputs::try_from_ints(INITIAL_HASH_VALUE.iter().map(|&v| v as u64)).unwrap();

    Example {
        program,
        stack_inputs,
        host,
        expected_result,
        num_outputs: 8,
    }
}

/// Generates a program to compute the `n`-th hash of blake3 1-to-1 hash chain
fn generate_blake3_program(n: usize) -> Program {
    let program = format!(
        "
        use.std::crypto::hashes::blake3
        use.std::sys

        begin
            repeat.{}
                exec.blake3::hash_1to1
            end
            exec.sys::truncate_stack
        end",
        n
    );

    Assembler::default()
        .with_library(StdLibrary::default())
        .unwrap()
        .assemble_program(program)
        .unwrap()
}

/// Computes the `n`-th hash of blake3 1-to-1 hash chain
fn compute_hash_chain(n: usize) -> Vec<Felt> {
    let mut bytes: [u8; 32] = INITIAL_HASH_VALUE
        .iter()
        .flat_map(|v| v.to_le_bytes())
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();

    for _ in 0..n {
        let hasher = blake3::hash(&bytes);
        bytes = *hasher.as_bytes();
    }

    group_slice_elements::<u8, 4>(&bytes)
        .iter()
        .map(|&bytes| Felt::from(u32::from_le_bytes(bytes)))
        .collect::<Vec<Felt>>()
}

// EXAMPLE TESTER
// ================================================================================================

#[test]
fn test_blake3_example() {
    let example = get_example(2);
    super::test_example(example, false);
}

#[test]
fn test_blake3_example_fail() {
    let example = get_example(2);
    super::test_example(example, true);
}
