use super::{utils::range as create_range, AUX_TABLE_OFFSET};
use core::ops::Range;

pub mod memory;

// CONSTANTS
// ================================================================================================

/// The number of columns in the auxiliary table which are used as selectors for the hasher segment.
pub const NUM_HASHER_SELECTORS: usize = 1;
/// The number of columns in the aux table which are used as selectors for the bitwise segment.
pub const NUM_BITWISE_SELECTORS: usize = 2;
/// The number of columns in the auxiliary table which are used as selectors for the memory segment.
pub const NUM_MEMORY_SELECTORS: usize = 3;

/// The first column of the hash co-processor.
pub const HASHER_TRACE_OFFSET: usize = AUX_TABLE_OFFSET + NUM_HASHER_SELECTORS;
/// The first column of the bitwise co-processor.
pub const BITWISE_TRACE_OFFSET: usize = AUX_TABLE_OFFSET + NUM_BITWISE_SELECTORS;
/// The first column of the memory co-processor.
pub const MEMORY_TRACE_OFFSET: usize = AUX_TABLE_OFFSET + NUM_MEMORY_SELECTORS;
