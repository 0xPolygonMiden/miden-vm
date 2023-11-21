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
        kernel_rom::KERNEL_PROC_LABEL,
        memory::{MEMORY_READ_LABEL, MEMORY_WRITE_LABEL},
        BITWISE_A_COL_IDX, BITWISE_B_COL_IDX, BITWISE_OUTPUT_COL_IDX, HASHER_NODE_INDEX_COL_IDX,
        HASHER_STATE_COL_RANGE, MEMORY_ADDR_COL_IDX, MEMORY_CLK_COL_IDX, MEMORY_CTX_COL_IDX,
        MEMORY_V_COL_RANGE,
    },
    decoder::{HASHER_STATE_OFFSET, NUM_HASHER_COLUMNS, USER_OP_HELPERS_OFFSET},
    CHIPLETS_OFFSET, CLK_COL_IDX, CTX_COL_IDX, DECODER_TRACE_OFFSET, STACK_TRACE_OFFSET,
};
pub(crate) use virtual_table::{ChipletsVTableRow, ChipletsVTableUpdate};
use vm_core::{
    utils::{range, uninit_vector},
    Operation,
};
use vm_core::{ONE, ZERO};
use winter_prover::math::batch_inversion;

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
        let mut result_1: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
        let mut result_2: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
        let mut result: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
        result_1[0] = E::ONE;
        result_2[0] = E::ONE;
        result[0] = E::ONE;
        let main_tr = MainTrace::new(main_trace);

        for i in 0..main_trace.num_rows() - 1 {
            result_1[i] = chiplets_requests(&main_tr, alphas, i);
            result_2[i] = chiplets_responses(&main_tr, alphas, i);
        }

        let result_1 = batch_inversion(&result_1);

        for i in 0..main_trace.num_rows() - 1 {
            result[i + 1] = result[i] * result_2[i] * result_1[i];
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

// CHIPLETS REQUESTS
// ================================================================================================

fn chiplets_requests<E>(main_trace: &MainTrace, alphas: &[E], i: usize) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let op_code_felt = main_trace.get_op_code(i);
    let op_code = op_code_felt.as_int() as u8;

    match op_code {
        JOIN | SPLIT | LOOP | DYN | CALL => {
            build_control_block_request(main_trace, op_code_felt, alphas, i)
        }

        SYSCALL => build_syscall_block_request(main_trace, op_code_felt, alphas, i),

        SPAN => build_span_block_request(main_trace, alphas, i),

        RESPAN => build_respan_block_request(main_trace, alphas, i),

        END => build_end_block_request(main_trace, alphas, i),

        AND => build_bitwise_request(main_trace, ZERO, alphas, i),

        XOR => build_bitwise_request(main_trace, ONE, alphas, i),

        MLOADW => build_mloadw_request(main_trace, alphas, i),

        MSTOREW => build_mstorew_request(main_trace, alphas, i),

        MLOAD => build_mload_request(main_trace, alphas, i),

        MSTORE => build_mstore_request(main_trace, alphas, i),

        MSTREAM => build_mstream_request(main_trace, alphas, i),

        HPERM => build_hperm_request(main_trace, alphas, i),

        MPVERIFY => build_mpverify_request(main_trace, alphas, i),

        MRUPDATE => build_mrupdate_request(main_trace, alphas, i),

        _ => E::ONE,
    }
}

fn build_control_block_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    op_code_felt: Felt,
    alphas: &[E],
    i: usize,
) -> E {
    let op_label = LINEAR_HASH_LABEL;
    let addr_nxt = main_trace.addr(i + 1);
    let first_cycle_row = addr_to_row_index(addr_nxt) % 8 == 0;
    let transition_label = if first_cycle_row { op_label + 16 } else { op_label + 32 };

    let header =
        alphas[0] + alphas[1].mul_base(Felt::from(transition_label)) + alphas[2].mul_base(addr_nxt);

    let state = main_trace.decoder_hasher_state(i);

    header + build_value(&alphas[8..16], &state) + alphas[5].mul_base(op_code_felt)
}

