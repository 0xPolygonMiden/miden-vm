use super::{
    trace::{build_lookup_table_row_values, AuxColumnBuilder, LookupTableRow},
    BTreeMap, ColMatrix, Felt, FieldElement, StarkField, Vec, Word,
};

mod bus;
pub(crate) use bus::{ChipletLookup, ChipletsBus, ChipletsBusRow};

mod virtual_table;
pub(crate) use virtual_table::{ChipletsVTableRow, ChipletsVTableUpdate};

/// Contains all relevant information and describes how to construct the execution trace for
/// chiplets-related auxiliary columns (used in multiset checks).
pub struct AuxTraceBuilder {
    bus_builder: BusTraceBuilder,
    table_builder: ChipletsVTableTraceBuilder,
}

impl AuxTraceBuilder {
    pub fn new(bus_builder: BusTraceBuilder, table_builder: ChipletsVTableTraceBuilder) -> Self {
        Self {
            bus_builder,
            table_builder,
        }
    }

    // COLUMN TRACE CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Builds and returns the Chiplets's auxiliary trace columns. Currently this consists of
    /// a single bus column `b_chip` describing chiplet lookups requested by the stack and
    /// provided by chiplets in the Chiplets module.
    pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &ColMatrix<Felt>,
        rand_elements: &[E],
    ) -> Vec<Vec<E>> {
        let t_chip = self.table_builder.build_aux_column(main_trace, rand_elements);
        let b_chip = self.bus_builder.build_aux_column(main_trace, rand_elements);
        vec![t_chip, b_chip]
    }
}

// BUS TRACE BUILDER
// ================================================================================================

/// Describes how to construct the execution trace of the chiplets bus auxiliary trace column.
pub struct BusTraceBuilder {
    pub(super) lookup_hints: Vec<(u32, ChipletsBusRow)>,
    pub(super) requests: Vec<ChipletLookup>,
    pub(super) responses: Vec<ChipletLookup>,
}

impl BusTraceBuilder {
    pub(crate) fn new(
        lookup_hints: Vec<(u32, ChipletsBusRow)>,
        requests: Vec<ChipletLookup>,
        responses: Vec<ChipletLookup>,
    ) -> Self {
        Self {
            lookup_hints,
            requests,
            responses,
        }
    }
}

impl AuxColumnBuilder<ChipletsBusRow, ChipletLookup, u32> for BusTraceBuilder {
    /// This method is required, but because it is only called inside `build_row_values` which is
    /// overridden below, it is not used here and should not be called.
    fn get_table_rows(&self) -> &[ChipletLookup] {
        unimplemented!()
    }

    /// Returns hints which describe the [Chiplets] lookup requests and responses during program
    /// execution. Each update hint is accompanied by a clock cycle at which the update happened.
    ///
    /// Internally, each update hint also contains an index of the row into the full list of request
    /// rows or response rows, depending on whether it is a request, a response, or both (in which
    /// case it contains 2 indices).
    fn get_table_hints(&self) -> &[(u32, ChipletsBusRow)] {
        &self.lookup_hints
    }

    /// Returns the value by which the running product column should be multiplied for the provided
    /// hint value.
    fn get_multiplicand<E: FieldElement<BaseField = Felt>>(
        &self,
        hint: ChipletsBusRow,
        row_values: &[E],
        inv_row_values: &[E],
    ) -> E {
        let mut mult = if let Some(response_idx) = hint.response() {
            row_values[response_idx as usize]
        } else {
            E::ONE
        };

        for request_idx in hint.requests() {
            mult *= inv_row_values[*request_idx as usize];
        }

        mult
    }

    /// Build the row values and inverse values used to build the auxiliary column.
    ///
    /// The row values to be included come from the responses and the inverse values come from
    /// requests. Since responses are grouped by chiplet, the operation order for the requests and
    /// responses will be permutations of each other rather than sharing the same order. Therefore,
    /// the `row_values` and `inv_row_values` must be built separately.
    fn build_row_values<E>(&self, main_trace: &ColMatrix<Felt>, alphas: &[E]) -> (Vec<E>, Vec<E>)
    where
        E: FieldElement<BaseField = Felt>,
    {
        // get the row values from the resonse rows
        let row_values = self
            .responses
            .iter()
            .map(|response| response.to_value(main_trace, alphas))
            .collect();
        // get the inverse values from the request rows
        let (_, inv_row_values) = build_lookup_table_row_values(&self.requests, main_trace, alphas);

        (row_values, inv_row_values)
    }
}

// VIRTUAL TABLE TRACE BUILDER
// ================================================================================================

