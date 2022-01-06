use super::{BaseElement, ExecutionError, Process};

// INPUT / OUTPUT OPERATIONS
// ================================================================================================

impl Process {
    /// Pushes the provided value onto the stack.
    ///
    /// The original stack is shifted to the right by one item.
    pub(super) fn op_push(&mut self, value: BaseElement) -> Result<(), ExecutionError> {
        self.stack.set(0, value);
        self.stack.shift_right(0);
        Ok(())
    }

    // MEMORY OPERATIONS
    // --------------------------------------------------------------------------------------------

    /// Loads a word (4 elements) from the specified memory address onto the stack.
    ///
    /// The operation works as follows:
    /// - The memory address is popped off the stack.
    /// - A word is retrieved from memory at the specified address. The memory is always
    ///   initialized to ZEROs, and thus, if the specified address has never been written to,
    ///   four ZERO elements are returned.
    /// - The top four elements of the stack are overwritten with values retried from memory.
    ///
    /// Thus, the net result of the operation is that the stack is shifted left by one item.
    ///
    /// # Errors
    /// Returns an error if the stack contains fewer than five elements.
    pub(super) fn op_loadw(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(5, "LOAD")?;

        // get the address from the stack and read the word from memory
        let addr = self.stack.get(0);
        let word = self.memory.read(addr);

        // update the stack state
        for (i, &value) in word.iter().rev().enumerate() {
            self.stack.set(i, value);
        }
        self.stack.shift_left(5);

        Ok(())
    }

    /// Stores a word (4 elements) from the stack into the specified memory address.
    ///
    /// The operation works as follows:
    /// - The memory address is popped off the stack.
    /// - The top four stack items are saved into the specified memory address. The items are not
    ///   removed from the stack.
    ///
    /// Thus, the net result of the operation is that the stack is shifted left by one item.
    ///
    /// # Errors
    /// Returns an error if the stack contains fewer than five elements.
    pub(super) fn op_storew(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(5, "STORE")?;

        // get the address from the stack and build the word to be saved from the stack values
        let addr = self.stack.get(0);
        let word = [
            self.stack.get(4),
            self.stack.get(3),
            self.stack.get(2),
            self.stack.get(1),
        ];

        // write the word to memory
        self.memory.write(addr, word);

        // update the stack state
        for (i, &value) in word.iter().rev().enumerate() {
            self.stack.set(i, value);
        }
        self.stack.shift_left(5);

        Ok(())
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        super::{FieldElement, Operation},
        BaseElement, Process,
    };

    #[test]
    fn op_push() {
        let mut processor = Process::new_dummy();
        assert_eq!(0, processor.stack.depth());
        assert_eq!(0, processor.stack.current_step());
        assert_eq!([BaseElement::ZERO; 16], processor.stack.trace_state());

        // push one item onto the stack
        let op = Operation::Push(BaseElement::ONE);
        processor.execute_op(op).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[0] = BaseElement::ONE;

        assert_eq!(1, processor.stack.depth());
        assert_eq!(1, processor.stack.current_step());
        assert_eq!(expected, processor.stack.trace_state());

        // push another item onto the stack
        let op = Operation::Push(BaseElement::new(3));
        processor.execute_op(op).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[0] = BaseElement::new(3);
        expected[1] = BaseElement::ONE;

        assert_eq!(2, processor.stack.depth());
        assert_eq!(2, processor.stack.current_step());
        assert_eq!(expected, processor.stack.trace_state());
    }

    // MEMORY OPERATION TESTS
    // --------------------------------------------------------------------------------------------

    #[test]
    fn op_storew() {
        let mut processor = Process::new_dummy();
        assert_eq!(0, processor.memory.size());

        // push the first word onto the stack and save it at address 0
        let word1 = [
            BaseElement::new(1),
            BaseElement::new(3),
            BaseElement::new(5),
            BaseElement::new(7),
        ];
        store_value(&mut processor, 0, word1);

        // check stack state
        let mut expected_stack = [BaseElement::ZERO; 16];
        expected_stack[0] = BaseElement::new(7);
        expected_stack[1] = BaseElement::new(5);
        expected_stack[2] = BaseElement::new(3);
        expected_stack[3] = BaseElement::new(1);
        assert_eq!(expected_stack, processor.stack.trace_state());

        // check memory state
        assert_eq!(1, processor.memory.size());
        assert_eq!(word1, processor.memory.get_value(0).unwrap());

        // push the second word onto the stack and save it at address 3
        let word2 = [
            BaseElement::new(2),
            BaseElement::new(4),
            BaseElement::new(6),
            BaseElement::new(8),
        ];
        store_value(&mut processor, 3, word2);

        // check stack state
        let mut expected_stack = [BaseElement::ZERO; 16];
        expected_stack[0] = BaseElement::new(8);
        expected_stack[1] = BaseElement::new(6);
        expected_stack[2] = BaseElement::new(4);
        expected_stack[3] = BaseElement::new(2);
        expected_stack[4] = BaseElement::new(7);
        expected_stack[5] = BaseElement::new(5);
        expected_stack[6] = BaseElement::new(3);
        expected_stack[7] = BaseElement::new(1);
        assert_eq!(expected_stack, processor.stack.trace_state());

        // check memory state
        assert_eq!(2, processor.memory.size());
        assert_eq!(word1, processor.memory.get_value(0).unwrap());
        assert_eq!(word2, processor.memory.get_value(3).unwrap());
    }

    #[test]
    fn op_loadw() {
        let mut processor = Process::new_dummy();
        assert_eq!(0, processor.memory.size());

        // push a word onto the stack and save it at address 1
        let word = [
            BaseElement::new(1),
            BaseElement::new(3),
            BaseElement::new(5),
            BaseElement::new(7),
        ];
        store_value(&mut processor, 1, word);

        // push four zeros onto the stack
        for _ in 0..4 {
            processor.execute_op(Operation::Pad).unwrap();
        }

        // push the address onto the stack and load the word
        processor
            .execute_op(Operation::Push(BaseElement::ONE))
            .unwrap();
        processor.execute_op(Operation::LoadW).unwrap();

        let mut expected_stack = [BaseElement::ZERO; 16];
        expected_stack[0] = BaseElement::new(7);
        expected_stack[1] = BaseElement::new(5);
        expected_stack[2] = BaseElement::new(3);
        expected_stack[3] = BaseElement::new(1);
        expected_stack[4] = BaseElement::new(7);
        expected_stack[5] = BaseElement::new(5);
        expected_stack[6] = BaseElement::new(3);
        expected_stack[7] = BaseElement::new(1);
        assert_eq!(expected_stack, processor.stack.trace_state());

        // check memory state
        assert_eq!(1, processor.memory.size());
        assert_eq!(word, processor.memory.get_value(1).unwrap());
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------
    fn store_value(processor: &mut Process, addr: u64, value: [BaseElement; 4]) {
        for &value in value.iter() {
            processor.execute_op(Operation::Push(value)).unwrap();
        }
        let addr = BaseElement::new(addr);
        processor.execute_op(Operation::Push(addr)).unwrap();
        processor.execute_op(Operation::StoreW).unwrap();
    }
}
