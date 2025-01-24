use alloc::vec::Vec;

use miden_air::{
    trace::chiplets::bitwise::{
        A_COL_IDX, A_COL_RANGE, BITWISE_AND, BITWISE_XOR, B_COL_IDX, B_COL_RANGE, OUTPUT_COL_IDX,
        PREV_OUTPUT_COL_IDX, TRACE_WIDTH,
    },
    RowIndex,
};

use super::{ExecutionError, Felt, TraceFragment, ZERO};

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

/// Initial capacity of each column.
const INIT_TRACE_CAPACITY: usize = 128;

// BITWISE
// ================================================================================================

#[derive(Debug)]
struct BitwiseTraceRow {
    selector: Felt,
    a: Felt,
    b: Felt,
    a0: Felt,
    a1: Felt,
    a2: Felt,
    a3: Felt,
    b0: Felt,
    b1: Felt,
    b2: Felt,
    b3: Felt,
    prev_output: Felt,
    output: Felt,
}

/// Helper for the VM that computes AND and XOR bitwise operations on 32-bit values.
/// It also builds an execution trace of these operations.
///
/// ## Bitwise operation execution trace (AND and XOR)
/// The execution trace for each operation consists of 8 rows and 14 columns. At a high level,
/// we break input values into 4-bit limbs, apply the bitwise operation to these limbs at every
/// row starting with the most significant limb, and accumulate the result in the result column.
///
/// The layout of the table is illustrated below.
///
///    s     a     b      a0     a1     a2     a3     b0     b1     b2     b3    zp     z
/// ├─────┴─────┴─────┴───────┴──────┴──────┴──────┴──────┴──────┴──────┴──────┴─────┴─────┤
///
/// In the above, the meaning of the columns is as follows:
/// - Selector column s is used to specify the bitwise operator for each row.
/// - Columns `a` and `b` contain accumulated 4-bit limbs of input values. Specifically, at the
///   first row, the values of columns `a` and `b` are set to the most significant 4-bit limb of
///   each input value. With all subsequent rows, the next most significant limb is appended to each
///   column for the corresponding value. Thus, by the 8th row, columns `a` and `b` contain full
///   input values for the bitwise operation.
/// - Columns `a0` through `a3` and `b0` through `b3` contain bits of the least significant 4-bit
///   limb of the values in `a` and `b` columns respectively.
/// - Column `zp` contains the accumulated result of applying the bitwise operation to 4-bit limbs,
///   but for the previous row. In the first row, it is 0.
/// - Column `z` contains the accumulated result of applying the bitwise operation to 4-bit limbs.
///   At the first row, column `z` contains the result of bitwise operation applied to the most
///   significant 4-bit limbs of the input values. With every subsequent row, the next most
///   significant 4-bit limb of the result is appended to it. Thus, by the 8th row, column `z`
///   contains the full result of the bitwise operation.
#[derive(Debug)]
pub struct Bitwise {
    rows: Vec<BitwiseTraceRow>,
}

