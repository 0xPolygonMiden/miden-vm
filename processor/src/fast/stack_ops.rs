use vm_core::{WORD_SIZE, ZERO};

use super::FastProcessor;
use crate::ExecutionError;

impl FastProcessor {
    /// Analogous to `Process::op_pad`.
    pub fn op_pad(&mut self) {
        self.stack[self.stack_top_idx] = ZERO;
        self.increment_stack_size();
    }

    /// Analogous to `Process::op_swap`.
    pub fn op_swap(&mut self) {
        self.stack.swap(self.stack_top_idx - 1, self.stack_top_idx - 2);
    }

    /// Analogous to `Process::op_swapdw`.
    pub fn op_swap_double_word(&mut self) {
        self.stack.swap(self.stack_top_idx - 1, self.stack_top_idx - 9);
        self.stack.swap(self.stack_top_idx - 2, self.stack_top_idx - 10);
        self.stack.swap(self.stack_top_idx - 3, self.stack_top_idx - 11);
        self.stack.swap(self.stack_top_idx - 4, self.stack_top_idx - 12);
        self.stack.swap(self.stack_top_idx - 5, self.stack_top_idx - 13);
        self.stack.swap(self.stack_top_idx - 6, self.stack_top_idx - 14);
        self.stack.swap(self.stack_top_idx - 7, self.stack_top_idx - 15);
        self.stack.swap(self.stack_top_idx - 8, self.stack_top_idx - 16);
    }

    /// Rotates the top `n` elements of the stack to the left by 1.
    ///
    /// For example, if the stack is [a, b, c, d], with `d` at the top, then `rotate_left(3)` will
    /// result in the top 3 elements being rotated left: [a, c, d, b].
    ///
    /// This operation is useful for implementing the `movup` instructions.
    ///
    /// The stack size doesn't change.
    #[inline(always)]
    pub fn rotate_left(&mut self, n: usize) {
        let rotation_bot_index = self.stack_top_idx - n;
        let new_stack_top_element = self.stack[rotation_bot_index];

        // shift the top n elements down by 1, starting from the bottom of the rotation.
        for i in 0..n - 1 {
            self.stack[rotation_bot_index + i] = self.stack[rotation_bot_index + i + 1];
        }

        // Set the top element (which comes from the bottom of the rotation).
        self.stack[self.stack_top_idx - 1] = new_stack_top_element;
    }

    /// Rotates the top `n` elements of the stack to the right by 1.
    ///
    /// Analogous to `rotate_left`, but in the opposite direction.
    #[inline(always)]
    pub fn rotate_right(&mut self, n: usize) {
        let rotation_bot_index = self.stack_top_idx - n;
        let new_stack_bot_element = self.stack[self.stack_top_idx - 1];

        // shift the top n elements up by 1, starting from the top of the rotation.
        for i in 1..n {
            self.stack[self.stack_top_idx - i] = self.stack[self.stack_top_idx - i - 1];
        }

        // Set the bot element (which comes from the top of the rotation).
        self.stack[rotation_bot_index] = new_stack_bot_element;
    }

    /// Duplicates the n'th element from the top of the stack to the top of the stack.
    ///
    /// The size of the stack is incremented by 1.
    #[inline(always)]
    pub fn dup_nth(&mut self, n: usize) {
        let to_dup_index = self.stack_top_idx - n - 1;
        self.stack[self.stack_top_idx] = self.stack[to_dup_index];
        self.increment_stack_size();
    }

    /// Swaps the nth word from the top of the stack with the top word of the stack.
    ///
    /// Valid values of `n` are 1, 2, and 3.
    pub fn swapw_nth(&mut self, n: usize) {
        // For example, for n=3, the stack words and variables look like:
        //    3     2     1     0
        // | ... | ... | ... | ... |
        // ^                 ^
        // nth_word       top_word
        let (rest_of_stack, top_word) = self.stack.split_at_mut(self.stack_top_idx - WORD_SIZE);
        let (_, nth_word) = rest_of_stack.split_at_mut(rest_of_stack.len() - n * WORD_SIZE);

        nth_word[0..WORD_SIZE].swap_with_slice(&mut top_word[0..WORD_SIZE]);
    }

    /// Analogous to `Process::op_cswap`.
    pub fn op_cswap(&mut self) -> Result<(), ExecutionError> {
        let condition = self.stack[self.stack_top_idx - 1];
        let b = self.stack[self.stack_top_idx - 2];
        let a = self.stack[self.stack_top_idx - 3];

        match condition.as_int() {
            0 => {
                // do nothing, a and b are already in the right place
            },
            1 => {
                self.stack[self.stack_top_idx - 2] = a;
                self.stack[self.stack_top_idx - 3] = b;
            },
            _ => return Err(ExecutionError::NotBinaryValue(condition)),
        }

        self.decrement_stack_size();
        Ok(())
    }

    /// Analogous to `Process::op_cswapw`.
    pub fn op_cswapw(&mut self) -> Result<(), ExecutionError> {
        let condition = self.stack[self.stack_top_idx - 1];

        // b is the top word of the stack, a is the 2rd word from the top of the stack.
        // The indices are chosen assuming that `condition` is removed from the stack.
        let b_word_start_idx = self.stack_top_idx - 1 - WORD_SIZE;
        let a_word_start_idx = self.stack_top_idx - 1 - (2 * WORD_SIZE);

        match condition.as_int() {
            0 => {
                // do nothing, the words are already in the right place
            },
            1 => {
                self.stack.swap(b_word_start_idx, a_word_start_idx);
                self.stack.swap(b_word_start_idx + 1, a_word_start_idx + 1);
                self.stack.swap(b_word_start_idx + 2, a_word_start_idx + 2);
                self.stack.swap(b_word_start_idx + 3, a_word_start_idx + 3);
            },
            _ => return Err(ExecutionError::NotBinaryValue(condition)),
        }

        self.decrement_stack_size();
        Ok(())
    }
}
