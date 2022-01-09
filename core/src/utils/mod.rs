use crate::BaseElement;
use core::ops::Range;


// TYPE CONVERSIONS
// ================================================================================================

pub trait ToElements {
    fn to_elements(&self) -> Vec<BaseElement>;
}

impl<const N: usize> ToElements for [u64; N] {
    fn to_elements(&self) -> Vec<BaseElement> {
        self.iter().map(|&v| BaseElement::new(v)).collect()
    }
}

impl ToElements for Vec<u64> {
    fn to_elements(&self) -> Vec<BaseElement> {
        self.iter().map(|&v| BaseElement::new(v)).collect()
    }
}
