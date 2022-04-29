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
