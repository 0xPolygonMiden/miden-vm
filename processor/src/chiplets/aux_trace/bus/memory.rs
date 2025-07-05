use core::fmt::{Display, Formatter, Result as FmtResult};

use miden_air::{
    RowIndex,
    trace::{
        chiplets::{
            ace::{ACE_INSTRUCTION_ID1_OFFSET, ACE_INSTRUCTION_ID2_OFFSET},
            memory::{
                MEMORY_ACCESS_ELEMENT, MEMORY_ACCESS_WORD, MEMORY_READ_ELEMENT_LABEL,
                MEMORY_READ_WORD_LABEL, MEMORY_WRITE_ELEMENT_LABEL, MEMORY_WRITE_WORD_LABEL,
            },
        },
        main_trace::MainTrace,
    },
};
use vm_core::{Felt, FieldElement, ONE, ZERO};

use crate::{
    chiplets::aux_trace::build_value,
    debug::{BusDebugger, BusMessage},
};

// CONSTANTS
// ================================================================================================

const FOUR: Felt = Felt::new(4);

// REQUESTS
// ================================================================================================

/// Builds ACE chiplet read requests as part of the `READ` section made to the memory chiplet.
pub fn build_ace_memory_read_word_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
    _debugger: &mut BusDebugger<E>,
) -> E {
    let word = [
        main_trace.chiplet_ace_v_0_0(row),
        main_trace.chiplet_ace_v_0_1(row),
        main_trace.chiplet_ace_v_1_0(row),
        main_trace.chiplet_ace_v_1_1(row),
    ];
    let op_label = MEMORY_READ_WORD_LABEL;
    let clk = main_trace.chiplet_ace_clk(row);
    let ctx = main_trace.chiplet_ace_ctx(row);
    let addr = main_trace.chiplet_ace_ptr(row);

    let message = MemoryWordMessage {
        op_label: Felt::from(op_label),
        ctx,
        addr,
        clk,
        word,
        source: "read word ACE",
    };

    let value = message.value(alphas);

    #[cfg(any(test, feature = "bus-debugger"))]
    _debugger.add_request(alloc::boxed::Box::new(message), alphas);

    value
}

/// Builds ACE chiplet read requests as part of the `EVAL` section made to the memory chiplet.
pub fn build_ace_memory_read_element_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
    _debugger: &mut BusDebugger<E>,
) -> E {
    let element = main_trace.chiplet_ace_eval_op(row);

    let id_0 = main_trace.chiplet_ace_id_1(row);
    let id_1 = main_trace.chiplet_ace_id_2(row);
    let element =
        id_0 + id_1 * ACE_INSTRUCTION_ID1_OFFSET + (element + ONE) * ACE_INSTRUCTION_ID2_OFFSET;
    let op_label = MEMORY_READ_ELEMENT_LABEL;
    let clk = main_trace.chiplet_ace_clk(row);
    let ctx = main_trace.chiplet_ace_ctx(row);
    let addr = main_trace.chiplet_ace_ptr(row);

    let message = MemoryElementMessage {
        op_label: Felt::from(op_label),
        ctx,
        addr,
        clk,
        element,
    };

    let value = message.value(alphas);

    #[cfg(any(test, feature = "bus-debugger"))]
    _debugger.add_request(alloc::boxed::Box::new(message), alphas);

    value
}

