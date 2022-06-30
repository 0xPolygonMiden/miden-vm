use super::{super::trace::LookupTableRow, Felt, FieldElement, Vec, Word};

// AUXILIARY TRACE HINTS
// ================================================================================================

/// TODO: add docs
#[derive(Debug, Clone)]
pub struct AuxTraceHints {
    sibling_hints: Vec<(usize, SiblingTableUpdate)>,
    sibling_rows: Vec<SiblingTableRow>,
}

impl AuxTraceHints {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns an empty [AuxTraceHints] struct.
    pub fn new() -> Self {
        Self {
            sibling_hints: Vec::new(),
            sibling_rows: Vec::new(),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    #[cfg(test)]
    pub fn sibling_table_hints(&self) -> &[(usize, SiblingTableUpdate)] {
        &self.sibling_hints
    }

    #[cfg(test)]
    pub fn sibling_table_rows(&self) -> &[SiblingTableRow] {
        &self.sibling_rows
    }

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
