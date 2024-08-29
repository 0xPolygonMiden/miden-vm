use alloc::vec::Vec;

use miden_air::{trace::main_trace::MainTrace, RowIndex};

use super::{Felt, FieldElement, OverflowTableRow};
use crate::trace::AuxColumnBuilder;

// AUXILIARY TRACE BUILDER
// ================================================================================================

/// Describes how to construct execution traces of stack-related auxiliary trace segment columns
/// (used in multiset checks).
pub struct AuxTraceBuilder;

impl AuxTraceBuilder {
    /// Builds and returns stack auxiliary trace columns. Currently this consists of a single
    /// column p1 describing states of the stack overflow table.
    pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &MainTrace,
        rand_elements: &[E],
    ) -> Vec<Vec<E>> {
        let p1 = self.build_aux_column(main_trace, rand_elements);
        vec![p1]
    }
}

impl<E: FieldElement<BaseField = Felt>> AuxColumnBuilder<E> for AuxTraceBuilder {
    /// Initializes the overflow stack auxiliary column.
    fn init_responses(&self, _main_trace: &MainTrace, _alphas: &[E]) -> E {
        E::ONE
    }

    /// Removes a row from the stack overflow table.
    fn get_requests_at(&self, main_trace: &MainTrace, alphas: &[E], i: RowIndex) -> E {
        let is_left_shift = main_trace.is_left_shift(i);
        let is_non_empty_overflow = main_trace.is_non_empty_overflow(i);

        if is_left_shift && is_non_empty_overflow {
            let b1 = main_trace.parent_overflow_address(i);
            let s15_prime = main_trace.stack_element(15, i + 1);
            let b1_prime = main_trace.parent_overflow_address(i + 1);

            let row = OverflowTableRow::new(b1, s15_prime, b1_prime);
            row.to_value(alphas)
        } else {
            E::ONE
        }
    }

    /// Adds a row to the stack overflow table.
    fn get_responses_at(&self, main_trace: &MainTrace, alphas: &[E], i: RowIndex) -> E {
        let is_right_shift = main_trace.is_right_shift(i);

        if is_right_shift {
            let k0 = main_trace.clk(i);
            let s15 = main_trace.stack_element(15, i);
            let b1 = main_trace.parent_overflow_address(i);

            let row = OverflowTableRow::new(k0, s15, b1);
            row.to_value(alphas)
        } else {
            E::ONE
        }
    }
}
