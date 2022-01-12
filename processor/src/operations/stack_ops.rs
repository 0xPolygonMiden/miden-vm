use super::{super::STACK_TOP_SIZE, ExecutionError, Felt, FieldElement, Process, StarkField};

impl Process {
    // STACK MANIPULATION
    // --------------------------------------------------------------------------------------------
    /// Pushes a ZERO onto the process.stack.
    pub(super) fn op_pad(&mut self) -> Result<(), ExecutionError> {
        self.stack.set(0, Felt::ZERO);
        self.stack.shift_right(0);
        Ok(())
    }

    /// Removes the top element off the process.stack.
    pub(super) fn op_drop(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(1, "DROP")?;
        self.stack.shift_left(1);
        Ok(())
    }

    /// Pushes the copy the n-th item onto the process.stack.
    pub(super) fn op_dup(&mut self, n: usize) -> Result<(), ExecutionError> {
        self.stack.check_depth(n + 1, "DUP")?;
        let value = self.stack.get(n);
        self.stack.set(0, value);
        self.stack.shift_right(0);
        Ok(())
    }

    /// TODO: add docs
    pub(super) fn op_swap(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(2, "SWAP")?;
        let a = self.stack.get(0);
        let b = self.stack.get(1);
        self.stack.set(0, b);
        self.stack.set(1, a);
        self.stack.copy_state(2);
        Ok(())
    }

    /// TODO: add docs
    pub(super) fn op_swapw(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(8, "SWAPW")?;

        let a0 = self.stack.get(0);
        let a1 = self.stack.get(1);
        let a2 = self.stack.get(2);
        let a3 = self.stack.get(3);
        let b0 = self.stack.get(4);
        let b1 = self.stack.get(5);
        let b2 = self.stack.get(6);
        let b3 = self.stack.get(7);

        self.stack.set(0, b0);
        self.stack.set(1, b1);
        self.stack.set(2, b2);
        self.stack.set(3, b3);
        self.stack.set(4, a0);
        self.stack.set(5, a1);
        self.stack.set(6, a2);
        self.stack.set(7, a3);

        self.stack.copy_state(8);
        Ok(())
    }

    /// TODO: add docs
    pub(super) fn op_swapw2(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(12, "SWAPW2")?;

        let a0 = self.stack.get(0);
        let a1 = self.stack.get(1);
        let a2 = self.stack.get(2);
        let a3 = self.stack.get(3);
        let b0 = self.stack.get(4);
        let b1 = self.stack.get(5);
        let b2 = self.stack.get(6);
        let b3 = self.stack.get(7);
        let c0 = self.stack.get(8);
        let c1 = self.stack.get(9);
        let c2 = self.stack.get(10);
        let c3 = self.stack.get(11);

        self.stack.set(0, c0);
        self.stack.set(1, c1);
        self.stack.set(2, c2);
        self.stack.set(3, c3);
        self.stack.set(4, b0);
        self.stack.set(5, b1);
        self.stack.set(6, b2);
        self.stack.set(7, b3);
        self.stack.set(8, a0);
        self.stack.set(9, a1);
        self.stack.set(10, a2);
        self.stack.set(11, a3);

        self.stack.copy_state(12);
        Ok(())
    }

    /// TODO: add docs
    pub(super) fn op_swapw3(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(16, "SWAPW3")?;

        let a0 = self.stack.get(0);
        let a1 = self.stack.get(1);
        let a2 = self.stack.get(2);
        let a3 = self.stack.get(3);
        let b0 = self.stack.get(4);
        let b1 = self.stack.get(5);
        let b2 = self.stack.get(6);
        let b3 = self.stack.get(7);
        let c0 = self.stack.get(8);
        let c1 = self.stack.get(9);
        let c2 = self.stack.get(10);
        let c3 = self.stack.get(11);
        let d0 = self.stack.get(12);
        let d1 = self.stack.get(13);
        let d2 = self.stack.get(14);
        let d3 = self.stack.get(15);

        self.stack.set(0, d0);
        self.stack.set(1, d1);
        self.stack.set(2, d2);
        self.stack.set(3, d3);
        self.stack.set(4, b0);
        self.stack.set(5, b1);
        self.stack.set(6, b2);
        self.stack.set(7, b3);
        self.stack.set(8, c0);
        self.stack.set(9, c1);
        self.stack.set(10, c2);
        self.stack.set(11, c3);
        self.stack.set(12, a0);
        self.stack.set(13, a1);
        self.stack.set(14, a2);
        self.stack.set(15, a3);

        Ok(())
    }

