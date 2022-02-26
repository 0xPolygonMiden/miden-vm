use super::{Felt, FieldElement, SysTrace};

// CONSTANTS
// ================================================================================================

// Memory addresses for procedure locals should start at 2^30 and not go below.
pub const FMP_MIN: u64 = 2_u64.pow(30);
// The total number of locals available to all procedures at runtime must be smaller than 2^32.
pub const FMP_MAX: u64 = FMP_MIN + u32::MAX as u64;

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
    /// Initializes the free memory pointer `fmp` used for local memory offsets to 2^30.
    pub fn new(init_trace_length: usize) -> Self {
        // set the first value of the fmp trace to 2^30.
        let fmp = Felt::new(FMP_MIN);
        let mut fmp_trace = Felt::zeroed_vector(init_trace_length);
        fmp_trace[0] = fmp;

        Self {
            clk: 0,
            clk_trace: Felt::zeroed_vector(init_trace_length),
            fmp,
            fmp_trace,
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

    /// Returns execution trace length for a process.
    #[inline(always)]
    pub fn trace_len(&self) -> usize {
        self.clk_trace.len()
    }

    /// Returns an execution trace of this system info container.
    pub fn into_trace(self) -> SysTrace {
        [self.clk_trace, self.fmp_trace]
    }

    /// Returns free memory pointer at the specified clock cycle.
    #[inline(always)]
    pub fn get_fmp_at(&self, clk: usize) -> Felt {
        self.fmp_trace[clk]
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
        if self.clk + 1 >= self.trace_len() {
            let new_length = self.trace_len() * 2;
            self.clk_trace.resize(new_length, Felt::ZERO);
            self.fmp_trace.resize(new_length, Felt::ZERO);
        }
    }
}
