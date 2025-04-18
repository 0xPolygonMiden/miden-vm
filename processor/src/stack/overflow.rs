use alloc::{collections::BTreeMap, vec::Vec};
use core::ops::RangeInclusive;

use miden_air::RowIndex;

use super::{Felt, ZERO};
use crate::ContextId;

// overflow stack
// ================================================================================================

/// An element of the overflow stack.
///
/// Store the value pushed on the overflow stack, and the clock cycle at which it was pushed.
#[derive(Debug, Clone)]
struct StackEntry {
    pub value: Felt,
    pub clk: RowIndex,
}

impl StackEntry {
    pub fn new(value: Felt, clk: RowIndex) -> Self {
        Self { value, clk }
    }

    pub fn value(&self) -> Felt {
        self.value
    }
}

/// An overflow stack which stores the values of the stack elements that overflow the top 16
/// elements of the stack per context.
///
/// This overflow stack does not keep track of the clock cycles at which the values were added or
/// removed from the table; it is only concerned with the state at the latest clock cycle.
///
/// The overflow stack keeps track of the current clock cycle, and hence `advance_clock()` must be
/// called whenever the clock cycle is incremented globally.
#[derive(Debug)]
pub struct OverflowStack {
    overflow: BTreeMap<ContextId, Vec<StackEntry>>,
    current_ctx: ContextId,
    clk: RowIndex,
    history: Option<OverflowStackHistory>,
}

impl OverflowStack {
    /// Creates a new empty overflow stack.
    ///
    /// If `save_history` is set to true, the table will keep track of the history of the overflow
    /// table at every clock cycle. This is used for debugging purposes.
    pub fn new(save_history: bool) -> Self {
        Self {
            overflow: BTreeMap::new(),
            current_ctx: ContextId::default(),
            clk: RowIndex::from(0),
            history: save_history.then(OverflowStackHistory::new),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Appends the values from the overflow stack to the end of the provided vector.
    pub fn append_into(&self, target: &mut Vec<Felt>) {
        if let Some(overflow) = self.overflow.get(&self.current_ctx) {
            target.extend(overflow.iter().rev().map(StackEntry::value));
        }
    }

    /// Appends the values from the overflow stack at the given clock cycle to the end of the
    /// provided vector.
    pub fn append_into_at_clk(&self, clk: RowIndex, target: &mut Vec<Felt>) {
        let history = self.history.as_ref().expect("overflow history not enabled");
        match history.get_at(clk) {
            Some(table) => {
                target.extend(table);
            },
            None => {
                // if the target clock cycle is greater than the last clock cycle at which the
                // history was updated, then the clk must lie in the current state of the overflow
                // table.
                self.append_into(target);
            },
        }
    }

    /// Returns the clock cycle at which the latest overflow stack entry was added in the current
    /// context.
    ///
    /// Hence, if no entries were added to the overflow stack in the current context, ZERO is
    /// returned.
    pub fn last_update_clk_in_current_ctx(&self) -> Felt {
        if let Some(overflow) = self.overflow.get(&self.current_ctx) {
            overflow.last().map_or(ZERO, |entry| Felt::from(entry.clk))
        } else {
            ZERO
        }
    }

    /// Returns the total number of elements in the overflow stack across all contexts.
    pub fn total_num_elements(&self) -> usize {
        self.overflow.values().map(|v| v.len()).sum()
    }

    // PUBLIC MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Pushes a value into the overflow stack in the current context.
    pub fn push(&mut self, value: Felt) {
        // 1. save history
        if let Some(history) = self.history.as_mut() {
            history.on_push_or_pop(
                self.clk,
                self.overflow
                    .get(&self.current_ctx)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(StackEntry::value)
                    .collect(),
            )
        }

        // 2. push value
        self.overflow
            .entry(self.current_ctx)
            .or_default()
            .push(StackEntry::new(value, self.clk));
    }

    /// Removes the last value from the overflow stack in the current context, if any, and returns
    /// it.
    pub fn pop(&mut self) -> Option<Felt> {
        // 1. save history
        if let Some(history) = self.history.as_mut() {
            history.on_push_or_pop(
                self.clk,
                self.overflow
                    .get(&self.current_ctx)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|e| e.value)
                    .collect(),
            )
        }

        // 2. pop value
        self.overflow
            .entry(self.current_ctx)
            .or_default()
            .pop()
            .as_ref()
            .map(StackEntry::value)
    }

    /// Sets the current context to the specified value.
    ///
    /// Subsequent calls to `push` and `pop` will affect the overflow stack in this context.
    pub fn set_current_context(&mut self, new_ctx: ContextId) {
        // 1. save history
        if let Some(history) = self.history.as_mut() {
            history.start_or_restore_context(
                self.clk,
                self.overflow
                    .get(&self.current_ctx)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(StackEntry::value)
                    .collect(),
            );
        }

        // 2. set new context
        self.current_ctx = new_ctx;
    }

    /// Increments the clock cycle.
    pub fn advance_clock(&mut self) {
        self.clk += 1;
    }
}

/// Stores the history of the overflow stack at every clock cycle, where only the relevant context
/// is stored in the history for each clock cycle.
///
/// The events which update the overflow stack history are:
/// - `push` operation,
/// - `pop` operation,
/// - a new context is started,
/// - a former context is restored.
#[derive(Debug)]
struct OverflowStackHistory {
    /// Stores the full state of the overflow stack at every clock cycle. Formally, if `clk <
    /// last_transition_clk`, then there is exactly one range in the history which contains `clk`.
    history: Vec<(RangeInclusive<RowIndex>, Vec<Felt>)>,
}

impl OverflowStackHistory {
    /// Creates a new empty overflow stack history.
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the overflow stack at the given clock cycle in the active context at that clock
    /// cycle if present in the history, otherwise returns `None`.
    ///
    /// That is, if the target clock cycle is greater than the last clock cycle at which the
    /// history was updated, the method returns `None`.
    ///
    /// The first element returned by the iterator is the top of the overflow stack.
    pub fn get_at(&self, target_clk: RowIndex) -> Option<impl Iterator<Item = &Felt>> {
        match self.last_clk_in_history() {
            Some(last_clk_in_history) => {
                if target_clk > last_clk_in_history {
                    None
                } else {
                    for (range, state) in self.history.iter() {
                        if range.contains(&target_clk) {
                            return Some(state.iter().rev());
                        }
                    }

                    unreachable!("overflow stack history not properly constructed")
                }
            },
            None => None,
        }
    }

