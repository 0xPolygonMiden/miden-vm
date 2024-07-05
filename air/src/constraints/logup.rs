use alloc::vec::Vec;
use vm_core::{Felt, FieldElement};
use winter_air::{Assertion, TransitionConstraintDegree};

use crate::{gkr_proof::inner_product, trace::logup::S_COL_IDX};

/// The number of auxiliary assertions.
pub const NUM_AUX_ASSERTIONS: usize = 2;

pub fn get_assertions_first_step(
    _result: &mut Vec<Assertion<Felt>>,
    _main_trace_first_row: &[Felt],
) {
    // TODOP: Add 70 assertions that public input `first_main_trace_row` is correct
    todo!()
}

pub fn get_aux_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    todo!()
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
