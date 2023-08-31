use super::{ExecutionError, Felt};
use crate::{ONE, ZERO};

/// TODO: add docs
#[inline(always)]
pub fn assert_binary(value: Felt) -> Result<Felt, ExecutionError> {
    if value != ZERO && value != ONE {
        Err(ExecutionError::NotBinaryValue(value))
    } else {
        Ok(value)
    }
}
