use super::{
    Bitwise, ChipletsTrace, Felt, FieldElement, Hasher, Memory, RangeChecker, TraceFragment, Vec,
    CHIPLETS_WIDTH,
};
use crate::{
    chiplets_bus::{AuxTraceBuilder as ChipletsAuxTraceBuilder, ChipletsBus},
    hasher::AuxTraceBuilder as HasherAuxTraceBuilder,
};

#[cfg(test)]
mod tests;

// CHIPLETS MODULE
// ================================================================================================

/// A module containing the VM's hasher, bitwise, and memory chiplet components.
///
/// This component is responsible for building a final execution trace from the stacked chiplet
/// execution traces.
///
/// The module's trace can be thought of as 4 stacked chiplet segments in the following form:
/// * Hasher segment: contains the trace and selector for the hasher chiplet *
/// This segment fills the first rows of the trace up to the length of the hasher `trace_len`.
/// - column 0: selector column with values set to ZERO
/// - columns 1-17: execution trace of hash chiplet
///
/// * Bitwise segment: contains the trace and selectors for the bitwise chiplet *
/// This segment begins at the end of the hasher segment and fills the next rows of the trace for
/// the `trace_len` of the bitwise chiplet.
/// - column 0: selector column with values set to ONE
/// - column 1: selector column with values set to ZERO
/// - columns 2-15: execution trace of bitwise chiplet
/// - column 16-17: unused columns padded with ZERO
///
/// * Memory segment: contains the trace and selectors for the memory chiplet *
/// This segment begins at the end of the bitwise segment and fills the next rows of the trace for
/// the `trace_len` of the memory chiplet.
/// - column 0-1: selector columns with values set to ONE
/// - column 2: selector column with values set to ZERO
/// - columns 3-16: execution trace of memory chiplet
/// - column 17: unused column padded with ZERO
///
/// * Padding segment: unused *
/// This segment begins at the end of the memory segment and fills the rest of the execution trace
/// minus the number of random rows. When it finishes, the execution trace should have exactly
/// enough rows remaining for the specified number of random rows.
/// - columns 0-2: selector columns with values set to ONE
/// - columns 3-17: unused columns padded with ZERO
///
pub struct Chiplets {
    hasher: Hasher,
    bitwise: Bitwise,
    memory: Memory,
    chiplets_bus: ChipletsBus,
}

