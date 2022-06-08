use super::{Felt, FieldElement};

// CONSTANTS
// ================================================================================================

/// Number of selector columns in the trace.
pub const NUM_SELECTORS: usize = 2;

/// Number of columns needed to record an execution trace of the bitwise and power of two helper.
pub const TRACE_WIDTH: usize = NUM_SELECTORS + 12;

// --- OPERATION SELECTORS ------------------------------------------------------------------------

/// Specifies a bitwise AND operation.
pub const BITWISE_AND: Selectors = [Felt::ZERO, Felt::ZERO];

/// Specifies a bitwise OR operation.
pub const BITWISE_OR: Selectors = [Felt::ZERO, Felt::ONE];

/// Specifies a bitwise XOR operation.
pub const BITWISE_XOR: Selectors = [Felt::ONE, Felt::ZERO];

/// Specifies a power of two operation.
pub const POWER_OF_TWO: Selectors = [Felt::ONE, Felt::ONE];

// --- INPUT DECOMPOSITION ------------------------------------------------------------------------

/// The number of bits decomposed per row per input parameter `a` or `b`.
pub const BITWISE_NUM_DECOMP_BITS: usize = 4;

/// The maximum power of two that can be added to the trace per row.
pub const POW2_POWERS_PER_ROW: usize = 8;

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
