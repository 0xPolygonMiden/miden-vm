use super::{ExecutionError, Felt, FieldElement, StarkField, TraceFragment};
use vm_core::bitwise::{
    BITWISE_AND, BITWISE_OR, BITWISE_XOR, NUM_SELECTORS, POW2_POWERS_PER_ROW, POWER_OF_TWO,
    TRACE_WIDTH,
};

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

/// Initial capacity of each column.
const INIT_TRACE_CAPACITY: usize = 128;

/// Column index of the helper column `h`, which indicates if the power decomposition continues onto
/// the next row or not.
const POW2_HELPER_COL: usize = NUM_SELECTORS + POW2_POWERS_PER_ROW;
/// Column index for the aggregated value of `a` which has been added to the trace.
const POW2_AGG_POWER_COL: usize = POW2_HELPER_COL + 1;
/// Column index for powers of 256.
const POW2_POW_256_COL: usize = POW2_AGG_POWER_COL + 1;
/// Column index for the aggregated result of the power of two operation.
const POW2_AGG_OUTPUT_COL: usize = POW2_POW_256_COL + 1;

// TYPE ALIASES
// ================================================================================================

type Selectors = [Felt; NUM_SELECTORS];

// BITWISE + POWER OF TWO HELPER
// ================================================================================================

/// Helper for the VM that computes AND, OR, and XOR bitwise operations on 32-bit values and power
/// of two operations 2^a for an exponent a < 64. It also builds an execution trace of these
/// operations. The execution trace takes a different form depending on whether the operation is
/// bitwise or power of two.
///
/// ## Bitwise operation execution trace (AND, OR, XOR)
/// The execution trace for each operation consists of 8 rows and 14 columns, the last column of
/// which is a padding column that enables combining this trace with that of the 14-column power of
/// two operation. At a high level, we break input values into 4-bit limbs, apply the bitwise
/// operation to these limbs at every row starting with the most significant limb, and accumulate
/// the result in the result column.
///
/// The layout of the table is illustrated below.
///
///    s0    s1    a     b      a0     a1     a2     a3     b0     b1     b2     b3     c     0
/// ├─────┴─────┴─────┴─────┴───────┴──────┴──────┴──────┴──────┴──────┴──────┴──────┴─────┴─────┤
///
/// In the above, the meaning of the columns is as follows:
/// - Selector columns s0 and s1 are used to specify the bitwise operator for each row.
/// - Columns `a` and `b` contain accumulated 4-bit limbs of input values. Specifically, at the
///   first row, the values of columns `a` and `b` are set to the most significant 4-bit limb
///   of each input value. With all subsequent rows, the next most significant limb is appended
///   to each column for the corresponding value. Thus, by the 8th row, columns `a` and `b`
///   contain full input values for the bitwise operation.
/// - Columns `a0` through `a3` and `b0` through `b3` contain bits of the least significant 4-bit
///   limb of the values in `a` and `b` columns respectively.
/// - Column `c` contains the accumulated result of applying the bitwise operation to 4-bit limbs.
///   At the first row, column `c` contains the result of bitwise operation applied to the most
///   significant 4-bit limbs of the input values. With every subsequent row, the next most
///   significant 4-bit limb of the result is appended to it. Thus, by the 8th row, column `c`
///   contains the full result of the bitwise operation.
/// - Column `0` is a padded column of zeros which is added so that the width of the bitwise trace
///   equals the width required by the execution trace for power of two operations.
///
/// ## Power of Two operation execution trace (2^a for a < 64)
/// The execution trace for each operation consists of 8 rows and 14 columns.
///
/// The layout of the table is illustrated below.
///
///    s0    s1   a0    a1     a2    a3    a4   a5     a6   a7     h     a     p     z
/// ├─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┤
///
/// In the above, the meaning of the columns is as follows:
/// - Selector columns s0 and s1 are used to specify the power of two operation.
/// - Columns `a0` through `a7` contain binary values representing a single power of two. The values
///   are set by decomposing the input exponent `a` one power at a time until the entire exponent
///   `a` is represented in the `a0` through `a7` trace columns. Between one cell and the next the
///   values can change from 1 -> 0 or stay the same. 0 -> 1 is always an invalid transition. Thus,
///   by the end of the operation, the number of 1's in the `a0` to `a7` columns over all 8 columns
///   will equal the input value `a`, and the value of `a7` in the 8th row will always equal 0.
/// - Column `h` contains a binary value that helps decompose `a` correctly into the columns `a0`
///   through `a7`. `h` must always equal the value of `a0` on the subsequent row, except in the 8th
///   row of the operation trace, where it must equal 0.
/// - Column `a` contains the aggregated exponent value that has been processed in the `a0` to `a7`
///   trace columns so far. Thus, by the 8th row, column `a` contains the full input value for the
///   power of two operation.
/// - Column `p` contains increasing powers of 256, starting from 256^0. These are used to aggregate
///   the output of the power of two operation.
/// - Column `z` contains the aggregated result of the power of two operation that has been
///   computed so far. Thus, by the 8th row, column `z` contains `2^a` for input value `a`.
pub struct Bitwise {
    trace: [Vec<Felt>; TRACE_WIDTH],
}

