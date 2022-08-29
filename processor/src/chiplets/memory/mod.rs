use super::{
    BTreeMap, ChipletsBus, Felt, FieldElement, StarkField, TraceFragment, Vec, Word, ONE, ZERO,
};
use crate::{
    range::RangeChecker,
    trace::LookupTableRow,
    utils::{split_element_u32_into_u16, split_u32_into_u16},
};
use vm_core::chiplets::memory::MEMORY_LABEL;

mod segment;
use segment::MemorySegmentTrace;

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

/// Initial value of every memory cell.
const INIT_MEM_VALUE: Word = [ZERO; 4];

// RANDOM ACCESS MEMORY
// ================================================================================================

/// Memory controller for the VM.
///
/// This component is responsible for tracking current memory state of the VM, as well as for
/// building an execution trace of all memory accesses.
///
/// The memory is comprised of one or more segments, each segment accessible from a specific
/// execution context. The root (kernel) context has context ID 0, and all additional contexts
/// have increasing IDs. Within each segment, the memory is word-addressable. That is, four field
/// elements are located at each memory address, and we can read and write elements to/from memory
/// in batches of four.
///
/// Memory for a a given address is always initialized to zeros. That is, reading from an address
/// before writing to it will return four ZERO elements.
///
/// ## Execution trace
/// The layout of the memory access trace is shown below.
///
///   ctx   addr   clk   u0   u1   u2   u3   v0   v1   v2   v3   d0   d1   d_inv
/// ├─────┴──────┴─────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴───────┤
///
/// In the above, the meaning of the columns is as follows:
/// - `ctx` contains execution context ID. Values in this column must increase monotonically but
///   there can be gaps between two consecutive context IDs of up to 2^32. Also, two consecutive
///   values can be the same.
/// - `addr` contains memory address. Values in this column must increase monotonically for a
///   given context but there can be gaps between two consecutive values of up to 2^32. Also,
///   two consecutive values can be the same.
/// - `clk` contains clock cycle at which a memory operation happened. Values in this column must
///   increase monotonically for a given context and memory address but there can be gaps between
///   two consecutive values of up to 2^32.
/// - Columns `u0`, `u1`, `u2`, `u3` contain field elements stored at a given context/address/clock
///   cycle prior to a memory operation.
/// - Columns `v0`, `v1`, `v2`, `v3` contain field elements stored at a given context/address/clock
///   cycle after the memory operation. Notice that for a READ operation `u0` = `v0`, `u1` = `v1`
///   etc.
/// - Columns `d0` and `d1` contain lower and upper 16 bits of the delta between two consecutive
///   context IDs, addresses, or clock cycles. Specifically:
///   - When the context changes, these columns contain (`new_ctx` - `old_ctx`).
///   - When the context remains the same but the address changes, these columns contain
///     (`new_addr` - `old-addr`).
///   - When both the context and the address remain the same, these columns contain
///     (`new_clk` - `old_clk` - 1).
/// - `d_inv` contains the inverse of the delta between two consecutive context IDs, addresses, or
///   clock cycles computed as described above.
///
/// For the first row of the trace, values in `d0`, `d1`, and `d_inv` are set to zeros.
#[derive(Default)]
pub struct Memory {
    /// Memory segment traces sorted by their execution context ID.
    trace: BTreeMap<u32, MemorySegmentTrace>,

    /// Total number of entries in the trace (across all contexts); tracked separately so that we
    /// don't have to sum up lengths of all address trace vectors for all contexts all the time.
    num_trace_rows: usize,
}

impl Memory {
    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns length of execution trace required to describe all memory access operations
    /// executed on the VM.
    pub fn trace_len(&self) -> usize {
        self.num_trace_rows
    }

    /// Returns a word located at the specified context/address, or None if the address hasn't
    /// been accessed previously.
    ///
    /// Unlike read() which modifies the memory access trace, this method returns the value at the
    /// specified address (if one exists) without altering the memory access trace.
    pub fn get_value(&self, ctx: u32, addr: u64) -> Option<Word> {
        match self.trace.get(&ctx) {
            Some(segment) => segment.get_value(addr),
            None => None,
        }
    }

