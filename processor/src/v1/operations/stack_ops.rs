use super::{BaseElement, ExecutionError, FieldElement, Process};

impl Process {
    // STACK MANIPULATION
    // --------------------------------------------------------------------------------------------
    /// Pushes a ZERO onto the processor.stack.
    pub(super) fn op_pad(&mut self) -> Result<(), ExecutionError> {
        self.stack.set(0, BaseElement::ZERO);
        self.stack.shift_right(0);
        Ok(())
    }

    /// Removes the top element off the processor.stack.
    pub(super) fn op_drop(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(1, "DROP")?;
        self.stack.shift_left(1);
        Ok(())
    }

    /// Pushes the copy the n-th item onto the processor.stack.
    pub(super) fn op_dup(&mut self, n: usize) -> Result<(), ExecutionError> {
        self.stack.check_depth(n + 1, "DUP")?;
        let value = self.stack.get(n);
        self.stack.set(0, value);
        self.stack.shift_right(0);
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

    pub(super) fn op_movup5(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movup6(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movup7(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movup9(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movup11(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movup13(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movup15(&mut self) -> Result<(), ExecutionError> {
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

    pub(super) fn op_movdn5(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movdn6(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movdn7(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movdn9(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movdn11(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movdn13(&mut self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    pub(super) fn op_movdn15(&mut self) -> Result<(), ExecutionError> {
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
    use super::{
        super::{FieldElement, Operation, Process},
        BaseElement,
    };

    #[test]
    fn op_pad() {
        let mut processor = Process::new_dummy();

        // push one item onto the stack
        processor
            .execute_op(Operation::Push(BaseElement::ONE))
            .unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[0] = BaseElement::ONE;
        assert_eq!(expected, processor.stack.trace_state());

        // pad the stack
        processor.execute_op(Operation::Pad).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[1] = BaseElement::ONE;

        assert_eq!(2, processor.stack.depth());
        assert_eq!(2, processor.stack.current_step());
        assert_eq!(expected, processor.stack.trace_state());

        // pad the stack again
        processor.execute_op(Operation::Pad).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[2] = BaseElement::ONE;

        assert_eq!(3, processor.stack.depth());
        assert_eq!(3, processor.stack.current_step());
        assert_eq!(expected, processor.stack.trace_state());
    }

    #[test]
    fn op_drop() {
        let mut processor = Process::new_dummy();

        // push one item onto the stack
        processor
            .execute_op(Operation::Push(BaseElement::ONE))
            .unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[0] = BaseElement::ONE;
        assert_eq!(expected, processor.stack.trace_state());

        // pad the stack
        processor.execute_op(Operation::Pad).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[1] = BaseElement::ONE;

        assert_eq!(2, processor.stack.depth());
        assert_eq!(2, processor.stack.current_step());
        assert_eq!(expected, processor.stack.trace_state());

        // pad the stack again
        processor.execute_op(Operation::Pad).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[2] = BaseElement::ONE;

        assert_eq!(3, processor.stack.depth());
        assert_eq!(3, processor.stack.current_step());
        assert_eq!(expected, processor.stack.trace_state());
    }

    #[test]
    fn op_dup() {
        let mut processor = Process::new_dummy();

        // calling DUP on an empty stack should be an error
        assert!(processor.execute_op(Operation::Dup0).is_err());

        // push one item onto the stack
        processor
            .execute_op(Operation::Push(BaseElement::ONE))
            .unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[0] = BaseElement::ONE;
        assert_eq!(expected, processor.stack.trace_state());

        // duplicate it
        processor.execute_op(Operation::Dup0).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[0] = BaseElement::ONE;
        expected[1] = BaseElement::ONE;

        assert_eq!(2, processor.stack.depth());
        assert_eq!(2, processor.stack.current_step());
        assert_eq!(expected, processor.stack.trace_state());

        // duplicating non-existent item should be an error
        assert!(processor.execute_op(Operation::Dup2).is_err());

        // put 15 more items onto the stack
        let mut expected = [BaseElement::ONE; 16];
        for i in 2..17 {
            processor
                .execute_op(Operation::Push(BaseElement::new(i)))
                .unwrap();
            expected[16 - i as usize] = BaseElement::new(i);
        }
        assert_eq!(expected, processor.stack.trace_state());

        // duplicate last stack item
        processor.execute_op(Operation::Dup15).unwrap();
        assert_eq!(BaseElement::ONE, processor.stack.trace_state()[0]);
        assert_eq!(&expected[..15], &processor.stack.trace_state()[1..]);

        // duplicate 8th stack item
        processor.execute_op(Operation::Dup7).unwrap();
        assert_eq!(BaseElement::new(10), processor.stack.trace_state()[0]);
        assert_eq!(BaseElement::new(1), processor.stack.trace_state()[1]);
        assert_eq!(&expected[..14], &processor.stack.trace_state()[2..]);

        // remove 4 items off the stack
        processor.execute_op(Operation::Drop).unwrap();
        processor.execute_op(Operation::Drop).unwrap();
        processor.execute_op(Operation::Drop).unwrap();
        processor.execute_op(Operation::Drop).unwrap();

        assert_eq!(15, processor.stack.depth());

        assert_eq!(&expected[2..], &processor.stack.trace_state()[..14]);
        assert_eq!(BaseElement::ONE, processor.stack.trace_state()[14]);
        assert_eq!(BaseElement::ZERO, processor.stack.trace_state()[15]);
    }
}
