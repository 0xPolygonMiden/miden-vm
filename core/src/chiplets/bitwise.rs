use super::{Felt, FieldElement};

// CONSTANTS
// ================================================================================================

/// Number of selector columns in the trace.
pub const NUM_SELECTORS: usize = 2;

/// Number of columns needed to record an execution trace of the bitwise helper.
pub const TRACE_WIDTH: usize = NUM_SELECTORS + 12;

/// The number of rows required to compute an operation in the Bitwise chiplet.
pub const OP_CYCLE_LEN: usize = 8;

// --- OPERATION SELECTORS ------------------------------------------------------------------------

/// Specifies a bitwise AND operation.
pub const BITWISE_AND: Selectors = [Felt::ZERO, Felt::ZERO];

/// Specifies a bitwise OR operation.
pub const BITWISE_OR: Selectors = [Felt::ZERO, Felt::ONE];

/// Specifies a bitwise XOR operation.
pub const BITWISE_XOR: Selectors = [Felt::ONE, Felt::ZERO];

// --- INPUT DECOMPOSITION ------------------------------------------------------------------------

/// The number of bits decomposed per row per input parameter `a` or `b`.
pub const BITWISE_NUM_DECOMP_BITS: usize = 4;

// --- COLUMN ACCESSORS ---------------------------------------------------------------------------

/// The index of the column holding the aggregated value of input `a` within the bitwise execution
/// trace.
pub const BITWISE_A_COL_IDX: usize = NUM_SELECTORS;

/// The index of the column holding the aggregated value of input `b` within the bitwise execution
/// trace.
pub const BITWISE_B_COL_IDX: usize = BITWISE_A_COL_IDX + 1;

/// The index of the column containing the aggregated output value within the bitwise execution
/// trace.
pub const BITWISE_OUTPUT_COL_IDX: usize = BITWISE_B_COL_IDX + 1 + 2 * BITWISE_NUM_DECOMP_BITS;

// TYPE ALIASES
// ================================================================================================

pub type Selectors = [Felt; NUM_SELECTORS];
