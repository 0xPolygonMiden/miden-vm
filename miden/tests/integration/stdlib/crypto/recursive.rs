use std::collections::BTreeMap;

use miden::{
    AdviceProvider, Digest, MerkleSet, Program, ProgramInfo, ProofOptions as MidenProofOptions,
    Rpo256, StackInputs,
};
use miden_air::{Felt, HashFunction, PublicInputs, StarkField};

use winter_air::{FieldExtension, ProofOptions as WinterProofOptions};

mod fibonacci;
mod verifier_recursive;

use verifier_recursive::VerifierError;
use winterfell::Serializable;

use crate::build_test;
// EXAMPLE
// ================================================================================================

pub struct Example<A>
where
    A: AdviceProvider,
{
    pub program: Program,
    pub stack_inputs: StackInputs,
    pub advice_provider: A,
    pub num_outputs: usize,
    pub expected_result: Vec<u64>,
}

pub fn verify_proof(
    sequence_length: usize,
) -> Result<(Digest, (Vec<u64>, Vec<MerkleSet>)), VerifierError> {
    let options = WinterProofOptions::new(27, 8, 16, FieldExtension::Quadratic, 4, 64);
    let proof_options = MidenProofOptions {
        hash_fn: HashFunction::Rpo256,
        options,
    };
    let Example {
        program,
        stack_inputs,
        advice_provider,
        num_outputs,
        expected_result,
        ..
    } = fibonacci::get_example(sequence_length);

    let (stack_outputs, proof) =
        miden::prove(&program, stack_inputs.clone(), advice_provider, proof_options).unwrap();
    assert_eq!(
        expected_result,
        stack_outputs.stack_truncated(num_outputs),
        "Program result was computed incorrectly"
    );
    let program_info = ProgramInfo::from(program);
    let _security_level = proof.security_level();

    // build public inputs and try to verify the proof
    let pub_inputs = PublicInputs::new(program_info, stack_inputs, stack_outputs);
    let (_, proof) = proof.into_parts();

    let proof_context = proof.context.clone();
    let mut public_coin_seed = Vec::new();
    pub_inputs.write_into(&mut public_coin_seed);
    proof_context.write_into(&mut public_coin_seed);
    let initial_seed = create_initial_seed(public_coin_seed);

    Ok((initial_seed, verifier_recursive::verify(proof, pub_inputs).unwrap()))
}

#[test]
fn stark_verifier_e2f4() {
    let source = "
        use.std::crypto::stark

        begin
            exec.stark::verify
        end
        ";
    let sequence_length = 128;
    let _seed = [
        Felt::from_mont(16821498324654216787),
        Felt::from_mont(11698866466158407452),
        Felt::from_mont(16304187408247845465),
        Felt::from_mont(17252369738950309823),
    ];

    //let initial_stack: Vec<u64> = _seed.into_iter().map(|e| (e).as_int()).collect();
    //let advice_map: BTreeMap<[u8; 32], Vec<Felt>> = BTreeMap::new();
    //let advice_sets = vec![MerkleSet::MerklePathSet(MerklePathSet::new(2))];
    //let tape: Vec<u64> =
    //serde_json::from_reader(BufReader::new(File::open("advice_tape_stark.json").unwrap()))
    //.unwrap();

    let (seed, (tape, advice_sets)) = verify_proof(sequence_length).unwrap();
    let advice_map: BTreeMap<[u8; 32], Vec<Felt>> = BTreeMap::new();
    let initial_stack: Vec<u64> = seed.as_elements().into_iter().map(|e| (*e).as_int()).collect();

    let test = build_test!(source, &initial_stack, &tape, advice_sets, advice_map.clone());

    test.expect_stack(&[]);
}

// Helper
pub fn create_initial_seed(public_coin_seed: Vec<u8>) -> Digest {
    Rpo256::hash(&public_coin_seed)
}