fn build_syscall_block_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    op_code_felt: Felt,
    alphas: &[E],
    i: usize,
) -> E {
    let op_label = LINEAR_HASH_LABEL;
    let addr_nxt = main_trace.addr(i + 1);
    let first_cycle_row = addr_to_row_index(addr_nxt) % 8 == 0;
    let transition_label = if first_cycle_row { op_label + 16 } else { op_label + 32 };

    let header =
        alphas[0] + alphas[1].mul_base(Felt::from(transition_label)) + alphas[2].mul_base(addr_nxt);

    let state = main_trace.decoder_hasher_state(i);
    let factor1 = header + build_value(&alphas[8..16], &state) + alphas[5].mul_base(op_code_felt);

    let op_label = KERNEL_PROC_LABEL;
    let factor2 = alphas[0]
        + alphas[1].mul_base(op_label)
        + alphas[2].mul_base(state[0])
        + alphas[3].mul_base(state[1])
        + alphas[4].mul_base(state[2])
        + alphas[5].mul_base(state[3]);

    factor1 * factor2
}

fn build_span_block_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    i: usize,
) -> E {
    let op_label = LINEAR_HASH_LABEL;
    let addr_nxt = main_trace.addr(i + 1);
    let first_cycle_row = addr_to_row_index(addr_nxt) % 8 == 0;
    let transition_label = if first_cycle_row { op_label + 16 } else { op_label + 32 };

    let header =
        alphas[0] + alphas[1].mul_base(Felt::from(transition_label)) + alphas[2].mul_base(addr_nxt);

    let state = main_trace.decoder_hasher_state(i);

    header + build_value(&alphas[8..16], &state)
}

fn build_respan_block_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    i: usize,
) -> E {
    let op_label = LINEAR_HASH_LABEL;
    let addr_nxt = main_trace.addr(i + 1);

    let first_cycle_row = addr_to_row_index(addr_nxt - ONE) % 8 == 0;
    let transition_label = if first_cycle_row { op_label + 16 } else { op_label + 32 };

    let header = alphas[0]
        + alphas[1].mul_base(Felt::from(transition_label))
        + alphas[2].mul_base(addr_nxt - ONE)
        + alphas[3].mul_base(ZERO);

    let state = &main_trace.chiplet_hasher_state(i - 2)[4..];
    let state_nxt = &main_trace.chiplet_hasher_state(i - 1)[4..];

    header + build_value(&alphas[8..16], state_nxt) - build_value(&alphas[8..16], state)
}

fn build_end_block_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    i: usize,
) -> E {
    let op_label = RETURN_HASH_LABEL;
    let addr = main_trace.addr(i) + Felt::from(7_u64);

    let first_cycle_row = addr_to_row_index(addr) % 8 == 0;
    let transition_label = if first_cycle_row { op_label + 16 } else { op_label + 32 };

    let header =
        alphas[0] + alphas[1].mul_base(Felt::from(transition_label)) + alphas[2].mul_base(addr);

    let state = main_trace.decoder_hasher_state(i);
    let digest = &state[..4];

    header + build_value(&alphas[8..12], digest)
}

fn build_bitwise_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    is_xor: Felt,
    alphas: &[E],
    i: usize,
) -> E {
    let op_label = get_op_label(ONE, ZERO, is_xor, ZERO);
    let a = main_trace.s0(i);
    let b = main_trace.s1(i);
    let z = main_trace.s0(i + 1);

    alphas[0]
        + alphas[1].mul_base(op_label)
        + alphas[2].mul_base(b)
        + alphas[3].mul_base(a)
        + alphas[4].mul_base(z)
}

