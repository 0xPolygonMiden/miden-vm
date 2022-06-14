use super::{range, Operation};
use core::ops::Range;

// CONSTANTS
// ================================================================================================

/// Index of the column holding code block IDs (which are row addresses from the hasher table).
pub const ADDR_COL_IDX: usize = 0;

/// Index at which operation bit columns start in the decoder trace.
pub const OP_BITS_OFFSET: usize = ADDR_COL_IDX + 1;

/// Number of columns needed to hold a binary representation of of opcodes.
pub const NUM_OP_BITS: usize = Operation::OP_BITS;

/// Location of operation bits columns in the decoder trace.
pub const OP_BITS_RANGE: Range<usize> = range(OP_BITS_OFFSET, NUM_OP_BITS);

// TODO: probably rename "hasher state" to something like "shared columns".

/// Index at which hasher state columns start in the decoder trace.
pub const HASHER_STATE_OFFSET: usize = OP_BITS_OFFSET + NUM_OP_BITS;

/// Number of hasher columns in the decoder trace.
pub const NUM_HASHER_COLUMNS: usize = 8;

/// Location of hasher columns in the decoder trace.
pub const HASHER_STATE_RANGE: Range<usize> = range(HASHER_STATE_OFFSET, NUM_HASHER_COLUMNS);

/// Index of the in_span column in the decoder trace.
pub const IN_SPAN_COL_IDX: usize = HASHER_STATE_OFFSET + NUM_HASHER_COLUMNS;

/// Index of the operation group count column in the decoder trace.
pub const GROUP_COUNT_COL_IDX: usize = IN_SPAN_COL_IDX + 1;

/// Index of the operation index column in the decoder trace.
pub const OP_INDEX_COL_IDX: usize = GROUP_COUNT_COL_IDX + 1;

/// Index at which operation batch flag columns start in the decoder trace.
pub const OP_BATCH_FLAGS_OFFSET: usize = OP_INDEX_COL_IDX + 1;

/// Number of operation batch flag columns.
pub const NUM_OP_BATCH_FLAGS: usize = 3;

/// Location of operation batch flag columns in the decoder trace.
pub const OP_BATCH_FLAGS_RANGE: Range<usize> = range(OP_BATCH_FLAGS_OFFSET, NUM_OP_BATCH_FLAGS);
