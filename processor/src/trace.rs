use super::{Felt, FieldElement, Process, StackTrace, STACK_TOP_SIZE};
use core::slice;
use winterfell::Trace;

// VM EXECUTION TRACE
// ================================================================================================

/// TODO: for now this consists only of stack trace, but will need to include decoder trace,
/// auxiliary table traces etc.
pub struct ExecutionTrace {
    meta: Vec<u8>,
    stack: StackTrace,
}

impl ExecutionTrace {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Builds an execution trace for the provided process.
    pub(super) fn new(process: Process) -> Self {
        let Process {
            step,
            decoder: _,
            stack,
            hasher: _,
            bitwise: _,
            memory: _,
            advice: _,
        } = process;

        let mut stack_trace = stack.into_trace();
        for column in stack_trace.iter_mut() {
            finalize_column(column, step);
        }

        Self {
            meta: Vec::new(),
            stack: stack_trace,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// TODO: add docs
    pub fn init_stack_state(&self) -> [Felt; STACK_TOP_SIZE] {
        let mut result = [Felt::ZERO; STACK_TOP_SIZE];
        self.read_row_into(0, &mut result);
        result
    }

    /// TODO: add docs
    pub fn last_stack_state(&self) -> [Felt; STACK_TOP_SIZE] {
        let mut result = [Felt::ZERO; STACK_TOP_SIZE];
        self.read_row_into(self.length() - 1, &mut result);
        result
    }
}

// TRACE TRAIT IMPLEMENTATION
// ================================================================================================

impl Trace for ExecutionTrace {
    type BaseField = Felt;

    fn width(&self) -> usize {
        self.stack.len()
    }

    fn length(&self) -> usize {
        self.stack[0].len()
    }

    fn get(&self, col_idx: usize, row_idx: usize) -> Self::BaseField {
        self.stack[col_idx][row_idx]
    }

    fn meta(&self) -> &[u8] {
        &self.meta
    }

    fn read_row_into(&self, step: usize, target: &mut [Self::BaseField]) {
        for (i, register) in self.stack.iter().enumerate() {
            target[i] = register[step];
        }
    }

    fn into_columns(self) -> Vec<Vec<Self::BaseField>> {
        self.stack.into()
    }
}

// TRACE FRAGMENT
// ================================================================================================

/// TODO: add docs
pub struct TraceFragment<'a> {
    data: Vec<&'a mut [Felt]>,
}

impl<'a> TraceFragment<'a> {
    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the number of columns in this execution trace fragment.
    pub fn width(&self) -> usize {
        self.data.len()
    }

    /// Returns the number of rows in this execution trace fragment.
    pub fn len(&self) -> usize {
        self.data[0].len()
    }

    // DATA MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Updates a single cell in this fragment with provided value.
    #[inline(always)]
    pub fn set(&mut self, row_idx: usize, col_idx: usize, value: Felt) {
        self.data[col_idx][row_idx] = value;
    }

    /// Returns a mutable iterator the the columns of this fragment.
    pub fn columns(&mut self) -> slice::IterMut<'_, &'a mut [Felt]> {
        self.data.iter_mut()
    }

    // TEST METHODS
    // --------------------------------------------------------------------------------------------

    #[cfg(test)]
    pub fn trace_to_fragment(trace: &'a mut [Vec<Felt>]) -> Self {
        let mut data = Vec::new();
        for column in trace.iter_mut() {
            data.push(column.as_mut_slice());
        }
        Self { data }
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn finalize_column(column: &mut Vec<Felt>, step: usize) {
    let last_value = column[step];
    column[step..].fill(last_value);
}
