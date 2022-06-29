use super::{utils::build_lookup_table_row_values, Felt, FieldElement, Matrix, Vec};
use crate::stack::{AuxTraceHints, OverflowTableUpdate};
use vm_core::utils::uninit_vector;

#[cfg(test)]
mod tests;

// STACK AUXILIARY TRACE COLUMNS
// ================================================================================================

/// Builds and returns stack auxiliary trace column p1 describing states of the stack overflow
/// table.
pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
    main_trace: &Matrix<Felt>,
    aux_trace_hints: &AuxTraceHints,
    rand_elements: &[E],
) -> Vec<Vec<E>> {
    vec![build_aux_col_p1(main_trace, aux_trace_hints, rand_elements)]
}

// STACK OVERFLOW COLUMN
// ================================================================================================

/// Builds the execution trace of the stack's `p1` column which describes the state of the overflow
/// table via multiset checks.
fn build_aux_col_p1<E: FieldElement<BaseField = Felt>>(
    main_trace: &Matrix<Felt>,
    aux_trace_hints: &AuxTraceHints,
    alphas: &[E],
) -> Vec<E> {
    // compute row values and their inverses for all rows that were added to the overflow table
    let table_rows = aux_trace_hints.overflow_table_rows();
    let (row_values, inv_row_values) = build_lookup_table_row_values(table_rows, alphas);

    // allocate memory for the running product column and set the initial value to ONE
    let mut result = unsafe { uninit_vector(main_trace.num_rows()) };
    result[0] = E::ONE;

    // keep track of the last updated row in the running product column
    let mut result_idx = 0;

    // iterate through the list of updates and apply them one by one
    for (clk, update) in aux_trace_hints.overflow_table_hints() {
        let clk = *clk;

        // if we skipped some cycles since the last update was processed, values in the last
        // updated row should by copied over until the current cycle.
        if result_idx < clk {
            let last_value = result[result_idx];
            result[(result_idx + 1)..=clk].fill(last_value);
        }

        // move the result pointer to the next row
        result_idx = clk + 1;

        // apply the relevant updates to the column
        match update {
            OverflowTableUpdate::RowInserted(inserted_row_idx) => {
                result[result_idx] = result[clk] * row_values[*inserted_row_idx as usize];
            }
            OverflowTableUpdate::RowRemoved(removed_row_idx) => {
                result[result_idx] = result[clk] * inv_row_values[*removed_row_idx as usize];
            }
        }
    }

    // at this point, overflow table must be empty - so, the last value must be ONE;
    // we also fill in all the remaining values in the column with ONE's.
    let last_value = result[result_idx];
    assert_eq!(last_value, E::ONE);
    if result_idx < result.len() - 1 {
        result[(result_idx + 1)..].fill(E::ONE);
    }

    result
}
