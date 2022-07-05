use vm_core::range::V_COL_IDX;
use winterfell::Matrix;

use super::{Felt, FieldElement, RangeCheckFlag, Vec};
use crate::trace::NUM_RAND_ROWS;

// AUXILIARY TRACE BUILDER
// ================================================================================================

/// Describes how to construct the execution trace of columns related to the range checker in the
/// auxiliary segment of the trace. These are used in multiset checks.
pub struct AuxTraceBuilder {
    row_flags: Vec<RangeCheckFlag>,
    start_16bit: usize,
}

impl AuxTraceBuilder {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    pub fn new(row_flags: Vec<RangeCheckFlag>, start_16bit: usize) -> Self {
        Self {
            row_flags,
            start_16bit,
        }
    }

    // AUX COLUMN BUILDERS
    // --------------------------------------------------------------------------------------------

    /// Builds and returns range checker auxiliary trace columns. Currently this consists of three
    /// columns:
    /// - `p0`: ensures that the range checker table is internally consistent between the 8-bit and
    ///    16-bit sections.
    /// - `p1`: ensures that the range checks performed by the Range Checker match those requested
    ///    by the Stack and Memory processors.
    /// - `q`: a helper column of intermediate values to reduce the degree of the constraints for
    ///    `p1`. It contains the product of the lookups performed by the Stack at each row.
    pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &Matrix<Felt>,
        rand_elements: &[E],
    ) -> Vec<Vec<E>> {
        let p0 = self.build_aux_col_p0(main_trace, rand_elements);
        vec![p0]
    }

    /// Builds the execution trace of the range checker's `p0` auxiliary column used for multiset
    /// checks. The running product is built up in the 8-bit section of the table and reduced in the
    /// 16-bit section of the table so that the starting and ending value are both one.
    fn build_aux_col_p0<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &Matrix<Felt>,
        rand_elements: &[E],
    ) -> Vec<E> {
        let mut aux_column = E::zeroed_vector(main_trace.num_rows());
        let alpha = rand_elements[0];
        let v_col = main_trace.get_column(V_COL_IDX);

        // Set the starting value to one.
        aux_column[0] = E::ONE;

        // Build the execution trace of the 8-bit running product.
        for (row_idx, (hint, lookup)) in self
            .row_flags
            .iter()
            .zip(v_col.iter())
            .enumerate()
            .take(self.start_16bit)
        {
            // This is the 8-bit section, where the running product must be built up.
            aux_column[row_idx + 1] = aux_column[row_idx] * hint.to_value(*lookup, rand_elements);
        }

        // Accumulate the value differences for each transition and their product in preparation for
        // using a modified batch inversion to build the execution trace of the 16-bit section where the
        // running product must be reduced by the value difference at each row with offset alpha:
        // (alpha + v' - v).
        let mut diff_values = Vec::with_capacity(v_col.len() - self.start_16bit - NUM_RAND_ROWS);
        let mut acc = E::ONE;
        for (row_idx, &v) in v_col
            .iter()
            .enumerate()
            .take(v_col.len() - 1)
            .skip(self.start_16bit)
        {
            // This is the 16-bit section, where the running product must be reduced.
            let v_next = v_col[row_idx + 1].into();
            let value = alpha + v_next - v.into();

            // Accumulate the transition difference values by which the running product must be reduced.
            diff_values.push(value);

            // Accumulate the product of the differences.
            if value != E::ZERO {
                acc *= value;
            }
        }

        // Invert the accumulated product and multiply it by the result from the 8-bit section.
        acc = acc.inv() * aux_column[self.start_16bit];

        // Do a modified version of batch inversion. We don't actually want an array of inverted
        // diff_values [1/a, 1/b, 1/c, ...], we want an array of inverted products all of which are
        // multiplied by the same 8-bit result `res`, e.g. [res/a, res/ab, res/abc, ...].
        for idx in (0..diff_values.len()).rev() {
            aux_column[self.start_16bit + idx + 1] = acc;
            if diff_values[idx] != E::ZERO {
                acc *= diff_values[idx];
            }
        }

        aux_column
    }
}
