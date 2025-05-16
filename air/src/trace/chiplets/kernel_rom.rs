use super::Felt;

// CONSTANTS
// ================================================================================================

/// Number of columns needed to record an execution trace of the kernel ROM chiplet.
pub const TRACE_WIDTH: usize = 5;

// --- OPERATION SELECTORS ------------------------------------------------------------------------

/// Specifies a kernel procedure call operation to access a procedure in the kernel ROM.
///
/// The unique operation label is computed as 1 plus the combined chiplet and internal selector
/// with the bits reversed: kernel ROM selector=[1, 1, 1, 0] +1=[0, 0, 0, 1].
pub const KERNEL_PROC_LABEL: Felt = Felt::new(0b1000);
