use core::ops::Range;

use super::{
    trace::{build_lookup_table_row_values, AuxColumnBuilder, LookupTableRow},
    BTreeMap, ColMatrix, Felt, FieldElement, StarkField, Vec, Word,
};

mod bus;
pub(crate) use bus::{ChipletLookup, ChipletsBus, ChipletsBusRow};

mod virtual_table;

const NUM_HEADER_ALPHAS: usize = 4;

use miden_air::trace::{
    chiplets::{
        hasher::{
            DIGEST_RANGE, HASH_CYCLE_LEN, LINEAR_HASH_LABEL, MP_VERIFY_LABEL, MR_UPDATE_NEW_LABEL,
            MR_UPDATE_OLD_LABEL, RETURN_HASH_LABEL, RETURN_STATE_LABEL, STATE_WIDTH,
        },
        BITWISE_A_COL_IDX, BITWISE_B_COL_IDX, BITWISE_OUTPUT_COL_IDX, HASHER_NODE_INDEX_COL_IDX,
        HASHER_RATE_COL_RANGE, HASHER_STATE_COL_RANGE, MEMORY_ADDR_COL_IDX, MEMORY_CLK_COL_IDX,
        MEMORY_CTX_COL_IDX, MEMORY_V_COL_RANGE,
    },
    decoder::{HASHER_STATE_OFFSET, NUM_HASHER_COLUMNS, USER_OP_HELPERS_OFFSET},
    stack::STACK_TOP_OFFSET,
    CHIPLETS_OFFSET, CLK_COL_IDX, CTX_COL_IDX, DECODER_TRACE_OFFSET, STACK_TRACE_OFFSET,
    TRACE_WIDTH,
};
pub(crate) use virtual_table::{ChipletsVTableRow, ChipletsVTableUpdate};
use vm_core::utils::{range, uninit_vector};
use vm_core::{ONE, ZERO};

/// Contains all relevant information and describes how to construct the execution trace for
/// chiplets-related auxiliary columns (used in multiset checks).
pub struct AuxTraceBuilder {
    bus_builder: BusTraceBuilder,
    table_builder: ChipletsVTableTraceBuilder,
}

impl AuxTraceBuilder {
    pub fn new(bus_builder: BusTraceBuilder, table_builder: ChipletsVTableTraceBuilder) -> Self {
        Self {
            bus_builder,
            table_builder,
        }
    }

    // COLUMN TRACE CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Builds and returns the Chiplets's auxiliary trace columns. Currently this consists of
    /// a single bus column `b_chip` describing chiplet lookups requested by the stack and
    /// provided by chiplets in the Chiplets module.
    pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &ColMatrix<Felt>,
        rand_elements: &[E],
    ) -> Vec<Vec<E>> {
        let t_chip = self.table_builder.build_aux_column(main_trace, rand_elements);
        let b_chip = self.bus_builder.build_aux_column(main_trace, rand_elements);
        vec![t_chip, b_chip]
    }
}

// BUS TRACE BUILDER
// ================================================================================================

/// Describes how to construct the execution trace of the chiplets bus auxiliary trace column.
pub struct BusTraceBuilder {
    pub(super) lookup_hints: Vec<(u32, ChipletsBusRow)>,
    pub(super) requests: Vec<ChipletLookup>,
    pub(super) responses: Vec<ChipletLookup>,
}

impl BusTraceBuilder {
    pub(crate) fn new(
        lookup_hints: Vec<(u32, ChipletsBusRow)>,
        requests: Vec<ChipletLookup>,
        responses: Vec<ChipletLookup>,
    ) -> Self {
        Self {
            lookup_hints,
            requests,
            responses,
        }
    }
}

impl AuxColumnBuilder<ChipletsBusRow, ChipletLookup, u32> for BusTraceBuilder {
    /// This method is required, but because it is only called inside `build_row_values` which is
    /// overridden below, it is not used here and should not be called.
    fn get_table_rows(&self) -> &[ChipletLookup] {
        unimplemented!()
    }

    /// Returns hints which describe the [Chiplets] lookup requests and responses during program
    /// execution. Each update hint is accompanied by a clock cycle at which the update happened.
    ///
    /// Internally, each update hint also contains an index of the row into the full list of request
    /// rows or response rows, depending on whether it is a request, a response, or both (in which
    /// case it contains 2 indices).
    fn get_table_hints(&self) -> &[(u32, ChipletsBusRow)] {
        &self.lookup_hints
    }

    /// Returns the value by which the running product column should be multiplied for the provided
    /// hint value.
    fn get_multiplicand<E: FieldElement<BaseField = Felt>>(
        &self,
        hint: ChipletsBusRow,
        row_values: &[E],
        inv_row_values: &[E],
    ) -> E {
        let mut mult = if let Some(response_idx) = hint.response() {
            row_values[response_idx as usize]
        } else {
            E::ONE
        };

        for request_idx in hint.requests() {
            mult *= inv_row_values[*request_idx as usize];
        }

        mult
    }

    /// Build the row values and inverse values used to build the auxiliary column.
    ///
    /// The row values to be included come from the responses and the inverse values come from
    /// requests. Since responses are grouped by chiplet, the operation order for the requests and
    /// responses will be permutations of each other rather than sharing the same order. Therefore,
    /// the `row_values` and `inv_row_values` must be built separately.
    fn build_row_values<E>(&self, main_trace: &ColMatrix<Felt>, alphas: &[E]) -> (Vec<E>, Vec<E>)
    where
        E: FieldElement<BaseField = Felt>,
    {
        // get the row values from the resonse rows
        let row_values = self
            .responses
            .iter()
            .map(|response| response.to_value(main_trace, alphas))
            .collect();
        // get the inverse values from the request rows
        let (_, inv_row_values) = build_lookup_table_row_values(&self.requests, main_trace, alphas);

        (row_values, inv_row_values)
    }

