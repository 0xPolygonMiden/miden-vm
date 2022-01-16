use super::{Felt, FieldElement, StarkField, TraceFragment, Word};
use winter_utils::collections::BTreeMap;

#[cfg(test)]
mod tests;

// RANDOM ACCESS MEMORY
// ================================================================================================

/// TODO: add comments
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
    #[cfg(test)]
    pub fn size(&self) -> usize {
        self.trace.len()
    }

    /// Returns length of execution trace required to describe this memory.
    pub fn trace_len(&self) -> usize {
        self.num_trace_rows
    }

    // TRACE ACCESSORS AND MUTATORS
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
            .or_insert_with(|| vec![(clk, [Felt::ZERO; 4])])
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

    // TRACE COMPLETION
    // --------------------------------------------------------------------------------------------

    /// Fills the provide trace fragment with trace data from this memory instance.
    #[allow(dead_code)]
    pub fn fill_trace(self, trace: &mut TraceFragment) {
        debug_assert_eq!(self.trace_len(), trace.len(), "inconsistent trace_length");

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
            let mut prev_value = [Felt::ZERO; 4];
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
                i += 1;
                prev_clk = clk;
                prev_value = value;
            }
            prev_addr = addr;
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

    // TEST METHODS
    // --------------------------------------------------------------------------------------------

    /// Instantiates a new processor for testing purposes.
    #[cfg(test)]
    pub fn get_value(&self, addr: u64) -> Option<Word> {
        match self.trace.get(&addr) {
            Some(addr_trace) => addr_trace.last().map(|(_, value)| *value),
            None => None,
        }
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