fn build_mloadw_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    i: usize,
) -> E {
    let ctx = main_trace.ctx(i);
    let clk = main_trace.clk(i);

    let s0_cur = main_trace.s0(i);
    let s0_nxt = main_trace.s0(i + 1);
    let s1_nxt = main_trace.s1(i + 1);
    let s2_nxt = main_trace.s2(i + 1);
    let s3_nxt = main_trace.s3(i + 1);

    let op_label = MEMORY_READ_LABEL;

    alphas[0]
        + alphas[1].mul_base(Felt::from(op_label))
        + alphas[2].mul_base(ctx)
        + alphas[3].mul_base(s0_cur)
        + alphas[4].mul_base(clk)
        + alphas[5].mul_base(s3_nxt)
        + alphas[6].mul_base(s2_nxt)
        + alphas[7].mul_base(s1_nxt)
        + alphas[8].mul_base(s0_nxt)
}

fn build_mstorew_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    i: usize,
) -> E {
    let ctx = main_trace.ctx(i);
    let clk = main_trace.clk(i);

    let s0_cur = main_trace.s0(i);
    let s0_nxt = main_trace.s0(i + 1);
    let s1_nxt = main_trace.s1(i + 1);
    let s2_nxt = main_trace.s2(i + 1);
    let s3_nxt = main_trace.s3(i + 1);

    let op_label = MEMORY_WRITE_LABEL;

    alphas[0]
        + alphas[1].mul_base(Felt::from(op_label))
        + alphas[2].mul_base(ctx)
        + alphas[3].mul_base(s0_cur)
        + alphas[4].mul_base(clk)
        + alphas[5].mul_base(s3_nxt)
        + alphas[6].mul_base(s2_nxt)
        + alphas[7].mul_base(s1_nxt)
        + alphas[8].mul_base(s0_nxt)
}

fn build_mload_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    i: usize,
) -> E {
    let ctx = main_trace.ctx(i);
    let clk = main_trace.clk(i);

    let helper_0 = main_trace.helper_0(i);
    let helper_1 = main_trace.helper_1(i);
    let helper_2 = main_trace.helper_2(i);

    let s0_cur = main_trace.s0(i);
    let s0_nxt = main_trace.s0(i + 1);

    let op_label = MEMORY_READ_LABEL;

    alphas[0]
        + alphas[1].mul_base(Felt::from(op_label))
        + alphas[2].mul_base(ctx)
        + alphas[3].mul_base(s0_cur)
        + alphas[4].mul_base(clk)
        + alphas[5].mul_base(s0_nxt)
        + alphas[6].mul_base(helper_2)
        + alphas[7].mul_base(helper_1)
        + alphas[8].mul_base(helper_0)
}

fn build_mstore_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    i: usize,
) -> E {
    let ctx = main_trace.ctx(i);
    let clk = main_trace.clk(i);

    let helper_0 = main_trace.helper_0(i);
    let helper_1 = main_trace.helper_1(i);
    let helper_2 = main_trace.helper_2(i);

    let s0_cur = main_trace.s0(i);
    let s0_nxt = main_trace.s0(i + 1);

    let op_label = MEMORY_WRITE_LABEL;

    alphas[0]
        + alphas[1].mul_base(Felt::from(op_label))
        + alphas[2].mul_base(ctx)
        + alphas[3].mul_base(s0_cur)
        + alphas[4].mul_base(clk)
        + alphas[5].mul_base(s0_nxt)
        + alphas[6].mul_base(helper_2)
        + alphas[7].mul_base(helper_1)
        + alphas[8].mul_base(helper_0)
}

fn build_mstream_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    i: usize,
) -> E {
    let ctx = main_trace.ctx(i);
    let clk = main_trace.clk(i);

    let s0_nxt = main_trace.s0(i + 1);
    let s1_nxt = main_trace.s1(i + 1);
    let s2_nxt = main_trace.s2(i + 1);
    let s3_nxt = main_trace.s3(i + 1);
    let s4_nxt = main_trace.s4(i + 1);
    let s5_nxt = main_trace.s5(i + 1);
    let s6_nxt = main_trace.s6(i + 1);
    let s7_nxt = main_trace.s7(i + 1);

    let s12_cur = main_trace.s12(i);

    let op_label = MEMORY_READ_LABEL;

    let factor1 = alphas[0]
        + alphas[1].mul_base(Felt::from(op_label))
        + alphas[2].mul_base(ctx)
        + alphas[3].mul_base(s12_cur)
        + alphas[4].mul_base(clk)
        + alphas[5].mul_base(s7_nxt)
        + alphas[6].mul_base(s6_nxt)
        + alphas[7].mul_base(s5_nxt)
        + alphas[8].mul_base(s4_nxt);

    let factor2 = alphas[0]
        + alphas[1].mul_base(Felt::from(op_label))
        + alphas[2].mul_base(ctx)
        + alphas[3].mul_base(s12_cur + ONE)
        + alphas[4].mul_base(clk)
        + alphas[5].mul_base(s3_nxt)
        + alphas[6].mul_base(s2_nxt)
        + alphas[7].mul_base(s1_nxt)
        + alphas[8].mul_base(s0_nxt);
    factor1 * factor2
}