    /// TODO
    fn build_aux_column<E>(&self, main_trace: &ColMatrix<Felt>, alphas: &[E]) -> Vec<E>
    where
        E: FieldElement<BaseField = Felt>,
    {
        let mut result: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
        result[0] = E::ONE;

        for i in 0..main_trace.num_rows() - 1 {
            let multiplicand1 = chiplets_requests(main_trace, alphas, i);
            let multiplicand2 = chiplets_responses(main_trace, alphas, i);

            result[i + 1] = result[i] * multiplicand2 / multiplicand1;
        }

        result
    }
}

// VIRTUAL TABLE TRACE BUILDER
// ================================================================================================

/// Describes how to construct the execution trace of the chiplets virtual table, used to manage
/// internal updates and data required by the chiplets.
///
/// This manages construction of a single column which first represents the state of the sibling
/// table (used in Merkle root update computation), and then is subsequently used to represent the
/// procedures contained in the kernel ROM. Thus, it is expected that the initial value is ONE, the
/// value after all sibling table updates are completed is again ONE, and the value at the end of
/// the trace is the product of the representations of the kernel ROM procedures.
#[derive(Debug, Clone, Default)]
pub struct ChipletsVTableTraceBuilder {
    pub(super) hints: Vec<(u32, ChipletsVTableUpdate)>,
    pub(super) rows: Vec<ChipletsVTableRow>,
}

impl ChipletsVTableTraceBuilder {
    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Specifies that an entry for the provided sibling was added to the chiplets virtual table at
    /// the specified step.
    ///
    /// It is assumed that the table is empty or contains only sibling entries at this point and has
    /// not been used for any other chiplet updates.
    pub fn sibling_added(&mut self, step: u32, index: Felt, sibling: Word) {
        let row_index = self.rows.len();
        let update = ChipletsVTableUpdate::SiblingAdded(row_index as u32);
        self.hints.push((step, update));
        self.rows.push(ChipletsVTableRow::new_sibling(index, sibling));
    }

    /// Specifies that an entry for a sibling was removed from the chiplets virtual table. The entry
    /// is defined by the provided offset. For example, if row_offset = 2, the second from the last
    /// entry was removed from the table.
    ///
    /// It is assumed that the table contains only sibling entries at this point and has not been
    /// used for any other chiplet updates.
    pub fn sibling_removed(&mut self, step: u32, row_offset: usize) {
        let row_index = self.rows.len() - row_offset - 1;
        let update = ChipletsVTableUpdate::SiblingRemoved(row_index as u32);
        self.hints.push((step, update));
    }

    /// Specifies a kernel procedure that must be added to the virtual table.
    ///
    /// It is assumed that kernel procedures will only be added after all sibling updates have been
    /// completed.
    pub fn add_kernel_proc(&mut self, step: u32, addr: Felt, proc_hash: Word) {
        let proc_index = self.rows.len();
        let update = ChipletsVTableUpdate::KernelProcAdded(proc_index as u32);
        self.hints.push((step, update));
        self.rows.push(ChipletsVTableRow::new_kernel_proc(addr, proc_hash));
    }

    // TEST HELPERS
    // --------------------------------------------------------------------------------------------
    #[cfg(test)]
    pub fn hints(&self) -> &[(u32, ChipletsVTableUpdate)] {
        &self.hints
    }

    #[cfg(test)]
    pub fn rows(&self) -> &[ChipletsVTableRow] {
        &self.rows
    }
}

impl AuxColumnBuilder<ChipletsVTableUpdate, ChipletsVTableRow, u32> for ChipletsVTableTraceBuilder {
    /// Returns a list of rows which were added to and then removed from the chiplets virtual table.
    ///
    /// The order of the rows in the list is the same as the order in which the rows were added to
    /// the table.
    fn get_table_rows(&self) -> &[ChipletsVTableRow] {
        &self.rows
    }

    /// Returns hints which describe how the chiplets virtual table was updated during program
    /// execution. Each update hint is accompanied by a clock cycle at which the update happened.
    ///
    /// Internally, each update hint also contains an index of the row into the full list of rows
    /// which was either added or removed.
    fn get_table_hints(&self) -> &[(u32, ChipletsVTableUpdate)] {
        &self.hints
    }

    /// Returns the value by which the running product column should be multiplied for the provided
    /// hint value.
    fn get_multiplicand<E: FieldElement<BaseField = Felt>>(
        &self,
        hint: ChipletsVTableUpdate,
        row_values: &[E],
        inv_row_values: &[E],
    ) -> E {
        match hint {
            ChipletsVTableUpdate::SiblingAdded(inserted_row_idx) => {
                row_values[inserted_row_idx as usize]
            }
            ChipletsVTableUpdate::SiblingRemoved(removed_row_idx) => {
                inv_row_values[removed_row_idx as usize]
            }
            ChipletsVTableUpdate::KernelProcAdded(idx) => row_values[idx as usize],
        }
    }

    /// Returns the final value in the auxiliary column. Default implementation of this method
    /// returns ONE.
    fn final_column_value<E: FieldElement<BaseField = Felt>>(&self, row_values: &[E]) -> E {
        let mut result = E::ONE;
        for (_, table_update) in self.hints.iter() {
            if let ChipletsVTableUpdate::KernelProcAdded(idx) = table_update {
                result *= row_values[*idx as usize];
            }
        }

        result
    }
}

