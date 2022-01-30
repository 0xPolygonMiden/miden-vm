use super::{Felt, FieldElement};

// SYSTEM INFO
// ================================================================================================

/// System info container for the VM.
///
/// Currently, this keeps track of the clock cycle and free memory pointer registers.
pub struct System {
    clk: usize,
    clk_trace: Vec<Felt>,
    fmp: Felt,
    fmp_trace: Vec<Felt>,
}

impl System {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [System] struct with execution traces instantiated with the specified length.
    pub fn new(init_trace_length: usize) -> Self {
        Self {
            clk: 0,
            clk_trace: vec![Felt::ZERO; init_trace_length],
            fmp: Felt::ZERO,
            fmp_trace: vec![Felt::ZERO; init_trace_length],
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the current clock cycle of a process.
    #[inline(always)]
    pub fn clk(&self) -> usize {
        self.clk
    }

    /// Returns the current value of the free memory pointer for a process.
    #[inline(always)]
    pub fn fmp(&self) -> Felt {
        self.fmp
    }

    /// Returns execution trace length of for a process.
    #[inline(always)]
    pub fn trace_length(&self) -> usize {
        self.clk_trace.len()
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Increments the clock cycle.
    pub fn advance_clock(&mut self) {
        self.clk += 1;
        self.clk_trace[self.clk] = Felt::new(self.clk as u64);

        self.fmp_trace[self.clk] = self.fmp;
    }

    /// Sets the value of free memory pointer for the next clock cycle.
    pub fn set_fmp(&mut self, fmp: Felt) {
        // we set only the current value of fmp here, the trace will be updated with this value
        // when the clock cycle advances.
        self.fmp = fmp;
    }

    // UTILITY METHODS
    // --------------------------------------------------------------------------------------------

    /// Makes sure there is enough memory allocated for the trace to accommodate a new row.
    ///
    /// Trace length is doubled every time it needs to be increased.
    pub fn ensure_trace_capacity(&mut self) {
        if self.clk + 1 >= self.trace_length() {
            let new_length = self.trace_length() * 2;
            self.clk_trace.resize(new_length, Felt::ZERO);
            self.fmp_trace.resize(new_length, Felt::ZERO);
        }
    }
}
