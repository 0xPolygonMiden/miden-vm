use super::{
    are_equal, enforce_left_shift, enforce_right_shift, enforce_stack_copy, BaseElement,
    EvaluationResult,
};

// STACK MANIPULATION OPERATIONS
// ================================================================================================

/// Enforces constraints for DUP operation. The constraints are based on the first element
/// of the stack; the old stack is shifted right by 1 element.
pub fn enforce_dup(
    result: &mut [BaseElement],
    old_stack: &[BaseElement],
    new_stack: &[BaseElement],
    op_flag: BaseElement,
) {
    result.agg_constraint(0, op_flag, are_equal(new_stack[0], old_stack[0]));
    enforce_right_shift(result, old_stack, new_stack, 1, op_flag);
}

/// Enforces constraints for DUP2 operation. The constraints are based on the first 2 element
/// of the stack; the old stack is shifted right by 2 element.
pub fn enforce_dup2(
    result: &mut [BaseElement],
    old_stack: &[BaseElement],
    new_stack: &[BaseElement],
    op_flag: BaseElement,
) {
    result.agg_constraint(0, op_flag, are_equal(new_stack[0], old_stack[0]));
    result.agg_constraint(1, op_flag, are_equal(new_stack[1], old_stack[1]));
    enforce_right_shift(result, old_stack, new_stack, 2, op_flag);
}

/// Enforces constraints for DUP4 operation. The constraints are based on the first 4 element
/// of the stack; the old stack is shifted right by 4 element.
pub fn enforce_dup4(
    result: &mut [BaseElement],
    old_stack: &[BaseElement],
    new_stack: &[BaseElement],
    op_flag: BaseElement,
) {
    result.agg_constraint(0, op_flag, are_equal(new_stack[0], old_stack[0]));
    result.agg_constraint(1, op_flag, are_equal(new_stack[1], old_stack[1]));
    result.agg_constraint(2, op_flag, are_equal(new_stack[2], old_stack[2]));
    result.agg_constraint(3, op_flag, are_equal(new_stack[3], old_stack[3]));
    enforce_right_shift(result, old_stack, new_stack, 4, op_flag);
}

/// Enforces constraints for PAD2 operation. The constraints are based on the first 2 element
/// of the stack; the old stack is shifted right by 2 element.
pub fn enforce_pad2(
    result: &mut [BaseElement],
    old_stack: &[BaseElement],
    new_stack: &[BaseElement],
    op_flag: BaseElement,
) {
    result.agg_constraint(0, op_flag, new_stack[0]);
    result.agg_constraint(1, op_flag, new_stack[1]);
    enforce_right_shift(result, old_stack, new_stack, 2, op_flag);
}

// Enforces constraints for DROP operation. The stack is simply shifted left by 1 element.
pub fn enforce_drop(
    result: &mut [BaseElement],
    old_stack: &[BaseElement],
    new_stack: &[BaseElement],
    op_flag: BaseElement,
) {
    enforce_left_shift(result, old_stack, new_stack, 1, 1, op_flag);
}

// Enforces constraints for DROP4 operation. The stack is simply shifted left by 4 element.
pub fn enforce_drop4(
    result: &mut [BaseElement],
    old_stack: &[BaseElement],
    new_stack: &[BaseElement],
    op_flag: BaseElement,
) {
    enforce_left_shift(result, old_stack, new_stack, 4, 4, op_flag);
}

/// Enforces constraints for SWAP operation. The constraints are based on the first 2 element
/// of the stack; the rest of the stack is unaffected.
pub fn enforce_swap(
    result: &mut [BaseElement],
    old_stack: &[BaseElement],
    new_stack: &[BaseElement],
    op_flag: BaseElement,
) {
    result.agg_constraint(0, op_flag, are_equal(new_stack[0], old_stack[1]));
    result.agg_constraint(0, op_flag, are_equal(new_stack[1], old_stack[0]));
    enforce_stack_copy(result, old_stack, new_stack, 2, op_flag);
}

