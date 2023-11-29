use super::{
    trace::{build_lookup_table_row_values, AuxColumnBuilder, LookupTableRow},
    BTreeMap, ColMatrix, Felt, FieldElement, StarkField, Vec, Word,
};

use miden_air::trace::chiplets::{
    hasher::{
        DIGEST_RANGE, HASH_CYCLE_LEN, LINEAR_HASH_LABEL, MP_VERIFY_LABEL, MR_UPDATE_NEW_LABEL,
        MR_UPDATE_OLD_LABEL, RETURN_HASH_LABEL, RETURN_STATE_LABEL, STATE_WIDTH,
    },
    kernel_rom::KERNEL_PROC_LABEL,
    memory::{MEMORY_READ_LABEL, MEMORY_WRITE_LABEL},
};
pub(crate) use virtual_table::{ChipletsVTableRow, ChipletsVTableUpdate};
use vm_core::{utils::uninit_vector, Operation, ONE, ZERO};
use winter_prover::math::batch_inversion;

mod bus;
mod virtual_table;
pub(crate) use bus::{ChipletLookup, ChipletsBus, ChipletsBusRow};

mod main_trace;
use main_trace::MainTrace;

// CONSTANTS
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
const NUM_HEADER_ALPHAS: usize = 4;

// CHIPLETS AUXILIARY TRACE BUILDER
// ================================================================================================

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

    /// Builds the chiplets bus auxiliary trace column.
    ///
    /// The bus is constructed in two stages. In the first stage, the requests sent to the chiplets
    /// are computed, batch inverted and stored in a vector of length equal to the trace length.
    /// The responses are also computed at this stage and stored in another vector of the same size
    /// separately. In the second stage, the bus column is constructed by computing the component-wise
    /// cumulative product of the two vectors.
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

/// Constructs the requests made by the VM-components to the chiplets at row i.
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
        MLOADW => build_mem_request(main_trace, MEMORY_READ_LABEL, true, alphas, i),
        MSTOREW => build_mem_request(main_trace, MEMORY_WRITE_LABEL, true, alphas, i),
        MLOAD => build_mem_request(main_trace, MEMORY_READ_LABEL, false, alphas, i),
        MSTORE => build_mem_request(main_trace, MEMORY_WRITE_LABEL, false, alphas, i),
        MSTREAM => build_mstream_request(main_trace, alphas, i),
        HPERM => build_hperm_request(main_trace, alphas, i),
        MPVERIFY => build_mpverify_request(main_trace, alphas, i),
        MRUPDATE => build_mrupdate_request(main_trace, alphas, i),
        _ => E::ONE,
    }
}

/// Builds requests made to the hasher chiplet at the start of a control block.
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

/// Builds requests made to kernel ROM chiplet when initializing a syscall block.
fn build_syscall_block_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    op_code_felt: Felt,
    alphas: &[E],
    i: usize,
) -> E {
    let factor1 = build_control_block_request(main_trace, op_code_felt, alphas, i);

    let op_label = KERNEL_PROC_LABEL;
    let state = main_trace.decoder_hasher_state(i);
    let factor2 = alphas[0]
        + alphas[1].mul_base(op_label)
        + alphas[2].mul_base(state[0])
        + alphas[3].mul_base(state[1])
        + alphas[4].mul_base(state[2])
        + alphas[5].mul_base(state[3]);

    factor1 * factor2
}

/// Builds requests made to the hasher chiplet at the start of a span block.
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

/// Builds requests made to the hasher chiplet at the start of a respan block.
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

/// Builds requests made to the hasher chiplet at the end of a block.
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

/// Builds requests made to the bitwise chiplet. This can be either a request for the computation
/// of a `XOR` or an `AND` operation.
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

