use alloc::{boxed::Box, vec::Vec};

use messages::{
    BitwiseMessage, ControlBlockRequestMessage, EndBlockMessage, HasherMessage, KernelRomMessage,
    MemRequestElementMessage, MemRequestWordMessage, RespanBlockMessage, SpanBlockMessage,
};
use miden_air::{
    trace::{
        chiplets::{
            bitwise::OP_CYCLE_LEN as BITWISE_OP_CYCLE_LEN,
            hasher::{
                DIGEST_RANGE, HASH_CYCLE_LEN, LINEAR_HASH_LABEL, MP_VERIFY_LABEL,
                MR_UPDATE_NEW_LABEL, MR_UPDATE_OLD_LABEL, NUM_ROUNDS, RETURN_HASH_LABEL,
                RETURN_STATE_LABEL,
            },
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
    Kernel, ONE, OPCODE_CALL, OPCODE_DYN, OPCODE_DYNCALL, OPCODE_END, OPCODE_HPERM, OPCODE_JOIN,
    OPCODE_LOOP, OPCODE_MLOAD, OPCODE_MLOADW, OPCODE_MPVERIFY, OPCODE_MRUPDATE, OPCODE_MSTORE,
    OPCODE_MSTOREW, OPCODE_MSTREAM, OPCODE_PIPE, OPCODE_RCOMBBASE, OPCODE_RESPAN, OPCODE_SPAN,
    OPCODE_SPLIT, OPCODE_SYSCALL, OPCODE_U32AND, OPCODE_U32XOR, ZERO,
};

use super::{super::trace::AuxColumnBuilder, Felt, FieldElement};
use crate::debug::{BusDebugger, BusMessage};

mod messages;

// CONSTANTS
// ================================================================================================

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
        debug_assert_eq!(*b_chip.last().unwrap(), E::ONE);
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
    fn init_requests(
        &self,
        _main_trace: &MainTrace,
        alphas: &[E],
        _debugger: &mut BusDebugger<E>,
    ) -> E {
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

    fn get_requests_at(
        &self,
        main_trace: &MainTrace,
        alphas: &[E],
        row: RowIndex,
        _debugger: &mut BusDebugger<E>,
    ) -> E {
        chiplets_vtable_remove_sibling(main_trace, alphas, row)
    }

    fn get_responses_at(
        &self,
        main_trace: &MainTrace,
        alphas: &[E],
        row: RowIndex,
        _debugger: &mut BusDebugger<E>,
    ) -> E {
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
    fn get_requests_at(
        &self,
        main_trace: &MainTrace,
        alphas: &[E],
        row: RowIndex,
        debugger: &mut BusDebugger<E>,
    ) -> E
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
                debugger,
            ),
            OPCODE_DYN | OPCODE_DYNCALL => {
                build_dyn_block_request(main_trace, op_code_felt, alphas, row, debugger)
            },
            OPCODE_SYSCALL => {
                build_syscall_block_request(main_trace, op_code_felt, alphas, row, debugger)
            },
            OPCODE_SPAN => build_span_block_request(main_trace, alphas, row, debugger),
            OPCODE_RESPAN => build_respan_block_request(main_trace, alphas, row, debugger),
            OPCODE_END => build_end_block_request(main_trace, alphas, row, debugger),
            OPCODE_U32AND => build_bitwise_request(main_trace, ZERO, alphas, row, debugger),
            OPCODE_U32XOR => build_bitwise_request(main_trace, ONE, alphas, row, debugger),
            OPCODE_MLOADW => build_mem_mloadw_mstorew_request(
                main_trace,
                MEMORY_READ_WORD_LABEL,
                alphas,
                row,
                debugger,
            ),
            OPCODE_MSTOREW => build_mem_mloadw_mstorew_request(
                main_trace,
                MEMORY_WRITE_WORD_LABEL,
                alphas,
                row,
                debugger,
            ),
            OPCODE_MLOAD => build_mem_mload_mstore_request(
                main_trace,
                MEMORY_READ_ELEMENT_LABEL,
                alphas,
                row,
                debugger,
            ),
            OPCODE_MSTORE => build_mem_mload_mstore_request(
                main_trace,
                MEMORY_WRITE_ELEMENT_LABEL,
                alphas,
                row,
                debugger,
            ),
            OPCODE_MSTREAM => build_mstream_request(main_trace, alphas, row, debugger),
            OPCODE_RCOMBBASE => build_rcomb_base_request(main_trace, alphas, row, debugger),
            OPCODE_HPERM => build_hperm_request(main_trace, alphas, row, debugger),
            OPCODE_MPVERIFY => build_mpverify_request(main_trace, alphas, row, debugger),
            OPCODE_MRUPDATE => build_mrupdate_request(main_trace, alphas, row, debugger),
            OPCODE_PIPE => build_pipe_request(main_trace, alphas, row, debugger),
            _ => E::ONE,
        }
    }

    // TODO(plafer): `debugger` field only in test mode
    /// Constructs the responses from the chiplets to the other VM-components at `row`.
    fn get_responses_at(
        &self,
        main_trace: &MainTrace,
        alphas: &[E],
        row: RowIndex,
        debugger: &mut BusDebugger<E>,
    ) -> E
    where
        E: FieldElement<BaseField = Felt>,
    {
        if main_trace.is_hash_row(row) {
            build_hasher_chiplet_responses(main_trace, row, alphas, debugger)
        } else if main_trace.is_bitwise_row(row) {
            build_bitwise_chiplet_responses(main_trace, row, alphas, debugger)
        } else if main_trace.is_memory_row(row) {
            build_memory_chiplet_responses(main_trace, row, alphas, debugger)
        } else if main_trace.is_kernel_row(row) {
            build_kernel_chiplet_responses(main_trace, row, alphas, debugger)
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
    debugger: &mut BusDebugger<E>,
) -> E {
    let message = ControlBlockRequestMessage {
        transition_label: Felt::from(LINEAR_HASH_LABEL + 16),
        addr_next: main_trace.addr(row + 1),
        op_code: op_code_felt,
        decoder_hasher_state,
    };

    let value = message.value(alphas);

    #[cfg(any(test, feature = "testing"))]
    debugger.add_request(Box::new(message), alphas);

    value
}

/// Builds requests made on a `DYN` or `DYNCALL` operation.
fn build_dyn_block_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    op_code_felt: Felt,
    alphas: &[E],
    row: RowIndex,
    debugger: &mut BusDebugger<E>,
) -> E {
    let control_block_req = ControlBlockRequestMessage {
        transition_label: Felt::from(LINEAR_HASH_LABEL + 16),
        addr_next: main_trace.addr(row + 1),
        op_code: op_code_felt,
        decoder_hasher_state: [ZERO; 8],
    };

    let memory_req = MemRequestWordMessage {
        op_label: Felt::from(MEMORY_READ_WORD_LABEL),
        ctx: main_trace.ctx(row),
        addr: main_trace.stack_element(0, row),
        clk: main_trace.clk(row),
        word: main_trace.decoder_hasher_state_first_half(row),
        source: if op_code_felt == OPCODE_DYNCALL.into() {
            "dyncall"
        } else {
            "dyn"
        },
    };

    let combined_value = control_block_req.value(alphas) * memory_req.value(alphas);
    #[cfg(any(test, feature = "testing"))]
    {
        debugger.add_request(Box::new(control_block_req), alphas);
        debugger.add_request(Box::new(memory_req), alphas);
    }

    combined_value
}

/// Builds requests made to kernel ROM chiplet when initializing a syscall block.
fn build_syscall_block_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    op_code_felt: Felt,
    alphas: &[E],
    row: RowIndex,
    debugger: &mut BusDebugger<E>,
) -> E {
    let control_block_req = ControlBlockRequestMessage {
        transition_label: Felt::from(LINEAR_HASH_LABEL + 16),
        addr_next: main_trace.addr(row + 1),
        op_code: op_code_felt,
        decoder_hasher_state: main_trace.decoder_hasher_state(row),
    };

    let kernel_rom_req = KernelRomMessage {
        kernel_proc_digest: main_trace.decoder_hasher_state(row)[0..4].try_into().unwrap(),
    };

    let combined_value = control_block_req.value(alphas) * kernel_rom_req.value(alphas);

    #[cfg(any(test, feature = "testing"))]
    {
        debugger.add_request(Box::new(control_block_req), alphas);
        debugger.add_request(Box::new(kernel_rom_req), alphas);
    }

    combined_value
}

