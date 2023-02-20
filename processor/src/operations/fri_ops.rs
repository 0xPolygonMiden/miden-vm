use super::{AdviceProvider, ExecutionError, Felt, Operation, Process};
use vm_core::{ExtensionOf, FieldElement, QuadExtension, StarkField, ONE, ZERO};

// CONSTANTS
// ================================================================================================

const TWO: Felt = Felt::new(2);

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
        let alpha = self.get_alpha();
        let poe = self.get_poe();
        let f_pos = self.get_folded_position();
        let d_seg = self.get_domain_segment().as_int();

        // compute x corresponding to query values
        let f_tau = get_tau_factor(d_seg);
        let x = poe * f_tau * DOMAIN_OFFSET;
        let x_inv = x.inv();

        // fold query values
        let (ev, es) = compute_evaluation_points(alpha, x_inv);
        let (folded_value, tmp0, tmp1) = fold4(query_values, ev, es);

        // write the relevant values into the next state of the stack
        self.set_folded_value(folded_value);
        self.set_folded_position(f_pos);
        self.set_next_poe(poe);
        self.set_temp_evaluations(tmp0, tmp1);
        self.set_domain_segment_flags(d_seg);
        self.set_tau_factor(f_tau);
        self.set_helper_registers(ev, es, x, x_inv);
        self.stack.set(14, ZERO);

        self.stack.copy_state(15);
        Ok(())
    }

    // HELPER METHODS - GETTERS
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
    fn get_alpha(&self) -> QuadFelt {
        let a1 = self.stack.get(8);
        let a0 = self.stack.get(9);
        QuadFelt::new(a0, a1)
    }

    /// TODO: add docs
    fn get_poe(&self) -> Felt {
        self.stack.get(10)
    }

    /// TODO: add docs
    fn get_folded_position(&self) -> Felt {
        self.stack.get(11)
    }

    /// TODO: add docs
    fn get_domain_segment(&self) -> Felt {
        self.stack.get(12)
    }

    // HELPER METHODS - SETTERS
    // --------------------------------------------------------------------------------------------

    /// TODO: add docs
    fn set_folded_value(&mut self, value: QuadFelt) {
        let value_arr = [value];
        let elements = QuadFelt::as_base_elements(&value_arr);
        self.stack.set(0, elements[1]);
        self.stack.set(1, elements[0]);
    }

    /// TODO: add docs
    fn set_folded_position(&mut self, pos: Felt) {
        self.stack.set(2, pos);
    }

    /// TODO: add docs
    fn set_next_poe(&mut self, poe: Felt) {
        let poe2 = poe.square();
        self.stack.set(3, poe2);
        self.stack.set(4, poe2.square());
    }

    /// TODO: add docs
    fn set_temp_evaluations(&mut self, tmp0: QuadFelt, tmp1: QuadFelt) {
        let tmp0_arr = [tmp0];
        let tmp0_felts = QuadFelt::as_base_elements(&tmp0_arr);

        let tmp1_arr = [tmp1];
        let tmp1_felts = QuadFelt::as_base_elements(&tmp1_arr);

        self.stack.set(5, tmp0_felts[1]);
        self.stack.set(6, tmp0_felts[0]);
        self.stack.set(7, tmp1_felts[1]);
        self.stack.set(8, tmp1_felts[0]);
    }

    /// TODO: add docs
    fn set_domain_segment_flags(&mut self, d_seg: u64) {
        let ds = match d_seg {
            0 => [ONE, ZERO, ZERO, ZERO],
            1 => [ZERO, ONE, ZERO, ZERO],
            2 => [ZERO, ZERO, ONE, ZERO],
            3 => [ZERO, ZERO, ZERO, ONE],
            _ => panic!("invalid domain segment {d_seg}"),
        };

        self.stack.set(9, ds[3]);
        self.stack.set(10, ds[2]);
        self.stack.set(11, ds[1]);
        self.stack.set(12, ds[0]);
    }

    /// TODO: add docs
    fn set_tau_factor(&mut self, f_tau: Felt) {
        self.stack.set(13, f_tau);
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
    // TODO: put two_inv into a constant
    let two_inv = QuadFelt::from(TWO).inv();
    (f_x + f_neg_x + ((f_x - f_neg_x) * evaluation_point)) * two_inv
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {

    use super::{ExtensionOf, Felt, FieldElement, QuadFelt, StarkField};
    use rand_utils::{rand_value, rand_vector};
    use winter_utils::transpose_slice;
    use winterfell::math::{fft, get_power_series_with_offset};

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
    fn tau_const() {
        let tau = Felt::get_root_of_unity(2);

        assert_eq!(super::TAU_INV, tau.inv());
        assert_eq!(super::TAU2_INV, tau.square().inv());
        assert_eq!(super::TAU3_INV, tau.cube().inv());
    }
}
