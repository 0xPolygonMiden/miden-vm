use vm_core::mast::MastNodeExt;

use super::{ExecutionError, Felt};
use crate::{ErrorContext, ONE, ZERO};

/// TODO: add docs
#[inline(always)]
pub fn assert_binary(
    value: Felt,
    err_ctx: &ErrorContext<'_, impl MastNodeExt>,
) -> Result<Felt, ExecutionError> {
    if value != ZERO && value != ONE {
        Err(ExecutionError::not_binary_value_op(value, err_ctx))
    } else {
        Ok(value)
    }
}