/// Builds requests made to the hasher chiplet at the start of a span block.
fn build_span_block_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
    debugger: &mut BusDebugger<E>,
) -> E {
    let span_block_message = SpanBlockMessage {
        transition_label: Felt::from(LINEAR_HASH_LABEL + 16),
        addr_next: main_trace.addr(row + 1),
        state: main_trace.decoder_hasher_state(row),
    };

    let value = span_block_message.value(alphas);

    #[cfg(any(test, feature = "testing"))]
    debugger.add_request(Box::new(span_block_message), alphas);

    value
}

/// Builds requests made to the hasher chiplet at the start of a respan block.
fn build_respan_block_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
    debugger: &mut BusDebugger<E>,
) -> E {
    let respan_block_message = RespanBlockMessage {
        transition_label: Felt::from(LINEAR_HASH_LABEL + 32),
        addr_next: main_trace.addr(row + 1),
        state: main_trace.decoder_hasher_state(row),
    };

    let value = respan_block_message.value(alphas);

    #[cfg(any(test, feature = "testing"))]
    debugger.add_request(Box::new(respan_block_message), alphas);

    value
}

/// Builds requests made to the hasher chiplet at the end of a block.
fn build_end_block_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
    debugger: &mut BusDebugger<E>,
) -> E {
    let end_block_message = EndBlockMessage {
        addr: main_trace.addr(row) + Felt::from(NUM_ROUNDS as u8),
        transition_label: Felt::from(RETURN_HASH_LABEL + 32),
        digest: main_trace.decoder_hasher_state(row)[..4].try_into().unwrap(),
    };

    let value = end_block_message.value(alphas);

    #[cfg(any(test, feature = "testing"))]
    debugger.add_request(Box::new(end_block_message), alphas);

    value
}

