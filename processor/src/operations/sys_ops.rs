use super::{
    super::system::{FMP_MAX, FMP_MIN},
    AdviceProvider, ExecutionError, Felt, FieldElement, Process, StarkField,
};

// SYSTEM OPERATIONS
// ================================================================================================

impl<A> Process<A>
where
    A: AdviceProvider,
{
    /// Pops a value off the stack and asserts that it is equal to ONE.
    ///
    /// # Errors
    /// Returns an error if the popped value is not ONE.
    pub(super) fn op_assert(&mut self) -> Result<(), ExecutionError> {
        if self.stack.get(0) != Felt::ONE {
            return Err(ExecutionError::FailedAssertion(self.system.clk()));
        }
        self.stack.shift_left(1);
        Ok(())
    }

    // FREE MEMORY POINTER
    // --------------------------------------------------------------------------------------------

    /// Pops an element off the stack, adds the current value of the `fmp` register to it, and
    /// pushes the result back onto the stack.
    pub(super) fn op_fmpadd(&mut self) -> Result<(), ExecutionError> {
        let offset = self.stack.get(0);
        let fmp = self.system.fmp();

        self.stack.set(0, fmp + offset);
        self.stack.copy_state(1);

        Ok(())
    }

    /// Pops an element off the stack and adds it to the current value of `fmp` register.
    ///
    /// # Errors
    /// Returns an error if the new value of `fmp` register is greater than or equal to 3 * 2^30.
    pub(super) fn op_fmpupdate(&mut self) -> Result<(), ExecutionError> {
        let offset = self.stack.get(0);
        let fmp = self.system.fmp();

        let new_fmp = fmp + offset;
        if new_fmp.as_int() < FMP_MIN || new_fmp.as_int() > FMP_MAX {
            return Err(ExecutionError::InvalidFmpValue(fmp, new_fmp));
        }

        self.system.set_fmp(new_fmp);
        self.stack.shift_left(1);

        Ok(())
    }

    // STACK DEPTH
    // --------------------------------------------------------------------------------------------

    /// Pushes the current depth of the stack (the depth before this operation is executed) onto
    /// the stack.
    pub(super) fn op_sdepth(&mut self) -> Result<(), ExecutionError> {
        let stack_depth = self.stack.depth();
        self.stack.set(0, Felt::new(stack_depth as u64));
        self.stack.shift_right(0);
        Ok(())
    }

    // CALLER
    // --------------------------------------------------------------------------------------------

    /// Overwrites the top four stack items with the hash of a function which initiated the current
    /// SYSCALL.
    ///
    /// # Errors
    /// Returns an error if the VM is not currently executing a SYSCALL block.
    pub(super) fn op_caller(&mut self) -> Result<(), ExecutionError> {
        if !self.system.in_syscall() {
            return Err(ExecutionError::CallerNotInSyscall);
        }

        let fn_hash = self.system.fn_hash();

        self.stack.set(0, fn_hash[3]);
        self.stack.set(1, fn_hash[2]);
        self.stack.set(2, fn_hash[1]);
        self.stack.set(3, fn_hash[0]);

        self.stack.copy_state(4);

        Ok(())
    }

    // CLOCK CYCLE
    // --------------------------------------------------------------------------------------------

    /// Pushes the current value of the clock cycle counter onto the stack. The clock cycle starts
    /// at 0 and is incremented with every operation executed by the VM, including control flow
    /// operations such as GRUOP, END etc.
    pub(super) fn op_clk(&mut self) -> Result<(), ExecutionError> {
        let clk = self.system.clk();
        self.stack.set(0, Felt::from(clk));
        self.stack.shift_right(0);
        Ok(())
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        super::Operation, super::STACK_TOP_SIZE, Felt, FieldElement, Process, FMP_MAX, FMP_MIN,
    };
    use crate::StackInputs;

    const MAX_PROC_LOCALS: u64 = 2_u64.pow(31) - 1;

    #[test]
    fn op_assert() {
        // calling assert with a minimum stack should be an ok, as long as the top value is ONE
        let mut process = Process::new_dummy_with_empty_stack();
        process.execute_op(Operation::Push(Felt::ONE)).unwrap();
        process.execute_op(Operation::Swap).unwrap();
        process.execute_op(Operation::Drop).unwrap();

        assert!(process.execute_op(Operation::Assert).is_ok());
    }

    #[test]
    fn op_fmpupdate() {
        let mut process = Process::new_dummy_with_empty_stack();

        // initial value of fmp register should be 2^30
        assert_eq!(Felt::new(2_u64.pow(30)), process.system.fmp());

        // increment fmp register
        process.execute_op(Operation::Push(Felt::new(2))).unwrap();
        process.execute_op(Operation::FmpUpdate).unwrap();
        assert_eq!(Felt::new(FMP_MIN + 2), process.system.fmp());

        // increment fmp register again
        process.execute_op(Operation::Push(Felt::new(3))).unwrap();
        process.execute_op(Operation::FmpUpdate).unwrap();
        assert_eq!(Felt::new(FMP_MIN + 5), process.system.fmp());

        // decrement fmp register
        process.execute_op(Operation::Push(-Felt::new(3))).unwrap();
        process.execute_op(Operation::FmpUpdate).unwrap();
        assert_eq!(Felt::new(FMP_MIN + 2), process.system.fmp());

        // decrementing beyond the minimum fmp value should be an error
        process.execute_op(Operation::Push(-Felt::new(3))).unwrap();
        assert!(process.execute_op(Operation::FmpUpdate).is_err());

        // going up to the max fmp value should be OK
        let stack = StackInputs::try_from_values([MAX_PROC_LOCALS]).unwrap();
        let mut process = Process::new_dummy(stack);
        process.execute_op(Operation::FmpUpdate).unwrap();
        assert_eq!(Felt::new(FMP_MAX), process.system.fmp());

        // but going beyond that should be an error
        let stack = StackInputs::try_from_values([MAX_PROC_LOCALS + 1]).unwrap();
        let mut process = Process::new_dummy(stack);
        assert!(process.execute_op(Operation::FmpUpdate).is_err());

        // should not affect the rest of the stack state
        let stack = StackInputs::try_from_values([2, 3]).unwrap();
        let mut process = Process::new_dummy(stack);
        process.execute_op(Operation::FmpUpdate).unwrap();

        let expected = build_expected_stack(&[2]);
        assert_eq!(expected, process.stack.trace_state());

        // calling fmpupdate with a minimum stack should be ok
        let mut process = Process::new_dummy_with_empty_stack();
        assert!(process.execute_op(Operation::FmpUpdate).is_ok());
    }

    #[test]
    fn op_fmpadd() {
        let mut process = Process::new_dummy_with_empty_stack();

        // set value of fmp register
        process.execute_op(Operation::Push(Felt::new(2))).unwrap();
        process.execute_op(Operation::FmpUpdate).unwrap();

        // compute address of the first local
        process.execute_op(Operation::Push(-Felt::new(1))).unwrap();
        process.execute_op(Operation::FmpAdd).unwrap();

        let expected = build_expected_stack(&[FMP_MIN + 1]);
        assert_eq!(expected, process.stack.trace_state());

        // compute address of second local (also make sure that rest of stack is not affected)
        process.execute_op(Operation::Push(-Felt::new(2))).unwrap();
        process.execute_op(Operation::FmpAdd).unwrap();

        let expected = build_expected_stack(&[FMP_MIN, FMP_MIN + 1]);
        assert_eq!(expected, process.stack.trace_state());
    }

    #[test]
    fn op_sdepth() {
        // stack is empty
        let mut process = Process::new_dummy_with_empty_stack();
        process.execute_op(Operation::SDepth).unwrap();
        let expected = build_expected_stack(&[STACK_TOP_SIZE as u64]);
        assert_eq!(expected, process.stack.trace_state());
        assert_eq!(STACK_TOP_SIZE + 1, process.stack.depth());

        // stack has one item
        process.execute_op(Operation::SDepth).unwrap();
        let expected = build_expected_stack(&[STACK_TOP_SIZE as u64 + 1, STACK_TOP_SIZE as u64]);
        assert_eq!(expected, process.stack.trace_state());
        assert_eq!(STACK_TOP_SIZE + 2, process.stack.depth());

        // stack has 3 items
        process.execute_op(Operation::Pad).unwrap();
        process.execute_op(Operation::SDepth).unwrap();
        let expected = build_expected_stack(&[
            STACK_TOP_SIZE as u64 + 3,
            0,
            STACK_TOP_SIZE as u64 + 1,
            STACK_TOP_SIZE as u64,
        ]);
        assert_eq!(expected, process.stack.trace_state());
        assert_eq!(STACK_TOP_SIZE + 4, process.stack.depth());
    }

    #[test]
    fn op_clk() {
        let mut process = Process::new_dummy_with_empty_stack();

        // initial value of clk register should be 1.
        process.execute_op(Operation::Clk).unwrap();
        let expected = build_expected_stack(&[1]);
        assert_eq!(expected, process.stack.trace_state());

        // increment clk register.
        process.execute_op(Operation::Push(Felt::new(2))).unwrap();
        process.execute_op(Operation::Clk).unwrap();
        let expected = build_expected_stack(&[3, 2, 1]);
        assert_eq!(expected, process.stack.trace_state());

        // increment clk register again.
        process.execute_op(Operation::Push(Felt::new(3))).unwrap();
        process.execute_op(Operation::Clk).unwrap();
        let expected = build_expected_stack(&[5, 3, 3, 2, 1]);
        assert_eq!(expected, process.stack.trace_state());
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------

    fn build_expected_stack(values: &[u64]) -> [Felt; 16] {
        let mut expected = [Felt::ZERO; 16];
        for (&value, result) in values.iter().zip(expected.iter_mut()) {
            *result = Felt::new(value);
        }
        expected
    }
}
