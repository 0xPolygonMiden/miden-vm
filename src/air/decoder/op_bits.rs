use super::{
    are_equal, binary_not, is_binary, BaseElement, EvaluationResult, FieldElement, FlowOps,
    UserOps, VmTransition, CYCLE_MASK_IDX, PREFIX_MASK_IDX, PUSH_MASK_IDX,
};

// CONSTRAINT EVALUATOR
// ================================================================================================

pub fn enforce_op_bits<E>(result: &mut [E], transition: &VmTransition<E>, masks: &[E])
where
    E: FieldElement<BaseField = BaseElement>,
{
    let mut i = 0;

    let current = transition.current();
    let next = transition.next();

    // make sure all op bits are binary and compute their product/sum
    let mut cf_bit_sum = E::ZERO;
    for &op_bit in current.cf_op_bits() {
        result[i] = is_binary(op_bit);
        cf_bit_sum += op_bit;
        i += 1;
    }

    let mut ld_bit_prod = E::ONE;
    for &op_bit in current.ld_op_bits() {
        result[i] = is_binary(op_bit);
        ld_bit_prod *= op_bit;
        i += 1;
    }

    let mut hd_bit_prod = E::ONE;
    for &op_bit in current.hd_op_bits() {
        result[i] = is_binary(op_bit);
        hd_bit_prod *= op_bit;
        i += 1;
    }

    // when cf_ops = hacc, operation counter should be incremented by 1;
    // otherwise, operation counter should remain the same
    let op_counter = current.op_counter();
    let is_hacc = transition.cf_op_flags()[FlowOps::Hacc.op_index()];
    let hacc_transition = (op_counter + E::ONE) * is_hacc;
    let rest_transition = op_counter * binary_not(is_hacc);
    result[i] = are_equal(hacc_transition + rest_transition, next.op_counter());
    i += 1;

    // ld_ops and hd_ops can be all 0s at the first step, but cannot be all 0s
    // at any other step
    result[i] = op_counter * binary_not(ld_bit_prod) * binary_not(hd_bit_prod);
    i += 1;

    // when cf_ops are not all 0s, ld_ops and hd_ops must be all 1s
    result[i] = cf_bit_sum * binary_not(ld_bit_prod * hd_bit_prod);
    i += 1;

    let cf_op_flags = transition.cf_op_flags();

    // VOID can be followed only by VOID
    let current_void_flag = cf_op_flags[FlowOps::Void.op_index()];
    let next_void_flag = next.get_void_op_flag();
    result[i] = current_void_flag * binary_not(next_void_flag);
    i += 1;

    let hd_op_flags = transition.hd_op_flags();

    // BEGIN, LOOP, BREAK, and WRAP are allowed only on one less than multiple of 16
    let prefix_mask = masks[PREFIX_MASK_IDX];
    result.agg_constraint(i, cf_op_flags[FlowOps::Begin.op_index()], prefix_mask);
    result.agg_constraint(i, cf_op_flags[FlowOps::Loop.op_index()], prefix_mask);
    result.agg_constraint(i, cf_op_flags[FlowOps::Wrap.op_index()], prefix_mask);
    result.agg_constraint(i, cf_op_flags[FlowOps::Break.op_index()], prefix_mask);

    // TEND and FEND is allowed only on multiples of 16
    let base_cycle_mask = masks[CYCLE_MASK_IDX];
    result.agg_constraint(i, cf_op_flags[FlowOps::Tend.op_index()], base_cycle_mask);
    result.agg_constraint(i, cf_op_flags[FlowOps::Fend.op_index()], base_cycle_mask);

    // PUSH is allowed only on multiples of 8
    let push_cycle_mask = masks[PUSH_MASK_IDX];
    result.agg_constraint(i, hd_op_flags[UserOps::Push.hd_index()], push_cycle_mask);
}

// TESTS
// ================================================================================================

// TODO: migrate
