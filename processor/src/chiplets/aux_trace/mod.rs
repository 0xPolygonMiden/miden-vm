use alloc::vec::Vec;

use miden_air::trace::main_trace::MainTrace;
use vm_core::Kernel;
use wiring_bus::WiringBusBuilder;

use super::{super::trace::AuxColumnBuilder, Felt, FieldElement, ace::AceHints};

mod bus;
pub use bus::{
    BusColumnBuilder, build_ace_memory_read_element_request, build_ace_memory_read_word_request,
};

mod virtual_table;
pub use virtual_table::ChipletsVTableColBuilder;

mod wiring_bus;

/// Constructs the execution trace for chiplets-related auxiliary columns (used in multiset checks).
#[derive(Debug)]
pub struct AuxTraceBuilder {
    kernel: Kernel,
    ace_hints: AceHints,
}

impl AuxTraceBuilder {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    pub fn new(kernel: Kernel, ace_hints: AceHints) -> Self {
        Self { kernel, ace_hints }
    }

    // COLUMN TRACE CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Builds and returns the Chiplets's auxiliary trace columns. This consists of:
    ///
    /// 1. a bus column `b_chip` describing requests made by the stack and decoder and resposes
    ///    received from the chiplets in the Chiplets module,
    /// 2. a column acting as both virtual tables, one for the sibling table used by the hasher
    ///    chiplet and the other for the kernel procedure table, and as a bus between the memory
    ///    chiplet and the ACE chiplet.
    /// 3. a column used as bus to wire the gates of the ACE chiplet.
    pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &MainTrace,
        rand_elements: &[E],
    ) -> Vec<Vec<E>> {
        let v_table_col_builder = ChipletsVTableColBuilder::new(self.kernel.clone());
        let bus_col_builder = BusColumnBuilder::default();
        let wiring_bus_builder = WiringBusBuilder::new(&self.ace_hints);
        let t_chip = v_table_col_builder.build_aux_column(main_trace, rand_elements);
        let b_chip = bus_col_builder.build_aux_column(main_trace, rand_elements);
        let wiring_bus = wiring_bus_builder.build_aux_column(main_trace, rand_elements);

        let v_table_final_value = t_chip.last().unwrap();
        let chiplets_bus_final_value = b_chip.last().unwrap();
        debug_assert_eq!(*v_table_final_value * *chiplets_bus_final_value, E::ONE);

        vec![t_chip, b_chip, wiring_bus]
    }
}
