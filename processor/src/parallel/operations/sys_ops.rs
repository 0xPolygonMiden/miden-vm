use vm_core::ONE;

use super::CoreTraceFragmentGenerator;

impl CoreTraceFragmentGenerator {
    /// Asserts that the top element on the stack is 1.
    pub(crate) fn op_assert(&mut self) {
        let value = self.stack_get(0);
        if value != ONE {
            panic!(
                "Assertion failed: expected 1, got {} at clock {}",
                value, self.state.system.clk
            );
        }
        self.stack_shift_left(1);
    }
}
