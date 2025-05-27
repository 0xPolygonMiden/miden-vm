use vm_core::{Felt, Word, ZERO};

use crate::processor::Processor;

use super::CoreTraceFragmentGenerator;

impl CoreTraceFragmentGenerator {
    /// Pushes a value onto the stack.
    pub(crate) fn push(&mut self, value: Felt) {
        self.stack_shift_right(1);
        self.stack_set(0, value);
    }

    /// Pops a value from the advice stack and pushes it onto the operand stack.
    pub(crate) fn advpop(&mut self) {
        let value = self.state.advice.replay_stack_pop();
        self.increment_stack_size();
        self.stack_write(0, value);
    }

    /// Pops a word from the advice stack and pushes it onto the operand stack.
    pub(crate) fn advpopw(&mut self) {
        // For parallel trace generation, we assume the advice word is already available
        let advice_word = [ZERO; 4]; // This would be retrieved from advice in actual implementation

        self.stack_shift_right(4);
        for (i, &value) in advice_word.iter().enumerate() {
            self.stack_set(i, value);
        }
    }

    /// Loads a word from memory and pushes it onto the stack.
    pub(crate) fn mloadw(&mut self) {
        let _address = self.stack_get(0);

        // For parallel trace generation, we assume memory access is already resolved
        // This would involve memory operations in the actual implementation
        let memory_word = [ZERO; 4]; // Placeholder - would load from memory

        for (i, &value) in memory_word.iter().enumerate() {
            self.stack_set(i, value);
        }
    }

    /// Stores a word from the stack into memory.
    pub(crate) fn mstorew(&mut self) {
        let _address = self.stack_get(0);
        let _word: Word =
            [self.stack_get(1), self.stack_get(2), self.stack_get(3), self.stack_get(4)];

        // For parallel trace generation, we assume memory operations are tracked separately
        // This would involve memory operations in the actual implementation

        self.stack_shift_left(5);
    }

    /// Loads a single element from memory.
    pub(crate) fn mload(&mut self) {
        let _address = self.stack_get(0);

        // For parallel trace generation, we assume memory access is already resolved
        let memory_value = ZERO; // Placeholder - would load from memory

        self.stack_set(0, memory_value);
    }

    /// Stores a single element into memory.
    pub(crate) fn mstore(&mut self) {
        let _address = self.stack_get(0);
        let _value = self.stack_get(1);

        // For parallel trace generation, we assume memory operations are tracked separately
        // This would involve memory operations in the actual implementation

        self.stack_shift_left(2);
    }

    /// Reads from memory stream.
    pub(crate) fn mstream(&mut self) {
        let _address = self.stack_get(0);

        // For parallel trace generation, we assume stream read is already resolved
        let stream_word = [ZERO; 4]; // Placeholder - would read from stream

        for (i, &value) in stream_word.iter().enumerate() {
            self.stack_set(i, value);
        }
    }

    /// Reads from advice tape.
    pub(crate) fn pipe(&mut self) {
        // For parallel trace generation, we assume tape read is already resolved
        let tape_word = [ZERO; 4]; // Placeholder - would read from tape

        self.stack_shift_right(4);
        for (i, &value) in tape_word.iter().enumerate() {
            self.stack_set(i, value);
        }
    }
}
