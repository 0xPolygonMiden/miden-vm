use super::{ExecutionError, Felt, FieldElement, StarkField};

// CONSTANTS
// ================================================================================================

/// Number of columns needed to record an execution trace of the power of two helper.
const TRACE_WIDTH: usize = 12;

/// The maximum power of two that can be added to the trace per row.
const POWERS_PER_ROW: usize = 8;
/// Column index of the helper column `h`.
const HELPER_COL: usize = POWERS_PER_ROW;
/// Column index for the aggregated value of `a` which has been added to the trace.
const AGG_POWER_COL: usize = HELPER_COL + 1;
/// Column index for powers of 256.
const POW_256_COL: usize = AGG_POWER_COL + 1;
/// Column index for the aggregated result of the power of two operation.
const AGG_OUTPUT_COL: usize = POW_256_COL + 1;

// POWER OF TWO HELPER
// ================================================================================================

/// Helper for the VM that computes power of two operations 2^a for an exponent a < 64. It also
/// builds an execution trace of this operation.
///
/// ## Execution trace
/// The execution trace for each operation consists of 8 rows and 12 columns.
///
/// The layout of the table is illustrated below.
///
///   a0    a1     a2    a3    a4   a5     a6   a7     h     a     p     z
/// ├─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┤
///
/// In the above, the meaning of the columns is as follows:
/// - Columns `a0` through `a7` contain binary values representing a single power of two. The values
///   are set by decomposing the input exponent `a` one power at a time until the entire exponent
///   `a` is represented in the `a0` through `a7` trace columns. Between one cell and the next the
///   values can change from 1 -> 0 or stay the same. 0 -> 1 is always an invalid transition. Thus,
///   by the end of the operation, the number of 1's in the `a0` to `a7` columns over all 8 columns
///   will equal the input value `a`.
/// - Column `h` contains a binary value that helps decompose `a` correctly into the columns `a0`
///   through `a7`. `h` must always equal the value of `a0` on the next row, except in the 8th row
///   of the operation trace, where it must equal 0.
/// - Column `a` contains the aggregated exponent value that has been processed in the `a0` to `a7`
///   trace columns so far. Thus, by the 8th row, column `a` contains the full input value for the
///   power of two operation.
/// - Column `p` contains increasing powers of 256, starting from 256^0, which are used to aggregate
///   the output of the power of two operation.
/// - Column `z` contains the aggregated result of the power of two operation that has been
///   computed so far. Thus, by the 8th row, column `z` contains `2^a` for input value `a`.
pub struct PowerOfTwo {
    trace: [Vec<Felt>; TRACE_WIDTH],
}

