use alloc::vec::Vec;

use miden_air::{
    trace::{
        chiplets::{
            bitwise::OP_CYCLE_LEN as BITWISE_OP_CYCLE_LEN,
            hasher::{
                CAPACITY_LEN, DIGEST_RANGE, HASH_CYCLE_LEN, LINEAR_HASH_LABEL, MP_VERIFY_LABEL,
                MR_UPDATE_NEW_LABEL, MR_UPDATE_OLD_LABEL, NUM_ROUNDS, RETURN_HASH_LABEL,
                RETURN_STATE_LABEL, STATE_WIDTH,
            },
            kernel_rom::KERNEL_PROC_LABEL,
            memory::{
                MEMORY_ACCESS_ELEMENT, MEMORY_ACCESS_WORD, MEMORY_READ_ELEMENT_LABEL,
                MEMORY_READ_WORD_LABEL, MEMORY_WRITE_ELEMENT_LABEL, MEMORY_WRITE_WORD_LABEL,
            },
        },
        main_trace::MainTrace,
    },
    RowIndex,
};
use vm_core::{
    Kernel, Word, ONE, OPCODE_CALL, OPCODE_DYN, OPCODE_DYNCALL, OPCODE_END, OPCODE_HORNERBASE,
    OPCODE_HORNEREXT, OPCODE_HPERM, OPCODE_JOIN, OPCODE_LOOP, OPCODE_MLOAD, OPCODE_MLOADW,
    OPCODE_MPVERIFY, OPCODE_MRUPDATE, OPCODE_MSTORE, OPCODE_MSTOREW, OPCODE_MSTREAM, OPCODE_PIPE,
    OPCODE_RESPAN, OPCODE_SPAN, OPCODE_SPLIT, OPCODE_SYSCALL, OPCODE_U32AND, OPCODE_U32XOR, ZERO,
};

use super::{super::trace::AuxColumnBuilder, Felt, FieldElement};

// CONSTANTS
// ================================================================================================

const NUM_HEADER_ALPHAS: usize = 4;
const FOUR: Felt = Felt::new(4);

// CHIPLETS AUXILIARY TRACE BUILDER
// ================================================================================================

/// Constructs the execution trace for chiplets-related auxiliary columns (used in multiset checks).
pub struct AuxTraceBuilder {
    kernel: Kernel,
}

impl AuxTraceBuilder {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    pub fn new(kernel: Kernel) -> Self {
        Self { kernel }
    }

    // COLUMN TRACE CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Builds and returns the Chiplets's auxiliary trace columns. Currently this consists of
    /// a single bus column `b_chip` describing chiplet lookups requested by the stack and
    /// provided by chiplets in the Chiplets module.
    pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &MainTrace,
        rand_elements: &[E],
    ) -> Vec<Vec<E>> {
        let v_table_col_builder = ChipletsVTableColBuilder::new(self.kernel.clone());
        let bus_col_builder = BusColumnBuilder::default();
        let t_chip = v_table_col_builder.build_aux_column(main_trace, rand_elements);
        let b_chip = bus_col_builder.build_aux_column(main_trace, rand_elements);

        debug_assert_eq!(*t_chip.last().unwrap(), E::ONE);
        // TODO: Fix and re-enable after testing with miden-base
        // debug_assert_eq!(*b_chip.last().unwrap(), E::ONE);
        vec![t_chip, b_chip]
    }
}

// VIRTUAL TABLE COLUMN BUILDER
// ================================================================================================

/// Describes how to construct the execution trace of the chiplets virtual table auxiliary trace
/// column.
pub struct ChipletsVTableColBuilder {
    kernel: Kernel,
}

impl ChipletsVTableColBuilder {
    fn new(kernel: Kernel) -> Self {
        Self { kernel }
    }
}

impl<E: FieldElement<BaseField = Felt>> AuxColumnBuilder<E> for ChipletsVTableColBuilder {
    fn init_requests(&self, _main_trace: &MainTrace, alphas: &[E]) -> E {
        let mut requests = E::ONE;
        for (idx, proc_hash) in self.kernel.proc_hashes().iter().enumerate() {
            requests *= alphas[0]
                + alphas[1].mul_base((idx as u32).into())
                + alphas[2].mul_base(proc_hash[0])
                + alphas[3].mul_base(proc_hash[1])
                + alphas[4].mul_base(proc_hash[2])
                + alphas[5].mul_base(proc_hash[3]);
        }
        requests
    }

    fn get_requests_at(&self, main_trace: &MainTrace, alphas: &[E], row: RowIndex) -> E {
        chiplets_vtable_remove_sibling(main_trace, alphas, row)
    }

