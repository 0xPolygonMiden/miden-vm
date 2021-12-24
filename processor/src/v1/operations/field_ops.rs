use super::{ExecutionError, Processor};

// FIELD OPERATIONS
// ================================================================================================

impl Processor {
    // ARITHMETIC OPERATIONS
    // --------------------------------------------------------------------------------------------
    /// Pops two elements off the stack, adds them together, and pushes the result back onto the
    /// stack.
    ///
    /// # Errors
    /// Returns an error if the stack contains fewer than two elements.
    pub(super) fn op_add(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(2, "ADD")?;

        let b = self.stack.get(0);
        let a = self.stack.get(1);
        self.stack.set(0, a + b);
        self.stack.shift_left(2);
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
        self.stack.check_depth(2, "MUL")?;

        let b = self.stack.get(0);
        let a = self.stack.get(1);
        self.stack.set(0, a * b);
        self.stack.shift_left(2);
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
        Processor,
    };

    #[test]
    fn op_add() {
        // initialize the stack with two values
        let mut processor = Processor::new_dummy();
        let (a, b) = init_stack(&mut processor);

        // add the values
        processor.execute_op(Operation::Add).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[0] = a + b;

        assert_eq!(1, processor.stack.depth());
        assert_eq!(3, processor.stack.current_step());
        assert_eq!(expected, processor.stack.trace_state());
    }

    #[test]
    fn op_mul() {
        // initialize the stack with two values
        let mut processor = Processor::new_dummy();
        let (a, b) = init_stack(&mut processor);

        // add the values
        processor.execute_op(Operation::Mul).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[0] = a * b;

        assert_eq!(1, processor.stack.depth());
        assert_eq!(3, processor.stack.current_step());
        assert_eq!(expected, processor.stack.trace_state());
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------

    fn init_stack(processor: &mut Processor) -> (BaseElement, BaseElement) {
        let a = BaseElement::new(3);
        let b = BaseElement::new(7);

        // push values a and b onto the stack
        processor.execute_op(Operation::Push(a)).unwrap();
        processor.execute_op(Operation::Push(b)).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[0] = b;
        expected[1] = a;

        assert_eq!(2, processor.stack.depth());
        assert_eq!(2, processor.stack.current_step());
        assert_eq!(expected, processor.stack.trace_state());

        (a, b)
    }
}
