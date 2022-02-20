use super::{compile, execute, push_to_stack, test_execution_failure, ProgramInputs};

// PUSHING VALUES ONTO THE STACK (PUSH)
// ================================================================================================

#[test]
fn push_adv() {
    let advice_tape = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    // --- push 1 ---------------------------------------------------------------------------------
    let n = 1;
    let script = compile(format!("begin push.adv.{} end", n).as_str());

    let inputs = ProgramInputs::new(&[], &advice_tape, vec![]).unwrap();
    let trace = execute(&script, &inputs).unwrap();
    let expected = push_to_stack(&advice_tape[..n]);

    assert_eq!(expected, trace.last_stack_state());

    // --- push max -------------------------------------------------------------------------------
    let n = 16;
    let script = compile(format!("begin push.adv.{} end", n).as_str());

    let inputs = ProgramInputs::new(&[], &advice_tape, vec![]).unwrap();
    let trace = execute(&script, &inputs).unwrap();
    let expected = push_to_stack(&advice_tape[..n]);

    assert_eq!(expected, trace.last_stack_state());
}

#[test]
fn push_adv_invalid() {
    // attempting to read from empty advice tape should throw an error
    test_execution_failure("push.adv.1", &[], "EmptyAdviceTape");
}

// OVERWRITING VALUES ON THE STACK (LOAD)
// ================================================================================================

#[test]
fn loadw_adv() {
    let script = compile("begin loadw.adv end");
    let advice_tape = [1, 2, 3, 4];

    let inputs = ProgramInputs::new(&[8, 7, 6, 5], &advice_tape, vec![]).unwrap();
    let trace = execute(&script, &inputs).unwrap();
    let expected = push_to_stack(&advice_tape);

    assert_eq!(expected, trace.last_stack_state());
}

#[test]
fn loadw_adv_invalid() {
    // attempting to read from empty advice tape should throw an error
    test_execution_failure("loadw.adv", &[0, 0, 0, 0], "EmptyAdviceTape");
}
