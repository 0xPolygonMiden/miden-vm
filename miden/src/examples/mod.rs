// use examples::{Example, ExampleOptions, ExampleType};
pub use assembly;
pub use prover::ProofOptions;
pub use vm_core::{program::Script, Felt, FieldElement, ProgramInputs, StarkField};

pub mod fibonacci;
// TODO enable other examples
// pub mod merkle;
// pub mod range;
// pub mod collatz;
// pub mod comparison;
// pub mod conditional;

// EXAMPLE
// ================================================================================================

pub struct Example {
    pub program: Script,
    pub inputs: ProgramInputs,
    pub pub_inputs: Vec<u64>,
    pub num_outputs: usize,
    pub expected_result: Vec<u64>,
}

// TESTS
// ================================================================================================

#[cfg(test)]
pub fn test_example(example: Example, fail: bool) {
    let Example {
        program,
        inputs,
        pub_inputs,
        num_outputs,
        expected_result,
    } = example;

    let (mut outputs, proof) =
        prover::prove(&program, &inputs, num_outputs, &ProofOptions::default()).unwrap();

    assert_eq!(
        expected_result, outputs,
        "Program result was computed incorrectly"
    );

    if fail {
        outputs[0] += 1;
        assert!(verifier::verify(program.hash(), &pub_inputs, &outputs, proof).is_err())
    } else {
        assert!(verifier::verify(program.hash(), &pub_inputs, &outputs, proof).is_ok());
    }
}
