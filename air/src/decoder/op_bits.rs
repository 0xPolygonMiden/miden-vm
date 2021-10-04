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

#[cfg(test)]
mod tests {

    use super::{super::NUM_OP_CONSTRAINTS, FlowOps, UserOps, VmTransition};
    use crate::{TraceState, ToElements};
    use winterfell::math::{fields::f128::BaseElement, FieldElement, StarkField};

    #[test]
    fn op_bits_are_binary() {
        let success_result = vec![BaseElement::ZERO; NUM_OP_CONSTRAINTS];

        // all bits are 1s: success
        let state = new_state(FlowOps::Void as u8, UserOps::Noop as u8, 1);
        assert_eq!(success_result, evaluate_state(&state, [0, 0, 0], false));

        // control flow bits are not binary
        for i in 0..3 {
            let mut op_bits = [1; 3];
            op_bits[i] = 3;
            let mut expected_evaluations = vec![BaseElement::ZERO; 10];
            expected_evaluations[i] = BaseElement::new(3 * 3 - 3);

            let state = new_state_from_bits(op_bits, [1, 1, 1, 1, 1, 1, 1]);
            assert_eq!(
                expected_evaluations,
                &evaluate_state(&state, [0, 0, 0], false)[..10]
            );
        }

        // user bits are not binary
        for i in 0..7 {
            let mut op_bits = [1, 1, 1, 1, 1, 1, 1];
            op_bits[i] = 3;
            let mut expected_evaluations = vec![BaseElement::ZERO; 10];
            expected_evaluations[i + 3] = BaseElement::new(3 * 3 - 3);

            let state = new_state_from_bits([0, 0, 0], op_bits);
            assert_eq!(
                expected_evaluations,
                &evaluate_state(&state, [0, 0, 0], false)[..10]
            );
        }
    }

    #[test]
    fn invalid_op_combinations() {
        let success_result = vec![BaseElement::ZERO; NUM_OP_CONSTRAINTS];

        // user op bits cannot be all 0s
        for cf_op in 0..8 {
            let state = new_state(cf_op, 0, 1);
            assert_ne!(success_result, evaluate_state(&state, [0, 0, 0], false));
        }

        // when cf_ops are not all 0s, user_ops must be all 1s
        for cf_op in 1..8 {
            for user_op in 0..127 {
                let state = new_state(cf_op as u8, user_op as u8, 1);
                assert_ne!(success_result, evaluate_state(&state, [0, 0, 0], false));
            }

            let state = new_state(cf_op as u8, UserOps::Noop as u8, 1);
            assert_eq!(success_result, evaluate_state(&state, [0, 0, 0], false));
        }
    }

    #[test]
    fn invalid_op_alignment() {
        let success_result = vec![BaseElement::ZERO; NUM_OP_CONSTRAINTS];

        // TEND and FEND are allowed only on multiples of 16
        let state = new_state(FlowOps::Tend as u8, UserOps::Noop as u8, 1);
        assert_eq!(success_result, evaluate_state(&state, [0, 0, 0], false));
        assert_ne!(success_result, evaluate_state(&state, [1, 0, 0], false));

        let state = new_state(FlowOps::Fend as u8, UserOps::Noop as u8, 1);
        assert_eq!(success_result, evaluate_state(&state, [0, 0, 0], false));
        assert_ne!(success_result, evaluate_state(&state, [1, 0, 0], false));

        // BEGIN, LOOP, WRAP, and BREAK are allowed only on one less than multiples of 16
        let state = new_state(FlowOps::Begin as u8, UserOps::Noop as u8, 1);
        assert_eq!(success_result, evaluate_state(&state, [0, 0, 0], false));
        assert_ne!(success_result, evaluate_state(&state, [0, 1, 0], false));

        let state = new_state(FlowOps::Loop as u8, UserOps::Noop as u8, 1);
        assert_eq!(success_result, evaluate_state(&state, [0, 0, 0], false));
        assert_ne!(success_result, evaluate_state(&state, [0, 1, 0], false));

        let state = new_state(FlowOps::Wrap as u8, UserOps::Noop as u8, 1);
        assert_eq!(success_result, evaluate_state(&state, [0, 0, 0], false));
        assert_ne!(success_result, evaluate_state(&state, [0, 1, 0], false));

        let state = new_state(FlowOps::Break as u8, UserOps::Noop as u8, 1);
        assert_eq!(success_result, evaluate_state(&state, [0, 0, 0], false));
        assert_ne!(success_result, evaluate_state(&state, [0, 1, 0], false));

        // PUSH is allowed only on multiples of 8
        let state = new_state(FlowOps::Hacc as u8, UserOps::Push as u8, 1);
        assert_eq!(success_result, evaluate_state(&state, [0, 0, 0], true));
        assert_ne!(success_result, evaluate_state(&state, [0, 0, 1], true));
    }

