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
    use rand_utils::rand_value;

    // ARITHMETIC OPERATIONS
    // --------------------------------------------------------------------------------------------

    #[test]
    fn op_add() {
        // initialize the stack with a few values
        let mut processor = Processor::new_dummy();
        let (c, b, a) = init_stack_rand(&mut processor);

        // add the top two values
        processor.execute_op(Operation::Add).unwrap();
        let expected = build_expected(&[a + b, c]);

        assert_eq!(2, processor.stack.depth());
        assert_eq!(4, processor.stack.current_step());
        assert_eq!(expected, processor.stack.trace_state());
    }

    #[test]
    fn op_neg() {
        // initialize the stack with a few values
        let mut processor = Processor::new_dummy();
        let (c, b, a) = init_stack_rand(&mut processor);

        // negate the top value
        processor.execute_op(Operation::Neg).unwrap();
        let expected = build_expected(&[-a, b, c]);

        assert_eq!(expected, processor.stack.trace_state());
        assert_eq!(3, processor.stack.depth());
        assert_eq!(4, processor.stack.current_step());
    }

    #[test]
    fn op_mul() {
        // initialize the stack with a few values
        let mut processor = Processor::new_dummy();
        let (c, b, a) = init_stack_rand(&mut processor);

        // add the top two values
        processor.execute_op(Operation::Mul).unwrap();
        let expected = build_expected(&[a * b, c]);

        assert_eq!(2, processor.stack.depth());
        assert_eq!(4, processor.stack.current_step());
        assert_eq!(expected, processor.stack.trace_state());
    }

    #[test]
    fn op_inv() {
        // initialize the stack with a few values
        let mut processor = Processor::new_dummy();
        let (c, b, a) = init_stack_rand(&mut processor);

        // invert the top value
        if b != BaseElement::ZERO {
            processor.execute_op(Operation::Inv).unwrap();
            let expected = build_expected(&[a.inv(), b, c]);

            assert_eq!(3, processor.stack.depth());
            assert_eq!(4, processor.stack.current_step());
            assert_eq!(expected, processor.stack.trace_state());
        }

        // inverting zero should be an error
        processor.execute_op(Operation::Pad).unwrap();
        assert!(processor.execute_op(Operation::Inv).is_err());
    }

    #[test]
    fn op_incr() {
        // initialize the stack with a few values
        let mut processor = Processor::new_dummy();
        let (c, b, a) = init_stack_rand(&mut processor);

        // negate the top value
        processor.execute_op(Operation::Incr).unwrap();
        let expected = build_expected(&[a + BaseElement::ONE, b, c]);

        assert_eq!(3, processor.stack.depth());
        assert_eq!(4, processor.stack.current_step());
        assert_eq!(expected, processor.stack.trace_state());
    }

    // BOOLEAN OPERATIONS
    // --------------------------------------------------------------------------------------------

    #[test]
    fn op_and() {
        // --- test 0 AND 0 ---------------------------------------------------
        let mut processor = Processor::new_dummy();
        init_stack_with(&mut processor, &[2, 0, 0]);

        processor.execute_op(Operation::And).unwrap();
        let expected = build_expected(&[BaseElement::ZERO, BaseElement::new(2)]);
        assert_eq!(expected, processor.stack.trace_state());

        // --- test 1 AND 0 ---------------------------------------------------
        let mut processor = Processor::new_dummy();
        init_stack_with(&mut processor, &[2, 0, 1]);

        processor.execute_op(Operation::And).unwrap();
        let expected = build_expected(&[BaseElement::ZERO, BaseElement::new(2)]);
        assert_eq!(expected, processor.stack.trace_state());

        // --- test 0 AND 1 ---------------------------------------------------
        let mut processor = Processor::new_dummy();
        init_stack_with(&mut processor, &[2, 1, 0]);

        processor.execute_op(Operation::And).unwrap();
        let expected = build_expected(&[BaseElement::ZERO, BaseElement::new(2)]);
        assert_eq!(expected, processor.stack.trace_state());

        // --- test 1 AND 0 ---------------------------------------------------
        let mut processor = Processor::new_dummy();
        init_stack_with(&mut processor, &[2, 1, 1]);

        processor.execute_op(Operation::And).unwrap();
        let expected = build_expected(&[BaseElement::ONE, BaseElement::new(2)]);
        assert_eq!(expected, processor.stack.trace_state());

        // --- first operand is not binary ------------------------------------
        let mut processor = Processor::new_dummy();
        init_stack_with(&mut processor, &[2, 1, 2]);
        assert!(processor.execute_op(Operation::And).is_err());

        // --- second operand is not binary ------------------------------------
        let mut processor = Processor::new_dummy();
        init_stack_with(&mut processor, &[2, 2, 1]);
        assert!(processor.execute_op(Operation::And).is_err());
    }

    #[test]
    fn op_not() {
        // --- test NOT 0 -----------------------------------------------------
        let mut processor = Processor::new_dummy();
        init_stack_with(&mut processor, &[2, 0]);

        processor.execute_op(Operation::Not).unwrap();
        let expected = build_expected(&[BaseElement::ONE, BaseElement::new(2)]);
        assert_eq!(expected, processor.stack.trace_state());

        // --- test NOT 1 ----------------------------------------------------
        let mut processor = Processor::new_dummy();
        init_stack_with(&mut processor, &[2, 1]);

        processor.execute_op(Operation::Not).unwrap();
        let expected = build_expected(&[BaseElement::ZERO, BaseElement::new(2)]);
        assert_eq!(expected, processor.stack.trace_state());

        // --- operand is not binary ------------------------------------------
        let mut processor = Processor::new_dummy();
        init_stack_with(&mut processor, &[2, 2]);
        assert!(processor.execute_op(Operation::Not).is_err());
    }

    // COMPARISON OPERATIONS
    // --------------------------------------------------------------------------------------------

    #[test]
    fn op_eq() {
        // --- test when top two values are equal -----------------------------
        let mut processor = Processor::new_dummy();
        init_stack_with(&mut processor, &[3, 7, 7]);

        processor.execute_op(Operation::Eq).unwrap();
        let expected = build_expected(&[BaseElement::ONE, BaseElement::new(3)]);
        assert_eq!(expected, processor.stack.trace_state());

        // --- test when top two values are not equal -------------------------
        let mut processor = Processor::new_dummy();
        init_stack_with(&mut processor, &[3, 5, 7]);

        processor.execute_op(Operation::Eq).unwrap();
        let expected = build_expected(&[BaseElement::ZERO, BaseElement::new(3)]);
        assert_eq!(expected, processor.stack.trace_state());
    }

    #[test]
    fn op_eqz() {
        // --- test when top is zero ------------------------------------------
        let mut processor = Processor::new_dummy();
        init_stack_with(&mut processor, &[3, 0]);

        processor.execute_op(Operation::Eqz).unwrap();
        let expected = build_expected(&[BaseElement::ONE, BaseElement::new(3)]);
        assert_eq!(expected, processor.stack.trace_state());

        // --- test when top is not zero --------------------------------------
        let mut processor = Processor::new_dummy();
        init_stack_with(&mut processor, &[3, 4]);

        processor.execute_op(Operation::Eqz).unwrap();
        let expected = build_expected(&[BaseElement::ZERO, BaseElement::new(3)]);
        assert_eq!(expected, processor.stack.trace_state());
    }

    #[test]
    fn op_eqw() {
        // --- test when top two words are equal ------------------------------
        let mut processor = Processor::new_dummy();
        let mut values = init_stack_with(&mut processor, &[1, 2, 3, 4, 5, 2, 3, 4, 5]);

        processor.execute_op(Operation::Eqw).unwrap();
        values.insert(0, BaseElement::ONE);
        let expected = build_expected(&values);
        assert_eq!(expected, processor.stack.trace_state());

        // --- test when top two words are not equal --------------------------
        let mut processor = Processor::new_dummy();
        let mut values = init_stack_with(&mut processor, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);

        processor.execute_op(Operation::Eqw).unwrap();
        values.insert(0, BaseElement::ZERO);
        let expected = build_expected(&values);
        assert_eq!(expected, processor.stack.trace_state());
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------

    fn init_stack_rand(processor: &mut Processor) -> (BaseElement, BaseElement, BaseElement) {
        // push values a and b onto the stack
        let a = rand_value();
        let b = rand_value();
        let c = rand_value();
        let values = init_stack_with(processor, &[a, b, c]);
        (values[2], values[1], values[0])
    }

    fn init_stack_with(processor: &mut Processor, values: &[u64]) -> Vec<BaseElement> {
        let mut result = Vec::with_capacity(values.len());
        for value in values.iter().map(|&v| BaseElement::new(v)) {
            processor.execute_op(Operation::Push(value)).unwrap();
            result.push(value);
        }

        assert_eq!(values.len(), processor.stack.depth());
        assert_eq!(values.len(), processor.stack.current_step());
        let rev_result: Vec<BaseElement> = result.iter().cloned().rev().collect();
        assert_eq!(build_expected(&rev_result), processor.stack.trace_state());

        rev_result
    }

    fn build_expected(values: &[BaseElement]) -> [BaseElement; 16] {
        let mut expected = [BaseElement::ZERO; 16];
        for (&value, result) in values.iter().zip(expected.iter_mut()) {
            *result = value;
        }
        expected
    }
}
