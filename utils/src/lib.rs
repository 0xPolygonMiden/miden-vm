use core::ops::Range;

// CONSTANTS
// ================================================================================================

const CYCLE_LENGTH: usize = 16;
const SPONGE_WIDTH: usize = 4;
const DIGEST_SIZE: usize = 2;
const HASH_NUM_ROUNDS: usize = 10;
const HASH_STATE_RATE: usize = 4;
const HASH_STATE_CAPACITY: usize = 2;
pub const HASH_STATE_WIDTH: usize = HASH_STATE_RATE + HASH_STATE_CAPACITY;

// RE-EXPORTS
// ================================================================================================
pub mod hasher;
pub mod sponge;

// RANGE
// ================================================================================================
pub trait RangeSlider {
    fn slide(self, slide_by: usize) -> Self;
}

impl RangeSlider for Range<usize> {
    fn slide(self, width: usize) -> Range<usize> {
        Range {
            start: self.end,
            end: self.end + width,
        }
    }
}
