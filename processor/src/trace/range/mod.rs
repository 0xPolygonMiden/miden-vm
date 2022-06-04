use super::{Felt, FieldElement, NUM_RAND_ROWS};
use crate::range::{AuxColumnHint, AuxTraceHints};

pub use vm_core::range::{P0_COL_IDX, P1_COL_IDX, T_COL_IDX, V_COL_IDX};

#[cfg(test)]
mod tests;

// RANGE CHECKER AUXILIARY TRACE COLUMNS
// ================================================================================================

/// Returns the range checker's auxiliary execution trace with its two auxiliary running product
/// columns `p0` and `p1`.
pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
    trace_len: usize,
    aux_trace_hints: &AuxTraceHints,
    rand_elements: &[E],
    v_col: &[Felt],
) -> Vec<Vec<E>> {
    let p0 = build_aux_col_p0(trace_len, aux_trace_hints, rand_elements, v_col);
    let p1 = build_aux_col_p1(trace_len, aux_trace_hints, rand_elements, v_col);

    vec![p0, p1]
}

// HELPER FUNCTIONS
// ================================================================================================

/// Builds the execution trace of the range checker's `p0` auxiliary column used for multiset
/// checks. The running product is built up in the 8-bit section of the table and reduced in the
/// 16-bit section of the table so that the starting and ending value are both one.
fn build_aux_col_p0<E: FieldElement<BaseField = Felt>>(
    trace_len: usize,
    aux_trace_hints: &AuxTraceHints,
    rand_elements: &[E],
    v_col: &[Felt],
) -> Vec<E> {
    let mut aux_column = E::zeroed_vector(trace_len);
    let alpha = rand_elements[0];

    // Set the starting value to one.
    aux_column[0] = E::ONE;

    // Build the execution trace of the 8-bit running product.
    for (row_idx, hint) in aux_trace_hints
        .aux_column_hints
        .iter()
        .enumerate()
        .take(aux_trace_hints.start_16bit)
    {
        // This is the 8-bit section, where the running product must be built up.
        let v: E = v_col[row_idx].into();
        let z = get_z(hint, alpha, v);
        aux_column[row_idx + 1] = aux_column[row_idx] * z;
    }

    // Accumulate the value differences for each transition and their product in preparation for
    // using a modified batch inversion to build the execution trace of the 16-bit section where the
    // running product must be reduced by the value difference at each row with offset alpha:
    // (alpha + v' - v).
    let mut diff_values =
        Vec::with_capacity(v_col.len() - aux_trace_hints.start_16bit - NUM_RAND_ROWS);
    let mut acc = E::ONE;
    for (row_idx, &v) in v_col
        .iter()
        .enumerate()
        .take(v_col.len() - 1)
        .skip(aux_trace_hints.start_16bit)
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
    acc = acc.inv() * aux_column[aux_trace_hints.start_16bit];

    // Do a modified version of batch inversion. We don't actually want an array of inverted
    // diff_values [1/a, 1/b, 1/c, ...], we want an array of inverted products all of which are
    // multiplied by the same 8-bit result `res`, e.g. [res/a, res/ab, res/abc, ...].
    for idx in (0..diff_values.len()).rev() {
        aux_column[aux_trace_hints.start_16bit + idx + 1] = acc;
        if diff_values[idx] != E::ZERO {
            acc *= diff_values[idx];
        }
    }

    aux_column
}

/// Builds the execution trace of the range checker's `p1` auxiliary column used for multiset
/// checks.
fn build_aux_col_p1<E: FieldElement<BaseField = Felt>>(
    trace_len: usize,
    aux_trace_hints: &AuxTraceHints,
    rand_elements: &[E],
    v_col: &[Felt],
) -> Vec<E> {
    let mut aux_column = Vec::with_capacity(trace_len);
    let alpha = rand_elements[0];

    // For the 8-bit section of the table, the starting value and all "next" values should be set to
    // one, since the value does not change.
    aux_column.resize(aux_trace_hints.start_16bit + 1, E::ONE);

    // For the 16-bit section of the range checker table, include `z` in the running product at each
    // step.
    for (row_idx, hint) in aux_trace_hints
        .aux_column_hints
        .iter()
        .enumerate()
        .take(v_col.len() - 1)
        .skip(aux_trace_hints.start_16bit)
    {
        let v = v_col[row_idx].into();
        let z = get_z(hint, alpha, v);
        aux_column.push(aux_column[row_idx] * z);
    }

    aux_column
}

/// Get the variable `z` to be included in the running product.
/// `z` is defined as: z=(α+v)4⋅f3​+(α+v)2⋅f2​+(α+v)⋅f1​+f0​.
fn get_z<E: FieldElement<BaseField = Felt>>(hint: &AuxColumnHint, alpha: E, v: E) -> E {
    match hint {
        AuxColumnHint::F0 => E::ONE,
        AuxColumnHint::F1 => v + alpha,
        AuxColumnHint::F2 => (v + alpha).square(),
        AuxColumnHint::F3 => ((v + alpha).square()).square(),
    }
}
