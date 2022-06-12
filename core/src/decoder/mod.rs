use super::{range, Operation};
use core::ops::Range;

// CONSTANTS
// ================================================================================================

/// Index of the column holding code block IDs (which are row addresses from the hasher table).
pub const ADDR_COL_IDX: usize = 0;

/// Index at which operation bit columns start in the decoder trace.
pub const OP_BITS_OFFSET: usize = 1;

/// Location of operation bits columns in the decoder trace.
pub const OP_BITS_RANGE: Range<usize> = range(OP_BITS_OFFSET, Operation::OP_BITS);

/// Index of the in_span column in the decoder trace.
pub const IN_SPAN_COL_IDX: usize = 8;

/// Index of the operation group count column in the decoder trace.
pub const GROUP_COUNT_COL_IDX: usize = 17;

/// Index of the operation index column in the decoder trace.
pub const OP_INDEX_COL_IDX: usize = 18;

// TODO: probably rename "hasher state" to something like "shared columns".

/// Index at which hasher state columns start in the decoder trace.
pub const HASHER_STATE_OFFSET: usize = 9;

/// Number of hasher columns in the decoder trace.
pub const NUM_HASHER_COLUMNS: usize = 8;

/// Location of hasher columns in the decoder trace.
pub const HASHER_STATE_RANGE: Range<usize> = range(HASHER_STATE_OFFSET, NUM_HASHER_COLUMNS);
