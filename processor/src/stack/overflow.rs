use super::{super::trace::LookupTableRow, BTreeMap, Felt, FieldElement, Vec, ZERO};

// OVERFLOW TABLE
// ================================================================================================

/// Stores the values which beyond the top 16 elements of the stack.
///
/// For each overflow item we also track the clock cycle at which it was inserted into the overflow
/// table.
///
/// When `trace_enabled` is set to true, we also record all changes to the table so that we can
/// reconstruct the overflow table at any clock cycle. This can be used for debugging purposes.
pub struct OverflowTable {
    all_rows: Vec<OverflowTableRow>,
    active_rows: Vec<usize>,
    update_trace: Vec<(usize, OverflowTableUpdate)>,
    trace: BTreeMap<usize, Vec<Felt>>,
    trace_enabled: bool,
}

impl OverflowTable {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [OverflowTable]. The returned table is empty.
    pub fn new(enable_trace: bool) -> Self {
        Self {
            all_rows: Vec::new(),
            active_rows: Vec::new(),
            update_trace: Vec::new(),
            trace: BTreeMap::new(),
            trace_enabled: enable_trace,
        }
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Pushes the specified value into the overflow table.
    pub fn push(&mut self, value: Felt, clk: usize) {
        let prev = self
            .active_rows
            .last()
            .map_or(ZERO, |&last_row_idx| self.all_rows[last_row_idx].clk);

        self.all_rows.push(OverflowTableRow::new(clk, value, prev));
        self.active_rows.push(self.all_rows.len() - 1);

        self.update_trace
            .push((clk, OverflowTableUpdate::InsertRow));

        if self.trace_enabled {
            // insert a copy of the current table state into the trace
            self.trace.insert(clk, self.get_values());
        }
    }

    /// Removes the last value from the overflow table and returns it together with the clock
    /// cycle of the next value in the table.
    ///
    /// If after the top value is removed the table is empty, the returned clock cycle is ZERO.
    pub fn pop(&mut self, clk: usize) -> (Felt, Felt) {
        let last_row_idx = self.active_rows.pop().expect("overflow table is empty");
        let last_row = &self.all_rows[last_row_idx];

        self.update_trace
            .push((clk, OverflowTableUpdate::RemoveRow));

        if self.trace_enabled {
            // insert a copy of the current table state into the trace
            self.trace.insert(clk, self.get_values());
        }

        // determine the clock cycle of the next row and return
        (last_row.val, last_row.prev)
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Appends the top n values from the overflow table to the end of the provided vector.
    pub fn append_into(&self, target: &mut Vec<Felt>, n: usize) {
        for &idx in self.active_rows.iter().rev().take(n) {
            target.push(self.all_rows[idx].val);
        }
    }

    /// Appends the state of the overflow table at the specified clock cycle to the provided vector.
    pub fn append_state_into(&self, target: &mut Vec<Felt>, clk: usize) {
        if let Some(x) = self.trace.range(0..=clk).last() {
            for item in x.1.iter().rev() {
                target.push(*item);
            }
        }
    }

    /// Returns a vector consisting of just the value portion of each table row.
    fn get_values(&self) -> Vec<Felt> {
        self.active_rows
            .iter()
            .map(|&idx| self.all_rows[idx].val)
            .collect()
    }

    // HINT GENERATION
    // --------------------------------------------------------------------------------------------

    pub fn into_hints(self) -> AuxTraceHints {
        AuxTraceHints {
            overflow_hints: self.update_trace,
            overflow_table_rows: self.all_rows,
        }
    }
}

// OVERFLOW TABLE ROW
// ================================================================================================

/// A single row in the stack overflow table. Each row stores the value of the stack item as well
/// as the clock cycle at which the stack item was pushed into the overflow table.
pub struct OverflowTableRow {
    clk: Felt,
    val: Felt,
    prev: Felt,
}

impl OverflowTableRow {
    pub fn new(clk: usize, val: Felt, prev: Felt) -> Self {
        Self {
            clk: Felt::new(clk as u64),
            val,
            prev,
        }
    }
}

impl LookupTableRow for OverflowTableRow {
    /// Reduces this row to a single field element in the field specified by E. This requires
    /// at least 4 alpha values.
    fn to_value<E: FieldElement<BaseField = Felt>>(&self, alphas: &[E]) -> E {
        alphas[0]
            + alphas[1].mul_base(self.clk)
            + alphas[2].mul_base(self.val)
            + alphas[3].mul_base(self.prev)
    }
}

// OVERFLOW TABLE UPDATES
// ================================================================================================

/// Describes an update to the stack overflow table. There could be two types of updates:
/// - A single row can be added to the table. This happens during a right shift.
/// - A single row can be removed from the table. This happens during a left shift.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OverflowTableUpdate {
    InsertRow,
    RemoveRow,
}

// AUXILIARY TRACE HINTS
// ================================================================================================

pub struct AuxTraceHints {
    overflow_hints: Vec<(usize, OverflowTableUpdate)>,
    overflow_table_rows: Vec<OverflowTableRow>,
}

impl AuxTraceHints {
    pub fn overflow_table_hints(&self) -> &[(usize, OverflowTableUpdate)] {
        &self.overflow_hints
    }

    pub fn overflow_table_rows(&self) -> &[OverflowTableRow] {
        &self.overflow_table_rows
    }
}