    fn get_responses_at(&self, main_trace: &MainTrace, alphas: &[E], row: RowIndex) -> E {
        chiplets_vtable_add_sibling(main_trace, alphas, row)
            * build_kernel_procedure_table_inclusions(main_trace, alphas, row)
    }
}

// VIRTUAL TABLE REQUESTS
// ================================================================================================

/// Constructs the removals from the table when the hasher absorbs a new sibling node while
/// computing the new Merkle root.
fn chiplets_vtable_remove_sibling<E>(main_trace: &MainTrace, alphas: &[E], row: RowIndex) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let f_mu: bool = main_trace.f_mu(row);
    let f_mua: bool = main_trace.f_mua(row);

    if f_mu {
        let index = main_trace.chiplet_node_index(row);
        let lsb = index.as_int() & 1;
        if lsb == 0 {
            let sibling = &main_trace.chiplet_hasher_state(row)[DIGEST_RANGE.end..];
            alphas[0]
                + alphas[3].mul_base(index)
                + alphas[12].mul_base(sibling[0])
                + alphas[13].mul_base(sibling[1])
                + alphas[14].mul_base(sibling[2])
                + alphas[15].mul_base(sibling[3])
        } else {
            let sibling = &main_trace.chiplet_hasher_state(row)[DIGEST_RANGE];
            alphas[0]
                + alphas[3].mul_base(index)
                + alphas[8].mul_base(sibling[0])
                + alphas[9].mul_base(sibling[1])
                + alphas[10].mul_base(sibling[2])
                + alphas[11].mul_base(sibling[3])
        }
    } else if f_mua {
        let index = main_trace.chiplet_node_index(row);
        let lsb = index.as_int() & 1;
        if lsb == 0 {
            let sibling = &main_trace.chiplet_hasher_state(row + 1)[DIGEST_RANGE.end..];
            alphas[0]
                + alphas[3].mul_base(index)
                + alphas[12].mul_base(sibling[0])
                + alphas[13].mul_base(sibling[1])
                + alphas[14].mul_base(sibling[2])
                + alphas[15].mul_base(sibling[3])
        } else {
            let sibling = &main_trace.chiplet_hasher_state(row + 1)[DIGEST_RANGE];
            alphas[0]
                + alphas[3].mul_base(index)
                + alphas[8].mul_base(sibling[0])
                + alphas[9].mul_base(sibling[1])
                + alphas[10].mul_base(sibling[2])
                + alphas[11].mul_base(sibling[3])
        }
    } else {
        E::ONE
    }
}

// VIRTUAL TABLE RESPONSES
// ================================================================================================

/// Constructs the inclusions to the table when the hasher absorbs a new sibling node while
/// computing the old Merkle root.
fn chiplets_vtable_add_sibling<E>(main_trace: &MainTrace, alphas: &[E], row: RowIndex) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let f_mv: bool = main_trace.f_mv(row);
    let f_mva: bool = main_trace.f_mva(row);

    if f_mv {
        let index = main_trace.chiplet_node_index(row);
        let lsb = index.as_int() & 1;
        if lsb == 0 {
            let sibling = &main_trace.chiplet_hasher_state(row)[DIGEST_RANGE.end..];
            alphas[0]
                + alphas[3].mul_base(index)
                + alphas[12].mul_base(sibling[0])
                + alphas[13].mul_base(sibling[1])
                + alphas[14].mul_base(sibling[2])
                + alphas[15].mul_base(sibling[3])
        } else {
            let sibling = &main_trace.chiplet_hasher_state(row)[DIGEST_RANGE];
            alphas[0]
                + alphas[3].mul_base(index)
                + alphas[8].mul_base(sibling[0])
                + alphas[9].mul_base(sibling[1])
                + alphas[10].mul_base(sibling[2])
                + alphas[11].mul_base(sibling[3])
        }
    } else if f_mva {
        let index = main_trace.chiplet_node_index(row);
        let lsb = index.as_int() & 1;
        if lsb == 0 {
            let sibling = &main_trace.chiplet_hasher_state(row + 1)[DIGEST_RANGE.end..];
            alphas[0]
                + alphas[3].mul_base(index)
                + alphas[12].mul_base(sibling[0])
                + alphas[13].mul_base(sibling[1])
                + alphas[14].mul_base(sibling[2])
                + alphas[15].mul_base(sibling[3])
        } else {
            let sibling = &main_trace.chiplet_hasher_state(row + 1)[DIGEST_RANGE];
            alphas[0]
                + alphas[3].mul_base(index)
                + alphas[8].mul_base(sibling[0])
                + alphas[9].mul_base(sibling[1])
                + alphas[10].mul_base(sibling[2])
                + alphas[11].mul_base(sibling[3])
        }
    } else {
        E::ONE
    }
}

