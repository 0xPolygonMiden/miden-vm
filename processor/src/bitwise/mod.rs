use super::{ExecutionError, Felt, StarkField, TraceFragment};

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

const TRACE_WIDTH: usize = 13;

const INIT_TRACE_LEN: usize = 128;

// BITWISE HELPER
// ================================================================================================

/// TODO: add docs
pub struct Bitwise {
    trace: [Vec<Felt>; TRACE_WIDTH],
}

impl Bitwise {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// TODO: add docs
    pub fn new() -> Self {
        let trace = (0..TRACE_WIDTH)
            .map(|_| Vec::with_capacity(INIT_TRACE_LEN))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self { trace }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// TODO: add docs
    pub fn trace_len(&self) -> usize {
        self.trace[0].len()
    }

    // TRACE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// TODO: add docs
    pub fn u32and(&mut self, a: Felt, b: Felt) -> Result<(Felt, Felt), ExecutionError> {
        let mut a = a.as_int();
        let mut b = b.as_int();
        let mut result = 0u64;

        // record the starting address of the trace table
        let addr = Felt::new(self.trace_len() as u64);

        // append 8 rows to the trace, each row computing bitwise AND in 4 bit limbs starting with
        // the least significant limb.
        for bit_offset in (0..32).step_by(4) {
            // add a new row to the trace table and populate it with binary decomposition of the
            // lower 4 bits of a and b.
            self.add_trace_row(a, b, bit_offset);

            // compute bitwise AND of the lower 4 bits of a and b
            let result_4_bit = (a & b) & 0xF;

            // append the 4 bit result to the result accumulator, and save the current result into
            // the 13th column of the trace.
            result |= result_4_bit << bit_offset;
            self.trace[12].push(Felt::new(result));

            // remove the lower 4 bits from a and b
            a >>= 4;
            b >>= 4;
        }

        Ok((addr, Felt::new(result)))
    }

    /// TODO: add docs
    pub fn u32or(&mut self, a: Felt, b: Felt) -> Result<(Felt, Felt), ExecutionError> {
        let mut a = a.as_int();
        let mut b = b.as_int();
        let mut result = 0u64;

        // record the starting address of the trace table
        let addr = Felt::new(self.trace_len() as u64);

        // append 8 rows to the trace, each row computing bitwise OR in 4 bit limbs starting with
        // the least significant limb.
        for bit_offset in (0..32).step_by(4) {
            // add a new row to the trace table and populate it with binary decomposition of the
            // lower 4 bits of a and b.
            self.add_trace_row(a, b, bit_offset);

            // compute bitwise OR of the lower 4 bits of a and b
            let result_4_bit = (a | b) & 0xF;

            // append the 4 bit result to the result accumulator, and save the current result into
            // the 13th column of the trace.
            result |= result_4_bit << bit_offset;
            self.trace[12].push(Felt::new(result));

            // remove the lower 4 bits from a and b
            a >>= 4;
            b >>= 4;
        }

        Ok((addr, Felt::new(result)))
    }

    /// TODO: add docs
    pub fn u32xor(&mut self, a: Felt, b: Felt) -> Result<(Felt, Felt), ExecutionError> {
        let mut a = a.as_int();
        let mut b = b.as_int();
        let mut result = 0u64;

        // record the starting address of the trace table
        let addr = Felt::new(self.trace_len() as u64);

        // append 8 rows to the trace, each row computing bitwise XOR in 4 bit limbs starting with
        // the least significant limb.
        for bit_offset in (0..32).step_by(4) {
            // add a new row to the trace table and populate it with binary decomposition of the
            // lower 4 bits of a and b.
            self.add_trace_row(a, b, bit_offset);

            // compute bitwise XOR of the lower 4 bits of a and b
            let result_4_bit = (a ^ b) & 0xF;

            // append the 4 bit result to the result accumulator, and save the current result into
            // the 13th column of the trace.
            result |= result_4_bit << bit_offset;
            self.trace[12].push(Felt::new(result));

            // remove the lower 4 bits from a and b
            a >>= 4;
            b >>= 4;
        }

        Ok((addr, Felt::new(result)))
    }

    // EXECUTION TRACE GENERATION
    // --------------------------------------------------------------------------------------------

    /// TODO: add docs
    #[allow(dead_code)]
    pub fn fill_trace(self, trace: &mut TraceFragment) {
        debug_assert_eq!(self.trace_len(), trace.len(), "inconsistent trace lengths");
        debug_assert_eq!(TRACE_WIDTH, trace.width(), "inconsistent trace widths");

        for (out_column, column) in trace.columns().zip(self.trace) {
            out_column.copy_from_slice(&column);
        }
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// TODO: add docs
    fn add_trace_row(&mut self, a: u64, b: u64, bit_offset: u64) {
        self.trace[0].push(Felt::new(self.trace_len() as u64));
        self.trace[1].push(Felt::new(a));
        self.trace[2].push(Felt::new(b));

        self.trace[3].push(Felt::new(a & 1));
        self.trace[4].push(Felt::new((a >> 1) & 1));
        self.trace[5].push(Felt::new((a >> 2) & 1));
        self.trace[6].push(Felt::new((a >> 3) & 1));

        self.trace[7].push(Felt::new(b & 1));
        self.trace[8].push(Felt::new((b >> 1) & 1));
        self.trace[9].push(Felt::new((b >> 2) & 1));
        self.trace[10].push(Felt::new((b >> 3) & 1));

        self.trace[11].push(Felt::new(1 << bit_offset));
    }
}
