use super::{
    super::trace::{AuxColumnBuilder, LookupTableRow},
    Felt, FieldElement, Vec, Word,
};

// AUXILIARY TRACE HINTS
// ================================================================================================

/// TODO: add docs
#[derive(Debug, Clone, Default)]
pub struct AuxTraceHints {
    pub(super) sibling_hints: Vec<(usize, SiblingTableUpdate)>,
    pub(super) sibling_rows: Vec<SiblingTableRow>,
}

impl AuxTraceHints {
    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    pub fn sibling_added(&mut self, step: usize, index: Felt, sibling: Word) {
        let row_index = self.sibling_rows.len();
        let update = SiblingTableUpdate::SiblingAdded(row_index as u32);
        self.sibling_hints.push((step, update));
        self.sibling_rows.push(SiblingTableRow::new(index, sibling));
    }

    pub fn sibling_removed(&mut self, step: usize, row_offset: usize) {
        let row_index = self.sibling_rows.len() - row_offset - 1;
        let update = SiblingTableUpdate::SiblingRemoved(row_index as u32);
        self.sibling_hints.push((step, update));
    }
}

impl AuxColumnBuilder<SiblingTableUpdate, SiblingTableRow> for AuxTraceHints {
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
    fn get_table_hints(&self) -> &[(usize, SiblingTableUpdate)] {
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

// SIBLING TABLE
// ================================================================================================

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SiblingTableUpdate {
    SiblingAdded(u32),
    SiblingRemoved(u32),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SiblingTableRow {
    index: Felt,
    sibling: Word,
}

impl SiblingTableRow {
    pub fn new(index: Felt, sibling: Word) -> Self {
        Self { index, sibling }
    }
}

impl LookupTableRow for SiblingTableRow {
    /// Reduces this row to a single field element in the field specified by E. This requires
    /// at least 6 alpha values.
    fn to_value<E: FieldElement<BaseField = Felt>>(&self, alphas: &[E]) -> E {
        alphas[0]
            + alphas[1].mul_base(self.index) // TODO: change to alpha[3]
            + alphas[2].mul_base(self.sibling[0]) // TODO: change to alpha[8]
            + alphas[3].mul_base(self.sibling[1]) // TODO: change to alpha[9]
            + alphas[4].mul_base(self.sibling[2]) // TODO: change to alpha[10]
            + alphas[5].mul_base(self.sibling[3]) // TODO: change to alpha[11]
    }
}