fn chiplets_requests<E>(main_trace: &ColMatrix<Felt>, alphas: &[E], i: usize) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    // get the address column and the op_bits columns
    let col_addr = main_trace.get_column(DECODER_TRACE_OFFSET);
    let col_b0 = main_trace.get_column(DECODER_TRACE_OFFSET + 1);
    let col_b1 = main_trace.get_column(DECODER_TRACE_OFFSET + 2);
    let col_b2 = main_trace.get_column(DECODER_TRACE_OFFSET + 3);
    let col_b3 = main_trace.get_column(DECODER_TRACE_OFFSET + 4);
    let col_b4 = main_trace.get_column(DECODER_TRACE_OFFSET + 5);
    let col_b5 = main_trace.get_column(DECODER_TRACE_OFFSET + 6);
    let col_b6 = main_trace.get_column(DECODER_TRACE_OFFSET + 7);

    let mut multiplicand = E::ONE;

    pub const DECODER_HASHER_RANGE: Range<usize> =
        range(DECODER_TRACE_OFFSET + HASHER_STATE_OFFSET, NUM_HASHER_COLUMNS);
    let [b0, b1, b2, b3, b4, b5, b6] =
        [col_b0[i], col_b1[i], col_b2[i], col_b3[i], col_b4[i], col_b5[i], col_b6[i]];

    if [b0, b1, b2, b3, b4, b5, b6] == [ONE, ONE, ONE, ZERO, ONE, ZERO, ONE]            // JOIN
                || [b0, b1, b2, b3, b4, b5, b6] == [ZERO, ZERO, ONE, ZERO, ONE, ZERO, ONE]      // SPLIT
                || [b0, b1, b2, b3, b4, b5, b6] == [ONE, ZERO, ONE, ZERO, ONE, ZERO, ONE]       // LOOP
                || [b0, b1, b2, b3, b4, b5, b6] == [ZERO, ZERO, ZERO, ONE, ONE, ZERO, ONE]      // DYN
                || [b0, b1, b2, b3, b4, b5, b6] == [ZERO, ZERO, ONE, ONE, ZERO, ONE, ONE]       // CALL
                || [b0, b1, b2, b3, b4, b5, b6] == [ZERO, ZERO, ZERO, ONE, ZERO, ONE, ONE]  // SYSCALL
                || [b0, b1, b2, b3, b4, b5, b6] == [ZERO, ONE, ONE, ZERO, ONE, ZERO, ONE]
    // SPAN
    {
        let mut d = b0
            + b1.mul_small(2)
            + b2.mul_small(4)
            + b3.mul_small(8)
            + b4.mul_small(16)
            + b5.mul_small(32)
            + b6.mul_small(64);

        if [b0, b1, b2, b3, b4, b5, b6] == [ZERO, ONE, ONE, ZERO, ONE, ZERO, ONE] {
            d = ZERO;
        }

        let op_label = LINEAR_HASH_LABEL;

        let first_cycle_row = addr_to_row_index(col_addr[i + 1]) % 8 == 0;
        let transition_label = if first_cycle_row { op_label + 16 } else { op_label + 32 };

        let header = alphas[0]
            + alphas[1].mul_base(Felt::from(transition_label))
            + alphas[2].mul_base(col_addr[i + 1])
            + alphas[3].mul_base(ZERO);

        // TODO: access only relevant portion of trace
        let mut row = vec![ZERO; TRACE_WIDTH];
        main_trace.read_row_into(i, &mut row);

        let state = &row[DECODER_HASHER_RANGE];
        let factor = header + build_value(&alphas[8..16], state) + alphas[5].mul_base(d);
        if [b0, b1, b2, b3, b4, b5, b6] == [ZERO, ZERO, ZERO, ONE, ZERO, ONE, ONE] {
            let [s0, s1, s2, s3] = [ONE, ONE, ONE, ZERO];
            let op_label = get_op_label(s0, s1, s2, s3);

            let factor = alphas[0]
                + alphas[1].mul_base(op_label)
                + alphas[2].mul_base(row[DECODER_HASHER_RANGE.start])
                + alphas[3].mul_base(row[DECODER_HASHER_RANGE.start + 1])
                + alphas[4].mul_base(row[DECODER_HASHER_RANGE.start + 2])
                + alphas[5].mul_base(row[DECODER_HASHER_RANGE.start + 3]);
            multiplicand *= factor;
        }
        multiplicand *= factor;
    }

    // RESPAN block
    if [b0, b1, b2, b3, b4, b5, b6] == [ZERO, ZERO, ZERO, ONE, ONE, ONE, ONE] {
        let op_label = LINEAR_HASH_LABEL;

        let first_cycle_row = addr_to_row_index(col_addr[i + 1] - ONE) % 8 == 0;
        let transition_label = if first_cycle_row { op_label + 16 } else { op_label + 32 };

        let header = alphas[0]
            + alphas[1].mul_base(Felt::from(transition_label))
            + alphas[2].mul_base(col_addr[i + 1] - ONE)
            + alphas[3].mul_base(ZERO);

        // TODO: access only relevant portion of trace
        let mut row = vec![ZERO; TRACE_WIDTH];
        main_trace.read_row_into(i - 2, &mut row);

        let mut row_next = vec![ZERO; TRACE_WIDTH];
        main_trace.read_row_into(i - 1, &mut row_next);

        let state = &row[HASHER_RATE_COL_RANGE];
        let state_nxt = &row_next[HASHER_RATE_COL_RANGE];

        let factor =
            header + build_value(&alphas[8..16], state_nxt) - build_value(&alphas[8..16], state);

        multiplicand *= factor;
    }

    // END of block
    if [b0, b1, b2, b3, b4, b5, b6] == [ZERO, ZERO, ZERO, ZERO, ONE, ONE, ONE] {
        let op_label = RETURN_HASH_LABEL;
        let first_cycle_row = addr_to_row_index(col_addr[i] + Felt::from(7_u64)) % 8 == 0;
        let transition_label = if first_cycle_row { op_label + 16 } else { op_label + 32 };

        let header = alphas[0]
            + alphas[1].mul_base(Felt::from(transition_label))
            + alphas[2].mul_base(col_addr[i] + Felt::from(7_u64))
            + alphas[3].mul_base(ZERO);

        // TODO: access only relevant portion of trace
        let mut row = vec![ZERO; TRACE_WIDTH];
        main_trace.read_row_into(i, &mut row);

        let state = &row[DECODER_HASHER_RANGE.start..DECODER_HASHER_RANGE.start + 4];

        let factor = header + build_value(&alphas[8..12], state);
        multiplicand *= factor;
    }

    // U32AND or U32XOR
    if [b1, b2, b3, b4, b5, b6] == [ONE, ONE, ZERO, ZERO, ONE, ZERO] {
        let s0_col = main_trace.get_column(STACK_TRACE_OFFSET);
        let s1_col = main_trace.get_column(STACK_TRACE_OFFSET + 1);

        let [s0, s1, s2, s3] = [ONE, ZERO, b0, ZERO];
        let op_label = get_op_label(s0, s1, s2, s3);

        let factor = alphas[0]
            + alphas[1].mul_base(op_label)
            + alphas[2].mul_base(s1_col[i])
            + alphas[3].mul_base(s0_col[i])
            + alphas[4].mul_base(s0_col[i + 1]);
        multiplicand *= factor;
    }

    // MLOADW or MSTOREW
    if [b0, b2, b3, b4, b5, b6] == [ZERO, ONE, ONE, ZERO, ONE, ZERO] {
        let ctx = main_trace.get_column(CTX_COL_IDX)[i];
        let clk = main_trace.get_column(CLK_COL_IDX)[i];

        let s0_col = main_trace.get_column(STACK_TRACE_OFFSET);
        let s1_col = main_trace.get_column(STACK_TRACE_OFFSET + 1);
        let s2_col = main_trace.get_column(STACK_TRACE_OFFSET + 2);
        let s3_col = main_trace.get_column(STACK_TRACE_OFFSET + 3);

        let s0_cur = s0_col[i];
        let s0_nxt = s0_col[i + 1];
        let s1_nxt = s1_col[i + 1];
        let s2_nxt = s2_col[i + 1];
        let s3_nxt = s3_col[i + 1];

        let op_label = get_op_label(ONE, ONE, ZERO, ONE - b1);

        let factor = alphas[0]
            + alphas[1].mul_base(op_label)
            + alphas[2].mul_base(ctx)
            + alphas[3].mul_base(s0_cur)
            + alphas[4].mul_base(clk)
            + alphas[5].mul_base(s3_nxt)
            + alphas[6].mul_base(s2_nxt)
            + alphas[7].mul_base(s1_nxt)
            + alphas[8].mul_base(s0_nxt);
        multiplicand *= factor;
    }

    // MLOAD
    if [b0, b1, b2, b3, b4, b5, b6] == [ONE, ONE, ONE, ZERO, ZERO, ZERO, ZERO] {
        let ctx = main_trace.get_column(CTX_COL_IDX)[i];
        let clk = main_trace.get_column(CLK_COL_IDX)[i];

        let s0_col = main_trace.get_column(STACK_TRACE_OFFSET);
        let helper_0 = main_trace.get_column(DECODER_TRACE_OFFSET + USER_OP_HELPERS_OFFSET)[i];
        let helper_1 = main_trace.get_column(DECODER_TRACE_OFFSET + USER_OP_HELPERS_OFFSET + 1)[i];
        let helper_2 = main_trace.get_column(DECODER_TRACE_OFFSET + USER_OP_HELPERS_OFFSET + 2)[i];

        let s0_cur = s0_col[i];
        let s0_nxt = s0_col[i + 1];

        let op_label = get_op_label(ONE, ONE, ZERO, ONE);

        let factor = alphas[0]
            + alphas[1].mul_base(op_label)
            + alphas[2].mul_base(ctx)
            + alphas[3].mul_base(s0_cur)
            + alphas[4].mul_base(clk)
            + alphas[5].mul_base(s0_nxt)
            + alphas[6].mul_base(helper_2)
            + alphas[7].mul_base(helper_1)
            + alphas[8].mul_base(helper_0);
        multiplicand *= factor;
    }

    //  MSTORE
    if [b0, b1, b2, b3, b4, b5, b6] == [ONE, ZERO, ONE, ONE, ZERO, ONE, ZERO] {
        let ctx = main_trace.get_column(CTX_COL_IDX)[i];
        let clk = main_trace.get_column(CLK_COL_IDX)[i];

        let s0_col = main_trace.get_column(STACK_TRACE_OFFSET);
        let helper_0 = main_trace.get_column(DECODER_TRACE_OFFSET + USER_OP_HELPERS_OFFSET)[i];
        let helper_1 = main_trace.get_column(DECODER_TRACE_OFFSET + USER_OP_HELPERS_OFFSET + 1)[i];
        let helper_2 = main_trace.get_column(DECODER_TRACE_OFFSET + USER_OP_HELPERS_OFFSET + 2)[i];

        let s0_cur = s0_col[i];
        let s0_nxt = s0_col[i + 1];

        let op_label = get_op_label(ONE, ONE, ZERO, ZERO);

        let factor = alphas[0]
            + alphas[1].mul_base(op_label)
            + alphas[2].mul_base(ctx)
            + alphas[3].mul_base(s0_cur)
            + alphas[4].mul_base(clk)
            + alphas[5].mul_base(s0_nxt)
            + alphas[6].mul_base(helper_2)
            + alphas[7].mul_base(helper_1)
            + alphas[8].mul_base(helper_0);

        multiplicand *= factor;
    }

    // MSTREAM
    if [b0, b1, b2, b3, b4, b5, b6] == [ONE, ONE, ZERO, ZERO, ONE, ZERO, ONE] {
        let ctx = main_trace.get_column(CTX_COL_IDX)[i];
        let clk = main_trace.get_column(CLK_COL_IDX)[i];

        let s0_nxt = main_trace.get_column(STACK_TRACE_OFFSET)[i + 1];
        let s1_nxt = main_trace.get_column(STACK_TRACE_OFFSET + 1)[i + 1];
        let s2_nxt = main_trace.get_column(STACK_TRACE_OFFSET + 2)[i + 1];
        let s3_nxt = main_trace.get_column(STACK_TRACE_OFFSET + 3)[i + 1];
        let s4_nxt = main_trace.get_column(STACK_TRACE_OFFSET + 4)[i + 1];
        let s5_nxt = main_trace.get_column(STACK_TRACE_OFFSET + 5)[i + 1];
        let s6_nxt = main_trace.get_column(STACK_TRACE_OFFSET + 6)[i + 1];
        let s7_nxt = main_trace.get_column(STACK_TRACE_OFFSET + 7)[i + 1];

        let s12_cur = main_trace.get_column(STACK_TRACE_OFFSET + 12)[i];

        let op_label = get_op_label(ONE, ONE, ZERO, ONE);

        let factor1 = alphas[0]
            + alphas[1].mul_base(op_label)
            + alphas[2].mul_base(ctx)
            + alphas[3].mul_base(s12_cur)
            + alphas[4].mul_base(clk)
            + alphas[5].mul_base(s7_nxt)
            + alphas[6].mul_base(s6_nxt)
            + alphas[7].mul_base(s5_nxt)
            + alphas[8].mul_base(s4_nxt);
        multiplicand *= factor1;

        let factor2 = alphas[0]
            + alphas[1].mul_base(op_label)
            + alphas[2].mul_base(ctx)
            + alphas[3].mul_base(s12_cur + ONE)
            + alphas[4].mul_base(clk)
            + alphas[5].mul_base(s3_nxt)
            + alphas[6].mul_base(s2_nxt)
            + alphas[7].mul_base(s1_nxt)
            + alphas[8].mul_base(s0_nxt);
        multiplicand *= factor2;
    }

    // HPERM
    if [b0, b1, b2, b3, b4, b5, b6] == [ZERO, ZERO, ZERO, ZERO, ONE, ZERO, ONE] {
        let helper_0 = main_trace.get_column(DECODER_TRACE_OFFSET + USER_OP_HELPERS_OFFSET)[i];

        let s0_s12_cur = [
            main_trace.get_column(STACK_TRACE_OFFSET)[i],
            main_trace.get_column(STACK_TRACE_OFFSET + 1)[i],
            main_trace.get_column(STACK_TRACE_OFFSET + 2)[i],
            main_trace.get_column(STACK_TRACE_OFFSET + 3)[i],
            main_trace.get_column(STACK_TRACE_OFFSET + 4)[i],
            main_trace.get_column(STACK_TRACE_OFFSET + 5)[i],
            main_trace.get_column(STACK_TRACE_OFFSET + 6)[i],
            main_trace.get_column(STACK_TRACE_OFFSET + 7)[i],
            main_trace.get_column(STACK_TRACE_OFFSET + 8)[i],
            main_trace.get_column(STACK_TRACE_OFFSET + 9)[i],
            main_trace.get_column(STACK_TRACE_OFFSET + 10)[i],
            main_trace.get_column(STACK_TRACE_OFFSET + 11)[i],
        ];

        let s0_s12_nxt = [
            main_trace.get_column(STACK_TRACE_OFFSET)[i + 1],
            main_trace.get_column(STACK_TRACE_OFFSET + 1)[i + 1],
            main_trace.get_column(STACK_TRACE_OFFSET + 2)[i + 1],
            main_trace.get_column(STACK_TRACE_OFFSET + 3)[i + 1],
            main_trace.get_column(STACK_TRACE_OFFSET + 4)[i + 1],
            main_trace.get_column(STACK_TRACE_OFFSET + 5)[i + 1],
            main_trace.get_column(STACK_TRACE_OFFSET + 6)[i + 1],
            main_trace.get_column(STACK_TRACE_OFFSET + 7)[i + 1],
            main_trace.get_column(STACK_TRACE_OFFSET + 8)[i + 1],
            main_trace.get_column(STACK_TRACE_OFFSET + 9)[i + 1],
            main_trace.get_column(STACK_TRACE_OFFSET + 10)[i + 1],
            main_trace.get_column(STACK_TRACE_OFFSET + 11)[i + 1],
        ];

        let op_label = LINEAR_HASH_LABEL;

        let op_label = if addr_to_hash_cycle(helper_0) == 0 {
            op_label + 16
        } else {
            op_label + 32
        };

        let sum_input = alphas[4..16]
            .iter()
            .rev()
            .enumerate()
            .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(s0_s12_cur[i]));
        let v_input = alphas[0]
            + alphas[1].mul_base(Felt::from(op_label))
            + alphas[2].mul_base(helper_0)
            + sum_input;

        let op_label = RETURN_STATE_LABEL;
        let op_label = if addr_to_hash_cycle(helper_0 + Felt::new(7)) == 0 {
            op_label + 16
        } else {
            op_label + 32
        };

        let sum_output = alphas[4..16]
            .iter()
            .rev()
            .enumerate()
            .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(s0_s12_nxt[i]));

        let v_output = alphas[0]
            + alphas[1].mul_base(Felt::from(op_label))
            + alphas[2].mul_base(helper_0 + Felt::new(7))
            + sum_output;

        multiplicand *= v_input * v_output;
    }

    // MPVERIFY
    if [b0, b1, b2, b3, b4, b5, b6] == [ONE, ZERO, ZERO, ZERO, ONE, ZERO, ONE] {
        let helper_0 = main_trace.get_column(DECODER_TRACE_OFFSET + USER_OP_HELPERS_OFFSET)[i];

        let s0_s3 = [
            main_trace.get_column(STACK_TRACE_OFFSET)[i],
            main_trace.get_column(STACK_TRACE_OFFSET + 1)[i],
            main_trace.get_column(STACK_TRACE_OFFSET + 2)[i],
            main_trace.get_column(STACK_TRACE_OFFSET + 3)[i],
        ];
        let s4 = main_trace.get_column(STACK_TRACE_OFFSET + 4)[i];
        let s5 = main_trace.get_column(STACK_TRACE_OFFSET + 5)[i];
        let s6_s9 = [
            main_trace.get_column(STACK_TRACE_OFFSET + 6)[i],
            main_trace.get_column(STACK_TRACE_OFFSET + 7)[i],
            main_trace.get_column(STACK_TRACE_OFFSET + 8)[i],
            main_trace.get_column(STACK_TRACE_OFFSET + 9)[i],
        ];
        let op_label = MP_VERIFY_LABEL;
        let op_label = if addr_to_hash_cycle(helper_0) == 0 {
            op_label + 16
        } else {
            op_label + 32
        };
        let sum_input = alphas[8..12]
            .iter()
            .rev()
            .enumerate()
            .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(s0_s3[i]));

        let v_input = alphas[0]
            + alphas[1].mul_base(Felt::from(op_label))
            + alphas[2].mul_base(helper_0)
            + alphas[3].mul_base(s5)
            + sum_input;

        let op_label = RETURN_HASH_LABEL;
        let op_label = if (helper_0).as_int() % 8 == 0 {
            op_label + 16
        } else {
            op_label + 32
        };
        let sum_output = alphas[8..12]
            .iter()
            .rev()
            .enumerate()
            .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(s6_s9[i]));

        let v_output = alphas[0]
            + alphas[1].mul_base(Felt::from(op_label))
            + alphas[2].mul_base(helper_0 + s4.mul_small(8) - ONE)
            + sum_output;

        multiplicand *= v_input * v_output;
    }

    // MRUPDATE
    if [b0, b1, b2, b3, b4, b5, b6] == [ZERO, ZERO, ZERO, ZERO, ZERO, ONE, ONE] {
        let helper_0 = main_trace.get_column(DECODER_TRACE_OFFSET + USER_OP_HELPERS_OFFSET)[i];

        let s0_s3 = [
            main_trace.get_column(STACK_TOP_OFFSET)[i],
            main_trace.get_column(STACK_TOP_OFFSET + 1)[i],
            main_trace.get_column(STACK_TOP_OFFSET + 2)[i],
            main_trace.get_column(STACK_TOP_OFFSET + 3)[i],
        ];
        let s0_s3_nxt = [
            main_trace.get_column(STACK_TOP_OFFSET)[i + 1],
            main_trace.get_column(STACK_TOP_OFFSET + 1)[i + 1],
            main_trace.get_column(STACK_TOP_OFFSET + 2)[i + 1],
            main_trace.get_column(STACK_TOP_OFFSET + 3)[i + 1],
        ];
        let s4 = main_trace.get_column(STACK_TOP_OFFSET + 4)[i];
        let s5 = main_trace.get_column(STACK_TOP_OFFSET + 5)[i];
        let s6_s9 = [
            main_trace.get_column(STACK_TOP_OFFSET + 6)[i],
            main_trace.get_column(STACK_TOP_OFFSET + 7)[i],
            main_trace.get_column(STACK_TOP_OFFSET + 8)[i],
            main_trace.get_column(STACK_TOP_OFFSET + 9)[i],
        ];
        let s10_s13 = [
            main_trace.get_column(STACK_TOP_OFFSET + 10)[i],
            main_trace.get_column(STACK_TOP_OFFSET + 11)[i],
            main_trace.get_column(STACK_TOP_OFFSET + 12)[i],
            main_trace.get_column(STACK_TOP_OFFSET + 13)[i],
        ];
        let op_label = MR_UPDATE_OLD_LABEL;
        let sum_input = alphas[8..12]
            .iter()
            .rev()
            .enumerate()
            .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(s0_s3[i]));
        let v_input_old = alphas[0]
            + alphas[1].mul_base(Felt::from(op_label))
            + alphas[2].mul_base(helper_0)
            + alphas[3].mul_base(s5)
            + sum_input;

        let op_label = RETURN_HASH_LABEL;

        let sum_output = alphas[8..12]
            .iter()
            .rev()
            .enumerate()
            .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(s6_s9[i]));
        let v_output_old = alphas[0]
            + alphas[1].mul_base(Felt::from(op_label))
            + alphas[2].mul_base(helper_0 + s4.mul_small(8) - ONE)
            + sum_output;

        let op_label = MR_UPDATE_NEW_LABEL;
        let sum_input = alphas[8..12]
            .iter()
            .rev()
            .enumerate()
            .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(s10_s13[i]));
        let v_input_new = alphas[0]
            + alphas[1].mul_base(Felt::from(op_label))
            + alphas[2].mul_base(helper_0 + s4.mul_small(8))
            + alphas[3].mul_base(s5)
            + sum_input;

        let op_label = RETURN_HASH_LABEL;
        let sum_output = alphas[8..12]
            .iter()
            .rev()
            .enumerate()
            .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(s0_s3_nxt[i]));
        let v_output_new = alphas[0]
            + alphas[1].mul_base(Felt::from(op_label))
            + alphas[2].mul_base(helper_0 + s4.mul_small(16) - ONE)
            + sum_output;

        multiplicand *= v_input_new * v_input_old * v_output_new * v_output_old;
    }
    multiplicand
}

