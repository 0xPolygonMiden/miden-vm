use super::{
    trace, utils, BTreeMap, ChipletsTrace, ColMatrix, ExecutionError, Felt, FieldElement,
    RangeChecker, StarkField, TraceFragment, Vec, Word, CHIPLETS_WIDTH, ONE, ZERO,
};
use vm_core::{
    chiplets::bitwise::{BITWISE_AND_LABEL, BITWISE_XOR_LABEL},
    chiplets::{
        hasher::{Digest, HasherState},
        memory::{MEMORY_READ_LABEL, MEMORY_WRITE_LABEL},
    },
    code_blocks::OpBatch,
    Kernel,
};

mod bitwise;
use bitwise::{Bitwise, BitwiseLookup};

mod hasher;
use hasher::Hasher;
pub use hasher::{AuxTraceBuilder as HasherAuxTraceBuilder, SiblingTableRow};

mod memory;
use memory::{Memory, MemoryLookup};

mod kernel_rom;
use kernel_rom::KernelRom;

mod bus;
pub use bus::{AuxTraceBuilder, ChipletsBus};

#[cfg(test)]
mod tests;

// HELPER STRUCTS
// ================================================================================================

/// Result of a merkle tree node update. The result contains the old merkle_root, which
/// corresponding to the old_value, and the new merkle_root, for the updated value. As well as the
/// row address of the execution trace at which the computation started.
#[derive(Debug, Copy, Clone)]
pub struct MerkleRootUpdate {
    address: Felt,
    old_root: Word,
    new_root: Word,
}

impl MerkleRootUpdate {
    pub fn get_address(&self) -> Felt {
        self.address
    }
    pub fn get_old_root(&self) -> Word {
        self.old_root
    }
    pub fn get_new_root(&self) -> Word {
        self.new_root
    }
}

// CHIPLETS MODULE OF HASHER, BITWISE, MEMORY, AND KERNEL ROM CHIPLETS
// ================================================================================================

/// This module manages the VM's hasher, bitwise, memory, and kernel ROM chiplets and is
/// responsible for building a final execution trace from their stacked execution traces and
/// chiplet selectors.
///
/// The module's trace can be thought of as 5 stacked chiplet segments in the following form:
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
/// - columns 2-14: execution trace of bitwise chiplet
/// - columns 15-17: unused columns padded with ZERO
///
/// * Memory segment: contains the trace and selectors for the memory chiplet *
/// This segment begins at the end of the bitwise segment and fills the next rows of the trace for
/// the `trace_len` of the memory chiplet.
/// - column 0-1: selector columns with values set to ONE
/// - column 2: selector column with values set to ZERO
/// - columns 3-14: execution trace of memory chiplet
/// - columns 15-17: unused column padded with ZERO
///
/// * Kernel ROM segment: contains the trace and selectors for the kernel ROM chiplet *
/// This segment begins at the end of the memory segment and fills the next rows of the trace for
/// the `trace_len` of the kernel ROM chiplet.
/// - column 0-2: selector columns with values set to ONE
/// - column 3: selector column with values set to ZERO
/// - columns 4-9: execution trace of kernel ROM chiplet
/// - columns 10-17: unused column padded with ZERO
///
/// * Padding segment: unused *
/// This segment begins at the end of the kernel ROM segment and fills the rest of the execution
/// trace minus the number of random rows. When it finishes, the execution trace should have
/// exactly enough rows remaining for the specified number of random rows.
/// - columns 0-3: selector columns with values set to ONE
/// - columns 3-17: unused columns padded with ZERO
pub struct Chiplets {
    /// Current clock cycle of the VM.
    clk: u32,
    hasher: Hasher,
    bitwise: Bitwise,
    memory: Memory,
    kernel_rom: KernelRom,
    bus: ChipletsBus,
}