/// Enforces constraints for SWAP2 operation. The constraints are based on the first 4 element
/// of the stack; the rest of the stack is unaffected.
pub fn enforce_swap2(
    result: &mut [BaseElement],
    old_stack: &[BaseElement],
    new_stack: &[BaseElement],
    op_flag: BaseElement,
) {
    result.agg_constraint(0, op_flag, are_equal(new_stack[0], old_stack[2]));
    result.agg_constraint(1, op_flag, are_equal(new_stack[1], old_stack[3]));
    result.agg_constraint(2, op_flag, are_equal(new_stack[2], old_stack[0]));
    result.agg_constraint(3, op_flag, are_equal(new_stack[3], old_stack[1]));
    enforce_stack_copy(result, old_stack, new_stack, 4, op_flag);
}

/// Enforces constraints for SWAP4 operation. The constraints are based on the first 8 element
/// of the stack; the rest of the stack is unaffected.
pub fn enforce_swap4(
    result: &mut [BaseElement],
    old_stack: &[BaseElement],
    new_stack: &[BaseElement],
    op_flag: BaseElement,
) {
    result.agg_constraint(0, op_flag, are_equal(new_stack[0], old_stack[4]));
    result.agg_constraint(1, op_flag, are_equal(new_stack[1], old_stack[5]));
    result.agg_constraint(2, op_flag, are_equal(new_stack[2], old_stack[6]));
    result.agg_constraint(3, op_flag, are_equal(new_stack[3], old_stack[7]));
    result.agg_constraint(4, op_flag, are_equal(new_stack[4], old_stack[0]));
    result.agg_constraint(5, op_flag, are_equal(new_stack[5], old_stack[1]));
    result.agg_constraint(6, op_flag, are_equal(new_stack[6], old_stack[2]));
    result.agg_constraint(7, op_flag, are_equal(new_stack[7], old_stack[3]));
    enforce_stack_copy(result, old_stack, new_stack, 8, op_flag);
}

/// Enforces constraints for ROLL4 operation. The constraints are based on the first 4 element
/// of the stack; the rest of the stack is unaffected.
pub fn enforce_roll4(
    result: &mut [BaseElement],
    old_stack: &[BaseElement],
    new_stack: &[BaseElement],
    op_flag: BaseElement,
) {
    result.agg_constraint(0, op_flag, are_equal(new_stack[0], old_stack[3]));
    result.agg_constraint(1, op_flag, are_equal(new_stack[1], old_stack[0]));
    result.agg_constraint(2, op_flag, are_equal(new_stack[2], old_stack[1]));
    result.agg_constraint(3, op_flag, are_equal(new_stack[3], old_stack[2]));
    enforce_stack_copy(result, old_stack, new_stack, 4, op_flag);
}

/// Enforces constraints for ROLL8 operation. The constraints are based on the first 8 element
/// of the stack; the rest of the stack is unaffected.
pub fn enforce_roll8(
    result: &mut [BaseElement],
    old_stack: &[BaseElement],
    new_stack: &[BaseElement],
    op_flag: BaseElement,
) {
    result.agg_constraint(0, op_flag, are_equal(new_stack[0], old_stack[7]));
    result.agg_constraint(1, op_flag, are_equal(new_stack[1], old_stack[0]));
    result.agg_constraint(2, op_flag, are_equal(new_stack[2], old_stack[1]));
    result.agg_constraint(3, op_flag, are_equal(new_stack[3], old_stack[2]));
    result.agg_constraint(4, op_flag, are_equal(new_stack[4], old_stack[3]));
    result.agg_constraint(5, op_flag, are_equal(new_stack[5], old_stack[4]));
    result.agg_constraint(6, op_flag, are_equal(new_stack[6], old_stack[5]));
    result.agg_constraint(7, op_flag, are_equal(new_stack[7], old_stack[6]));
    enforce_stack_copy(result, old_stack, new_stack, 8, op_flag);
}
