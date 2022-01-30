use super::{execute, Felt, FieldElement, ProgramInputs, Script, STACK_TOP_SIZE};
use crate::Word;
use proptest::prelude::*;

mod crypto_ops;
mod field_ops;
mod flow_control;
mod u32_ops;

// TESTS
// ================================================================================================

#[test]
fn simple_program() {
    let script = compile("begin push.1 push.2 add end");

    let inputs = ProgramInputs::none();
    let trace = super::execute(&script, &inputs).unwrap();

    let last_state = trace.last_stack_state();
    let expected_state = build_stack_state(&[3]);
    assert_eq!(expected_state, last_state);
}

// HELPER FUNCTIONS
// ================================================================================================

fn compile(source: &str) -> Script {
    let assembler = assembly::Assembler::new();
    assembler.compile_script(source).unwrap()
}

fn build_inputs(stack_init: &[u64]) -> ProgramInputs {
    ProgramInputs::new(stack_init, &[], vec![]).unwrap()
}

fn build_stack_state(values: &[u64]) -> [Felt; STACK_TOP_SIZE] {
    let mut result = [Felt::ZERO; STACK_TOP_SIZE];
    for (&value, result) in values.iter().zip(result.iter_mut()) {
        *result = Felt::new(value);
    }
    result
}

/// This helper function tests that when the given assembly instruction is executed on the
/// the provided inputs, it results in the provided outputs.
fn test_execution(asm_op: &str, inputs: &[u64], outputs: &[u64]) {
    let script = compile(format!("begin {} end", asm_op).as_str());

    let inputs = build_inputs(inputs);
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();

    let expected_state = build_stack_state(outputs);
    assert_eq!(expected_state, last_state);
}

/// This helper function tests failures where the execution of a given assembly operation with the
/// provided inputs is expected to panic. This function catches the panic and tests it against a
/// provided string to make sure it contains the expected error string.
fn test_execution_failure(asm_op: &str, inputs: &[u64], err_substr: &str) {
    let script = compile(format!("begin {} end", asm_op).as_str());

    let inputs = build_inputs(inputs);
    assert_eq!(
        std::panic::catch_unwind(|| execute(&script, &inputs).unwrap())
            .err()
            .and_then(|a| { a.downcast_ref::<String>().map(|s| s.contains(err_substr)) }),
        Some(true)
    );
}

/// This helper function tests failures where the compilation of a given assembly operation is
/// expected to panic. This function catches the panic and tests it against a provided string to
/// make sure it contains the expected error string.
fn test_compilation_failure(asm_op: &str, err_substr: &str) {
    assert_eq!(
        std::panic::catch_unwind(|| compile(format!("begin {} end", asm_op).as_str()))
            .err()
            .and_then(|a| { a.downcast_ref::<String>().map(|s| s.contains(err_substr)) }),
        Some(true)
    );
}

/// This helper function tests a provided assembly operation which takes a single parameter
/// to ensure that it fails when that parameter is over the maximum allowed value (out of bounds).
fn test_param_out_of_bounds(asm_op_base: &str, gt_max_value: u64) {
    let build_asm_op = |param: u64| format!("{}.{}", asm_op_base, param);

    test_compilation_failure(build_asm_op(gt_max_value).as_str(), "parameter value");
}

// This is a proptest strategy for generating a random word with 4 values of type T.
fn rand_word<T: proptest::arbitrary::Arbitrary>() -> impl Strategy<Value = Vec<T>> {
    prop::collection::vec(any::<T>(), 4)
}