impl PowerOfTwo {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [PowerOfTwo] initialized with an empty trace.
    pub fn new() -> Self {
        let trace = (0..TRACE_WIDTH)
            .map(|_| Vec::new())
            .collect::<Vec<_>>()
            .try_into()
            .expect("Failed to convert to an array");

        Self { trace }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns length of execution trace required to describe power of two operations executed in
    /// the VM.
    #[allow(dead_code)]
    pub fn trace_len(&self) -> usize {
        self.trace[0].len()
    }

    // TRACE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Returns 2^exponent for the provided exponent value and adds the operation execution to the
    /// trace.
    ///
    /// # Errors
    /// Returns an error if the exponent is out of range (greater than 63).
    pub fn pow2(&mut self, exponent: Felt) -> Result<Felt, ExecutionError> {
        if exponent.as_int() > 63 {
            return Err(ExecutionError::InvalidPowerOfTwo(exponent));
        }

        // The power to decompose into single powers of two.
        let mut power_to_decomp = exponent.as_int();
        // The aggregated power that has been added to the trace.
        let mut agg_power = 0;
        // The aggregated output result.
        let mut agg_output = Felt::ZERO;

        // Append 8 rows to the trace.
        for row in 0_u32..8 {
            self.add_trace_row(&mut power_to_decomp, &mut agg_power, &mut agg_output, row);
        }

        Ok(agg_output)
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Appends a new row to the trace table and populates the 12 columns of trace as follows:
    /// - Columns 0 to 7 are each set to 1 for each power of two that needs to be calculated.
    /// - Column 8 is a helper column for constraints and is set to the value of column 0 in the
    ///   next row.
    /// - Column 9 is set to the value of the total aggregated power that has been added to the
    ///   trace so far.
    /// - Column 10 is the power of 256 for the row.
    /// - Column 11 is the aaggregated output so far.
    fn add_trace_row(
        &mut self,
        power_to_decomp: &mut u64,
        agg_power: &mut u64,
        agg_output: &mut Felt,
        row: u32,
    ) {
        // The row's power of 256, used for computing the aggregated output.
        let power_of_256 = Felt::new(256_u64.pow(row));

        if *power_to_decomp == 0 {
            if row == 0 {
                // If it's the first row then the input exponent is 0; Update the output.
                *agg_output += Felt::ONE;
            }
            // We're done decomposing the exponent.
            for idx in 0..=HELPER_COL {
                // Set all decomposition columns and the helper column to zero.
                self.trace[idx].push(Felt::ZERO);
            }

            // Add all previously aggregated values to the trace without modification.
        } else if *power_to_decomp <= POWERS_PER_ROW as u64 {
            // The decomposition ends in this row and the final output is computed.
            let final_decomp_idx = *power_to_decomp as usize;
            for idx in 0..final_decomp_idx {
                // Decompose the remaining powers.
                self.trace[idx].push(Felt::ONE);
            }
            for idx in final_decomp_idx..=HELPER_COL {
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
            for idx in 0..=HELPER_COL {
                // Set all decomposition columns and the helper column to one.
                self.trace[idx].push(Felt::ONE);
            }

            // Update the aggregated values.
            *agg_power += POWERS_PER_ROW as u64;
            *power_to_decomp -= POWERS_PER_ROW as u64;
        }

        // Set the aggregated power of two value that has been decomposed into the trace so far.
        self.trace[AGG_POWER_COL].push(Felt::new(*agg_power));

        // Set the power of 256 for the row.
        self.trace[POW_256_COL].push(power_of_256);

        // Set the output value.
        self.trace[AGG_OUTPUT_COL].push(*agg_output);
    }
}

// TESTS
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{Felt, FieldElement, PowerOfTwo, AGG_OUTPUT_COL};

    #[test]
    fn pow2() {
        let mut power_of_two = PowerOfTwo::new();

        // --- ensure correct results -------------------------------------------------------------
        // Minimum exponent value.
        let result = power_of_two.pow2(Felt::ZERO).unwrap();
        let trace_result = power_of_two.trace[AGG_OUTPUT_COL].last().unwrap();
        assert_eq!(result, Felt::ONE);
        assert_eq!(trace_result, &result);

        // Power decomposition ends at end of row.
        let result = power_of_two.pow2(Felt::new(8)).unwrap();
        let trace_result = power_of_two.trace[AGG_OUTPUT_COL].last().unwrap();
        assert_eq!(result, Felt::new(2_u64.pow(8)));
        assert_eq!(trace_result, &result);

        // Power decomposition ends at start of row.
        let result = power_of_two.pow2(Felt::new(9)).unwrap();
        let trace_result = power_of_two.trace[AGG_OUTPUT_COL].last().unwrap();
        assert_eq!(result, Felt::new(2_u64.pow(9)));
        assert_eq!(trace_result, &result);

        // Maximumm exponent value.
        let result = power_of_two.pow2(Felt::new(63)).unwrap();
        let trace_result = power_of_two.trace[AGG_OUTPUT_COL].last().unwrap();
        assert_eq!(result, Felt::new(2_u64.pow(63)));
        assert_eq!(trace_result, &result);

        // --- check the trace --------------------------------------------------------------------
        // The trace length should equal four full power-of-two operation cycles.
        assert_eq!(power_of_two.trace_len(), 32);
    }

    #[test]
    fn pow2_fail() {
        let mut power_of_two = PowerOfTwo::new();

        // --- ensure failure with out-of-bounds exponent -----------------------------------------
        let result = power_of_two.pow2(Felt::new(64));
        assert!(result.is_err());
    }
}