/// Builds requests made to the bitwise chiplet. This can be either a request for the computation
/// of a `XOR` or an `AND` operation.
fn build_bitwise_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    is_xor: Felt,
    alphas: &[E],
    row: RowIndex,
    debugger: &mut BusDebugger<E>,
) -> E {
    let bitwise_request_message = BitwiseMessage {
        op_label: get_op_label(ONE, ZERO, is_xor, ZERO),
        a: main_trace.stack_element(1, row),
        b: main_trace.stack_element(0, row),
        z: main_trace.stack_element(0, row + 1),
    };

    let value = bitwise_request_message.value(alphas);

    #[cfg(any(test, feature = "testing"))]
    debugger.add_request(Box::new(bitwise_request_message), alphas);

    value
}

/// Builds `MSTREAM` requests made to the memory chiplet.
fn build_mstream_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
    debugger: &mut BusDebugger<E>,
) -> E {
    let op_label = Felt::from(MEMORY_READ_WORD_LABEL);
    let addr = main_trace.stack_element(12, row);
    let ctx = main_trace.ctx(row);
    let clk = main_trace.clk(row);

    let mem_req_1 = MemRequestWordMessage {
        op_label,
        ctx,
        addr,
        clk,
        word: [
            main_trace.stack_element(7, row + 1),
            main_trace.stack_element(6, row + 1),
            main_trace.stack_element(5, row + 1),
            main_trace.stack_element(4, row + 1),
        ],
        source: "mstream req 1",
    };
    let mem_req_2 = MemRequestWordMessage {
        op_label,
        ctx,
        addr: addr + FOUR,
        clk,
        word: [
            main_trace.stack_element(3, row + 1),
            main_trace.stack_element(2, row + 1),
            main_trace.stack_element(1, row + 1),
            main_trace.stack_element(0, row + 1),
        ],
        source: "mstream req 2",
    };

    let combined_value = mem_req_1.value(alphas) * mem_req_2.value(alphas);

    #[cfg(any(test, feature = "testing"))]
    {
        debugger.add_request(Box::new(mem_req_1), alphas);
        debugger.add_request(Box::new(mem_req_2), alphas);
    }

    combined_value
}

