use super::{uninit_vector, BTreeMap, ColMatrix, Felt, FieldElement, Vec, NUM_RAND_ROWS};
use miden_air::trace::range::{M_COL_IDX, V_COL_IDX};

// AUXILIARY TRACE BUILDER
// ================================================================================================

/// Describes how to construct the execution trace of columns related to the range checker in the
/// auxiliary segment of the trace. These are used in multiset checks.
pub struct AuxTraceBuilder {
    /// Range check lookups performed by all user operations, grouped and sorted by the clock cycle
    /// at which they are requested.
    cycle_range_checks: BTreeMap<u32, Vec<Felt>>,
    // The index of the first row of Range Checker's trace when the padded rows end and values to
    // be range checked start.
    values_start: usize,
}

impl AuxTraceBuilder {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    pub fn new(cycle_range_checks: BTreeMap<u32, Vec<Felt>>, values_start: usize) -> Self {
        Self {
            cycle_range_checks,
            values_start,
        }
    }

    // AUX COLUMN BUILDERS
    // --------------------------------------------------------------------------------------------

    /// Builds and returns range checker auxiliary trace columns. Currently this consists of two
    /// columns:
    /// - `b_range`: ensures that the range checks performed by the Range Checker match those
    ///   requested by the Stack and Memory processors.
    pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &ColMatrix<Felt>,
        rand_elements: &[E],
    ) -> Vec<Vec<E>> {
        let b_range = self.build_aux_col_b_range(main_trace, rand_elements);
        vec![b_range]
    }

    /// Builds the execution trace of the range check `b_range` and `q` columns which ensure that the
    /// range check lookups performed by user operations match those executed by the Range Checker.
    fn build_aux_col_b_range<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &ColMatrix<Felt>,
        alphas: &[E],
    ) -> Vec<E> {
        // TODO: replace this with an efficient solution
        // // compute the inverses for range checks performed by operations.
        // let (_, inv_row_values) =
        //     build_lookup_table_row_values(&self.cycle_range_check_values(), main_trace, alphas);

        // allocate memory for the running sum column and set the initial value to ONE
        let mut b_range = unsafe { uninit_vector(main_trace.num_rows()) };
        b_range[0] = E::ONE;

        // keep track of the last updated row in the `b_range` running sum column. `b_range` is
        // filled with result values that are added to the next row after the operation's execution.
        let mut b_range_idx = 0_usize;

        // the first half of the trace only includes values from the operations.
        for (clk, range_checks) in self.cycle_range_checks.range(0..self.values_start as u32) {
            let clk = *clk as usize;

            // if we skipped some cycles since the last update was processed, values in the last
            // updated row should be copied over until the current cycle.
            if b_range_idx < clk {
                let last_value = b_range[b_range_idx];
                b_range[(b_range_idx + 1)..=clk].fill(last_value);
            }

            // move the column pointer to the next row.
            b_range_idx = clk + 1;

            b_range[b_range_idx] = b_range[clk];
            // include the operation lookups
            for lookup in range_checks.iter() {
                let value = (alphas[0] - (*lookup).into()).inv();
                b_range[b_range_idx] -= value;
            }
        }

        // if we skipped some cycles since the last update was processed, values in the last
        // updated row should by copied over until the current cycle.
        if b_range_idx < self.values_start {
            let last_value = b_range[b_range_idx];
            b_range[(b_range_idx + 1)..=self.values_start].fill(last_value);
        }

        // after the padded section of the range checker table, include the lookup value specified
        // by the range checker into the running sum at each step, and remove lookups from user ops
        // at any step where user ops were executed.
        for (row_idx, (multiplicity, lookup)) in main_trace
            .get_column(M_COL_IDX)
            .iter()
            .zip(main_trace.get_column(V_COL_IDX).iter())
            .enumerate()
            .take(main_trace.num_rows() - NUM_RAND_ROWS)
            .skip(self.values_start)
        {
            b_range_idx = row_idx + 1;

            // add the value in the range checker: multiplicity / (alpha - lookup)
            let lookup_val = (alphas[0] - (*lookup).into()).inv().mul_base(*multiplicity);
            b_range[b_range_idx] = b_range[row_idx] + lookup_val;
            // subtract the range checks requested by operations
            if let Some(range_checks) = self.cycle_range_checks.get(&(row_idx as u32)) {
                for lookup in range_checks.iter() {
                    let value = (alphas[0] - (*lookup).into()).inv();
                    b_range[b_range_idx] -= value;
                }
            }
        }

        // at this point, all range checks from user operations and the range checker should be
        // matched - so, the last value must be ONE;
        assert_eq!(b_range[b_range_idx], E::ONE);

        if b_range_idx < b_range.len() - 1 {
            b_range[(b_range_idx + 1)..].fill(E::ONE);
        }

        b_range
    }
}