/// Builds the inclusions to the kernel procedure table at `row`.
fn build_kernel_procedure_table_inclusions<E>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    if main_trace.is_kernel_row(row) {
        let idx = main_trace.chiplet_kernel_idx(row);
        let idx_delta = {
            let idx_next = main_trace.chiplet_kernel_idx(row + 1);
            idx_next - idx
        };
        let next_row_is_kernel = main_trace.is_kernel_row(row + 1);

        // We want to add an entry to the table in 2 cases:
        // 1. when the next row is a kernel row and the idx changes
        //    - this adds the last row of all rows that share the same idx
        // 2. when the next row is not a kernel row
        //    - this is the edge case of (1)
        if !next_row_is_kernel || idx_delta == ONE {
            let root0 = main_trace.chiplet_kernel_root_0(row);
            let root1 = main_trace.chiplet_kernel_root_1(row);
            let root2 = main_trace.chiplet_kernel_root_2(row);
            let root3 = main_trace.chiplet_kernel_root_3(row);

            alphas[0]
                + alphas[1].mul_base(idx)
                + alphas[2].mul_base(root0)
                + alphas[3].mul_base(root1)
                + alphas[4].mul_base(root2)
                + alphas[5].mul_base(root3)
        } else {
            E::ONE
        }
    } else {
        E::ONE
    }
}

// BUS COLUMN BUILDER
// ================================================================================================

/// Describes how to construct the execution trace of the chiplets bus auxiliary trace column.
#[derive(Default)]
pub struct BusColumnBuilder {}

impl<E: FieldElement<BaseField = Felt>> AuxColumnBuilder<E> for BusColumnBuilder {
    /// Constructs the requests made by the VM-components to the chiplets at `row`.
    fn get_requests_at(&self, main_trace: &MainTrace, alphas: &[E], row: RowIndex) -> E
    where
        E: FieldElement<BaseField = Felt>,
    {
        let op_code_felt = main_trace.get_op_code(row);
        let op_code = op_code_felt.as_int() as u8;

        match op_code {
            OPCODE_JOIN | OPCODE_SPLIT | OPCODE_LOOP | OPCODE_CALL => build_control_block_request(
                main_trace,
                main_trace.decoder_hasher_state(row),
                op_code_felt,
                alphas,
                row,
            ),
            OPCODE_DYN | OPCODE_DYNCALL => {
                build_dyn_block_request(main_trace, op_code_felt, alphas, row)
            },
            OPCODE_SYSCALL => build_syscall_block_request(main_trace, op_code_felt, alphas, row),
            OPCODE_SPAN => build_span_block_request(main_trace, alphas, row),
            OPCODE_RESPAN => build_respan_block_request(main_trace, alphas, row),
            OPCODE_END => build_end_block_request(main_trace, alphas, row),
            OPCODE_U32AND => build_bitwise_request(main_trace, ZERO, alphas, row),
            OPCODE_U32XOR => build_bitwise_request(main_trace, ONE, alphas, row),
            OPCODE_MLOADW => {
                build_mem_mloadw_mstorew_request(main_trace, MEMORY_READ_WORD_LABEL, alphas, row)
            },
            OPCODE_MSTOREW => {
                build_mem_mloadw_mstorew_request(main_trace, MEMORY_WRITE_WORD_LABEL, alphas, row)
            },
            OPCODE_MLOAD => {
                build_mem_mload_mstore_request(main_trace, MEMORY_READ_ELEMENT_LABEL, alphas, row)
            },
            OPCODE_MSTORE => {
                build_mem_mload_mstore_request(main_trace, MEMORY_WRITE_ELEMENT_LABEL, alphas, row)
            },
            OPCODE_MSTREAM => build_mstream_request(main_trace, alphas, row),
            OPCODE_HORNERBASE => build_horner_eval_request(main_trace, alphas, row),
            OPCODE_HORNEREXT => build_horner_eval_request(main_trace, alphas, row),
            OPCODE_HPERM => build_hperm_request(main_trace, alphas, row),
            OPCODE_MPVERIFY => build_mpverify_request(main_trace, alphas, row),
            OPCODE_MRUPDATE => build_mrupdate_request(main_trace, alphas, row),
            OPCODE_PIPE => build_pipe_request(main_trace, alphas, row),
            _ => E::ONE,
        }
    }