/// Builds `PIPE` requests made to the memory chiplet.
fn build_pipe_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
    debugger: &mut BusDebugger<E>,
) -> E {
    let op_label = Felt::from(MEMORY_WRITE_WORD_LABEL);
    let addr = main_trace.stack_element(12, row);
    let ctx = main_trace.ctx(row);
    let clk = main_trace.clk(row);

    let mem_req_1 = MemRequestWordMessage {
        op_label,
        ctx,
        addr,
        clk,
        word: [
            main_trace.stack_element(7, row + 1),
            main_trace.stack_element(6, row + 1),
            main_trace.stack_element(5, row + 1),
            main_trace.stack_element(4, row + 1),
        ],
        source: "pipe req 1",
    };
    let mem_req_2 = MemRequestWordMessage {
        op_label,
        ctx,
        addr: addr + FOUR,
        clk,
        word: [
            main_trace.stack_element(3, row + 1),
            main_trace.stack_element(2, row + 1),
            main_trace.stack_element(1, row + 1),
            main_trace.stack_element(0, row + 1),
        ],
        source: "pipe req 2",
    };

    let combined_value = mem_req_1.value(alphas) * mem_req_2.value(alphas);

    #[cfg(any(test, feature = "testing"))]
    {
        debugger.add_request(Box::new(mem_req_1), alphas);
        debugger.add_request(Box::new(mem_req_2), alphas);
    }

    combined_value
}

/// Builds `RCOMBBASE` requests made to the memory chiplet.
fn build_rcomb_base_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
    debugger: &mut BusDebugger<E>,
) -> E {
    let tz0 = main_trace.helper_register(0, row);
    let tz1 = main_trace.helper_register(1, row);
    let tzg0 = main_trace.helper_register(2, row);
    let tzg1 = main_trace.helper_register(3, row);
    let a0 = main_trace.helper_register(4, row);
    let a1 = main_trace.helper_register(5, row);
    let z_ptr = main_trace.stack_element(13, row);
    let a_ptr = main_trace.stack_element(14, row);
    let op_label = Felt::from(MEMORY_READ_WORD_LABEL);

    let ctx = main_trace.ctx(row);
    let clk = main_trace.clk(row);

    let mem_req_1 = MemRequestWordMessage {
        op_label,
        ctx,
        addr: z_ptr,
        clk,
        word: [tz0, tz1, tzg0, tzg1],
        source: "rcombbase req 1",
    };
    let mem_req_2 = MemRequestWordMessage {
        op_label,
        ctx,
        addr: a_ptr,
        clk,
        word: [a0, a1, ZERO, ZERO],
        source: "rcombbase req 2",
    };

    let combined_value = mem_req_1.value(alphas) * mem_req_2.value(alphas);

    #[cfg(any(test, feature = "testing"))]
    {
        debugger.add_request(Box::new(mem_req_1), alphas);
        debugger.add_request(Box::new(mem_req_2), alphas);
    }

    combined_value
}

/// Builds `HPERM` requests made to the hash chiplet.
fn build_hperm_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
    debugger: &mut BusDebugger<E>,
) -> E {
    let helper_0 = main_trace.helper_register(0, row);
    let s0 = main_trace.stack_element(0, row);
    let s1 = main_trace.stack_element(1, row);
    let s2 = main_trace.stack_element(2, row);
    let s3 = main_trace.stack_element(3, row);
    let s4 = main_trace.stack_element(4, row);
    let s5 = main_trace.stack_element(5, row);
    let s6 = main_trace.stack_element(6, row);
    let s7 = main_trace.stack_element(7, row);
    let s8 = main_trace.stack_element(8, row);
    let s9 = main_trace.stack_element(9, row);
    let s10 = main_trace.stack_element(10, row);
    let s11 = main_trace.stack_element(11, row);
    let s0_nxt = main_trace.stack_element(0, row + 1);
    let s1_nxt = main_trace.stack_element(1, row + 1);
    let s2_nxt = main_trace.stack_element(2, row + 1);
    let s3_nxt = main_trace.stack_element(3, row + 1);
    let s4_nxt = main_trace.stack_element(4, row + 1);
    let s5_nxt = main_trace.stack_element(5, row + 1);
    let s6_nxt = main_trace.stack_element(6, row + 1);
    let s7_nxt = main_trace.stack_element(7, row + 1);
    let s8_nxt = main_trace.stack_element(8, row + 1);
    let s9_nxt = main_trace.stack_element(9, row + 1);
    let s10_nxt = main_trace.stack_element(10, row + 1);
    let s11_nxt = main_trace.stack_element(11, row + 1);

    let input_req = HasherMessage {
        transition_label: Felt::from(LINEAR_HASH_LABEL + 16),
        addr_next: helper_0,
        node_index: ZERO,
        hasher_state: [s11, s10, s9, s8, s7, s6, s5, s4, s3, s2, s1, s0],
        source: "hperm input",
    };
    let output_req = HasherMessage {
        transition_label: Felt::from(RETURN_STATE_LABEL + 32),
        addr_next: helper_0 + Felt::new(7),
        node_index: ZERO,
        hasher_state: [
            s11_nxt, s10_nxt, s9_nxt, s8_nxt, s7_nxt, s6_nxt, s5_nxt, s4_nxt, s3_nxt, s2_nxt,
            s1_nxt, s0_nxt,
        ],
        source: "hperm input",
    };

    let combined_value = input_req.value(alphas) * output_req.value(alphas);

    #[cfg(any(test, feature = "testing"))]
    {
        debugger.add_request(Box::new(input_req), alphas);
        debugger.add_request(Box::new(output_req), alphas);
    }

    combined_value
}

