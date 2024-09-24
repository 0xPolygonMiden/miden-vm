use alloc::vec::Vec;

use miden_air::trace::main_trace::MainTrace;
use vm_core::FieldElement;

use super::{Felt, ONE, ZERO};
use crate::trace::AuxColumnBuilder;

mod block_stack_table;
use block_stack_table::BlockStackColumnBuilder;

// AUXILIARY TRACE BUILDER
// ================================================================================================

/// Constructs the execution traces of stack-related auxiliary trace segment columns
/// (used in multiset checks).
#[derive(Default, Clone, Copy)]
pub struct AuxTraceBuilder {}

impl AuxTraceBuilder {
    /// Builds and returns decoder auxiliary trace columns p1, p2, and p3 describing states of block
    /// stack, block hash, and op group tables respectively.
    pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &MainTrace,
        rand_elements: &[E],
    ) -> Vec<Vec<E>> {
        let block_stack_column_builder = BlockStackColumnBuilder::default();
        let p1 = block_stack_column_builder.build_aux_column(main_trace, rand_elements);

        debug_assert_eq!(*p1.last().unwrap(), E::ONE);
        vec![p1]
    }
}
