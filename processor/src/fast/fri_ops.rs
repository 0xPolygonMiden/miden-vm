// CONSTANTS
// ================================================================================================

use vm_core::{ExtensionOf, Felt, FieldElement, ONE, StarkField, ZERO};

use super::FastProcessor;
use crate::{ExecutionError, QuadFelt};

const EIGHT: Felt = Felt::new(8);
const TWO_INV: Felt = Felt::new(9223372034707292161);

const DOMAIN_OFFSET: Felt = Felt::GENERATOR;

// Pre-computed powers of 1/tau, where tau is the generator of multiplicative subgroup of size 4
// (i.e., tau is the 4th root of unity). Correctness of these constants is checked in the test at
// the end of this module.
const TAU_INV: Felt = Felt::new(18446462594437873665); // tau^{-1}
const TAU2_INV: Felt = Felt::new(18446744069414584320); // tau^{-2}
const TAU3_INV: Felt = Felt::new(281474976710656); // tau^{-3}

impl FastProcessor {
    /// Analogous to `Process::op_fri_ext2fold4`.
    pub fn op_fri_ext2fold4(&mut self) -> Result<(), ExecutionError> {
        // --- read all relevant variables from the stack ---------------------
        let query_values = self.get_query_values();
        let folded_pos = self.stack_get(8);
        // the segment identifier of the position in the source domain
        let domain_segment = self.stack_get(9).as_int();
        // the power of the domain generator which can be used to determine current domain value x
        let poe = self.stack_get(10);
        // the result of the previous layer folding
        let prev_value = {
            let pe1 = self.stack_get(11);
            let pe0 = self.stack_get(12);
            QuadFelt::new(pe0, pe1)
        };
        // the verifier challenge for the current layer
        let alpha = {
            let a1 = self.stack_get(13);
            let a0 = self.stack_get(14);
            QuadFelt::new(a0, a1)
        };
        // the memory address of the current layer
        let layer_ptr = self.stack_get(15);

        // --- make sure the previous folding was done correctly --------------
        if domain_segment > 3 {
            return Err(ExecutionError::InvalidFriDomainSegment(domain_segment));
        }

        let d_seg = domain_segment as usize;
        if query_values[d_seg] != prev_value {
            return Err(ExecutionError::InvalidFriLayerFolding(prev_value, query_values[d_seg]));
        }

        // --- fold query values ----------------------------------------------
        let f_tau = get_tau_factor(d_seg);
        let x = poe * f_tau * DOMAIN_OFFSET;
        let x_inv = x.inv();

        let (ev, es) = compute_evaluation_points(alpha, x_inv);
        let (folded_value, tmp0, tmp1) = fold4(query_values, ev, es);

        // --- write the relevant values into the next state of the stack -----
        let tmp0 = tmp0.to_base_elements();
        let tmp1 = tmp1.to_base_elements();
        let ds = get_domain_segment_flags(d_seg);
        let folded_value = folded_value.to_base_elements();

        let poe2 = poe.square();
        let poe4 = poe2.square();

        self.decrement_stack_size();

        self.stack_write(0, tmp0[1]);
        self.stack_write(1, tmp0[0]);
        self.stack_write(2, tmp1[1]);
        self.stack_write(3, tmp1[0]);
        self.stack_write_word(4, &ds.into());
        self.stack_write(8, poe2);
        self.stack_write(9, f_tau);
        self.stack_write(10, layer_ptr + EIGHT);
        self.stack_write(11, poe4);
        self.stack_write(12, folded_pos);
        self.stack_write(13, folded_value[1]);
        self.stack_write(14, folded_value[0]);

        Ok(())
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Returns 4 query values in the source domain. These values are to be folded into a single
    /// value in the folded domain.
    fn get_query_values(&self) -> [QuadFelt; 4] {
        let [v4, v5, v6, v7] = self.stack_get_word(0).into();
        let [v0, v1, v2, v3] = self.stack_get_word(4).into();

        [
            QuadFelt::new(v0, v1),
            QuadFelt::new(v2, v3),
            QuadFelt::new(v4, v5),
            QuadFelt::new(v6, v7),
        ]
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Determines tau factor (needed to compute x value) for the specified domain segment.
fn get_tau_factor(domain_segment: usize) -> Felt {
    match domain_segment {
        0 => ONE,
        1 => TAU_INV,
        2 => TAU2_INV,
        3 => TAU3_INV,
        _ => panic!("invalid domain segment {domain_segment}"),
    }
}

/// Determines a set of binary flags needed to describe the specified domain segment.
fn get_domain_segment_flags(domain_segment: usize) -> [Felt; 4] {
    match domain_segment {
        0 => [ONE, ZERO, ZERO, ZERO],
        1 => [ZERO, ONE, ZERO, ZERO],
        2 => [ZERO, ZERO, ONE, ZERO],
        3 => [ZERO, ZERO, ZERO, ONE],
        _ => panic!("invalid domain segment {domain_segment}"),
    }
}

/// Computes 2 evaluation points needed for [fold4] function.
fn compute_evaluation_points(alpha: QuadFelt, x_inv: Felt) -> (QuadFelt, QuadFelt) {
    let ev = alpha.mul_base(x_inv);
    let es = ev.square();
    (ev, es)
}

/// Performs folding by a factor of 4. ev and es are values computed based on x and
/// verifier challenge alpha as follows:
/// - ev = alpha / x
/// - es = (alpha / x)^2
fn fold4(values: [QuadFelt; 4], ev: QuadFelt, es: QuadFelt) -> (QuadFelt, QuadFelt, QuadFelt) {
    let tmp0 = fold2(values[0], values[2], ev);
    let tmp1 = fold2(values[1], values[3], ev.mul_base(TAU_INV));
    let folded_value = fold2(tmp0, tmp1, es);
    (folded_value, tmp0, tmp1)
}

/// Performs folding by a factor of 2. ep is a value computed based on x and verifier challenge
/// alpha.
fn fold2(f_x: QuadFelt, f_neg_x: QuadFelt, ep: QuadFelt) -> QuadFelt {
    (f_x + f_neg_x + ((f_x - f_neg_x) * ep)).mul_base(TWO_INV)
}
