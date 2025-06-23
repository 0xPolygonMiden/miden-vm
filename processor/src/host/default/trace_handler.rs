use crate::{AdviceProvider, ExecutionError, ProcessState, handlers::TraceHandler};

/// Default implementation that does nothing
#[derive(Clone, Default)]
pub struct DefaultTraceHandler;

impl TraceHandler for DefaultTraceHandler {
    fn on_trace(
        &mut self,
        _advice: &dyn AdviceProvider,
        _process: ProcessState,
        _trace_id: u32,
    ) -> Result<(), ExecutionError> {
        #[cfg(feature = "std")]
        std::println!(
            "Trace with id {} emitted at step {} in context {}",
            _trace_id,
            _process.clk(),
            _process.ctx()
        );
        Ok(())
    }
}
