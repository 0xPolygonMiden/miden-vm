use crate::{AdviceProvider, ExecutionError, ProcessState, host::TraceHandler};

/// Default implementation that does nothing
#[derive(Clone, Default)]
pub struct DefaultTraceHandler;

impl<A: AdviceProvider> TraceHandler<A> for DefaultTraceHandler {
    fn on_trace(
        &mut self,
        _advice: &A,
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
