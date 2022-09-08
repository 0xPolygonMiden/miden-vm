use super::{range, Range};

// CONSTANTS
// ================================================================================================

/// Length of a stack word.
pub const WORD_LENGTH: usize = 4;

/// Index at which the first word starts in the stack trace.
pub const WORD1_OFFSET: usize = 0;

/// Location of first word in the stack trace.
pub const WORD1_RANGE: Range<usize> = range(WORD1_OFFSET, WORD_LENGTH);

/// Index at which the second word starts in the stack trace.
pub const WORD2_OFFSET: usize = WORD1_RANGE.end;

/// Location of second word in the stack trace.
pub const WORD2_RANGE: Range<usize> = range(WORD2_OFFSET, WORD_LENGTH);

/// Index at which the third word starts in the stack trace.
pub const WORD3_OFFSET: usize = WORD2_RANGE.end;

/// Location of third word in the stack trace.
pub const WORD3_RANGE: Range<usize> = range(WORD3_OFFSET, WORD_LENGTH);

/// Index at which the fourth word starts in the stack trace.
pub const WORD4_OFFSET: usize = WORD3_RANGE.end;

/// Location of fourth word in the stack trace.
pub const WORD4_RANGE: Range<usize> = range(WORD4_OFFSET, WORD_LENGTH);