fn build_hperm_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    i: usize,
) -> E {
    let helper_0 = main_trace.helper_0(i);

    let s0_s12_cur = [
        main_trace.s0(i),
        main_trace.s1(i),
        main_trace.s2(i),
        main_trace.s3(i),
        main_trace.s4(i),
        main_trace.s5(i),
        main_trace.s6(i),
        main_trace.s7(i),
        main_trace.s8(i),
        main_trace.s9(i),
        main_trace.s10(i),
        main_trace.s11(i),
    ];

    let s0_s12_nxt = [
        main_trace.s0(i + 1),
        main_trace.s1(i + 1),
        main_trace.s2(i + 1),
        main_trace.s3(i + 1),
        main_trace.s4(i + 1),
        main_trace.s5(i + 1),
        main_trace.s6(i + 1),
        main_trace.s7(i + 1),
        main_trace.s8(i + 1),
        main_trace.s9(i + 1),
        main_trace.s10(i + 1),
        main_trace.s11(i + 1),
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

    v_input * v_output
}

fn build_mpverify_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    i: usize,
) -> E {
    let helper_0 = main_trace.helper_0(i);

    let s0_s3 = [main_trace.s0(i), main_trace.s1(i), main_trace.s2(i), main_trace.s3(i)];
    let s4 = main_trace.s4(i);
    let s5 = main_trace.s5(i);
    let s6_s9 = [main_trace.s6(i), main_trace.s7(i), main_trace.s8(i), main_trace.s9(i)];

    let op_label = MP_VERIFY_LABEL;
    let op_label = if addr_to_hash_cycle(helper_0) == 0 {
        op_label + 16
    } else {
        op_label + 32
    };

    let hash_row = main_trace.chiplet_hasher_state(helper_0.as_int() as usize - 1);

    let sibling = &hash_row[4..8];

    let sum_input = alphas[8..12]
        .iter()
        //.rev()
        .enumerate()
        //.fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(s0_s3[i]));
        .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(sibling[i]));
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

    v_input * v_output
}

fn build_mrupdate_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    i: usize,
) -> E {
    let helper_0 = main_trace.helper_0(i);

    let s0_s3 = [main_trace.s0(i), main_trace.s1(i), main_trace.s2(i), main_trace.s3(i)];
    let s0_s3_nxt = [
        main_trace.s0(i + 1),
        main_trace.s1(i + 1),
        main_trace.s2(i + 1),
        main_trace.s3(i + 1),
    ];
    let s4 = main_trace.s4(i);
    let s5 = main_trace.s5(i);
    let s6_s9 = [main_trace.s6(i), main_trace.s7(i), main_trace.s8(i), main_trace.s9(i)];
    let s10_s13 = [main_trace.s10(i), main_trace.s11(i), main_trace.s12(i), main_trace.s13(i)];

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

    v_input_new * v_input_old * v_output_new * v_output_old
}

// CHIPLETS RESPONSES
// ================================================================================================

