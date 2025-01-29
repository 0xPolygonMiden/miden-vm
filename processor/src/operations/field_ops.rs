use vm_core::{Operation, ONE, ZERO};

use super::{utils::assert_binary, ExecutionError, Felt, FieldElement, Process};

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
        self.stack.pop_and_set([a + b]);
        Ok(())
    }

    /// Pops an element off the stack, computes its additive inverse, and pushes the result back
    /// onto the stack.
    pub(super) fn op_neg(&mut self) -> Result<(), ExecutionError> {
        let a = self.stack.get(0);
        self.stack.set_and_copy([-a]);
        Ok(())
    }

    /// Pops two elements off the stack, multiplies them, and pushes the result back onto the
    /// stack.
    pub(super) fn op_mul(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack.get(0);
        let a = self.stack.get(1);
        self.stack.pop_and_set([a * b]);
        Ok(())
    }

    /// Pops an element off the stack, computes its multiplicative inverse, and pushes the result
    /// back onto the stack.
    ///
    /// # Errors
    /// Returns an error if the value on the top of the stack is ZERO.
    pub(super) fn op_inv(&mut self) -> Result<(), ExecutionError> {
        let a = self.stack.get(0);
        if a == ZERO {
            return Err(ExecutionError::DivideByZero(self.system.clk()));
        }

        self.stack.set_and_copy([a.inv()]);
        Ok(())
    }

    /// Pops an element off the stack, adds ONE to it, and pushes the result back onto the stack.
    pub(super) fn op_incr(&mut self) -> Result<(), ExecutionError> {
        let a = self.stack.get(0);

        self.stack.set_and_copy([a + ONE]);

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
        if a == ONE && b == ONE {
            self.stack.pop_and_set([ONE]);
        } else {
            self.stack.pop_and_set([ZERO]);
        }
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
        if a == ONE || b == ONE {
            self.stack.pop_and_set([ONE]);
        } else {
            self.stack.pop_and_set([ZERO]);
        }
        Ok(())
    }

    /// Pops an element off the stack, computes its boolean NOT, and pushes the result back onto
    /// the stack.
    ///
    /// # Errors
    /// Returns an error if the value on the top of the stack is not a binary value.
    pub(super) fn op_not(&mut self) -> Result<(), ExecutionError> {
        let a = assert_binary(self.stack.get(0))?;

        self.stack.set_and_copy([ONE - a]);

        Ok(())
    }

    // COMPARISON OPERATIONS
    // --------------------------------------------------------------------------------------------

    /// Pops two elements off the stack and compares them. If the elements are equal, pushes ONE
    /// onto the stack, otherwise pushes ZERO onto the stack.
    pub(super) fn op_eq(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack.get(0);
        let a = self.stack.get(1);

        // helper variable provided by the prover. If top elements are same, then, it can be set to
        // anything otherwise set it to the reciprocal of the difference between the top two
        // elements.
        let mut h0 = ZERO;

        if a == b {
            self.stack.pop_and_set([ONE]);
        } else {
            self.stack.pop_and_set([ZERO]);
            // setting h0 to the inverse of the difference between the top two elements of the
            // stack.
            h0 = (b - a).inv();
        }

        // save h0 in the decoder helper register.
        self.decoder.set_user_op_helpers(Operation::Eq, &[h0]);

        Ok(())
    }

    /// Pops an element off the stack and compares it to ZERO. If the element is ZERO, pushes ONE
    /// onto the stack, otherwise pushes ZERO onto the stack.
    pub(super) fn op_eqz(&mut self) -> Result<(), ExecutionError> {
        let a = self.stack.get(0);

        // helper variable provided by the prover. If the top element is zero, then, h0 can be set
        // to anything otherwise set it to the inverse of the top element in the stack.
        let mut h0 = ZERO;

        if a == ZERO {
            self.stack.set_and_copy([ONE]);
        } else {
            self.stack.set_and_copy([ZERO]);

            // setting h0 to the inverse of the top element of the stack.
            h0 = a.inv();
        }

        // save h0 in the decoder helper register.
        self.decoder.set_user_op_helpers(Operation::Eq, &[h0]);

        Ok(())
    }

    /// Computes a single turn of exp accumulation for the given inputs. The top 4 elements in the
    /// stack is arranged as follows (from the top):
    /// - least significant bit of the exponent in the previous trace if there's an expacc call,
    ///   otherwise ZERO
    /// - exponent of base for this turn
    /// - accumulated power of base so far
    /// - number which needs to be shifted to the right
    ///
    /// To perform the operation we do the following:
    /// 1. Pops top three elements off the stack and calculate the least significant bit of the
    ///    number `b`.
    /// 2. Use this bit to decide if the current `base` raise to the power exponent needs to be
    ///    included in the accumulator.
    /// 3. Update exponent with its square and the number b with one right shift.
    /// 4. Pushes the calcuted new values to the stack in the mentioned order.
    pub(super) fn op_expacc(&mut self) -> Result<(), ExecutionError> {
        let mut exp = self.stack.get(1);
        let mut acc = self.stack.get(2);
        let mut b = self.stack.get(3);

        // least significant bit of the number b.
        let bit = b.as_int() & 1;

        // value which would be incorporated in the accumulator.
        let value = Felt::new((exp.as_int() - 1) * bit + 1);

        // current value of acc after including the value based on whether the bit is
        // 1 or not.
        acc *= value;

        // number `b` shifted right by one bit.
        b = Felt::new(b.as_int() >> 1);

        // exponent updated with its square.
        exp *= exp;

        // save val in the decoder helper register.
        self.decoder.set_user_op_helpers(Operation::Expacc, &[value]);

        self.stack.set_and_copy([Felt::new(bit), exp, acc, b]);

        Ok(())
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use test_utils::rand::rand_value;
    use vm_core::{ONE, ZERO};

    use super::{
        super::{Felt, FieldElement, Operation, MIN_STACK_DEPTH},
        Process,
    };
    use crate::{AdviceInputs, DefaultHost, StackInputs};

    // ARITHMETIC OPERATIONS
    // --------------------------------------------------------------------------------------------

    #[test]
    fn op_add() {
        // initialize the stack with a few values
        let (a, b, c) = get_rand_values();
        let stack = StackInputs::try_from_ints([c.as_int(), b.as_int(), a.as_int()]).unwrap();
        let mut process = Process::new_dummy(stack);
        let mut host = DefaultHost::default();

        // add the top two values
        process.execute_op(Operation::Add, &mut host).unwrap();
        let expected = build_expected(&[a + b, c]);

        assert_eq!(MIN_STACK_DEPTH, process.stack.depth());
        assert_eq!(2, process.stack.current_clk());
        assert_eq!(expected, process.stack.trace_state());

        // calling add with a stack of minimum depth is ok
        let mut process = Process::new_dummy_with_empty_stack();
        assert!(process.execute_op(Operation::Add, &mut host).is_ok());
    }

    #[test]
    fn op_neg() {
        // initialize the stack with a few values
        let (a, b, c) = get_rand_values();
        let stack = StackInputs::try_from_ints([c.as_int(), b.as_int(), a.as_int()]).unwrap();
        let mut process = Process::new_dummy(stack);
        let mut host = DefaultHost::default();

        // negate the top value
        process.execute_op(Operation::Neg, &mut host).unwrap();
        let expected = build_expected(&[-a, b, c]);

        assert_eq!(expected, process.stack.trace_state());
        assert_eq!(MIN_STACK_DEPTH, process.stack.depth());
        assert_eq!(2, process.stack.current_clk());
    }

    #[test]
    fn op_mul() {
        // initialize the stack with a few values
        let (a, b, c) = get_rand_values();
        let stack = StackInputs::try_from_ints([c.as_int(), b.as_int(), a.as_int()]).unwrap();
        let mut process = Process::new_dummy(stack);
        let mut host = DefaultHost::default();

        // add the top two values
        process.execute_op(Operation::Mul, &mut host).unwrap();
        let expected = build_expected(&[a * b, c]);

        assert_eq!(MIN_STACK_DEPTH, process.stack.depth());
        assert_eq!(2, process.stack.current_clk());
        assert_eq!(expected, process.stack.trace_state());

        // calling mul with a stack of minimum depth is ok
        let mut process = Process::new_dummy_with_empty_stack();
        assert!(process.execute_op(Operation::Mul, &mut host).is_ok());
    }

    #[test]
    fn op_inv() {
        // initialize the stack with a few values
        let (a, b, c) = get_rand_values();
        let stack = StackInputs::try_from_ints([c.as_int(), b.as_int(), a.as_int()]).unwrap();
        let mut process = Process::new_dummy(stack);
        let mut host = DefaultHost::default();

        // invert the top value
        if b != ZERO {
            process.execute_op(Operation::Inv, &mut host).unwrap();
            let expected = build_expected(&[a.inv(), b, c]);

            assert_eq!(MIN_STACK_DEPTH, process.stack.depth());
            assert_eq!(2, process.stack.current_clk());
            assert_eq!(expected, process.stack.trace_state());
        }

        // inverting zero should be an error
        process.execute_op(Operation::Pad, &mut host).unwrap();
        assert!(process.execute_op(Operation::Inv, &mut host).is_err());
    }

    #[test]
    fn op_incr() {
        // initialize the stack with a few values
        let (a, b, c) = get_rand_values();
        let stack = StackInputs::try_from_ints([c.as_int(), b.as_int(), a.as_int()]).unwrap();
        let mut process = Process::new_dummy(stack);
        let mut host = DefaultHost::default();

        // negate the top value
        process.execute_op(Operation::Incr, &mut host).unwrap();
        let expected = build_expected(&[a + ONE, b, c]);

        assert_eq!(MIN_STACK_DEPTH, process.stack.depth());
        assert_eq!(2, process.stack.current_clk());
        assert_eq!(expected, process.stack.trace_state());
    }

    // BOOLEAN OPERATIONS
    // --------------------------------------------------------------------------------------------

    #[test]
    fn op_and() {
        let mut host = DefaultHost::default();
        // --- test 0 AND 0 ---------------------------------------------------
        let stack = StackInputs::try_from_ints([2, 0, 0]).unwrap();
        let mut process = Process::new_dummy(stack);

        process.execute_op(Operation::And, &mut host).unwrap();
        let expected = build_expected(&[ZERO, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test 1 AND 0 ---------------------------------------------------
        let stack = StackInputs::try_from_ints([2, 0, 1]).unwrap();
        let mut process = Process::new_dummy(stack);

        process.execute_op(Operation::And, &mut host).unwrap();
        let expected = build_expected(&[ZERO, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test 0 AND 1 ---------------------------------------------------
        let stack = StackInputs::try_from_ints([2, 1, 0]).unwrap();
        let mut process = Process::new_dummy(stack);

        process.execute_op(Operation::And, &mut host).unwrap();
        let expected = build_expected(&[ZERO, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test 1 AND 1 ---------------------------------------------------
        let stack = StackInputs::try_from_ints([2, 1, 1]).unwrap();
        let mut process = Process::new_dummy(stack);

        process.execute_op(Operation::And, &mut host).unwrap();
        let expected = build_expected(&[ONE, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- first operand is not binary ------------------------------------
        let stack = StackInputs::try_from_ints([2, 1, 2]).unwrap();
        let mut process = Process::new_dummy(stack);
        assert!(process.execute_op(Operation::And, &mut host).is_err());

        // --- second operand is not binary -----------------------------------
        let stack = StackInputs::try_from_ints([2, 2, 1]).unwrap();
        let mut process = Process::new_dummy(stack);
        assert!(process.execute_op(Operation::And, &mut host).is_err());

        // --- calling AND with a stack of minimum depth is ok ----------------
        let mut process = Process::new_dummy_with_empty_stack();
        assert!(process.execute_op(Operation::And, &mut host).is_ok());
    }

    #[test]
    fn op_or() {
        let mut host = DefaultHost::default();
        // --- test 0 OR 0 ---------------------------------------------------
        let stack = StackInputs::try_from_ints([2, 0, 0]).unwrap();
        let mut process = Process::new_dummy(stack);

        process.execute_op(Operation::Or, &mut host).unwrap();
        let expected = build_expected(&[ZERO, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test 1 OR 0 ---------------------------------------------------
        let stack = StackInputs::try_from_ints([2, 0, 1]).unwrap();
        let mut process = Process::new_dummy(stack);

        process.execute_op(Operation::Or, &mut host).unwrap();
        let expected = build_expected(&[ONE, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test 0 OR 1 ---------------------------------------------------
        let stack = StackInputs::try_from_ints([2, 1, 0]).unwrap();
        let mut process = Process::new_dummy(stack);

        process.execute_op(Operation::Or, &mut host).unwrap();
        let expected = build_expected(&[ONE, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test 1 OR 0 ---------------------------------------------------
        let stack = StackInputs::try_from_ints([2, 1, 1]).unwrap();
        let mut process = Process::new_dummy(stack);

        process.execute_op(Operation::Or, &mut host).unwrap();
        let expected = build_expected(&[ONE, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- first operand is not binary ------------------------------------
        let stack = StackInputs::try_from_ints([2, 1, 2]).unwrap();
        let mut process = Process::new_dummy(stack);
        assert!(process.execute_op(Operation::Or, &mut host).is_err());

        // --- second operand is not binary -----------------------------------
        let stack = StackInputs::try_from_ints([2, 2, 1]).unwrap();
        let mut process = Process::new_dummy(stack);
        assert!(process.execute_op(Operation::Or, &mut host).is_err());

        // --- calling OR with a stack of minimum depth is a ok ----------------
        let mut process = Process::new_dummy_with_empty_stack();
        assert!(process.execute_op(Operation::Or, &mut host).is_ok());
    }

    #[test]
    fn op_not() {
        let mut host = DefaultHost::default();
        // --- test NOT 0 -----------------------------------------------------
        let stack = StackInputs::try_from_ints([2, 0]).unwrap();
        let mut process = Process::new_dummy(stack);
        process.execute_op(Operation::Not, &mut host).unwrap();
        let expected = build_expected(&[ONE, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test NOT 1 ----------------------------------------------------
        let stack = StackInputs::try_from_ints([2, 1]).unwrap();
        let mut process = Process::new_dummy(stack);
        process.execute_op(Operation::Not, &mut host).unwrap();
        let expected = build_expected(&[ZERO, Felt::new(2)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- operand is not binary ------------------------------------------
        let stack = StackInputs::try_from_ints([2, 2]).unwrap();
        let mut process = Process::new_dummy(stack);
        assert!(process.execute_op(Operation::Not, &mut host).is_err());
    }

    // COMPARISON OPERATIONS
    // --------------------------------------------------------------------------------------------

    #[test]
    fn op_eq() {
        // --- test when top two values are equal -----------------------------
        let advice_inputs = AdviceInputs::default();
        let stack_inputs = StackInputs::try_from_ints([3, 7, 7]).unwrap();
        let (mut process, mut host) =
            Process::new_dummy_with_inputs_and_decoder_helpers(stack_inputs, advice_inputs);

        process.execute_op(Operation::Eq, &mut host).unwrap();
        let expected = build_expected(&[ONE, Felt::new(3)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test when top two values are not equal -------------------------
        let advice_inputs = AdviceInputs::default();
        let stack_inputs = StackInputs::try_from_ints([3, 5, 7]).unwrap();
        let (mut process, mut host) =
            Process::new_dummy_with_inputs_and_decoder_helpers(stack_inputs, advice_inputs);

        process.execute_op(Operation::Eq, &mut host).unwrap();
        let expected = build_expected(&[ZERO, Felt::new(3)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- calling EQ with a stack of minimum depth is a ok ---------------
        let advice_inputs = AdviceInputs::default();
        let stack_inputs = StackInputs::default();
        let (mut process, mut host) =
            Process::new_dummy_with_inputs_and_decoder_helpers(stack_inputs, advice_inputs);
        assert!(process.execute_op(Operation::Eq, &mut host).is_ok());
    }

    #[test]
    fn op_eqz() {
        // --- test when top is zero ------------------------------------------
        let advice_inputs = AdviceInputs::default();
        let stack_inputs = StackInputs::try_from_ints([3, 0]).unwrap();
        let (mut process, mut host) =
            Process::new_dummy_with_inputs_and_decoder_helpers(stack_inputs, advice_inputs);

        process.execute_op(Operation::Eqz, &mut host).unwrap();
        let expected = build_expected(&[ONE, Felt::new(3)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test when top is not zero --------------------------------------
        let advice_inputs = AdviceInputs::default();
        let stack_inputs = StackInputs::try_from_ints([3, 4]).unwrap();
        let (mut process, mut host) =
            Process::new_dummy_with_inputs_and_decoder_helpers(stack_inputs, advice_inputs);

        process.execute_op(Operation::Eqz, &mut host).unwrap();
        let expected = build_expected(&[ZERO, Felt::new(3)]);
        assert_eq!(expected, process.stack.trace_state());
    }

    // EXPONENT OPERATIONS
    // --------------------------------------------------------------------------------------------

    #[test]
    fn op_expacc() {
        // --- test when b become 0 ---------------------------------------------------------------

        let a = 0;
        let b = 32;
        let c = 4;

        let advice_inputs = AdviceInputs::default();
        let stack_inputs = StackInputs::try_from_ints([a, b, c, 0]).unwrap();
        let (mut process, mut host) =
            Process::new_dummy_with_inputs_and_decoder_helpers(stack_inputs, advice_inputs);

        process.execute_op(Operation::Expacc, &mut host).unwrap();
        let expected = build_expected(&[ZERO, Felt::new(16), Felt::new(32), Felt::new(a >> 1)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test when bit from b is 1 ----------------------------------------------------------

        let a = 3;
        let b = 1;
        let c = 16;

        let advice_inputs = AdviceInputs::default();
        let stack_inputs = StackInputs::try_from_ints([a, b, c, 0]).unwrap();
        let (mut process, mut host) =
            Process::new_dummy_with_inputs_and_decoder_helpers(stack_inputs, advice_inputs);

        process.execute_op(Operation::Expacc, &mut host).unwrap();
        let expected = build_expected(&[ONE, Felt::new(256), Felt::new(16), Felt::new(a >> 1)]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test when bit from b is 1 & exp is 2**32 -------------------------------------------
        // exp will overflow the field after this operation.

        let a = 17;
        let b = 5;
        let c = 625;

        let advice_inputs = AdviceInputs::default();
        let stack_inputs = StackInputs::try_from_ints([a, b, c, 0]).unwrap();
        let (mut process, mut host) =
            Process::new_dummy_with_inputs_and_decoder_helpers(stack_inputs, advice_inputs);

        process.execute_op(Operation::Expacc, &mut host).unwrap();
        let expected =
            build_expected(&[ONE, Felt::new(390625), Felt::new(3125), Felt::new(a >> 1)]);
        assert_eq!(expected, process.stack.trace_state());
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------

    fn get_rand_values() -> (Felt, Felt, Felt) {
        let a = rand_value();
        let b = rand_value();
        let c = rand_value();
        (Felt::new(a), Felt::new(b), Felt::new(c))
    }

    fn build_expected(values: &[Felt]) -> [Felt; 16] {
        let mut expected = [ZERO; 16];
        for (&value, result) in values.iter().zip(expected.iter_mut()) {
            *result = value;
        }
        expected
    }
}
