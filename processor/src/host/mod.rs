use alloc::{boxed::Box, sync::Arc, vec::Vec};

use vm_core::{crypto::hash::RpoDigest, mast::MastForest, DebugOptions};

use super::{ExecutionError, ProcessState};
use crate::{KvMap, MemAdviceProvider};

pub(super) mod advice;
use advice::AdviceProvider;

#[cfg(feature = "std")]
mod debug;

mod event_handling;
pub use event_handling::{EventHandler, EventHandlerRegistry};

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
    type AdviceProvider: AdviceProvider;

    // REQUIRED METHODS
    // --------------------------------------------------------------------------------------------

    /// Returns a reference to the advice provider.
    fn advice_provider(&self) -> &Self::AdviceProvider;

    /// Returns a mutable reference to the advice provider.
    fn advice_provider_mut(&mut self) -> &mut Self::AdviceProvider;

    /// Returns MAST forest corresponding to the specified digest, or None if the MAST forest for
    /// this digest could not be found in this [Host].
    fn get_mast_forest(&self, node_digest: &RpoDigest) -> Option<Arc<MastForest>>;

    /// Handles the event emitted from the VM.
    fn on_event(&mut self, _process: ProcessState, _event_id: u32) -> Result<(), ExecutionError>;

    /// Handles the debug request from the VM.
    fn on_debug(
        &mut self,
        _process: ProcessState,
        _options: &DebugOptions,
    ) -> Result<(), ExecutionError>;

    /// Handles the trace emitted from the VM.
    fn on_trace(&mut self, _process: ProcessState, _trace_id: u32) -> Result<(), ExecutionError>;

    // PROVIDED METHODS
    // --------------------------------------------------------------------------------------------

    /// Handles the failure of the assertion instruction.
    fn on_assert_failed(&mut self, process: ProcessState, err_code: u32) -> ExecutionError {
        ExecutionError::FailedAssertion {
            clk: process.clk(),
            err_code,
            err_msg: None,
        }
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

    fn get_mast_forest(&self, node_digest: &RpoDigest) -> Option<Arc<MastForest>> {
        H::get_mast_forest(self, node_digest)
    }

    fn on_debug(
        &mut self,
        process: ProcessState,
        options: &DebugOptions,
    ) -> Result<(), ExecutionError> {
        H::on_debug(self, process, options)
    }

    fn on_event(&mut self, process: ProcessState, event_id: u32) -> Result<(), ExecutionError> {
        H::on_event(self, process, event_id)
    }

    fn on_trace(&mut self, process: ProcessState, trace_id: u32) -> Result<(), ExecutionError> {
        H::on_trace(self, process, trace_id)
    }

    fn on_assert_failed(&mut self, process: ProcessState, err_code: u32) -> ExecutionError {
        H::on_assert_failed(self, process, err_code)
    }
}

// HOST LIBRARY
// ================================================================================================

pub trait HostLibrary {
    // Returns all event handlers
    fn get_event_handlers<A>(&self) -> Vec<Box<dyn EventHandler<A>>>
    where
        A: AdviceProvider + 'static;

    fn get_mast_forest(&self) -> Arc<MastForest>;
}

// DEFAULT HOST IMPLEMENTATION
// ================================================================================================

/// A default [Host] implementation that provides the essential functionality required by the VM.
pub struct DefaultHost<A, T, D> {
    adv_provider: A,
    store: MemMastForestStore,
    event_registry: EventHandlerRegistry<A>,
    trace_handler: T,
    debug_handler: D,
}

impl Default for DefaultHost<MemAdviceProvider, DefaultTraceHandler, DefaultDebugHandler> {
    fn default() -> Self {
        Self {
            adv_provider: MemAdviceProvider::default(),
            store: MemMastForestStore::default(),
            event_registry: EventHandlerRegistry::default(),
            trace_handler: DefaultTraceHandler,
            debug_handler: DefaultDebugHandler,
        }
    }
}

impl<A> DefaultHost<A, DefaultTraceHandler, DefaultDebugHandler> {
    pub fn with_advice_provider(
        self,
        adv_provider: A,
    ) -> DefaultHost<A, DefaultTraceHandler, DefaultDebugHandler> {
        DefaultHost { adv_provider, ..self }
    }
}

impl<A, D> DefaultHost<A, DefaultTraceHandler, D>
where
    A: AdviceProvider,
    D: DebugHandler,
{
    pub fn with_debug_handler(self, debug_handler: D) -> DefaultHost<A, DefaultTraceHandler, D> {
        DefaultHost { debug_handler, ..self }
    }
}