fn chiplets_responses<E>(main_trace: &MainTrace, alphas: &[E], i: usize) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let selector0 = main_trace.chiplet_selector_0(i);
    let selector1 = main_trace.chiplet_selector_1(i);
    let selector2 = main_trace.chiplet_selector_2(i);
    let selector3 = main_trace.chiplet_selector_3(i);
    let selector4 = main_trace.chiplet_selector_4(i);

    if selector0 == ZERO {
        return build_hasher_chiplet(main_trace, i, alphas, selector1, selector2, selector3);
    }

    if selector0 == ONE && selector1 == ZERO {
        return build_bitwise_chiplet(main_trace, i, selector2, alphas);
    }

    if selector0 == ONE && selector1 == ONE && selector2 == ZERO {
        return build_memory_chiplet(main_trace, i, selector3, alphas);
    }

    if selector0 == ONE
        && selector1 == ONE
        && selector2 == ONE
        && selector3 == ZERO
        && selector4 == ONE
    {
        build_kernel_chiplet(main_trace, i, alphas)
    } else {
        E::ONE
    }
}

fn build_hasher_chiplet<E>(
    main_trace: &MainTrace,
    i: usize,
    alphas: &[E],
    col1: Felt,
    col2: Felt,
    col3: Felt,
) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let mut multiplicand = E::ONE;

    // f_bp, f_mp, f_mv or f_mu == 1
    if i % 8 == 0 {
        let [s0, s1, s2] = [col1, col2, col3];

        let state = main_trace.chiplet_hasher_state(i);
        let alphas_state = &alphas[NUM_HEADER_ALPHAS..(NUM_HEADER_ALPHAS + STATE_WIDTH)];
        let node_index = main_trace.chiplet_node_index(i);

        // f_bp == 1
        // v_all = v_h + v_a + v_b + v_c
        if s0 == ONE && s1 == ZERO && s2 == ZERO {
            let op_label = LINEAR_HASH_LABEL;
            let transition_label = Felt::from(op_label) + Felt::from(16_u8);

            let header = alphas[0]
                + alphas[1].mul_base(transition_label)
                + alphas[2].mul_base(Felt::from((i + 1) as u64))
                + alphas[3].mul_base(node_index);

            multiplicand = header + build_value(alphas_state, &state);
        }

        // f_mp or f_mv or f_mu == 1
        // v_leaf = v_h + (1 - b) * v_b + b * v_d
        if s0 == ONE && !(s1 == ZERO && s2 == ZERO) {
            let op_label = get_op_label(ZERO, s0, s1, s2);
            let transition_label = op_label + Felt::from(16_u8);

            let header = alphas[0]
                + alphas[1].mul_base(transition_label)
                + alphas[2].mul_base(Felt::from((i + 1) as u64))
                + alphas[3].mul_base(node_index);

            let bit = (node_index.as_int() >> 1) & 1;
            let left_word = build_value(&alphas[8..12], &state[4..8]);
            let right_word = build_value(&alphas[8..12], &state[8..]);

            multiplicand = header + E::from(1 - bit).mul(left_word) + E::from(bit).mul(right_word);
        }
    }

    // f_hout, f_sout, f_abp, f_mpa, f_mva or f_mua == 1
    if i % 8 == 7 {
        let [s0, s1, s2] = [col1, col2, col3];

        let state = main_trace.chiplet_hasher_state(i);
        let alphas_state = &alphas[NUM_HEADER_ALPHAS..(NUM_HEADER_ALPHAS + STATE_WIDTH)];
        let node_index = main_trace.chiplet_node_index(i);

        // f_hout == 1
        // v_res = v_h + v_b;
        if s0 == ZERO && s1 == ZERO && s2 == ZERO {
            let op_label = RETURN_HASH_LABEL;
            let transition_label = Felt::from(op_label) + Felt::from(32_u8);

            let header = alphas[0]
                + alphas[1].mul_base(transition_label)
                + alphas[2].mul_base(Felt::from((i + 1) as u64))
                + alphas[3].mul_base(node_index);

            multiplicand = header + build_value(&alphas_state[4..8], &state[DIGEST_RANGE]);
        }

        // f_sout == 1
        // v_all = v_h + v_a + v_b + v_c
        if s0 == ZERO && s1 == ZERO && s2 == ONE {
            let op_label = RETURN_STATE_LABEL;
            let transition_label = Felt::from(op_label) + Felt::from(32_u8);

            let header = alphas[0]
                + alphas[1].mul_base(transition_label)
                + alphas[2].mul_base(Felt::from((i + 1) as u64))
                + alphas[3].mul_base(node_index);

            multiplicand = header + build_value(alphas_state, &state);
        }

        // f_abp == 1
        // v_abp = v_h + v_b' + v_c' - v_b - v_c
        if s0 == ONE && s1 == ZERO && s2 == ZERO {
            let op_label = get_op_label(ZERO, s0, s1, s2);
            let transition_label = op_label + Felt::from(32_u8);

            let header = alphas[0]
                + alphas[1].mul_base(transition_label)
                + alphas[2].mul_base(Felt::from((i + 1) as u64))
                + alphas[3].mul_base(node_index);

            let state_nxt = main_trace.chiplet_hasher_state(i + 1);

            // build the value from the difference of the hasher state's just before and right
            // after the absorption of new elements.
            let next_state_value = build_value(&alphas_state[4..12], &state_nxt[4..]);
            let state_value = build_value(&alphas_state[4..12], &state[4..]);

            multiplicand = header + next_state_value - state_value;
        }
    }
    multiplicand
}

