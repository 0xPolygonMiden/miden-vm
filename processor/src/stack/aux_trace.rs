use miden_air::trace::{
    decoder::{IS_LOOP_FLAG_COL_IDX, OP_BITS_EXTRA_COLS_OFFSET},
    stack::{B0_COL_IDX, B1_COL_IDX, H0_COL_IDX},
    CLK_COL_IDX, DECODER_TRACE_OFFSET, STACK_TRACE_OFFSET,
};
use vm_core::{utils::uninit_vector, ONE, ZERO};
use winter_prover::math::batch_inversion;

use super::{
    super::trace::AuxColumnBuilder, ColMatrix, Felt, FieldElement, OverflowTableRow,
    OverflowTableUpdate, Vec,
};

// AUXILIARY TRACE BUILDER
// ================================================================================================

/// Describes how to construct execution traces of stack-related auxiliary trace segment columns
/// (used in multiset checks).
pub struct AuxTraceBuilder {
    /// A list of updates made to the overflow table during program execution. For each update we
    /// also track the cycle at which the update happened.
    pub(super) overflow_hints: Vec<(u64, OverflowTableUpdate)>,
    /// A list of all rows that were added to and then removed from the overflow table.
    pub(super) overflow_table_rows: Vec<OverflowTableRow>,
    /// The number of rows in the overflow table when execution begins.
    pub(super) num_init_rows: usize,
    /// A list of indices into the `all_rows` vector which describes the rows remaining in the
    /// overflow table at the end of execution.
    pub(super) final_rows: Vec<usize>,
}

impl AuxTraceBuilder {
    /// Builds and returns stack auxiliary trace columns. Currently this consists of a single
    /// column p1 describing states of the stack overflow table.
    pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &ColMatrix<Felt>,
        rand_elements: &[E],
    ) -> Vec<Vec<E>> {
        let p1 = self.build_aux_column(main_trace, rand_elements);
        vec![p1]
    }
}

// OVERFLOW TABLE
// ================================================================================================

impl AuxColumnBuilder<OverflowTableUpdate, OverflowTableRow, u64> for AuxTraceBuilder {
    /// Returns a list of rows which were added to and then removed from the stack overflow table.
    ///
    /// The order of the rows in the list is the same as the order in which the rows were added to
    /// the table.
    fn get_table_rows(&self) -> &[OverflowTableRow] {
        &self.overflow_table_rows
    }

    /// Returns hints which describe how the stack overflow table was updated during program
    /// execution. Each update hint is accompanied by a clock cycle at which the update happened.
    ///
    /// Internally, each update hint also contains an index of the row into the full list of rows
    /// which was either added or removed.
    fn get_table_hints(&self) -> &[(u64, OverflowTableUpdate)] {
        &self.overflow_hints[self.num_init_rows..]
    }

    /// Returns the value by which the running product column should be multiplied for the provided
    /// hint value.
    fn get_multiplicand<E: FieldElement<BaseField = Felt>>(
        &self,
        hint: OverflowTableUpdate,
        row_values: &[E],
        inv_row_values: &[E],
    ) -> E {
        match hint {
            OverflowTableUpdate::RowInserted(inserted_row_idx) => {
                row_values[inserted_row_idx as usize]
            }
            OverflowTableUpdate::RowRemoved(removed_row_idx) => {
                inv_row_values[removed_row_idx as usize]
            }
        }
    }

    /// Returns the initial value in the auxiliary column.
    fn init_column_value<E: FieldElement<BaseField = Felt>>(&self, row_values: &[E]) -> E {
        let mut init_column_value = E::ONE;
        // iterate through the elements in the initial table
        for (_, hint) in &self.overflow_hints[..self.num_init_rows] {
            // no rows should have been removed from the table before execution begins.
            if let OverflowTableUpdate::RowInserted(row) = hint {
                init_column_value *= row_values[*row as usize];
            } else {
                debug_assert!(
                    false,
                    "overflow table row incorrectly removed before execution started"
                )
            }
        }

        init_column_value
    }

    /// Builds the execution trace of the decoder's `p1` column which describes the state of the block
    /// stack table via multiset checks.
    fn build_aux_column<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &ColMatrix<Felt>,
        alphas: &[E],
    ) -> Vec<E> {
        let mut result_1: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
        let mut result_2: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
        let mut result: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };

        result_1[0] = E::ONE;
        result_2[0] = E::ONE;
        result[0] = E::ONE;

        let main_tr = MainTrace::new(main_trace);
        for i in 0..main_trace.num_rows() - 1 {
            result_1[i] = stack_overflow_table_inclusions(&main_tr, alphas, i);
            result_2[i] = stack_overflow_table_removals(&main_tr, alphas, i);
        }

        let result_2 = batch_inversion(&result_2);

        for i in 0..main_trace.num_rows() - 1 {
            result[i + 1] = result[i] * result_1[i] * result_2[i];
        }

        result
    }

    /// Returns the final value in the auxiliary column.
    fn final_column_value<E: FieldElement<BaseField = Felt>>(&self, row_values: &[E]) -> E {
        let mut final_column_value = E::ONE;
        for &row in &self.final_rows {
            final_column_value *= row_values[row];
        }

        final_column_value
    }
}

