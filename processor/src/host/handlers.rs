use vm_core::DebugOptions;

use crate::{ExecutionError, ProcessState};

// DEBUG HANDLER
// ================================================================================================

/// Handler for debug and trace operations
pub trait DebugHandler {
    /// This function is invoked when the `Debug` decorator is executed.
    fn on_debug(
        &mut self,
        process: &mut ProcessState,
        options: &DebugOptions,
    ) -> Result<(), ExecutionError> {
        let _ = (&process, options);
        #[cfg(feature = "std")]
        crate::host::debug::print_debug_info(process, options);
        Ok(())
    }

    /// This function is invoked when the `Trace` decorator is executed.
    fn on_trace(
        &mut self,
        process: &mut ProcessState,
        trace_id: u32,
    ) -> Result<(), ExecutionError> {
        let _ = (&process, trace_id);
        #[cfg(feature = "std")]
        std::println!(
            "Trace with id {} emitted at step {} in context {}",
            trace_id,
            process.clk(),
            process.ctx()
        );
        Ok(())
    }
}