    /// TODO: add docs
    pub(super) fn op_movup(&mut self, n: usize) -> Result<(), ExecutionError> {
        self.stack.check_depth(n + 1, "MOVUP")?;

        // move the nth value to the top of the stack
        let value = self.stack.get(n);
        self.stack.set(0, value);

        // shift all values up to n by one slot to the right
        for i in 0..n {
            let value = self.stack.get(i);
            self.stack.set(i + 1, value);
        }

        // all other items on the stack remain in place
        if (n + 1) < STACK_TOP_SIZE {
            self.stack.copy_state(n + 1);
        }
        Ok(())
    }

    /// TODO: add docs
    pub(super) fn op_movdn(&mut self, n: usize) -> Result<(), ExecutionError> {
        self.stack.check_depth(n + 1, "MOVDN")?;

        // move the value at the top of the stack to the nth position
        let value = self.stack.get(0);
        self.stack.set(n, value);

        // shift all values up to n by one slot to the left
        for i in 0..n {
            let value = self.stack.get(i + 1);
            self.stack.set(i, value);
        }

        // all other items on the stack remain in place
        if (n + 1) < STACK_TOP_SIZE {
            self.stack.copy_state(n + 1);
        }
        Ok(())
    }

    // CONDITIONAL MANIPULATION
    // --------------------------------------------------------------------------------------------

    /// TODO: add docs
    pub(super) fn op_cswap(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(3, "CSWAP")?;
        let c = self.stack.get(0);
        let b = self.stack.get(1);
        let a = self.stack.get(2);

        match c.as_int() {
            0 => {
                self.stack.set(0, b);
                self.stack.set(1, a);
            }
            1 => {
                self.stack.set(0, a);
                self.stack.set(1, b);
            }
            _ => return Err(ExecutionError::NotBinaryValue(c)),
        }

        self.stack.shift_left(3);
        Ok(())
    }

    /// TODO: add docs
    pub(super) fn op_cswapw(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(9, "CSWAPW")?;
        let c = self.stack.get(0);
        let b0 = self.stack.get(1);
        let b1 = self.stack.get(2);
        let b2 = self.stack.get(3);
        let b3 = self.stack.get(4);
        let a0 = self.stack.get(5);
        let a1 = self.stack.get(6);
        let a2 = self.stack.get(7);
        let a3 = self.stack.get(8);

        match c.as_int() {
            0 => {
                self.stack.set(0, b0);
                self.stack.set(1, b1);
                self.stack.set(2, b2);
                self.stack.set(3, b3);
                self.stack.set(4, a0);
                self.stack.set(5, a1);
                self.stack.set(6, a2);
                self.stack.set(7, a3);
            }
            1 => {
                self.stack.set(0, a0);
                self.stack.set(1, a1);
                self.stack.set(2, a2);
                self.stack.set(3, a3);
                self.stack.set(4, b0);
                self.stack.set(5, b1);
                self.stack.set(6, b2);
                self.stack.set(7, b3);
            }
            _ => return Err(ExecutionError::NotBinaryValue(c)),
        }

        self.stack.shift_left(9);
        Ok(())
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        super::{FieldElement, Operation, Process},
        Felt,
    };

