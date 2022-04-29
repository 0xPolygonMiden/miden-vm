use super::{
    AuxTable, AuxTableTrace, Digest, Felt, FieldElement, Process, RangeCheckTrace, StackTopState,
    StackTrace, SysTrace,
};
use core::slice;
use vm_core::{StarkField, MIN_STACK_DEPTH, STACK_TRACE_OFFSET, TRACE_WIDTH};
use winterfell::Trace;
use winterfell::{EvaluationFrame, Matrix, TraceLayout};

// VM EXECUTION TRACE
// ================================================================================================

/// TODO: for now this consists only of system register trace, stack trace, range check trace, and
/// auxiliary table trace, but will also need to include the decoder trace.
pub struct ExecutionTrace {
    meta: Vec<u8>,
    layout: TraceLayout,
    main_trace: Matrix<Felt>,
    // TODO: program hash should be retrieved from decoder trace, but for now we store it explicitly
    program_hash: Digest,
}

impl ExecutionTrace {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Builds an execution trace for the provided process.
    pub(super) fn new(process: Process, program_hash: Digest) -> Self {
        let (system_trace, stack_trace, range_check_trace, aux_table_trace) =
            finalize_trace(process);

        let main_trace = system_trace
            .into_iter()
            .chain(stack_trace)
            .chain(range_check_trace)
            .chain(aux_table_trace)
            .collect::<Vec<_>>();

        Self {
            meta: Vec::new(),
            layout: TraceLayout::new(TRACE_WIDTH, [0], [0]),
            main_trace: Matrix::new(main_trace),
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

    /// Returns the initial state of the top 16 stack registers.
    pub fn init_stack_state(&self) -> StackTopState {
        let mut result = [Felt::ZERO; MIN_STACK_DEPTH];
        for (i, result) in result.iter_mut().enumerate() {
            *result = self.main_trace.get_column(i + STACK_TRACE_OFFSET)[0];
        }
        result
    }

    /// Returns the final state of the top 16 stack registers.
    pub fn last_stack_state(&self) -> StackTopState {
        let last_step = self.length() - 1;
        let mut result = [Felt::ZERO; MIN_STACK_DEPTH];
        for (i, result) in result.iter_mut().enumerate() {
            *result = self.main_trace.get_column(i + STACK_TRACE_OFFSET)[last_step];
        }
        result
    }

    // TEST HELPERS
    // --------------------------------------------------------------------------------------------

    #[allow(dead_code)]
    pub fn print(&self) {
        let mut row = [Felt::ZERO; TRACE_WIDTH];
        for i in 0..self.length() {
            self.main_trace.read_row_into(i, &mut row);
            println!("{:?}", row.iter().map(|v| v.as_int()).collect::<Vec<_>>());
        }
    }

    #[cfg(test)]
    pub fn test_finalize_trace(
        process: Process,
    ) -> (SysTrace, StackTrace, RangeCheckTrace, AuxTableTrace) {
        finalize_trace(process)
    }
}

// TRACE TRAIT IMPLEMENTATION
// ================================================================================================

impl Trace for ExecutionTrace {
    type BaseField = Felt;

    fn layout(&self) -> &TraceLayout {
        &self.layout
    }

    fn length(&self) -> usize {
        self.main_trace.num_rows()
    }

    fn meta(&self) -> &[u8] {
        &self.meta
    }

    fn main_segment(&self) -> &Matrix<Felt> {
        &self.main_trace
    }

    fn build_aux_segment<E: FieldElement<BaseField = Felt>>(
        &mut self,
        _aux_segments: &[Matrix<E>],
        _rand_elements: &[E],
    ) -> Option<Matrix<E>> {
        // TODO: implement
        unimplemented!()
    }

    fn read_main_frame(&self, row_idx: usize, frame: &mut EvaluationFrame<Felt>) {
        let next_row_idx = (row_idx + 1) % self.length();
        self.main_trace.read_row_into(row_idx, frame.current_mut());
        self.main_trace
            .read_row_into(next_row_idx, frame.next_mut());
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

fn finalize_trace(process: Process) -> (SysTrace, StackTrace, RangeCheckTrace, AuxTableTrace) {
    let Process {
        system,
        decoder: _,
        stack,
        range,
        hasher,
        bitwise,
        memory,
        pow2: _,
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
