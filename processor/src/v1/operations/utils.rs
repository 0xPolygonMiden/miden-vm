use super::{BaseElement, ExecutionError, FieldElement};

/// TODO: add docs
#[inline(always)]
pub fn assert_binary(value: BaseElement) -> Result<BaseElement, ExecutionError> {
    if value != BaseElement::ZERO && value != BaseElement::ONE {
        Err(ExecutionError::NotBinaryValue(value))
    } else {
        Ok(value)
    }
}