    /// Returns the word at the specified context/address which should be used as the "old value" for a
    /// write request. It will be the previously stored value, if one exists, or initialized memory.
    pub fn get_old_value(&self, ctx: u32, addr: Felt) -> Word {
        // get the stored word or return [0, 0, 0, 0], since the memory is initialized with zeros
        self.get_value(ctx, addr.as_int()).unwrap_or(INIT_MEM_VALUE)
    }

    /// Returns the entire memory state for the specified execution context at the specified cycle.
    /// The state is returned as a vector of (address, value) tuples, and includes addresses which
    /// have been accessed at least once.
    pub fn get_state_at(&self, ctx: u32, clk: u32) -> Vec<(u64, Word)> {
        if clk == 0 {
            return vec![];
        }

        match self.trace.get(&ctx) {
            Some(segment) => segment.get_state_at(clk),
            None => vec![],
        }
    }

    // STATE ACCESSORS AND MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Returns a word located in memory at the specified context/address.
    ///
    /// If the specified address hasn't been previously written to, four ZERO elements are
    /// returned. This effectively implies that memory is initialized to ZERO.
    pub fn read(&mut self, ctx: u32, addr: Felt, clk: u32) -> Word {
        self.num_trace_rows += 1;
        self.trace
            .entry(ctx)
            .or_insert_with(MemorySegmentTrace::default)
            .read(addr, Felt::from(clk))
    }

    /// Writes the provided word at the specified context/address.
    pub fn write(&mut self, ctx: u32, addr: Felt, clk: u32, value: Word) {
        self.num_trace_rows += 1;
        self.trace
            .entry(ctx)
            .or_insert_with(MemorySegmentTrace::default)
            .write(addr, Felt::from(clk), value);
    }

    // EXECUTION TRACE GENERATION
    // --------------------------------------------------------------------------------------------

    /// Adds all of the range checks required by the [Memory] chiplet to the provided
    /// [RangeChecker] chiplet instance, along with their row in the finalized execution trace.
    pub fn append_range_checks(&self, memory_start_row: usize, range: &mut RangeChecker) {
        // set the previous address and clock cycle to the first address and clock cycle of the
        // trace; we also adjust the clock cycle so that delta value for the first row would end
        // up being ZERO. if the trace is empty, return without any further processing.
        let (mut prev_ctx, mut prev_addr, mut prev_clk) = match self.get_first_row_info() {
            Some((ctx, addr, clk)) => (ctx, addr, clk.as_int() - 1),
            None => return,
        };

        // op range check index
        let mut row = memory_start_row as u32;

        for (&ctx, segment) in self.trace.iter() {
            for (&addr, addr_trace) in segment.inner().iter() {
                // when we start a new address, we set the previous value to all zeros. the effect of
                // this is that memory is always initialized to zero.
                for (clk, _) in addr_trace {
                    let clk = clk.as_int();

                    // compute delta as difference between context IDs, addresses, or clock cycles
                    let delta = if prev_ctx != ctx {
                        (ctx - prev_ctx) as u64
                    } else if prev_addr != addr {
                        addr - prev_addr
                    } else {
                        clk - prev_clk - 1
                    };

                    let (delta_hi, delta_lo) = split_u32_into_u16(delta);
                    range.add_mem_checks(row, &[delta_lo, delta_hi]);

                    // update values for the next iteration of the loop
                    prev_ctx = ctx;
                    prev_addr = addr;
                    prev_clk = clk;
                    row += 1;
                }
            }
        }
    }

