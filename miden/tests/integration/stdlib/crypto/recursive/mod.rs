use miden::{
    crypto::MerkleStore, Assembler, MemAdviceProvider, ProgramInfo,
    ProofOptions as MidenProofOptions,
};
use miden_air::{Felt, HashFunction, PublicInputs, StarkField};
use vm_core::{crypto::merkle::MerklePathSet, ToElements};
use winter_air::{FieldExtension, ProofOptions as WinterProofOptions};

mod verifier_recursive;

use verifier_recursive::VerifierError;

use crate::build_test;

#[test]
fn stark_verifier_e2f4() {
    // An example MASM program to be verified inside Miden VM
    // Note that output stack-overflow is not yet supported because of the way we handle public inputs
    // in the STARK verifier is not yet general enough. Thus the output stack should be of size exactly 16.
    let example_source = "begin
            repeat.32
                swap dup.1 add
            end
        end";
    let mut stack_inputs = vec![0_u64; 16];
    stack_inputs[15] = 0;
    stack_inputs[14] = 1;

    let (program_root, (tape, advice_sets, advice_map)) =
        generate_recursive_verifier_data(example_source, stack_inputs).unwrap();

    // Verify inside Miden VM
    let source = "
        use.std::crypto::stark::verifier

        begin
            exec.verifier::verify
        end
        ";
    let initial_stack = program_root;
    let mut store = MerkleStore::new();
    for path_set in &advice_sets {
        store.add_merkle_path_set(&path_set).unwrap();
    }
    let test = build_test!(source, &initial_stack, &tape, store, advice_map);

    test.expect_stack(&[]);
}

// Helper function for recursive verification
pub fn generate_recursive_verifier_data(
    source: &str,
    stack_inputs: Vec<u64>,
) -> Result<(Vec<u64>, (Vec<u64>, Vec<MerklePathSet>, Vec<([u8; 32], Vec<Felt>)>)), VerifierError> {
    let program = Assembler::default().compile(&source).unwrap();
    let stack_inputs = crate::helpers::StackInputs::try_from_values(stack_inputs).unwrap();
    let advice_inputs = crate::helpers::AdviceInputs::default();
    let advice_provider = MemAdviceProvider::from(advice_inputs);

    let options = WinterProofOptions::new(27, 8, 16, FieldExtension::Quadratic, 4, 7);
    let proof_options = MidenProofOptions {
        hash_fn: HashFunction::Rpo256,
        options,
    };
    let (stack_outputs, proof) =
        miden::prove(&program, stack_inputs.clone(), advice_provider, proof_options).unwrap();

    let program_info = ProgramInfo::from(program);

    // build public inputs and generate the advice data needed for recursive proof verification
    let pub_inputs = PublicInputs::new(program_info, stack_inputs, stack_outputs);
    let (_, proof) = proof.into_parts();
    let pub_inputs_elem: Vec<u64> = pub_inputs.to_elements().iter().map(|a| a.as_int()).collect();
    let program_root = pub_inputs_elem[..4].to_owned();
    Ok((
        program_root,
        verifier_recursive::generate_advice_inputs(proof, pub_inputs).unwrap(),
    ))
}
