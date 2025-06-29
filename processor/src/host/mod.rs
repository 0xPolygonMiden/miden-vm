use alloc::sync::Arc;

use vm_core::{DebugOptions, Felt, Word, mast::MastForest};

use crate::{ExecutionError, ProcessState, RowIndex, errors::ErrorContext};

pub(super) mod advice;
use advice::AdviceProvider;

#[cfg(feature = "std")]
mod debug;

mod mast_forest_store;
pub use mast_forest_store::{MastForestStore, MemMastForestStore};
// HOST TRAIT
// ================================================================================================

/// Defines an interface by which the VM can interact with the host.
///
/// There are four main categories of interactions between the VM and the host:
/// 1. accessing the advice provider,
/// 2. getting a library's MAST forest,
/// 3. handling advice events (which internally mutates the advice provider), and
/// 4. handling debug and trace events.
pub trait Host {
    // REQUIRED METHODS
    // --------------------------------------------------------------------------------------------

    /// Returns a reference to the advice provider.
    fn advice_provider(&self) -> &AdviceProvider;

    /// Returns a mutable reference to the advice provider.
    fn advice_provider_mut(&mut self) -> &mut AdviceProvider;

    /// Returns MAST forest corresponding to the specified digest, or None if the MAST forest for
    /// this digest could not be found in this [Host].
    fn get_mast_forest(&mut self, node_digest: &Word) -> Option<Arc<MastForest>>;

    // PROVIDED METHODS
    // --------------------------------------------------------------------------------------------

    /// Handles the event emitted from the VM.
    fn on_event(
        &mut self,
        _process: ProcessState,
        _event_id: u32,
        _err_ctx: &impl ErrorContext,
    ) -> Result<(), ExecutionError> {
        #[cfg(feature = "std")]
        std::println!(
            "Event with id {} emitted at step {} in context {}",
            _event_id,
            _process.clk(),
            _process.ctx()
        );
        Ok(())
    }

    /// Handles the debug request from the VM.
    fn on_debug(
        &mut self,
        _process: ProcessState,
        _options: &DebugOptions,
    ) -> Result<(), ExecutionError>
    where
        Self: Sized,
    {
        #[cfg(feature = "std")]
        debug::print_debug_info(self, _process, _options);
        Ok(())
    }

    /// Handles the trace emitted from the VM.
    fn on_trace(&mut self, _process: ProcessState, _trace_id: u32) -> Result<(), ExecutionError> {
        #[cfg(feature = "std")]
        std::println!(
            "Trace with id {} emitted at step {} in context {}",
            _trace_id,
            _process.clk(),
            _process.ctx()
        );
        Ok(())
    }

    /// Handles the failure of the assertion instruction.
    fn on_assert_failed(&mut self, _process: ProcessState, _err_code: Felt) {}
}

impl<H> Host for &mut H
where
    H: Host,
{
    fn advice_provider(&self) -> &AdviceProvider {
        H::advice_provider(self)
    }

    fn advice_provider_mut(&mut self) -> &mut AdviceProvider {
        H::advice_provider_mut(self)
    }

    fn get_mast_forest(&mut self, node_digest: &Word) -> Option<Arc<MastForest>> {
        H::get_mast_forest(self, node_digest)
    }

    fn on_event(
        &mut self,
        process: ProcessState,
        event_id: u32,
        err_ctx: &impl ErrorContext,
    ) -> Result<(), ExecutionError> {
        H::on_event(self, process, event_id, err_ctx)
    }

    fn on_debug(
        &mut self,
        process: ProcessState,
        options: &DebugOptions,
    ) -> Result<(), ExecutionError> {
        H::on_debug(self, process, options)
    }

    fn on_trace(&mut self, process: ProcessState, trace_id: u32) -> Result<(), ExecutionError> {
        H::on_trace(self, process, trace_id)
    }

    fn on_assert_failed(&mut self, process: ProcessState, err_code: Felt) {
        H::on_assert_failed(self, process, err_code)
    }
}

// DEFAULT HOST IMPLEMENTATION
// ================================================================================================

/// A default [Host] implementation that provides the essential functionality required by the VM.
#[derive(Debug, Clone, Default)]
pub struct DefaultHost {
    adv_provider: AdviceProvider,
    store: MemMastForestStore,
}

impl DefaultHost {
    pub fn new(adv_provider: AdviceProvider) -> Self {
        Self {
            adv_provider,
            store: MemMastForestStore::default(),
        }
    }

    pub fn load_mast_forest(&mut self, mast_forest: Arc<MastForest>) -> Result<(), ExecutionError> {
        // Load the MAST's advice data into the advice provider.
        self.adv_provider
            .merge_advice_map(mast_forest.advice_map())
            .map_err(|err| ExecutionError::advice_error(err, RowIndex::from(0), &()))?;

        self.store.insert(mast_forest);
        Ok(())
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn advice_provider(&self) -> &AdviceProvider {
        &self.adv_provider
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn advice_provider_mut(&mut self) -> &mut AdviceProvider {
        &mut self.adv_provider
    }
}

impl Host for DefaultHost {
    fn advice_provider(&self) -> &AdviceProvider {
        &self.adv_provider
    }

    fn advice_provider_mut(&mut self) -> &mut AdviceProvider {
        &mut self.adv_provider
    }

    fn get_mast_forest(&mut self, node_digest: &Word) -> Option<Arc<MastForest>> {
        self.store.get(node_digest)
    }

    fn on_event(
        &mut self,
        _process: ProcessState,
        _event_id: u32,
        _err_ctx: &impl ErrorContext,
    ) -> Result<(), ExecutionError> {
        #[cfg(feature = "std")]
        std::println!(
            "Event with id {} emitted at step {} in context {}",
            _event_id,
            _process.clk(),
            _process.ctx()
        );
        Ok(())
    }
}
