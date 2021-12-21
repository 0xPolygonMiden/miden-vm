use super::{ExecutionError, Stack};

// FIELD OPERATIONS
// ================================================================================================

impl Stack {
    // ARITHMETIC OPERATIONS
    // --------------------------------------------------------------------------------------------
    /// Pops two elements off the stack, adds them together, and pushes the result back onto the
    /// stack.
    ///
    /// # Errors
    /// Returns an error if the stack contains fewer than two elements.
    pub(super) fn op_add(&mut self) -> Result<(), ExecutionError> {
        if self.depth < 2 {
            return Err(ExecutionError::StackUnderflow("ADD", self.step));
        }

        let b = self.trace[0][self.step];
        let a = self.trace[1][self.step];
        self.trace[0][self.step + 1] = a + b;
        self.shift_left(2);
        Ok(())
    }

    pub(super) fn op_neg(&mut self) -> Result<(), ExecutionError> {
        unimplemented!()
    }

    /// Pops two elements off the stack, multiplies them, and pushes the result back onto the
    /// stack.
    ///
    /// # Errors
    /// Returns an error if the stack contains fewer than two elements.
    pub(super) fn op_mul(&mut self) -> Result<(), ExecutionError> {
        if self.depth < 2 {
            return Err(ExecutionError::StackUnderflow("MUL", self.step));
        }

        let b = self.trace[0][self.step];
        let a = self.trace[1][self.step];
        self.trace[0][self.step + 1] = a * b;
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

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        super::{BaseElement, FieldElement, Operation},
        Stack,
    };

    #[test]
    fn op_add() {
        // initialize the stack with two values
        let mut stack = Stack::new(2);
        let (a, b) = init_stack(&mut stack);

        // add the values
        stack.execute(Operation::Add).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[0] = a + b;

        assert_eq!(1, stack.depth());
        assert_eq!(3, stack.current_step());
        assert_eq!(expected, stack.trace_state());
    }

    #[test]
    fn op_mul() {
        // initialize the stack with two values
        let mut stack = Stack::new(2);
        let (a, b) = init_stack(&mut stack);

        // add the values
        stack.execute(Operation::Mul).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[0] = a * b;

        assert_eq!(1, stack.depth());
        assert_eq!(3, stack.current_step());
        assert_eq!(expected, stack.trace_state());
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------

    fn init_stack(stack: &mut Stack) -> (BaseElement, BaseElement) {
        let a = BaseElement::new(3);
        let b = BaseElement::new(7);

        // push values a and b onto the stack
        stack.execute(Operation::Push(a)).unwrap();
        stack.execute(Operation::Push(b)).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[0] = b;
        expected[1] = a;

        assert_eq!(2, stack.depth());
        assert_eq!(2, stack.current_step());
        assert_eq!(expected, stack.trace_state());

        (a, b)
    }
}