    /// Fills the provided trace fragment with trace data from this memory instance.
    pub fn fill_trace(
        self,
        trace: &mut TraceFragment,
        chiplets_bus: &mut ChipletsBus,
        memory_start_row: usize,
    ) {
        debug_assert_eq!(self.trace_len(), trace.len(), "inconsistent trace lengths");

        // set the pervious address and clock cycle to the first address and clock cycle of the
        // trace; we also adjust the clock cycle so that delta value for the first row would end
        // up being ZERO. if the trace is empty, return without any further processing.
        let (mut prev_ctx, mut prev_addr, mut prev_clk) = match self.get_first_row_info() {
            Some((ctx, addr, clk)) => (Felt::from(ctx), Felt::from(addr), clk - ONE),
            None => return,
        };

        // iterate through addresses in ascending order, and write trace row for each memory access
        // into the trace. we expect the trace to be 14 columns wide.
        let mut i = 0;

        for (ctx, segment) in self.trace {
            let ctx = Felt::from(ctx);
            for (addr, addr_trace) in segment.into_inner() {
                // when we start a new address, we set the previous value to all zeros. the effect of
                // this is that memory is always initialized to zero.
                let addr = Felt::new(addr);
                let mut prev_value = INIT_MEM_VALUE;
                for (clk, value) in addr_trace {
                    trace.set(i, 0, ctx);
                    trace.set(i, 1, addr);
                    trace.set(i, 2, clk);
                    trace.set(i, 3, prev_value[0]);
                    trace.set(i, 4, prev_value[1]);
                    trace.set(i, 5, prev_value[2]);
                    trace.set(i, 6, prev_value[3]);
                    trace.set(i, 7, value[0]);
                    trace.set(i, 8, value[1]);
                    trace.set(i, 9, value[2]);
                    trace.set(i, 10, value[3]);

                    // compute delta as difference between context IDs, addresses, or clock cycles
                    let delta = if prev_ctx != ctx {
                        ctx - prev_ctx
                    } else if prev_addr != addr {
                        addr - prev_addr
                    } else {
                        clk - prev_clk - ONE
                    };

                    let (delta_hi, delta_lo) = split_element_u32_into_u16(delta);
                    trace.set(i, 11, delta_lo);
                    trace.set(i, 12, delta_hi);
                    // TODO: switch to batch inversion to improve efficiency.
                    trace.set(i, 13, delta.inv());

                    // provide the memory access data to the chiplets bus.
                    let memory_lookup = MemoryLookup::new(ctx, addr, clk, prev_value, value);
                    chiplets_bus
                        .provide_memory_operation(memory_lookup, (memory_start_row + i) as u32);

                    // update values for the next iteration of the loop
                    prev_ctx = ctx;
                    prev_addr = addr;
                    prev_clk = clk;
                    prev_value = value;
                    i += 1;
                }
            }
        }
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Returns the context, address, and clock cycle of the first trace row, or None if the trace
    /// is empty.
    fn get_first_row_info(&self) -> Option<(u32, u64, Felt)> {
        let (ctx, segment) = match self.trace.iter().next() {
            Some((&ctx, segment)) => (ctx, segment),
            None => return None,
        };

        let (&addr, addr_trace) = segment.inner().iter().next().expect("empty memory segment");

        Some((ctx, addr, addr_trace[0].0))
    }

    // TEST HELPERS
    // --------------------------------------------------------------------------------------------

    /// Returns current size of the memory (in words) across all contexts.
    #[cfg(test)]
    pub fn size(&self) -> usize {
        self.trace.iter().fold(0, |acc, (_, s)| acc + s.size())
    }
}

// MEMORY LOOKUPS
// ================================================================================================

/// Contains the data required to describe a memory read or write.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MemoryLookup {
    ctx: Felt,
    addr: Felt,
    clk: Felt,
    old_word: Word,
    new_word: Word,
}

impl MemoryLookup {
    pub fn new(ctx: Felt, addr: Felt, clk: Felt, old_word: Word, new_word: Word) -> Self {
        Self {
            ctx,
            addr,
            clk,
            old_word,
            new_word,
        }
    }

    pub fn from_ints(ctx: u32, addr: Felt, clk: u32, old_word: Word, new_word: Word) -> Self {
        Self {
            ctx: Felt::from(ctx),
            addr,
            clk: Felt::from(clk),
            old_word,
            new_word,
        }
    }
}

impl LookupTableRow for MemoryLookup {
    /// Reduces this row to a single field element in the field specified by E. This requires
    /// at least 13 alpha values.
    fn to_value<E: FieldElement<BaseField = Felt>>(&self, alphas: &[E]) -> E {
        let old_word_value = self
            .old_word
            .iter()
            .enumerate()
            .fold(E::ZERO, |acc, (j, element)| {
                acc + alphas[j + 5].mul_base(*element)
            });
        let new_word_value = self
            .new_word
            .iter()
            .enumerate()
            .fold(E::ZERO, |acc, (j, element)| {
                acc + alphas[j + 9].mul_base(*element)
            });

        alphas[0]
            + alphas[1].mul_base(MEMORY_LABEL)
            + alphas[2].mul_base(self.ctx)
            + alphas[3].mul_base(self.addr)
            + alphas[4].mul_base(self.clk)
            + old_word_value
            + new_word_value
    }
}
