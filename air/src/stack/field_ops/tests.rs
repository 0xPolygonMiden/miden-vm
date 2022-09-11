use super::{enforce_constraints, EvaluationFrame, NUM_CONSTRAINTS};
use crate::stack::op_flags::{generate_evaluation_frame, OpFlags};
use core::ops::Neg;
use vm_core::{Felt, FieldElement, Operation, ONE, STACK_TRACE_OFFSET, ZERO};

use proptest::prelude::*;

// RANDOMIZED TESTS
// ================================================================================================

proptest! {
    // --------------------------------INCR test --------------------------------------------------

    #[test]
    fn test_incr_stack_operation(a in any::<u64>()) {
        let expected = [Felt::ZERO; NUM_CONSTRAINTS];
        let frame = get_incr_test_frame(a);
        let result = get_constraint_evaluation(frame);
        assert_eq!(expected, result);
    }

    // --------------------------------INV test --------------------------------------------------

    #[test]
    fn test_inv_stack_operation(a in any::<u64>()) {
        let expected = [Felt::ZERO; NUM_CONSTRAINTS];
        let frame = get_inv_test_frame(a);
        let result = get_constraint_evaluation(frame);
        assert_eq!(expected, result);
    }
    // --------------------------------NEG test --------------------------------------------------

    #[test]
    fn test_neg_stack_operation(a in any::<u64>()) {
        let expected = [Felt::ZERO; NUM_CONSTRAINTS];
        let frame = get_neg_test_frame(a);
        let result = get_constraint_evaluation(frame);
        assert_eq!(expected, result);
    }
}

// UNIT TESTS
// ================================================================================================

// --------------------------------NOT test --------------------------------------------------

#[test]
fn test_not_stack_operation() {
    let expected = [Felt::ZERO; NUM_CONSTRAINTS];

    // ----------------- top element is 1 -----------------------------------------------------
    let a = ONE;
    let frame = get_not_test_frame(a);
    let result = get_constraint_evaluation(frame);
    assert_eq!(expected, result);

    // ----------------- top element is 0 -----------------------------------------------------
    let a = ZERO;
    let frame = get_not_test_frame(a);
    let result = get_constraint_evaluation(frame);
    assert_eq!(expected, result);
}

// TEST HELPERS
// ================================================================================================

/// Returns the result of stack operation constraint evaluations on the provided frame.
fn get_constraint_evaluation(frame: EvaluationFrame<Felt>) -> [Felt; NUM_CONSTRAINTS] {
    let mut result = [Felt::ZERO; NUM_CONSTRAINTS];

    let op_flag = OpFlags::new(&frame);

    enforce_constraints(&frame, &mut result, &op_flag);

    result
}

/// Generates the correct current and next rows for the INCR operation and inputs and
/// returns an EvaluationFrame for testing.
pub fn get_incr_test_frame(a: u64) -> EvaluationFrame<Felt> {
    // frame initialised with a Incr operation using it's unique opcode.
    let mut frame = generate_evaluation_frame(Operation::Incr.op_code() as usize);

    // Set the output. First element in the next frame should be 1 + first
    // element of the current frame.
    frame.current_mut()[STACK_TRACE_OFFSET] = Felt::new(a);
    frame.next_mut()[STACK_TRACE_OFFSET] = Felt::new(a) + ONE;

    frame
}

/// Generates the correct current and next rows for the INV operation and inputs and
/// returns an EvaluationFrame for testing.
pub fn get_inv_test_frame(a: u64) -> EvaluationFrame<Felt> {
    // frame initialised with a Inv operation using it's unique opcode.
    let mut frame = generate_evaluation_frame(Operation::Inv.op_code() as usize);

    // Set the output. First element in the next frame should be the inverse of
    // the first element of the current frame.
    frame.current_mut()[STACK_TRACE_OFFSET] = Felt::new(a);
    frame.next_mut()[STACK_TRACE_OFFSET] = Felt::new(a).inv();

    frame
}

/// Generates the correct current and next rows for the NEG operation and inputs and
/// returns an EvaluationFrame for testing.
pub fn get_neg_test_frame(a: u64) -> EvaluationFrame<Felt> {
    // frame initialised with a Neg operation using it's unique opcode.
    let mut frame = generate_evaluation_frame(Operation::Neg.op_code() as usize);

    // Set the output. First element in the next frame should be the negation of
    // the first element of the current frame.
    frame.current_mut()[STACK_TRACE_OFFSET] = Felt::new(a);
    frame.next_mut()[STACK_TRACE_OFFSET] = Felt::new(a).neg();

    frame
}

/// Generates the correct current and next rows for the NOT operation and inputs and
/// returns an EvaluationFrame for testing.
pub fn get_not_test_frame(a: Felt) -> EvaluationFrame<Felt> {
    // frame initialised with a NOT operation using it's unique opcode.
    let mut frame = generate_evaluation_frame(Operation::Not.op_code() as usize);

    // Set the output. First element in the next frame should be the binary not of
    // the first element of the current frame.
    frame.current_mut()[STACK_TRACE_OFFSET] = a;
    frame.next_mut()[STACK_TRACE_OFFSET] = ONE - a;

    frame
}
