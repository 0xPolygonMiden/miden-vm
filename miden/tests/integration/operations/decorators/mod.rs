use processor::{
    AdviceExtractor, AdviceProvider, ExecutionError, Host, HostResponse, MemAdviceProvider,
    ProcessState,
};
use vm_core::AdviceInjector;

mod advice;
mod asmop;
mod event;
mod trace;

// TEST HOST
// ================================================================================================
pub struct TestHost<A> {
    pub adv_provider: A,
    pub event_handler: Vec<u32>,
    pub trace_handler: Vec<u32>,
}

impl Default for TestHost<MemAdviceProvider> {
    fn default() -> Self {
        Self {
            adv_provider: MemAdviceProvider::default(),
            event_handler: Vec::new(),
            trace_handler: Vec::new(),
        }
    }
}

impl<A: AdviceProvider> Host for TestHost<A> {
    fn get_advice<S: ProcessState>(
        &mut self,
        process: &S,
        extractor: AdviceExtractor,
    ) -> Result<HostResponse, ExecutionError> {
        self.adv_provider.get_advice(process, &extractor)
    }

    fn set_advice<S: ProcessState>(
        &mut self,
        process: &S,
        injector: AdviceInjector,
    ) -> Result<HostResponse, ExecutionError> {
        self.adv_provider.set_advice(process, &injector)
    }

    fn on_event<S: ProcessState>(
        &mut self,
        _process: &S,
        event_id: u32,
    ) -> Result<HostResponse, ExecutionError> {
        self.event_handler.push(event_id);
        Ok(HostResponse::None)
    }

    fn on_trace<S: ProcessState>(
        &mut self,
        _process: &S,
        trace_id: u32,
    ) -> Result<HostResponse, ExecutionError> {
        self.trace_handler.push(trace_id);
        Ok(HostResponse::None)
    }
}