/// Builds `MLOADW` and `MSTOREW` requests made to the memory chiplet.
pub(super) fn build_mem_mloadw_mstorew_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    op_label: u8,
    alphas: &[E],
    row: RowIndex,
    _debugger: &mut BusDebugger<E>,
) -> E {
    let word = [
        main_trace.stack_element(3, row + 1),
        main_trace.stack_element(2, row + 1),
        main_trace.stack_element(1, row + 1),
        main_trace.stack_element(0, row + 1),
    ];
    let addr = main_trace.stack_element(0, row);

    debug_assert!(op_label == MEMORY_READ_WORD_LABEL || op_label == MEMORY_WRITE_WORD_LABEL);
    let ctx = main_trace.ctx(row);
    let clk = main_trace.clk(row);

    let message = MemoryWordMessage {
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

    #[cfg(any(test, feature = "bus-debugger"))]
    _debugger.add_request(alloc::boxed::Box::new(message), alphas);

    value
}

/// Builds `MLOAD` and `MSTORE` requests made to the memory chiplet.
pub(super) fn build_mem_mload_mstore_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    op_label: u8,
    alphas: &[E],
    row: RowIndex,
    _debugger: &mut BusDebugger<E>,
) -> E {
    let element = main_trace.stack_element(0, row + 1);
    let addr = main_trace.stack_element(0, row);

    debug_assert!(op_label == MEMORY_READ_ELEMENT_LABEL || op_label == MEMORY_WRITE_ELEMENT_LABEL);

    let ctx = main_trace.ctx(row);
    let clk = main_trace.clk(row);

    let message = MemoryElementMessage {
        op_label: Felt::from(op_label),
        ctx,
        addr,
        clk,
        element,
    };

    let value = message.value(alphas);

    #[cfg(any(test, feature = "bus-debugger"))]
    _debugger.add_request(alloc::boxed::Box::new(message), alphas);

    value
}