impl Chiplets {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [Chiplets] component instantiated with the provided Kernel.
    pub fn new(kernel: Kernel) -> Self {
        Self {
            clk: 0,
            hasher: Hasher::default(),
            bitwise: Bitwise::default(),
            memory: Memory::default(),
            kernel_rom: KernelRom::new(kernel),
            bus: ChipletsBus::default(),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the length of the trace required to accommodate chiplet components and 1
    /// mandatory padding row required for ensuring sufficient trace length for auxiliary connector
    /// columns that rely on the memory chiplet.
    pub fn trace_len(&self) -> usize {
        self.hasher.trace_len()
            + self.bitwise.trace_len()
            + self.memory.trace_len()
            + self.kernel_rom.trace_len()
            + 1
    }

    /// Returns the index of the first row of [Bitwise] execution trace.
    pub fn bitwise_start(&self) -> usize {
        self.hasher.trace_len()
    }

    /// Returns the index of the first row of the [Memory] execution trace.
    pub fn memory_start(&self) -> usize {
        self.bitwise_start() + self.bitwise.trace_len()
    }

    /// Returns the index of the first row of [KernelRom] execution trace.
    pub fn kernel_rom_start(&self) -> usize {
        self.memory_start() + self.memory.trace_len()
    }

    /// Returns the index of the first row of the padding section of the execution trace.
    pub fn padding_start(&self) -> usize {
        self.kernel_rom_start() + self.kernel_rom.trace_len()
    }

    /// Returns the underlying kernel used to initilize this instance.
    pub const fn kernel(&self) -> &Kernel {
        self.kernel_rom.kernel()
    }

    // HASH CHIPLET ACCESSORS FOR OPERATIONS
    // --------------------------------------------------------------------------------------------

    /// Requests a single permutation of the hash function to the provided state from the Hash
    /// chiplet.
    ///
    /// The returned tuple contains the hasher state after the permutation and the row address of
    /// the execution trace at which the permutation started.
    pub fn permute(&mut self, state: HasherState) -> (Felt, HasherState) {
        let mut lookups = Vec::new();
        let (addr, return_state) = self.hasher.permute(state, &mut lookups);
        self.bus.request_hasher_operation(&lookups, self.clk);

        // provide the responses to the bus
        self.bus.provide_hasher_lookups(&lookups);

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
        let mut lookups = Vec::new();
        let (addr, root) = self.hasher.build_merkle_root(value, path, index, &mut lookups);

        self.bus.request_hasher_operation(&lookups, self.clk);

        // provide the responses to the bus
        self.bus.provide_hasher_lookups(&lookups);

        (addr, root)
    }

    /// Requests a Merkle root update computation from the Hash chiplet.
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
    ) -> MerkleRootUpdate {
        let mut lookups = Vec::new();

        let merkle_root_update =
            self.hasher.update_merkle_root(old_value, new_value, path, index, &mut lookups);
        self.bus.request_hasher_operation(&lookups, self.clk);

        // provide the responses to the bus
        self.bus.provide_hasher_lookups(&lookups);

        merkle_root_update
    }

    // HASH CHIPLET ACCESSORS FOR CONTROL BLOCK DECODING
    // --------------------------------------------------------------------------------------------

    /// Requests the hash of the provided words from the Hash chiplet and checks the result
    /// hash(h1, h2) against the provided `expected_result`.
    ///
    /// It returns the row address of the execution trace at which the hash computation started.
    pub fn hash_control_block(
        &mut self,
        h1: Word,
        h2: Word,
        domain: Felt,
        expected_hash: Digest,
    ) -> Felt {
        let mut lookups = Vec::new();
        let (addr, result) =
            self.hasher.hash_control_block(h1, h2, domain, expected_hash, &mut lookups);

        // make sure the result computed by the hasher is the same as the expected block hash
        debug_assert_eq!(expected_hash, result.into());

        // send the request for the hash initialization
        self.bus.request_hasher_lookup(lookups[0], self.clk);

        // enqueue the request for the hash result
        self.bus.enqueue_hasher_request(lookups[1]);

        // provide the responses to the bus
        self.bus.provide_hasher_lookups(&lookups);

        addr
    }

    /// Requests computation a sequential hash of all operation batches in the list from the Hash
    /// chiplet and checks the result against the provided `expected_result`.
    ///
    /// It returns the row address of the execution trace at which the hash computation started.
    pub fn hash_span_block(&mut self, op_batches: &[OpBatch], expected_hash: Digest) -> Felt {
        let mut lookups = Vec::new();
        let (addr, result) = self.hasher.hash_span_block(op_batches, expected_hash, &mut lookups);

        // make sure the result computed by the hasher is the same as the expected block hash
        debug_assert_eq!(expected_hash, result.into());

        // send the request for the hash initialization
        self.bus.request_hasher_lookup(lookups[0], self.clk);

        // enqueue the rest of the requests in reverse order so that the next request is at
        // the top of the queue.
        for lookup in lookups.iter().skip(1).rev() {
            self.bus.enqueue_hasher_request(*lookup);
        }

        // provide the responses to the bus
        self.bus.provide_hasher_lookups(&lookups);

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
        let lookup = MemoryLookup::from_ints(MEMORY_READ_LABEL, ctx, addr, self.clk, value);
        self.bus.request_memory_operation(&[lookup], self.clk);

        value
    }

    /// Returns two words read from consecutive addresses started with `addr` in the specified
    /// context while recording memory accesses in the memory trace.
    ///
    /// If either of the accessed addresses hasn't been previously written to, ZERO elements are
    /// returned. This effectively implies that memory is initialized to ZERO.
    pub fn read_mem_double(&mut self, ctx: u32, addr: Felt) -> [Word; 2] {
        // read two words from memory: from addr and from addr + 1
        let addr2 = addr + ONE;
        let words = [self.memory.read(ctx, addr, self.clk), self.memory.read(ctx, addr2, self.clk)];

        // create lookups for both memory reads
        let lookups = [
            MemoryLookup::from_ints(MEMORY_READ_LABEL, ctx, addr, self.clk, words[0]),
            MemoryLookup::from_ints(MEMORY_READ_LABEL, ctx, addr2, self.clk, words[1]),
        ];

        // send lookups to the bus and return the result
        self.bus.request_memory_operation(&lookups, self.clk);
        words
    }

    /// Writes the provided word at the specified context/address.
    ///
    /// This also modifies the memory access trace and sends a memory lookup request to the bus.
    pub fn write_mem(&mut self, ctx: u32, addr: Felt, word: Word) {
        self.memory.write(ctx, addr, self.clk, word);

        // send the memory write request to the bus
        let lookup = MemoryLookup::from_ints(MEMORY_WRITE_LABEL, ctx, addr, self.clk, word);
        self.bus.request_memory_operation(&[lookup], self.clk);
    }

    /// Writes the provided element into the specified context/address leaving the remaining 3
    /// elements of the word previously stored at that address unchanged.
    ///
    /// This also modifies the memory access trace and sends a memory lookup request to the bus.
    pub fn write_mem_element(&mut self, ctx: u32, addr: Felt, value: Felt) -> Word {
        let old_word = self.memory.get_old_value(ctx, addr.as_int());
        let new_word = [value, old_word[1], old_word[2], old_word[3]];

        self.memory.write(ctx, addr, self.clk, new_word);

        // send the memory write request to the bus
        let lookup = MemoryLookup::from_ints(MEMORY_WRITE_LABEL, ctx, addr, self.clk, new_word);
        self.bus.request_memory_operation(&[lookup], self.clk);

        old_word
    }

    /// Writes the two provided words to two consecutive addresses in memory in the specified
    /// context, starting at the specified address.
    ///
    /// This also modifies the memory access trace and sends two memory lookup requests to the bus.
    pub fn write_mem_double(&mut self, ctx: u32, addr: Felt, words: [Word; 2]) {
        let addr2 = addr + ONE;
        // write two words to memory at addr and addr + 1
        self.memory.write(ctx, addr, self.clk, words[0]);
        self.memory.write(ctx, addr2, self.clk, words[1]);

        // create lookups for both memory writes
        let lookups = [
            MemoryLookup::from_ints(MEMORY_WRITE_LABEL, ctx, addr, self.clk, words[0]),
            MemoryLookup::from_ints(MEMORY_WRITE_LABEL, ctx, addr2, self.clk, words[1]),
        ];

        // send lookups to the bus
        self.bus.request_memory_operation(&lookups, self.clk);
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

    // KERNEL ROM ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Increments access counter for the specified kernel procedure.
    ///
    /// # Errors
    /// Returns an error if the procedure with the specified hash does not exist in the kernel
    /// with which the kernel ROM was instantiated.
    pub fn access_kernel_proc(&mut self, proc_hash: Digest) -> Result<(), ExecutionError> {
        self.kernel_rom.access_proc(proc_hash)

        // TODO: record the access in the chiplet bus
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
        self.memory.append_range_checks(self.memory_start(), range_checker);
    }

    /// Returns an execution trace of the chiplets containing the stacked traces of the
    /// Hasher, Bitwise, and Memory chiplets.
    ///
    /// `num_rand_rows` indicates the number of rows at the end of the trace which will be
    /// overwritten with random values.
    pub fn into_trace(self, trace_len: usize, num_rand_rows: usize) -> ChipletsTrace {
        // make sure that only padding rows will be overwritten by random values
        assert!(self.trace_len() + num_rand_rows <= trace_len, "target trace length too small");

        // Allocate columns for the trace of the chiplets.
        let mut trace = (0..CHIPLETS_WIDTH)
            .map(|_| Felt::zeroed_vector(trace_len))
            .collect::<Vec<_>>()
            .try_into()
            .expect("failed to convert vector to array");

        let (hasher_aux_builder, aux_builder) = self.fill_trace(&mut trace);

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
    /// It returns the auxiliary trace builders for generating auxiliary trace columns that depend
    /// on data from [Chiplets].
    fn fill_trace(
        self,
        trace: &mut [Vec<Felt>; CHIPLETS_WIDTH],
    ) -> (HasherAuxTraceBuilder, AuxTraceBuilder) {
        // get the rows where chiplets begin.
        let bitwise_start = self.bitwise_start();
        let memory_start = self.memory_start();
        let kernel_rom_start = self.kernel_rom_start();
        let padding_start = self.padding_start();

        let Chiplets {
            clk: _,
            hasher,
            bitwise,
            memory,
            kernel_rom,
            mut bus,
        } = self;

        // populate external selector columns for all chiplets
        trace[0][bitwise_start..].fill(ONE);
        trace[1][memory_start..].fill(ONE);
        trace[2][kernel_rom_start..].fill(ONE);
        trace[3][padding_start..].fill(ONE);

        // allocate fragments to be filled with the respective execution traces of each chiplet
        let mut hasher_fragment = TraceFragment::new(CHIPLETS_WIDTH);
        let mut bitwise_fragment = TraceFragment::new(CHIPLETS_WIDTH);
        let mut memory_fragment = TraceFragment::new(CHIPLETS_WIDTH);
        let mut kernel_rom_fragment = TraceFragment::new(CHIPLETS_WIDTH);

        // add the hasher, bitwise, memory, and kernel ROM segments to their respective fragments
        // so they can be filled with the chiplet traces
        for (column_num, column) in trace.iter_mut().enumerate().skip(1) {
            match column_num {
                1 | 15..=17 => {
                    // columns 1 and 15 - 17 are relevant only for the hasher
                    hasher_fragment.push_column_slice(column, hasher.trace_len());
                }
                2 => {
                    // column 2 is relevant to the hasher and to bitwise chiplet
                    let rest = hasher_fragment.push_column_slice(column, hasher.trace_len());
                    bitwise_fragment.push_column_slice(rest, bitwise.trace_len());
                }
                3 | 10..=14 => {
                    // columns 3 and 10 - 14 are relevant for hasher, bitwise, and memory chiplets
                    let rest = hasher_fragment.push_column_slice(column, hasher.trace_len());
                    let rest = bitwise_fragment.push_column_slice(rest, bitwise.trace_len());
                    memory_fragment.push_column_slice(rest, memory.trace_len());
                }
                4..=9 => {
                    // columns 4 - 9 are relevant to all chiplets
                    let rest = hasher_fragment.push_column_slice(column, hasher.trace_len());
                    let rest = bitwise_fragment.push_column_slice(rest, bitwise.trace_len());
                    let rest = memory_fragment.push_column_slice(rest, memory.trace_len());
                    kernel_rom_fragment.push_column_slice(rest, kernel_rom.trace_len());
                }
                _ => panic!("invalid column index"),
            }
        }

        // fill the fragments with the execution trace from each chiplet
        // TODO: this can be parallelized to fill the traces in multiple threads
        let hasher_aux_builder = hasher.fill_trace(&mut hasher_fragment);
        bitwise.fill_trace(&mut bitwise_fragment, &mut bus, bitwise_start);
        memory.fill_trace(&mut memory_fragment, &mut bus, memory_start);
        kernel_rom.fill_trace(&mut kernel_rom_fragment);

        (hasher_aux_builder, bus.into_aux_builder())
    }
}
