use super::{enforce_constraints, EvaluationFrame, NUM_CONSTRAINTS};
use crate::stack::op_flags::{generate_evaluation_frame, OpFlags};
use vm_core::{Felt, FieldElement, Operation, FMP_COL_IDX, STACK_TRACE_OFFSET};

use proptest::prelude::*;

// RANDOMIZED TESTS
// ================================================================================================

proptest! {
    #[test]
    fn test_stack_operation(a in any::<u64>()) {
        let expected = [Felt::ZERO; NUM_CONSTRAINTS];
        let frame = get_system_test_frame(a);
        let result = get_constraint_evaluation(frame);
        assert_eq!(expected, result);
    }
}

// TEST HELPERS
// ================================================================================================

/// Returns the result of stack operation constraint evaluations on the provided frame.
fn get_constraint_evaluation(frame: EvaluationFrame<Felt>) -> [Felt; NUM_CONSTRAINTS] {
    let mut result = [Felt::ZERO; NUM_CONSTRAINTS];

    let op_flag = &OpFlags::new(&frame);

    enforce_constraints(&frame, &mut result, op_flag);

    result
}

/// Generates the correct current and next rows for the fmpadd operation and inputs and
/// returns an EvaluationFrame for testing.
pub fn get_system_test_frame(a: u64) -> EvaluationFrame<Felt> {
    // frame initialised with a fmpadd operation using it's unique opcode.
    let mut frame = generate_evaluation_frame(Operation::FmpAdd.op_code() as usize);

    // Set the output. First element in the next frame should be incremented
    // by value present in the fmp register.
    frame.current_mut()[FMP_COL_IDX] = Felt::new(a);
    frame.next_mut()[STACK_TRACE_OFFSET] =
        frame.current()[STACK_TRACE_OFFSET] + frame.current()[FMP_COL_IDX];

    frame
}
