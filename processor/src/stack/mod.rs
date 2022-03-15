use super::{
    Felt, FieldElement, ProgramInputs, StackTopState, MIN_STACK_DEPTH, NUM_STACK_HELPER_COLS,
    STACK_TRACE_WIDTH,
};
use core::cmp;

mod trace;
pub use trace::StackTrace;

// CONSTANTS
// ================================================================================================

/// Number of columns in the virtual overflow table.
const NUM_OVERFLOW_COLS: usize = 3;

/// The address column of the overflow table.
const OVERFLOW_ADDR_IDX: usize = 0;

/// The column of the overflow table that contains the overflowed values.
const OVERFLOW_VAL_IDX: usize = 1;

/// The column of the overflow table that has the addresses of each of the previous overflow rows.
const OVERFLOW_PREV_ADDR_IDX: usize = 2;

/// The last stack index accessible by the VM.
const MAX_TOP_IDX: usize = MIN_STACK_DEPTH - 1;

// STACK
// ================================================================================================

/// TODO: add comments
pub struct Stack {
    step: usize,
    trace: StackTrace,
    overflow: [Vec<Felt>; NUM_OVERFLOW_COLS],
    depth: usize,
}

impl Stack {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// TODO: add comments
    pub fn new(inputs: &ProgramInputs, init_trace_length: usize) -> Self {
        let trace = StackTrace::new(inputs, init_trace_length);

        let overflow = [Vec::new(), Vec::new(), Vec::new()];

        Self {
            step: 0,
            trace,
            overflow,
            depth: MIN_STACK_DEPTH,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns depth of the stack at the current step.
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Returns the current step of the execution trace.
    #[allow(dead_code)]
    pub fn current_step(&self) -> usize {
        self.step
    }

    /// Returns execution trace length for this stack.
    pub fn trace_len(&self) -> usize {
        self.trace.trace_len()
    }

    /// Returns a copy of the item currently at the top of the stack.
    pub fn peek(&self) -> Felt {
        self.trace.peek_at(self.step)
    }

    /// Return states from the stack, which includes both the trace state and overflow table.
    /// If n is not passed in, this returns all states.
    pub fn get_values(&self, n: Option<usize>) -> Vec<Felt> {
        let n = n.unwrap_or(usize::MAX);
        let num_items = cmp::min(n, self.depth());

        let num_top_items = cmp::min(MIN_STACK_DEPTH, num_items);
        let mut result = self.trace.get_stack_values_at(self.step, num_top_items);

        if num_items > MIN_STACK_DEPTH {
            let num_overflow_items = num_items - MIN_STACK_DEPTH;
            result.extend_from_slice(&self.overflow[OVERFLOW_VAL_IDX][..num_overflow_items]);
        }

        result
    }

    /// Returns an execution trace of the stack and helper columns from the StackTrace as a single
    /// array.
    pub fn into_trace(self) -> [Vec<Felt>; STACK_TRACE_WIDTH] {
        self.trace.into_array()
    }

    // TRACE ACCESSORS AND MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Returns the value located at the specified position on the stack at the current clock cycle.
    pub fn get(&self, pos: usize) -> Felt {
        debug_assert!(pos < self.depth, "stack underflow");
        self.trace.get_stack_value_at(self.step, pos)
    }

    /// Sets the value at the specified position on the stack at the next clock cycle.
    pub fn set(&mut self, pos: usize, value: Felt) {
        debug_assert!(pos == 0 || pos < self.depth, "stack underflow");
        self.trace.set_stack_value_at(self.step + 1, pos, value);
    }

    /// Returns trace state at the current step.
    ///
    /// Trace state is always 16 elements long and contains the top 16 values of the stack. When
    /// the stack depth is less than 16, the un-used slots contain ZEROs.
    #[allow(dead_code)]
    pub fn trace_state(&self) -> StackTopState {
        self.trace.get_stack_state_at(self.step)
    }

    /// Copies stack values starting at the specified position at the current clock cycle to the
    /// same position at the next clock cycle.
    pub fn copy_state(&mut self, start_pos: usize) {
        debug_assert!(
            start_pos < MIN_STACK_DEPTH,
            "start cannot exceed stack top size"
        );
        self.trace.copy_stack_state_at(self.step, start_pos);
    }

    /// Copies stack values starting at the specified position at the current clock cycle to
    /// position - 1 at the next clock cycle.
    ///
    /// If the stack depth is greater than 16, an item is moved from the overflow stack to the
    /// "in-memory" portion of the stack.
    pub fn shift_left(&mut self, start_pos: usize) {
        debug_assert!(start_pos > 0, "start position must be greater than 0");
        debug_assert!(
            start_pos < MIN_STACK_DEPTH,
            "start position cannot exceed stack top size"
        );

        match self.depth {
            0..=MAX_TOP_IDX => unreachable!("stack underflow"),
            MIN_STACK_DEPTH => {
                // Shift in a ZERO, to prevent depth shrinking below the minimum stack depth.
                self.trace
                    .stack_shift_left_at(self.step, start_pos, Felt::ZERO);
                self.trace.copy_helpers_at(self.step);
            }
            _ => {
                // Update the stack & overflow table.
                let from_overflow = self.pop_overflow();
                self.trace.stack_shift_left_at(
                    self.step,
                    start_pos,
                    from_overflow[OVERFLOW_VAL_IDX],
                );

                // Update the bookkeeping & helper columns.
                self.trace
                    .helpers_shift_left_at(self.step, from_overflow[OVERFLOW_PREV_ADDR_IDX]);

                // Stack depth only decreases when it is greater than the minimum stack depth.
                self.depth -= 1;
            }
        }
    }

    /// Copies stack values starting a the specified position at the current clock cycle to
    /// position + 1 at the next clock cycle
    ///
    /// If stack depth grows beyond 16 items, the additional item is pushed into the overflow table.
    pub fn shift_right(&mut self, start_pos: usize) {
        debug_assert!(
            start_pos < MIN_STACK_DEPTH,
            "start position cannot exceed stack top size"
        );

        // Update the stack.
        self.trace.stack_shift_right_at(self.step, start_pos);

        // Update the overflow table.
        let to_overflow = self
            .trace
            .get_stack_value_at(self.step, MIN_STACK_DEPTH - 1);
        self.push_overflow(to_overflow);

        // Update the bookkeeping & helper columns.
        self.trace.helpers_shift_right_at(self.step);

        // Stack depth always increases on right shift.
        self.depth += 1;
    }

    /// Increments the clock cycle.
    pub fn advance_clock(&mut self) {
        self.step += 1;
    }

    // UTILITY METHODS
    // --------------------------------------------------------------------------------------------

    /// Makes sure there is enough memory allocated for the trace to accommodate a new row.
    ///
    /// Trace length is doubled every time it needs to be increased.
    pub fn ensure_trace_capacity(&mut self) {
        self.trace.ensure_trace_capacity(self.step);
    }

    /// Pushes a new row onto the overflow table that contains the value which has overflowed the
    /// stack.
    ///
    /// Each row of the overflow table looks like ```[addr, value, prev_addr]```, where:
    /// - `addr` is the address of the row, which is set to the current clock cycle.
    /// - `value` is the overflowed value.
    /// - `prev_addr` is the address of the previous row in the overflow table.
    pub fn push_overflow(&mut self, value: Felt) {
        let prev_addr = match self.overflow[OVERFLOW_ADDR_IDX].last() {
            Some(addr) => *addr,
            None => Felt::ZERO,
        };
        // Set the address of this overflow row to the current clock cycle.
        self.overflow[OVERFLOW_ADDR_IDX].push(Felt::new(self.step as u64));
        // Save the overflow value.
        self.overflow[OVERFLOW_VAL_IDX].push(value);
        // Save the address of the previous row.
        self.overflow[OVERFLOW_PREV_ADDR_IDX].push(prev_addr)
    }

    /// Pops the last row off the overflow table and returns it.
    ///
    /// # Errors
    /// This function will panic if the overflow table is empty.
    pub fn pop_overflow(&mut self) -> [Felt; NUM_OVERFLOW_COLS] {
        let mut row = [Felt::ZERO; 3];
        row[OVERFLOW_ADDR_IDX] = self.overflow[OVERFLOW_ADDR_IDX]
            .pop()
            .expect("overflow stack is empty");
        row[OVERFLOW_VAL_IDX] = self.overflow[OVERFLOW_VAL_IDX]
            .pop()
            .expect("overflow stack is empty");
        row[OVERFLOW_PREV_ADDR_IDX] = self.overflow[OVERFLOW_PREV_ADDR_IDX]
            .pop()
            .expect("overflow stack is empty");

        row
    }
}
