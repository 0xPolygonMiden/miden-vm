use alloc::{collections::BTreeMap, vec::Vec};
use core::ops::RangeInclusive;

use miden_air::RowIndex;

use super::{Felt, ZERO};
use crate::ContextId;

// OVERFLOW TABLE
// ================================================================================================

/// An element of an overflow stack.
///
/// Stores the value pushed on an overflow stack, and the clock cycle at which it was pushed.
#[derive(Debug, Clone)]
struct OverflowStackEntry {
    pub value: Felt,
    pub clk: RowIndex,
}

impl OverflowStackEntry {
    pub fn new(value: Felt, clk: RowIndex) -> Self {
        Self { value, clk }
    }

    pub fn value(&self) -> Felt {
        self.value
    }
}

/// Represents an overflow stack at a given context.
///
/// All contexts other than the root context can never be re-entered, so can have at most a single
/// instance of `OverflowStack` per context. However, the root context can be re-entered with a
/// syscall, and so can have up to 2 instances of `OverflowStack` per context.
#[derive(Debug, Default)]
struct OverflowStack {
    overflow: Vec<OverflowStackEntry>,
}

impl OverflowStack {
    pub fn new() -> Self {
        Self { overflow: Vec::new() }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the last value in the overflow stack, if any.
    pub fn last(&self) -> Option<&OverflowStackEntry> {
        self.overflow.last()
    }

    /// Returns the number of elements in the overflow stack.
    pub fn num_elements(&self) -> usize {
        self.overflow.len()
    }

    pub fn is_empty(&self) -> bool {
        self.overflow.is_empty()
    }

    /// Returns an iterator over the elements in the overflow stack.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &OverflowStackEntry> {
        self.overflow.iter()
    }

    // PUBLIC MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Pushes a value onto the overflow stack.
    pub fn push(&mut self, entry: OverflowStackEntry) {
        self.overflow.push(entry);
    }

    /// Pops a value from the overflow stack, if any.
    pub fn pop(&mut self) -> Option<OverflowStackEntry> {
        self.overflow.pop()
    }
}

/// An overflow table which stores the values of the stack elements that overflow the top 16
/// elements of the stack per context.
///
/// This overflow table does not keep track of the clock cycles at which the values were added or
/// removed from the table; it is only concerned with the state at the latest clock cycle.
///
/// The overflow table keeps track of the current clock cycle, and hence `advance_clock()` must be
/// called whenever the clock cycle is incremented globally.
#[derive(Debug)]
pub struct OverflowTable {
    overflow: BTreeMap<ContextId, Vec<OverflowStack>>,
    current_ctx: ContextId,
    clk: RowIndex,
    history: Option<OverflowTableHistory>,
}