impl Bitwise {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [Bitwise] initialized with an empty trace.
    pub fn new() -> Self {
        Self {
            rows: Vec::with_capacity(INIT_TRACE_CAPACITY),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns length of execution trace required to describe bitwise operations executed on the
    /// VM.
    pub fn trace_len(&self) -> usize {
        self.rows.len()
    }

    // TRACE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Computes a bitwise AND of `a` and `b` and returns the result. We assume that `a` and `b`
    /// are 32-bit values. If that's not the case, the result of the computation is undefined.
    ///
    /// This also adds 8 rows to the internal execution trace table required for computing the
    /// operation.
    pub fn u32and(&mut self, a: Felt, b: Felt) -> Result<Felt, ExecutionError> {
        let a = assert_u32(a)?;
        let b = assert_u32(b)?;
        let mut prev_output;
        let mut output = 0u64;

        // append 8 rows to the trace, each row computing bitwise AND in 4 bit limbs starting with
        // the most significant limb.
        for bit_offset in (0..32).step_by(4).rev() {
            // shift a and b so that the next 4-bit limb is in the least significant position
            let a = a >> bit_offset;
            let b = b >> bit_offset;

            // compute the output
            prev_output = output;
            output = {
                // compute bitwise AND of the 4 least significant bits of a and b
                let result_4_bit = (a & b) & 0xf;

                // append the 4 bit output to the output accumulator
                (output << 4) | result_4_bit
            };

            let row = BitwiseTraceRow {
                selector: BITWISE_AND,
                a: Felt::new(a),
                b: Felt::new(b),
                // decompose the 4 bit limbs of a and b
                a0: Felt::new(a & 1),
                a1: Felt::new((a >> 1) & 1),
                a2: Felt::new((a >> 2) & 1),
                a3: Felt::new((a >> 3) & 1),
                b0: Felt::new(b & 1),
                b1: Felt::new((b >> 1) & 1),
                b2: Felt::new((b >> 2) & 1),
                b3: Felt::new((b >> 3) & 1),
                prev_output: Felt::new(prev_output),
                output: Felt::new(output),
            };

            self.rows.push(row);
        }

        Ok(Felt::new(output))
    }

    /// Computes a bitwise XOR of `a` and `b` and returns the result. We assume that `a` and `b`
    /// are 32-bit values. If that's not the case, the result of the computation is undefined.
    ///
    /// This also adds 8 rows to the internal execution trace table required for computing the
    /// operation.
    pub fn u32xor(&mut self, a: Felt, b: Felt) -> Result<Felt, ExecutionError> {
        let a = assert_u32(a)?;
        let b = assert_u32(b)?;
        let mut prev_output;
        let mut output = 0u64;

        // append 8 rows to the trace, each row computing bitwise XOR in 4 bit limbs starting with
        // the most significant limb.
        for bit_offset in (0..32).step_by(4).rev() {
            // shift a and b so that the next 4-bit limb is in the least significant position
            let a = a >> bit_offset;
            let b = b >> bit_offset;

            // compute the output
            prev_output = output;
            output = {
                // compute bitwise XOR of the 4 least significant bits of a and b
                let result_4_bit = (a ^ b) & 0xf;

                // append the 4 bit output to the output accumulator
                (output << 4) | result_4_bit
            };

            let row = BitwiseTraceRow {
                selector: BITWISE_XOR,
                a: Felt::new(a),
                b: Felt::new(b),
                // decompose the 4 bit limbs of a and b
                a0: Felt::new(a & 1),
                a1: Felt::new((a >> 1) & 1),
                a2: Felt::new((a >> 2) & 1),
                a3: Felt::new((a >> 3) & 1),
                b0: Felt::new(b & 1),
                b1: Felt::new((b >> 1) & 1),
                b2: Felt::new((b >> 2) & 1),
                b3: Felt::new((b >> 3) & 1),
                prev_output: Felt::new(prev_output),
                output: Felt::new(output),
            };

            self.rows.push(row);
        }

        Ok(Felt::new(output))
    }

    // EXECUTION TRACE GENERATION
    // --------------------------------------------------------------------------------------------

    /// Fills the provided trace fragment with trace data from this bitwise helper instance.
    pub fn fill_trace(self, trace: &mut TraceFragment) {
        // make sure fragment dimensions are consistent with the dimensions of this trace
        debug_assert_eq!(self.trace_len(), trace.len(), "inconsistent trace lengths");
        debug_assert_eq!(TRACE_WIDTH, trace.width(), "inconsistent trace widths");

        for (row_idx, row) in self.rows.into_iter().enumerate() {
            let row_idx: RowIndex = (row_idx as u32).into();

            trace.set(row_idx, 0, row.selector);
            trace.set(row_idx, A_COL_IDX, row.a);
            trace.set(row_idx, B_COL_IDX, row.b);
            trace.set(row_idx, A_COL_RANGE.start, row.a0);
            trace.set(row_idx, A_COL_RANGE.start + 1, row.a1);
            trace.set(row_idx, A_COL_RANGE.start + 2, row.a2);
            trace.set(row_idx, A_COL_RANGE.start + 3, row.a3);
            trace.set(row_idx, B_COL_RANGE.start, row.b0);
            trace.set(row_idx, B_COL_RANGE.start + 1, row.b1);
            trace.set(row_idx, B_COL_RANGE.start + 2, row.b2);
            trace.set(row_idx, B_COL_RANGE.start + 3, row.b3);
            trace.set(row_idx, PREV_OUTPUT_COL_IDX, row.prev_output);
            trace.set(row_idx, OUTPUT_COL_IDX, row.output);
        }
    }
}

impl Default for Bitwise {
    fn default() -> Self {
        Self::new()
    }
}

// HELPER FUNCTIONS
// --------------------------------------------------------------------------------------------

fn assert_u32(value: Felt) -> Result<u64, ExecutionError> {
    let val_u64 = value.as_int();
    if val_u64 > u32::MAX.into() {
        Err(ExecutionError::NotU32Value(value, ZERO))
    } else {
        Ok(val_u64)
    }
}