/// Builds `MPVERIFY` requests made to the hash chiplet.
fn build_mpverify_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
    debugger: &mut BusDebugger<E>,
) -> E {
    let helper_0 = main_trace.helper_register(0, row);

    let s0 = main_trace.stack_element(0, row);
    let s1 = main_trace.stack_element(1, row);
    let s2 = main_trace.stack_element(2, row);
    let s3 = main_trace.stack_element(3, row);
    let s4 = main_trace.stack_element(4, row);
    let s5 = main_trace.stack_element(5, row);
    let s6 = main_trace.stack_element(6, row);
    let s7 = main_trace.stack_element(7, row);
    let s8 = main_trace.stack_element(8, row);
    let s9 = main_trace.stack_element(9, row);

    let input = HasherMessage {
        transition_label: Felt::from(MP_VERIFY_LABEL + 16),
        addr_next: helper_0,
        node_index: s5,
        hasher_state: [ZERO, ZERO, ZERO, ZERO, s3, s2, s1, s0, ZERO, ZERO, ZERO, ZERO],
        source: "mpverify input",
    };

    let output = HasherMessage {
        transition_label: Felt::from(RETURN_HASH_LABEL + 32),
        addr_next: helper_0 + s4.mul_small(8) - ONE,
        node_index: ZERO,
        hasher_state: [ZERO, ZERO, ZERO, ZERO, s9, s8, s7, s6, ZERO, ZERO, ZERO, ZERO],
        source: "mpverify output",
    };

    let combined_value = input.value(alphas) * output.value(alphas);

    #[cfg(any(test, feature = "testing"))]
    {
        debugger.add_request(Box::new(input), alphas);
        debugger.add_request(Box::new(output), alphas);
    }

    combined_value
}

/// Builds `MRUPDATE` requests made to the hash chiplet.
fn build_mrupdate_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
    debugger: &mut BusDebugger<E>,
) -> E {
    let helper_0 = main_trace.helper_register(0, row);

    let s0 = main_trace.stack_element(0, row);
    let s1 = main_trace.stack_element(1, row);
    let s2 = main_trace.stack_element(2, row);
    let s3 = main_trace.stack_element(3, row);
    let s4 = main_trace.stack_element(4, row);
    let s5 = main_trace.stack_element(5, row);
    let s6 = main_trace.stack_element(6, row);
    let s7 = main_trace.stack_element(7, row);
    let s8 = main_trace.stack_element(8, row);
    let s9 = main_trace.stack_element(9, row);
    let s10 = main_trace.stack_element(10, row);
    let s11 = main_trace.stack_element(11, row);
    let s12 = main_trace.stack_element(12, row);
    let s13 = main_trace.stack_element(13, row);
    let s0_nxt = main_trace.stack_element(0, row + 1);
    let s1_nxt = main_trace.stack_element(1, row + 1);
    let s2_nxt = main_trace.stack_element(2, row + 1);
    let s3_nxt = main_trace.stack_element(3, row + 1);

    let input_old = HasherMessage {
        transition_label: Felt::from(MR_UPDATE_OLD_LABEL + 16),
        addr_next: helper_0,
        node_index: s5,
        hasher_state: [ZERO, ZERO, ZERO, ZERO, s3, s2, s1, s0, ZERO, ZERO, ZERO, ZERO],
        source: "mrupdate input_old",
    };

    let output_old = HasherMessage {
        transition_label: Felt::from(RETURN_HASH_LABEL + 32),
        addr_next: helper_0 + s4.mul_small(8) - ONE,
        node_index: ZERO,
        hasher_state: [ZERO, ZERO, ZERO, ZERO, s9, s8, s7, s6, ZERO, ZERO, ZERO, ZERO],
        source: "mrupdate output_old",
    };

    let input_new = HasherMessage {
        transition_label: Felt::from(MR_UPDATE_NEW_LABEL + 16),
        addr_next: helper_0 + s4.mul_small(8),
        node_index: s5,
        hasher_state: [ZERO, ZERO, ZERO, ZERO, s13, s12, s11, s10, ZERO, ZERO, ZERO, ZERO],
        source: "mrupdate input_new",
    };

    let output_new = HasherMessage {
        transition_label: Felt::from(RETURN_HASH_LABEL + 32),
        addr_next: helper_0 + s4.mul_small(16) - ONE,
        node_index: ZERO,
        hasher_state: [
            ZERO, ZERO, ZERO, ZERO, s3_nxt, s2_nxt, s1_nxt, s0_nxt, ZERO, ZERO, ZERO, ZERO,
        ],
        source: "mrupdate output_new",
    };

    let combined_value = input_old.value(alphas)
        * output_old.value(alphas)
        * input_new.value(alphas)
        * output_new.value(alphas);

    #[cfg(any(test, feature = "testing"))]
    {
        debugger.add_request(Box::new(input_old), alphas);
        debugger.add_request(Box::new(output_old), alphas);
        debugger.add_request(Box::new(input_new), alphas);
        debugger.add_request(Box::new(output_new), alphas);
    }

    combined_value
}

