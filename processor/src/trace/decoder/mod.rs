use super::{Felt, FieldElement, Matrix, Vec};
use crate::decoder::{AuxTraceHints, OpGroupTableRow, OpGroupTableUpdate};
use vm_core::utils::uninit_vector;

// DECODER AUXILIARY TRACE COLUMNS
// ================================================================================================

/// TODO: add docs
pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
    _main_trace: &Matrix<Felt>,
    _aux_trace_hints: &AuxTraceHints,
    _rand_elements: &[E],
) -> Vec<Vec<E>> {
    //let _p3 = build_aux_col_p3(main_trace.num_rows(), aux_trace_hints, rand_elements);
    Vec::new()
}

// OP GROUP TABLE COLUMN
// ================================================================================================

#[allow(dead_code)]
fn build_aux_col_p3<E: FieldElement<BaseField = Felt>>(
    trace_len: usize,
    aux_trace_hints: &AuxTraceHints,
    alphas: &[E],
) -> Vec<E> {
    let mut result = unsafe { uninit_vector(trace_len) };
    result[0] = E::ONE;

    let (row_values, inv_row_values) =
        build_op_group_table_row_values(aux_trace_hints.op_group_table_rows(), alphas);

    let mut inserted_group_idx = 0_usize;
    let mut removed_group_idx = 0_usize;
    let mut result_idx = 0_usize;

    for (&clk, update) in aux_trace_hints.op_group_table_hints() {
        if result_idx < clk {
            let last_value = result[result_idx];
            result[(result_idx + 1)..=clk].fill(last_value);
        }

        result_idx = clk + 1;
        match update {
            OpGroupTableUpdate::InsertRows(num_op_groups) => {
                let mut value = row_values[inserted_group_idx];
                for i in 1..(*num_op_groups as usize) {
                    value *= row_values[inserted_group_idx + i];
                }
                result[result_idx] = result[clk] * value;

                inserted_group_idx += *num_op_groups as usize;
            }
            OpGroupTableUpdate::RemoveRow => {
                let value = inv_row_values[removed_group_idx];
                result[result_idx] = result[clk] * value;
                removed_group_idx += 1;
            }
        }
    }

    result
}

fn build_op_group_table_row_values<E: FieldElement<BaseField = Felt>>(
    rows: &[OpGroupTableRow],
    alphas: &[E],
) -> (Vec<E>, Vec<E>) {
    let mut row_values = unsafe { uninit_vector(rows.len()) };
    let mut inv_row_values = unsafe { uninit_vector(rows.len()) };

    let mut last = E::ONE;
    for ((row, value), inv_value) in rows
        .iter()
        .zip(row_values.iter_mut())
        .zip(inv_row_values.iter_mut())
    {
        *inv_value = last;
        *value = row.to_value(alphas);

        last *= *value;
    }

    last = last.inv();

    for i in (0..row_values.len()).rev() {
        inv_row_values[i] *= last;
        last *= row_values[i];
    }

    (row_values, inv_row_values)
}