impl Chiplets {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a [Chiplets] module initialized with its chiplet components.
    pub fn new(
        hasher: Hasher,
        bitwise: Bitwise,
        memory: Memory,
        chiplets_bus: ChipletsBus,
    ) -> Self {
        Self {
            hasher,
            bitwise,
            memory,
            chiplets_bus,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the length of the trace required to accommodate chiplet components and 1
    /// mandatory padding row required for ensuring sufficient trace length for auxiliary connector
    /// columns that rely on the memory chiplet.
    pub fn trace_len(&self) -> usize {
        self.hasher.trace_len() + self.bitwise.trace_len() + self.memory.trace_len() + 1
    }

    /// Returns the index of the first row of the [Memory] execution trace.
    pub fn memory_start(&self) -> usize {
        self.hasher.trace_len() + self.bitwise.trace_len()
    }

    /// Adds all range checks required by the memory chiplet to the provided [RangeChecker]
    /// instance, along with the cycle rows at which the processor performs the lookups.
    pub fn append_range_checks(&self, range_checker: &mut RangeChecker) {
        self.memory
            .append_range_checks(self.memory_start(), range_checker);
    }

    /// Returns an execution trace of the chiplets containing the stacked traces of the
    /// Hasher, Bitwise, and Memory chiplets.
    ///
    /// `num_rand_rows` indicates the number of rows at the end of the trace which will be
    /// overwritten with random values.
    pub fn into_trace(self, trace_len: usize, num_rand_rows: usize) -> ChipletsTrace {
        // make sure that only padding rows will be overwritten by random values
        assert!(
            self.trace_len() + num_rand_rows <= trace_len,
            "target trace length too small"
        );

        // Allocate columns for the trace of the chiplets.
        // note: it may be possible to optimize this by initializing with Felt::zeroed_vector,
        // depending on how the compiler reduces Felt(0) and whether initializing here + iterating
        // to update selector values is faster than using resize to initialize all values
        let mut trace = (0..CHIPLETS_WIDTH)
            .map(|_| Vec::<Felt>::with_capacity(trace_len))
            .collect::<Vec<_>>()
            .try_into()
            .expect("failed to convert vector to array");

        let (hasher_aux_builder, aux_builder) = self.fill_trace(&mut trace, trace_len);

        ChipletsTrace {
            trace,
            hasher_aux_builder,
            aux_builder,
        }
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Fills the provided trace for the chiplets module with the stacked execution traces of the
    /// Hasher, Bitwise, and Memory chiplets, along with selector columns to identify each chiplet
    /// trace and padding to fill the rest of the trace.
    ///
    /// It returns the auxiilary trace builders for generating auxiliary trace columns that depend
    /// on data from [Chiplets].
    fn fill_trace(
        self,
        trace: &mut [Vec<Felt>; CHIPLETS_WIDTH],
        trace_len: usize,
    ) -> (HasherAuxTraceBuilder, ChipletsAuxTraceBuilder) {
        // get the row where the memory segment begins before destructuring.
        let memory_start = self.memory_start();
        let Chiplets {
            hasher,
            bitwise,
            memory,
            mut chiplets_bus,
        } = self;

        // allocate fragments to be filled with the respective execution traces of each chiplet
        let mut hasher_fragment = TraceFragment::new(CHIPLETS_WIDTH);
        let mut bitwise_fragment = TraceFragment::new(CHIPLETS_WIDTH);
        let mut memory_fragment = TraceFragment::new(CHIPLETS_WIDTH);

        // set the selectors and padding as required by each column and segment
        // and add the hasher, bitwise, and memory segments to their respective fragments
        // so they can be filled with the chiplet traces
        for (column_num, column) in trace.iter_mut().enumerate() {
            match column_num {
                0 => {
                    // set the selector value for the hasher segment to ZERO
                    column.resize(hasher.trace_len(), Felt::ZERO);
                    // set the selector value for all other segments ONE
                    column.resize(trace_len, Felt::ONE);
                }
                1 => {
                    // initialize hasher segment and set bitwise segment selector value to ZERO
                    column.resize(hasher.trace_len() + bitwise.trace_len(), Felt::ZERO);
                    // set selector value for all other segments to ONE
                    column.resize(trace_len, Felt::ONE);
                    // add hasher segment to the hasher fragment to be filled from the hasher trace
                    hasher_fragment.push_column_slice(column, hasher.trace_len());
                }
                2 => {
                    // initialize hasher and bitwise segments and set memory segment selector to ZERO
                    column.resize(
                        hasher.trace_len() + bitwise.trace_len() + memory.trace_len(),
                        Felt::ZERO,
                    );
                    // set selector value for the final segment to ONE
                    column.resize(trace_len, Felt::ONE);
                    // add hasher segment to the hasher fragment to be filled from the hasher trace
                    let rest_of_column =
                        hasher_fragment.push_column_slice(column, hasher.trace_len());
                    // add bitwise segment to the bitwise fragment to be filled from the bitwise trace
                    bitwise_fragment.push_column_slice(rest_of_column, bitwise.trace_len());
                }
                16 => {
                    // initialize hasher & memory segments and bitwise, padding segments with ZERO
                    column.resize(trace_len, Felt::ZERO);
                    // add hasher segment to the hasher fragment to be filled from the hasher trace
                    let rest_of_column =
                        hasher_fragment.push_column_slice(column, hasher.trace_len());
                    // split the column to skip bitwise which have already been padded.
                    let (_, rest_of_column) = rest_of_column.split_at_mut(bitwise.trace_len());
                    // add memory segment to the memory fragment to be filled from the memory trace
                    memory_fragment.push_column_slice(rest_of_column, memory.trace_len());
                }
                17 => {
                    // initialize hasher segment and pad bitwise, memory, padding segments with ZERO
                    column.resize(trace_len, Felt::ZERO);
                    // add hasher segment to the hasher fragment to be filled from the hasher trace
                    hasher_fragment.push_column_slice(column, hasher.trace_len());
                }
                _ => {
                    // initialize hasher, bitwise, memory segments and pad the rest with ZERO
                    column.resize(trace_len, Felt::ZERO);
                    // add hasher segment to the hasher fragment to be filled from the hasher trace
                    let rest_of_column =
                        hasher_fragment.push_column_slice(column, hasher.trace_len());
                    // add bitwise segment to the bitwise fragment to be filled from the bitwise trace
                    let rest_of_column =
                        bitwise_fragment.push_column_slice(rest_of_column, bitwise.trace_len());
                    // add memory segment to the memory fragment to be filled from the memory trace
                    memory_fragment.push_column_slice(rest_of_column, memory.trace_len());
                }
            }
        }

        // fill the fragments with the execution trace from each chiplet
        // TODO: this can be parallelized to fill the traces in multiple threads
        let hasher_aux_builder = hasher.fill_trace(&mut hasher_fragment);
        bitwise.fill_trace(&mut bitwise_fragment);
        memory.fill_trace(&mut memory_fragment, memory_start, &mut chiplets_bus);

        (hasher_aux_builder, chiplets_bus.into_aux_builder())
    }
}
