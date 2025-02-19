use miden_air::{trace::main_trace::MainTrace, RowIndex};
use vm_core::{Felt, FieldElement, ONE};

use super::messages::KernelRomMessage;
use crate::debug::{BusDebugger, BusMessage};

// REQUESTS
// ================================================================================================

// Note: all requests are handled in the `super` module, since they involve messages to multiple
// chiplets.

// RESPONSES
// ================================================================================================

/// Builds the response from the kernel chiplet at `row`.
pub(super) fn build_kernel_chiplet_responses<E>(
    main_trace: &MainTrace,
    row: RowIndex,
    alphas: &[E],
    _debugger: &mut BusDebugger<E>,
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

        #[cfg(any(test, feature = "bus-debugger"))]
        _debugger.add_response(alloc::boxed::Box::new(message), alphas);

        value
    } else {
        E::ONE
    }
}
