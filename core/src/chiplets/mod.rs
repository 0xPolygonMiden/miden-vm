use super::{
    utils::range as create_range, Felt, FieldElement, Word, CHIPLETS_OFFSET,
    HASHER_AUX_TRACE_OFFSET,
};
use core::ops::Range;

pub mod bitwise;
pub mod hasher;
pub mod memory;

// CONSTANTS
// ================================================================================================

/// The number of columns in the chiplets which are used as selectors for the hasher chiplet.
pub const NUM_HASHER_SELECTORS: usize = 1;
/// The number of columns in the chiplets which are used as selectors for the bitwise chiplet.
pub const NUM_BITWISE_SELECTORS: usize = 2;
/// The number of columns in the chiplets which are used as selectors for the memory chiplet.
pub const NUM_MEMORY_SELECTORS: usize = 3;

/// The first column of the hash chiplet.
pub const HASHER_TRACE_OFFSET: usize = CHIPLETS_OFFSET + NUM_HASHER_SELECTORS;
/// The first column of the bitwise chiplet.
pub const BITWISE_TRACE_OFFSET: usize = CHIPLETS_OFFSET + NUM_BITWISE_SELECTORS;
/// The first column of the memory chiplet.
pub const MEMORY_TRACE_OFFSET: usize = CHIPLETS_OFFSET + NUM_MEMORY_SELECTORS;