fn build_bitwise_chiplet<E>(main_trace: &MainTrace, i: usize, is_xor: Felt, alphas: &[E]) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let mut multiplicand = E::ONE;
    if i % 8 == 7 {
        let op_label = get_op_label(ONE, ZERO, is_xor, ZERO);

        let a = main_trace.chiplet_bitwise_a(i);
        let b = main_trace.chiplet_bitwise_b(i);
        let z = main_trace.chiplet_bitwise_z(i);

        multiplicand = alphas[0]
            + alphas[1].mul_base(op_label)
            + alphas[2].mul_base(a)
            + alphas[3].mul_base(b)
            + alphas[4].mul_base(z);
    }
    multiplicand
}

fn build_memory_chiplet<E>(main_trace: &MainTrace, i: usize, is_read: Felt, alphas: &[E]) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let op_label = get_op_label(ONE, ONE, ZERO, is_read);

    let ctx = main_trace.chiplet_memory_ctx(i);
    let clk = main_trace.chiplet_memory_clk(i);
    let addr = main_trace.chiplet_memory_addr(i);
    let value0 = main_trace.chiplet_memory_value_0(i);
    let value1 = main_trace.chiplet_memory_value_1(i);
    let value2 = main_trace.chiplet_memory_value_2(i);
    let value3 = main_trace.chiplet_memory_value_3(i);

    alphas[0]
        + alphas[1].mul_base(op_label)
        + alphas[2].mul_base(ctx)
        + alphas[3].mul_base(addr)
        + alphas[4].mul_base(clk)
        + alphas[5].mul_base(value0)
        + alphas[6].mul_base(value1)
        + alphas[7].mul_base(value2)
        + alphas[8].mul_base(value3)
}

fn build_kernel_chiplet<E>(main_trace: &MainTrace, i: usize, alphas: &[E]) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let op_label = KERNEL_PROC_LABEL;

    let root0 = main_trace.chiplet_kernel_root_0(i);
    let root1 = main_trace.chiplet_kernel_root_1(i);
    let root2 = main_trace.chiplet_kernel_root_2(i);
    let root3 = main_trace.chiplet_kernel_root_3(i);

    alphas[0]
        + alphas[1].mul_base(op_label)
        + alphas[2].mul_base(root0)
        + alphas[3].mul_base(root1)
        + alphas[4].mul_base(root2)
        + alphas[5].mul_base(root3)
}

// HELPER CONSTANTS, STRUCTS AND FUNCTIONS
// ================================================================================================

