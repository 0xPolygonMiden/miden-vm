use super::{
    are_equal, BaseElement, EvaluationResult, FieldElement, TraceState, UserOps, SPONGE_WIDTH,
};
use crate::utils::sponge::{apply_inv_mds, apply_mds, apply_sbox};

// CONSTRAINT EVALUATOR
// ================================================================================================

pub fn enforce_hacc(
    result: &mut [BaseElement],
    current: &TraceState,
    next: &TraceState,
    ark: &[BaseElement],
    op_flag: BaseElement,
) {
    // determine current op_value
    let stack_top = next.user_stack()[0];
    let push_flag = current.hd_op_flags()[UserOps::Push.hd_index()];
    let op_value = stack_top * push_flag;

    // evaluate the first half of Rescue round
    let mut old_sponge = [BaseElement::ZERO; SPONGE_WIDTH];
    old_sponge.copy_from_slice(current.sponge());
    for i in 0..SPONGE_WIDTH {
        old_sponge[i] += ark[i];
    }
    apply_sbox(&mut old_sponge);
    apply_mds(&mut old_sponge);

    // op_code injection
    old_sponge[0] += current.op_code();
    old_sponge[1] += op_value;

    // evaluate inverse of the second half of Rescue round
    let mut new_sponge = [BaseElement::ZERO; SPONGE_WIDTH];
    new_sponge.copy_from_slice(next.sponge());
    apply_inv_mds(&mut new_sponge);
    apply_sbox(&mut new_sponge);
    for i in 0..SPONGE_WIDTH {
        new_sponge[i] -= ark[SPONGE_WIDTH + i];
    }

    // add the constraints to the result
    for i in 0..SPONGE_WIDTH {
        result.agg_constraint(i, op_flag, are_equal(old_sponge[i], new_sponge[i]));
    }
}