// CHIPLETS RESPONSES
// ================================================================================================

/// Builds the response from the hasher chiplet at `row`.
fn build_hasher_chiplet_responses<E>(
    main_trace: &MainTrace,
    row: RowIndex,
    alphas: &[E],
    debugger: &mut BusDebugger<E>,
) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let mut multiplicand = E::ONE;
    let selector0 = main_trace.chiplet_selector_0(row);
    let selector1 = main_trace.chiplet_selector_1(row);
    let selector2 = main_trace.chiplet_selector_2(row);
    let selector3 = main_trace.chiplet_selector_3(row);
    let op_label = get_op_label(selector0, selector1, selector2, selector3);
    let addr_next = Felt::from(row + 1);

    // f_bp, f_mp, f_mv or f_mu == 1
    if row.as_usize() % HASH_CYCLE_LEN == 0 {
        let state = main_trace.chiplet_hasher_state(row);
        let node_index = main_trace.chiplet_node_index(row);
        let transition_label = op_label + Felt::from(16_u8);

        // f_bp == 1
        // v_all = v_h + v_a + v_b + v_c
        if selector1 == ONE && selector2 == ZERO && selector3 == ZERO {
            let hasher_message = HasherMessage {
                transition_label,
                addr_next,
                node_index,
                hasher_state: state,
                source: "hasher",
            };
            multiplicand = hasher_message.value(alphas);

            #[cfg(any(test, feature = "testing"))]
            debugger.add_response(Box::new(hasher_message), alphas);
        }

        // f_mp or f_mv or f_mu == 1
        // v_leaf = v_h + (1 - b) * v_b + b * v_d
        if selector1 == ONE && !(selector2 == ZERO && selector3 == ZERO) {
            let bit = (node_index.as_int() & 1) as u8;
            if bit == 0 {
                let hasher_message = HasherMessage {
                    transition_label,
                    addr_next,
                    node_index,
                    hasher_state: [
                        state[4], state[5], state[6], state[7], ZERO, ZERO, ZERO, ZERO, ZERO, ZERO,
                        ZERO, ZERO,
                    ],
                    source: "hasher",
                };

                multiplicand = hasher_message.value(alphas);

                #[cfg(any(test, feature = "testing"))]
                debugger.add_response(Box::new(hasher_message), alphas);
            } else {
                let hasher_message = HasherMessage {
                    transition_label,
                    addr_next,
                    node_index,
                    hasher_state: [
                        ZERO, ZERO, ZERO, ZERO, state[8], state[9], state[10], state[11], ZERO,
                        ZERO, ZERO, ZERO,
                    ],
                    source: "hasher",
                };

                multiplicand = hasher_message.value(alphas);

                #[cfg(any(test, feature = "testing"))]
                debugger.add_response(Box::new(hasher_message), alphas);
            }
        }
    }

    // f_hout, f_sout, f_abp == 1
    if row.as_usize() % HASH_CYCLE_LEN == HASH_CYCLE_LEN - 1 {
        let state = main_trace.chiplet_hasher_state(row);
        let node_index = main_trace.chiplet_node_index(row);
        let transition_label = op_label + Felt::from(32_u8);

        // f_hout == 1
        // v_res = v_h + v_b;
        if selector1 == ZERO && selector2 == ZERO && selector3 == ZERO {
            let hasher_message = HasherMessage {
                transition_label,
                addr_next,
                node_index,
                hasher_state: [
                    ZERO, ZERO, ZERO, ZERO, state[4], state[5], state[6], state[7], ZERO, ZERO,
                    ZERO, ZERO,
                ],
                source: "hasher",
            };
            multiplicand = hasher_message.value(alphas);

            #[cfg(any(test, feature = "testing"))]
            debugger.add_response(Box::new(hasher_message), alphas);
        }

        // f_sout == 1
        // v_all = v_h + v_a + v_b + v_c
        if selector1 == ZERO && selector2 == ZERO && selector3 == ONE {
            let hasher_message = HasherMessage {
                transition_label,
                addr_next,
                node_index,
                hasher_state: state,
                source: "hasher",
            };

            multiplicand = hasher_message.value(alphas);

            #[cfg(any(test, feature = "testing"))]
            debugger.add_response(Box::new(hasher_message), alphas);
        }

        // f_abp == 1
        // v_abp = v_h + v_b' + v_c' - v_b - v_c
        if selector1 == ONE && selector2 == ZERO && selector3 == ZERO {
            // build the value from the hasher state's just right after the absorption of new
            // elements.
            let state_nxt = main_trace.chiplet_hasher_state(row + 1);

            let hasher_message = HasherMessage {
                transition_label,
                addr_next,
                node_index,
                hasher_state: [
                    ZERO,
                    ZERO,
                    ZERO,
                    ZERO,
                    state_nxt[4],
                    state_nxt[5],
                    state_nxt[6],
                    state_nxt[7],
                    state_nxt[8],
                    state_nxt[9],
                    state_nxt[10],
                    state_nxt[11],
                ],
                source: "hasher",
            };

            multiplicand = hasher_message.value(alphas);

            #[cfg(any(test, feature = "testing"))]
            debugger.add_response(Box::new(hasher_message), alphas);
        }
    }
    multiplicand
}

