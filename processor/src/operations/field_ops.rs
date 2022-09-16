use super::{utils::assert_binary, ExecutionError, Felt, FieldElement, Process};
use vm_core::{Operation, ZERO};

// FIELD OPERATIONS
// ================================================================================================

impl Process {
    // ARITHMETIC OPERATIONS
    // --------------------------------------------------------------------------------------------
    /// Pops two elements off the stack, adds them together, and pushes the result back onto the
    /// stack.
    pub(super) fn op_add(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack.get(0);
        let a = self.stack.get(1);
        self.stack.set(0, a + b);
        self.stack.shift_left(2);
        Ok(())
    }

    /// Pops an element off the stack, computes its additive inverse, and pushes the result back
    /// onto the stack.
    pub(super) fn op_neg(&mut self) -> Result<(), ExecutionError> {
        let a = self.stack.get(0);
        self.stack.set(0, -a);
        self.stack.copy_state(1);
        Ok(())
    }

    /// Pops two elements off the stack, multiplies them, and pushes the result back onto the
    /// stack.
    pub(super) fn op_mul(&mut self) -> Result<(), ExecutionError> {
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
    /// Returns an error if the value on the top of the stack is ZERO.
    pub(super) fn op_inv(&mut self) -> Result<(), ExecutionError> {
        let a = self.stack.get(0);
        if a == Felt::ZERO {
            return Err(ExecutionError::DivideByZero(self.system.clk()));
        }

        self.stack.set(0, a.inv());
        self.stack.copy_state(1);
        Ok(())
    }

    /// Pops an element off the stack, adds ONE to it, and pushes the result back onto the stack.
    pub(super) fn op_incr(&mut self) -> Result<(), ExecutionError> {
        let a = self.stack.get(0);
        self.stack.set(0, a + Felt::ONE);
        self.stack.copy_state(1);
        Ok(())
    }

    // BOOLEAN OPERATIONS
    // --------------------------------------------------------------------------------------------

    /// Pops two elements off the stack, computes their boolean AND, and pushes the result back
    /// onto the stack.
    ///
    /// # Errors
    /// Returns an error if either of the two elements on the top of the stack is not a binary
    /// value.
    pub(super) fn op_and(&mut self) -> Result<(), ExecutionError> {
        let b = assert_binary(self.stack.get(0))?;
        let a = assert_binary(self.stack.get(1))?;
        if a == Felt::ONE && b == Felt::ONE {
            self.stack.set(0, Felt::ONE);
        } else {
            self.stack.set(0, Felt::ZERO);
        }
        self.stack.shift_left(2);
        Ok(())
    }

    /// Pops two elements off the stack, computes their boolean OR, and pushes the result back
    /// onto the stack.
    ///
    /// # Errors
    /// Returns an error if either of the two elements on the top of the stack is not a binary
    /// value.
    pub(super) fn op_or(&mut self) -> Result<(), ExecutionError> {
        let b = assert_binary(self.stack.get(0))?;
        let a = assert_binary(self.stack.get(1))?;
        if a == Felt::ONE || b == Felt::ONE {
            self.stack.set(0, Felt::ONE);
        } else {
            self.stack.set(0, Felt::ZERO);
        }
        self.stack.shift_left(2);
        Ok(())
    }

    /// Pops an element off the stack, computes its boolean NOT, and pushes the result back onto
    /// the stack.
    ///
    /// # Errors
    /// Returns an error if the value on the top of the stack is not a binary value.
    pub(super) fn op_not(&mut self) -> Result<(), ExecutionError> {
        let a = assert_binary(self.stack.get(0))?;
        self.stack.set(0, Felt::ONE - a);
        self.stack.copy_state(1);
        Ok(())
    }

    // COMPARISON OPERATIONS
    // --------------------------------------------------------------------------------------------

    /// Pops two elements off the stack and compares them. If the elements are equal, pushes ONE
    /// onto the stack, otherwise pushes ZERO onto the stack.
    pub(super) fn op_eq(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack.get(0);
        let a = self.stack.get(1);

        // helper variable provided by the prover. If top elements are same, then, it can be set to anything
        // otherwise set it to the reciprocal of the difference between the top two elements.
        let mut h0 = ZERO;

        if a == b {
            self.stack.set(0, Felt::ONE);
        } else {
            self.stack.set(0, Felt::ZERO);
            // setting h0 to the inverse of the difference between the top two elements of the stack.
            h0 = (b - a).inv();
        }

        // save h0 in the decoder helper register.
        self.decoder.set_user_op_helpers(Operation::Eq, &[h0]);

        self.stack.shift_left(2);
        Ok(())
    }

    /// Pops an element off the stack and compares it to ZERO. If the element is ZERO, pushes ONE
    /// onto the stack, otherwise pushes ZERO onto the stack.
    pub(super) fn op_eqz(&mut self) -> Result<(), ExecutionError> {
        let a = self.stack.get(0);

        // helper variable provided by the prover. If the top element is zero, then, h0 can be set to anything
        // otherwise set it to the inverse of the top element in the stack.
        let mut h0 = ZERO;

        if a == Felt::ZERO {
            self.stack.set(0, Felt::ONE);
        } else {
            // setting h0 to the inverse of the top element of the stack.
            h0 = a.inv();
            self.stack.set(0, Felt::ZERO);
        }

        // save h0 in the decoder helper register.
        self.decoder.set_user_op_helpers(Operation::Eq, &[h0]);

        self.stack.copy_state(1);
        Ok(())
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        super::{init_stack_with, Felt, FieldElement, Operation, STACK_TOP_SIZE},
        Process,
    };
    use rand_utils::rand_value;

    // ARITHMETIC OPERATIONS
    // --------------------------------------------------------------------------------------------

    #[test]
    fn op_add() {
        // initialize the stack with a few values
        let mut process = Process::new_dummy();
        let (a, b, c) = init_stack_rand(&mut process);

        // add the top two values
        process.execute_op(Operation::Add).unwrap();
        let expected = build_expected(&[a + b, c]);

        assert_eq!(STACK_TOP_SIZE + 2, process.stack.depth());
        assert_eq!(4, process.stack.current_clk());
        assert_eq!(expected, process.stack.trace_state());

        // calling add with a stack of minimum depth is ok
        let mut process = Process::new_dummy();
        assert!(process.execute_op(Operation::Add).is_ok());
    }

    #[test]
    fn op_neg() {
        // initialize the stack with a few values
        let mut process = Process::new_dummy();
        let (a, b, c) = init_stack_rand(&mut process);

        // negate the top value
        process.execute_op(Operation::Neg).unwrap();
        let expected = build_expected(&[-a, b, c]);

        assert_eq!(expected, process.stack.trace_state());
        assert_eq!(STACK_TOP_SIZE + 3, process.stack.depth());
        assert_eq!(4, process.stack.current_clk());
    }

    #[test]
    fn op_mul() {
        // initialize the stack with a few values
        let mut process = Process::new_dummy();
        let (a, b, c) = init_stack_rand(&mut process);

        // add the top two values
        process.execute_op(Operation::Mul).unwrap();
        let expected = build_expected(&[a * b, c]);

        assert_eq!(STACK_TOP_SIZE + 2, process.stack.depth());
        assert_eq!(4, process.stack.current_clk());
        assert_eq!(expected, process.stack.trace_state());

        // calling mul with a stack of minimum depth is ok
        let mut process = Process::new_dummy();
        assert!(process.execute_op(Operation::Mul).is_ok());
    }

    #[test]
    fn op_inv() {
        // initialize the stack with a few values
        let mut process = Process::new_dummy();
        let (a, b, c) = init_stack_rand(&mut process);

        // invert the top value
        if b != Felt::ZERO {
            process.execute_op(Operation::Inv).unwrap();
            let expected = build_expected(&[a.inv(), b, c]);

            assert_eq!(STACK_TOP_SIZE + 3, process.stack.depth());
            assert_eq!(4, process.stack.current_clk());
            assert_eq!(expected, process.stack.trace_state());
        }

        // inverting zero should be an error
        process.execute_op(Operation::Pad).unwrap();
        assert!(process.execute_op(Operation::Inv).is_err());
    }

    #[test]
    fn op_incr() {
        // initialize the stack with a few values
        let mut process = Process::new_dummy();
        let (a, b, c) = init_stack_rand(&mut process);

        // negate the top value
        process.execute_op(Operation::Incr).unwrap();
        let expected = build_expected(&[a + Felt::ONE, b, c]);

        assert_eq!(STACK_TOP_SIZE + 3, process.stack.depth());
        assert_eq!(4, process.stack.current_clk());
        assert_eq!(expected, process.stack.trace_state());
    }

    // BOOLEAN OPERATIONS
    // --------------------------------------------------------------------------------------------

    #[test]
    fn op_and() {
        // --- test 0 AND 0 ---------------------------------------------------
        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[2, 0, 0]);

        process.execute_op(Operation::And).unwrap();
        let expected = build_expected(&[Felt::ZERO, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test 1 AND 0 ---------------------------------------------------
        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[2, 0, 1]);

        process.execute_op(Operation::And).unwrap();
        let expected = build_expected(&[Felt::ZERO, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test 0 AND 1 ---------------------------------------------------
        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[2, 1, 0]);

        process.execute_op(Operation::And).unwrap();
        let expected = build_expected(&[Felt::ZERO, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test 1 AND 1 ---------------------------------------------------
        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[2, 1, 1]);

        process.execute_op(Operation::And).unwrap();
        let expected = build_expected(&[Felt::ONE, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- first operand is not binary ------------------------------------
        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[2, 1, 2]);
        assert!(process.execute_op(Operation::And).is_err());

        // --- second operand is not binary -----------------------------------
        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[2, 2, 1]);
        assert!(process.execute_op(Operation::And).is_err());

        // --- calling AND with a stack of minimum depth is ok ----------------
        let mut process = Process::new_dummy();
        assert!(process.execute_op(Operation::And).is_ok());
    }

    #[test]
    fn op_or() {
        // --- test 0 OR 0 ---------------------------------------------------
        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[2, 0, 0]);

        process.execute_op(Operation::Or).unwrap();
        let expected = build_expected(&[Felt::ZERO, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test 1 OR 0 ---------------------------------------------------
        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[2, 0, 1]);

        process.execute_op(Operation::Or).unwrap();
        let expected = build_expected(&[Felt::ONE, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test 0 OR 1 ---------------------------------------------------
        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[2, 1, 0]);

        process.execute_op(Operation::Or).unwrap();
        let expected = build_expected(&[Felt::ONE, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test 1 OR 0 ---------------------------------------------------
        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[2, 1, 1]);

        process.execute_op(Operation::Or).unwrap();
        let expected = build_expected(&[Felt::ONE, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- first operand is not binary ------------------------------------
        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[2, 1, 2]);
        assert!(process.execute_op(Operation::Or).is_err());

        // --- second operand is not binary -----------------------------------
        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[2, 2, 1]);
        assert!(process.execute_op(Operation::Or).is_err());

        // --- calling OR with a stack of minimum depth is a ok ----------------
        let mut process = Process::new_dummy();
        assert!(process.execute_op(Operation::Or).is_ok());
    }

    #[test]
    fn op_not() {
        // --- test NOT 0 -----------------------------------------------------
        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[2, 0]);

        process.execute_op(Operation::Not).unwrap();
        let expected = build_expected(&[Felt::ONE, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test NOT 1 ----------------------------------------------------
        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[2, 1]);

        process.execute_op(Operation::Not).unwrap();
        let expected = build_expected(&[Felt::ZERO, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- operand is not binary ------------------------------------------
        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[2, 2]);
        assert!(process.execute_op(Operation::Not).is_err());
    }

    // COMPARISON OPERATIONS
    // --------------------------------------------------------------------------------------------

    #[test]
    fn op_eq() {
        // --- test when top two values are equal -----------------------------
        let inputs = ProgramInputs::new(&[3, 7, 7], &[], (&[]).to_vec()).unwrap();
        let mut process = Process::new_dummy_with_inputs_and_decoder_helpers(inputs);

        process.execute_op(Operation::Eq).unwrap();
        let expected = build_expected(&[Felt::ONE, Felt::new(3)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test when top two values are not equal -------------------------
        let inputs = ProgramInputs::new(&[3, 5, 7], &[], (&[]).to_vec()).unwrap();
        let mut process = Process::new_dummy_with_inputs_and_decoder_helpers(inputs);

        process.execute_op(Operation::Eq).unwrap();
        let expected = build_expected(&[Felt::ZERO, Felt::new(3)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- calling EQ with a stack of minimum depth is a ok ---------------
        let inputs = ProgramInputs::new(&[], &[], (&[]).to_vec()).unwrap();
        let mut process = Process::new_dummy_with_inputs_and_decoder_helpers(inputs);
        assert!(process.execute_op(Operation::Eq).is_ok());
    }

    #[test]
    fn op_eqz() {
        // --- test when top is zero ------------------------------------------
        let inputs = ProgramInputs::new(&[3, 0], &[], (&[]).to_vec()).unwrap();
        let mut process = Process::new_dummy_with_inputs_and_decoder_helpers(inputs);

        process.execute_op(Operation::Eqz).unwrap();
        let expected = build_expected(&[Felt::ONE, Felt::new(3)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test when top is not zero --------------------------------------
        let inputs = ProgramInputs::new(&[3, 4], &[], (&[]).to_vec()).unwrap();
        let mut process = Process::new_dummy_with_inputs_and_decoder_helpers(inputs);

        process.execute_op(Operation::Eqz).unwrap();
        let expected = build_expected(&[Felt::ZERO, Felt::new(3)]);
        assert_eq!(expected, process.stack.trace_state());
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------

    fn init_stack_rand(process: &mut Process) -> (Felt, Felt, Felt) {
        // push values a and b onto the stack
        let a = rand_value();
        let b = rand_value();
        let c = rand_value();
        init_stack_with(process, &[a, b, c]);
        (Felt::new(c), Felt::new(b), Felt::new(a))
    }

    fn build_expected(values: &[Felt]) -> [Felt; 16] {
        let mut expected = [Felt::ZERO; 16];
        for (&value, result) in values.iter().zip(expected.iter_mut()) {
            *result = value;
        }
        expected
    }
}
