use super::{compile, execute, push_to_stack, test_execution_failure, ProgramInputs};
use rand_utils::rand_value;

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

// ADVICE INJECTORS
// ================================================================================================

#[test]
fn adv_inject_u64div() {
    let script = compile("begin adv.u64div push.adv.4 end");

    // get two random 64-bit integers and split them into 32-bit limbs
    let a = rand_value::<u64>();
    let a_hi = a >> 32;
    let a_lo = a as u32 as u64;

    let b = rand_value::<u64>();
    let b_hi = b >> 32;
    let b_lo = b as u32 as u64;

    // compute expected quotient
    let q = a / b;
    let q_hi = q >> 32;
    let q_lo = q as u32 as u64;

    // compute expected remainder
    let r = a % b;
    let r_hi = r >> 32;
    let r_lo = r as u32 as u64;

    // inject a/b into the advice tape and then read these values from the tape
    let inputs = ProgramInputs::new(&[a_lo, a_hi, b_lo, b_hi], &[], vec![]).unwrap();
    let trace = execute(&script, &inputs).unwrap();

    let expected = push_to_stack(&[a_lo, a_hi, b_lo, b_hi, q_lo, q_hi, r_lo, r_hi]);
    assert_eq!(expected, trace.last_stack_state());
}
