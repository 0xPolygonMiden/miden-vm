use super::{BaseElement, ExecutionError, Stack};

impl Stack {
    pub(super) fn op_push(&mut self, value: BaseElement) -> Result<(), ExecutionError> {
        self.trace[0][self.step] = value;
        self.shift_right(0);
        Ok(())
    }
}
