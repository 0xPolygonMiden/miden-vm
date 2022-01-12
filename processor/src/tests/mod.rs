use super::{Felt, FieldElement, ProgramInputs, STACK_TOP_SIZE};

#[test]
fn simple_program() {
    let assembler = assembly::Assembler::new();
    let script = assembler
        .compile_script("begin push.1 push.2 add end")
        .unwrap();

    let inputs = ProgramInputs::none();
    let trace = super::execute(&script, &inputs).unwrap();

    let last_state = trace.last_stack_state();
    let expected_state = build_stack_state(&[3]);
    assert_eq!(expected_state, last_state);
}

// HELPER FUNCTIONS
// ================================================================================================

fn build_stack_state(values: &[u64]) -> [Felt; STACK_TOP_SIZE] {
    let mut result = [Felt::ZERO; STACK_TOP_SIZE];
    for (&value, result) in values.iter().zip(result.iter_mut()) {
        *result = Felt::new(value);
    }
    result
}
