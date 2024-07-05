use alloc::vec::Vec;
use vm_core::{Felt, FieldElement};
use winter_air::{Assertion, LagrangeKernelRandElements, TransitionConstraintDegree};

use crate::{gkr_proof::inner_product, trace::logup::S_COL_IDX, TRACE_WIDTH};

/// The number of auxiliary assertions.
pub const NUM_ASSERTIONS: usize = TRACE_WIDTH;

/// The number of auxiliary assertions.
pub const NUM_AUX_ASSERTIONS: usize = 2;

pub fn get_assertions_first_step(result: &mut Vec<Assertion<Felt>>, main_trace_first_row: &[Felt]) {
    for col_idx in 0..TRACE_WIDTH {
        result.push(Assertion::single(col_idx, 0, main_trace_first_row[col_idx]))
    }
}

pub fn get_aux_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    todo!()
}

pub fn get_aux_assertions_first_step<E>(
    result: &mut Vec<Assertion<E>>,
    lagrange_kernel_rand_elements: &LagrangeKernelRandElements<E>,
    main_trace_first_row: &[Felt],
    openings_combining_randomness: &[E],
) where
    E: FieldElement<BaseField = Felt>,
{
    // Lagrange kernel column value at row 0
    let eq_fn_at_0: E = lagrange_kernel_rand_elements
        .iter()
        .map(|r| E::ONE - *r)
        .fold(E::ONE, |acc, ele| acc * ele);

    let main_trace_first_row: Vec<E> =
        main_trace_first_row.iter().map(|&ele| E::from(ele)).collect();
    let assertion_value =
        eq_fn_at_0 * inner_product(&main_trace_first_row, openings_combining_randomness);

    result.push(Assertion::single(S_COL_IDX, 0, assertion_value));
}

pub fn get_aux_assertions_last_step<E>(
    result: &mut Vec<Assertion<E>>,
    openings_combining_randomness: &[E],
    openings: &[E],
    step: usize,
) where
    E: FieldElement<BaseField = Felt>,
{
    let value = inner_product(openings_combining_randomness, openings);

    result.push(Assertion::single(S_COL_IDX, step, value));
}
