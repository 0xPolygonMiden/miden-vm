use std::sync::Arc;
mod verifier_recursive;
use assembly::{Assembler, DefaultSourceManager};
use miden_air::{FieldExtension, HashFunction, PublicInputs};
use processor::{DefaultHost, Program, ProgramInfo};
use test_utils::{
    AdviceInputs, MemAdviceProvider, ProvingOptions, StackInputs, VerifierError, prove,
};
use verifier_recursive::{VerifierData, generate_advice_inputs};

// Note: Changes to Miden VM may cause this test to fail when some of the assumptions documented
// in `stdlib/asm/crypto/stark/verifier.masm` are violated.
#[test]
fn stark_verifier_e2f4_without_kernel() {
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
    } = generate_recursive_verifier_data(example_source, stack_inputs, None).unwrap();

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

// Note: Changes to Miden VM may cause this test to fail when some of the assumptions documented
// in `stdlib/asm/crypto/stark/verifier.masm` are violated.
#[test]
fn stark_verifier_e2f4_with_kernel_odd_number_procedures() {
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
    } = generate_recursive_verifier_data(example_source, stack_inputs, Some(KERNEL_ODD_NUM_PROC))
        .unwrap();

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

// Note: Changes to Miden VM may cause this test to fail when some of the assumptions documented
// in `stdlib/asm/crypto/stark/verifier.masm` are violated.
#[test]
fn stark_verifier_e2f4_with_kernel_even_number_procedures() {
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
    } = generate_recursive_verifier_data(example_source, stack_inputs, Some(KERNEL_EVEN_NUM_PROC))
        .unwrap();

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
