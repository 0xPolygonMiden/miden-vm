use alloc::vec::Vec;

use miden_air::{
    trace::{
        stack::{B0_COL_IDX, B1_COL_IDX, H0_COL_IDX, STACK_TOP_OFFSET},
        STACK_TRACE_OFFSET, TRACE_WIDTH,
    },
    RowIndex,
};
use vm_core::{stack::MIN_STACK_DEPTH, utils::uninit_vector, Word, WORD_SIZE};
use winter_prover::math::batch_inversion;

use super::{
    ExecutionError, Felt, FieldElement, StackInputs, StackOutputs, ONE, STACK_TRACE_WIDTH, ZERO,
};

mod overflow;
use overflow::OverflowTable;
pub use overflow::OverflowTableRow;

mod aux_trace;
pub use aux_trace::AuxTraceBuilder;

#[cfg(test)]
mod tests;

// STACK
// ================================================================================================

#[derive(Debug, Clone, Copy)]
struct StackTraceRow {
    stack: [Felt; MIN_STACK_DEPTH],

    // helpers
    stack_depth: Felt,
    next_overflow_address: Felt,
}

/// Stack for the VM.
///
/// This component is responsible for managing the state of the VM's stack, as well as building an
/// execution trace for all stack transitions.
///
/// The state is separated into two parts: the top 16 slots of the stack (stored in the `trace`
/// member), and items which don't fit into the top 16 slots (stored in the `overflow` member).
///
/// The depth of the stack can never drop below 16. If an item is removed from the stack when the
/// depth is 16, a ZERO element is inserted into the 16th slot.
///
/// ## Execution trace
/// The stack execution trace consists of 19 columns as illustrated below:
///
///   s0   s1   s2        s13   s14   s15   b0   b1   h0
/// ├────┴────┴────┴ ... ┴────┴─────┴─────┴────┴────┴────┤
///
/// The meaning of the above columns is as follows:
/// - s0...s15 are the columns representing the top 16 slots of the stack.
/// - Bookkeeping column b0 contains the number of items on the stack (i.e., the stack depth).
/// - Bookkeeping column b1 contains an address of a row in the “overflow table” in which we’ll
///   store the data that doesn’t fit into the top 16 slots. When b1=0, it means that all stack data
///   fits into the top 16 slots of the stack.
/// - Helper column h0 is used to ensure that stack depth does not drop below 16. Values in this
///   column are set by the prover non-deterministically to 1 / (b0−16) when b0 != 16, and to any
///   other value otherwise.
#[derive(Debug)]
pub struct Stack {
    clk: RowIndex,
    overflow: OverflowTable,
    active_depth: usize,
    full_depth: usize,
    rows: Vec<StackTraceRow>,
}

