use alloc::{boxed::Box, sync::Arc, vec::Vec};

use vm_core::{
    DebugOptions, Felt, Word,
    mast::{MastForest, MastNodeExt},
};

use crate::{ErrorContext, ExecutionError, ProcessState, handlers::EventHandler};

pub mod advice;
use advice::AdviceProvider;

mod mast_forest_store;
pub use mast_forest_store::{MastForestStore, MemMastForestStore};

pub mod default;

pub mod handlers;

// HOST TRAIT
// ================================================================================================

/// Defines an interface by which the VM can interact with the host.
///
/// There are four main categories of interactions between the VM and the host:
/// 1. accessing the advice provider,
/// 2. getting a library's MAST forest,
/// 3. handling advice events (which internally mutates the advice provider), and
/// 4. handling debug and trace events.
/// 5. handling custom events.
pub trait Host {
    type AdviceProvider: AdviceProvider;

    // REQUIRED METHODS
    // --------------------------------------------------------------------------------------------

    /// Returns a reference to the advice provider.
    fn advice_provider(&self) -> &Self::AdviceProvider;

    /// Returns a mutable reference to the advice provider.
    fn advice_provider_mut(&mut self) -> &mut Self::AdviceProvider;

    /// Returns MAST forest corresponding to the specified digest, or None if the MAST forest for
    /// this digest could not be found in this [Host].
    fn get_mast_forest(&self, node_digest: &Word) -> Option<Arc<MastForest>>;

    /// Handles the event emitted from the VM.
    fn on_event(
        &mut self,
        process: ProcessState,
        event_id: u32,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Result<(), ExecutionError>;

    /// Handles the debug request from the VM.
    fn on_debug(
        &mut self,
        process: ProcessState,
        options: &DebugOptions,
    ) -> Result<(), ExecutionError>;

    /// Handles the trace emitted from the VM.
    fn on_trace(&mut self, _process: ProcessState, _trace_id: u32) -> Result<(), ExecutionError>;

    // PROVIDED METHODS
    // --------------------------------------------------------------------------------------------

    /// Handles the failure of the assertion instruction.
    fn on_assert_failed(&mut self, _process: ProcessState, _err_code: Felt) {}
}

// HOST LIBRARY
// ================================================================================================

/// Trait for libraries that want to provide event handlers to a (Default) Host
pub trait HostLibrary {
    /// Returns all event handlers defined by this library
    fn event_handlers(&self) -> Vec<Box<dyn EventHandler>> {
        Vec::default()
    }

    /// Returns the MAST forest for this library
    fn mast_forest(&self) -> Arc<MastForest>;
}

/// Default implementation for loading a MastForest without handlers.
impl HostLibrary for Arc<MastForest> {
    fn mast_forest(&self) -> Arc<MastForest> {
        (*self).clone()
    }
}

impl<H> Host for &mut H
where
    H: Host,
{
    type AdviceProvider = H::AdviceProvider;

    fn advice_provider(&self) -> &Self::AdviceProvider {
        H::advice_provider(self)
    }

    fn advice_provider_mut(&mut self) -> &mut Self::AdviceProvider {
        H::advice_provider_mut(self)
    }

    fn get_mast_forest(&self, node_digest: &Word) -> Option<Arc<MastForest>> {
        H::get_mast_forest(self, node_digest)
    }

    fn on_event(
        &mut self,
        process: ProcessState,
        event_id: u32,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
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