impl Bitwise {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [Bitwise] initialized with an empty trace.
    pub fn new() -> Self {
        let trace = (0..TRACE_WIDTH)
            .map(|_| Vec::with_capacity(INIT_TRACE_CAPACITY))
            .collect::<Vec<_>>()
            .try_into()
            .expect("failed to convert vector to array");
        Self { trace }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns length of execution trace required to describe bitwise operations executed on the
    /// VM.
    pub fn trace_len(&self) -> usize {
        self.trace[0].len()
    }

    // TRACE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Computes a bitwise AND of `a` and `b` and returns the result. We assume that `a` and `b`
    /// are 32-bit values. If that's not the case, the result of the computation is undefined.
    ///
    /// This also adds 8 rows to the internal execution trace table required for computing the
    /// operation.
    pub fn u32and(&mut self, a: Felt, b: Felt) -> Result<Felt, ExecutionError> {
        let a = assert_u32(a)?.as_int();
        let b = assert_u32(b)?.as_int();
        let mut result = 0u64;

        // append 8 rows to the trace, each row computing bitwise AND in 4 bit limbs starting with
        // the most significant limb.
        for bit_offset in (0..32).step_by(4).rev() {
            // shift a and b so that the next 4-bit limb is in the least significant position
            let a = a >> bit_offset;
            let b = b >> bit_offset;

            // add a new row to the trace table and populate it with binary decomposition of the 4
            // least significant bits of a and b.
            self.add_bitwise_trace_row(BITWISE_AND, a, b);

            // compute bitwise AND of the 4 least significant bits of a and b
            let result_4_bit = (a & b) & 0xF;

            // append the 4 bit result to the result accumulator, and save the current result into
            // the 13th column of the trace.
            result = (result << 4) | result_4_bit;
            self.trace[12].push(Felt::new(result));
        }

        Ok(Felt::new(result))
    }

    /// Computes a bitwise OR of `a` and `b` and returns the result. We assume that `a` and `b`
    /// are 32-bit values. If that's not the case, the result of the computation is undefined.
    ///
    /// This also adds 8 rows to the internal execution trace table required for computing the
    /// operation.
    pub fn u32or(&mut self, a: Felt, b: Felt) -> Result<Felt, ExecutionError> {
        let a = assert_u32(a)?.as_int();
        let b = assert_u32(b)?.as_int();
        let mut result = 0u64;

        // append 8 rows to the trace, each row computing bitwise OR in 4 bit limbs starting with
        // the most significant limb.
        for bit_offset in (0..32).step_by(4).rev() {
            // shift a and b so that the next 4-bit limb is in the least significant position
            let a = a >> bit_offset;
            let b = b >> bit_offset;

            // add a new row to the trace table and populate it with binary decomposition of the 4
            // least significant bits of a and b.
            self.add_bitwise_trace_row(BITWISE_OR, a, b);

            // compute bitwise OR of the 4 least significant bits of a and b
            let result_4_bit = (a | b) & 0xF;

            // append the 4 bit result to the result accumulator, and save the current result into
            // the 13th column of the trace.
            result = (result << 4) | result_4_bit;
            self.trace[12].push(Felt::new(result));
        }

        Ok(Felt::new(result))
    }

    /// Computes a bitwise XOR of `a` and `b` and returns the result. We assume that `a` and `b`
    /// are 32-bit values. If that's not the case, the result of the computation is undefined.
    ///
    /// This also adds 8 rows to the internal execution trace table required for computing the
    /// operation.
    pub fn u32xor(&mut self, a: Felt, b: Felt) -> Result<Felt, ExecutionError> {
        let a = assert_u32(a)?.as_int();
        let b = assert_u32(b)?.as_int();
        let mut result = 0u64;

        // append 8 rows to the trace, each row computing bitwise XOR in 4 bit limbs starting with
        // the most significant limb.
        for bit_offset in (0..32).step_by(4).rev() {
            // shift a and b so that the next 4-bit limb is in the least significant position
            let a = a >> bit_offset;
            let b = b >> bit_offset;

            // add a new row to the trace table and populate it with binary decomposition of the 4
            // least significant bits of a and b.
            self.add_bitwise_trace_row(BITWISE_XOR, a, b);

            // compute bitwise XOR of the 4 least significant bits of a and b
            let result_4_bit = (a ^ b) & 0xF;

            // append the 4 bit result to the result accumulator, and save the current result into
            // the 13th column of the trace.
            result = (result << 4) | result_4_bit;
            self.trace[12].push(Felt::new(result));
        }

        Ok(Felt::new(result))
    }

    /// Returns 2^exponent for the provided exponent value and adds the operation execution to the
    /// trace.
    ///
    /// # Errors
    /// Returns an error if the exponent is out of range (greater than 63).
    pub fn pow2(&mut self, exponent: Felt) -> Result<Felt, ExecutionError> {
        if exponent.as_int() > 63 {
            return Err(ExecutionError::InvalidPowerOfTwo(exponent));
        }

        // The power which must be decomposed into individual powers of two.
        let mut power_to_decomp = exponent.as_int();
        // The aggregated power that has been added to the trace.
        let mut agg_power = 0;
        // The aggregated output result.
        let mut agg_output = Felt::ZERO;

        // Append 8 rows to the trace.
        for row in 0_u32..8 {
            self.add_pow2_trace_row(&mut power_to_decomp, &mut agg_power, &mut agg_output, row);
        }

        Ok(agg_output)
    }

    // EXECUTION TRACE GENERATION
    // --------------------------------------------------------------------------------------------

    /// Fills the provide trace fragment with trace data from this bitwise helper instance.
    pub fn fill_trace(self, trace: &mut TraceFragment) {
        // make sure fragment dimensions are consistent with the dimensions of this trace
        debug_assert_eq!(self.trace_len(), trace.len(), "inconsistent trace lengths");
        debug_assert_eq!(TRACE_WIDTH, trace.width(), "inconsistent trace widths");

        // copy trace into the fragment column-by-column
        // TODO: this can be parallelized to copy columns in multiple threads
        for (out_column, column) in trace.columns().zip(self.trace) {
            out_column.copy_from_slice(&column);
        }
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Appends a new row to the trace table and populates the first 12 column of trace as follows:
    /// - Columns 0 and 1 are set to the selector values for the bitwise operation being executed.
    /// - Column 2 is set to the current value of `a`.
    /// - Column 3 is set to the current value of `b`.
    /// - Columns 4 to 7 are set to the 4 least-significant bits of `a`.
    /// - Columns 8 to 11 are set to the 4 least-significant bits of `b`.
    /// - Column 12 is left for the output value which is set elsewhere.
    /// - Column 13 is padded with 0 so the width of the bitwise trace row will equal the width of
    ///   power of two operation trace rows.
    fn add_bitwise_trace_row(&mut self, selectors: Selectors, a: u64, b: u64) {
        self.trace[0].push(selectors[0]);
        self.trace[1].push(selectors[1]);

        self.trace[2].push(Felt::new(a));
        self.trace[3].push(Felt::new(b));

        self.trace[4].push(Felt::new(a & 1));
        self.trace[5].push(Felt::new((a >> 1) & 1));
        self.trace[6].push(Felt::new((a >> 2) & 1));
        self.trace[7].push(Felt::new((a >> 3) & 1));

        self.trace[8].push(Felt::new(b & 1));
        self.trace[9].push(Felt::new((b >> 1) & 1));
        self.trace[10].push(Felt::new((b >> 2) & 1));
        self.trace[11].push(Felt::new((b >> 3) & 1));

        // Pad the final column.
        self.trace[13].push(Felt::ZERO);
    }

    /// Appends a new row to the trace table and populates the 12 columns of trace as follows:
    /// - Columns 0 and 1 are set to the selector values for the power of two operation.
    /// - Columns 2 to 9 are each set to 1 for each power of two that needs to be calculated.
    /// - Column 10 is a helper column for constraints and is set to the value of column 0 in the
    ///   next row.
    /// - Column 11 is set to the value of the total aggregated power that has been added to the
    ///   trace so far.
    /// - Column 12 is the power of 256 for the row.
    /// - Column 13 is the aaggregated output so far.
    fn add_pow2_trace_row(
        &mut self,
        power_to_decomp: &mut u64,
        agg_power: &mut u64,
        agg_output: &mut Felt,
        row: u32,
    ) {
        // Add the selectors for the power of two operation.
        self.trace[0].push(POWER_OF_TWO[0]);
        self.trace[1].push(POWER_OF_TWO[1]);

        // The row's power of 256, used for computing the aggregated output.
        let power_of_256 = Felt::new(256_u64.pow(row));

        if *power_to_decomp == 0 {
            if row == 0 {
                // If it's the first row then the input exponent is 0; Update the output.
                *agg_output += Felt::ONE;
            }
            // We're done decomposing the exponent.
            for idx in NUM_SELECTORS..=POW2_HELPER_COL {
                // Set all decomposition columns and the helper column to zero.
                self.trace[idx].push(Felt::ZERO);
            }

            // Add all previously aggregated values to the trace without modification.
        } else if *power_to_decomp <= POW2_POWERS_PER_ROW as u64 {
            // The decomposition ends in this row and the final output is computed.
            let final_decomp_idx = NUM_SELECTORS + *power_to_decomp as usize;
            for idx in NUM_SELECTORS..final_decomp_idx {
                // Decompose the remaining powers.
                self.trace[idx].push(Felt::ONE);
            }
            for idx in final_decomp_idx..=POW2_HELPER_COL {
                // Set the rest of the decomposition columns and the helper column to zero.
                self.trace[idx].push(Felt::ZERO);
            }

            // Aggregate the row output when the exponent decomposition stops.
            *agg_output += power_of_256 * Felt::new(2_u64.pow(*power_to_decomp as u32));
            // Update the aggregated values.
            *agg_power += *power_to_decomp;
            *power_to_decomp -= *power_to_decomp;
        } else {
            // We have more than 8 powers to decompose, so it will continue on the next row.
            for idx in NUM_SELECTORS..=POW2_HELPER_COL {
                // Set all decomposition columns and the helper column to one.
                self.trace[idx].push(Felt::ONE);
            }

            // Update the aggregated values.
            *agg_power += POW2_POWERS_PER_ROW as u64;
            *power_to_decomp -= POW2_POWERS_PER_ROW as u64;
        }

        // Set the aggregated power of two value that has been decomposed into the trace so far.
        self.trace[POW2_AGG_POWER_COL].push(Felt::new(*agg_power));

        // Set the power of 256 for the row.
        self.trace[POW2_POW_256_COL].push(power_of_256);

        // Set the output value.
        self.trace[POW2_AGG_OUTPUT_COL].push(*agg_output);
    }
}

impl Default for Bitwise {
    fn default() -> Self {
        Self::new()
    }
}

// HELPER FUNCTIONS
// --------------------------------------------------------------------------------------------

pub fn assert_u32(value: Felt) -> Result<Felt, ExecutionError> {
    let val_u64 = value.as_int();
    if val_u64 > u32::MAX.into() {
        Err(ExecutionError::NotU32Value(value))
    } else {
        Ok(value)
    }
}
