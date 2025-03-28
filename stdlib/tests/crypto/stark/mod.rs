use std::sync::Arc;
mod verifier_recursive;
use assembly::{Assembler, DefaultSourceManager};
use miden_air::{FieldExtension, HashFunction, PublicInputs};
use processor::{
    DefaultHost, Program, ProgramInfo,
    crypto::{RandomCoin, RpoRandomCoin},
};
use rstest::rstest;
use test_utils::{
    AdviceInputs, MemAdviceProvider, ProvingOptions, StackInputs, VerifierError,
    proptest::proptest, prove,
};
use verifier_recursive::{VerifierData, generate_advice_inputs};
use vm_core::Word;

// Note: Changes to Miden VM may cause this test to fail when some of the assumptions documented
// in `stdlib/asm/crypto/stark/verifier.masm` are violated.
#[rstest]
#[case(None)]
#[case(Some(KERNEL_ODD_NUM_PROC))]
#[case(Some(KERNEL_EVEN_NUM_PROC))]
fn stark_verifier_e2f4(#[case] kernel: Option<&str>) {
    // An example MASM program to be verified inside Miden VM.
    let example_source = "begin
            repeat.32
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
        advice_map,
    } = generate_recursive_verifier_data(example_source, stack_inputs, kernel).unwrap();

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
            push.1 exec.constants::set_rpo_current

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