const JOIN: u8 = Operation::Join.op_code();
const SPLIT: u8 = Operation::Split.op_code();
const LOOP: u8 = Operation::Loop.op_code();
const DYN: u8 = Operation::Dyn.op_code();
const CALL: u8 = Operation::Call.op_code();
const SYSCALL: u8 = Operation::SysCall.op_code();
const SPAN: u8 = Operation::Span.op_code();
const RESPAN: u8 = Operation::Respan.op_code();
const END: u8 = Operation::End.op_code();
const AND: u8 = Operation::U32and.op_code();
const XOR: u8 = Operation::U32xor.op_code();
const MLOADW: u8 = Operation::MLoadW.op_code();
const MSTOREW: u8 = Operation::MStoreW.op_code();
const MLOAD: u8 = Operation::MLoad.op_code();
const MSTORE: u8 = Operation::MStore.op_code();
const MSTREAM: u8 = Operation::MStream.op_code();
const HPERM: u8 = Operation::HPerm.op_code();
const MPVERIFY: u8 = Operation::MpVerify.op_code();
const MRUPDATE: u8 = Operation::MrUpdate.op_code();
const DECODER_HASHER_RANGE: Range<usize> =
    range(DECODER_TRACE_OFFSET + HASHER_STATE_OFFSET, NUM_HASHER_COLUMNS);

struct MainTrace<'a> {
    columns: &'a ColMatrix<Felt>,
}

impl<'a> MainTrace<'a> {
    pub fn new(main_trace: &'a ColMatrix<Felt>) -> Self {
        Self {
            columns: main_trace,
        }
    }

    // System columns

    pub fn clk(&self, i: usize) -> Felt {
        self.columns.get_column(CLK_COL_IDX)[i]
    }

    pub fn ctx(&self, i: usize) -> Felt {
        self.columns.get_column(CTX_COL_IDX)[i]
    }

    // Decoder columns

    pub fn addr(&self, i: usize) -> Felt {
        self.columns.get_column(DECODER_TRACE_OFFSET)[i]
    }

    pub fn helper_0(&self, i: usize) -> Felt {
        self.columns.get_column(DECODER_TRACE_OFFSET + USER_OP_HELPERS_OFFSET)[i]
    }

    pub fn helper_1(&self, i: usize) -> Felt {
        self.columns.get_column(DECODER_TRACE_OFFSET + USER_OP_HELPERS_OFFSET + 1)[i]
    }

    pub fn helper_2(&self, i: usize) -> Felt {
        self.columns.get_column(DECODER_TRACE_OFFSET + USER_OP_HELPERS_OFFSET + 2)[i]
    }

    pub fn decoder_hasher_state(&self, i: usize) -> [Felt; NUM_HASHER_COLUMNS] {
        let mut state = [ZERO; NUM_HASHER_COLUMNS];
        for (idx, col_idx) in DECODER_HASHER_RANGE.enumerate() {
            let column = self.columns.get_column(col_idx);
            state[idx] = column[i];
        }
        state
    }

    pub fn get_op_code(&self, i: usize) -> Felt {
        let col_b0 = self.columns.get_column(DECODER_TRACE_OFFSET + 1);
        let col_b1 = self.columns.get_column(DECODER_TRACE_OFFSET + 2);
        let col_b2 = self.columns.get_column(DECODER_TRACE_OFFSET + 3);
        let col_b3 = self.columns.get_column(DECODER_TRACE_OFFSET + 4);
        let col_b4 = self.columns.get_column(DECODER_TRACE_OFFSET + 5);
        let col_b5 = self.columns.get_column(DECODER_TRACE_OFFSET + 6);
        let col_b6 = self.columns.get_column(DECODER_TRACE_OFFSET + 7);
        let [b0, b1, b2, b3, b4, b5, b6] =
            [col_b0[i], col_b1[i], col_b2[i], col_b3[i], col_b4[i], col_b5[i], col_b6[i]];
        b0 + b1.mul_small(2)
            + b2.mul_small(4)
            + b3.mul_small(8)
            + b4.mul_small(16)
            + b5.mul_small(32)
            + b6.mul_small(64)
    }

    // Stack columns

