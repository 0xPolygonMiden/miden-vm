use alloc::vec::Vec;

use miden_air::{trace::main_trace::MainTrace, RowIndex};
use vm_core::OPCODE_DYNCALL;

use super::{Felt, FieldElement, OverflowTableRow};
use crate::trace::AuxColumnBuilder;

// AUXILIARY TRACE BUILDER
// ================================================================================================

/// Describes how to construct execution traces of stack-related auxiliary trace segment columns
/// (used in multiset checks).
pub struct AuxTraceBuilder<E> where E: FieldElement<BaseField = Felt> {
    // Not implemented yet
    #[cfg(any(test, feature = "bus-debugger"))]
    bus_debugger: core::cell::RefCell<crate::debug::BusDebugger<E>>,
    #[cfg(not(any(test, feature = "bus-debugger")))]
    bus_debugger: core::marker::PhantomData<E>,

}

impl<E> AuxTraceBuilder<E> where E: FieldElement<BaseField = Felt> {
    /// Builds and returns stack auxiliary trace columns. Currently this consists of a single
    /// column p1 describing states of the stack overflow table.
    pub fn build_aux_columns(
        &self,
        main_trace: &MainTrace,
        rand_elements: &[E],
    ) -> Vec<Vec<E>> {
        let p1 = self.build_aux_column(main_trace, rand_elements);

        debug_assert_eq!(*p1.last().unwrap(), E::ONE);
        vec![p1]
    }
}

impl<E: FieldElement<BaseField = Felt>> AuxColumnBuilder<E> for AuxTraceBuilder<E> {
    /// Removes a row from the stack overflow table.
    fn get_requests_at(
        &self,
        main_trace: &MainTrace,
        alphas: &[E],
        i: RowIndex,
    ) -> E {
        let is_left_shift = main_trace.is_left_shift(i);
        let is_dyncall = main_trace.get_op_code(i) == OPCODE_DYNCALL.into();
        let is_non_empty_overflow = main_trace.is_non_empty_overflow(i);

        if is_left_shift && is_non_empty_overflow {
            let b1 = main_trace.parent_overflow_address(i);
            let s15_prime = main_trace.stack_element(15, i + 1);
            let b1_prime = main_trace.parent_overflow_address(i + 1);

            OverflowTableRow::new(b1, s15_prime, b1_prime).to_value(alphas)
        } else if is_dyncall && is_non_empty_overflow {
            let b1 = main_trace.parent_overflow_address(i);
            let s15_prime = main_trace.stack_element(15, i + 1);
            let b1_prime = main_trace.decoder_hasher_state_element(5, i);

            OverflowTableRow::new(b1, s15_prime, b1_prime).to_value(alphas)
        } else {
            E::ONE
        }
    }

    /// Adds a row to the stack overflow table.
    fn get_responses_at(
        &self,
        main_trace: &MainTrace,
        alphas: &[E],
        i: RowIndex,
    ) -> E {
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
    
    #[cfg(any(test, feature = "bus-debugger"))]
    fn bus_debugger(&self) -> &crate::debug::BusDebugger<E> {
        &self.bus_debugger.borrow()
    }
}
