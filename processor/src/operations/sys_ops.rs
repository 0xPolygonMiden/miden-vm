use super::{ExecutionError, Felt, FieldElement, Process, StarkField};

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
            return Err(ExecutionError::FailedAssertion(self.system.clk()));
        }
        self.stack.shift_left(1);
        Ok(())
    }

    // FREE MEMORY POINTER
    // --------------------------------------------------------------------------------------------

    /// Pops an element off the stack, adds the current value of the `fmp` register to it, and
    /// pushes the result back onto the stack.
    ///
    /// # Errors
    /// Returns an error if the stack is empty.
    pub(super) fn op_fmpadd(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(1, "FMPADD")?;

        let offset = self.stack.get(0);
        let fmp = self.system.fmp();

        self.stack.set(0, fmp + offset);
        self.stack.copy_state(1);

        Ok(())
    }

    /// Pops an element off the stack and adds it to the current value of `fmp` register.
    ///
    /// # Errors
    /// Returns an error if:
    /// * The stack is empty.
    /// * New value of `fmp` register is greater than or equal to 2^32.
    pub(super) fn op_fmpupdate(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(1, "FMPUPDATE")?;

        let offset = self.stack.get(0);
        let fmp = self.system.fmp();

        let new_fmp = fmp + offset;
        if new_fmp.as_int() >= u32::MAX as u64 {
            return Err(ExecutionError::InvalidFmpValue(fmp, new_fmp));
        }

        self.system.set_fmp(new_fmp);
        self.stack.shift_left(1);

        Ok(())
    }
}