/// Adds a row to the stack overflow table.
fn stack_overflow_table_inclusions<E>(main_trace: &MainTrace, alphas: &[E], i: usize) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let is_right_shift = main_trace.is_right_shift(i);

    if is_right_shift {
        let k0 = main_trace.get_clk(i);
        let s15 = main_trace.get_stack_element(15, i);
        let b1 = main_trace.get_parent_overflow_address(i);

        alphas[0] + alphas[1].mul_base(k0) + alphas[2].mul_base(s15) + alphas[3].mul_base(b1)
    } else {
        E::ONE
    }
}

/// Removes a row from the stack overflow table.
fn stack_overflow_table_removals<E>(main_trace: &MainTrace, alphas: &[E], i: usize) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let is_left_shift = main_trace.is_left_shift(i);
    let is_non_empty_overflow = main_trace.is_non_empty_overflow(i);

    if is_left_shift && is_non_empty_overflow {
        let b1 = main_trace.get_parent_overflow_address(i);
        let s15_prime = main_trace.get_stack_element(15, i + 1);
        let b1_prime = main_trace.get_parent_overflow_address(i + 1);

        alphas[0]
            + alphas[1].mul_base(b1)
            + alphas[2].mul_base(s15_prime)
            + alphas[3].mul_base(b1_prime)
    } else {
        E::ONE
    }
}

// HELPER FUNCTIONS
// ================================================================================================

struct MainTrace<'a> {
    columns: &'a ColMatrix<Felt>,
}

impl<'a> MainTrace<'a> {
    pub fn new(main_trace: &'a ColMatrix<Felt>) -> Self {
        Self {
            columns: main_trace,
        }
    }

    /// Returns the value of the context column at row i.
    pub fn get_clk(&self, i: usize) -> Felt {
        self.columns.get_column(CLK_COL_IDX)[i]
    }

    /// Returns the address of the top element in the stack overflow table at row i.
    pub fn get_parent_overflow_address(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + B1_COL_IDX)[i]
    }

    /// Returns the element at row i in a given stack trace column.
    pub fn get_stack_element(&self, column: usize, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + column)[i]
    }

    /// Returns a flag indicating whether the overflow stack is non-empty.
    pub fn is_non_empty_overflow(&self, i: usize) -> bool {
        let b0 = self.columns.get_column(STACK_TRACE_OFFSET + B0_COL_IDX)[i];
        let h0 = self.columns.get_column(STACK_TRACE_OFFSET + H0_COL_IDX)[i];
        (b0 - Felt::new(16)) * h0 == ONE
    }

    /// Returns a flag indicating whether the current operation induces a left shift of the operand
    /// stack.
    pub fn is_left_shift(&self, i: usize) -> bool {
        let b0 = self.columns.get(DECODER_TRACE_OFFSET + 1, i);
        let b1 = self.columns.get(DECODER_TRACE_OFFSET + 2, i);
        let b2 = self.columns.get(DECODER_TRACE_OFFSET + 3, i);
        let b3 = self.columns.get(DECODER_TRACE_OFFSET + 4, i);
        let b4 = self.columns.get(DECODER_TRACE_OFFSET + 5, i);
        let b5 = self.columns.get(DECODER_TRACE_OFFSET + 6, i);
        let b6 = self.columns.get(DECODER_TRACE_OFFSET + 7, i);
        let e0 = self.columns.get(OP_BITS_EXTRA_COLS_OFFSET, i);
        let h5 = self.columns.get(IS_LOOP_FLAG_COL_IDX, i);

        // group with left shift effect grouped by a common prefix
        ([b6, b5, b4] == [ZERO, ONE, ZERO])||
        // U32ADD3 or U32MADD
        ([b6, b5, b4, b3, b2] == [ONE, ZERO, ZERO, ONE, ONE]) ||
        // SPLIT or LOOP block
        ([e0, b3, b2, b1] == [ONE, ZERO, ONE, ZERO]) ||
        // REPEAT
        ([b6, b5, b4, b3, b2, b1, b0] == [ONE, ONE, ONE, ZERO, ONE, ZERO, ZERO]) ||
        // END of a loop
        ([b6, b5, b4, b3, b2, b1, b0] == [ONE, ONE, ONE, ZERO, ZERO, ZERO, ZERO] && h5 == ONE)
    }

    /// Returns a flag indicating whether the current operation induces a right shift of the operand
    /// stack.
    pub fn is_right_shift(&self, i: usize) -> bool {
        let b0 = self.columns.get(DECODER_TRACE_OFFSET + 1, i);
        let b1 = self.columns.get(DECODER_TRACE_OFFSET + 2, i);
        let b2 = self.columns.get(DECODER_TRACE_OFFSET + 3, i);
        let b3 = self.columns.get(DECODER_TRACE_OFFSET + 4, i);
        let b4 = self.columns.get(DECODER_TRACE_OFFSET + 5, i);
        let b5 = self.columns.get(DECODER_TRACE_OFFSET + 6, i);
        let b6 = self.columns.get(DECODER_TRACE_OFFSET + 7, i);

        // group with right shift effect grouped by a common prefix
        [b6, b5, b4] == [ZERO, ONE, ONE]||
        // u32SPLIT 100_1000
        ([b6, b5, b4, b3, b2, b1, b0] == [ONE, ZERO, ZERO, ONE, ZERO, ZERO, ZERO]) ||
        // PUSH i.e., 110_0100
        ([b6, b5, b4, b3, b2, b1, b0] == [ONE, ONE, ZERO, ZERO, ONE, ZERO, ZERO])
    }
}
