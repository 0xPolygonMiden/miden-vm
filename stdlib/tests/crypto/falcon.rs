use assembly::{utils::Serializable, Assembler};
use miden_air::{Felt, ProvingOptions};
use miden_stdlib::StdLibrary;
use processor::{AdviceInputs, DefaultHost, Digest, MemAdviceProvider, StackInputs};
use test_utils::{
    crypto::{rpo_falcon512::KeyPair, MerkleStore},
    rand::rand_vector,
    ProgramInfo, Word,
};

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
        .assemble(source)
        .expect("failed to compile test source");

    let stack_inputs = StackInputs::try_from_ints(op_stack).expect("failed to create stack inputs");
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

#[allow(clippy::type_complexity)]
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

    let advice_map: Vec<(Digest, Vec<Felt>)> = vec![(pk, to_adv_map)];

    let mut op_stack = vec![];
    let message = message.into_iter().map(|a| a.as_int()).collect::<Vec<u64>>();
    op_stack.extend_from_slice(&message);
    op_stack.extend_from_slice(&pk.as_elements().iter().map(|a| a.as_int()).collect::<Vec<u64>>());

    let adv_stack = vec![];
    let store = MerkleStore::new();

    (source, op_stack, adv_stack, store, advice_map)
}