impl OverflowTable {
    /// Creates a new empty overflow table.
    ///
    /// If `save_history` is set to true, the table will keep track of the history of the overflow
    /// table at every clock cycle. This is used for debugging purposes.
    pub fn new(save_history: bool) -> Self {
        // The root context is initialized with an empty overflow stack.
        let overflow = {
            let mut overflow = BTreeMap::new();
            overflow.insert(ContextId::root(), vec![OverflowStack::new()]);
            overflow
        };

        Self {
            overflow,
            current_ctx: ContextId::default(),
            clk: RowIndex::from(0),
            history: save_history.then(OverflowTableHistory::new),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Appends the values from the overflow stack corresponding to the current context to the end
    /// of the provided vector.
    pub fn append_into(&self, target: &mut Vec<Felt>) {
        let current_overflow_stack = self.get_current_overflow_stack();
        target.extend(current_overflow_stack.iter().rev().map(OverflowStackEntry::value));
    }

    /// Appends the values from the overflow table at the given clock cycle to the end of the
    /// provided vector.
    ///
    /// # Panics
    /// - if the overflow history is not enabled.
    pub fn append_from_history_at(&self, clk: RowIndex, target: &mut Vec<Felt>) {
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

    /// Returns the clock cycle at which the latest overflow table entry was added in the current
    /// context.
    ///
    /// Hence, if no entries were added to the overflow table in the current context, ZERO is
    /// returned.
    pub fn last_update_clk_in_current_ctx(&self) -> Felt {
        self.get_current_overflow_stack()
            .last()
            .map_or(ZERO, |entry| Felt::from(entry.clk))
    }

    /// Returns the total number of elements in the overflow table across all stacks in all
    /// contexts.
    pub fn total_num_elements(&self) -> usize {
        self.overflow
            .values()
            .map(|overflow_stacks| {
                overflow_stacks.iter().map(OverflowStack::num_elements).sum::<usize>()
            })
            .sum()
    }

    // PUBLIC MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Pushes a value into the overflow table in the current context.
    pub fn push(&mut self, value: Felt) {
        let clk = self.clk;

        // 1. save history
        if self.history.is_some() {
            let table_before_push: Vec<Felt> = self
                .get_current_overflow_stack()
                .iter()
                .map(OverflowStackEntry::value)
                .collect();

            // Note: we do the `history.is_some()` and `history.unwrap()` instead of matching to
            // satisfy the borrow checker (due to the call to `get_current_overflow_stack()` which
            // borrows `self` immutably).
            self.history
                .as_mut()
                .unwrap()
                .save_stack_to_history_before_clk(clk, table_before_push);
        }

        // 2. push value
        self.get_current_overflow_stack_mut().push(OverflowStackEntry::new(value, clk));
    }

    /// Removes the last value from the overflow table in the current context, if any, and returns
    /// it.
    pub fn pop(&mut self) -> Option<Felt> {
        let clk = self.clk;

        // 1. save history
        if self.history.is_some() {
            let table_before_pop: Vec<Felt> = self
                .get_current_overflow_stack()
                .iter()
                .map(OverflowStackEntry::value)
                .collect();

            // Note: we do the `history.is_some()` and `history.unwrap()` instead of matching to
            // satisfy the borrow checker (due to the call to `get_current_overflow_stack()` which
            // borrows `self` immutably).
            self.history
                .as_mut()
                .unwrap()
                .save_stack_to_history_before_clk(clk, table_before_pop);
        }

        // 2. pop value
        self.get_current_overflow_stack_mut()
            .pop()
            .as_ref()
            .map(OverflowStackEntry::value)
    }

    /// Starts the specified context.
    ///
    /// Subsequent calls to `push` and `pop` will affect the overflow table in this context.
    ///
    /// Note: It is possible to return to context 0 with a syscall; in this case, each instantiation
    /// of context 0 will get a separate overflow table.
    pub fn start_context(&mut self, new_ctx: ContextId) {
        // 1. save history
        if self.history.is_some() {
            let table_before_context_change: Vec<Felt> = self
                .get_current_overflow_stack()
                .iter()
                .map(OverflowStackEntry::value)
                .collect();

            self.history
                .as_mut()
                .unwrap()
                .save_stack_to_history_before_clk(self.clk, table_before_context_change);
        }

        // 2. Initialize the overflow stack for the new context if it doesn't exist.
        self.overflow.entry(new_ctx).or_default().push(OverflowStack::new());

        // 3. set new context
        self.current_ctx = new_ctx;
    }

    /// Restores the specified context.
    ///
    /// # Panics
    /// - if there is no overflow stack for the current context.
    /// - if the overflow stack for the current context is not empty.
    ///   - i.e. this should be checked before calling this function.
    pub fn restore_context(&mut self, new_ctx: ContextId) {
        // 1. save history
        if self.history.is_some() {
            let table_before_context_change: Vec<Felt> = self
                .get_current_overflow_stack()
                .iter()
                .map(OverflowStackEntry::value)
                .collect();

            self.history
                .as_mut()
                .unwrap()
                .save_stack_to_history_before_clk(self.clk, table_before_context_change);
        }

        // 2. pop the last overflow stack for the current context, and make sure it is empty.
        let overflow_stack_for_ctx = self
            .overflow
            .get_mut(&self.current_ctx)
            .expect("no overflow stack at the end of a context")
            .pop()
            .expect("no overflow stack at the end of a context");
        assert!(overflow_stack_for_ctx.is_empty());

        // 3. set new context
        self.current_ctx = new_ctx;
    }

    /// Increments the clock cycle.
    pub fn advance_clock(&mut self) {
        self.clk += 1;
    }

    // HELPERS
    // --------------------------------------------------------------------------------------------

    /// Returns the overflow stack for the current context.
    ///
    /// Specifically, this is a reference to the more recent overflow stack in the list of overflow
    /// stacks for the current context. Recall that for all contexts other than the root context,
    /// there is at most one overflow stack, but for the root context, there can be two.
    fn get_current_overflow_stack(&self) -> &OverflowStack {
        self.overflow
            .get(&self.current_ctx)
            .and_then(|overflow_stacks| overflow_stacks.last())
            .expect("The current context should always have an overflow stack initialized")
    }

    /// Mutable version of `get_current_overflow_stack()`.
    fn get_current_overflow_stack_mut(&mut self) -> &mut OverflowStack {
        self.overflow
            .get_mut(&self.current_ctx)
            .and_then(|overflow_stacks| overflow_stacks.last_mut())
            .expect("The current context should always have an overflow stack initialized")
    }
}

/// Stores the history of the overflow table at every clock cycle, where only the relevant context
/// is stored in the history for each clock cycle.
///
/// The events which update the overflow table history are:
/// - `push` operation,
/// - `pop` operation,
/// - a new context is started,
/// - a former context is restored.
#[derive(Debug)]
struct OverflowTableHistory {
    /// Stores the full state of the overflow table at every clock cycle. Formally, if `clk <
    /// last_transition_clk`, then there is exactly one range in the history which contains `clk`.
    history: Vec<(RangeInclusive<RowIndex>, Vec<Felt>)>,
}

impl OverflowTableHistory {
    /// Creates a new empty overflow table history.
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the overflow table at the given clock cycle in the active context at that clock
    /// cycle if present in the history, otherwise returns `None`.
    ///
    /// That is, if the target clock cycle is greater than the last clock cycle at which the
    /// history was updated, the method returns `None`.
    ///
    /// The first element returned by the iterator is the top of the overflow table.
    pub fn get_at(&self, target_clk: RowIndex) -> Option<impl Iterator<Item = &Felt>> {
        match self.last_clk_in_history() {
            Some(last_clk_in_history) => {
                if target_clk > last_clk_in_history {
                    None
                } else {
                    let (_, overflow_stack) = self
                        .history
                        .iter()
                        .find(|(range, _)| range.contains(&target_clk))
                        .expect("overflow table history not properly constructed");

                    Some(overflow_stack.iter().rev())
                }
            },
            None => None,
        }
    }

    // PUBLIC MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Saves the stack to the history to end right before the given clock cycle.
    ///
    /// The `stack_before_operation` parameter specifies the state of the overflow stack
    /// *before* the operation at the given clock cycle was performed.
    pub fn save_stack_to_history_before_clk(
        &mut self,
        clk: RowIndex,
        stack_before_operation: Vec<Felt>,
    ) {
        // The edge case where `clk == 0` happens if e.g. the first instruction of the program is a
        // `CALL`. In this case, there is no history to save.
        if clk > 0 {
            self.save_stack_to_history(clk - 1, stack_before_operation);
        }
    }

    // HELPERS
    // --------------------------------------------------------------------------------------------

    /// Saves the current state of the overflow table to the history.
    ///
    /// The `clk_end_inclusive` parameter specifies the last clock cycle with the given table as the
    /// state of the overflow table.
    fn save_stack_to_history(&mut self, clk_end_inclusive: RowIndex, stack: Vec<Felt>) {
        let range_start = match self.last_clk_in_history() {
            Some(last_clk) => last_clk + 1,
            None => RowIndex::from(0),
        };

        // If `range_start > clk_end_inclusive`, this indicates that the overflow stack was updated
        // twice in the same clock cycle, which only occurs with the `DYNCALL` operation. In this
        // case, we just ignore the 2nd update.
        if range_start <= clk_end_inclusive {
            let clk_range = range_start..=clk_end_inclusive;
            self.history.push((clk_range, stack));
        }
    }

    /// Returns the last clock cycle contained in the history, or `None` if the history is empty.
    fn last_clk_in_history(&self) -> Option<RowIndex> {
        self.history.last().map(|(range, _)| *range.end())
    }
}