    /// Constructs the responses from the chiplets to the other VM-components at `row`.
    fn get_responses_at(&self, main_trace: &MainTrace, alphas: &[E], row: RowIndex) -> E
    where
        E: FieldElement<BaseField = Felt>,
    {
        if main_trace.is_hash_row(row) {
            build_hasher_chiplet_responses(main_trace, row, alphas)
        } else if main_trace.is_bitwise_row(row) {
            build_bitwise_chiplet_responses(main_trace, row, alphas)
        } else if main_trace.is_memory_row(row) {
            build_memory_chiplet_responses(main_trace, row, alphas)
        } else if main_trace.is_kernel_row(row) {
            build_kernel_chiplet_responses(main_trace, row, alphas)
        } else {
            E::ONE
        }
    }
}

// CHIPLETS REQUESTS
// ================================================================================================

/// Builds requests made to the hasher chiplet at the start of a control block.
fn build_control_block_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    decoder_hasher_state: [Felt; 8],
    op_code_felt: Felt,
    alphas: &[E],
    row: RowIndex,
) -> E {
    let op_label = LINEAR_HASH_LABEL;
    let addr_nxt = main_trace.addr(row + 1);
    let transition_label = op_label + 16;

    let header =
        alphas[0] + alphas[1].mul_base(Felt::from(transition_label)) + alphas[2].mul_base(addr_nxt);

    header + build_value(&alphas[8..16], decoder_hasher_state) + alphas[5].mul_base(op_code_felt)
}

/// Builds requests made on a `DYN` or `DYNCALL` operation.
fn build_dyn_block_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    op_code_felt: Felt,
    alphas: &[E],
    row: RowIndex,
) -> E {
    let control_block_req =
        build_control_block_request(main_trace, [ZERO; 8], op_code_felt, alphas, row);

    let memory_req = {
        let mem_addr = main_trace.stack_element(0, row);
        let mem_value = main_trace.decoder_hasher_state_first_half(row);

        compute_mem_request_word(
            main_trace,
            MEMORY_READ_WORD_LABEL,
            alphas,
            row,
            mem_addr,
            mem_value,
        )
    };

    control_block_req * memory_req
}

/// Builds requests made to kernel ROM chiplet when initializing a syscall block.
fn build_syscall_block_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    op_code_felt: Felt,
    alphas: &[E],
    row: RowIndex,
) -> E {
    let factor1 = build_control_block_request(
        main_trace,
        main_trace.decoder_hasher_state(row),
        op_code_felt,
        alphas,
        row,
    );

    let op_label = KERNEL_PROC_LABEL;
    let state = main_trace.decoder_hasher_state(row);
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
    row: RowIndex,
) -> E {
    let op_label = LINEAR_HASH_LABEL;
    let addr_nxt = main_trace.addr(row + 1);
    let transition_label = op_label + 16;

    let header =
        alphas[0] + alphas[1].mul_base(Felt::from(transition_label)) + alphas[2].mul_base(addr_nxt);

    let state = main_trace.decoder_hasher_state(row);
    header + build_value(&alphas[8..16], state)
}

/// Builds requests made to the hasher chiplet at the start of a respan block.
fn build_respan_block_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
) -> E {
    let op_label = LINEAR_HASH_LABEL;
    let addr_nxt = main_trace.addr(row + 1);
    let transition_label = op_label + 32;

    let header = alphas[0]
        + alphas[1].mul_base(Felt::from(transition_label))
        + alphas[2].mul_base(addr_nxt - ONE)
        + alphas[3].mul_base(ZERO);

    let state = main_trace.decoder_hasher_state(row);

    header + build_value(&alphas[8..16], state)
}

/// Builds requests made to the hasher chiplet at the end of a block.
fn build_end_block_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
) -> E {
    let op_label = RETURN_HASH_LABEL;
    let addr = main_trace.addr(row) + Felt::from(NUM_ROUNDS as u8);
    let transition_label = op_label + 32;

    let header =
        alphas[0] + alphas[1].mul_base(Felt::from(transition_label)) + alphas[2].mul_base(addr);

    let state = main_trace.decoder_hasher_state(row);
    let digest: [Felt; 4] = state[..4].try_into().unwrap();

    header + build_value(&alphas[8..12], digest)
}

/// Builds requests made to the bitwise chiplet. This can be either a request for the computation
/// of a `XOR` or an `AND` operation.
fn build_bitwise_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    is_xor: Felt,
    alphas: &[E],
    row: RowIndex,
) -> E {
    let op_label = get_op_label(ONE, ZERO, is_xor, ZERO);
    let a = main_trace.stack_element(1, row);
    let b = main_trace.stack_element(0, row);
    let z = main_trace.stack_element(0, row + 1);

    alphas[0] + build_value(&alphas[1..5], [op_label, a, b, z])
}

