use super::{utils::assert_binary, BaseElement, ExecutionError, FieldElement, Processor};

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

    /// Pops an element off the stack, computes its additive inverse, and pushes the result back
    /// onto the stack.
    ///
    /// # Errors
    /// Returns an error if the stack is empty.
    pub(super) fn op_neg(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(1, "NEG")?;

        let a = self.stack.get(0);
        self.stack.set(0, -a);
        self.stack.copy_state(1);
        Ok(())
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

    /// Pops an element off the stack, computes its multiplicative inverse, and pushes the result
    /// back onto the stack.
    ///
    /// # Errors
    /// Returns an error if:
    /// * The stack is empty.
    /// * The value on the top of the stack is ZERO.
    pub(super) fn op_inv(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(1, "INV")?;

        let a = self.stack.get(0);
        if a == BaseElement::ZERO {
            return Err(ExecutionError::DivideByZero(self.step));
        }

        self.stack.set(0, a.inv());
        self.stack.copy_state(1);
        Ok(())
    }

    /// Pops an element off the stack, adds ONE to it, and pushes the result back onto the stack.
    ///
    /// # Errors
    /// Returns an error if the stack is empty.
    pub(super) fn op_incr(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(1, "INCR")?;

        let a = self.stack.get(0);
        self.stack.set(0, a + BaseElement::ONE);
        self.stack.copy_state(1);
        Ok(())
    }

    // BOOLEAN OPERATIONS
    // --------------------------------------------------------------------------------------------

    /// Pops two elements off the stack, computes their boolean AND, and pushes the result back
    /// onto the stack.
    ///
    /// # Errors
    /// Returns an error if:
    /// * The stack contains fewer than two elements.
    /// * Either of the two elements on the top of the stack is not a binary value.
    pub(super) fn op_and(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(2, "AND")?;

        let b = assert_binary(self.stack.get(0))?;
        let a = assert_binary(self.stack.get(1))?;
        if a == BaseElement::ONE && b == BaseElement::ONE {
            self.stack.set(0, BaseElement::ONE);
        } else {
            self.stack.set(0, BaseElement::ZERO);
        }
        self.stack.shift_left(2);
        Ok(())
    }

    /// Pops two elements off the stack, computes their boolean OR, and pushes the result back
    /// onto the stack.
    ///
    /// # Errors
    /// Returns an error if:
    /// * The stack contains fewer than two elements.
    /// * Either of the two elements on the top of the stack is not a binary value.
    pub(super) fn op_or(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(2, "OR")?;

        let b = assert_binary(self.stack.get(0))?;
        let a = assert_binary(self.stack.get(1))?;
        if a == BaseElement::ONE || b == BaseElement::ONE {
            self.stack.set(0, BaseElement::ONE);
        } else {
            self.stack.set(0, BaseElement::ZERO);
        }
        self.stack.shift_left(2);
        Ok(())
    }

    /// Pops an element off the stack, computes its boolean NOT, and pushes the result back onto
    /// the stack.
    ///
    /// # Errors
    /// Returns an error if:
    /// * The stack is empty.
    /// * The value on the top of the stack is not a binary value.
    pub(super) fn op_not(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(1, "NOT")?;

        let a = assert_binary(self.stack.get(0))?;
        self.stack.set(0, BaseElement::ONE - a);
        self.stack.copy_state(1);
        Ok(())
    }

    // COMPARISON OPERATIONS
    // --------------------------------------------------------------------------------------------

    /// Pops two elements off the stack and compares them. If the elements are equal, pushes ONE
    /// onto the stack, otherwise pushes ZERO onto the stack.
    ///
    /// # Errors
    /// Returns an error if the stack contains fewer than two elements.
    pub(super) fn op_eq(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(2, "EQ")?;

        let b = self.stack.get(0);
        let a = self.stack.get(1);
        if a == b {
            self.stack.set(0, BaseElement::ONE);
        } else {
            self.stack.set(0, BaseElement::ZERO);
        }
        self.stack.shift_left(2);
        Ok(())
    }

    /// Pops an element off the stack and compares it to ZERO. If the element is ZERO, pushes ONE
    /// onto the stack, otherwise pushes ZERO onto the stack.
    ///
    /// # Errors
    /// Returns an error if the stack is empty.
    pub(super) fn op_eqz(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(1, "EQZ")?;

        let a = self.stack.get(0);
        if a == BaseElement::ZERO {
            self.stack.set(0, BaseElement::ONE);
        } else {
            self.stack.set(0, BaseElement::ZERO);
        }
        self.stack.copy_state(1);
        Ok(())
    }

    /// Compares the first word (four elements) with the second word on the stack, if the words are
    /// equal, pushes ONE onto the stack, otherwise pushes ZERO onto the stack.
    ///
    /// # Errors
    /// Returns an error if the stack contains fewer than 8 elements.
    pub(super) fn op_eqw(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(8, "EQW")?;

        let b3 = self.stack.get(0);
        let b2 = self.stack.get(1);
        let b1 = self.stack.get(2);
        let b0 = self.stack.get(3);

        let a3 = self.stack.get(4);
        let a2 = self.stack.get(5);
        let a1 = self.stack.get(6);
        let a0 = self.stack.get(7);

        if a0 == b0 && a1 == b1 && a2 == b2 && a3 == b3 {
            self.stack.set(0, BaseElement::ONE);
        } else {
            self.stack.set(0, BaseElement::ZERO);
        }
        self.stack.shift_right(0);
        Ok(())
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
