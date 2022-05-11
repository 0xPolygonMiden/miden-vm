use super::{Felt, FieldElement};

// CONSTANTS
// ================================================================================================

/// Number of selector columns in the trace.
pub const NUM_SELECTORS: usize = 2;

/// Number of columns needed to record an execution trace of the bitwise and power of two helper.
pub const TRACE_WIDTH: usize = NUM_SELECTORS + 12;

/// Specifies a bitwise AND operation.
pub const BITWISE_AND: Selectors = [Felt::ZERO, Felt::ZERO];

/// Specifies a bitwise OR operation.
pub const BITWISE_OR: Selectors = [Felt::ZERO, Felt::ONE];

/// Specifies a bitwise XOR operation.
pub const BITWISE_XOR: Selectors = [Felt::ONE, Felt::ZERO];

/// Specifies a power of two operation.
pub const POWER_OF_TWO: Selectors = [Felt::ONE, Felt::ONE];

/// The maximum power of two that can be added to the trace per row.
pub const POW2_POWERS_PER_ROW: usize = 8;

// TYPE ALIASES
// ================================================================================================

pub type Selectors = [Felt; NUM_SELECTORS];
