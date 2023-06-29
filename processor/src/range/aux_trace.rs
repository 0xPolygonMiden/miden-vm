use super::{
    build_lookup_table_row_values, uninit_vector, BTreeMap, ColMatrix, CycleRangeChecks, Felt,
    FieldElement, RangeCheckFlag, Vec, NUM_RAND_ROWS,
};
use miden_air::trace::range::V_COL_IDX;

// AUXILIARY TRACE BUILDER
// ================================================================================================

/// Describes how to construct the execution trace of columns related to the range checker in the
/// auxiliary segment of the trace. These are used in multiset checks.
pub struct AuxTraceBuilder {
    // Range check lookups performed by all user operations, grouped and sorted by clock cycle. Each
    // cycle is mapped to a single CycleRangeChecks instance which includes lookups from the stack,
    // memory, or both.
    // TODO: once we switch to backfilling memory range checks this approach can change to tracking
    // vectors of hints and rows like in the Stack and Hasher AuxTraceBuilders, and the
    // CycleRangeChecks struct can be removed.
    cycle_range_checks: BTreeMap<u32, CycleRangeChecks>,
    // A trace-length vector of RangeCheckFlags which indicate how many times the range check value
    // at that row should be included in the trace.
    row_flags: Vec<RangeCheckFlag>,
    // The index of the first row of Range Checker's trace when the padded rows end and values to
    // be range checked start.
    values_start: usize,
}

impl AuxTraceBuilder {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    pub fn new(
        cycle_range_checks: BTreeMap<u32, CycleRangeChecks>,
        row_flags: Vec<RangeCheckFlag>,
        values_start: usize,
    ) -> Self {
        Self {
            cycle_range_checks,
            row_flags,
            values_start,
        }
    }

    // ACCESSORS
    // --------------------------------------------------------------------------------------------
    pub fn cycle_range_check_values(&self) -> Vec<CycleRangeChecks> {
        self.cycle_range_checks.values().cloned().collect()
    }

    // AUX COLUMN BUILDERS
    // --------------------------------------------------------------------------------------------

    /// Builds and returns range checker auxiliary trace columns. Currently this consists of three
    /// columns:
    /// - `b_range`: ensures that the range checks performed by the Range Checker match those
    ///   requested
    ///    by the Stack and Memory processors.
    /// - `q`: a helper column of intermediate values to reduce the degree of the constraints for
    ///    `b_range`. It contains the product of the lookups performed by the Stack at each row.
    pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &ColMatrix<Felt>,
        rand_elements: &[E],
    ) -> Vec<Vec<E>> {
        let (b_range, q) = self.build_aux_col_b_range(main_trace, rand_elements);
        vec![b_range, q]
    }

    /// Builds the execution trace of the range check `b_range` and `q` columns which ensure that the
    /// range check lookups performed by user operations match those executed by the Range Checker.
    fn build_aux_col_b_range<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &ColMatrix<Felt>,
        alphas: &[E],
    ) -> (Vec<E>, Vec<E>) {
        // compute the inverses for range checks performed by operations.
        let (_, inv_row_values) =
            build_lookup_table_row_values(&self.cycle_range_check_values(), main_trace, alphas);

        // allocate memory for the running product column and set the initial value to ONE
        let mut q = unsafe { uninit_vector(main_trace.num_rows()) };
        let mut b_range = unsafe { uninit_vector(main_trace.num_rows()) };
        q[0] = E::ONE;
        b_range[0] = E::ONE;

        // keep track of the last updated row in the `b_range` running product column. the `q`
        // column index is always one row behind, since `q` is filled with intermediate values in
        // the same row as the operation is executed, whereas `b_range` is filled with result
        // values that are added to the next row after the operation's execution.
        let mut b_range_idx = 0_usize;
        // keep track of the next row to be included from the user op range check values.
        let mut rc_user_op_idx = 0;

        // the first half of the trace only includes values from the operations.
        for (clk, range_checks) in self.cycle_range_checks.range(0..self.values_start as u32) {
            let clk = *clk as usize;

            // if we skipped some cycles since the last update was processed, values in the last
            // updated row should by copied over until the current cycle.
            if b_range_idx < clk {
                let last_value = b_range[b_range_idx];
                b_range[(b_range_idx + 1)..=clk].fill(last_value);
                q[b_range_idx..clk].fill(E::ONE);
            }

            // move the column pointers to the next row.
            b_range_idx = clk + 1;

            // update the intermediate values in the q column.
            q[clk] = range_checks.to_stack_value(main_trace, alphas);

            // include the operation lookups in the running product.
            b_range[b_range_idx] = b_range[clk] * inv_row_values[rc_user_op_idx];
            rc_user_op_idx += 1;
        }

        // if we skipped some cycles since the last update was processed, values in the last
        // updated row should by copied over until the current cycle.
        if b_range_idx < self.values_start {
            let last_value = b_range[b_range_idx];
            b_range[(b_range_idx + 1)..=self.values_start].fill(last_value);
            q[b_range_idx..self.values_start].fill(E::ONE);
        }

        // after the padded section of the range checker table, include `z` in the running product
        // at each step and remove lookups from user ops at any step where user ops were executed.
        for (row_idx, (hint, lookup)) in self
            .row_flags
            .iter()
            .zip(main_trace.get_column(V_COL_IDX).iter())
            .enumerate()
            .take(main_trace.num_rows() - NUM_RAND_ROWS)
            .skip(self.values_start)
        {
            b_range_idx = row_idx + 1;

            b_range[b_range_idx] = b_range[row_idx] * hint.to_value(*lookup, alphas);

            if let Some(range_check) = self.cycle_range_checks.get(&(row_idx as u32)) {
                // update the intermediate values in the q column.
                q[row_idx] = range_check.to_stack_value(main_trace, alphas);

                // include the operation lookups in the running product.
                b_range[b_range_idx] *= inv_row_values[rc_user_op_idx];
                rc_user_op_idx += 1;
            } else {
                q[row_idx] = E::ONE;
            }
        }

        // at this point, all range checks from user operations and the range checker should be
        // matched - so, the last value must be ONE;
        assert_eq!(q[b_range_idx - 1], E::ONE);
        assert_eq!(b_range[b_range_idx], E::ONE);

        if (b_range_idx - 1) < b_range.len() - 1 {
            q[b_range_idx..].fill(E::ONE);
        }
        if b_range_idx < b_range.len() - 1 {
            b_range[(b_range_idx + 1)..].fill(E::ONE);
        }

        (b_range, q)
    }
}
