use alloc::vec::Vec;

use miden_air::trace::{
    decoder::{NUM_USER_OP_HELPERS, USER_OP_HELPERS_OFFSET},
    main_trace::MainTrace,
    range::RANGE_CHECKER_TRACE_WIDTH,
    AUX_TRACE_RAND_ELEMENTS, AUX_TRACE_WIDTH, CHIPLETS_WIDTH, DECODER_TRACE_OFFSET,
    DECODER_TRACE_WIDTH, MIN_TRACE_LEN, STACK_TRACE_OFFSET, STACK_TRACE_WIDTH, SYS_TRACE_WIDTH,
    TRACE_WIDTH,
};
use vm_core::{
    stack::MIN_STACK_DEPTH,
    utils::{range, uninit_vector},
    ProgramInfo, StackInputs, StackOutputs,
};
use winter_prover::{crypto::RandomCoin, EvaluationFrame, Trace, TraceInfo};

use super::{
    chiplets::AuxTraceBuilder as ChipletsAuxTraceBuilder, crypto::RpoRandomCoin,
    decoder::AuxTraceBuilder as DecoderAuxTraceBuilder,
    range::AuxTraceBuilder as RangeCheckerAuxTraceBuilder,
    stack::AuxTraceBuilder as StackAuxTraceBuilder, ColMatrix, Digest, Felt, FieldElement, Process,
};

mod utils;
pub use utils::{AuxColumnBuilder, ChipletsLengths, TraceFragment, TraceLenSummary};

#[cfg(test)]
mod tests;
#[cfg(test)]
use super::EMPTY_WORD;

// CONSTANTS
// ================================================================================================

/// Number of rows at the end of an execution trace which are injected with random values.
pub const NUM_RAND_ROWS: usize = 1;

// VM EXECUTION TRACE
// ================================================================================================

pub struct AuxTraceBuilders {
    pub(crate) decoder: DecoderAuxTraceBuilder,
    pub(crate) stack: StackAuxTraceBuilder,
    pub(crate) range: RangeCheckerAuxTraceBuilder,
    pub(crate) chiplets: ChipletsAuxTraceBuilder,
}

/// Execution trace which is generated when a program is executed on the VM.
///
/// The trace consists of the following components:
/// - Main traces of System, Decoder, Operand Stack, Range Checker, and Auxiliary Co-Processor
///   components.
/// - Hints used during auxiliary trace segment construction.
/// - Metadata needed by the STARK prover.
pub struct ExecutionTrace {
    meta: Vec<u8>,
    trace_info: TraceInfo,
    main_trace: MainTrace,
    aux_trace_builders: AuxTraceBuilders,
    program_info: ProgramInfo,
    stack_outputs: StackOutputs,
    trace_len_summary: TraceLenSummary,
}

impl ExecutionTrace {
    // CONSTANTS
    // --------------------------------------------------------------------------------------------