/// Builds the response from the bitwise chiplet at `row`.
fn build_bitwise_chiplet_responses<E>(
    main_trace: &MainTrace,
    row: RowIndex,
    alphas: &[E],
    debugger: &mut BusDebugger<E>,
) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let is_xor = main_trace.chiplet_selector_2(row);
    if row.as_usize() % BITWISE_OP_CYCLE_LEN == BITWISE_OP_CYCLE_LEN - 1 {
        let bitwise_message = BitwiseMessage {
            op_label: get_op_label(ONE, ZERO, is_xor, ZERO),
            a: main_trace.chiplet_bitwise_a(row),
            b: main_trace.chiplet_bitwise_b(row),
            z: main_trace.chiplet_bitwise_z(row),
        };

        let value = bitwise_message.value(alphas);

        #[cfg(any(test, feature = "testing"))]
        debugger.add_response(Box::new(bitwise_message), alphas);

        value
    } else {
        E::ONE
    }
}

/// Builds the response from the memory chiplet at `row`.
fn build_memory_chiplet_responses<E>(
    main_trace: &MainTrace,
    row: RowIndex,
    alphas: &[E],
    debugger: &mut BusDebugger<E>,
) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let access_type = main_trace.chiplet_selector_4(row);
    let op_label = {
        let is_read = main_trace.chiplet_selector_3(row);
        get_memory_op_label(is_read, access_type)
    };
    let ctx = main_trace.chiplet_memory_ctx(row);
    let clk = main_trace.chiplet_memory_clk(row);
    let addr = {
        let word = main_trace.chiplet_memory_word(row);
        let idx0 = main_trace.chiplet_memory_idx0(row);
        let idx1 = main_trace.chiplet_memory_idx1(row);

        word + idx1.mul_small(2) + idx0
    };

    let message: Box<dyn BusMessage<E>> = if access_type == MEMORY_ACCESS_ELEMENT {
        let idx0 = main_trace.chiplet_memory_idx0(row);
        let idx1 = main_trace.chiplet_memory_idx1(row);

        let element = if idx1 == ZERO && idx0 == ZERO {
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

        let message = MemRequestElementMessage { op_label, ctx, addr, clk, element };

        Box::new(message)
    } else if access_type == MEMORY_ACCESS_WORD {
        let value0 = main_trace.chiplet_memory_value_0(row);
        let value1 = main_trace.chiplet_memory_value_1(row);
        let value2 = main_trace.chiplet_memory_value_2(row);
        let value3 = main_trace.chiplet_memory_value_3(row);

        let message = MemRequestWordMessage {
            op_label,
            ctx,
            addr,
            clk,
            word: [value0, value1, value2, value3],
            source: "memory chiplet",
        };

        Box::new(message)
    } else {
        panic!("Invalid memory element/word column value: {access_type}");
    };

    let value = message.value(alphas);

    #[cfg(any(test, feature = "testing"))]
    debugger.add_response(message, alphas);

    value
}

