use super::{BaseElement, ExecutionError, FieldElement, Stack};

impl Stack {
    // STACK MANIPULATION
    // --------------------------------------------------------------------------------------------
    /// Pushes a ZERO onto the stack.
    pub(super) fn op_pad(&mut self) -> Result<(), ExecutionError> {
        self.trace[0][self.step + 1] = BaseElement::ZERO;
        self.shift_right(0);
        Ok(())
    }

    /// Removes the top element off the stack.
    pub(super) fn op_drop(&mut self) -> Result<(), ExecutionError> {
        if self.depth == 0 {
            return Err(ExecutionError::StackUnderflow("DROP", self.step));
        }
        self.shift_left(1);
        Ok(())
    }

    /// Pushes the copy the n-th item onto the stack.
    pub(super) fn op_dup(&mut self, n: usize) -> Result<(), ExecutionError> {
        if self.depth <= n {
            return Err(ExecutionError::StackUnderflow("DUP", self.step));
        }
        self.trace[0][self.step + 1] = self.trace[n][self.step];
        self.shift_right(0);
        Ok(())
    }

    pub(super) fn op_swap(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_swapw(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_swapw2(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_swapw3(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movup2(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movup3(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movup4(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movup8(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movup12(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movdn2(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movdn3(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movdn4(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movdn8(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movdn12(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    // CONDITIONAL MANIPULATION
    // --------------------------------------------------------------------------------------------

    pub(super) fn op_cswap(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_cswapw(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use vm_core::StarkField;

    use super::{
        super::{FieldElement, Operation},
        BaseElement, Stack,
    };

    #[test]
    fn op_pad() {
        let mut stack = Stack::new(2);

        // push one item onto the stack
        stack.execute(Operation::Push(BaseElement::ONE)).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[0] = BaseElement::ONE;
        assert_eq!(expected, stack.trace_state());

        // pad the stack
        stack.execute(Operation::Pad).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[1] = BaseElement::ONE;

        assert_eq!(2, stack.depth());
        assert_eq!(2, stack.current_step());
        assert_eq!(expected, stack.trace_state());

        // pad the stack again
        stack.execute(Operation::Pad).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[2] = BaseElement::ONE;

        assert_eq!(3, stack.depth());
        assert_eq!(3, stack.current_step());
        assert_eq!(expected, stack.trace_state());
    }

    #[test]
    fn op_drop() {
        let mut stack = Stack::new(2);

        // push one item onto the stack
        stack.execute(Operation::Push(BaseElement::ONE)).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[0] = BaseElement::ONE;
        assert_eq!(expected, stack.trace_state());

        // pad the stack
        stack.execute(Operation::Pad).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[1] = BaseElement::ONE;

        assert_eq!(2, stack.depth());
        assert_eq!(2, stack.current_step());
        assert_eq!(expected, stack.trace_state());

        // pad the stack again
        stack.execute(Operation::Pad).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[2] = BaseElement::ONE;

        assert_eq!(3, stack.depth());
        assert_eq!(3, stack.current_step());
        assert_eq!(expected, stack.trace_state());
    }

    #[test]
    fn op_dup() {
        let mut stack = Stack::new(2);

        // calling DUP on an empty stack should be an error
        assert!(stack.execute(Operation::Dup0).is_err());

        // push one item onto the stack
        stack.execute(Operation::Push(BaseElement::ONE)).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[0] = BaseElement::ONE;
        assert_eq!(expected, stack.trace_state());

        // duplicate it
        stack.execute(Operation::Dup0).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[0] = BaseElement::ONE;
        expected[1] = BaseElement::ONE;

        assert_eq!(2, stack.depth());
        assert_eq!(2, stack.current_step());
        assert_eq!(expected, stack.trace_state());

        // duplicating non-existent item should be an error
        assert!(stack.execute(Operation::Dup2).is_err());

        // put 15 more items onto the stack
        let mut expected = [BaseElement::ONE; 16];
        for i in 2..17 {
            stack.execute(Operation::Push(BaseElement::new(i))).unwrap();
            expected[16 - i as usize] = BaseElement::new(i);
        }
        assert_eq!(expected, stack.trace_state());

        // duplicate last stack item
        stack.execute(Operation::Dup15).unwrap();
        assert_eq!(BaseElement::ONE, stack.trace_state()[0]);
        assert_eq!(&expected[..15], &stack.trace_state()[1..]);

        // duplicate 8th stack item
        stack.execute(Operation::Dup7).unwrap();
        assert_eq!(BaseElement::new(10), stack.trace_state()[0]);
        assert_eq!(BaseElement::new(1), stack.trace_state()[1]);
        assert_eq!(&expected[..14], &stack.trace_state()[2..]);

        println!(
            "{:?}",
            stack
                .trace_state()
                .iter()
                .map(|v| v.as_int())
                .collect::<Vec<_>>()
        );

        // remove 4 items off the stack
        stack.execute(Operation::Drop).unwrap();
        stack.execute(Operation::Drop).unwrap();
        stack.execute(Operation::Drop).unwrap();
        stack.execute(Operation::Drop).unwrap();

        assert_eq!(15, stack.depth());

        assert_eq!(&expected[2..], &stack.trace_state()[..14]);
        assert_eq!(BaseElement::ONE, stack.trace_state()[14]);
        assert_eq!(BaseElement::ZERO, stack.trace_state()[15]);
    }
}
