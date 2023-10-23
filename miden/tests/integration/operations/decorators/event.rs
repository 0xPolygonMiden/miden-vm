use assembly::Assembler;
use processor::{
    AdviceExtractor, AdviceProvider, ExecutionError, Host, HostResponse, MemAdviceProvider,
    ProcessState,
};
use vm_core::AdviceInjector;

// EVENT TEST HOST
// ================================================================================================
pub struct TestEventHost<A> {
    pub adv_provider: A,
    pub event_handler: Vec<u32>,
}

impl Default for TestEventHost<MemAdviceProvider> {
    fn default() -> Self {
        Self {
            adv_provider: MemAdviceProvider::default(),
            event_handler: Vec::new(),
        }
    }
}

impl<A: AdviceProvider> Host for TestEventHost<A> {
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
}

// TESTS
// ================================================================================================

#[test]
fn test_event_handling() {
    let source = "\
    begin
        push.1
        emit.1
        push.2
        emit.2
    end";

    // compile and execute program
    let program = Assembler::default().compile(source).unwrap();
    let mut host = TestEventHost::default();
    processor::execute(&program, Default::default(), &mut host, Default::default()).unwrap();

    // make sure events were handled correctly
    let expected = vec![1, 2];
    assert_eq!(host.event_handler, expected);
}
