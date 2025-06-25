use super::{ExecutionError, Felt};
use crate::{ErrorContext, ONE, ZERO};

/// Asserts that the given value is a binary value (0 or 1).
#[inline(always)]
pub fn assert_binary_with_ctx(
    value: Felt,
    err_ctx: &impl ErrorContext,
) -> Result<Felt, ExecutionError> {
    if value != ZERO && value != ONE {
        Err(ExecutionError::not_binary_value_op(value, err_ctx))
    } else {
        Ok(value)
    }
}