/// Builds `MSTREAM` requests made to the memory chiplet.
fn build_mstream_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
) -> E {
    let word1 = [
        main_trace.stack_element(7, row + 1),
        main_trace.stack_element(6, row + 1),
        main_trace.stack_element(5, row + 1),
        main_trace.stack_element(4, row + 1),
    ];
    let word2 = [
        main_trace.stack_element(3, row + 1),
        main_trace.stack_element(2, row + 1),
        main_trace.stack_element(1, row + 1),
        main_trace.stack_element(0, row + 1),
    ];
    let addr = main_trace.stack_element(12, row);
    let op_label = MEMORY_READ_WORD_LABEL;

    let factor1 = compute_mem_request_word(main_trace, op_label, alphas, row, addr, word1);
    let factor2 = compute_mem_request_word(main_trace, op_label, alphas, row, addr + FOUR, word2);

    factor1 * factor2
}

/// Builds `PIPE` requests made to the memory chiplet.
fn build_pipe_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
) -> E {
    let word1 = [
        main_trace.stack_element(7, row + 1),
        main_trace.stack_element(6, row + 1),
        main_trace.stack_element(5, row + 1),
        main_trace.stack_element(4, row + 1),
    ];
    let word2 = [
        main_trace.stack_element(3, row + 1),
        main_trace.stack_element(2, row + 1),
        main_trace.stack_element(1, row + 1),
        main_trace.stack_element(0, row + 1),
    ];
    let addr = main_trace.stack_element(12, row);
    let op_label = MEMORY_WRITE_WORD_LABEL;

    let req1 = compute_mem_request_word(main_trace, op_label, alphas, row, addr, word1);
    let req2 = compute_mem_request_word(main_trace, op_label, alphas, row, addr + FOUR, word2);

    req1 * req2
}

/// Builds `HORNERBASE` or `HORNEREXT` requests made to the memory chiplet.
fn build_horner_eval_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
) -> E {
    let eval_point_0 = main_trace.helper_register(0, row);
    let eval_point_1 = main_trace.helper_register(1, row);
    let eval_point_ptr = main_trace.stack_element(13, row);
    let op_label = MEMORY_READ_WORD_LABEL;

    compute_mem_request_word(
        main_trace,
        op_label,
        alphas,
        row,
        eval_point_ptr,
        [eval_point_0, eval_point_1, ZERO, ZERO],
    )
}

