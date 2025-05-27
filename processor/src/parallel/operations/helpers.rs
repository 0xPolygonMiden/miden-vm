use vm_core::{Felt, ZERO};

use super::MainTraceFragmentGenerator;

impl MainTraceFragmentGenerator {
    /// Helper method to get a value from stack position
    pub(super) fn stack_get(&self, pos: usize) -> Felt {
        self.state.stack.stack_top[pos]
    }

    /// Helper method to set a value at stack position
    pub(super) fn stack_set(&mut self, pos: usize, value: Felt) {
        self.state.stack.stack_top[pos] = value;
    }

    /// Helper method to shift stack left by n positions (simulating pop operations)
    pub(super) fn stack_shift_left(&mut self, n: usize) {
        for i in 0..(self.state.stack.stack_top.len() - n) {
            self.state.stack.stack_top[i] = self.state.stack.stack_top[i + n];
        }
        // Fill the remaining positions with ZERO (or keep existing overflow data)
        for i in (self.state.stack.stack_top.len() - n)..self.state.stack.stack_top.len() {
            self.state.stack.stack_top[i] = ZERO;
        }
        // Note: In the refactored version, stack depth is derived from the overflow table.
        // Actual stack operations would need to manipulate the overflow table accordingly.
    }

    /// Helper method to shift stack right by n positions (simulating push operations)
    pub(super) fn stack_shift_right(&mut self, n: usize) {
        // Shift elements to the right
        for i in (n..self.state.stack.stack_top.len()).rev() {
            self.state.stack.stack_top[i] = self.state.stack.stack_top[i - n];
        }
        // Clear the new top positions
        for i in 0..n {
            self.state.stack.stack_top[i] = ZERO;
        }
        // Note: In the refactored version, stack depth is derived from the overflow table.
        // Actual stack operations would need to manipulate the overflow table accordingly.
    }
}
