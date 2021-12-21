use super::{ExecutionError, Stack};

impl Stack {
    // ARITHMETIC OPERATIONS
    // --------------------------------------------------------------------------------------------
    pub(super) fn op_add(&mut self) -> Result<(), ExecutionError> {
        if self.depth < 2 {
            return Err(ExecutionError::StackUnderflow("ADD", self.step));
        }

        let a = self.trace[0][self.step - 1];
        let b = self.trace[1][self.step - 1];

        self.trace[0][self.step] = a + b;
        self.shift_left(2);
        Ok(())
    }

    pub(super) fn op_neg(&mut self) -> Result<(), ExecutionError> {
        unimplemented!()
    }

    pub(super) fn op_mul(&mut self) -> Result<(), ExecutionError> {
        if self.depth < 2 {
            return Err(ExecutionError::StackUnderflow("MUL", self.step));
        }

        let a = self.trace[0][self.step - 1];
        let b = self.trace[1][self.step - 1];
        self.trace[0][self.step] = a * b;
        self.shift_left(2);
        Ok(())
    }

    pub(super) fn op_inv(&mut self) -> Result<(), ExecutionError> {
        unimplemented!()
    }

    pub(super) fn op_incr(&mut self) -> Result<(), ExecutionError> {
        unimplemented!()
    }

    // BOOLEAN OPERATIONS
    // --------------------------------------------------------------------------------------------

    pub(super) fn op_and(&mut self) -> Result<(), ExecutionError> {
        unimplemented!()
    }

    pub(super) fn op_or(&mut self) -> Result<(), ExecutionError> {
        unimplemented!()
    }

    pub(super) fn op_not(&mut self) -> Result<(), ExecutionError> {
        unimplemented!()
    }

    // COMPARISON OPERATIONS
    // --------------------------------------------------------------------------------------------

    pub(super) fn op_eq(&mut self) -> Result<(), ExecutionError> {
        unimplemented!()
    }

    pub(super) fn op_eqz(&mut self) -> Result<(), ExecutionError> {
        unimplemented!()
    }
}