/// Builds `MLOAD*` and `MSTORE*` requests made to the memory chiplet.
fn build_mem_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    op_label: u8,
    word: bool,
    alphas: &[E],
    i: usize,
) -> E {
    let ctx = main_trace.ctx(i);
    let clk = main_trace.clk(i);

    let (v0, v1, v2, v3) = if word {
        (
            main_trace.s0(i + 1),
            main_trace.s1(i + 1),
            main_trace.s2(i + 1),
            main_trace.s3(i + 1),
        )
    } else {
        (
            main_trace.helper_0(i),
            main_trace.helper_1(i),
            main_trace.helper_2(i),
            main_trace.s0(i + 1),
        )
    };

    let s0_cur = main_trace.s0(i);

    alphas[0]
        + alphas[1].mul_base(Felt::from(op_label))
        + alphas[2].mul_base(ctx)
        + alphas[3].mul_base(s0_cur)
        + alphas[4].mul_base(clk)
        + alphas[5].mul_base(v3)
        + alphas[6].mul_base(v2)
        + alphas[7].mul_base(v1)
        + alphas[8].mul_base(v0)
}

/// Builds `MSTREAM` requests made to the memory chiplet.
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

/// Builds `HPERM` requests made to the hash chiplet.
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

/// Builds `MPVERIFY` requests made to the hash chiplet.
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

    v_input * v_output
}

/// Builds `MRUPDATE` requests made to the hash chiplet.
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
    let v_input_old = alphas[0]
        + alphas[1].mul_base(Felt::from(op_label))
        + alphas[2].mul_base(helper_0)
        + alphas[3].mul_base(s5)
        + sum_input;

    let op_label = RETURN_HASH_LABEL;
    let op_label = if addr_to_hash_cycle(helper_0 + s4.mul_small(8) - ONE) == 0 {
        op_label + 16
    } else {
        op_label + 32
    };

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
    let op_label = if addr_to_hash_cycle(helper_0 + s4.mul_small(8)) == 0 {
        op_label + 16
    } else {
        op_label + 32
    };
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
    let op_label = if addr_to_hash_cycle(helper_0 + s4.mul_small(16) - ONE) == 0 {
        op_label + 16
    } else {
        op_label + 32
    };

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

/// Constructs the responses from the chiplets to the other VM-components at row i.
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
        build_hasher_chiplet_responses(main_trace, i, alphas, selector1, selector2, selector3)
    } else if selector1 == ZERO {
        debug_assert_eq!(selector0, ONE);
        build_bitwise_chiplet_responses(main_trace, i, selector2, alphas)
    } else if selector2 == ZERO {
        debug_assert_eq!(selector0, ONE);
        debug_assert_eq!(selector1, ONE);
        build_memory_chiplet_responses(main_trace, i, selector3, alphas)
    } else if selector3 == ZERO && selector4 == ONE {
        debug_assert_eq!(selector0, ONE);
        debug_assert_eq!(selector1, ONE);
        debug_assert_eq!(selector2, ONE);
        build_kernel_chiplet_responses(main_trace, i, alphas)
    } else {
        debug_assert_eq!(selector0, ONE);
        debug_assert_eq!(selector1, ONE);
        debug_assert_eq!(selector2, ONE);
        debug_assert_eq!(selector3, ONE);
        E::ONE
    }
}

/// Builds the response from the hasher chiplet at row `i`.
fn build_hasher_chiplet_responses<E>(
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

            let bit = node_index.as_int() & 1;
            let left_word = build_value(&alphas[8..12], &state[4..8]);
            let right_word = build_value(&alphas[8..12], &state[8..]);

            multiplicand = header + E::from(1 - bit).mul(left_word) + E::from(bit).mul(right_word);
        }
    }

    // f_hout, f_sout, f_abp == 1
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

/// Builds the response from the bitwise chiplet at row `i`.
fn build_bitwise_chiplet_responses<E>(
    main_trace: &MainTrace,
    i: usize,
    is_xor: Felt,
    alphas: &[E],
) -> E
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

/// Builds the response from the memory chiplet at row `i`.
fn build_memory_chiplet_responses<E>(
    main_trace: &MainTrace,
    i: usize,
    is_read: Felt,
    alphas: &[E],
) -> E
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

/// Builds the response from the kernel chiplet at row `i`.
fn build_kernel_chiplet_responses<E>(main_trace: &MainTrace, i: usize, alphas: &[E]) -> E
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

// HELPER FUNCTIONS
// ================================================================================================

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

/// Returns the operation unique label.
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

/// Convenience method to convert from addresses to rows.
fn addr_to_row_index(addr: Felt) -> usize {
    (addr.as_int() - 1) as usize
}
