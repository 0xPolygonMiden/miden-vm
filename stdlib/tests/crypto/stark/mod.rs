use std::sync::Arc;

use assembly::{Assembler, DefaultSourceManager};
use miden_air::{FieldExtension, HashFunction, PublicInputs};
use processor::{
    DefaultHost, Program, ProgramInfo,
    crypto::{RandomCoin, Rpo256, RpoRandomCoin},
};
use rstest::rstest;
use test_utils::{
    AdviceInputs, MemAdviceProvider, ProvingOptions, StackInputs, VerifierError,
    proptest::proptest, prove,
};
use verifier_recursive::{VerifierData, generate_advice_inputs};
use vm_core::Word;

mod verifier_recursive;

// Note: Changes to Miden VM may cause this test to fail when some of the assumptions documented
// in `stdlib/asm/crypto/stark/verifier.masm` are violated.
#[rstest]
#[case(None)]
#[ignore = "see-https://github.com/0xMiden/miden-vm/issues/1781"]
#[case(Some(KERNEL_ODD_NUM_PROC))]
#[ignore = "see-https://github.com/0xMiden/miden-vm/issues/1781"]
#[case(Some(KERNEL_EVEN_NUM_PROC))]
fn stark_verifier_e2f4(#[case] kernel: Option<&str>) {
    // An example MASM program to be verified inside Miden VM.

    use vm_core::Felt;
    let example_source = "begin
            repeat.320
                swap dup.1 add
            end
        end";
    let mut stack_inputs = vec![0_u64; 16];
    stack_inputs[15] = 0;
    stack_inputs[14] = 1;

    let VerifierData {
        initial_stack,
        advice_stack: tape,
        store,
        mut advice_map,
    } = generate_recursive_verifier_data(example_source, stack_inputs, kernel).unwrap();

    let circuit: Vec<Felt> = CONSTRAINT_EVALUATION_CIRCUIT.iter().map(|a| Felt::new(*a)).collect();
    let circuit_digest = Rpo256::hash_elements(&circuit);

    advice_map.push((circuit_digest, circuit));

    // Verify inside Miden VM
    let source = "
        use.std::crypto::stark::verifier
        begin
            exec.verifier::verify
        end
        ";

    let test = build_test!(source, &initial_stack, &tape, store, advice_map);
    test.expect_stack(&[]);
}

// Helper function for recursive verification
pub fn generate_recursive_verifier_data(
    source: &str,
    stack_inputs: Vec<u64>,
    kernel: Option<&str>,
) -> Result<VerifierData, VerifierError> {
    let program = {
        match kernel {
            Some(kernel) => {
                let context = assembly::testing::TestContext::new();
                let kernel_lib =
                    Assembler::new(context.source_manager()).assemble_kernel(kernel).unwrap();
                let assembler = Assembler::with_kernel(context.source_manager(), kernel_lib);
                let program: Program = assembler.assemble_program(source).unwrap();
                program
            },
            None => {
                let program: Program = Assembler::default().assemble_program(source).unwrap();
                program
            },
        }
    };
    let stack_inputs = StackInputs::try_from_ints(stack_inputs).unwrap();
    let advice_inputs = AdviceInputs::default();
    let advice_provider = MemAdviceProvider::from(advice_inputs);
    let mut host = DefaultHost::new(advice_provider);

    let options =
        ProvingOptions::new(27, 8, 16, FieldExtension::Quadratic, 4, 127, HashFunction::Rpo256);

    let (stack_outputs, proof) = prove(
        &program,
        stack_inputs.clone(),
        &mut host,
        options,
        Arc::new(DefaultSourceManager::default()),
    )
    .unwrap();

    let program_info = ProgramInfo::from(program);

    // build public inputs and generate the advice data needed for recursive proof verification
    let pub_inputs = PublicInputs::new(program_info, stack_inputs, stack_outputs);
    let (_, proof) = proof.into_parts();
    Ok(generate_advice_inputs(proof, pub_inputs).unwrap())
}

proptest! {
    #[test]
    fn generate_query_indices_proptest(num_queries in 7..150_usize, lde_log_size in 9..32_usize) {
        let source = TEST_RANDOM_INDICES_GENERATION;
        let lde_size = 1 << lde_log_size;

        let seed = Word::default();
        let mut coin = RpoRandomCoin::new(seed);
        let indices = coin
            .draw_integers(num_queries, lde_size, 0)
            .expect("should not fail to generate the indices");
        let advice_stack: Vec<u64> = indices.iter().rev().map(|index| *index as u64).collect();

        let input_stack = vec![num_queries as u64, lde_log_size as u64, lde_size as u64];
        let test = build_test!(source, &input_stack, &advice_stack);
        test.prop_expect_stack(&[])?;
    }
}

