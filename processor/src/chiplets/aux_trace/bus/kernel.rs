use core::fmt::{Display, Formatter, Result as FmtResult};

use miden_air::{
    RowIndex,
    trace::{chiplets::kernel_rom::KERNEL_PROC_LABEL, main_trace::MainTrace},
};
use vm_core::{Felt, FieldElement, crypto::hash::RpoDigest};

use crate::{
    chiplets::aux_trace::bus::build_value,
    debug::{BusDebugger, BusMessage},
};

// REQUESTS
// ================================================================================================

/// Builds the requests for each unique kernel proc hash, to be provided via public inputs.
pub(super) fn build_kernel_init_requests<E>(
    proc_hashes: &[RpoDigest],
    alphas: &[E],
    _debugger: &mut BusDebugger<E>,
) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let mut requests = E::ONE;
    // Initialize the bus with the kernel rom hashes provided by the public inputs.
    // The verifier computes this value, and is enforced with a boundary constraint in the
    // first row.
    for proc_hash in proc_hashes {
        let message = KernelRomInitMessage { kernel_proc_digest: proc_hash.into() };

        requests *= message.value(alphas);

        #[cfg(any(test, feature = "bus-debugger"))]
        _debugger.add_request(std::boxed::Box::new(message), alphas);
    }
    requests
}

// RESPONSES
// ================================================================================================

/// Builds the response from the kernel chiplet at `row`.
///
/// # Details
/// Each row responds to either
/// - requests made by the verifier for checking that the ROM contains exactly the hashes given by
///   public inputs, or,
/// - requests by the decoder when it performs a SYSCALL.
///
/// If a kernel proc hash is requested `n` times by the decoder, it is repeated `n+1` times
/// in the trace. In the first row, the chiplet responds to a request made via public inputs.
/// The remaining `n` rows respond to decoder requests.
pub(super) fn build_kernel_chiplet_responses<E>(
    main_trace: &MainTrace,
    row: RowIndex,
    alphas: &[E],
    _debugger: &mut BusDebugger<E>,
) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let root0 = main_trace.chiplet_kernel_root_0(row);
    let root1 = main_trace.chiplet_kernel_root_1(row);
    let root2 = main_trace.chiplet_kernel_root_2(row);
    let root3 = main_trace.chiplet_kernel_root_3(row);

    // The caller ensures this row is a kernel ROM row, so we just need to check if this is
    // the first row for a unique proc hash.
    if main_trace.chiplet_kernel_is_first_hash_row(row) {
        // Respond to the requests performed by the verifier when they initialize the bus
        // column with the unique proc hashes.
        let message = KernelRomInitMessage {
            kernel_proc_digest: [root0, root1, root2, root3],
        };
        let value = message.value(alphas);

        #[cfg(any(test, feature = "bus-debugger"))]
        _debugger.add_response(std::boxed::Box::new(message), alphas);

        value
    } else {
        // Respond to decoder messages.
        let message = KernelRomMessage {
            kernel_proc_digest: [root0, root1, root2, root3],
        };
        let value = message.value(alphas);

        #[cfg(any(test, feature = "bus-debugger"))]
        _debugger.add_response(std::boxed::Box::new(message), alphas);
        value
    }
}

// MESSAGES
// ===============================================================================================

/// A message between the decoder and the kernel ROM to ensure a SYSCALL can only call procedures
/// provided through public inputs.
pub struct KernelRomMessage {
    pub kernel_proc_digest: [Felt; 4],
}

impl<E> BusMessage<E> for KernelRomMessage
where
    E: FieldElement<BaseField = Felt>,
{
    #[inline(always)]
    fn value(&self, alphas: &[E]) -> E {
        alphas[0]
            + build_value(
                &alphas[1..6],
                [
                    KERNEL_PROC_LABEL,
                    self.kernel_proc_digest[0],
                    self.kernel_proc_digest[1],
                    self.kernel_proc_digest[2],
                    self.kernel_proc_digest[3],
                ],
            )
    }

    fn source(&self) -> &str {
        "kernel rom"
    }
}

impl Display for KernelRomMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{{ proc digest: {:?} }}", self.kernel_proc_digest)
    }
}

/// A message linking unique kernel procedure hashes provided by public inputs, with hashes
/// contained in the kernel ROM chiplet trace.
pub struct KernelRomInitMessage {
    pub kernel_proc_digest: [Felt; 4],
}

impl<E> BusMessage<E> for KernelRomInitMessage
where
    E: FieldElement<BaseField = Felt>,
{
    #[inline(always)]
    fn value(&self, alphas: &[E]) -> E {
        // In contrast to the responses to the decoder, we omit the label to simplify
        // the verifier's work when initializing the virtual table's bus.
        // This is safe, as long as all other messages are domain separated with a tag.
        // TODO(adr1anh):
        //   This may actually be unsafe, unless we explicitly use the 0 tag for
        //   this message. The first element of a hash could correspond to one of the
        //   the OP tags.
        //   One solution could be to just use the 0 tag, and use challenges 2..5.
        alphas[0]
            + build_value(
                &alphas[1..5],
                [
                    self.kernel_proc_digest[0],
                    self.kernel_proc_digest[1],
                    self.kernel_proc_digest[2],
                    self.kernel_proc_digest[3],
                ],
            )
    }

    fn source(&self) -> &str {
        "kernel rom init"
    }
}

impl Display for KernelRomInitMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{{ proc digest init: {:?} }}", self.kernel_proc_digest)
    }
}
