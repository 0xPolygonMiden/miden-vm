use alloc::vec::Vec;

use miden_air::trace::main_trace::MainTrace;
use vm_core::Kernel;

use super::{super::trace::AuxColumnBuilder, Felt, FieldElement};

mod bus;
pub use bus::{BusColumnBuilder,build_ace_memory_read_element_request,build_ace_memory_read_word_request};

mod virtual_table;
pub use virtual_table::ChipletsVTableColBuilder;

/// Constructs the execution trace for chiplets-related auxiliary columns (used in multiset checks).
#[derive(Debug)]
pub struct AuxTraceBuilder {
    kernel: Kernel,
}

impl AuxTraceBuilder {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    pub fn new(kernel: Kernel) -> Self {
        Self { kernel }
    }

    // COLUMN TRACE CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Builds and returns the Chiplets's auxiliary trace columns. Currently this consists of
    /// a single bus column `b_chip` describing chiplet lookups requested by the stack and
    /// provided by chiplets in the Chiplets module.
    pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &MainTrace,
        rand_elements: &[E],
    ) -> Vec<Vec<E>> {
        let v_table_col_builder = ChipletsVTableColBuilder::new(self.kernel.clone());
        let bus_col_builder = BusColumnBuilder::default();
        let t_chip = v_table_col_builder.build_aux_column(main_trace, rand_elements);
        let b_chip = bus_col_builder.build_aux_column(main_trace, rand_elements);

        let v_table_final_value = t_chip.last().unwrap();
        let chiplets_bus_final_value = b_chip.last().unwrap();
        debug_assert_eq!(*v_table_final_value * *chiplets_bus_final_value, E::ONE);
        vec![t_chip, b_chip]
    }
}