    pub fn s0(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET)[i]
    }

    pub fn s1(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 1)[i]
    }

    pub fn s2(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 2)[i]
    }

    pub fn s3(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 3)[i]
    }

    pub fn s4(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 4)[i]
    }

    pub fn s5(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 5)[i]
    }

    pub fn s6(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 6)[i]
    }

    pub fn s7(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 7)[i]
    }

    pub fn s8(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 8)[i]
    }

    pub fn s9(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 9)[i]
    }

    pub fn s10(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 10)[i]
    }

    pub fn s11(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 11)[i]
    }

    pub fn s12(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 12)[i]
    }

    pub fn s13(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 13)[i]
    }

    // Chiplets columns

    pub fn chiplet_selector_0(&self, i: usize) -> Felt {
        self.columns.get_column(CHIPLETS_OFFSET)[i]
    }

    pub fn chiplet_selector_1(&self, i: usize) -> Felt {
        self.columns.get_column(CHIPLETS_OFFSET + 1)[i]
    }

    pub fn chiplet_selector_2(&self, i: usize) -> Felt {
        self.columns.get_column(CHIPLETS_OFFSET + 2)[i]
    }

    pub fn chiplet_selector_3(&self, i: usize) -> Felt {
        self.columns.get_column(CHIPLETS_OFFSET + 3)[i]
    }

    pub fn chiplet_selector_4(&self, i: usize) -> Felt {
        self.columns.get_column(CHIPLETS_OFFSET + 4)[i]
    }

    pub fn chiplet_hasher_state(&self, i: usize) -> [Felt; STATE_WIDTH] {
        let mut state = [ZERO; STATE_WIDTH];
        for (idx, col_idx) in HASHER_STATE_COL_RANGE.enumerate() {
            let column = self.columns.get_column(col_idx);
            state[idx] = column[i];
        }
        state
    }

    pub fn chiplet_node_index(&self, i: usize) -> Felt {
        self.columns.get(HASHER_NODE_INDEX_COL_IDX, i)
    }

    pub fn chiplet_bitwise_a(&self, i: usize) -> Felt {
        self.columns.get_column(BITWISE_A_COL_IDX)[i]
    }

    pub fn chiplet_bitwise_b(&self, i: usize) -> Felt {
        self.columns.get_column(BITWISE_B_COL_IDX)[i]
    }

    pub fn chiplet_bitwise_z(&self, i: usize) -> Felt {
        self.columns.get_column(BITWISE_OUTPUT_COL_IDX)[i]
    }

    pub fn chiplet_memory_ctx(&self, i: usize) -> Felt {
        self.columns.get_column(MEMORY_CTX_COL_IDX)[i]
    }

    pub fn chiplet_memory_addr(&self, i: usize) -> Felt {
        self.columns.get_column(MEMORY_ADDR_COL_IDX)[i]
    }

    pub fn chiplet_memory_clk(&self, i: usize) -> Felt {
        self.columns.get_column(MEMORY_CLK_COL_IDX)[i]
    }

    pub fn chiplet_memory_value_0(&self, i: usize) -> Felt {
        self.columns.get_column(MEMORY_V_COL_RANGE.start)[i]
    }

    pub fn chiplet_memory_value_1(&self, i: usize) -> Felt {
        self.columns.get_column(MEMORY_V_COL_RANGE.start + 1)[i]
    }

    pub fn chiplet_memory_value_2(&self, i: usize) -> Felt {
        self.columns.get_column(MEMORY_V_COL_RANGE.start + 2)[i]
    }

    pub fn chiplet_memory_value_3(&self, i: usize) -> Felt {
        self.columns.get_column(MEMORY_V_COL_RANGE.start + 3)[i]
    }

    fn chiplet_kernel_root_0(&self, i: usize) -> Felt {
        self.columns.get_column(CHIPLETS_OFFSET + 6)[i]
    }

    fn chiplet_kernel_root_1(&self, i: usize) -> Felt {
        self.columns.get_column(CHIPLETS_OFFSET + 7)[i]
    }

    fn chiplet_kernel_root_2(&self, i: usize) -> Felt {
        self.columns.get_column(CHIPLETS_OFFSET + 8)[i]
    }

    fn chiplet_kernel_root_3(&self, i: usize) -> Felt {
        self.columns.get_column(CHIPLETS_OFFSET + 9)[i]
    }
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
