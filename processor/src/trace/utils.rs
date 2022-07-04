use super::{Felt, FieldElement, Matrix, Vec};
use core::slice;
use vm_core::utils::uninit_vector;

#[cfg(test)]
use vm_core::{program::blocks::CodeBlock, Operation, ProgramInputs};

#[cfg(test)]
use super::{ExecutionTrace, Process};

// TRACE FRAGMENT
// ================================================================================================

/// TODO: add docs
pub struct TraceFragment<'a> {
    data: Vec<&'a mut [Felt]>,
}

impl<'a> TraceFragment<'a> {
    /// Creates a new TraceFragment with its data allocated to the specified capacity.
    pub fn new(capacity: usize) -> Self {
        TraceFragment {
            data: Vec::with_capacity(capacity),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the number of columns in this execution trace fragment.
    pub fn width(&self) -> usize {
        self.data.len()
    }

    /// Returns the number of rows in this execution trace fragment.
    pub fn len(&self) -> usize {
        self.data[0].len()
    }

    // DATA MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Updates a single cell in this fragment with provided value.
    #[inline(always)]
    pub fn set(&mut self, row_idx: usize, col_idx: usize, value: Felt) {
        self.data[col_idx][row_idx] = value;
    }

    /// Returns a mutable iterator to the columns of this fragment.
    pub fn columns(&mut self) -> slice::IterMut<'_, &'a mut [Felt]> {
        self.data.iter_mut()
    }

    /// Adds a new column to this fragment by pushing a mutable slice with the first `len`
    /// elements of the provided column. Returns the rest of the provided column as a separate
    /// mutable slice.
    pub fn push_column_slice(&mut self, column: &'a mut [Felt], len: usize) -> &'a mut [Felt] {
        let (column_fragment, rest) = column.split_at_mut(len);
        self.data.push(column_fragment);
        rest
    }

    // TEST METHODS
    // --------------------------------------------------------------------------------------------

    #[cfg(test)]
    pub fn trace_to_fragment(trace: &'a mut [Vec<Felt>]) -> Self {
        let mut data = Vec::new();
        for column in trace.iter_mut() {
            data.push(column.as_mut_slice());
        }
        Self { data }
    }
}

// LOOKUP TABLES
// ================================================================================================

/// Defines a single row in a lookup table defined via multiset checks.
pub trait LookupTableRow {
    /// Returns a single element representing the row in the field defined by E. The value is
    /// computed using the provided random values.
    fn to_value<E: FieldElement<BaseField = Felt>>(&self, rand_values: &[E]) -> E;
}

/// Computes values as well as inverse value for all specified lookup table rows.
///
/// To compute the inverses of row values we use a modified version of batch inversion algorithm.
/// The main modification is that we don't need to check for ZERO values, because, assuming
/// random values are drawn from a large enough field, coming across a ZERO value should be
/// computationally infeasible.
pub fn build_lookup_table_row_values<E: FieldElement<BaseField = Felt>, R: LookupTableRow>(
    rows: &[R],
    rand_values: &[E],
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
        *value = row.to_value(rand_values);
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

/// TODO: add comments
pub trait AuxColumnBuilder<H: Copy, R: LookupTableRow> {
    // REQUIRED METHODS
    // --------------------------------------------------------------------------------------------

    fn get_table_rows(&self) -> &[R];
    fn get_table_hints(&self) -> &[(usize, H)];

    fn get_multiplicand<E: FieldElement<BaseField = Felt>>(
        &self,
        hint: H,
        row_values: &[E],
        inv_row_values: &[E],
    ) -> E;

    // PROVIDED METHODS
    // --------------------------------------------------------------------------------------------

    fn build_aux_column<E>(&self, main_trace: &Matrix<Felt>, alphas: &[E]) -> Vec<E>
    where
        E: FieldElement<BaseField = Felt>,
    {
        // compute row values and their inverses for all rows that were added to the sibling table
        let table_rows = self.get_table_rows();
        let (row_values, inv_row_values) = build_lookup_table_row_values(table_rows, alphas);

        // allocate memory for the running product column and set the initial value to ONE
        let mut result = unsafe { uninit_vector(main_trace.num_rows()) };
        result[0] = E::ONE;

        // keep track of the last updated row in the running product column
        let mut result_idx = 0;

        // iterate through the list of updates and apply them one by one
        for (clk, hint) in self.get_table_hints() {
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
            let multiplicand = self.get_multiplicand(*hint, &row_values, &inv_row_values);
            result[result_idx] = result[clk] * multiplicand;
        }

        // at this point, sibling table must be empty - so, the last value must be ONE;
        // we also fill in all the remaining values in the column with ONE's.
        let last_value = result[result_idx];
        assert_eq!(last_value, E::ONE);
        if result_idx < result.len() - 1 {
            result[(result_idx + 1)..].fill(E::ONE);
        }

        result
    }
}

// TEST HELPERS
// ================================================================================================

/// Builds a sample trace by executing the provided code block against the provided stack inputs.
#[cfg(test)]
pub fn build_trace_from_block(program: &CodeBlock, stack: &[u64]) -> ExecutionTrace {
    let inputs = ProgramInputs::new(stack, &[], vec![]).unwrap();
    let mut process = Process::new(inputs);
    process.execute_code_block(program).unwrap();
    ExecutionTrace::new(process)
}

/// Builds a sample trace by executing a span block containing the specified operations. This
/// results in 1 additional hash cycle at the beginning of the hasher coprocessor.
#[cfg(test)]
pub fn build_trace_from_ops(operations: Vec<Operation>, stack: &[u64]) -> ExecutionTrace {
    let program = CodeBlock::new_span(operations);
    build_trace_from_block(&program, stack)
}