/// Builds `HPERM` requests made to the hash chiplet.
fn build_hperm_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
) -> E {
    let helper_0 = main_trace.helper_register(0, row);

    let s0_s12_cur = [
        main_trace.stack_element(0, row),
        main_trace.stack_element(1, row),
        main_trace.stack_element(2, row),
        main_trace.stack_element(3, row),
        main_trace.stack_element(4, row),
        main_trace.stack_element(5, row),
        main_trace.stack_element(6, row),
        main_trace.stack_element(7, row),
        main_trace.stack_element(8, row),
        main_trace.stack_element(9, row),
        main_trace.stack_element(10, row),
        main_trace.stack_element(11, row),
    ];

    let s0_s12_nxt = [
        main_trace.stack_element(0, row + 1),
        main_trace.stack_element(1, row + 1),
        main_trace.stack_element(2, row + 1),
        main_trace.stack_element(3, row + 1),
        main_trace.stack_element(4, row + 1),
        main_trace.stack_element(5, row + 1),
        main_trace.stack_element(6, row + 1),
        main_trace.stack_element(7, row + 1),
        main_trace.stack_element(8, row + 1),
        main_trace.stack_element(9, row + 1),
        main_trace.stack_element(10, row + 1),
        main_trace.stack_element(11, row + 1),
    ];

    let op_label = LINEAR_HASH_LABEL + 16;

    let sum_input = alphas[4..16]
        .iter()
        .rev()
        .enumerate()
        .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(s0_s12_cur[i]));
    let v_input = alphas[0]
        + alphas[1].mul_base(Felt::from(op_label))
        + alphas[2].mul_base(helper_0)
        + sum_input;

    let op_label = RETURN_STATE_LABEL + 32;

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
    row: RowIndex,
) -> E {
    let helper_0 = main_trace.helper_register(0, row);

    let s0_s3 = [
        main_trace.stack_element(0, row),
        main_trace.stack_element(1, row),
        main_trace.stack_element(2, row),
        main_trace.stack_element(3, row),
    ];
    let s4 = main_trace.stack_element(4, row);
    let s5 = main_trace.stack_element(5, row);
    let s6_s9 = [
        main_trace.stack_element(6, row),
        main_trace.stack_element(7, row),
        main_trace.stack_element(8, row),
        main_trace.stack_element(9, row),
    ];

    let op_label = MP_VERIFY_LABEL + 16;

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

    let op_label = RETURN_HASH_LABEL + 32;

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
    row: RowIndex,
) -> E {
    let helper_0 = main_trace.helper_register(0, row);

    let s0_s3 = [
        main_trace.stack_element(0, row),
        main_trace.stack_element(1, row),
        main_trace.stack_element(2, row),
        main_trace.stack_element(3, row),
    ];
    let s0_s3_nxt = [
        main_trace.stack_element(0, row + 1),
        main_trace.stack_element(1, row + 1),
        main_trace.stack_element(2, row + 1),
        main_trace.stack_element(3, row + 1),
    ];
    let s4 = main_trace.stack_element(4, row);
    let s5 = main_trace.stack_element(5, row);
    let s6_s9 = [
        main_trace.stack_element(6, row),
        main_trace.stack_element(7, row),
        main_trace.stack_element(8, row),
        main_trace.stack_element(9, row),
    ];
    let s10_s13 = [
        main_trace.stack_element(10, row),
        main_trace.stack_element(11, row),
        main_trace.stack_element(12, row),
        main_trace.stack_element(13, row),
    ];

    let op_label = MR_UPDATE_OLD_LABEL + 16;

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

    let op_label = RETURN_HASH_LABEL + 32;

    let sum_output = alphas[8..12]
        .iter()
        .rev()
        .enumerate()
        .fold(E::ZERO, |acc, (i, x)| acc + x.mul_base(s6_s9[i]));
    let v_output_old = alphas[0]
        + alphas[1].mul_base(Felt::from(op_label))
        + alphas[2].mul_base(helper_0 + s4.mul_small(8) - ONE)
        + sum_output;

    let op_label = MR_UPDATE_NEW_LABEL + 16;
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

    let op_label = RETURN_HASH_LABEL + 32;

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

/// Builds the response from the hasher chiplet at `row`.
fn build_hasher_chiplet_responses<E>(main_trace: &MainTrace, row: RowIndex, alphas: &[E]) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let mut multiplicand = E::ONE;
    let selector0 = main_trace.chiplet_selector_0(row);
    let selector1 = main_trace.chiplet_selector_1(row);
    let selector2 = main_trace.chiplet_selector_2(row);
    let selector3 = main_trace.chiplet_selector_3(row);
    let op_label = get_op_label(selector0, selector1, selector2, selector3);

    // f_bp, f_mp, f_mv or f_mu == 1
    if row.as_usize() % HASH_CYCLE_LEN == 0 {
        let state = main_trace.chiplet_hasher_state(row);
        let alphas_state = &alphas[NUM_HEADER_ALPHAS..(NUM_HEADER_ALPHAS + STATE_WIDTH)];
        let node_index = main_trace.chiplet_node_index(row);
        let transition_label = op_label + Felt::from(16_u8);

        // f_bp == 1
        // v_all = v_h + v_a + v_b + v_c
        if selector1 == ONE && selector2 == ZERO && selector3 == ZERO {
            let header = alphas[0]
                + build_value(&alphas[1..4], [transition_label, Felt::from(row + 1), node_index]);

            multiplicand = header + build_value(alphas_state, state);
        }

        // f_mp or f_mv or f_mu == 1
        // v_leaf = v_h + (1 - b) * v_b + b * v_d
        if selector1 == ONE && !(selector2 == ZERO && selector3 == ZERO) {
            let header = alphas[0]
                + build_value(&alphas[1..4], [transition_label, Felt::from(row + 1), node_index]);

            let bit = (node_index.as_int() & 1) as u8;
            let left_word = build_value::<_, 4>(
                &alphas_state[DIGEST_RANGE],
                state[DIGEST_RANGE].try_into().unwrap(),
            );
            let right_word = build_value::<_, 4>(
                &alphas_state[DIGEST_RANGE],
                state[DIGEST_RANGE.end..].try_into().unwrap(),
            );

            multiplicand = header + E::from(1 - bit).mul(left_word) + E::from(bit).mul(right_word);
        }
    }

    // f_hout, f_sout, f_abp == 1
    if row.as_usize() % HASH_CYCLE_LEN == HASH_CYCLE_LEN - 1 {
        let state = main_trace.chiplet_hasher_state(row);
        let alphas_state = &alphas[NUM_HEADER_ALPHAS..(NUM_HEADER_ALPHAS + STATE_WIDTH)];
        let node_index = main_trace.chiplet_node_index(row);
        let transition_label = op_label + Felt::from(32_u8);

        // f_hout == 1
        // v_res = v_h + v_b;
        if selector1 == ZERO && selector2 == ZERO && selector3 == ZERO {
            let header = alphas[0]
                + build_value(&alphas[1..4], [transition_label, Felt::from(row + 1), node_index]);

            multiplicand = header
                + build_value::<_, 4>(
                    &alphas_state[DIGEST_RANGE],
                    state[DIGEST_RANGE].try_into().unwrap(),
                );
        }

        // f_sout == 1
        // v_all = v_h + v_a + v_b + v_c
        if selector1 == ZERO && selector2 == ZERO && selector3 == ONE {
            let header = alphas[0]
                + build_value(&alphas[1..4], [transition_label, Felt::from(row + 1), node_index]);

            multiplicand = header + build_value(alphas_state, state);
        }

        // f_abp == 1
        // v_abp = v_h + v_b' + v_c' - v_b - v_c
        if selector1 == ONE && selector2 == ZERO && selector3 == ZERO {
            let header = alphas[0]
                + build_value(&alphas[1..4], [transition_label, Felt::from(row + 1), node_index]);

            let state_nxt = main_trace.chiplet_hasher_state(row + 1);

            // build the value from the hasher state's just right after the absorption of new
            // elements.
            const SIZE: usize = STATE_WIDTH - CAPACITY_LEN;
            let next_state_value = build_value::<_, SIZE>(
                &alphas_state[CAPACITY_LEN..],
                state_nxt[CAPACITY_LEN..].try_into().unwrap(),
            );

            multiplicand = header + next_state_value;
        }
    }
    multiplicand
}

