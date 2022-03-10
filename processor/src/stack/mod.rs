use super::{Felt, FieldElement, ProgramInputs, StackTrace, MIN_STACK_DEPTH};
use core::cmp;

// STACK
// ================================================================================================

/// TODO: add comments
pub struct Stack {
    step: usize,
    trace: StackTrace,
    overflow: Vec<Felt>,
    depth: usize,
}

impl Stack {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// TODO: add comments
    pub fn new(inputs: &ProgramInputs, init_trace_length: usize) -> Self {
        let init_values = inputs.stack_init();
        let mut trace: Vec<Vec<Felt>> = Vec::with_capacity(MIN_STACK_DEPTH);
        for i in 0..MIN_STACK_DEPTH {
            let mut column = vec![Felt::ZERO; init_trace_length];
            if i < init_values.len() {
                column[0] = init_values[i];
            }
            trace.push(column)
        }

        Self {
            step: 0,
            trace: trace.try_into().expect("failed to convert vector to array"),
            overflow: Vec::new(),
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
        self.trace[0].len()
    }

    /// Returns a copy of the item currently at the top of the stack.
    pub fn peek(&self) -> Felt {
        self.trace[0][self.step]
    }

    /// Return states from the stack, which includes both the trace state and overflow table.
    /// If n is not passed in, this returns all states.
    pub fn get_values(&self, n: Option<usize>) -> Vec<Felt> {
        let n = n.unwrap_or(usize::MAX);
        let num_items = cmp::min(n, self.depth());

        let num_top_items = cmp::min(MIN_STACK_DEPTH, num_items);
        let mut result = self.trace_state()[..num_top_items].to_vec();

        if num_items > MIN_STACK_DEPTH {
            let num_overflow_items = num_items - MIN_STACK_DEPTH;
            result.extend_from_slice(&self.overflow[..num_overflow_items]);
        }

        result
    }

    /// Returns trace state at the current step.
    ///
    /// Trace state is always 16 elements long and contains the top 16 values of the stack. When
    /// the stack depth is less than 16, the un-used slots contain ZEROs.
    #[allow(dead_code)]
    pub fn trace_state(&self) -> [Felt; MIN_STACK_DEPTH] {
        let mut result = [Felt::ZERO; MIN_STACK_DEPTH];
        for (result, column) in result.iter_mut().zip(self.trace.iter()) {
            *result = column[self.step];
        }
        result
    }

    /// TODO: add docs
    pub fn into_trace(self) -> StackTrace {
        self.trace
    }

    // TRACE ACCESSORS AND MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Returns the value located at the specified position on the stack at the current clock cycle.
    pub fn get(&self, pos: usize) -> Felt {
        debug_assert!(pos < self.depth, "stack underflow");
        self.trace[pos][self.step]
    }

    /// Sets the value at the specified position on the stack at the next clock cycle.
    pub fn set(&mut self, pos: usize, value: Felt) {
        debug_assert!(pos == 0 || pos < self.depth, "stack underflow");
        self.trace[pos][self.step + 1] = value;
    }

    /// Copies stack values starting at the specified position at the current clock cycle to the
    /// same position at the next clock cycle.
    pub fn copy_state(&mut self, start_pos: usize) {
        debug_assert!(
            start_pos < MIN_STACK_DEPTH,
            "start cannot exceed stack top size"
        );
        debug_assert!(start_pos <= self.depth, "stack underflow");
        let end_pos = cmp::min(self.depth, MIN_STACK_DEPTH);
        for i in start_pos..end_pos {
            self.trace[i][self.step + 1] = self.trace[i][self.step];
        }
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
        debug_assert!(
            start_pos <= self.depth,
            "start position cannot exceed current depth"
        );

        const MAX_TOP_IDX: usize = MIN_STACK_DEPTH - 1;
        match self.depth {
            0..=MAX_TOP_IDX => unreachable!("stack underflow"),
            MIN_STACK_DEPTH => {
                for i in start_pos..=MAX_TOP_IDX {
                    self.trace[i - 1][self.step + 1] = self.trace[i][self.step];
                }
                // Shift in a ZERO to prevent depth shrinking below the minimum stack depth
                self.trace[MAX_TOP_IDX][self.step + 1] = Felt::ZERO;
            }
            _ => {
                for i in start_pos..=MAX_TOP_IDX {
                    self.trace[i - 1][self.step + 1] = self.trace[i][self.step];
                }
                let from_overflow = self.overflow.pop().expect("overflow stack is empty");
                self.trace[MAX_TOP_IDX][self.step + 1] = from_overflow;

                self.depth -= 1;
            }
        }
    }

    /// Copies stack values starting a the specified position at the current clock cycle to
    /// position + 1 at the next clock cycle
    ///
    /// If stack depth grows beyond 16 items, the additional item is pushed into the overflow
    /// stack.
    pub fn shift_right(&mut self, start_pos: usize) {
        debug_assert!(
            start_pos < MIN_STACK_DEPTH,
            "start position cannot exceed stack top size"
        );
        debug_assert!(
            start_pos <= self.depth,
            "start position cannot exceed current depth"
        );

        const MAX_TOP_IDX: usize = MIN_STACK_DEPTH - 1;
        match self.depth {
            0 => {} // if the stack is empty, do nothing
            1..=MAX_TOP_IDX => {
                for i in start_pos..self.depth {
                    self.trace[i + 1][self.step + 1] = self.trace[i][self.step];
                }
            }
            _ => {
                for i in start_pos..MAX_TOP_IDX {
                    self.trace[i + 1][self.step + 1] = self.trace[i][self.step];
                }
                let to_overflow = self.trace[MAX_TOP_IDX][self.step];
                self.overflow.push(to_overflow)
            }
        }

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
        if self.step + 1 >= self.trace_len() {
            let new_length = self.trace_len() * 2;
            for register in self.trace.iter_mut() {
                register.resize(new_length, Felt::ZERO);
            }
        }
    }
}
