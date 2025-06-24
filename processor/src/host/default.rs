use alloc::{
    boxed::Box,
    collections::{BTreeMap, btree_map::Entry},
    sync::Arc,
};

use vm_core::{
    DebugOptions, Word,
    mast::{MastForest, MastNodeExt},
    utils::collections::KvMap,
};

use crate::{
    AdviceProvider, ErrorContext, ExecutionError, Host, HostLibrary, MastForestStore,
    MemAdviceProvider, MemMastForestStore, ProcessState,
    handlers::{DebugHandler, DefaultDebugHandler, EventHandler},
};

// DEFAULT HOST IMPLEMENTATION
// ================================================================================================

/// A default [`Host`] implementation that provides the essential functionality required by the VM.
pub struct DefaultHost<A, D = DefaultDebugHandler> {
    adv_provider: A,
    store: MemMastForestStore,
    event_handlers: EventHandlerRegistry,
    debug_handler: D,
}

impl<A: AdviceProvider> DefaultHost<A> {
    pub fn new(adv_provider: A) -> Self {
        Self {
            adv_provider,
            store: MemMastForestStore::default(),
            event_handlers: EventHandlerRegistry::new(),
            debug_handler: DefaultDebugHandler,
        }
    }
}

impl<A: AdviceProvider, D: DebugHandler> DefaultHost<A, D> {
    /// Loads a [`HostLibrary`]'s [`MastForest`] and all of its associated [`EventHandler`]s.
    pub fn load_library(&mut self, library: &dyn HostLibrary) -> Result<(), ExecutionError> {
        // Load the MAST's advice data into the advice provider.
        let mast_forest = library.mast_forest();
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

        // Register event handlers
        for handler in library.event_handlers() {
            self.event_handlers.register(handler)?;
        }

        Ok(())
    }

    /// Loads a single [`EventHandler`] into this [`Host`].
    ///
    /// It is particularly useful for adding stateless handlers, which are obtained with the
    /// [`new_handler(id, handler_func)`](crate::host::handlers::new_handler) constructor.
    pub fn load_handler(&mut self, handler: Box<dyn EventHandler>) -> Result<(), ExecutionError> {
        self.event_handlers.register(handler)
    }

    /// Replace the [`DefaultDebugHandler`] with a custom one.
    pub fn with_debug_handler<H: DebugHandler>(self, handler: H) -> DefaultHost<A, H> {
        DefaultHost {
            adv_provider: self.adv_provider,
            store: self.store,
            event_handlers: self.event_handlers,
            debug_handler: handler,
        }
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn debug_handler(&self) -> &D {
        &self.debug_handler
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn debug_handler_mut(&mut self) -> &mut D {
        &mut self.debug_handler
    }
}

impl<A: AdviceProvider, D: DebugHandler> Host for DefaultHost<A, D> {
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
        let handler = self
            .event_handlers
            .get(event_id)
            .ok_or_else(|| ExecutionError::invalid_event_id_error(event_id, err_ctx))?;

        handler
            .on_event(&mut self.adv_provider, process)
            .map_err(|err| ExecutionError::event_error(event_id, err, err_ctx))
    }

    fn on_debug(
        &mut self,
        process: ProcessState,
        options: &DebugOptions,
    ) -> Result<(), ExecutionError> {
        self.debug_handler.on_debug(&self.adv_provider, process, options)
    }

    fn on_trace(&mut self, process: ProcessState, trace_id: u32) -> Result<(), ExecutionError> {
        self.debug_handler.on_trace(&self.adv_provider, process, trace_id)
    }
}

impl Default for DefaultHost<MemAdviceProvider> {
    fn default() -> Self {
        Self {
            adv_provider: MemAdviceProvider::default(),
            store: MemMastForestStore::default(),
            event_handlers: EventHandlerRegistry::default(),
            debug_handler: DefaultDebugHandler,
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
        match self.handlers.entry(id) {
            Entry::Vacant(e) => e.insert(handler),
            Entry::Occupied(_) => return Err(ExecutionError::DuplicateEventHandler { id }),
        };
        Ok(())
    }

    pub fn get(&mut self, id: u32) -> Option<&mut Box<dyn EventHandler>> {
        self.handlers.get_mut(&id)
    }
}
