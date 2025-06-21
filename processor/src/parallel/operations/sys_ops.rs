use vm_core::ONE;

use super::CoreTraceFragmentGenerator;
use crate::processor::Processor;

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
        self.decrement_stack_size();
    }
}
