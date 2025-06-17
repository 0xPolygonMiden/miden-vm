use core::error::Error;

use vm_core::{
    DebugOptions,
    mast::{MastForest, MastNodeExt},
};

use crate::{
    AdviceProvider, Arc, Box, ErrorContext, ExecutionError, Host, KvMap, MastForestStore,
    MemAdviceProvider, MemMastForestStore, ProcessState, Vec,
    crypto::RpoDigest,
    host::{DebugHandler, TraceHandler},
};

mod event_handler;
use crate::host::default::event_handler::EventHandlerRegistry;

mod debug_handler;

pub use crate::host::default::debug_handler::DefaultDebugHandler;
mod trace_handler;
pub use crate::host::default::trace_handler::DefaultTraceHandler;

/// Trait for handling VM events
/// # TODO
/// - Does the handler need to be stateful?
pub trait EventHandler<A> {
    /// Returns the event ID this handler responds to
    fn id(&self) -> u32;

    /// Handles the event when triggered
    fn on_event(
        &mut self,
        advice_provider: &mut A,
        process: ProcessState,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
}

/// Trait for libraries that want to provide event handlers to hosts
pub trait HostLibrary<A> {
    /// Returns all event handlers defined by this library
    fn get_event_handlers(&self) -> Vec<Box<dyn EventHandler<A>>>;

    /// Returns the MAST forest for this library
    fn get_mast_forest(&self) -> Arc<MastForest>;
}

// DEFAULT HOST IMPLEMENTATION
// ================================================================================================

/// A default [Host] implementation that provides the essential functionality required by the VM.
pub struct DefaultHost<A, D = DefaultDebugHandler, T = DefaultTraceHandler> {
    adv_provider: A,
    store: MemMastForestStore,
    event_handlers: EventHandlerRegistry<A>,
    debug_handler: D,
    trace_handler: T,
}

impl<A: AdviceProvider, T: TraceHandler<A>> DefaultHost<A, DefaultDebugHandler, T> {
    pub fn with_debug_handler<D: DebugHandler<A>>(self, handler: D) -> DefaultHost<A, D, T> {
        DefaultHost {
            adv_provider: self.adv_provider,
            store: self.store,
            event_handlers: self.event_handlers,
            debug_handler: handler,
            trace_handler: self.trace_handler,
        }
    }
}

impl<A: AdviceProvider, D: DebugHandler<A>> DefaultHost<A, D, DefaultTraceHandler> {
    pub fn with_trace_handler<T: TraceHandler<A>>(self, handler: T) -> DefaultHost<A, D, T> {
        DefaultHost {
            adv_provider: self.adv_provider,
            store: self.store,
            event_handlers: self.event_handlers,
            debug_handler: self.debug_handler,
            trace_handler: handler,
        }
    }
}

impl<A: AdviceProvider> DefaultHost<A> {
    pub fn new(adv_provider: A) -> Self {
        Self {
            adv_provider,
            store: MemMastForestStore::default(),
            event_handlers: EventHandlerRegistry::new(),
            debug_handler: DefaultDebugHandler,
            trace_handler: DefaultTraceHandler,
        }
    }
}

impl<A: AdviceProvider, D: DebugHandler<A>, T: TraceHandler<A>> DefaultHost<A, D, T> {
    pub fn load_mast_forest(&mut self, mast_forest: Arc<MastForest>) -> Result<(), ExecutionError> {
        // Load the MAST's advice data into the advice provider.
        for (digest, values) in mast_forest.advice_map().iter() {
            if let Some(stored_values) = self.advice_provider().get_mapped_values(digest) {
                if stored_values != values {
                    return Err(ExecutionError::AdviceMapKeyAlreadyPresent {
                        key: digest.into(),
                        prev_values: stored_values.to_vec(),
                        new_values: values.clone(),
                    });
                }
            } else {
                self.advice_provider_mut().insert_into_map(digest.into(), values.clone());
            }
        }

        self.store.insert(mast_forest);
        Ok(())
    }

    pub fn load_library(&mut self, library: &impl HostLibrary<A>) -> Result<(), ExecutionError> {
        // Load the MAST forest
        self.load_mast_forest(library.get_mast_forest().clone())?;

        // Register event handlers
        self.event_handlers.register_many(library.get_event_handlers())?;

        Ok(())
    }

    pub fn into_inner(self) -> A {
        self.adv_provider
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn advice_provider(&self) -> &A {
        &self.adv_provider
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn advice_provider_mut(&mut self) -> &mut A {
        &mut self.adv_provider
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn debug_handler(&self) -> &D {
        &self.debug_handler
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn debug_handler_mut(&mut self) -> &mut D {
        &mut self.debug_handler
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn trace_handler(&self) -> &T {
        &self.trace_handler
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn trace_handler_mut(&mut self) -> &mut T {
        &mut self.trace_handler
    }
}

impl<A: AdviceProvider, D: DebugHandler<A>, T: TraceHandler<A>> Host for DefaultHost<A, D, T> {
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

    fn on_event(
        &mut self,
        process: ProcessState,
        event_id: u32,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Result<(), ExecutionError> {
        if let Some(handler) = self.event_handlers.get(event_id) {
            handler
                .on_event(&mut self.adv_provider, process)
                .map_err(|err| ExecutionError::event_error(event_id, err, err_ctx))
        } else {
            Err(ExecutionError::invalid_event_id_error(event_id, err_ctx))
        }
    }

    fn on_debug(
        &mut self,
        process: ProcessState,
        options: &DebugOptions,
    ) -> Result<(), ExecutionError> {
        self.debug_handler.on_debug(&self.adv_provider, process, options)
    }

    fn on_trace(&mut self, process: ProcessState, trace_id: u32) -> Result<(), ExecutionError> {
        self.trace_handler.on_trace(&self.adv_provider, process, trace_id)
    }
}

impl Default for DefaultHost<MemAdviceProvider> {
    fn default() -> Self {
        Self {
            adv_provider: MemAdviceProvider::default(),
            store: MemMastForestStore::default(),
            event_handlers: EventHandlerRegistry::default(),
            debug_handler: DefaultDebugHandler,
            trace_handler: DefaultTraceHandler,
        }
    }
}
