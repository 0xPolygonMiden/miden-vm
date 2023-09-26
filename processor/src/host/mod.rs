use crate::MemAdviceProvider;

use super::{ExecutionError, Felt, ProcessState, Word};
use vm_core::HostFunction;

pub(super) mod advice;
use advice::AdviceProvider;

mod debug;
use debug::DebugHandler;

// HOST TRAIT
// ================================================================================================

pub trait Host {
    fn execute_host_function<S: ProcessState>(
        &mut self,
        process: &S,
        function: &HostFunction,
    ) -> Result<usize, ExecutionError>;

    fn pop_stack(&mut self) -> Result<Felt, ExecutionError>;

    fn pop_stack_word(&mut self) -> Result<Word, ExecutionError>;

    fn pop_stack_dword(&mut self) -> Result<[Word; 2], ExecutionError>;

    fn drain_stack_vec(&mut self, len: usize) -> Result<Vec<Felt>, ExecutionError>;
}

// DEFAULT HOST IMPLEMENTATION
// ================================================================================================

pub struct DefaultHost<A> {
    adv_provider: A,
}

impl Default for DefaultHost<MemAdviceProvider> {
    fn default() -> Self {
        Self {
            adv_provider: MemAdviceProvider::default(),
        }
    }
}

impl<A: AdviceProvider> DefaultHost<A> {
    pub fn new(adv_provider: A) -> Self {
        Self { adv_provider }
    }

    #[cfg(any(test, feature = "internals"))]
    pub fn advice_provider(&self) -> &A {
        &self.adv_provider
    }

    #[cfg(any(test, feature = "internals"))]
    pub fn advice_provider_mut(&mut self) -> &mut A {
        &mut self.adv_provider
    }
}

impl<A: AdviceProvider> DebugHandler for DefaultHost<A> {}

impl<A: AdviceProvider> Host for DefaultHost<A> {
    fn execute_host_function<S: ProcessState>(
        &mut self,
        process: &S,
        function: &HostFunction,
    ) -> Result<usize, ExecutionError> {
        match function {
            HostFunction::AdvanceClock => {
                self.adv_provider.advance_clock();
                Ok(0)
            }
            HostFunction::AdviceInjector(injector) => {
                self.adv_provider.handle_advice_injector(process, injector)
            }
            HostFunction::Debug(options) => self.handle_debug(process, options),
        }
    }

    fn pop_stack(&mut self) -> Result<Felt, ExecutionError> {
        self.adv_provider.pop_stack()
    }

    fn pop_stack_word(&mut self) -> Result<Word, ExecutionError> {
        self.adv_provider.pop_stack_word()
    }

    fn pop_stack_dword(&mut self) -> Result<[Word; 2], ExecutionError> {
        self.adv_provider.pop_stack_dword()
    }

    fn drain_stack_vec(&mut self, len: usize) -> Result<Vec<Felt>, ExecutionError> {
        self.adv_provider.drain_stack_vec(len)
    }
}
