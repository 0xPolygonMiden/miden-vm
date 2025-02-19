use miden_air::{
    trace::{chiplets::bitwise::OP_CYCLE_LEN as BITWISE_OP_CYCLE_LEN, main_trace::MainTrace},
    RowIndex,
};
use vm_core::{Felt, FieldElement, ONE, ZERO};

use super::{get_op_label, messages::BitwiseMessage};
use crate::debug::{BusDebugger, BusMessage};

// REQUESTS
// ==============================================================================================

/// Builds requests made to the bitwise chiplet. This can be either a request for the computation
/// of a `XOR` or an `AND` operation.
pub(super) fn build_bitwise_request<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    is_xor: Felt,
    alphas: &[E],
    row: RowIndex,
    _debugger: &mut BusDebugger<E>,
) -> E {
    let bitwise_request_message = BitwiseMessage {
        op_label: get_op_label(ONE, ZERO, is_xor, ZERO),
        a: main_trace.stack_element(1, row),
        b: main_trace.stack_element(0, row),
        z: main_trace.stack_element(0, row + 1),
        source: if is_xor == ONE { "u32xor" } else { "u32and" },
    };

    let value = bitwise_request_message.value(alphas);

    #[cfg(any(test, feature = "bus-debugger"))]
    _debugger.add_request(alloc::boxed::Box::new(bitwise_request_message), alphas);

    value
}

// RESPONSES
// ==============================================================================================

/// Builds the response from the bitwise chiplet at `row`.
pub(super) fn build_bitwise_chiplet_responses<E>(
    main_trace: &MainTrace,
    row: RowIndex,
    alphas: &[E],
    _debugger: &mut BusDebugger<E>,
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
            source: "bitwise chiplet",
        };

        let value = bitwise_message.value(alphas);

        #[cfg(any(test, feature = "bus-debugger"))]
        _debugger.add_response(alloc::boxed::Box::new(bitwise_message), alphas);

        value
    } else {
        E::ONE
    }
}