/// Builds the response from the kernel chiplet at `row`.
fn build_kernel_chiplet_responses<E>(
    main_trace: &MainTrace,
    row: RowIndex,
    alphas: &[E],
    debugger: &mut BusDebugger<E>,
) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let kernel_chiplet_selector = main_trace.chiplet_selector_4(row);
    if kernel_chiplet_selector == ONE {
        let message = {
            let root0 = main_trace.chiplet_kernel_root_0(row);
            let root1 = main_trace.chiplet_kernel_root_1(row);
            let root2 = main_trace.chiplet_kernel_root_2(row);
            let root3 = main_trace.chiplet_kernel_root_3(row);

            KernelRomMessage {
                kernel_proc_digest: [root0, root1, root2, root3],
            }
        };

        let value = message.value(alphas);

        #[cfg(any(test, feature = "testing"))]
        debugger.add_response(Box::new(message), alphas);

        value
    } else {
        E::ONE
    }
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
    debugger: &mut BusDebugger<E>,
) -> E {
    let word = [
        main_trace.stack_element(3, row + 1),
        main_trace.stack_element(2, row + 1),
        main_trace.stack_element(1, row + 1),
        main_trace.stack_element(0, row + 1),
    ];
    let addr = main_trace.stack_element(0, row);

    compute_mem_request_word(main_trace, op_label, alphas, row, addr, word, debugger)
}

/// Builds `MLOAD` and `MSTORE` requests made to the memory chiplet.
fn build_mem_mload_mstore_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    op_label: u8,
    alphas: &[E],
    row: RowIndex,
    debugger: &mut BusDebugger<E>,
) -> E {
    let element = main_trace.stack_element(0, row + 1);
    let addr = main_trace.stack_element(0, row);

    compute_mem_request_element(main_trace, op_label, alphas, row, addr, element, debugger)
}

/// Computes a memory request for a read or write of a single element.
fn compute_mem_request_element<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    op_label: u8,
    alphas: &[E],
    row: RowIndex,
    addr: Felt,
    element: Felt,
    debugger: &mut BusDebugger<E>,
) -> E {
    debug_assert!(op_label == MEMORY_READ_ELEMENT_LABEL || op_label == MEMORY_WRITE_ELEMENT_LABEL);

    let ctx = main_trace.ctx(row);
    let clk = main_trace.clk(row);

    let message = MemRequestElementMessage {
        op_label: Felt::from(op_label),
        ctx,
        addr,
        clk,
        element,
    };

    let value = message.value(alphas);

    #[cfg(any(test, feature = "testing"))]
    debugger.add_request(Box::new(message), alphas);

    value
}

/// Computes a memory request for a read or write of a word.
fn compute_mem_request_word<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    op_label: u8,
    alphas: &[E],
    row: RowIndex,
    addr: Felt,
    word: [Felt; 4],
    debugger: &mut BusDebugger<E>,
) -> E {
    debug_assert!(op_label == MEMORY_READ_WORD_LABEL || op_label == MEMORY_WRITE_WORD_LABEL);
    let ctx = main_trace.ctx(row);
    let clk = main_trace.clk(row);

    let message = MemRequestWordMessage {
        op_label: Felt::from(op_label),
        ctx,
        addr,
        clk,
        word,
        source: if op_label == MEMORY_READ_WORD_LABEL {
            "mloadw"
        } else {
            "mstorew"
        },
    };

    let value = message.value(alphas);

    #[cfg(any(test, feature = "testing"))]
    debugger.add_request(Box::new(message), alphas);

    value
}
