use super::{Felt, FieldElement, StarkField, TraceFragment, Word};
use core::ops::RangeInclusive;
use vm_core::utils::collections::BTreeMap;

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

/// Initial value of every memory cell.
const INIT_MEM_VALUE: Word = [Felt::ZERO; 4];

// RANDOM ACCESS MEMORY
// ================================================================================================

/// Memory controller for the VM.
///
/// This component is responsible for tracking current memory state of the VM, as well as for
/// building an execution trace of all memory accesses.
///
/// The memory is word-addressable. That is, four field elements are located at each memory
/// address, and we can read and write elements to/from memory in batches of four.
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
/// - `ctx` contains context ID. Currently, context ID is always set to ZERO.
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
pub struct Memory {
    /// Current clock cycle of the VM.
    step: u64,

    /// Memory access trace sorted first by address and then by clock cycle.
    trace: BTreeMap<u64, Vec<(Felt, Word)>>,

    /// Total number of entries in the trace; tracked separately so that we don't have to sum up
    /// length of all vectors in the trace map all the time.
    num_trace_rows: usize,
}

impl Memory {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [Memory] initialized with an empty trace.
    pub fn new() -> Self {
        Self {
            step: 0,
            trace: BTreeMap::new(),
            num_trace_rows: 0,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns current size of the memory (in words).
    pub fn size(&self) -> usize {
        self.trace.len()
    }

    /// Returns length of execution trace required to describe all memory access operations
    /// executed on the VM.
    pub fn trace_len(&self) -> usize {
        self.num_trace_rows
    }

    // STATE ACCESSORS AND MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Returns a word (4 elements) located in memory at the specified address.
    ///
    /// If the specified address hasn't been previously written to, four ZERO elements are
    /// returned. This effectively implies that memory is initialized to ZERO.
    pub fn read(&mut self, addr: Felt) -> Word {
        self.num_trace_rows += 1;
        let clk = Felt::new(self.step);

        // look up the previous value in the appropriate address trace and add (clk, prev_value)
        // to it; if this is the first time we access this address, create address trace for it
        // with entry (clk, [ZERO, 4]). in both cases, return the last value in the address trace.
        self.trace
            .entry(addr.as_int())
            .and_modify(|addr_trace| {
                let last_value = addr_trace.last().expect("empty address trace").1;
                addr_trace.push((clk, last_value));
            })
            .or_insert_with(|| vec![(clk, INIT_MEM_VALUE)])
            .last()
            .expect("empty address trace")
            .1
    }

    /// Writes the provided words (4 elements) at the specified address.
    pub fn write(&mut self, addr: Felt, value: Word) {
        self.num_trace_rows += 1;
        let clk = Felt::new(self.step);

        // add a tuple (clk, value) to the appropriate address trace; if this is the first time
        // we access this address, initialize address trace.
        self.trace
            .entry(addr.as_int())
            .and_modify(|addr_trace| addr_trace.push((clk, value)))
            .or_insert_with(|| vec![(clk, value)]);
    }

    // CONTEXT MANAGEMENT
    // --------------------------------------------------------------------------------------------

    /// Increments the clock cycle.
    pub fn advance_clock(&mut self) {
        self.step += 1;
    }

    // EXECUTION TRACE GENERATION
    // --------------------------------------------------------------------------------------------

    /// Fills the provide trace fragment with trace data from this memory instance.
    pub fn fill_trace(self, trace: &mut TraceFragment) {
        debug_assert_eq!(self.trace_len(), trace.len(), "inconsistent trace lengths");

        // set the pervious address and clock cycle to the first address and clock cycle of the
        // trace; we also adjust the clock cycle so that delta value for the first row would end
        // up being ZERO. if the trace is empty, return without any further processing.
        let (mut prev_addr, mut prev_clk) = match self.get_first_row_info() {
            Some((addr, clk)) => (addr, clk - Felt::ONE),
            None => return,
        };

        // iterate through addresses in ascending order, and write trace row for each memory access
        // into the trace. we expect the trace to be 14 columns wide.
        let mut i = 0;
        for (addr, addr_trace) in self.trace {
            // when we start a new address, we set the previous value to all zeros. the effect of
            // this is that memory is always initialized to zero.
            let addr = Felt::new(addr);
            let mut prev_value = INIT_MEM_VALUE;
            for (clk, value) in addr_trace {
                trace.set(i, 0, Felt::ZERO); // ctx
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

                // compute delta as difference either between addresses or clock cycles
                let delta = if prev_addr != addr {
                    addr - prev_addr
                } else {
                    clk - prev_clk - Felt::ONE
                };

                let (delta_hi, delta_lo) = split_u32_into_u16(delta);
                trace.set(i, 11, delta_lo);
                trace.set(i, 12, delta_hi);
                trace.set(i, 13, delta.inv());

                // update values for the next iteration of the loop
                prev_addr = addr;
                prev_clk = clk;
                prev_value = value;
                i += 1;
            }
        }
    }

    /// Returns the address and clock cycle of the first trace row, or None if the trace is empty.
    fn get_first_row_info(&self) -> Option<(Felt, Felt)> {
        match self.trace.iter().next() {
            Some((&addr, addr_trace)) => {
                let clk = addr_trace[0].0;
                Some((Felt::new(addr), clk))
            }
            None => None,
        }
    }

    /// Returns a word located at the specified address, or None if the address hasn't been
    /// accessed previously.
    /// Unlike read() that modifies the underlying map, get_value() only attempts to read
    /// or return None when no value exists.
    pub fn get_value(&self, addr: u64) -> Option<Word> {
        match self.trace.get(&addr) {
            Some(addr_trace) => addr_trace.last().map(|(_, value)| *value),
            None => None,
        }
    }

    /// Returns all the addresses and values stored in memory.
    pub fn get_all_values(&self) -> Vec<(u64, Word)> {
        self.get_values(RangeInclusive::new(0, u64::MAX))
    }

    /// Returns values within a range of addresses at the last clock cycle.
    pub fn get_values(&self, range: RangeInclusive<u64>) -> Vec<(u64, Word)> {
        let mut data: Vec<(u64, Word)> = Vec::new();

        for (&addr, addr_trace) in self.trace.range(range) {
            let value = addr_trace.last().expect("empty address trace").1;
            data.push((addr, value));
        }

        data
    }

    /// Returns values within a range of addresses, or optionally all values at the beginning of.
    /// the specified cycle.
    pub fn get_values_at(&self, range: RangeInclusive<u64>, step: u64) -> Vec<(u64, Word)> {
        let mut data: Vec<(u64, Word)> = Vec::new();

        if step == 0 {
            return data;
        }

        // Because we want to view the memory state at the beginning of the specified cycle, we
        // view the memory state at the previous cycle, as the current memory state is at the
        // end of the current cycle.
        let search_step = step - 1;

        for (&addr, addr_trace) in self.trace.range(range) {
            match addr_trace.binary_search_by(|(x, _)| x.as_int().cmp(&search_step)) {
                Ok(i) => data.push((addr, addr_trace[i].1)),
                Err(i) => {
                    // Binary search would find the index the specified step should be in.
                    // We decrement the index to get the equal or less than specified step
                    // trace to insert into the results.
                    if i > 0 {
                        data.push((addr, addr_trace[i - 1].1));
                    }
                }
            }
        }

        data
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn split_u32_into_u16(value: Felt) -> (Felt, Felt) {
    const U32MAX: u64 = u32::MAX as u64;

    let value = value.as_int();
    assert!(value <= U32MAX, "not a 32-bit value");

    let lo = (value as u16) as u64;
    let hi = value >> 16;

    (Felt::new(hi), Felt::new(lo))
}