#[test]
fn generate_query_indices() {
    let source = TEST_RANDOM_INDICES_GENERATION;

    let num_queries = 27;
    let lde_log_size = 18;
    let lde_size = 1 << lde_log_size;

    let input_stack = vec![num_queries as u64, lde_log_size as u64, lde_size as u64];

    let seed = Word::default();
    let mut coin = RpoRandomCoin::new(seed);
    let indices = coin
        .draw_integers(num_queries, lde_size, 0)
        .expect("should not fail to generate the indices");

    let advice_stack: Vec<u64> = indices.iter().rev().map(|index| *index as u64).collect();

    let test = build_test!(source, &input_stack, &advice_stack);

    test.expect_stack(&[]);
}

const KERNEL_ODD_NUM_PROC: &str = r#"
        export.foo
            add
        end
        export.bar
            div
        end
        export.baz
            mul
        end"#;

const KERNEL_EVEN_NUM_PROC: &str = r#"
        export.foo
            add
        end
        export.bar
            div
        end"#;

/// This is an output of the ACE codegen in AirScript and encodes the circuit for executing
/// the constraint evaluation check i.e., DEEP-ALI.
const CONSTRAINT_EVALUATION_CIRCUIT: [u64; 96] = [
    1,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    97710506174,
    230854492443,
    267361714459,
    266287972635,
    1152921594801160277,
    265214230811,
    1152921592653676627,
    264140488987,
    1152921590506192977,
    1152921589432451158,
    214748365083,
    213674623259,
    1152921586211225677,
    1152921585137483854,
    1152921584063742061,
    1152921584063742142,
    1152921585137483855,
    1152921580842516696,
    2305843084375621704,
    1152921585137483862,
    272730423387,
    1152921576547549436,
    271656681563,
    1152921574400065602,
    1152921573326323780,
    1152921572252581972,
    1152921571178840146,
    1152921570105098320,
    64424509509,
    1152921572252581973,
    1152921566883872850,
    1152921565810131024,
    60129542203,
    1152921566883872852,
    1152921562588905552,
    56908316727,
    1152921562588905554,
    54760833076,
    1152921733313855702,
    227633266779,
    1152921556146454577,
    1152921555072712782,
    1152921553998970956,
    48318382130,
    1152921553998970957,
    46170898476,
    45097156681,
    2305843106924200203,
    42949673146,
    100931731552,
    98784247904,
    97710506080,
    97710506079,
    1152921548630261857,
    1152921608759804001,
    2305843045720916007,
    1152921538966585382,
    1152921537892843556,
    1152921536819101733,
    1152921535745359909,
    1152921540040327435,
    1152921540040327265,
    1152921533597876259,
    1152921531450392613,
    1152921532524134487,
    1152921529302908963,
    1152921528229167140,
    2305843034983497756,
    2305843030688530453,
    1152921606612320354,
    2305843028541046883,
    1152921522860458079,
    2305843026393563236,
    1152921520712974431,
    2305843024246079589,
    1152921518565490783,
    2305843022098595942,
    1152921516418007135,
    2305843019951112295,
    1152921514270523487,
    2305843017803628648,
    1152921512123039839,
    2305843015656145001,
    1152921509975556131,
    1152921508901814308,
    1152921507828072485,
    2147483667,
    1152921505680588801,
];

const TEST_RANDOM_INDICES_GENERATION: &str = r#"
        const.QUERY_ADDRESS=1024

        use.std::crypto::stark::random_coin
        use.std::crypto::stark::constants

        begin
            exec.constants::set_lde_domain_size
            exec.constants::set_lde_domain_log_size
            exec.constants::set_number_queries
            push.QUERY_ADDRESS exec.constants::set_fri_queries_address

            exec.random_coin::load_random_coin_state
            hperm
            hperm
            exec.random_coin::store_random_coin_state

            exec.random_coin::generate_list_indices

            exec.constants::get_lde_domain_log_size
            exec.constants::get_number_queries neg
            push.QUERY_ADDRESS
            # => [query_ptr, loop_counter, lde_size_log, ...]

            push.1
            while.true
                dup add.3 mem_load
                movdn.3
                # => [query_ptr, loop_counter, lde_size_log, query_index, ...]
                dup
                add.2 mem_load
                dup.3 assert_eq

                add.4
                # => [query_ptr + 4, loop_counter, lde_size_log, query_index, ...]

                swap add.1 swap
                # => [query_ptr + 4, loop_counter, lde_size_log, query_index, ...]

                dup.1 neq.0
                # => [?, query_ptr + 4, loop_counter + 1, lde_size_log, query_index, ...]
            end
            drop drop drop

            exec.constants::get_number_queries neg
            push.1
            while.true
                swap
                adv_push.1
                assert_eq
                add.1
                dup
                neq.0
            end
            drop  
        end
        "#;