fn chiplets_responses<E>(main_trace: &ColMatrix<Felt>, alphas: &[E], i: usize) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let col0 = main_trace.get_column(CHIPLETS_OFFSET);
    let col1 = main_trace.get_column(CHIPLETS_OFFSET + 1);
    let col2 = main_trace.get_column(CHIPLETS_OFFSET + 2);
    let col3 = main_trace.get_column(CHIPLETS_OFFSET + 3);
    let col4 = main_trace.get_column(CHIPLETS_OFFSET + 4);

    if col0[i] == ZERO {
        return build_hasher_chiplet(main_trace, &i, alphas, col1, col2, col3);
    }

    if col0[i] == ONE && col1[i] == ZERO {
        return build_bitwise_chiplet(main_trace, &i, alphas, col2);
    }

    if col0[i] == ONE && col1[i] == ONE && col2[i] == ZERO {
        return build_memory_chiplet(main_trace, &i, alphas, col3);
    }

    if col0[i] == ONE && col1[i] == ONE && col2[i] == ONE && col3[i] == ZERO && col4[i] == ONE {
        build_kernel_chiplet(main_trace, &i, alphas)
    } else {
        E::ONE
    }
}

fn build_hasher_chiplet<E>(
    main_trace: &ColMatrix<Felt>,
    i: &usize,
    alphas: &[E],
    col1: &[Felt],
    col2: &[Felt],
    col3: &[Felt],
) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let mut multiplicand = E::ONE;

    // only f_bp, f_mp, f_mv or f_mu
    if *i % 8 == 0 {
        let alphas_state = &alphas[NUM_HEADER_ALPHAS..(NUM_HEADER_ALPHAS + STATE_WIDTH)];

        let mut row = vec![ZERO; TRACE_WIDTH];

        main_trace.read_row_into(*i, &mut row);
        let [s0, s1, s2] = [col1[*i], col2[*i], col3[*i]];

        // f_bp == 1
        // v_all = v_h + v_a + v_b + v_c
        if s0 == ONE && s1 == ZERO && s2 == ZERO {
            let op_label = LINEAR_HASH_LABEL;
            let transition_label = Felt::from(op_label) + Felt::from(16_u8);

            let header = alphas[0]
                + alphas[1].mul_base(transition_label)
                + alphas[2].mul_base(Felt::from((*i + 1) as u64))
                + alphas[3].mul_base(row[HASHER_NODE_INDEX_COL_IDX]);

            let state = &row[HASHER_STATE_COL_RANGE];
            multiplicand = header + build_value(alphas_state, state);
        }

        // f_mp or f_mv or f_mu == 1
        // v_leaf = v_h + (1 - b) * v_b + b * v_d
        if s0 == ONE && !(s1 == ZERO && s2 == ZERO) {
            let op_label = get_op_label(ZERO, s0, s1, s2);
            let transition_label = op_label + Felt::from(16_u8);

            let header = alphas[0]
                + alphas[1].mul_base(transition_label)
                + alphas[2].mul_base(Felt::from((*i + 1) as u64))
                + alphas[3].mul_base(row[HASHER_NODE_INDEX_COL_IDX]);

            let bit = (row[HASHER_NODE_INDEX_COL_IDX].as_int() >> 1) & 1;
            let rate = &row[HASHER_RATE_COL_RANGE];
            let left_word = build_value(&alphas[8..12], &rate[..4]);
            let right_word = build_value(&alphas[8..12], &rate[4..]);

            multiplicand = header + E::from(1 - bit).mul(left_word) + E::from(bit).mul(right_word);
        }
    }

    // only f_hout, f_sout, f_abp, f_mpa, f_mva or f_mua
    if *i % 8 == 7 {
        let alphas_state = &alphas[NUM_HEADER_ALPHAS..(NUM_HEADER_ALPHAS + STATE_WIDTH)];

        let mut row = vec![ZERO; TRACE_WIDTH];
        let mut row_next = vec![ZERO; TRACE_WIDTH];

        main_trace.read_row_into(*i, &mut row);
        let [s0, s1, s2] = [col1[*i], col2[*i], col3[*i]];

        // f_hout == 1
        // v_res = v_h + v_b;
        if s0 == ZERO && s1 == ZERO && s2 == ZERO {
            let op_label = RETURN_HASH_LABEL;
            let transition_label = Felt::from(op_label) + Felt::from(32_u8);

            let header = alphas[0]
                + alphas[1].mul_base(transition_label)
                + alphas[2].mul_base(Felt::from((*i + 1) as u64))
                + alphas[3].mul_base(row[HASHER_NODE_INDEX_COL_IDX]);

            let state = &row[HASHER_STATE_COL_RANGE];

            multiplicand = header + build_value(&alphas_state[4..8], &state[DIGEST_RANGE]);
        }

        // f_sout == 1
        // v_all = v_h + v_a + v_b + v_c
        if s0 == ZERO && s1 == ZERO && s2 == ONE {
            let op_label = RETURN_STATE_LABEL;
            let transition_label = Felt::from(op_label) + Felt::from(32_u8);

            let header = alphas[0]
                + alphas[1].mul_base(transition_label)
                + alphas[2].mul_base(Felt::from((*i + 1) as u64))
                + alphas[3].mul_base(row[HASHER_NODE_INDEX_COL_IDX]);

            let state = &row[HASHER_STATE_COL_RANGE];
            multiplicand = header + build_value(alphas_state, state);
        }

        // f_abp == 1
        // v_abp = v_h + v_b' + v_c' - v_b - v_c
        if s0 == ONE && s1 == ZERO && s2 == ZERO {
            let op_label = get_op_label(ZERO, s0, s1, s2);
            let transition_label = op_label + Felt::from(32_u8);

            let header = alphas[0]
                + alphas[1].mul_base(transition_label)
                + alphas[2].mul_base(Felt::from((*i + 1) as u64))
                + alphas[3].mul_base(row[HASHER_NODE_INDEX_COL_IDX]);

            main_trace.read_row_into(*i + 1, &mut row_next);
            let curr_hasher_rate = &row[HASHER_RATE_COL_RANGE];
            let next_hasher_rate = &row_next[HASHER_RATE_COL_RANGE];

            // build the value from the difference of the hasher state's just before and  right
            // after the absorption of new elements.
            let next_state_value = build_value(&alphas_state[4..12], next_hasher_rate);
            let state_value = build_value(&alphas_state[4..12], curr_hasher_rate);

            multiplicand = header + next_state_value - state_value;
        }
    }
    multiplicand
}