    #[test]
    fn invalid_op_sequence() {
        let success_result = vec![BaseElement::ZERO; NUM_OP_CONSTRAINTS];

        // void can follow non-void
        let state1 = new_state(FlowOps::Hacc as u8, UserOps::Add as u8, 1);
        let state2 = new_state(FlowOps::Void as u8, UserOps::Noop as u8, 2);
        let transition = VmTransition::from_states(state1, state2);
        let mut evaluations = vec![BaseElement::ZERO; NUM_OP_CONSTRAINTS];
        super::enforce_op_bits(&mut evaluations, &transition, &[0, 0, 0].to_elements());
        assert_eq!(success_result, evaluations);

        // void can follow void
        let state1 = new_state(FlowOps::Void as u8, UserOps::Noop as u8, 1);
        let state2 = new_state(FlowOps::Void as u8, UserOps::Noop as u8, 1);
        let transition = VmTransition::from_states(state1, state2);
        let mut evaluations = vec![BaseElement::ZERO; NUM_OP_CONSTRAINTS];
        super::enforce_op_bits(&mut evaluations, &transition, &[0, 0, 0].to_elements());
        assert_eq!(success_result, evaluations);

        // non-void cannot follow void
        let state1 = new_state(FlowOps::Void as u8, UserOps::Noop as u8, 1);
        let state2 = new_state(FlowOps::Hacc as u8, UserOps::Add as u8, 1);
        let transition = VmTransition::from_states(state1, state2);
        let mut evaluations = vec![BaseElement::ZERO; NUM_OP_CONSTRAINTS];
        super::enforce_op_bits(&mut evaluations, &transition, &[0, 0, 0].to_elements());
        assert_ne!(success_result, evaluations);
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------
    fn new_state(flow_op: u8, user_op: u8, op_counter: u128) -> TraceState<BaseElement> {
        let mut state = TraceState::new(1, 0, 1);

        let mut op_bits = [BaseElement::ZERO; 10];
        for i in 0..3 {
            op_bits[i] = BaseElement::new(((flow_op as u128) >> i) & 1);
        }

        for i in 0..7 {
            op_bits[i + 3] = BaseElement::new(((user_op as u128) >> i) & 1);
        }

        state.set_op_bits(op_bits);
        state.set_op_counter(BaseElement::new(op_counter));
        state
    }

    fn new_state_from_bits(cf_bits: [u128; 3], u_bits: [u128; 7]) -> TraceState<BaseElement> {
        let mut state = TraceState::new(1, 0, 1);
        let cf_bits = cf_bits.to_elements();
        let u_bits = u_bits.to_elements();
        state.set_op_bits([
            cf_bits[0], cf_bits[1], cf_bits[2], u_bits[0], u_bits[1], u_bits[2], u_bits[3],
            u_bits[4], u_bits[5], u_bits[6],
        ]);
        state
    }

    fn evaluate_state(
        state: &TraceState<BaseElement>,
        masks: [u128; 3],
        inc_counter: bool,
    ) -> Vec<BaseElement> {
        let op_counter = if inc_counter {
            state.op_counter().as_int() + 1
        } else {
            state.op_counter().as_int()
        };
        let next_state = new_state(FlowOps::Void as u8, UserOps::Noop as u8, op_counter);
        let transition = VmTransition::from_states(state.clone(), next_state);
        let mut evaluations = vec![BaseElement::ZERO; NUM_OP_CONSTRAINTS];
        super::enforce_op_bits(&mut evaluations, &transition, &masks.to_elements());
        evaluations
    }
}
