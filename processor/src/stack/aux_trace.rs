use super::{
    super::trace::{LookupTableRow},
    ColMatrix, Felt, FieldElement, OverflowTableRow, Vec,
};
use miden_air::trace::{
    decoder::{IS_LOOP_FLAG_COL_IDX, OP_BITS_EXTRA_COLS_OFFSET},
    stack::{B0_COL_IDX, B1_COL_IDX, H0_COL_IDX},
    CLK_COL_IDX, DECODER_TRACE_OFFSET, STACK_TRACE_OFFSET,
};
use vm_core::{utils::uninit_vector, ONE, ZERO};
use winter_prover::math::batch_inversion;

// AUXILIARY TRACE BUILDER
// ================================================================================================

/// Describes how to construct execution traces of stack-related auxiliary trace segment columns
/// (used in multiset checks).
pub struct AuxTraceBuilder {
    /// A list of all rows that were added to and then removed from the overflow table.
    pub(super) overflow_table_rows: Vec<OverflowTableRow>,
    /// The number of rows in the overflow table when execution begins.
    pub(super) num_init_rows: usize,
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

impl  AuxTraceBuilder {
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
        result[0] = init_overflow_table(self, main_trace, alphas);

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
}

/// Initializes the overflow stack auxiliary column.
fn init_overflow_table<E>(
    overflow_table: &AuxTraceBuilder,
    main_trace: &ColMatrix<Felt>,
    alphas: &[E],
) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let mut initial_column_value = E::ONE;
    for row in overflow_table.overflow_table_rows.iter().take(overflow_table.num_init_rows) {
        let value = (*row).to_value(main_trace, alphas);
        initial_column_value *= value;
    }
    initial_column_value
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
        let e0 = self.columns.get(DECODER_TRACE_OFFSET + OP_BITS_EXTRA_COLS_OFFSET, i);
        let h5 = self.columns.get(DECODER_TRACE_OFFSET + IS_LOOP_FLAG_COL_IDX, i);

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
