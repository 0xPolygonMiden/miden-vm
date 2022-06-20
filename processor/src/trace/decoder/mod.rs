use super::{Felt, FieldElement, Matrix, Vec};
use crate::decoder::{AuxTraceHints, OpGroupTableRow, OpGroupTableUpdate};
use vm_core::utils::uninit_vector;

#[cfg(test)]
mod tests;

// DECODER AUXILIARY TRACE COLUMNS
// ================================================================================================

/// TODO: add docs
pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
    main_trace: &Matrix<Felt>,
    aux_trace_hints: &AuxTraceHints,
    rand_elements: &[E],
) -> Vec<Vec<E>> {
    let p1 = build_aux_col_p1(main_trace.num_rows(), aux_trace_hints, rand_elements);
    let p2 = build_aux_col_p2(main_trace.num_rows(), aux_trace_hints, rand_elements);
    let p3 = build_aux_col_p3(main_trace.num_rows(), aux_trace_hints, rand_elements);
    vec![p1, p2, p3]
}

// BLOCK STACK TABLE COLUMN
// ================================================================================================

fn build_aux_col_p1<E: FieldElement<BaseField = Felt>>(
    trace_len: usize,
    _aux_trace_hints: &AuxTraceHints,
    _alphas: &[E],
) -> Vec<E> {
    // TODO: implement
    unsafe { uninit_vector(trace_len) }
}

// BLOCK HASH TABLE COLUMN
// ================================================================================================

fn build_aux_col_p2<E: FieldElement<BaseField = Felt>>(
    trace_len: usize,
    _aux_trace_hints: &AuxTraceHints,
    _alphas: &[E],
) -> Vec<E> {
    // TODO: implement
    unsafe { uninit_vector(trace_len) }
}

// OP GROUP TABLE COLUMN
// ================================================================================================

/// Builds the execution trace of the decoder's `p3` column which describes the state of the op
/// group table via multiset checks.
fn build_aux_col_p3<E: FieldElement<BaseField = Felt>>(
    trace_len: usize,
    aux_trace_hints: &AuxTraceHints,
    alphas: &[E],
) -> Vec<E> {
    // allocate memory for the column and set the starting value to ONE
    let mut result = unsafe { uninit_vector(trace_len) };
    result[0] = E::ONE;

    // compute row values and their inverses for all rows which were added to the op group table
    let (row_values, inv_row_values) =
        build_op_group_table_row_values(aux_trace_hints.op_group_table_rows(), alphas);

    // keep track of indexes into the list of op group table rows separately for inserted and
    // removed rows
    let mut inserted_group_idx = 0_usize;
    let mut removed_group_idx = 0_usize;

    // keep track of the last updated value in the running product column
    let mut result_idx = 0_usize;

    for (&clk, update) in aux_trace_hints.op_group_table_hints() {
        // if we skipped some cycles since the last update was processed, values in the last
        // updated row should by copied over until the current cycle.
        if result_idx < clk {
            let last_value = result[result_idx];
            result[(result_idx + 1)..=clk].fill(last_value);
        }

        // apply the relevant updates to the column
        result_idx = clk + 1;
        match update {
            OpGroupTableUpdate::InsertRows(num_op_groups) => {
                // if the rows were added, multiply the current value in the column by the values
                // of all added rows
                let mut value = row_values[inserted_group_idx];
                for i in 1..(*num_op_groups as usize) {
                    value *= row_values[inserted_group_idx + i];
                }
                result[result_idx] = result[clk] * value;

                // advance the inserted group pointer by the number of inserted rows
                inserted_group_idx += *num_op_groups as usize;
            }
            OpGroupTableUpdate::RemoveRow => {
                // if a row was removed, divide the current value in the column by the value
                // of the row
                let value = inv_row_values[removed_group_idx];
                result[result_idx] = result[clk] * value;

                // advance the removed group pointer by one
                removed_group_idx += 1;
            }
        }
    }

    // at this point, op group table must be empty - so, the last value must be ONE;
    // we also fill in all the remaining values in the column with ONE's.
    let last_value = result[result_idx];
    assert_eq!(last_value, E::ONE);
    if result_idx < result.len() - 1 {
        result[(result_idx + 1)..].fill(E::ONE);
    }

    result
}

/// Computes values for all rows of the op group table. This also computes the inverse values
/// for each row. We need both because all added rows are also removed from the op group table.
///
/// To compute the inverses of row values we use a modified version of batch inversion algorithm.
/// The main modification is that we don't need to check for ZERO values, because, assuming,
/// alphas are random and are in a large enough field, coming across a ZERO value should be
/// computationally infeasible.
fn build_op_group_table_row_values<E: FieldElement<BaseField = Felt>>(
    rows: &[OpGroupTableRow],
    alphas: &[E],
) -> (Vec<E>, Vec<E>) {
    let mut row_values = unsafe { uninit_vector(rows.len()) };
    let mut inv_row_values = unsafe { uninit_vector(rows.len()) };

    // compute row values and compute their product
    let mut acc = E::ONE;
    for ((row, value), inv_value) in rows
        .iter()
        .zip(row_values.iter_mut())
        .zip(inv_row_values.iter_mut())
    {
        *inv_value = acc;
        *value = row.to_value(alphas);
        debug_assert_ne!(*value, E::ZERO, "row value cannot be ZERO");

        acc *= *value;
    }

    // invert the accumulated product
    acc = acc.inv();

    // multiply the accumulated value by original values to compute inverses
    for i in (0..row_values.len()).rev() {
        inv_row_values[i] *= acc;
        acc *= row_values[i];
    }

    (row_values, inv_row_values)
}
