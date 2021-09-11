use super::{enforce_right_shift, BaseElement};

/// Enforces constraints for PUSH operation. The constraints on the first element of the stack
/// are enforced in the Decoder where the value pushed onto the stack is injected into sponge
/// state. This constraint enforces that the rest of the stack is shifted right by 1 element.
pub fn enforce_push(
    result: &mut [BaseElement],
    old_stack: &[BaseElement],
    new_stack: &[BaseElement],
    op_flag: BaseElement,
) {
    enforce_right_shift(result, old_stack, new_stack, 1, op_flag);
}

/// Enforces constraints for READ operation. No constraints are placed on the first element of
/// the stack; the old stack is shifted right by 1 element.
pub fn enforce_read(
    result: &mut [BaseElement],
    old_stack: &[BaseElement],
    new_stack: &[BaseElement],
    op_flag: BaseElement,
) {
    enforce_right_shift(result, old_stack, new_stack, 1, op_flag);
}

/// Enforces constraints for READ2 operation. No constraints are placed on the first two elements
/// of the stack; the old stack is shifted right by 2 element.
pub fn enforce_read2(
    result: &mut [BaseElement],
    old_stack: &[BaseElement],
    new_stack: &[BaseElement],
    op_flag: BaseElement,
) {
    enforce_right_shift(result, old_stack, new_stack, 2, op_flag);
}
