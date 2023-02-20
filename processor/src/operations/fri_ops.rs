use super::{AdviceProvider, ExecutionError, Felt, Operation, Process};
use vm_core::{ExtensionOf, FieldElement, QuadExtension, StarkField, ONE, ZERO};

// CONSTANTS
// ================================================================================================

const TWO: Felt = Felt::new(2);
const TWO_INV: Felt = Felt::new(9223372034707292161);

const DOMAIN_OFFSET: Felt = Felt::GENERATOR;

const TAU_INV: Felt = Felt::new(18446462594437873665); // tau^{-1}
const TAU2_INV: Felt = Felt::new(18446744069414584320); // tau^{-2}
const TAU3_INV: Felt = Felt::new(281474976710656); // tau^{-3}

// TYPE ALIASES
// ================================================================================================

type QuadFelt = QuadExtension<Felt>;

// FRI OPERATIONS
// ================================================================================================

impl<A> Process<A>
where
    A: AdviceProvider,
{
    // FRI FOLDING OPERATION
    // --------------------------------------------------------------------------------------------
    /// TODO: add docs
    pub(super) fn op_fri_ext2fold4(&mut self) -> Result<(), ExecutionError> {
        // read all relevant variables from the current state of the stack
        let query_values = self.get_query_values();
        let f_pos = self.get_folded_position();
        let d_seg = self.get_domain_segment().as_int();
        let poe = self.get_poe();
        let alpha = self.get_alpha();
        let layer_ptr = self.get_layer_ptr();

        // compute x corresponding to query values
        let f_tau = get_tau_factor(d_seg);
        let x = poe * f_tau * DOMAIN_OFFSET;
        let x_inv = x.inv();

        // fold query values
        let (ev, es) = compute_evaluation_points(alpha, x_inv);
        let (folded_value, tmp0, tmp1) = fold4(query_values, ev, es);

        // write the relevant values into the next state of the stack
        let tmp0 = tmp0.to_base_elements();
        let tmp1 = tmp1.to_base_elements();
        let ds = get_domain_segment_flags(d_seg);
        let folded_value = folded_value.to_base_elements();

        let poe2 = poe.square();
        let poe4 = poe2.square();

        self.stack.set(0, tmp0[1]);
        self.stack.set(1, tmp0[0]);
        self.stack.set(2, tmp1[1]);
        self.stack.set(3, tmp1[0]);
        self.stack.set(4, ds[3]);
        self.stack.set(5, ds[2]);
        self.stack.set(6, ds[1]);
        self.stack.set(7, ds[0]);
        self.stack.set(8, poe2);
        self.stack.set(9, f_tau);
        self.stack.set(10, layer_ptr + TWO);
        self.stack.set(11, poe4);
        self.stack.set(12, f_pos);
        self.stack.set(13, folded_value[1]);
        self.stack.set(14, folded_value[0]);

        self.set_helper_registers(ev, es, x, x_inv);

        self.stack.shift_left(16);
        Ok(())
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// TODO: add docs
    fn get_query_values(&self) -> [QuadFelt; 4] {
        let v7 = self.stack.get(0);
        let v6 = self.stack.get(1);
        let v5 = self.stack.get(2);
        let v4 = self.stack.get(3);
        let v3 = self.stack.get(4);
        let v2 = self.stack.get(5);
        let v1 = self.stack.get(6);
        let v0 = self.stack.get(7);

        [
            QuadFelt::new(v0, v1),
            QuadFelt::new(v2, v3),
            QuadFelt::new(v4, v5),
            QuadFelt::new(v6, v7),
        ]
    }

    /// TODO: add docs
    fn get_folded_position(&self) -> Felt {
        self.stack.get(8)
    }

    /// TODO: add docs
    fn get_domain_segment(&self) -> Felt {
        self.stack.get(9)
    }

    /// TODO: add docs
    fn get_poe(&self) -> Felt {
        self.stack.get(10)
    }

    /// TODO: add docs
    fn get_alpha(&self) -> QuadFelt {
        let a1 = self.stack.get(13);
        let a0 = self.stack.get(14);
        QuadFelt::new(a0, a1)
    }

    fn get_layer_ptr(&self) -> Felt {
        self.stack.get(15)
    }

    /// TODO: add docs
    fn set_helper_registers(&mut self, ev: QuadFelt, es: QuadFelt, x: Felt, x_inv: Felt) {
        let ev_arr = [ev];
        let ev_felts = QuadFelt::as_base_elements(&ev_arr);

        let es_arr = [es];
        let es_felts = QuadFelt::as_base_elements(&es_arr);

        let values = [ev_felts[0], ev_felts[1], es_felts[0], es_felts[1], x, x_inv];
        self.decoder.set_user_op_helpers(Operation::FriE2F4, &values);
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// TODO: add docs
fn get_tau_factor(domain_segment: u64) -> Felt {
    match domain_segment {
        0 => Felt::ONE,
        1 => TAU_INV,
        2 => TAU2_INV,
        3 => TAU3_INV,
        _ => panic!("invalid domain segment {domain_segment}"),
    }
}

/// TODO: add docs
fn get_domain_segment_flags(domain_segment: u64) -> [Felt; 4] {
    match domain_segment {
        0 => [ONE, ZERO, ZERO, ZERO],
        1 => [ZERO, ONE, ZERO, ZERO],
        2 => [ZERO, ZERO, ONE, ZERO],
        3 => [ZERO, ZERO, ZERO, ONE],
        _ => panic!("invalid domain segment {domain_segment}"),
    }
}

/// TODO: add docs
fn compute_evaluation_points(alpha: QuadFelt, x_inv: Felt) -> (QuadFelt, QuadFelt) {
    let ev = alpha.mul_base(x_inv);
    let es = ev.square();
    (ev, es)
}

/// TODO: add docs
fn fold4(values: [QuadFelt; 4], ev: QuadFelt, es: QuadFelt) -> (QuadFelt, QuadFelt, QuadFelt) {
    let tmp0 = fold2(values[0], values[2], ev);
    let tmp1 = fold2(values[1], values[3], ev.mul_base(TAU_INV));
    let folded_value = fold2(tmp0, tmp1, es);
    (folded_value, tmp0, tmp1)
}

/// TODO: add docs
fn fold2(f_x: QuadFelt, f_neg_x: QuadFelt, evaluation_point: QuadFelt) -> QuadFelt {
    (f_x + f_neg_x + ((f_x - f_neg_x) * evaluation_point)).mul_base(TWO_INV)
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {

    use super::{
        ExtensionOf, Felt, FieldElement, Operation, Process, QuadFelt, StarkField, TWO, TWO_INV,
    };
    use rand_utils::{rand_array, rand_value, rand_vector};
    use vm_core::StackInputs;
    use winter_prover::math::{fft, get_power_series_with_offset};
    use winter_utils::transpose_slice;

    #[test]
    fn fold4() {
        let blowup = 4_usize;

        // generate random alpha
        let alpha: QuadFelt = rand_value();

        // generate degree 7 polynomial f(x)
        let poly: Vec<QuadFelt> = rand_vector(8);

        // evaluate the polynomial over domain of 32 elements
        let offset = Felt::GENERATOR;
        let twiddles = fft::get_twiddles(poly.len());
        let evaluations = fft::evaluate_poly_with_offset(&poly, &twiddles, offset, blowup);

        // fold the evaluations using FRI folding procedure from Winterfell
        let transposed_evaluations = transpose_slice::<QuadFelt, 4>(&evaluations);
        let folded_evaluations =
            winter_fri::folding::apply_drp(&transposed_evaluations, offset, alpha);

        // build the evaluation domain of 32 elements
        let n = poly.len() * blowup;
        let g = Felt::get_root_of_unity(n.trailing_zeros());
        let domain = get_power_series_with_offset(g, offset, n);

        // fold evaluations at a single point using fold4 procedure
        let pos = 3;
        let x = domain[pos];
        let ev = alpha.mul_base(x.inv());
        let (result, _, _) = super::fold4(transposed_evaluations[pos], ev, ev.square());

        // make sure the results of fold4 are the same as results form Winterfell
        assert_eq!(folded_evaluations[pos], result)
    }

    #[test]
    fn constants() {
        let tau = Felt::get_root_of_unity(2);

        assert_eq!(super::TAU_INV, tau.inv());
        assert_eq!(super::TAU2_INV, tau.square().inv());
        assert_eq!(super::TAU3_INV, tau.cube().inv());

        assert_eq!(TWO.inv(), TWO_INV);
    }

    #[test]
    fn op_fri_ext2fold4() {
        // --- build stack inputs ---------------------------------------------
        // we need 17 values because we also assume that the pointer to the last FRI layer will
        // be in the first position of the stack overflow table
        let mut inputs = rand_array::<Felt, 17>();
        inputs[7] = TWO; // domain segment must be < 4

        // assign meaning to these values
        let end_ptr = inputs[0];
        let layer_ptr = inputs[1];
        let alpha = QuadFelt::new(inputs[2], inputs[3]);
        let poe = inputs[6];
        let d_seg = inputs[7];
        let f_pos = inputs[8];
        let query_values = [
            QuadFelt::new(inputs[9], inputs[10]),
            QuadFelt::new(inputs[11], inputs[12]),
            QuadFelt::new(inputs[13], inputs[14]),
            QuadFelt::new(inputs[15], inputs[16]),
        ];

        // println!("{:?}", inputs.iter().map(|v| v.as_int()).collect::<Vec<_>>());

        // --- execute FRIE2F4 operation --------------------------------------
        let stack_inputs = StackInputs::new(inputs.to_vec());
        let mut process = Process::new_dummy_with_decoder_helpers(stack_inputs);
        process.execute_op(Operation::FriE2F4).unwrap();

        // --- check the stack state-------------------------------------------
        let stack_state = process.stack.trace_state();

        // perform layer folding
        let f_tau = super::get_tau_factor(d_seg.as_int());
        let x = poe * f_tau * super::DOMAIN_OFFSET;
        let x_inv = x.inv();

        let (ev, es) = super::compute_evaluation_points(alpha, x_inv);
        let (folded_value, tmp0, tmp1) = super::fold4(query_values, ev, es);

        // check temp values
        let tmp0 = tmp0.to_base_elements();
        let tmp1 = tmp1.to_base_elements();
        assert_eq!(stack_state[0], tmp0[1]);
        assert_eq!(stack_state[1], tmp0[0]);
        assert_eq!(stack_state[2], tmp1[1]);
        assert_eq!(stack_state[3], tmp1[0]);

        // check domain segment flags
        let ds = super::get_domain_segment_flags(d_seg.as_int());
        assert_eq!(stack_state[4], ds[3]);
        assert_eq!(stack_state[5], ds[2]);
        assert_eq!(stack_state[6], ds[1]);
        assert_eq!(stack_state[7], ds[0]);

        // check poe, f_tau, layer_ptr, f_pos
        assert_eq!(stack_state[8], poe.square());
        assert_eq!(stack_state[9], f_tau);
        assert_eq!(stack_state[10], layer_ptr + TWO);
        assert_eq!(stack_state[11], poe.exp(4));
        assert_eq!(stack_state[12], f_pos);

        // check folded value
        let folded_value = folded_value.to_base_elements();
        assert_eq!(stack_state[13], folded_value[1]);
        assert_eq!(stack_state[14], folded_value[0]);

        // check end ptr (should be moved from overflow table)
        assert_eq!(stack_state[15], end_ptr);

        // --- check helper registers -----------------------------------------
        let mut expected_helpers = QuadFelt::as_base_elements(&[ev, es]).to_vec();
        expected_helpers.push(x);
        expected_helpers.push(x_inv);
        assert_eq!(expected_helpers, process.decoder.get_user_op_helpers().to_vec());
    }
}
