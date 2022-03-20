use super::{
    AuxTableTrace, Bitwise, Felt, FieldElement, Hasher, Memory, TraceFragment, AUX_TRACE_WIDTH,
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
/// - columns 1-17: execution trace of hasher coprocessor
///
/// * Bitwise segment: contains the bitwise trace and selectors *
/// This segment begins at the end of the hasher segment and fills the next rows of the table for
/// the `trace_len` of the bitwise coprocessor.
/// - column 0: selector column with values set to ONE
/// - column 1: selector column with values set to ZERO
/// - columns 2-14: execution trace of bitwise coprocessor
/// - columns 15-17: unused columns padded with ZERO
///
/// * Memory segment: contains the memory trace and selectors *
/// This segment begins at the end of the bitwise segment and fills the next rows of the table for
/// the `trace_len` of the memory coprocessor.
/// - column 0-1: selector columns with values set to ONE
/// - column 2: selector column with values set to ZERO
/// - columns 3-16: execution trace of memory coprocessor
/// - column 17: unused column padded with ZERO
///
/// * Final segment: unused *
/// This segment begins at the end of the memory segment and fills the rest of the rows in the table
/// up to the full length of the execution trace.
/// - columns 0-2: selector columns with values set to ONE
/// - columns 3-17: unused columns padded with ZERO
///
pub struct AuxTable {
    trace: AuxTableTrace,
}

impl AuxTable {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Returns an [AuxTable] initialized with its co-processor components and an empty trace.
    pub fn new(target_length: usize) -> Self {
        // Allocate columns for the trace of the auxiliary table.
        // note: it may be possible to optimize this by initializing with Felt::zeroed_vector,
        // depending on how the compiler reduces Felt(0) and whether initializing here + iterating
        // to update selector values is faster than using resize to initialize all values
        let trace = (0..AUX_TRACE_WIDTH)
            .map(|_| Vec::<Felt>::with_capacity(target_length))
            .collect::<Vec<_>>()
            .try_into()
            .expect("failed to convert vector to array");

        Self { trace }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns an execution trace of the auxiliary table containing the stacked traces of the
    /// Hasher, Bitwise, and Memory coprocessors.
    pub fn into_trace(self) -> AuxTableTrace {
        self.trace
    }

    // TRACE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Fills the provided auxiliary table trace with the stacked execution traces of the Hasher,
    /// Bitwise, and Memory coprocessors, along with selector columns to identify each coprocessor
    /// trace and padding to fill the rest of the table.
    pub fn fill_columns(&mut self, hasher: Hasher, bitwise: Bitwise, memory: Memory) {
        let trace_len = self.trace[0].capacity();

        // allocate fragments to be filled with the respective execution traces of each coprocessor
        let mut hasher_fragment = TraceFragment::new(AUX_TRACE_WIDTH);
        let mut bitwise_fragment = TraceFragment::new(AUX_TRACE_WIDTH);
        let mut memory_fragment = TraceFragment::new(AUX_TRACE_WIDTH);

        // set the selectors and padding as required by each column and segment
        // and add the hasher, bitwise, and memory segments to their respective fragments
        // so they can be filled with the coprocessor traces
        for (column_num, column) in self.trace.iter_mut().enumerate() {
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
                15 | 16 => {
                    // initialize hasher & memory segments and pad bitwise & final segments with ZERO
                    column.resize(trace_len, Felt::ZERO);
                    // add hasher segment to the hasher fragment to be filled from the hasher trace
                    let rest_of_column =
                        hasher_fragment.push_column_slice(column, hasher.trace_len());
                    // split the column again to skip the bitwise segment, which has already been padded
                    let (_, rest_of_column) = rest_of_column.split_at_mut(bitwise.trace_len());
                    // add memory segment to the memory fragment to be filled from the memory trace
                    memory_fragment.push_column_slice(rest_of_column, memory.trace_len());
                }
                17 => {
                    // initialize hasher segment and pad bitwise, memory, and final segments with ZERO
                    column.resize(trace_len, Felt::ZERO);
                    // add hasher segment to the hasher fragment to be filled from the hasher trace
                    hasher_fragment.push_column_slice(column, hasher.trace_len());
                }
                _ => {
                    // initialize hasher, bitwise, memory segments and pad the final segment with ZERO
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
        hasher.fill_trace(&mut hasher_fragment);
        bitwise.fill_trace(&mut bitwise_fragment);
        memory.fill_trace(&mut memory_fragment);
    }
}