/// Builds the response from the bitwise chiplet at `row`.
fn build_bitwise_chiplet_responses<E>(main_trace: &MainTrace, row: RowIndex, alphas: &[E]) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let is_xor = main_trace.chiplet_selector_2(row);
    if row.as_usize() % BITWISE_OP_CYCLE_LEN == BITWISE_OP_CYCLE_LEN - 1 {
        let op_label = get_op_label(ONE, ZERO, is_xor, ZERO);

        let a = main_trace.chiplet_bitwise_a(row);
        let b = main_trace.chiplet_bitwise_b(row);
        let z = main_trace.chiplet_bitwise_z(row);

        alphas[0] + build_value(&alphas[1..5], [op_label, a, b, z])
    } else {
        E::ONE
    }
}

/// Builds the response from the memory chiplet at `row`.
fn build_memory_chiplet_responses<E>(main_trace: &MainTrace, row: RowIndex, alphas: &[E]) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let is_word_access = main_trace.chiplet_selector_4(row);
    let header = {
        let is_read = main_trace.chiplet_selector_3(row);
        let op_label = get_memory_op_label(is_read, is_word_access);

        let ctx = main_trace.chiplet_memory_ctx(row);
        let clk = main_trace.chiplet_memory_clk(row);
        let address = {
            let word = main_trace.chiplet_memory_word(row);
            let idx0 = main_trace.chiplet_memory_idx0(row);
            let idx1 = main_trace.chiplet_memory_idx1(row);

            word + idx1.mul_small(2) + idx0
        };

        alphas[0] + build_value(&alphas[1..5], [op_label, ctx, address, clk])
    };

    if is_word_access == MEMORY_ACCESS_ELEMENT {
        let idx0 = main_trace.chiplet_memory_idx0(row);
        let idx1 = main_trace.chiplet_memory_idx1(row);

        let value = if idx1 == ZERO && idx0 == ZERO {
            main_trace.chiplet_memory_value_0(row)
        } else if idx1 == ZERO && idx0 == ONE {
            main_trace.chiplet_memory_value_1(row)
        } else if idx1 == ONE && idx0 == ZERO {
            main_trace.chiplet_memory_value_2(row)
        } else if idx1 == ONE && idx0 == ONE {
            main_trace.chiplet_memory_value_3(row)
        } else {
            panic!("Invalid word indices. idx0: {idx0}, idx1: {idx1}");
        };

        header + alphas[5].mul_base(value)
    } else if is_word_access == MEMORY_ACCESS_WORD {
        let value0 = main_trace.chiplet_memory_value_0(row);
        let value1 = main_trace.chiplet_memory_value_1(row);
        let value2 = main_trace.chiplet_memory_value_2(row);
        let value3 = main_trace.chiplet_memory_value_3(row);

        header + build_value(&alphas[5..9], [value0, value1, value2, value3])
    } else {
        panic!("Invalid memory element/word column value: {is_word_access}");
    }
}

