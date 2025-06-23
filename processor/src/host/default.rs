use alloc::{boxed::Box, collections::BTreeMap, sync::Arc, vec::Vec};

use vm_core::{
    DebugOptions, Word,
    mast::{MastForest, MastNodeExt},
    utils::collections::KvMap,
};

use crate::{
    AdviceProvider, ErrorContext, ExecutionError, Host, HostLibrary, MastForestStore,
    MemAdviceProvider, MemMastForestStore, ProcessState,
    handlers::{DebugHandler, EventHandler, TraceHandler},
};

mod debug_handler;
pub use debug_handler::DefaultDebugHandler;

mod trace_handler;
pub use trace_handler::DefaultTraceHandler;

// DEFAULT HOST IMPLEMENTATION
// ================================================================================================

/// A default [Host] implementation that provides the essential functionality required by the VM.
pub struct DefaultHost<A, D = DefaultDebugHandler, T = DefaultTraceHandler> {
    adv_provider: A,
    store: MemMastForestStore,
    event_handlers: EventHandlerRegistry,
    debug_handler: D,
    trace_handler: T,
}

impl<A: AdviceProvider, T: TraceHandler> DefaultHost<A, DefaultDebugHandler, T> {
    /// Replace the [`DefaultDebugHandler`] with a custom one, ensuring it cannot be overridden.
    pub fn with_debug_handler<D: DebugHandler>(self, handler: D) -> DefaultHost<A, D, T> {
        DefaultHost {
            adv_provider: self.adv_provider,
            store: self.store,
            event_handlers: self.event_handlers,
            debug_handler: handler,
            trace_handler: self.trace_handler,
        }
    }
}

impl<A: AdviceProvider, D: DebugHandler> DefaultHost<A, D, DefaultTraceHandler> {
    /// Replace the [`DefaultTraceHandler`] with a custom one, ensuring it cannot be overridden.
    pub fn with_trace_handler<T: TraceHandler>(self, handler: T) -> DefaultHost<A, D, T> {
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

impl<A: AdviceProvider, D: DebugHandler, T: TraceHandler> DefaultHost<A, D, T> {
    pub fn load_library(&mut self, library: &dyn HostLibrary) -> Result<(), ExecutionError> {
        // Load the MAST forest
        self.load_mast_forest(library.mast_forest())?;

        // Register event handlers
        self.event_handlers.register_many(library.event_handlers())?;

        Ok(())
    }

    pub fn load_handler(&mut self, handler: Box<dyn EventHandler>) -> Result<(), ExecutionError> {
        self.event_handlers.register(handler)
    }

    fn load_mast_forest(&mut self, mast_forest: Arc<MastForest>) -> Result<(), ExecutionError> {
        // Load the MAST's advice data into the advice provider.
        for (digest, values) in mast_forest.advice_map().iter() {
            if let Some(stored_values) = self.advice_provider().get_mapped_values(digest) {
                if stored_values != values {
                    return Err(ExecutionError::AdviceMapKeyAlreadyPresent {
                        key: *digest,
                        prev_values: stored_values.to_vec(),
                        new_values: values.clone(),
                    });
                }
            } else {
                self.advice_provider_mut().insert_into_map(*digest, values.clone());
            }
        }

        self.store.insert(mast_forest);
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

impl<A: AdviceProvider, D: DebugHandler, T: TraceHandler> Host for DefaultHost<A, D, T> {
    type AdviceProvider = A;

    fn advice_provider(&self) -> &Self::AdviceProvider {
        &self.adv_provider
    }

    fn advice_provider_mut(&mut self) -> &mut Self::AdviceProvider {
        &mut self.adv_provider
    }

    fn get_mast_forest(&self, node_digest: &Word) -> Option<Arc<MastForest>> {
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

// REGISTRY
// ================================================================================================

/// Registry for maintaining event handlers.
#[derive(Default)]
pub struct EventHandlerRegistry {
    handlers: BTreeMap<u32, Box<dyn EventHandler>>,
}

impl EventHandlerRegistry {
    pub fn new() -> Self {
        Self { handlers: BTreeMap::new() }
    }

    pub fn register(&mut self, handler: Box<dyn EventHandler>) -> Result<(), ExecutionError> {
        let id = handler.id();
        if self.handlers.contains_key(&id) {
            return Err(ExecutionError::DuplicateEventHandler { id });
        }
        self.handlers.insert(id, handler);
        Ok(())
    }

    pub fn register_many(
        &mut self,
        handlers: Vec<Box<dyn EventHandler>>,
    ) -> Result<(), ExecutionError> {
        for handler in handlers {
            self.register(handler)?;
        }
        Ok(())
    }

    pub fn get(&mut self, id: u32) -> Option<&mut Box<dyn EventHandler>> {
        self.handlers.get_mut(&id)
    }
}
