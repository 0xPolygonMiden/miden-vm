use super::{
    AuxTableTrace, Bitwise, Felt, FieldElement, Hasher, Memory, RangeChecker, TraceFragment, Vec,
    AUX_TABLE_WIDTH,
};
use crate::{
    aux_table_bus::{AuxTableBus, AuxTraceBuilder as AuxTableAuxTraceBuilder},
    hasher::AuxTraceBuilder as HasherAuxTraceBuilder,
};

#[cfg(test)]
mod tests;

// AUXILIARY TABLE
// ================================================================================================

/// Auxiliary table for the VM's hasher, bitwise, and memory co-processor components.
///
/// This component is responsible for building a final execution trace from the stacked coprocessor
/// execution traces.
///
/// The auxiliary trace table can be thought of as 4 stacked segments in the following form:
/// * Hasher segment: contains the hasher trace and selector *
/// This segment fills the first rows of the table up to the length of the hasher `trace_len`.
/// - column 0: selector column with values set to ZERO
/// - columns 1-17: execution trace of hash co-processor
///
/// * Bitwise segment: contains the bitwise trace and selectors *
/// This segment begins at the end of the hasher segment and fills the next rows of the table for
/// the `trace_len` of the bitwise coprocessor.
/// - column 0: selector column with values set to ONE
/// - column 1: selector column with values set to ZERO
/// - columns 2-15: execution trace of bitwise coprocessor
/// - column 16-17: unused columns padded with ZERO
///
/// * Padding segment: unused *
/// This segment begins at the end of the bitwise segment and fills as many rows in the table as
/// required to ensure that the next and final segment of the trace, containing the memory, will end
/// at the full length of the execution trace minus the number of random rows.
/// - column 0-1: selector columns with values set to ONE
/// - column 2: selector column with values set to ZERO
/// - columns 3-17: unused columns padded with ZERO
///
/// * Memory segment: contains the memory trace and selectors *
/// This segment begins at the end of the padding segment and fills the next rows of the table for
/// the `trace_len` of the memory coprocessor. When it finishes, the execution trace should have
/// exactly enough rows remaining for the specified number of random rows. The memory segment is
/// placed at the end of the auxiliary table to simplify transition constraints.
/// - columns 0-2: selector columns with values set to ONE
/// - columns 3-16: execution trace of memory coprocessor
/// - column 17: unused column padded with ZERO
///
pub struct AuxTable {
    hasher: Hasher,
    bitwise: Bitwise,
    memory: Memory,
    aux_table_bus: AuxTableBus,
}

impl AuxTable {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns an [AuxTable] initialized with its co-processor components.
    pub fn new(
        hasher: Hasher,
        bitwise: Bitwise,
        memory: Memory,
        aux_table_bus: AuxTableBus,
    ) -> Self {
        Self {
            hasher,
            bitwise,
            memory,
            aux_table_bus,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the length of the trace required to accommodate co-processor components and 1
    /// mandatory padding row required for ensuring sufficient trace length for auxiliary connector
    /// columns that rely on the memory co-processor.
    pub fn trace_len(&self) -> usize {
        self.hasher.trace_len() + self.bitwise.trace_len() + self.memory.trace_len() + 1
    }

    /// Returns the index of the first row of the [Memory] execution trace.
    pub fn memory_start(&self) -> usize {
        self.hasher.trace_len() + self.bitwise.trace_len()
    }

    /// Adds all range checks required by the memory co-processor to the provided [RangeChecker]
    /// instance, along with the cycle rows at which the processor performs the lookups.
    pub fn append_range_checks(&self, range_checker: &mut RangeChecker) {
        self.memory
            .append_range_checks(self.memory_start(), range_checker);
    }

    /// Returns an execution trace of the auxiliary table containing the stacked traces of the
    /// Hasher, Bitwise, and Memory coprocessors.
    ///
    /// `num_rand_rows` indicates the number of rows at the end of the trace which will be
    /// overwritten with random values.
    pub fn into_trace(self, trace_len: usize, num_rand_rows: usize) -> AuxTableTrace {
        // make sure that only padding rows will be overwritten by random values
        assert!(
            self.trace_len() + num_rand_rows <= trace_len,
            "target trace length too small"
        );

        // Allocate columns for the trace of the auxiliary table.
        // note: it may be possible to optimize this by initializing with Felt::zeroed_vector,
        // depending on how the compiler reduces Felt(0) and whether initializing here + iterating
        // to update selector values is faster than using resize to initialize all values
        let mut trace = (0..AUX_TABLE_WIDTH)
            .map(|_| Vec::<Felt>::with_capacity(trace_len))
            .collect::<Vec<_>>()
            .try_into()
            .expect("failed to convert vector to array");

        let (hasher_aux_builder, aux_builder) = self.fill_trace(&mut trace, trace_len);

        AuxTableTrace {
            trace,
            hasher_aux_builder,
            aux_builder,
        }
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Fills the provided auxiliary table trace with the stacked execution traces of the Hasher,
    /// Bitwise, and Memory coprocessors, along with selector columns to identify each coprocessor
    /// trace and padding to fill the rest of the table.
    ///
    /// It returns the auxiilary trace builders for generating auxiliary trace columns that depend
    /// on data from [AuxTable] co-processors.
    fn fill_trace(
        self,
        trace: &mut [Vec<Felt>; AUX_TABLE_WIDTH],
        trace_len: usize,
    ) -> (HasherAuxTraceBuilder, AuxTableAuxTraceBuilder) {
        // get the row where the memory segment begins before destructuring.
        let memory_start = self.memory_start();
        let AuxTable {
            hasher,
            bitwise,
            memory,
            mut aux_table_bus,
        } = self;

        // allocate fragments to be filled with the respective execution traces of each coprocessor
        let mut hasher_fragment = TraceFragment::new(AUX_TABLE_WIDTH);
        let mut bitwise_fragment = TraceFragment::new(AUX_TABLE_WIDTH);
        let mut memory_fragment = TraceFragment::new(AUX_TABLE_WIDTH);

        // set the selectors and padding as required by each column and segment
        // and add the hasher, bitwise, and memory segments to their respective fragments
        // so they can be filled with the coprocessor traces
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

        // fill the fragments with the execution trace from each coprocessor
        // TODO: this can be parallelized to fill the traces in multiple threads
        let hasher_aux_builder = hasher.fill_trace(&mut hasher_fragment);
        bitwise.fill_trace(&mut bitwise_fragment);
        memory.fill_trace(&mut memory_fragment, memory_start, &mut aux_table_bus);

        (hasher_aux_builder, aux_table_bus.into_aux_builder())
    }
}
