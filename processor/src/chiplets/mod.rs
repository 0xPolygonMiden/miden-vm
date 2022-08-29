use super::{
    BTreeMap, ChipletsTrace, Felt, FieldElement, RangeChecker, StarkField, TraceFragment, Vec,
    Word, CHIPLETS_WIDTH, ONE, ZERO,
};
use crate::{trace::LookupTableRow, ExecutionError};

use vm_core::{
    chiplets::bitwise::{BITWISE_AND_LABEL, BITWISE_XOR_LABEL},
    chiplets::hasher::{Digest, HasherState},
    code_blocks::OpBatch,
};

mod bitwise;
use bitwise::{Bitwise, BitwiseLookup};

mod hasher;
use hasher::Hasher;
pub use hasher::{AuxTraceBuilder as HasherAuxTraceBuilder, SiblingTableRow};

mod memory;
use memory::{Memory, MemoryLookup};

mod bus;
pub use bus::{AuxTraceBuilder, ChipletsBus};

#[cfg(test)]
mod tests;

// CHIPLETS MODULE OF HASHER, BITWISE, AND MEMORY CHIPLETS
// ================================================================================================

/// This module manages the VM's hasher, bitwise, and memory chiplets and is responsible for
/// building a final execution trace from their stacked execution traces and chiplet selectors.
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
#[derive(Default)]
pub struct Chiplets {
    /// Current clock cycle of the VM.
    clk: u32,
    hasher: Hasher,
    bitwise: Bitwise,
    memory: Memory,
    bus: ChipletsBus,
}

impl Chiplets {
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

    // HASH CHIPLET ACCESSORS FOR OPERATIONS
    // --------------------------------------------------------------------------------------------

    /// Requests a single permutation of the hash function to the provided state from the Hash
    /// chiplet.
    ///
    /// The returned tuple contains the hasher state after the permutation and the row address of
    /// the execution trace at which the permutation started.
    pub fn permute(&mut self, state: HasherState) -> (Felt, HasherState) {
        let (addr, return_state, lookups) = self.hasher.permute(state);
        self.bus.request_hasher_operation(lookups, self.clk);

        (addr, return_state)
    }

    /// Requests a Merkle root computation from the Hash chiplet for the specified path and the node
    /// with the specified value.
    ///
    /// The returned tuple contains the root of the Merkle path and the row address of the
    /// execution trace at which the computation started.
    ///
    /// # Panics
    /// Panics if:
    /// - The provided path does not contain any nodes.
    /// - The provided index is out of range for the specified path.
    pub fn build_merkle_root(&mut self, value: Word, path: &[Word], index: Felt) -> (Felt, Word) {
        let (addr, root, lookups) = self.hasher.build_merkle_root(value, path, index);
        self.bus.request_hasher_operation(lookups, self.clk);

        (addr, root)
    }

    /// Requests a Merkle root update computation from the Hash chiplet.
    ///
    /// The returned tuple contains computed roots for the old value and the new value of the node
    /// with the specified path, as well as the row address of the execution trace at which the
    /// computation started.
    ///
    /// # Panics
    /// Panics if:
    /// - The provided path does not contain any nodes.
    /// - The provided index is out of range for the specified path.
    pub fn update_merkle_root(
        &mut self,
        old_value: Word,
        new_value: Word,
        path: &[Word],
        index: Felt,
    ) -> (Felt, Word, Word) {
        let (addr, old_root, new_root, lookups) = self
            .hasher
            .update_merkle_root(old_value, new_value, path, index);
        self.bus.request_hasher_operation(lookups, self.clk);

        (addr, old_root, new_root)
    }

    // HASH CHIPLET ACCESSORS FOR CONTROL BLOCK DECODING
    // --------------------------------------------------------------------------------------------

    /// Requests the hash of the provided words from the Hash chiplet and checks the result
    /// hash(h1, h2) against the provided `expected_result`.
    ///
    /// It returns the row address of the execution trace at which the hash computation started.
    pub fn hash_control_block(&mut self, h1: Word, h2: Word, expected_result: Digest) -> Felt {
        let (addr, result, lookups) = self.hasher.merge(h1, h2);

        // make sure the result computed by the hasher is the same as the expected block hash
        debug_assert_eq!(expected_result, result.into());

        // send the request for the hash initialization
        self.bus.request_hasher_lookup(lookups[0], self.clk);

        // enqueue the request for the hash result
        self.bus.enqueue_hasher_request(lookups[1]);

        addr
    }

    /// Requests computation a sequential hash of all operation batches in the list from the Hash
    /// chiplet and checks the result against the provided `expected_result`.
    ///
    /// It returns the row address of the execution trace at which the hash computation started.
    pub fn hash_span_block(
        &mut self,
        op_batches: &[OpBatch],
        num_op_groups: usize,
        expected_result: Digest,
    ) -> Felt {
        let (addr, result, lookups) = self.hasher.hash_span_block(op_batches, num_op_groups);

        // make sure the result computed by the hasher is the same as the expected block hash
        debug_assert_eq!(expected_result, result.into());

        // send the request for the hash initialization
        self.bus.request_hasher_lookup(lookups[0], self.clk);

        // enqueue the rest of the requests in reverse order so that the next request is at
        // the top of the queue.
        for lookup in lookups.iter().skip(1).rev() {
            self.bus.enqueue_hasher_request(*lookup);
        }

        addr
    }

    /// Sends a request for a [HasherLookup] required for verifying absorption of a new `SPAN` batch
    /// to the Chiplets Bus. It's expected to be called by the decoder while processing a `RESPAN`.
    ///
    /// It's processed by moving the corresponding lookup from the Chiplets bus' queued lookups to
    /// its requested lookups. Therefore, the next queued lookup is expected to be a precomputed
    /// lookup for absorbing new elements into the hasher state.
    pub fn absorb_span_batch(&mut self) {
        self.bus.send_queued_hasher_request(self.clk);
    }