impl Stack {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a [Stack] initialized with the specified program inputs.
    pub fn new(
        inputs: &StackInputs,
        init_trace_capacity: usize,
        keep_overflow_trace: bool,
    ) -> Self {
        let overflow = OverflowTable::new(keep_overflow_trace);
        let rows = {
            let first_row = StackTraceRow {
                stack: **inputs,
                stack_depth: Felt::new(MIN_STACK_DEPTH as u64),
                next_overflow_address: ZERO,
            };

            let mut rows = Vec::with_capacity(init_trace_capacity);
            rows.push(first_row);
            rows
        };

        Self {
            clk: RowIndex::from(0),
            overflow,
            active_depth: MIN_STACK_DEPTH,
            full_depth: MIN_STACK_DEPTH,
            rows,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns depth of the stack at the current clock cycle.
    pub fn depth(&self) -> usize {
        self.active_depth
    }

    /// Returns the current clock cycle of the execution trace.
    pub fn current_clk(&self) -> RowIndex {
        self.clk
    }

    /// Returns execution trace length for this stack.
    ///
    /// Trace length of the stack is equal to the number of cycles executed by the VM.
    pub fn trace_len(&self) -> usize {
        self.clk.into()
    }

    /// Returns a copy of the item currently at the top of the stack.
    pub fn peek(&self) -> Felt {
        self.current_stack_state().stack[0]
    }

    /// Returns stack state at the specified clock cycle. This includes the top 16 items of the
    /// stack + overflow entries.
    ///
    /// # Panics
    /// Panics if invoked for non-last clock cycle on a stack instantiated with
    /// `keep_overflow_trace` set to false.
    pub fn get_state_at(&self, clk: RowIndex) -> Vec<Felt> {
        let mut result = self.current_stack_state().stack.to_vec();
        if clk == self.clk {
            self.overflow.append_into(&mut result);
        } else {
            self.overflow.append_state_into(&mut result, clk.into());
        }

        result
    }

    /// Returns [StackOutputs] consisting of all values on the stack.
    ///
    /// # Errors
    /// Returns an error if the overflow table is not empty at the current clock cycle.
    pub fn build_stack_outputs(&self) -> Result<StackOutputs, ExecutionError> {
        if self.overflow.num_active_rows() != 0 {
            return Err(ExecutionError::OutputStackOverflow(self.overflow.num_active_rows()));
        }

        let stack_items = self.current_stack_state().stack.to_vec();
        Ok(StackOutputs::new(stack_items).expect("processor stack handling logic is valid"))
    }

    // TRACE ACCESSORS AND MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Returns the value located at the specified position on the stack at the current clock cycle.
    pub fn get(&self, pos: usize) -> Felt {
        self.current_stack_state().stack[pos]
    }

    /// Returns a word located at the specified word index on the stack.
    ///
    /// Specifically, word 0 is defined by the first 4 elements of the stack, word 1 is defined
    /// by the next 4 elements etc. Since the top of the stack contains 4 word, the highest valid
    /// word index is 3.
    ///
    /// The words are created in reverse order. For example, for word 0 the top element of the
    /// stack will be at the last position in the word.
    ///
    /// Creating a word does not change the state of the stack.
    pub fn get_word(&self, word_idx: usize) -> Word {
        let offset = word_idx * WORD_SIZE;
        [
            self.get(offset + 3),
            self.get(offset + 2),
            self.get(offset + 1),
            self.get(offset),
        ]
    }

    /// Sets the top elements of the stack to `new_stack_top`, and copies over the rest of the stack
    /// from their corresponding position from the previous state.
    pub fn set_and_copy<const N: usize>(&mut self, new_stack_top: [Felt; N]) {
        debug_assert!(N <= MIN_STACK_DEPTH, "Cannot place more than 16 elements on the stack");

        let new_row = {
            let last_row = *self.current_stack_state();
            let mut new_stack = [ZERO; MIN_STACK_DEPTH];

            // Overwrite the top of the stack.
            new_stack[..N].copy_from_slice(&new_stack_top[..N]);

            // Copy the rest of the stack.
            new_stack[N..].copy_from_slice(&last_row.stack[N..]);

            StackTraceRow {
                stack: new_stack,
                stack_depth: Felt::try_from(self.active_depth as u64)
                    .expect("stack depth is larger than 2^32"),
                next_overflow_address: self.overflow.last_row_addr(),
            }
        };
        self.rows.push(new_row);
    }

    /// Pops the top element from the stack, shifting in a value from the overflow table if the
    /// stack depth is greater than 16, and overwrites the top elements of the stack with
    /// `new_stack_top`.
    pub fn pop_and_set<const N: usize>(&mut self, new_stack_top: [Felt; N]) {
        debug_assert!(N < MIN_STACK_DEPTH, "Cannot place more than 15 elements on the stack");

        let new_row = {
            let last_row = *self.current_stack_state();
            let mut new_stack = [ZERO; MIN_STACK_DEPTH];

            // Overwrite the top of the stack.
            new_stack[..N].copy_from_slice(&new_stack_top[..N]);

            // Copy the rest of the stack, accounting for the left shift.
            new_stack[N..(MIN_STACK_DEPTH - 1)]
                .copy_from_slice(&last_row.stack[(N + 1)..MIN_STACK_DEPTH]);

            // Pop an element from the overflow table if non-empty, or else shift in a ZERO.
            let (last_stack_value, stack_depth, next_overflow_address) = self.pop_overflow_table();
            new_stack[MIN_STACK_DEPTH - 1] = last_stack_value;

            StackTraceRow {
                stack: new_stack,
                stack_depth,
                next_overflow_address,
            }
        };

        self.rows.push(new_row);
    }

    /// Creates a new row by pushing `new_top_element` to the stack.
    pub fn push(&mut self, new_top_element: Felt) {
        let last_row = *self.current_stack_state();

        // Update the stack.
        let new_row = StackTraceRow {
            stack: [
                new_top_element,
                last_row.stack[0],
                last_row.stack[1],
                last_row.stack[2],
                last_row.stack[3],
                last_row.stack[4],
                last_row.stack[5],
                last_row.stack[6],
                last_row.stack[7],
                last_row.stack[8],
                last_row.stack[9],
                last_row.stack[10],
                last_row.stack[11],
                last_row.stack[12],
                last_row.stack[13],
                last_row.stack[14],
            ],
            stack_depth: last_row.stack_depth + ONE,
            next_overflow_address: self.clk.into(),
        };
        self.rows.push(new_row);

        // Update the overflow table.
        let to_overflow = last_row.stack[15];
        self.overflow.push(to_overflow, Felt::from(self.clk));

        // Update depth.
        self.active_depth += 1;
        self.full_depth += 1;
    }

    /// Creates a new row by
    /// 1. dropping the top element of the stack,
    /// 2. pushing `second_element` to the stack, and
    /// 3. pushing `new_top_element` to the stack.
    ///
    /// Note that this increases the stack depth by 1.
    pub fn drop_and_push(&mut self, new_top_element: Felt, second_element: Felt) {
        let last_row = *self.current_stack_state();

        // Update the stack.
        let new_row = StackTraceRow {
            stack: [
                new_top_element,
                second_element,
                last_row.stack[1],
                last_row.stack[2],
                last_row.stack[3],
                last_row.stack[4],
                last_row.stack[5],
                last_row.stack[6],
                last_row.stack[7],
                last_row.stack[8],
                last_row.stack[9],
                last_row.stack[10],
                last_row.stack[11],
                last_row.stack[12],
                last_row.stack[13],
                last_row.stack[14],
            ],
            stack_depth: last_row.stack_depth + ONE,
            next_overflow_address: self.clk.into(),
        };
        self.rows.push(new_row);

        // Update the overflow table.
        let to_overflow = last_row.stack[15];
        self.overflow.push(to_overflow, Felt::from(self.clk));

        // Update depth.
        self.active_depth += 1;
        self.full_depth += 1;
    }

    /// Pop an element from the overflow table if non-empty, or else shift in a ZERO.
    ///
    /// Returns (last_stack_value, stack_depth, next_overflow_address)
    fn pop_overflow_table(&mut self) -> (Felt, Felt, Felt) {
        let last_row = *self.current_stack_state();

        let (last_stack_value, stack_depth, next_overflow_address) =
            if self.active_depth > MIN_STACK_DEPTH {
                let last_stack_value = self.overflow.pop(u64::from(self.clk));

                // Stack depth only decreases when it is greater than the minimum stack depth.
                self.active_depth -= 1;
                self.full_depth -= 1;

                (last_stack_value, last_row.stack_depth - ONE, self.overflow.last_row_addr())
            } else {
                (ZERO, last_row.stack_depth, last_row.next_overflow_address)
            };

        (last_stack_value, stack_depth, next_overflow_address)
    }

    // CONTEXT MANAGEMENT
    // --------------------------------------------------------------------------------------------

    /// Pops the top element off the stack, writes the default values for the stack helper registers
    /// in the trace (stack depth and next overflow address), and returns the value of those helper
    /// registers before the new context wipe.
    ///
    /// This specialized method is needed because the other ones write the updated helper register
    /// values directly to the trace in the next row. However, the dyncall instruction needs to
    /// shift the stack left, and start a new context simultaneously (and hence reset the stack
    /// helper registers to their default value). It is assumed that the caller will write the
    /// return values somewhere else in the trace.
    pub fn pop_and_start_context(&mut self) -> (usize, Felt) {
        // Pop the stack
        let new_stack = {
            let last_row = *self.current_stack_state();

            let mut new_stack = [ZERO; MIN_STACK_DEPTH];
            new_stack[..(MIN_STACK_DEPTH - 1)].copy_from_slice(&last_row.stack[1..MIN_STACK_DEPTH]);

            let (last_stack_value, ..) = self.pop_overflow_table();
            new_stack[MIN_STACK_DEPTH - 1] = last_stack_value;

            new_stack
        };

        // reset the helper columns to their default value, and write those to the trace in the next
        // row.
        let (next_depth, next_overflow_addr) = self.start_context();

        // Note: `start_context()` reset `active_depth` to 16, and `overflow.last_row_addr` to 0.
        let new_row = StackTraceRow {
            stack: new_stack,
            stack_depth: Felt::from(self.active_depth as u32),
            next_overflow_address: self.overflow.last_row_addr(),
        };
        self.rows.push(new_row);

        // return the helper registers' state before the new context
        (next_depth, next_overflow_addr)
    }

    /// Starts a new execution context for this stack and returns a tuple consisting of the current
    /// stack depth and the address of the overflow table row prior to starting the new context.
    ///
    /// This has the effect of hiding the contents of the overflow table such that it appears as
    /// if the overflow table in the new context is empty.
    pub fn start_context(&mut self) -> (usize, Felt) {
        let current_depth = self.active_depth;
        let current_overflow_addr = self.overflow.last_row_addr();
        self.active_depth = MIN_STACK_DEPTH;
        self.overflow.set_last_row_addr(ZERO);
        (current_depth, current_overflow_addr)
    }

    /// Restores the prior context for this stack.
    ///
    /// This has the effect bringing back items previously hidden from the overflow table.
    pub fn restore_context(&mut self, stack_depth: usize, next_overflow_addr: Felt) {
        debug_assert!(stack_depth <= self.full_depth, "stack depth too big");
        debug_assert_eq!(self.active_depth, MIN_STACK_DEPTH, "overflow table not empty");
        self.active_depth = stack_depth;
        self.overflow.set_last_row_addr(next_overflow_addr);
    }

    // TRACE GENERATION
    // --------------------------------------------------------------------------------------------

    // TODO(plafer): Remove this method (and all other unused `into_trace()` methods)
    /// Returns an execution trace of the top 16 stack slots and helper columns as a single array
    /// together with hints to be used in construction of stack-related auxiliary trace segment
    /// columns.
    ///
    /// If the stack trace is smaller than the specified `trace_len`, last value in each column is
    /// duplicated until the length of the columns reaches `trace_len`.
    ///
    /// `num_rand_rows` indicates the number of rows at the end of the trace which will be
    /// overwritten with random values. This parameter is unused because last rows are just
    /// duplicates of the prior rows and thus can be safely overwritten.
    pub fn into_trace(self, trace_len: usize, num_rand_rows: usize) -> super::StackTrace {
        let clk = self.current_clk().as_usize();
        // make sure that only the duplicate rows will be overwritten with random values
        assert!(clk + num_rand_rows <= trace_len, "target trace length too small");

        let own_len = self.rows.len();

        // at the end of program execution we must be in the root context, and thus active and
        // full stack depth must be the same.
        assert_eq!(self.active_depth, self.full_depth, "inconsistent stack depth");

        let mut trace_columns = unsafe { vec![uninit_vector(trace_len); STACK_TRACE_WIDTH] };

        // Note: we need to compute the h0 column. We do this in 2 steps:
        // 1. in the coming for loop, compute the denominator for the h0 column
        // 2. then run batch inversion on the denominators to get the values for the h0 column.
        let mut h0_denoms_col = unsafe { uninit_vector(own_len) };

        const MIN_STACK_DEPTH_FELT: Felt = Felt::new(MIN_STACK_DEPTH as u64);
        for (i, row) in self.rows.into_iter().enumerate() {
            trace_columns[STACK_TOP_OFFSET][i] = row.stack[0];
            trace_columns[STACK_TOP_OFFSET + 1][i] = row.stack[1];
            trace_columns[STACK_TOP_OFFSET + 2][i] = row.stack[2];
            trace_columns[STACK_TOP_OFFSET + 3][i] = row.stack[3];
            trace_columns[STACK_TOP_OFFSET + 4][i] = row.stack[4];
            trace_columns[STACK_TOP_OFFSET + 5][i] = row.stack[5];
            trace_columns[STACK_TOP_OFFSET + 6][i] = row.stack[6];
            trace_columns[STACK_TOP_OFFSET + 7][i] = row.stack[7];
            trace_columns[STACK_TOP_OFFSET + 8][i] = row.stack[8];
            trace_columns[STACK_TOP_OFFSET + 9][i] = row.stack[9];
            trace_columns[STACK_TOP_OFFSET + 10][i] = row.stack[10];
            trace_columns[STACK_TOP_OFFSET + 11][i] = row.stack[11];
            trace_columns[STACK_TOP_OFFSET + 12][i] = row.stack[12];
            trace_columns[STACK_TOP_OFFSET + 13][i] = row.stack[13];
            trace_columns[STACK_TOP_OFFSET + 14][i] = row.stack[14];
            trace_columns[STACK_TOP_OFFSET + 15][i] = row.stack[15];
            trace_columns[B0_COL_IDX][i] = row.stack_depth;
            trace_columns[B1_COL_IDX][i] = row.next_overflow_address;

            h0_denoms_col[i] = row.stack_depth - MIN_STACK_DEPTH_FELT;
        }

        let h0_col = batch_inversion(&h0_denoms_col);
        for (i, ele) in h0_col.into_iter().enumerate() {
            trace_columns[H0_COL_IDX][i] = ele;
        }

        // padding
        for col in &mut trace_columns {
            let last_value = col[own_len - 1];
            col[own_len..trace_len].fill(last_value);
        }

        super::StackTrace { trace: trace_columns.try_into().unwrap() }
    }

    pub fn write_row(&self, row_idx: usize, main_trace: &mut [Felt]) {
        let start_idx = row_idx * TRACE_WIDTH + STACK_TRACE_OFFSET;

        const MIN_STACK_DEPTH_FELT: Felt = Felt::new(MIN_STACK_DEPTH as u64);

        if row_idx < self.rows.len() {
            main_trace[start_idx..(start_idx + MIN_STACK_DEPTH)]
                .copy_from_slice(&self.rows[row_idx].stack);
            main_trace[start_idx + B0_COL_IDX] = self.rows[row_idx].stack_depth;
            main_trace[start_idx + B1_COL_IDX] = self.rows[row_idx].next_overflow_address;

            // TODO(plafer): use batch inversion
            let denom = self.rows[row_idx].stack_depth - MIN_STACK_DEPTH_FELT;
            main_trace[start_idx + H0_COL_IDX] = if denom == ZERO { ZERO } else { denom.inv() };
        } else {
            let last_row_start_idx = (row_idx - 1) * TRACE_WIDTH + STACK_TRACE_OFFSET;

            // padding: copy over from the last row
            for j in 0..STACK_TRACE_WIDTH {
                main_trace[start_idx + j] = main_trace[last_row_start_idx + j];
            }
        }
    }

    // UTILITY METHODS
    // --------------------------------------------------------------------------------------------

    /// Increments the clock cycle.
    pub fn advance_clock(&mut self) {
        self.clk += 1;
    }

    // HELPERS
    // --------------------------------------------------------------------------------------------

    fn current_stack_state(&self) -> &StackTraceRow {
        self.rows.last().expect("stack trace always has at least one row")
    }

    // TEST HELPERS
    // --------------------------------------------------------------------------------------------

    /// Returns state of stack item columns at the current clock cycle. This does not include stack
    /// values in the overflow table.
    #[cfg(any(test, feature = "testing"))]
    pub fn trace_state(&self) -> [Felt; MIN_STACK_DEPTH] {
        self.current_stack_state().stack
    }

    /// Returns state of helper columns at the current clock cycle.
    #[cfg(test)]
    pub fn helpers_state(&self) -> [Felt; miden_air::trace::stack::NUM_STACK_HELPER_COLS] {
        let current_state = self.current_stack_state();

        [
            current_state.stack_depth,
            current_state.next_overflow_address,
            current_state.stack_depth - Felt::from(MIN_STACK_DEPTH as u32),
        ]
    }
}
