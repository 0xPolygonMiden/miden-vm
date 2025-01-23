use super::{ExecutionError, Process, MIN_STACK_DEPTH};
use crate::ZERO;

impl Process {
    // STACK MANIPULATION
    // --------------------------------------------------------------------------------------------
    /// Pushes a ZERO onto the stack.
    pub(super) fn op_pad(&mut self) -> Result<(), ExecutionError> {
        self.stack.push(ZERO);
        Ok(())
    }

    /// Removes the top element off the stack.
    pub(super) fn op_drop(&mut self) -> Result<(), ExecutionError> {
        self.stack.pop_and_set([]);
        Ok(())
    }

    /// Pushes the copy the n-th item onto the stack. n is 0-based.
    pub(super) fn op_dup(&mut self, n: usize) -> Result<(), ExecutionError> {
        let value = self.stack.get(n);
        self.stack.push(value);
        Ok(())
    }

    /// Swaps stack elements 0 and 1.
    pub(super) fn op_swap(&mut self) -> Result<(), ExecutionError> {
        let a = self.stack.get(0);
        let b = self.stack.get(1);
        self.stack.set_and_copy([b, a]);
        Ok(())
    }

    /// Swaps stack elements 0, 1, 2, and 3 with elements 4, 5, 6, and 7.
    pub(super) fn op_swapw(&mut self) -> Result<(), ExecutionError> {
        let a0 = self.stack.get(0);
        let a1 = self.stack.get(1);
        let a2 = self.stack.get(2);
        let a3 = self.stack.get(3);
        let b0 = self.stack.get(4);
        let b1 = self.stack.get(5);
        let b2 = self.stack.get(6);
        let b3 = self.stack.get(7);

        self.stack.set_and_copy([b0, b1, b2, b3, a0, a1, a2, a3]);

        Ok(())
    }

    /// Swaps stack elements 0, 1, 2, and 3 with elements 8, 9, 10, and 11.
    pub(super) fn op_swapw2(&mut self) -> Result<(), ExecutionError> {
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

        self.stack.set_and_copy([c0, c1, c2, c3, b0, b1, b2, b3, a0, a1, a2, a3]);

        Ok(())
    }

    /// Swaps stack elements 0, 1, 2, and 3, with elements 12, 13, 14, and 15.
    pub(super) fn op_swapw3(&mut self) -> Result<(), ExecutionError> {
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

        self.stack
            .set_and_copy([d0, d1, d2, d3, b0, b1, b2, b3, c0, c1, c2, c3, a0, a1, a2, a3]);

        Ok(())
    }

    /// Swaps the top two words pair wise.
    ///
    /// Input: [D, C, B, A, ...]
    /// Output: [B, A, D, C, ...]
    pub(super) fn op_swapdw(&mut self) -> Result<(), ExecutionError> {
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

        self.stack
            .set_and_copy([c0, c1, c2, c3, d0, d1, d2, d3, a0, a1, a2, a3, b0, b1, b2, b3]);

        Ok(())
    }

    /// Rotates the top N elements to the right, such that the right-most element is moved to the
    /// top.
    pub(super) fn rotate_right<const N: usize>(&mut self) -> Result<(), ExecutionError> {
        debug_assert!(N < MIN_STACK_DEPTH, "N too large");

        let new_stack_top = {
            let mut new_stack_top = [ZERO; N];

            // move the N-1'th value to the top of the stack
            new_stack_top[0] = self.stack.get(N - 1);

            // shift all values up to n by one slot to the right
            for i in 0..(N - 1) {
                new_stack_top[i + 1] = self.stack.get(i);
            }

            new_stack_top
        };

        self.stack.set_and_copy(new_stack_top);

        Ok(())
    }
    /// Rotates the top N elements to the left, such that the left-most element is moved to the
    /// N-1'th position.
    pub(super) fn rotate_left<const N: usize>(&mut self) -> Result<(), ExecutionError> {
        debug_assert!(N < MIN_STACK_DEPTH - 1, "N too large");

        let new_stack_top = {
            let mut new_stack_top = [ZERO; N];

            // move the top of the stack to the bottom of the rotation.
            new_stack_top[N - 1] = self.stack.get(0);

            // shift all values up to n by one slot to the left
            for (i, stack_element) in new_stack_top.iter_mut().enumerate().take(N - 1) {
                *stack_element = self.stack.get(i + 1);
            }

            new_stack_top
        };

        self.stack.set_and_copy(new_stack_top);

        Ok(())
    }

    // CONDITIONAL MANIPULATION
    // --------------------------------------------------------------------------------------------

