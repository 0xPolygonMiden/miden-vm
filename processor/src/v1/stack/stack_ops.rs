use super::{BaseElement, ExecutionError, FieldElement, Stack};

impl Stack {
    // STACK MANIPULATION
    // --------------------------------------------------------------------------------------------
    pub(super) fn op_pad(&mut self) -> Result<(), ExecutionError> {
        self.trace[0][self.step] = BaseElement::ZERO;
        self.shift_right(0);
        Ok(())
    }

    pub(super) fn op_drop(&mut self) -> Result<(), ExecutionError> {
        if self.depth == 0 {
            return Err(ExecutionError::StackUnderflow("DROP", self.step));
        }
        self.shift_left(1);
        Ok(())
    }
}
