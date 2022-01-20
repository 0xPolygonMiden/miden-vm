use super::{Felt, FieldElement, ProgramInputs, Script, STACK_TOP_SIZE};
use crate::Word;

mod crypto_ops;
mod field_ops;
mod flow_control;

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