    /// Pops an element off the stack, and if the element is 1, swaps the top two elements on the
    /// stack. If the popped element is 0, the stack remains unchanged.
    ///
    /// # Errors
    /// Returns an error if the top element of the stack is neither 0 nor 1.
    pub(super) fn op_cswap(&mut self) -> Result<(), ExecutionError> {
        let c = self.stack.get(0);
        let b = self.stack.get(1);
        let a = self.stack.get(2);

        match c.as_int() {
            0 => {
                self.stack.pop_and_set([b, a]);
            },
            1 => {
                self.stack.pop_and_set([a, b]);
            },
            _ => return Err(ExecutionError::NotBinaryValue(c)),
        }

        Ok(())
    }

    /// Pops an element off the stack, and if the element is 1, swaps elements 0, 1, 2, and 3 with
    /// elements 4, 5, 6, and 7. If the popped element is 0, the stack remains unchanged.
    ///
    /// # Errors
    /// Returns an error if the top element of the stack is neither 0 nor 1.
    pub(super) fn op_cswapw(&mut self) -> Result<(), ExecutionError> {
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
                self.stack.pop_and_set([b0, b1, b2, b3, a0, a1, a2, a3]);
            },
            1 => {
                self.stack.pop_and_set([a0, a1, a2, a3, b0, b1, b2, b3]);
            },
            _ => return Err(ExecutionError::NotBinaryValue(c)),
        }

        Ok(())
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        super::{Operation, Process},
        MIN_STACK_DEPTH,
    };
    use crate::{DefaultHost, Felt, StackInputs, ONE, ZERO};

    #[test]
    fn op_pad() {
        let stack = StackInputs::default();
        let mut process = Process::new_dummy(stack);
        let mut host = DefaultHost::default();

        // push one item onto the stack
        process.execute_op(Operation::Push(ONE), &mut host).unwrap();
        let expected = build_expected(&[1]);
        assert_eq!(expected, process.stack.trace_state());

        // pad the stack
        process.execute_op(Operation::Pad, &mut host).unwrap();
        let expected = build_expected(&[0, 1]);

        assert_eq!(MIN_STACK_DEPTH + 2, process.stack.depth());
        assert_eq!(3, process.stack.current_clk());
        assert_eq!(expected, process.stack.trace_state());

        // pad the stack again
        process.execute_op(Operation::Pad, &mut host).unwrap();
        let expected = build_expected(&[0, 0, 1]);

        assert_eq!(MIN_STACK_DEPTH + 3, process.stack.depth());
        assert_eq!(4, process.stack.current_clk());
        assert_eq!(expected, process.stack.trace_state());
    }

    #[test]
    fn op_drop() {
        // push a few items onto the stack
        let stack = StackInputs::default();
        let mut process = Process::new_dummy(stack);
        let mut host = DefaultHost::default();
        process.execute_op(Operation::Push(ONE), &mut host).unwrap();
        process.execute_op(Operation::Push(Felt::new(2)), &mut host).unwrap();

        // drop the first value
        process.execute_op(Operation::Drop, &mut host).unwrap();
        let expected = build_expected(&[1]);
        assert_eq!(expected, process.stack.trace_state());
        assert_eq!(MIN_STACK_DEPTH + 1, process.stack.depth());

        // drop the next value
        process.execute_op(Operation::Drop, &mut host).unwrap();
        let expected = build_expected(&[]);
        assert_eq!(expected, process.stack.trace_state());
        assert_eq!(MIN_STACK_DEPTH, process.stack.depth());

        // calling drop with a minimum stack depth should be ok
        assert!(process.execute_op(Operation::Drop, &mut host).is_ok());
    }

    #[test]
    fn op_dup() {
        let stack = StackInputs::default();
        let mut process = Process::new_dummy(stack);
        let mut host = DefaultHost::default();

        // push one item onto the stack
        process.execute_op(Operation::Push(ONE), &mut host).unwrap();
        let expected = build_expected(&[1]);
        assert_eq!(expected, process.stack.trace_state());

        // duplicate it
        process.execute_op(Operation::Dup0, &mut host).unwrap();
        let expected = build_expected(&[1, 1]);
        assert_eq!(expected, process.stack.trace_state());

        // duplicating non-existent item from the min stack range should be ok
        assert!(process.execute_op(Operation::Dup2, &mut host).is_ok());
        // drop it again before continuing the tests and stack comparison
        process.execute_op(Operation::Drop, &mut host).unwrap();

        // put 15 more items onto the stack
        let mut expected = [ONE; 16];
        for i in 2..17 {
            process.execute_op(Operation::Push(Felt::new(i)), &mut host).unwrap();
            expected[16 - i as usize] = Felt::new(i);
        }
        assert_eq!(expected, process.stack.trace_state());

        // duplicate last stack item
        process.execute_op(Operation::Dup15, &mut host).unwrap();
        assert_eq!(ONE, process.stack.trace_state()[0]);
        assert_eq!(&expected[..15], &process.stack.trace_state()[1..]);

        // duplicate 8th stack item
        process.execute_op(Operation::Dup7, &mut host).unwrap();
        assert_eq!(Felt::new(10), process.stack.trace_state()[0]);
        assert_eq!(ONE, process.stack.trace_state()[1]);
        assert_eq!(&expected[..14], &process.stack.trace_state()[2..]);

        // remove 4 items off the stack
        process.execute_op(Operation::Drop, &mut host).unwrap();
        process.execute_op(Operation::Drop, &mut host).unwrap();
        process.execute_op(Operation::Drop, &mut host).unwrap();
        process.execute_op(Operation::Drop, &mut host).unwrap();

        assert_eq!(MIN_STACK_DEPTH + 15, process.stack.depth());

        assert_eq!(&expected[2..], &process.stack.trace_state()[..14]);
        assert_eq!(ONE, process.stack.trace_state()[14]);
        assert_eq!(ZERO, process.stack.trace_state()[15]);
    }

    #[test]
    fn op_swap() {
        // push a few items onto the stack
        let stack = StackInputs::try_from_ints([1, 2, 3]).unwrap();
        let mut process = Process::new_dummy(stack);
        let mut host = DefaultHost::default();

        process.execute_op(Operation::Swap, &mut host).unwrap();
        let expected = build_expected(&[2, 3, 1]);
        assert_eq!(expected, process.stack.trace_state());

        // swapping with a minimum stack should be ok
        let stack = StackInputs::default();
        let mut process = Process::new_dummy(stack);
        assert!(process.execute_op(Operation::Swap, &mut host).is_ok());
    }

    #[test]
    fn op_swapw() {
        // push a few items onto the stack
        let stack = StackInputs::try_from_ints([1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
        let mut process = Process::new_dummy(stack);
        let mut host = DefaultHost::default();

        process.execute_op(Operation::SwapW, &mut host).unwrap();
        let expected = build_expected(&[5, 4, 3, 2, 9, 8, 7, 6, 1]);
        assert_eq!(expected, process.stack.trace_state());

        // swapping with a minimum stack should be ok
        let stack = StackInputs::default();
        let mut process = Process::new_dummy(stack);
        assert!(process.execute_op(Operation::SwapW, &mut host).is_ok());
    }

    #[test]
    fn op_swapw2() {
        // push a few items onto the stack
        let stack =
            StackInputs::try_from_ints([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]).unwrap();
        let mut process = Process::new_dummy(stack);
        let mut host = DefaultHost::default();

        process.execute_op(Operation::SwapW2, &mut host).unwrap();
        let expected = build_expected(&[5, 4, 3, 2, 9, 8, 7, 6, 13, 12, 11, 10, 1]);
        assert_eq!(expected, process.stack.trace_state());

        // swapping with a minimum stack should be ok
        let stack = StackInputs::default();
        let mut process = Process::new_dummy(stack);
        assert!(process.execute_op(Operation::SwapW2, &mut host).is_ok());
    }

    #[test]
    fn op_swapw3() {
        // push a few items onto the stack
        let stack =
            StackInputs::try_from_ints([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16])
                .unwrap();
        let mut process = Process::new_dummy(stack);
        let mut host = DefaultHost::default();

        process.execute_op(Operation::SwapW3, &mut host).unwrap();
        let expected = build_expected(&[4, 3, 2, 1, 12, 11, 10, 9, 8, 7, 6, 5, 16, 15, 14, 13]);
        assert_eq!(expected, process.stack.trace_state());

        // swapping with a minimum stack should be ok
        let mut process = Process::new_dummy_with_empty_stack();
        assert!(process.execute_op(Operation::SwapW3, &mut host).is_ok());
    }

    #[test]
    fn op_movup() {
        // push a few items onto the stack
        let stack =
            StackInputs::try_from_ints([16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
                .unwrap();
        let mut process = Process::new_dummy(stack);
        let mut host = DefaultHost::default();

        // movup2
        process.execute_op(Operation::MovUp2, &mut host).unwrap();
        let expected = build_expected(&[3, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        assert_eq!(expected, process.stack.trace_state());

        // movup3
        process.execute_op(Operation::MovUp3, &mut host).unwrap();
        let expected = build_expected(&[4, 3, 1, 2, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        assert_eq!(expected, process.stack.trace_state());

        // movup7
        process.execute_op(Operation::MovUp7, &mut host).unwrap();
        let expected = build_expected(&[8, 4, 3, 1, 2, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15, 16]);
        assert_eq!(expected, process.stack.trace_state());

        // movup8
        process.execute_op(Operation::MovUp8, &mut host).unwrap();
        let expected = build_expected(&[9, 8, 4, 3, 1, 2, 5, 6, 7, 10, 11, 12, 13, 14, 15, 16]);
        assert_eq!(expected, process.stack.trace_state());

        // executing movup with a minimum stack depth should be ok
        let mut process = Process::new_dummy_with_empty_stack();
        assert!(process.execute_op(Operation::MovUp2, &mut host).is_ok());
    }

    #[test]
    fn op_movdn() {
        // push a few items onto the stack
        let stack =
            StackInputs::try_from_ints([16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
                .unwrap();
        let mut process = Process::new_dummy(stack);
        let mut host = DefaultHost::default();

        // movdn2
        process.execute_op(Operation::MovDn2, &mut host).unwrap();
        let expected = build_expected(&[2, 3, 1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        assert_eq!(expected, process.stack.trace_state());

        // movdn3
        process.execute_op(Operation::MovDn3, &mut host).unwrap();
        let expected = build_expected(&[3, 1, 4, 2, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        assert_eq!(expected, process.stack.trace_state());

        // movdn7
        process.execute_op(Operation::MovDn7, &mut host).unwrap();
        let expected = build_expected(&[1, 4, 2, 5, 6, 7, 8, 3, 9, 10, 11, 12, 13, 14, 15, 16]);
        assert_eq!(expected, process.stack.trace_state());

        // movdn15
        process.execute_op(Operation::MovDn8, &mut host).unwrap();
        let expected = build_expected(&[4, 2, 5, 6, 7, 8, 3, 9, 1, 10, 11, 12, 13, 14, 15, 16]);
        assert_eq!(expected, process.stack.trace_state());

        // executing movdn with a minimum stack depth should be ok
        let mut process = Process::new_dummy_with_empty_stack();
        assert!(process.execute_op(Operation::MovDn2, &mut host).is_ok());
    }

    #[test]
    fn op_cswap() {
        // push a few items onto the stack
        let stack = StackInputs::try_from_ints([4, 3, 2, 1, 0]).unwrap();
        let mut process = Process::new_dummy(stack);
        let mut host = DefaultHost::default();

        // no swap (top of the stack is 0)
        process.execute_op(Operation::CSwap, &mut host).unwrap();
        let expected = build_expected(&[1, 2, 3, 4]);
        assert_eq!(expected, process.stack.trace_state());

        // swap (top of the stack is 1)
        process.execute_op(Operation::CSwap, &mut host).unwrap();
        let expected = build_expected(&[3, 2, 4]);
        assert_eq!(expected, process.stack.trace_state());

        // error: top of the stack is not binary
        assert!(process.execute_op(Operation::CSwap, &mut host).is_err());

        // executing conditional swap with a minimum stack depth should be ok
        let mut process = Process::new_dummy_with_empty_stack();
        assert!(process.execute_op(Operation::CSwap, &mut host).is_ok());
    }

    #[test]
    fn op_cswapw() {
        // push a few items onto the stack
        let stack = StackInputs::try_from_ints([11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]).unwrap();
        let mut process = Process::new_dummy(stack);
        let mut host = DefaultHost::default();

        // no swap (top of the stack is 0)
        process.execute_op(Operation::CSwapW, &mut host).unwrap();
        let expected = build_expected(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        assert_eq!(expected, process.stack.trace_state());

        // swap (top of the stack is 1)
        process.execute_op(Operation::CSwapW, &mut host).unwrap();
        let expected = build_expected(&[6, 7, 8, 9, 2, 3, 4, 5, 10, 11]);
        assert_eq!(expected, process.stack.trace_state());

        // error: top of the stack is not binary
        assert!(process.execute_op(Operation::CSwapW, &mut host).is_err());

        // executing conditional swap with a minimum stack depth should be ok
        let mut process = Process::new_dummy_with_empty_stack();
        assert!(process.execute_op(Operation::CSwapW, &mut host).is_ok());
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------

    fn build_expected(values: &[u64]) -> [Felt; 16] {
        let mut expected = [ZERO; 16];
        for (&value, result) in values.iter().zip(expected.iter_mut()) {
            *result = Felt::new(value);
        }
        expected
    }
}
