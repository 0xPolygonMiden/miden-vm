use super::{enforce_constraints, EvaluationFrame, NUM_CONSTRAINTS};
use crate::stack::op_flags::{generate_evaluation_frame, OpFlags};
use vm_core::{Felt, FieldElement, Operation, STACK_TRACE_OFFSET};

use proptest::prelude::*;

// RANDOMIZED TESTS
// ================================================================================================

proptest! {
    #[test]
    fn test_stack_operation(a in any::<u64>(), b in any::<u64>()) {
        let expected = [Felt::ZERO; NUM_CONSTRAINTS];
        let frame = get_swap_test_frame(a, b);
        let result = get_constraint_evaluation(frame);
        assert_eq!(expected, result);
    }
}

// TEST HELPERS
// ================================================================================================

/// Returns the result of SWAP operation constraint evaluations on the provided frame.
fn get_constraint_evaluation(frame: EvaluationFrame<Felt>) -> [Felt; NUM_CONSTRAINTS] {
    let mut result = [Felt::ZERO; NUM_CONSTRAINTS];

    let op_flag = &OpFlags::new(&frame);

    enforce_constraints(&frame, &mut result, op_flag);

    result
}

/// Generates the correct current and next rows for the SWAP operation and inputs and
/// returns an EvaluationFrame for testing.
pub fn get_swap_test_frame(a: u64, b: u64) -> EvaluationFrame<Felt> {
    // frame initialised with a swap operation using it's unique opcode.
    let mut frame = generate_evaluation_frame(Operation::Swap.op_code() as usize);

    // Set the output.
    frame.current_mut()[STACK_TRACE_OFFSET] = Felt::new(a);
    frame.current_mut()[STACK_TRACE_OFFSET + 1] = Felt::new(b);
    frame.next_mut()[STACK_TRACE_OFFSET] = Felt::new(b);
    frame.next_mut()[STACK_TRACE_OFFSET + 1] = Felt::new(a);

    frame
}