/// Describes how to construct the execution trace of the chiplets virtual table, used to manage
/// internal updates and data required by the chiplets.
///
/// This manages construction of a single column which first represents the state of the sibling
/// table (used in Merkle root update computation), and then is subsequently used to represent the
/// procedures contained in the kernel ROM. Thus, it is expected that the initial value is ONE, the
/// value after all sibling table updates are completed is again ONE, and the value at the end of
/// the trace is the product of the representations of the kernel ROM procedures.
#[derive(Debug, Clone, Default)]
pub struct ChipletsVTableTraceBuilder {
    pub(super) hints: Vec<(u32, ChipletsVTableUpdate)>,
    pub(super) rows: Vec<ChipletsVTableRow>,
}

impl ChipletsVTableTraceBuilder {
    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Specifies that an entry for the provided sibling was added to the chiplets virtual table at
    /// the specified step.
    ///
    /// It is assumed that the table is empty or contains only sibling entries at this point and has
    /// not been used for any other chiplet updates.
    pub fn sibling_added(&mut self, step: u32, index: Felt, sibling: Word) {
        let row_index = self.rows.len();
        let update = ChipletsVTableUpdate::SiblingAdded(row_index as u32);
        self.hints.push((step, update));
        self.rows.push(ChipletsVTableRow::new_sibling(index, sibling));
    }

    /// Specifies that an entry for a sibling was removed from the chiplets virtual table. The entry
    /// is defined by the provided offset. For example, if row_offset = 2, the second from the last
    /// entry was removed from the table.
    ///
    /// It is assumed that the table contains only sibling entries at this point and has not been
    /// used for any other chiplet updates.
    pub fn sibling_removed(&mut self, step: u32, row_offset: usize) {
        let row_index = self.rows.len() - row_offset - 1;
        let update = ChipletsVTableUpdate::SiblingRemoved(row_index as u32);
        self.hints.push((step, update));
    }

    /// Specifies a kernel procedure that must be added to the virtual table.
    ///
    /// It is assumed that kernel procedures will only be added after all sibling updates have been
    /// completed.
    pub fn add_kernel_proc(&mut self, step: u32, addr: Felt, proc_hash: Word) {
        let proc_index = self.rows.len();
        let update = ChipletsVTableUpdate::KernelProcAdded(proc_index as u32);
        self.hints.push((step, update));
        self.rows.push(ChipletsVTableRow::new_kernel_proc(addr, proc_hash));
    }

    // TEST HELPERS
    // --------------------------------------------------------------------------------------------
    #[cfg(test)]
    pub fn hints(&self) -> &[(u32, ChipletsVTableUpdate)] {
        &self.hints
    }

    #[cfg(test)]
    pub fn rows(&self) -> &[ChipletsVTableRow] {
        &self.rows
    }
}

impl AuxColumnBuilder<ChipletsVTableUpdate, ChipletsVTableRow, u32> for ChipletsVTableTraceBuilder {
    /// Returns a list of rows which were added to and then removed from the chiplets virtual table.
    ///
    /// The order of the rows in the list is the same as the order in which the rows were added to
    /// the table.
    fn get_table_rows(&self) -> &[ChipletsVTableRow] {
        &self.rows
    }

    /// Returns hints which describe how the chiplets virtual table was updated during program
    /// execution. Each update hint is accompanied by a clock cycle at which the update happened.
    ///
    /// Internally, each update hint also contains an index of the row into the full list of rows
    /// which was either added or removed.
    fn get_table_hints(&self) -> &[(u32, ChipletsVTableUpdate)] {
        &self.hints
    }

    /// Returns the value by which the running product column should be multiplied for the provided
    /// hint value.
    fn get_multiplicand<E: FieldElement<BaseField = Felt>>(
        &self,
        hint: ChipletsVTableUpdate,
        row_values: &[E],
        inv_row_values: &[E],
    ) -> E {
        match hint {
            ChipletsVTableUpdate::SiblingAdded(inserted_row_idx) => {
                row_values[inserted_row_idx as usize]
            }
            ChipletsVTableUpdate::SiblingRemoved(removed_row_idx) => {
                inv_row_values[removed_row_idx as usize]
            }
            ChipletsVTableUpdate::KernelProcAdded(idx) => row_values[idx as usize],
        }
    }

    /// Returns the final value in the auxiliary column. Default implementation of this method
    /// returns ONE.
    fn final_column_value<E: FieldElement<BaseField = Felt>>(&self, row_values: &[E]) -> E {
        let mut result = E::ONE;
        for (_, table_update) in self.hints.iter() {
            if let ChipletsVTableUpdate::KernelProcAdded(idx) = table_update {
                result *= row_values[*idx as usize];
            }
        }

        result
    }
}
