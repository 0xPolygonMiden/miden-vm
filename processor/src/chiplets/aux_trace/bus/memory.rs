use alloc::boxed::Box;

use miden_air::{
    trace::{
        chiplets::memory::{
            MEMORY_ACCESS_ELEMENT, MEMORY_ACCESS_WORD, MEMORY_READ_ELEMENT_LABEL,
            MEMORY_READ_WORD_LABEL, MEMORY_WRITE_ELEMENT_LABEL, MEMORY_WRITE_WORD_LABEL,
        },
        main_trace::MainTrace,
    },
    RowIndex,
};
use vm_core::{Felt, FieldElement, ONE, ZERO};

use super::messages::{MemoryElementMessage, MemoryWordMessage};
use crate::debug::{BusDebugger, BusMessage};

// CONSTANTS
// ================================================================================================

const FOUR: Felt = Felt::new(4);

// REQUESTS
// ================================================================================================

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
    _debugger.add_request(Box::new(message), alphas);

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
    _debugger.add_request(Box::new(message), alphas);

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
        _debugger.add_request(Box::new(mem_req_1), alphas);
        _debugger.add_request(Box::new(mem_req_2), alphas);
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
        _debugger.add_request(Box::new(mem_req_1), alphas);
        _debugger.add_request(Box::new(mem_req_2), alphas);
    }

    combined_value
}

/// Builds `RCOMBBASE` requests made to the memory chiplet.
pub(super) fn build_rcomb_base_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    alphas: &[E],
    row: RowIndex,
    _debugger: &mut BusDebugger<E>,
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

    let mem_req_1 = MemoryWordMessage {
        op_label,
        ctx,
        addr: z_ptr,
        clk,
        word: [tz0, tz1, tzg0, tzg1],
        source: "rcombbase req 1",
    };
    let mem_req_2 = MemoryWordMessage {
        op_label,
        ctx,
        addr: a_ptr,
        clk,
        word: [a0, a1, ZERO, ZERO],
        source: "rcombbase req 2",
    };

    let combined_value = mem_req_1.value(alphas) * mem_req_2.value(alphas);

    #[cfg(any(test, feature = "bus-debugger"))]
    {
        _debugger.add_request(Box::new(mem_req_1), alphas);
        _debugger.add_request(Box::new(mem_req_2), alphas);
    }

    combined_value
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

        let message = MemoryElementMessage { op_label, ctx, addr, clk, element };

        Box::new(message)
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

        Box::new(message)
    } else {
        panic!("Invalid memory element/word column value: {access_type}");
    };

    let value = message.value(alphas);

    #[cfg(any(test, feature = "bus-debugger"))]
    _debugger.add_response(message, alphas);

    value
}

// HELPER FUNCTIONS
// ================================================================================================

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
