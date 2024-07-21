use alloc::vec::Vec;
use static_assertions::const_assert;
use vm_core::FieldElement;

use crate::trace::{
    chiplets::{MEMORY_D0_COL_IDX, MEMORY_D1_COL_IDX},
    decoder::{DECODER_OP_BITS_OFFSET, DECODER_USER_OP_HELPERS_OFFSET},
    range::{M_COL_IDX, V_COL_IDX},
    CHIPLETS_OFFSET, TRACE_WIDTH,
};

use super::{EqFunction, MultiLinearPoly};

/// A multi-variate polynomial for composing individual multi-linear polynomials.
pub trait CompositionPolynomial<E: FieldElement> {
    /// Maximum degree in all variables.
    fn max_degree(&self) -> u32;

    /// Given a query, of length equal the number of variables, evaluates [Self] at this query.
    fn evaluate(&self, query: &[E]) -> E;
}

/// A composition polynomial used in the GKR protocol for all of its sum-checks except the final
/// one.
#[derive(Clone)]
pub struct GkrComposition<E>
where
    E: FieldElement,
{
    pub combining_randomness: E,
}

impl<E> GkrComposition<E>
where
    E: FieldElement,
{
    pub fn new(combining_randomness: E) -> Self {
        Self {
            combining_randomness,
        }
    }
}

impl<E> CompositionPolynomial<E> for GkrComposition<E>
where
    E: FieldElement,
{
    fn max_degree(&self) -> u32 {
        3
    }

    fn evaluate(&self, query: &[E]) -> E {
        let eval_left_numerator = query[0];
        let eval_right_numerator = query[1];
        let eval_left_denominator = query[2];
        let eval_right_denominator = query[3];
        let eq_eval = query[4];
        eq_eval
            * ((eval_left_numerator * eval_right_denominator
                + eval_right_numerator * eval_left_denominator)
                + eval_left_denominator * eval_right_denominator * self.combining_randomness)
    }
}

/// A composition polynomial used in the GKR protocol for its final sum-check.
#[derive(Clone)]
pub struct GkrCompositionMerge<E>
where
    E: FieldElement,
{
    pub sum_check_combining_randomness: E,
    pub tensored_merge_randomness: Vec<E>,
    pub log_up_randomness: Vec<E>,
}

impl<E> GkrCompositionMerge<E>
where
    E: FieldElement,
{
    pub fn new(
        combining_randomness: E,
        merge_randomness: Vec<E>,
        log_up_randomness: Vec<E>,
    ) -> Self {
        let tensored_merge_randomness =
            EqFunction::ml_at(merge_randomness.clone()).evaluations().to_vec();

        Self {
            sum_check_combining_randomness: combining_randomness,
            tensored_merge_randomness,
            log_up_randomness,
        }
    }
}

impl<E> CompositionPolynomial<E> for GkrCompositionMerge<E>
where
    E: FieldElement,
{
    fn max_degree(&self) -> u32 {
        // Computed as:
        // 1 + max(left_numerator_degree + right_denom_degree, right_numerator_degree +
        // left_denom_degree)
        5
    }

    fn evaluate(&self, query: &[E]) -> E {
        let [numerators, denominators] =
            evaluate_fractions_at_main_trace_query(query, &self.log_up_randomness);

        let numerators = MultiLinearPoly::from_evaluations(numerators.to_vec()).unwrap();
        let denominators = MultiLinearPoly::from_evaluations(denominators.to_vec()).unwrap();

        let (left_numerators, right_numerators) = numerators.project_least_significant_variable();
        let (left_denominators, right_denominators) =
            denominators.project_least_significant_variable();

        let eval_left_numerators =
            left_numerators.evaluate_with_lagrange_kernel(&self.tensored_merge_randomness);
        let eval_right_numerators =
            right_numerators.evaluate_with_lagrange_kernel(&self.tensored_merge_randomness);

        let eval_left_denominators =
            left_denominators.evaluate_with_lagrange_kernel(&self.tensored_merge_randomness);
        let eval_right_denominators =
            right_denominators.evaluate_with_lagrange_kernel(&self.tensored_merge_randomness);

        let eq_eval = query[TRACE_WIDTH];

        eq_eval
            * ((eval_left_numerators * eval_right_denominators
                + eval_right_numerators * eval_left_denominators)
                + eval_left_denominators
                    * eval_right_denominators
                    * self.sum_check_combining_randomness)
    }
}

/// Defines the number of wires in the input layer that are generated from a single main trace row.
pub const NUM_WIRES_PER_TRACE_ROW: usize = 8;
const_assert!(NUM_WIRES_PER_TRACE_ROW.is_power_of_two());

/// Converts a main trace row (or more generally "query") to numerators and denominators of the
/// input layer.
pub fn evaluate_fractions_at_main_trace_query<E>(
    query: &[E],
    log_up_randomness: &[E],
) -> [[E; NUM_WIRES_PER_TRACE_ROW]; 2]
where
    E: FieldElement,
{
    // numerators
    let multiplicity = query[M_COL_IDX];
    let f_m = {
        let mem_selec0 = query[CHIPLETS_OFFSET];
        let mem_selec1 = query[CHIPLETS_OFFSET + 1];
        let mem_selec2 = query[CHIPLETS_OFFSET + 2];
        mem_selec0 * mem_selec1 * (E::ONE - mem_selec2)
    };

    let f_rc = {
        let op_bit_4 = query[DECODER_OP_BITS_OFFSET + 4];
        let op_bit_5 = query[DECODER_OP_BITS_OFFSET + 5];
        let op_bit_6 = query[DECODER_OP_BITS_OFFSET + 6];

        (E::ONE - op_bit_4) * (E::ONE - op_bit_5) * op_bit_6
    };

    // denominators
    let alphas = log_up_randomness;

    let table_denom = alphas[0] - query[V_COL_IDX];
    let memory_denom_0 = -(alphas[0] - query[MEMORY_D0_COL_IDX]);
    let memory_denom_1 = -(alphas[0] - query[MEMORY_D1_COL_IDX]);
    let stack_value_denom_0 = -(alphas[0] - query[DECODER_USER_OP_HELPERS_OFFSET]);
    let stack_value_denom_1 = -(alphas[0] - query[DECODER_USER_OP_HELPERS_OFFSET + 1]);
    let stack_value_denom_2 = -(alphas[0] - query[DECODER_USER_OP_HELPERS_OFFSET + 2]);
    let stack_value_denom_3 = -(alphas[0] - query[DECODER_USER_OP_HELPERS_OFFSET + 3]);

    [
        [multiplicity, f_m, f_m, f_rc, f_rc, f_rc, f_rc, E::ZERO],
        [
            table_denom,
            memory_denom_0,
            memory_denom_1,
            stack_value_denom_0,
            stack_value_denom_1,
            stack_value_denom_2,
            stack_value_denom_3,
            E::ONE,
        ],
    ]
}
