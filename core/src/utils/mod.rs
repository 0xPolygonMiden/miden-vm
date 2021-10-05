use crate::BaseElement;
use core::ops::Range;

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

// TYPE CONVERSIONS
// ================================================================================================

pub trait ToElements {
    fn to_elements(&self) -> Vec<BaseElement>;
}

impl<const N: usize> ToElements for [u128; N] {
    fn to_elements(&self) -> Vec<BaseElement> {
        self.iter().map(|&v| BaseElement::new(v)).collect()
    }
}

impl ToElements for Vec<u128> {
    fn to_elements(&self) -> Vec<BaseElement> {
        self.iter().map(|&v| BaseElement::new(v)).collect()
    }
}
