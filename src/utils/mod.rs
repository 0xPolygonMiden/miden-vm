use core::ops::Range;

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
