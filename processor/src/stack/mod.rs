use super::{ExecutionError, Felt, FieldElement, ProgramInputs, StackTrace, STACK_TOP_SIZE};
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
        let mut trace: Vec<Vec<Felt>> = Vec::with_capacity(STACK_TOP_SIZE);
        for i in 0..STACK_TOP_SIZE {
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
            depth: init_values.len(),
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
    pub fn trace_length(&self) -> usize {
        self.trace[0].len()
    }

    /// Returns a copy of the item currently at the top of the stack.
    ///
    /// # Errors
    /// Returns an error if the stack is empty.
    pub fn peek(&self) -> Result<Felt, ExecutionError> {
        if self.depth == 0 {
            return Err(ExecutionError::StackUnderflow("peek", self.step));
        }

        Ok(self.trace[0][self.step])
    }

    /// Return states from the stack, which includes both the trace state and overflow table.
    /// If n is not passed in, this returns all states.
    pub fn get_values(&self, n: Option<usize>) -> Vec<Felt> {
        let n = n.unwrap_or(usize::MAX);
        let num_items = cmp::min(n, self.depth());

        let num_top_items = cmp::min(STACK_TOP_SIZE, num_items);
        let mut result = self.trace_state()[..num_top_items].to_vec();

        if num_items > STACK_TOP_SIZE {
            let num_overflow_items = num_items - STACK_TOP_SIZE;
            result.extend_from_slice(&self.overflow[..num_overflow_items]);
        }

        result
    }

    /// Returns trace state at the current step.
    ///
    /// Trace state is always 16 elements long and contains the top 16 values of the stack. When
    /// the stack depth is less than 16, the un-used slots contain ZEROs.
    #[allow(dead_code)]
    pub fn trace_state(&self) -> [Felt; STACK_TOP_SIZE] {
        let mut result = [Felt::ZERO; STACK_TOP_SIZE];
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
            start_pos < STACK_TOP_SIZE,
            "start cannot exceed stack top size"
        );
        debug_assert!(start_pos <= self.depth, "stack underflow");
        let end_pos = cmp::min(self.depth, STACK_TOP_SIZE);
        for i in start_pos..end_pos {
            self.trace[i][self.step + 1] = self.trace[i][self.step];
        }
    }

    /// Copies stack values starting at the specified position at the current clock cycle to
    /// position - 1 at the next clock cycle.
    ///
    /// If the stack depth is greater than 16, an item is moved from the overflow stack to the
    /// "in-memory" portion of the stack.
    ///
    /// # Panics
    /// Panics if the stack is empty.
    pub fn shift_left(&mut self, start_pos: usize) {
        debug_assert!(start_pos > 0, "start position must be greater than 0");
        debug_assert!(
            start_pos < STACK_TOP_SIZE,
            "start position cannot exceed stack top size"
        );
        debug_assert!(
            start_pos <= self.depth,
            "start position cannot exceed current depth"
        );

        match self.depth {
            0 => unreachable!("stack underflow"),
            1..=16 => {
                for i in start_pos..self.depth {
                    self.trace[i - 1][self.step + 1] = self.trace[i][self.step];
                }
            }
            _ => {
                for i in start_pos..STACK_TOP_SIZE {
                    self.trace[i - 1][self.step + 1] = self.trace[i][self.step];
                }
                let from_overflow = self.overflow.pop().expect("overflow stack is empty");
                self.trace[STACK_TOP_SIZE - 1][self.step + 1] = from_overflow;
            }
        }

        self.depth -= 1;
    }

    /// Copies stack values starting a the specified position at the current clock cycle to
    /// position + 1 at the next clock cycle
    ///
    /// If stack depth grows beyond 16 items, the additional item is pushed into the overflow
    /// stack.
    pub fn shift_right(&mut self, start_pos: usize) {
        debug_assert!(
            start_pos < STACK_TOP_SIZE,
            "start position cannot exceed stack top size"
        );
        debug_assert!(
            start_pos <= self.depth,
            "start position cannot exceed current depth"
        );

        const MAX_TOP_IDX: usize = STACK_TOP_SIZE - 1;
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
        if self.step + 1 >= self.trace_length() {
            let new_length = self.trace_length() * 2;
            for register in self.trace.iter_mut() {
                register.resize(new_length, Felt::ZERO);
            }
        }
    }

    /// Returns an error if the current stack depth is smaller than the specified required depth.
    ///
    /// The returned error includes the name of the operation (passed in as `op`) which triggered
    /// the check.
    pub fn check_depth(&self, req_depth: usize, op: &'static str) -> Result<(), ExecutionError> {
        if self.depth < req_depth {
            Err(ExecutionError::StackUnderflow(op, self.step))
        } else {
            Ok(())
        }
    }
}