    // PUBLIC MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Indicates that an element was added to or removed from the overflow stack at the current
    /// clock cycle.
    ///
    /// The `table_before_operation` parameter specifies the state of the overflow stack *before*
    /// the operation was performed.
    pub fn on_push_or_pop(&mut self, clk: RowIndex, table_before_operation: Vec<Felt>) {
        self.save_table_to_history(clk - 1, table_before_operation);
    }

    /// Indicates that a new context was started or a former context was restored at the current
    /// clock cycle.
    ///
    /// The `table_before_context_change` parameter specifies the state of the overflow stack
    /// *before* the context change was performed.
    pub fn start_or_restore_context(
        &mut self,
        clk: RowIndex,
        table_before_context_change: Vec<Felt>,
    ) {
        self.save_table_to_history(clk, table_before_context_change);
    }

    // HELPERS
    // --------------------------------------------------------------------------------------------

    /// Saves the current state of the overflow stack to the history.
    ///
    /// The `clk_end` specifies the last clock cycle with the given table as the state of the
    /// overflow stack.
    fn save_table_to_history(&mut self, clk_end_inclusive: RowIndex, table: Vec<Felt>) {
        let clk_range = {
            let range_start = match self.last_clk_in_history() {
                Some(last_clk) => last_clk + 1,
                None => RowIndex::from(0),
            };

            range_start..=clk_end_inclusive
        };

        self.history.push((clk_range, table));
    }

    /// Returns the last clock cycle contained in the history, or `None` if the history is empty.
    fn last_clk_in_history(&self) -> Option<RowIndex> {
        self.history.last().map(|(range, _)| *range.end())
    }
}
