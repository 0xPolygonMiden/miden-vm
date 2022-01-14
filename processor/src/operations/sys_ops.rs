use super::{ExecutionError, Felt, FieldElement, Process};

// SYSTEM OPERATIONS
// ================================================================================================

impl Process {
    /// Pops a value off the stack and asserts that it is equal to ONE.
    ///
    /// # Errors
    /// Returns an error if the popped value is not ONE.
    pub(super) fn op_assert(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(1, "ASSERT")?;
        if self.stack.get(0) != Felt::ONE {
            return Err(ExecutionError::FailedAssertion(self.step));
        }
        self.stack.shift_left(1);
        Ok(())
    }
}
