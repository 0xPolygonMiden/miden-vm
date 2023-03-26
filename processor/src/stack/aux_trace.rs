use super::{
    super::trace::AuxColumnBuilder, ColMatrix, Felt, FieldElement, OverflowTableRow,
    OverflowTableUpdate, Vec,
};

// AUXILIARY TRACE BUILDER
// ================================================================================================

/// Describes how to construct execution traces of stack-related auxiliary trace segment columns
/// (used in multiset checks).
pub struct AuxTraceBuilder {
    /// A list of updates made to the overflow table during program execution. For each update we
    /// also track the cycle at which the update happened.
    pub(super) overflow_hints: Vec<(u64, OverflowTableUpdate)>,
    /// A list of all rows that were added to and then removed from the overflow table.
    pub(super) overflow_table_rows: Vec<OverflowTableRow>,
    /// The number of rows in the overflow table when execution begins.
    pub(super) num_init_rows: usize,
    /// A list of indices into the `all_rows` vector which describes the rows remaining in the
    /// overflow table at the end of execution.
    pub(super) final_rows: Vec<usize>,
}

impl AuxTraceBuilder {
    /// Builds and returns stack auxiliary trace columns. Currently this consists of a single
    /// column p1 describing states of the stack overflow table.
    pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &ColMatrix<Felt>,
        rand_elements: &[E],
    ) -> Vec<Vec<E>> {
        let p1 = self.build_aux_column(main_trace, rand_elements);
        vec![p1]
    }
}

// OVERFLOW TABLE
// ================================================================================================

impl AuxColumnBuilder<OverflowTableUpdate, OverflowTableRow, u64> for AuxTraceBuilder {
    /// Returns a list of rows which were added to and then removed from the stack overflow table.
    ///
    /// The order of the rows in the list is the same as the order in which the rows were added to
    /// the table.
    fn get_table_rows(&self) -> &[OverflowTableRow] {
        &self.overflow_table_rows
    }

    /// Returns hints which describe how the stack overflow table was updated during program
    /// execution. Each update hint is accompanied by a clock cycle at which the update happened.
    ///
    /// Internally, each update hint also contains an index of the row into the full list of rows
    /// which was either added or removed.
    fn get_table_hints(&self) -> &[(u64, OverflowTableUpdate)] {
        &self.overflow_hints[self.num_init_rows..]
    }

    /// Returns the value by which the running product column should be multiplied for the provided
    /// hint value.
    fn get_multiplicand<E: FieldElement<BaseField = Felt>>(
        &self,
        hint: OverflowTableUpdate,
        row_values: &[E],
        inv_row_values: &[E],
    ) -> E {
        match hint {
            OverflowTableUpdate::RowInserted(inserted_row_idx) => {
                row_values[inserted_row_idx as usize]
            }
            OverflowTableUpdate::RowRemoved(removed_row_idx) => {
                inv_row_values[removed_row_idx as usize]
            }
        }
    }

    /// Returns the initial value in the auxiliary column.
    fn init_column_value<E: FieldElement<BaseField = Felt>>(&self, row_values: &[E]) -> E {
        let mut init_column_value = E::ONE;
        // iterate through the elements in the initial table
        for (_, hint) in &self.overflow_hints[..self.num_init_rows] {
            // no rows should have been removed from the table before execution begins.
            if let OverflowTableUpdate::RowInserted(row) = hint {
                init_column_value *= row_values[*row as usize];
            } else {
                debug_assert!(
                    false,
                    "overflow table row incorrectly removed before execution started"
                )
            }
        }

        init_column_value
    }

    /// Returns the final value in the auxiliary column.
    fn final_column_value<E: FieldElement<BaseField = Felt>>(&self, row_values: &[E]) -> E {
        let mut final_column_value = E::ONE;
        for &row in &self.final_rows {
            final_column_value *= row_values[row];
        }

        final_column_value
    }
}
