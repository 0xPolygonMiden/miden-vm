use vm_core::StarkField;

use super::{Felt, FieldElement, SysTrace, Vec, ZERO};

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
/// This keeps track of the clock cycle, execution context, and free memory pointer registers.
pub struct System {
    clk: u32,
    ctx: u32,
    fmp: Felt,
    ctx_trace: Vec<Felt>,
    clk_trace: Vec<Felt>,
    fmp_trace: Vec<Felt>,
}

impl System {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [System] struct with execution traces instantiated with the specified length.
    ///
    /// Initializes the free memory pointer `fmp` used for local memory offsets to 2^30.
    pub fn new(init_trace_capacity: usize) -> Self {
        // set the first value of the fmp trace to 2^30.
        let fmp = Felt::from(FMP_MIN);
        let mut fmp_trace = Felt::zeroed_vector(init_trace_capacity);
        fmp_trace[0] = fmp;

        Self {
            clk: 0,
            ctx: 0,
            fmp,
            clk_trace: Felt::zeroed_vector(init_trace_capacity),
            ctx_trace: Felt::zeroed_vector(init_trace_capacity),
            fmp_trace,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the current clock cycle of a process.
    #[inline(always)]
    pub fn clk(&self) -> u32 {
        self.clk
    }

    /// Returns the current execution context ID.
    #[inline(always)]
    pub fn ctx(&self) -> u32 {
        self.ctx
    }

    /// Returns the current value of the free memory pointer for a process.
    #[inline(always)]
    pub fn fmp(&self) -> Felt {
        self.fmp
    }

    /// Returns execution trace length for the systems columns of the process.
    ///
    /// Trace length of the system columns is equal to the number of cycles executed by the VM.
    #[inline(always)]
    pub fn trace_len(&self) -> usize {
        self.clk as usize
    }

    /// Returns execution context ID at the specified clock cycle.
    #[inline(always)]
    pub fn get_ctx_at(&self, clk: u32) -> u32 {
        self.ctx_trace[clk as usize].as_int() as u32
    }

    /// Returns free memory pointer at the specified clock cycle.
    #[inline(always)]
    pub fn get_fmp_at(&self, clk: u32) -> Felt {
        self.fmp_trace[clk as usize]
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Increments the clock cycle.
    pub fn advance_clock(&mut self) {
        self.clk += 1;

        self.clk_trace[self.clk as usize] = Felt::from(self.clk);
        self.fmp_trace[self.clk as usize] = self.fmp;
        self.ctx_trace[self.clk as usize] = Felt::from(self.ctx);
    }

    /// Sets the execution context ID for the next clock cycle.
    pub fn set_ctx(&mut self, ctx: u32) {
        // we set only the current value of ctx here, the trace will be updated with this value
        // when the clock cycle advances.
        self.ctx = ctx;
    }

    /// Sets the value of free memory pointer for the next clock cycle.
    pub fn set_fmp(&mut self, fmp: Felt) {
        // we set only the current value of fmp here, the trace will be updated with this value
        // when the clock cycle advances.
        self.fmp = fmp;
    }

    // TRACE GENERATIONS
    // --------------------------------------------------------------------------------------------

    /// Returns an execution trace of this system info container.
    ///
    /// If the trace is smaller than the specified `trace_len`, the columns of the trace are
    /// extended to match the specified length as follows:
    /// - the remainder of the `clk` column is filled in with increasing values of `clk`.
    /// - the remainder of the `ctx` column is filled in with ZERO, which should be the last value
    ///   in the column.
    /// - the remainder of the `fmp` column is filled in with the last value in the column.
    ///
    /// `num_rand_rows` indicates the number of rows at the end of the trace which will be
    /// overwritten with random values. This parameter is unused because last rows are just
    /// duplicates of the prior rows and thus can be safely overwritten.
    pub fn into_trace(mut self, trace_len: usize, num_rand_rows: usize) -> SysTrace {
        let clk = self.clk() as usize;
        // make sure that only the duplicate rows will be overwritten with random values
        assert!(
            clk + num_rand_rows <= trace_len,
            "target trace length too small"
        );

        // complete the clk column by filling in all values after the last clock cycle. The values
        // in the clk column are equal to the index of the row in the trace table.
        self.clk_trace.resize(trace_len, ZERO);
        for (i, clk) in self.clk_trace.iter_mut().enumerate().skip(clk) {
            // converting from u32 is OK here because max trace length is 2^32
            *clk = Felt::from(i as u32);
        }

        // complete the ctx column by filling all values after the last clock cycle with ZEROs as
        // the last context must be zero context.
        debug_assert_eq!(0, self.ctx);
        self.ctx_trace[clk..].fill(ZERO);
        self.ctx_trace.resize(trace_len, ZERO);

        // complete the fmp column by filling in all values after the last clock cycle with the
        // value in the column at the last clock cycle.
        let last_value = self.fmp_trace[clk];
        self.fmp_trace[clk..].fill(last_value);
        self.fmp_trace.resize(trace_len, last_value);

        [self.clk_trace, self.fmp_trace, self.ctx_trace]
    }

    // UTILITY METHODS
    // --------------------------------------------------------------------------------------------

    /// Makes sure there is enough memory allocated for the trace to accommodate a new row.
    ///
    /// Trace length is doubled every time it needs to be increased.
    pub fn ensure_trace_capacity(&mut self) {
        let current_capacity = self.clk_trace.len();
        if self.clk + 1 >= current_capacity as u32 {
            let new_length = current_capacity * 2;
            self.clk_trace.resize(new_length, ZERO);
            self.ctx_trace.resize(new_length, ZERO);
            self.fmp_trace.resize(new_length, ZERO);
        }
    }
}
