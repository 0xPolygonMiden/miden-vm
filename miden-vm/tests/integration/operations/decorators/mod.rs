use std::sync::Arc;

use processor::{ErrorContext, ExecutionError, Host, MastForest, ProcessState};
use prover::Word;
use vm_core::DebugOptions;

mod advice;
mod asmop;
mod events;

// TEST HOST
// ================================================================================================
#[derive(Debug, Clone, Default)]
pub struct TestHost {
    pub event_handler: Vec<u32>,
    pub trace_handler: Vec<u32>,
    pub debug_handler: Vec<String>,
}

impl Host for TestHost {
    fn get_mast_forest(&mut self, _node_digest: &Word) -> Option<Arc<MastForest>> {
        // Empty MAST forest store
        None
    }

    fn iter_mast_forests(&self) -> impl Iterator<Item = Arc<MastForest>> {
        [].into_iter()
    }

    fn on_event(
        &mut self,
        _process: &mut ProcessState,
        event_id: u32,
        _err_ctx: &impl ErrorContext,
    ) -> Result<(), ExecutionError> {
        self.event_handler.push(event_id);
        Ok(())
    }

    fn on_debug(
        &mut self,
        _process: &mut ProcessState,
        options: &DebugOptions,
    ) -> Result<(), ExecutionError> {
        self.debug_handler.push(options.to_string());
        Ok(())
    }

    fn on_trace(
        &mut self,
        _process: &mut ProcessState,
        trace_id: u32,
    ) -> Result<(), ExecutionError> {
        self.trace_handler.push(trace_id);
        Ok(())
    }
}
