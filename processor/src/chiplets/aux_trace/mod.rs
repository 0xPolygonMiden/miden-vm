use super::{
    trace::{build_lookup_table_row_values, AuxColumnBuilder, LookupTableRow},
    BTreeMap, ColMatrix, Felt, FieldElement, StarkField, Vec, Word,
};

mod bus;
pub(crate) use bus::{ChipletLookup, ChipletsBus, ChipletsBusRow};

mod virtual_table;
pub(crate) use virtual_table::{SiblingTableRow, SiblingTableUpdate};

/// Contains all relevant information and describes how to construct the execution trace for
/// chiplets-related auxiliary columns (used in multiset checks).
pub struct AuxTraceBuilder {
    bus_builder: BusTraceBuilder,
    table_builder: TableTraceBuilder,
}

impl AuxTraceBuilder {
    pub fn new(bus_builder: BusTraceBuilder, table_builder: TableTraceBuilder) -> Self {
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

/// Contains all relevant information and describes how to construct execution trace of hasher-
/// related auxiliary trace columns (used in multiset checks).
///
/// Currently, this manages construction of a single column representing the state of the sibling
/// table (used in Merkle root update computation).
#[derive(Debug, Clone, Default)]
pub struct TableTraceBuilder {
    pub(super) sibling_hints: Vec<(u32, SiblingTableUpdate)>,
    pub(super) sibling_rows: Vec<SiblingTableRow>,
}

impl TableTraceBuilder {
    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Specifies that an entry for the provided sibling was added to the sibling table at the
    /// specified step.
    pub fn sibling_added(&mut self, step: u32, index: Felt, sibling: Word) {
        let row_index = self.sibling_rows.len();
        let update = SiblingTableUpdate::SiblingAdded(row_index as u32);
        self.sibling_hints.push((step, update));
        self.sibling_rows.push(SiblingTableRow::new(index, sibling));
    }

    /// Specifies that an entry for a sibling was removed from the sibling table. The entry is
    /// defined by the provided offset. For example, if row_offset = 2, the second from the last
    /// entry was removed from the table.
    pub fn sibling_removed(&mut self, step: u32, row_offset: usize) {
        let row_index = self.sibling_rows.len() - row_offset - 1;
        let update = SiblingTableUpdate::SiblingRemoved(row_index as u32);
        self.sibling_hints.push((step, update));
    }

    // TEST HELPERS
    // --------------------------------------------------------------------------------------------
    #[cfg(test)]
    pub fn sibling_hints(&self) -> &[(u32, SiblingTableUpdate)] {
        &self.sibling_hints
    }

    #[cfg(test)]
    pub fn sibling_rows(&self) -> &[SiblingTableRow] {
        &self.sibling_rows
    }
}

impl AuxColumnBuilder<SiblingTableUpdate, SiblingTableRow, u32> for TableTraceBuilder {
    /// Returns a list of rows which were added to and then removed from the sibling table.
    ///
    /// The order of the rows in the list is the same as the order in which the rows were added to
    /// the table.
    fn get_table_rows(&self) -> &[SiblingTableRow] {
        &self.sibling_rows
    }

    /// Returns hints which describe how the sibling table was updated during program execution.
    /// Each update hint is accompanied by a clock cycle at which the update happened.
    ///
    /// Internally, each update hint also contains an index of the row into the full list of rows
    /// which was either added or removed.
    fn get_table_hints(&self) -> &[(u32, SiblingTableUpdate)] {
        &self.sibling_hints
    }

    /// Returns the value by which the running product column should be multiplied for the provided
    /// hint value.
    fn get_multiplicand<E: FieldElement<BaseField = Felt>>(
        &self,
        hint: SiblingTableUpdate,
        row_values: &[E],
        inv_row_values: &[E],
    ) -> E {
        match hint {
            SiblingTableUpdate::SiblingAdded(inserted_row_idx) => {
                row_values[inserted_row_idx as usize]
            }
            SiblingTableUpdate::SiblingRemoved(removed_row_idx) => {
                inv_row_values[removed_row_idx as usize]
            }
        }
    }
}