impl<T, D> DefaultHost<MemAdviceProvider, T, D>
where
    T: TraceHandler,
    D: DebugHandler,
{
    pub fn with_trace_handler(self, trace_handler: T) -> DefaultHost<MemAdviceProvider, T, D> {
        DefaultHost { trace_handler, ..self }
    }
}

impl<A, T, D> DefaultHost<A, T, D>
where
    A: AdviceProvider + 'static,
    T: TraceHandler,
    D: DebugHandler,
{
    // TODO(plafer): make sure we use this in the tests when it makes sense
    /// Loads the specified library into the host.
    pub fn load_library(&mut self, library: &impl HostLibrary) -> Result<(), ExecutionError> {
        self.load_mast_forest(library.get_mast_forest())?;
        self.event_registry
            .register_event_handlers(library.get_event_handlers().into_iter());

        Ok(())
    }

    /// Loads the specified MAST forest into the host.
    ///
    /// Using [Self::load_library] is recommended over this method so that any event handlers
    /// provided by a library are also registered.
    pub fn load_mast_forest(&mut self, mast_forest: Arc<MastForest>) -> Result<(), ExecutionError> {
        // Load the MAST's advice data into the advice provider.

        for (digest, values) in mast_forest.advice_map().iter() {
            if let Some(stored_values) = self.advice_provider().get_mapped_values(digest) {
                if stored_values != values {
                    return Err(ExecutionError::AdviceMapKeyAlreadyPresent(digest.into()));
                }
            } else {
                self.advice_provider_mut().insert_into_map(digest.into(), values.clone());
            }
        }

        self.store.insert(mast_forest);
        Ok(())
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn advice_provider(&self) -> &A {
        &self.adv_provider
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn advice_provider_mut(&mut self) -> &mut A {
        &mut self.adv_provider
    }
}

impl<A, T, D> Host for DefaultHost<A, T, D>
where
    A: AdviceProvider,
    T: TraceHandler,
    D: DebugHandler,
{
    type AdviceProvider = A;

    fn advice_provider(&self) -> &Self::AdviceProvider {
        &self.adv_provider
    }

    fn advice_provider_mut(&mut self) -> &mut Self::AdviceProvider {
        &mut self.adv_provider
    }

    fn get_mast_forest(&self, node_digest: &RpoDigest) -> Option<Arc<MastForest>> {
        self.store.get(node_digest)
    }

    fn on_event(&mut self, process: ProcessState, event_id: u32) -> Result<(), ExecutionError> {
        let handler = self
            .event_registry
            .get_event_handler(event_id)
            .ok_or_else(|| ExecutionError::EventHandlerNotFound { event_id, clk: process.clk() })?;

        handler.on_event(process, &mut self.adv_provider)
    }

    fn on_debug(
        &mut self,
        process: ProcessState,
        options: &DebugOptions,
    ) -> Result<(), ExecutionError> {
        self.debug_handler.on_debug(process, options)
    }

    fn on_trace(&mut self, process: ProcessState, trace_id: u32) -> Result<(), ExecutionError> {
        self.trace_handler.on_trace(process, trace_id)
    }
}

// EVENT HANDLERS
// ================================================================================================

/// Provides a method to handle trace events emitted from the VM.
pub trait TraceHandler {
    fn on_trace(&mut self, process: ProcessState, trace_id: u32) -> Result<(), ExecutionError>;
}

/// A default implementation of the [TraceHandler] trait which prints the trace event to the
/// console.
#[derive(Debug)]
pub struct DefaultTraceHandler;

impl TraceHandler for DefaultTraceHandler {
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
}

/// Provides a method to handle debug events emitted from the VM.
pub trait DebugHandler {
    fn on_debug(
        &mut self,
        process: ProcessState,
        options: &DebugOptions,
    ) -> Result<(), ExecutionError>;
}

/// A default implementation of the [DebugHandler] trait which prints the debug information to the
/// console.
#[derive(Debug)]
pub struct DefaultDebugHandler;

impl DebugHandler for DefaultDebugHandler {
    fn on_debug(
        &mut self,
        _process: ProcessState,
        _options: &DebugOptions,
    ) -> Result<(), ExecutionError> {
        #[cfg(feature = "std")]
        debug::print_debug_info(_process, _options);
        Ok(())
    }
}