fn build_bitwise_chiplet<E>(
    main_trace: &ColMatrix<Felt>,
    i: &usize,
    alphas: &[E],
    col: &[Felt],
) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let mut multiplicand = E::ONE;
    if *i % 8 == 7 {
        let [s0, s1, s2, s3] = [ONE, ZERO, col[*i], ZERO];
        let op_label = get_op_label(s0, s1, s2, s3);

        let mut row = vec![ZERO; TRACE_WIDTH];
        main_trace.read_row_into(*i, &mut row);

        multiplicand = alphas[0]
            + alphas[1].mul_base(op_label)
            + alphas[2].mul_base(row[BITWISE_A_COL_IDX])
            + alphas[3].mul_base(row[BITWISE_B_COL_IDX])
            + alphas[4].mul_base(row[BITWISE_OUTPUT_COL_IDX]);
    }
    multiplicand
}

fn build_memory_chiplet<E>(main_trace: &ColMatrix<Felt>, i: &usize, alphas: &[E], col: &[Felt]) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let [s0, s1, s2, s3] = [ONE, ONE, ZERO, col[*i]];
    let op_label = get_op_label(s0, s1, s2, s3);

    let mut row = vec![ZERO; TRACE_WIDTH];
    main_trace.read_row_into(*i, &mut row);

    alphas[0]
        + alphas[1].mul_base(op_label)
        + alphas[2].mul_base(row[MEMORY_CTX_COL_IDX])
        + alphas[3].mul_base(row[MEMORY_ADDR_COL_IDX])
        + alphas[4].mul_base(row[MEMORY_CLK_COL_IDX])
        + alphas[5].mul_base(row[MEMORY_V_COL_RANGE.start])
        + alphas[6].mul_base(row[MEMORY_V_COL_RANGE.start + 1])
        + alphas[7].mul_base(row[MEMORY_V_COL_RANGE.start + 2])
        + alphas[8].mul_base(row[MEMORY_V_COL_RANGE.start + 3])
}

