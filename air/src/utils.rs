use super::FieldElement;

// BASIC CONSTRAINT OPERATORS
// ================================================================================================

#[inline(always)]
pub fn is_binary<E: FieldElement>(v: E) -> E {
    v.square() - v
}