/// Builds the response from the kernel chiplet at `row`.
fn build_kernel_chiplet_responses<E>(main_trace: &MainTrace, row: RowIndex, alphas: &[E]) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let op_label = KERNEL_PROC_LABEL;

    let root0 = main_trace.chiplet_kernel_root_0(row);
    let root1 = main_trace.chiplet_kernel_root_1(row);
    let root2 = main_trace.chiplet_kernel_root_2(row);
    let root3 = main_trace.chiplet_kernel_root_3(row);

    let v = alphas[0] + build_value(&alphas[1..6], [op_label, root0, root1, root2, root3]);

    let kernel_chiplet_selector = main_trace.chiplet_selector_4(row);
    v.mul_base(kernel_chiplet_selector) + E::from(ONE - kernel_chiplet_selector)
}

// HELPER FUNCTIONS
// ================================================================================================

/// Runs an inner product between the alphas and the elements.
#[inline(always)]
fn build_value<E: FieldElement<BaseField = Felt>, const N: usize>(
    alphas: &[E],
    elements: [Felt; N],
) -> E {
    debug_assert_eq!(alphas.len(), elements.len());
    let mut value = E::ZERO;
    for i in 0..N {
        value += alphas[i].mul_base(elements[i]);
    }
    value
}

/// Returns the operation unique label.
fn get_op_label(s0: Felt, s1: Felt, s2: Felt, s3: Felt) -> Felt {
    s3.mul_small(1 << 3) + s2.mul_small(1 << 2) + s1.mul_small(2) + s0 + ONE
}

/// Returns the operation unique label for memory operations.
///
/// The memory operation label is currently the only label that is built differently (or *simpler*)
/// from the other chiplets. We should refactor the other chiplets to use a similar (simpler)
/// approach.
fn get_memory_op_label(is_read: Felt, is_word_access: Felt) -> Felt {
    const MEMORY_SELECTOR: u8 = 0b110;
    // Equivalent to `is_read << 1`
    let is_read_left_shift_1 = is_read + is_read;

    Felt::from(MEMORY_SELECTOR << 2) + is_read_left_shift_1 + is_word_access
}

/// Builds `MLOADW` and `MSTOREW` requests made to the memory chiplet.
fn build_mem_mloadw_mstorew_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    op_label: u8,
    alphas: &[E],
    row: RowIndex,
) -> E {
    let word = [
        main_trace.stack_element(3, row + 1),
        main_trace.stack_element(2, row + 1),
        main_trace.stack_element(1, row + 1),
        main_trace.stack_element(0, row + 1),
    ];
    let addr = main_trace.stack_element(0, row);

    compute_mem_request_word(main_trace, op_label, alphas, row, addr, word)
}

/// Builds `MLOAD` and `MSTORE` requests made to the memory chiplet.
fn build_mem_mload_mstore_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    op_label: u8,
    alphas: &[E],
    row: RowIndex,
) -> E {
    let element = main_trace.stack_element(0, row + 1);
    let addr = main_trace.stack_element(0, row);

    compute_mem_request_element(main_trace, op_label, alphas, row, addr, element)
}

/// Computes a memory request for a read or write of a single element.
fn compute_mem_request_element<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    op_label: u8,
    alphas: &[E],
    row: RowIndex,
    addr: Felt,
    element: Felt,
) -> E {
    debug_assert!(op_label == MEMORY_READ_ELEMENT_LABEL || op_label == MEMORY_WRITE_ELEMENT_LABEL);

    let ctx = main_trace.ctx(row);
    let clk = main_trace.clk(row);

    alphas[0] + build_value(&alphas[1..6], [Felt::from(op_label), ctx, addr, clk, element])
}

/// Computes a memory request for a read or write of a word.
fn compute_mem_request_word<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    op_label: u8,
    alphas: &[E],
    row: RowIndex,
    addr: Felt,
    word: Word,
) -> E {
    debug_assert!(op_label == MEMORY_READ_WORD_LABEL || op_label == MEMORY_WRITE_WORD_LABEL);
    let ctx = main_trace.ctx(row);
    let clk = main_trace.clk(row);

    alphas[0]
        + build_value(
            &alphas[1..9],
            [Felt::from(op_label), ctx, addr, clk, word[0], word[1], word[2], word[3]],
        )
}
