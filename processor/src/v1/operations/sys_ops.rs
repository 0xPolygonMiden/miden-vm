use super::{BaseElement, ExecutionError, FieldElement, Processor};

// INPUT OPERATIONS
// ================================================================================================

impl Processor {
    /// Pushes the provided value onto the stack.
    ///
    /// The original stack is shifted to the right by one item.
    pub(super) fn op_assert(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(1, "ASSERT")?;
        if self.stack.get(0) != BaseElement::ONE {
            return Err(ExecutionError::FailedAssertion(self.step));
        }
        self.stack.shift_left(1);
        Ok(())
    }
}
