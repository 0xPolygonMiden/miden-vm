use super::{
    AuxTable, AuxTableTrace, Digest, Felt, FieldElement, Process, RangeCheckTrace, StackTopState,
    StackTrace, SysTrace,
};
use core::slice;
use vm_core::{
    StarkField, AUX_TRACE_OFFSET, AUX_TRACE_RANGE, MIN_STACK_DEPTH, RANGE_CHECK_TRACE_OFFSET,
    RANGE_CHECK_TRACE_RANGE, STACK_TRACE_OFFSET, STACK_TRACE_RANGE, SYS_TRACE_OFFSET,
    SYS_TRACE_RANGE, TRACE_WIDTH,
};
use winterfell::Trace;

// VM EXECUTION TRACE
// ================================================================================================

/// TODO: for now this consists only of system register trace, stack trace, range check trace, and
/// auxiliary table trace, but will also need to include the decoder trace.
pub struct ExecutionTrace {
    meta: Vec<u8>,
    system: SysTrace,
    stack: StackTrace,
    range: RangeCheckTrace,
    aux_table: AuxTableTrace,
    // TODO: program hash should be retrieved from decoder trace, but for now we store it explicitly
    program_hash: Digest,
}

impl ExecutionTrace {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Builds an execution trace for the provided process.
    pub(super) fn new(process: Process, program_hash: Digest) -> Self {
        let (system_trace, stack_trace, range_check_trace, aux_table_trace) =
            Self::finalize_trace(process);

        Self {
            meta: Vec::new(),
            system: system_trace,
            stack: stack_trace,
            range: range_check_trace,
            aux_table: aux_table_trace,
            program_hash,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// TODO: add docs
    pub fn program_hash(&self) -> Digest {
        // TODO: program hash should be read from the decoder trace
        self.program_hash
    }

    pub fn aux_table(&self) -> &AuxTableTrace {
        &self.aux_table
    }

    pub fn stack(&self) -> &StackTrace {
        &self.stack
    }

    /// TODO: add docs
    pub fn init_stack_state(&self) -> StackTopState {
        let mut result = [Felt::ZERO; MIN_STACK_DEPTH];
        for (result, column) in result.iter_mut().zip(self.stack.iter()) {
            *result = column[0];
        }
        result
    }

    /// TODO: add docs
    pub fn last_stack_state(&self) -> StackTopState {
        let last_step = self.length() - 1;
        let mut result = [Felt::ZERO; MIN_STACK_DEPTH];
        for (result, column) in result.iter_mut().zip(self.stack.iter()) {
            *result = column[last_step];
        }
        result
    }

    // HELPER FUNCTIONS
    // ================================================================================================

    fn finalize_trace(process: Process) -> (SysTrace, StackTrace, RangeCheckTrace, AuxTableTrace) {
        let Process {
            system,
            decoder: _,
            stack,
            range,
            hasher,
            bitwise,
            memory,
            advice: _,
        } = process;

        // Get the trace length required to hold all execution trace steps.
        let aux_trace_len = hasher.trace_len() + bitwise.trace_len() + memory.trace_len();
        let trace_len = vec![stack.trace_len(), range.trace_len(), aux_trace_len]
            .iter()
            .max()
            .expect("failed to get max of component trace lengths")
            .next_power_of_two();

        // Finalize the system trace.
        let step = system.clk();
        let mut system_trace = system.into_trace();
        finalize_clk_column(&mut system_trace[0], step, trace_len);
        finalize_column(&mut system_trace[1], step, trace_len);

        // Finalize stack trace.
        let mut stack_trace = stack.into_trace();
        for column in stack_trace.iter_mut() {
            finalize_column(column, step, trace_len);
        }

        // Finalize the range check trace.
        let range_check_trace: RangeCheckTrace = range
            .into_trace(trace_len)
            .try_into()
            .expect("failed to convert vector to array");

        // Finalize the auxilliary table trace.
        let mut aux_table = AuxTable::new(trace_len);
        aux_table.fill_columns(hasher, bitwise, memory);
        let aux_table_trace = aux_table.into_trace();

        (
            system_trace,
            stack_trace,
            range_check_trace,
            aux_table_trace,
        )
    }

    // ACCESSORS FOR TESTING
    // --------------------------------------------------------------------------------------------

    #[allow(dead_code)]
    pub fn print(&self) {
        let mut row = [Felt::ZERO; TRACE_WIDTH];
        for i in 0..self.length() {
            self.read_row_into(i, &mut row);
            println!("{:?}", row.iter().map(|v| v.as_int()).collect::<Vec<_>>());
        }
    }

    #[cfg(test)]
    pub fn test_finalize_trace(
        process: Process,
    ) -> (SysTrace, StackTrace, RangeCheckTrace, AuxTableTrace) {
        Self::finalize_trace(process)
    }
}

// TRACE TRAIT IMPLEMENTATION
// ================================================================================================

impl Trace for ExecutionTrace {
    type BaseField = Felt;

    fn width(&self) -> usize {
        TRACE_WIDTH
    }

    fn length(&self) -> usize {
        self.system[0].len()
    }

    fn get(&self, col_idx: usize, row_idx: usize) -> Felt {
        match col_idx {
            i if SYS_TRACE_RANGE.contains(&i) => self.system[i - SYS_TRACE_OFFSET][row_idx],
            i if STACK_TRACE_RANGE.contains(&i) => self.stack[i - STACK_TRACE_OFFSET][row_idx],
            i if RANGE_CHECK_TRACE_RANGE.contains(&i) => {
                self.range[i - RANGE_CHECK_TRACE_OFFSET][row_idx]
            }
            i if AUX_TRACE_RANGE.contains(&i) => self.aux_table[i - AUX_TRACE_OFFSET][row_idx],
            _ => panic!("invalid column index"),
        }
    }

    fn meta(&self) -> &[u8] {
        &self.meta
    }

    fn read_row_into(&self, step: usize, target: &mut [Felt]) {
        for (i, column) in self.system.iter().enumerate() {
            target[i + SYS_TRACE_OFFSET] = column[step];
        }

        for (i, column) in self.stack.iter().enumerate() {
            target[i + STACK_TRACE_OFFSET] = column[step];
        }

        for (i, column) in self.range.iter().enumerate() {
            target[i + RANGE_CHECK_TRACE_OFFSET] = column[step];
        }

        for (i, column) in self.aux_table.iter().enumerate() {
            target[i + AUX_TRACE_OFFSET] = column[step];
        }
    }

    fn into_columns(self) -> Vec<Vec<Felt>> {
        let mut result: Vec<Vec<Felt>> = self.system.into();
        self.stack.into_iter().for_each(|v| result.push(v));
        self.range.into_iter().for_each(|v| result.push(v));
        self.aux_table.into_iter().for_each(|v| result.push(v));
        result
    }
}

// TRACE FRAGMENT
// ================================================================================================

/// TODO: add docs
pub struct TraceFragment<'a> {
    data: Vec<&'a mut [Felt]>,
}

impl<'a> TraceFragment<'a> {
    /// Creates a new TraceFragment with its data allocated to the specified capacity.
    pub fn new(capacity: usize) -> Self {
        TraceFragment {
            data: Vec::with_capacity(capacity),
        }
    }

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

    /// Adds a new column to this fragment by pushing a mutable slice with the first `len`
    /// elements of the provided column. Returns the rest of the provided column as a separate
    /// mutable slice.
    pub fn push_column_slice(&mut self, column: &'a mut [Felt], len: usize) -> &'a mut [Felt] {
        let (column_fragment, rest) = column.split_at_mut(len);
        self.data.push(column_fragment);
        rest
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

/// Copies the final output value down to the end of the stack trace, then extends the column to
/// the length of the execution trace, if it's longer than the stack trace, and copies the last
/// value to the end of that as well.
fn finalize_column(column: &mut Vec<Felt>, step: usize, trace_len: usize) {
    let last_value = column[step];
    column[step..].fill(last_value);
    column.resize(trace_len, last_value);
}

/// Completes the clk column by filling in all values after the specified step. The values
/// in the clk column are equal to the index of the row in the trace table.
fn finalize_clk_column(column: &mut Vec<Felt>, step: usize, trace_len: usize) {
    column.resize(trace_len, Felt::ZERO);
    for (i, clk) in column.iter_mut().enumerate().take(trace_len).skip(step) {
        // converting from u32 is OK here because max trace length is 2^32
        *clk = Felt::from(i as u32);
    }
}