    #[test]
    fn op_pad() {
        let mut process = Process::new_dummy();

        // push one item onto the stack
        process.execute_op(Operation::Push(Felt::ONE)).unwrap();
        let mut expected = [Felt::ZERO; 16];
        expected[0] = Felt::ONE;
        assert_eq!(expected, process.stack.trace_state());

        // pad the stack
        process.execute_op(Operation::Pad).unwrap();
        let mut expected = [Felt::ZERO; 16];
        expected[1] = Felt::ONE;

        assert_eq!(2, process.stack.depth());
        assert_eq!(2, process.stack.current_step());
        assert_eq!(expected, process.stack.trace_state());

        // pad the stack again
        process.execute_op(Operation::Pad).unwrap();
        let mut expected = [Felt::ZERO; 16];
        expected[2] = Felt::ONE;

        assert_eq!(3, process.stack.depth());
        assert_eq!(3, process.stack.current_step());
        assert_eq!(expected, process.stack.trace_state());
    }

    #[test]
    fn op_drop() {
        let mut process = Process::new_dummy();

        // push one item onto the stack
        process.execute_op(Operation::Push(Felt::ONE)).unwrap();
        let mut expected = [Felt::ZERO; 16];
        expected[0] = Felt::ONE;
        assert_eq!(expected, process.stack.trace_state());

        // pad the stack
        process.execute_op(Operation::Pad).unwrap();
        let mut expected = [Felt::ZERO; 16];
        expected[1] = Felt::ONE;

        assert_eq!(2, process.stack.depth());
        assert_eq!(2, process.stack.current_step());
        assert_eq!(expected, process.stack.trace_state());

        // pad the stack again
        process.execute_op(Operation::Pad).unwrap();
        let mut expected = [Felt::ZERO; 16];
        expected[2] = Felt::ONE;

        assert_eq!(3, process.stack.depth());
        assert_eq!(3, process.stack.current_step());
        assert_eq!(expected, process.stack.trace_state());
    }

    #[test]
    fn op_dup() {
        let mut process = Process::new_dummy();

        // calling DUP on an empty stack should be an error
        assert!(process.execute_op(Operation::Dup0).is_err());

        // push one item onto the stack
        process.execute_op(Operation::Push(Felt::ONE)).unwrap();
        let mut expected = [Felt::ZERO; 16];
        expected[0] = Felt::ONE;
        assert_eq!(expected, process.stack.trace_state());

        // duplicate it
        process.execute_op(Operation::Dup0).unwrap();
        let mut expected = [Felt::ZERO; 16];
        expected[0] = Felt::ONE;
        expected[1] = Felt::ONE;

        assert_eq!(2, process.stack.depth());
        assert_eq!(2, process.stack.current_step());
        assert_eq!(expected, process.stack.trace_state());

        // duplicating non-existent item should be an error
        assert!(process.execute_op(Operation::Dup2).is_err());

        // put 15 more items onto the stack
        let mut expected = [Felt::ONE; 16];
        for i in 2..17 {
            process.execute_op(Operation::Push(Felt::new(i))).unwrap();
            expected[16 - i as usize] = Felt::new(i);
        }
        assert_eq!(expected, process.stack.trace_state());

        // duplicate last stack item
        process.execute_op(Operation::Dup15).unwrap();
        assert_eq!(Felt::ONE, process.stack.trace_state()[0]);
        assert_eq!(&expected[..15], &process.stack.trace_state()[1..]);

        // duplicate 8th stack item
        process.execute_op(Operation::Dup7).unwrap();
        assert_eq!(Felt::new(10), process.stack.trace_state()[0]);
        assert_eq!(Felt::new(1), process.stack.trace_state()[1]);
        assert_eq!(&expected[..14], &process.stack.trace_state()[2..]);

        // remove 4 items off the stack
        process.execute_op(Operation::Drop).unwrap();
        process.execute_op(Operation::Drop).unwrap();
        process.execute_op(Operation::Drop).unwrap();
        process.execute_op(Operation::Drop).unwrap();

        assert_eq!(15, process.stack.depth());

        assert_eq!(&expected[2..], &process.stack.trace_state()[..14]);
        assert_eq!(Felt::ONE, process.stack.trace_state()[14]);
        assert_eq!(Felt::ZERO, process.stack.trace_state()[15]);
    }
}
