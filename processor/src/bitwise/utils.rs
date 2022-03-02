use super::{ExecutionError, Felt, StarkField};

pub fn assert_u32(value: Felt) -> Result<Felt, ExecutionError> {
    let val_u64 = value.as_int();
    if val_u64 > u32::MAX.into() {
        Err(ExecutionError::NotU32Value(value))
    } else {
        Ok(value)
    }
}
