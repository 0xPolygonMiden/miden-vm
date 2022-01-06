use super::{BaseElement, FieldElement, Process, ProgramInputs, STACK_TOP_SIZE};

#[test]
fn simple_program() {
    let assembler = assembly::v1::Assembler::new();
    let script = assembler
        .compile_script("begin push.1 push.2 add end")
        .unwrap();

    let inputs = ProgramInputs::none();
    let mut processor = Process::new(inputs);

    let trace = processor.execute(&script).unwrap();

    let last_state = trace.last_stack_state();
    let expected_state = build_stack_state(&[3]);
    assert_eq!(expected_state, last_state);
}

// HELPER FUNCTIONS
// ================================================================================================

fn build_stack_state(values: &[u64]) -> [BaseElement; STACK_TOP_SIZE] {
    let mut result = [BaseElement::ZERO; STACK_TOP_SIZE];
    for (&value, result) in values.iter().zip(result.iter_mut()) {
        *result = BaseElement::new(value);
    }
    result
}