    /// Sends a request for a control block hash result to the Chiplets Bus. It's expected to be
    /// called by the decoder to request the finalization (return hash) of a control block hash
    /// computation for the control block it has just finished decoding.
    ///
    /// It's processed by moving the corresponding lookup from the Chiplets bus' queued lookups to
    /// its requested lookups. Therefore, the next queued lookup is expected to be a precomputed
    /// lookup for returning a hash result.
    pub fn read_hash_result(&mut self) {
        self.bus.send_queued_hasher_request(self.clk);
    }

    // BITWISE CHIPLET ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Requests a bitwise AND of `a` and `b` from the Bitwise chiplet and returns the result.
    /// We assume that `a` and `b` are 32-bit values. If that's not the case, the result of the
    /// computation is undefined.
    pub fn u32and(&mut self, a: Felt, b: Felt) -> Result<Felt, ExecutionError> {
        let result = self.bitwise.u32and(a, b)?;

        let bitwise_lookup = BitwiseLookup::new(BITWISE_AND_LABEL, a, b, result);
        self.bus.request_bitwise_operation(bitwise_lookup, self.clk);

        Ok(result)
    }

    /// Requests a bitwise XOR of `a` and `b` from the Bitwise chiplet and returns the result.
    /// We assume that `a` and `b` are 32-bit values. If that's not the case, the result of the
    /// computation is undefined.
    pub fn u32xor(&mut self, a: Felt, b: Felt) -> Result<Felt, ExecutionError> {
        let result = self.bitwise.u32xor(a, b)?;

        let bitwise_lookup = BitwiseLookup::new(BITWISE_XOR_LABEL, a, b, result);
        self.bus.request_bitwise_operation(bitwise_lookup, self.clk);

        Ok(result)
    }

    // MEMORY CHIPLET ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a word located in memory at the specified context/address while recording the
    /// memory access in the memory trace.
    ///
    /// If the specified address hasn't been previously written to, four ZERO elements are
    /// returned. This effectively implies that memory is initialized to ZERO.
    pub fn read_mem(&mut self, ctx: u32, addr: Felt) -> Word {
        // read the word from memory
        let value = self.memory.read(ctx, addr, self.clk);

        // send the memory read request to the bus
        let memory_lookup = MemoryLookup::from_ints(ctx, addr, self.clk, value, value);
        self.bus.request_memory_operation(memory_lookup, self.clk);

        value
    }

    /// Writes the provided word at the specified context/address.
    ///
    /// This also modifies the memory access trace.
    pub fn write_mem(&mut self, ctx: u32, addr: Felt, word: Word) -> Word {
        let old_word = self.memory.get_old_value(ctx, addr);
        self.memory.write(ctx, addr, self.clk, word);

        // send the memory write request to the bus
        let memory_lookup = MemoryLookup::from_ints(ctx, addr, self.clk, old_word, word);
        self.bus.request_memory_operation(memory_lookup, self.clk);

        old_word
    }

    /// Writes the provided element into the specified context/address leaving the remaining 3
    /// elements of the word previously stored at that address unchanged.
    ///
    /// This also modifies the memory access trace.
    pub fn write_mem_single(&mut self, ctx: u32, addr: Felt, value: Felt) -> Word {
        let old_word = self.memory.get_old_value(ctx, addr);
        let new_word = [value, old_word[1], old_word[2], old_word[3]];

        self.memory.write(ctx, addr, self.clk, new_word);

        // send the memory write request to the bus
        let memory_lookup = MemoryLookup::from_ints(ctx, addr, self.clk, old_word, new_word);
        self.bus.request_memory_operation(memory_lookup, self.clk);

        old_word
    }

    /// Returns a word located at the specified context/address, or None if the address hasn't
    /// been accessed previously.
    ///
    /// Unlike mem_read() which modifies the memory access trace, this method returns the value at
    /// the specified address (if one exists) without altering the memory access trace.
    pub fn get_mem_value(&self, ctx: u32, addr: u64) -> Option<Word> {
        self.memory.get_value(ctx, addr)
    }

    /// Returns the entire memory state for the specified execution context at the specified cycle.
    /// The state is returned as a vector of (address, value) tuples, and includes addresses which
    /// have been accessed at least once.
    pub fn get_mem_state_at(&self, ctx: u32, clk: u32) -> Vec<(u64, Word)> {
        self.memory.get_state_at(ctx, clk)
    }

    /// Returns current size of the memory (in words) across all execution contexts.
    #[cfg(test)]
    pub fn get_mem_size(&self) -> usize {
        self.memory.size()
    }

    // CONTEXT MANAGEMENT
    // --------------------------------------------------------------------------------------------

    /// Increments the clock cycle.
    pub fn advance_clock(&mut self) {
        self.clk += 1;
    }

    // EXECUTION TRACE
    // --------------------------------------------------------------------------------------------

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
    ) -> (HasherAuxTraceBuilder, AuxTraceBuilder) {
        // get the rows where chiplets begin.
        let bitwise_start = self.hasher.trace_len();
        let memory_start = self.memory_start();
        let Chiplets {
            clk: _,
            hasher,
            bitwise,
            memory,
            mut bus,
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
                15 | 16 => {
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
        let hasher_aux_builder = hasher.fill_trace(&mut hasher_fragment, &mut bus);
        bitwise.fill_trace(&mut bitwise_fragment, &mut bus, bitwise_start);
        memory.fill_trace(&mut memory_fragment, &mut bus, memory_start);

        (hasher_aux_builder, bus.into_aux_builder())
    }
}
