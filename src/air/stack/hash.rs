use super::{
    are_equal, enforce_stack_copy, BaseElement, EvaluationResult, FieldElement, HASH_STATE_WIDTH,
};
use crate::utils::hasher::{apply_inv_mds, apply_mds, apply_sbox};

/// Evaluates constraints for a single round of a modified Rescue hash function. Hash state is
/// assumed to be in the first 6 registers of user stack; the rest of the stack does not change.
pub fn enforce_rescr<E: FieldElement<BaseField = BaseElement>>(
    result: &mut [E],
    old_stack: &[E],
    new_stack: &[E],
    ark: &[E],
    op_flag: E,
) {
    // evaluate the first half of Rescue round
    let mut old_state = [E::ZERO; HASH_STATE_WIDTH];
    old_state.copy_from_slice(&old_stack[..HASH_STATE_WIDTH]);
    for i in 0..HASH_STATE_WIDTH {
        old_state[i] += ark[i];
    }
    apply_sbox(&mut old_state);
    apply_mds(&mut old_state);

    // evaluate inverse of the second half of Rescue round
    let mut new_state = [E::ZERO; HASH_STATE_WIDTH];
    new_state.copy_from_slice(&new_stack[..HASH_STATE_WIDTH]);
    apply_inv_mds(&mut new_state);
    apply_sbox(&mut new_state);
    for i in 0..HASH_STATE_WIDTH {
        new_state[i] -= ark[HASH_STATE_WIDTH + i];
    }

    // compar the results of both rounds
    for i in 0..HASH_STATE_WIDTH {
        result.agg_constraint(i, op_flag, are_equal(new_state[i], old_state[i]));
    }

    // make sure the rest of the stack didn't change
    enforce_stack_copy(result, old_stack, new_stack, HASH_STATE_WIDTH, op_flag);
}
