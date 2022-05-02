use super::{Felt, FieldElement};
use vm_core::utils::collections::BTreeMap;

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
    rows: Vec<OverflowRow>,
    trace: BTreeMap<usize, Vec<Felt>>,
    trace_enabled: bool,
}

impl OverflowTable {
    /// Returns a new [OverflowTable]. The returned table is empty.
    pub fn new(enable_trace: bool) -> Self {
        Self {
            rows: Vec::new(),
            trace: BTreeMap::new(),
            trace_enabled: enable_trace,
        }
    }

    /// Pushes the specified value into the overflow table.
    pub fn push(&mut self, value: Felt, clk: usize) {
        self.rows.push(OverflowRow::new(clk, value));
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
        let row = self.rows.pop().expect("overflow table is empty");

        if self.trace_enabled {
            // insert a copy of the current table state into the trace
            self.trace.insert(clk, self.get_values());
        }

        // determine the clock cycle of the next row and return
        if self.rows.is_empty() {
            (row.val, Felt::ZERO)
        } else {
            let prev_row = self.rows.last().expect("no previous row");
            (row.val, prev_row.clk)
        }
    }

    /// Appends the top n values from the overflow table to the end of the provided vector.
    pub fn append_into(&self, target: &mut Vec<Felt>, n: usize) {
        for row in self.rows.iter().rev().take(n) {
            target.push(row.val);
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
        self.rows.iter().map(|r| r.val).collect()
    }
}

// OVERFLOW ROW
// ================================================================================================

/// A single row in the stack overflow table. Each row stores the value of the stack item as well
/// as the clock cycle at which the stack item was pushed into the overflow table.
struct OverflowRow {
    clk: Felt,
    val: Felt,
}

impl OverflowRow {
    pub fn new(clk: usize, val: Felt) -> Self {
        Self {
            clk: Felt::new(clk as u64),
            val,
        }
    }
}