    /// Number of rows at the end of an execution trace which are injected with random values.
    pub const NUM_RAND_ROWS: usize = NUM_RAND_ROWS;

    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Builds an execution trace for the provided process.
    pub fn new(process: Process, stack_outputs: StackOutputs) -> Self {
        // use program hash to initialize random element generator; this generator will be used
        // to inject random values at the end of the trace; using program hash here is OK because
        // we are using random values only to stabilize constraint degrees, and not to achieve
        // perfect zero knowledge.
        let program_hash = process.decoder.program_hash();
        let rng = RpoRandomCoin::new(program_hash);

        // create a new program info instance with the underlying kernel
        let kernel = process.kernel().clone();
        let program_info = ProgramInfo::new(program_hash.into(), kernel);
        let (main_trace, aux_trace_builders, trace_len_summary) = finalize_trace(process, rng);
        let trace_info = TraceInfo::new_multi_segment(
            TRACE_WIDTH,
            AUX_TRACE_WIDTH,
            AUX_TRACE_RAND_ELEMENTS,
            main_trace.num_rows(),
            vec![],
        );

        Self {
            meta: Vec::new(),
            trace_info,
            aux_trace_builders,
            main_trace,
            program_info,
            stack_outputs,
            trace_len_summary,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the program info of this execution trace.
    pub fn program_info(&self) -> &ProgramInfo {
        &self.program_info
    }

    /// Returns hash of the program execution of which resulted in this execution trace.
    pub fn program_hash(&self) -> &Digest {
        self.program_info.program_hash()
    }

    /// Returns outputs of the program execution which resulted in this execution trace.
    pub fn stack_outputs(&self) -> &StackOutputs {
        &self.stack_outputs
    }

    /// Returns the initial state of the top 16 stack registers.
    pub fn init_stack_state(&self) -> StackInputs {
        let first_row_stack =
            &self.main_trace.get_row(0.into())[range(STACK_TRACE_OFFSET, MIN_STACK_DEPTH)];
        let first_row_stack: [Felt; MIN_STACK_DEPTH] = first_row_stack.try_into().unwrap();

        first_row_stack.into()
    }

    /// Returns the final state of the top 16 stack registers.
    pub fn last_stack_state(&self) -> StackOutputs {
        let last_step = self.last_step();

        let stack_state =
            &self.main_trace.get_row(last_step.into())[range(STACK_TRACE_OFFSET, MIN_STACK_DEPTH)];
        let stack_state: [Felt; MIN_STACK_DEPTH] = stack_state.try_into().unwrap();
        stack_state.into()
    }

    /// Returns helper registers state at the specified `clk` of the VM
    pub fn get_user_op_helpers_at(&self, clk: u32) -> [Felt; NUM_USER_OP_HELPERS] {
        let user_op_helpers = &self.main_trace.get_row(clk.into())
            [range(DECODER_TRACE_OFFSET + USER_OP_HELPERS_OFFSET, NUM_USER_OP_HELPERS)];
        let user_op_helpers: [Felt; NUM_USER_OP_HELPERS] = user_op_helpers.try_into().unwrap();

        user_op_helpers
    }

    /// Returns the trace length.
    pub fn get_trace_len(&self) -> usize {
        self.main_trace.num_rows()
    }

    /// Returns a summary of the lengths of main, range and chiplet traces.
    pub fn trace_len_summary(&self) -> &TraceLenSummary {
        &self.trace_len_summary
    }

    /// Returns the trace meta data.
    pub fn meta(&self) -> &[u8] {
        &self.meta
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Returns the index of the last row in the trace.
    fn last_step(&self) -> usize {
        self.length() - NUM_RAND_ROWS - 1
    }

    // TEST HELPERS
    // --------------------------------------------------------------------------------------------
    #[cfg(feature = "std")]
    #[allow(dead_code)]
    pub fn print(&self) {
        for row_idx in 0..self.length() {
            let row = self.main_trace.get_row(row_idx.into());
            std::println!("{:?}", row.iter().map(|v| v.as_int()).collect::<Vec<_>>());
        }
    }

    #[cfg(test)]
    pub fn test_finalize_trace(process: Process) -> (MainTrace, AuxTraceBuilders, TraceLenSummary) {
        let rng = RpoRandomCoin::new(EMPTY_WORD);
        finalize_trace(process, rng)
    }

    pub fn build_aux_trace<E>(&self, rand_elements: &[E]) -> Option<ColMatrix<E>>
    where
        E: FieldElement<BaseField = Felt>,
    {
        // add decoder's running product columns
        let decoder_aux_columns = self
            .aux_trace_builders
            .decoder
            .build_aux_columns(&self.main_trace, rand_elements);

        // add stack's running product columns
        let stack_aux_columns =
            self.aux_trace_builders.stack.build_aux_columns(&self.main_trace, rand_elements);

        // add the range checker's running product columns
        let range_aux_columns =
            self.aux_trace_builders.range.build_aux_columns(&self.main_trace, rand_elements);

        // add the running product columns for the chiplets
        let chiplets = self
            .aux_trace_builders
            .chiplets
            .build_aux_columns(&self.main_trace, rand_elements);

        // combine all auxiliary columns into a single vector
        let mut aux_columns = decoder_aux_columns
            .into_iter()
            .chain(stack_aux_columns)
            .chain(range_aux_columns)
            .chain(chiplets)
            .collect::<Vec<_>>();

        // inject random values into the last rows of the trace
        let mut rng = RpoRandomCoin::new(self.program_hash().into());
        for i in self.length() - NUM_RAND_ROWS..self.length() {
            for column in aux_columns.iter_mut() {
                column[i] = rng.draw().expect("failed to draw a random value");
            }
        }

        Some(ColMatrix::new(aux_columns))
    }
}

// TRACE TRAIT IMPLEMENTATION
// ================================================================================================

impl Trace for ExecutionTrace {
    type BaseField = Felt;

    fn length(&self) -> usize {
        self.main_trace.num_rows()
    }

    fn main_segment(&self) -> &ColMatrix<Felt> {
        // TODO(plafer): Measure the cost of generating the col matrix as discussed with Bobbin.
        // Note: Switching to RowMatrix will be quite expensive in Winterfell, as we need to change
        // the GPU impls. Also, the FFT assumes column major, but does a segment-wise transpose. So
        // maybe it would benefit from a row major matrix.
        todo!()
    }

    fn read_main_frame(&self, row_idx: usize, frame: &mut EvaluationFrame<Felt>) {
        let next_row_idx = (row_idx + 1) % self.length();
        frame.current_mut().copy_from_slice(&self.main_trace.get_row(row_idx.into()));
        frame.next_mut().copy_from_slice(&self.main_trace.get_row(next_row_idx.into()));
    }

    fn info(&self) -> &TraceInfo {
        &self.trace_info
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Converts a process into a set of execution trace columns for each component of the trace.
///
/// The process includes:
/// - Determining the length of the trace required to accommodate the longest trace column.
/// - Padding the columns to make sure all columns are of the same length.
/// - Inserting random values in the last row of all columns. This helps ensure that there are no
///   repeating patterns in each column and each column contains a least two distinct values. This,
///   in turn, ensures that polynomial degrees of all columns are stable.
fn finalize_trace(
    process: Process,
    mut rng: RpoRandomCoin,
) -> (MainTrace, AuxTraceBuilders, TraceLenSummary) {
    let (system, decoder, stack, mut range, chiplets) = process.into_parts();

    let clk = system.clk();

    // Trace lengths of system and stack components must be equal to the number of executed cycles
    assert_eq!(clk.as_usize(), system.trace_len(), "inconsistent system trace lengths");
    assert_eq!(clk.as_usize(), decoder.trace_len(), "inconsistent decoder trace length");
    assert_eq!(clk.as_usize(), stack.trace_len(), "inconsistent stack trace lengths");

    // Add the range checks required by the chiplets to the range checker.
    chiplets.append_range_checks(&mut range);

    // Generate number of rows for the range trace.
    let range_table_len = range.get_number_range_checker_rows();

    // Get the trace length required to hold all execution trace steps.
    let max_len = range_table_len.max(clk.into()).max(chiplets.trace_len());

    // Pad the trace length to the next power of two and ensure that there is space for the
    // Rows to hold random values
    let trace_len = (max_len + NUM_RAND_ROWS).next_power_of_two();
    assert!(
        trace_len >= MIN_TRACE_LEN,
        "trace length must be at least {MIN_TRACE_LEN}, but was {trace_len}",
    );

    // Get the lengths of the traces: main, range, and chiplets
    let trace_len_summary =
        TraceLenSummary::new(clk.into(), range_table_len, ChipletsLengths::new(&chiplets));

    // TODO(plafer): implement similar to Memory
    let range_check_trace =
        range.into_trace_with_table_row_major(range_table_len, trace_len, NUM_RAND_ROWS);

    // TODO(plafer): convert all `write_row` methods to take it in the entire `main_trace`, so that
    // they are allowed to read from previous rows.

    // TODO(plafer): to avoid having `if` statements in the `write_row` methods, we run this loop
    // over `own_len`. Then, separately fill out the range checker, and chiplets traces.

    //Combine all trace segments into the main trace
    let mut main_trace: Vec<Felt> = unsafe { uninit_vector(TRACE_WIDTH * trace_len) };
    for row_idx in 0..trace_len {
        let mut start_idx = row_idx * TRACE_WIDTH;

        // System trace
        system.write_row(row_idx, &mut main_trace[start_idx..start_idx + SYS_TRACE_WIDTH]);
        start_idx += SYS_TRACE_WIDTH;

        // Decoder trace
        decoder.write_row(row_idx, &mut main_trace[start_idx..start_idx + DECODER_TRACE_WIDTH]);
        start_idx += DECODER_TRACE_WIDTH;

        // Stack trace
        stack.write_row(row_idx, &mut main_trace);
        start_idx += STACK_TRACE_WIDTH;

        // TODO(plafer): Modify range checker to write directly into the main trace (just like
        // Memory and kernel ROM) Range checker trace
        range_check_trace
            .write_row(row_idx, &mut main_trace[start_idx..start_idx + RANGE_CHECKER_TRACE_WIDTH]);
        start_idx += RANGE_CHECKER_TRACE_WIDTH;

        // TODO(plafer): refactor to make cleaner?
        // Chiplets trace
        // Note: The memory and kernel ROM chiplets write their traces directly into the main trace
        // (i.e. not from this method).
        chiplets.write_row(row_idx, &mut main_trace[start_idx..start_idx + CHIPLETS_WIDTH]);
    }

    // Note: The memory and kernel ROM chiplets write their traces directly into the main trace.
    chiplets.memory().fill_trace_2(&mut main_trace, chiplets.memory_start());
    chiplets.kernel_rom.fill_trace_2(&mut main_trace, chiplets.kernel_rom_start());

    // Inject random values into the last rows of the trace
    {
        debug_assert_eq!(NUM_RAND_ROWS, 1);
        let last_row_start = (trace_len - 1) * TRACE_WIDTH;
        for i in 0..TRACE_WIDTH {
            main_trace[last_row_start + i] = rng.draw().expect("failed to draw a random value");
        }
    }

    let aux_trace_hints = AuxTraceBuilders {
        decoder: DecoderAuxTraceBuilder::default(),
        stack: StackAuxTraceBuilder,
        range: range_check_trace.aux_builder,
        chiplets: ChipletsAuxTraceBuilder::new(chiplets.kernel().clone()),
    };

    let main_trace = MainTrace::new(main_trace, clk);

    (main_trace, aux_trace_hints, trace_len_summary)
}