/// Builds `MSTREAM` requests made to the memory chiplet.
pub(super) fn build_mstream_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
    _debugger: &mut BusDebugger<E>,
) -> E {
    let op_label = Felt::from(MEMORY_READ_WORD_LABEL);
    let addr = main_trace.stack_element(12, row);
    let ctx = main_trace.ctx(row);
    let clk = main_trace.clk(row);

    let mem_req_1 = MemoryWordMessage {
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
    let mem_req_2 = MemoryWordMessage {
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

    #[cfg(any(test, feature = "bus-debugger"))]
    {
        _debugger.add_request(alloc::boxed::Box::new(mem_req_1), alphas);
        _debugger.add_request(alloc::boxed::Box::new(mem_req_2), alphas);
    }

    combined_value
}

/// Builds `PIPE` requests made to the memory chiplet.
pub(super) fn build_pipe_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
    _debugger: &mut BusDebugger<E>,
) -> E {
    let op_label = Felt::from(MEMORY_WRITE_WORD_LABEL);
    let addr = main_trace.stack_element(12, row);
    let ctx = main_trace.ctx(row);
    let clk = main_trace.clk(row);

    let mem_req_1 = MemoryWordMessage {
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
    let mem_req_2 = MemoryWordMessage {
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

    #[cfg(any(test, feature = "bus-debugger"))]
    {
        _debugger.add_request(alloc::boxed::Box::new(mem_req_1), alphas);
        _debugger.add_request(alloc::boxed::Box::new(mem_req_2), alphas);
    }

    combined_value
}

/// Builds `HORNERBASE` or `HORNEREXT` requests made to the memory chiplet.
pub(super) fn build_horner_eval_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
    _debugger: &mut BusDebugger<E>,
) -> E {
    let eval_point_0 = main_trace.helper_register(0, row);
    let eval_point_1 = main_trace.helper_register(1, row);
    let mem_junk_0 = main_trace.helper_register(2, row);
    let mem_junk_1 = main_trace.helper_register(3, row);
    let eval_point_ptr = main_trace.stack_element(13, row);
    let op_label = Felt::from(MEMORY_READ_WORD_LABEL);

    let ctx = main_trace.ctx(row);
    let clk = main_trace.clk(row);

    let mem_req = MemoryWordMessage {
        op_label,
        ctx,
        addr: eval_point_ptr,
        clk,
        word: [eval_point_0, eval_point_1, mem_junk_0, mem_junk_1],
        source: "horner_eval_* req",
    };

    let value = mem_req.value(alphas);

    #[cfg(any(test, feature = "bus-debugger"))]
    {
        _debugger.add_request(alloc::boxed::Box::new(mem_req), alphas);
    }

    value
}

// RESPONSES
// ================================================================================================

/// Builds the response from the memory chiplet at `row`.
pub(super) fn build_memory_chiplet_responses<E>(
    main_trace: &MainTrace,
    row: RowIndex,
    alphas: &[E],
    _debugger: &mut BusDebugger<E>,
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

    if access_type == MEMORY_ACCESS_ELEMENT {
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

        let message = MemoryElementMessage { op_label, ctx, addr, clk, element };

        let value = message.value(alphas);

        #[cfg(any(test, feature = "bus-debugger"))]
        _debugger.add_response(alloc::boxed::Box::new(message), alphas);

        value
    } else if access_type == MEMORY_ACCESS_WORD {
        let value0 = main_trace.chiplet_memory_value_0(row);
        let value1 = main_trace.chiplet_memory_value_1(row);
        let value2 = main_trace.chiplet_memory_value_2(row);
        let value3 = main_trace.chiplet_memory_value_3(row);

        let message = MemoryWordMessage {
            op_label,
            ctx,
            addr,
            clk,
            word: [value0, value1, value2, value3],
            source: "memory chiplet",
        };

        let value = message.value(alphas);

        #[cfg(any(test, feature = "bus-debugger"))]
        _debugger.add_response(alloc::boxed::Box::new(message), alphas);

        value
    } else {
        panic!("Invalid memory element/word column value: {access_type}");
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Returns the operation unique label for memory operations.
///
/// The memory selector flags are `[1, 1, 0, is_read, is_word_access]`.
/// The flag is derived as the big-endian representation of these flags, plus one.
/// They are also defined in [`chiplets::memory`](miden_air::trace::chiplets::memory).
fn get_memory_op_label(is_read: Felt, is_word_access: Felt) -> Felt {
    let is_read = (is_read == ONE) as u8;
    let is_word_access = (is_word_access == ONE) as u8;

    const MEMORY_SELECTOR_FLAG_BASE: u8 = 0b011 + 1;
    const OP_FLAG_SHIFT: u8 = 3;

    let op_flag = is_read + 2 * is_word_access;

    Felt::from(MEMORY_SELECTOR_FLAG_BASE + (op_flag << OP_FLAG_SHIFT))
}

// MESSAGES
// ===============================================================================================

pub struct MemoryWordMessage {
    pub op_label: Felt,
    pub ctx: Felt,
    pub addr: Felt,
    pub clk: Felt,
    pub word: [Felt; 4],
    pub source: &'static str,
}

impl<E> BusMessage<E> for MemoryWordMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        alphas[0]
            + build_value(
                &alphas[1..9],
                [
                    self.op_label,
                    self.ctx,
                    self.addr,
                    self.clk,
                    self.word[0],
                    self.word[1],
                    self.word[2],
                    self.word[3],
                ],
            )
    }

    fn source(&self) -> &str {
        self.source
    }
}

impl Display for MemoryWordMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{{ op_label: {}, ctx: {}, addr: {}, clk: {}, word: {:?} }}",
            self.op_label, self.ctx, self.addr, self.clk, self.word
        )
    }
}

pub struct MemoryElementMessage {
    pub op_label: Felt,
    pub ctx: Felt,
    pub addr: Felt,
    pub clk: Felt,
    pub element: Felt,
}

impl<E> BusMessage<E> for MemoryElementMessage
where
    E: FieldElement<BaseField = Felt>,
{
    fn value(&self, alphas: &[E]) -> E {
        alphas[0]
            + build_value(
                &alphas[1..6],
                [self.op_label, self.ctx, self.addr, self.clk, self.element],
            )
    }

    fn source(&self) -> &str {
        "memory element"
    }
}

impl Display for MemoryElementMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{{ op_label: {}, ctx: {}, addr: {}, clk: {}, element: {} }}",
            self.op_label, self.ctx, self.addr, self.clk, self.element
        )
    }
}