fn build_kernel_chiplet<E>(main_trace: &ColMatrix<Felt>, i: &usize, alphas: &[E]) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let [s0, s1, s2, s3] = [ONE, ONE, ONE, ZERO];
    let op_label = get_op_label(s0, s1, s2, s3);

    let mut row = vec![ZERO; TRACE_WIDTH];
    main_trace.read_row_into(*i, &mut row);

    alphas[0]
        + alphas[1].mul_base(op_label)
        + alphas[2].mul_base(row[CHIPLETS_OFFSET + 6])
        + alphas[3].mul_base(row[CHIPLETS_OFFSET + 7])
        + alphas[4].mul_base(row[CHIPLETS_OFFSET + 8])
        + alphas[5].mul_base(row[CHIPLETS_OFFSET + 9])
}

/// Reduces a slice of elements to a single field element in the field specified by E using a slice
/// of alphas of matching length. This can be used to build the value for a single word or for an
/// entire [HasherState].
fn build_value<E: FieldElement<BaseField = Felt>>(alphas: &[E], elements: &[Felt]) -> E {
    assert_eq!(alphas.len(), elements.len());
    let mut value = E::ZERO;
    for (&alpha, &element) in alphas.iter().zip(elements.iter()) {
        value += alpha.mul_base(element);
    }
    value
}

fn get_op_label(s0: Felt, s1: Felt, s2: Felt, s3: Felt) -> Felt {
    s3.mul_small(8) + s2.mul_small(4) + s1.mul_small(2) + s0 + ONE
}

/// Returns the hash cycle corresponding to the provided Hasher address.
fn addr_to_hash_cycle(addr: Felt) -> usize {
    let row = (addr.as_int() - 1) as usize;
    let cycle_row = row % HASH_CYCLE_LEN;
    debug_assert!(
        cycle_row == 0 || cycle_row == HASH_CYCLE_LEN - 1,
        "invalid address for hasher lookup"
    );

    cycle_row
}

fn addr_to_row_index(addr: Felt) -> usize {
    (addr.as_int() - 1) as usize
}
