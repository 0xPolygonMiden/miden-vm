use super::Felt;

// CONSTANTS
// ================================================================================================

/// Number of columns needed to record an execution trace of the kernel ROM chiplet.
pub const TRACE_WIDTH: usize = 5;

// --- OPERATION SELECTORS ------------------------------------------------------------------------

// All kernel ROM bus labels encode the chiplet selector (1, 1, 1, 0), appended with the internal
// selector `s_first` which indicates whether the chiplet should respond to an `init` or `call`
// request.
// These do not collide with any other labels.

/// Specifies a kernel procedure call operation to access a procedure in the kernel ROM.
///
/// The unique operation label is given by the chiplet selector `[1, 1, 1, 0]` followed by `0`
/// which is the value of the `s_first` column in this row.
pub const KERNEL_PROC_CALL_LABEL: Felt = Felt::new(0b11100);

/// Specified the label of the kernel ROM initialization request by the verifier.
///
/// The unique operation label is given by the chiplet selector `[1, 1, 1, 0]` followed by `1`
/// which is the value of the `s_first` column in this row.
pub const KERNEL_PROC_INIT_LABEL: Felt = Felt::new(0b11101);
