use crate::MemAdviceProvider;

use super::{ExecutionError, Felt, ProcessState};
use vm_core::{HostFunction, HostResult};

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
    ) -> Result<HostResult, ExecutionError>;
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
    ) -> Result<HostResult, ExecutionError> {
        match function {
            HostFunction::AdviceFunction(advice_function) => {
                self.adv_provider.handle_advice_function(process, advice_function)
            }
            HostFunction::Debug(options) => self.handle_debug(process, options),
        }
    }
}
