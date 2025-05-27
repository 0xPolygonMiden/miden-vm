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

    /// Adds the specified value to the frame pointer.
    pub(crate) fn fmpadd(&mut self) {
        let value = self.stack_get(0);
        self.state.system.fmp += value;
        self.stack_set(0, self.state.system.fmp);
    }

    /// Updates the frame pointer.
    pub(crate) fn fmpupdate(&mut self) {
        let new_fmp = self.stack_get(0);
        let old_fmp = self.state.system.fmp;
        self.state.system.fmp = new_fmp;
        self.stack_set(0, old_fmp);
    }

    /// Pushes the current stack depth onto the stack.
    pub(crate) fn sdepth(&mut self) {
        let depth = self.state.stack.stack_depth();
        self.stack_shift_right(1);
        self.stack_set(0, depth);
    }

    /// Pushes the caller's context ID onto the stack.
    pub(crate) fn caller(&mut self) {
        let caller_id = self.state.system.ctx;
        self.stack_shift_right(1);
        self.stack_set(0, caller_id.into());
    }

    /// Pushes the current clock cycle onto the stack.
    pub(crate) fn clk(&mut self) {
        let clk = self.state.system.clk;
        self.stack_shift_right(1);
        self.stack_set(0, clk.into());
    }

    /// We don't actually emit events in the parallel trace generation context; the actual event
    /// handling would be done by the fast processor.
    pub(crate) fn emit(&mut self, _event_id: u32) {
        // No operation needed for parallel trace generation
    }
}
