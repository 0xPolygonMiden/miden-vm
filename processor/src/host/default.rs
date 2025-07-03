use alloc::{boxed::Box, sync::Arc};

use vm_core::{DebugOptions, Word, mast::MastForest};

use crate::{
    DebugHandler, ErrorContext, EventHandler, EventHandlerRegistry, ExecutionError, Host,
    HostLibrary, MastForestStore, MemMastForestStore, ProcessState,
};

// DEFAULT HOST IMPLEMENTATION
// ================================================================================================

/// A default [Host] implementation that provides the essential functionality required by the VM.
#[derive(Debug)]
pub struct DefaultHost<D: DebugHandler = DefaultDebugHandler> {
    store: MemMastForestStore,
    event_handlers: EventHandlerRegistry,
    debug_handler: D,
}

impl Default for DefaultHost {
    fn default() -> Self {
        Self {
            store: MemMastForestStore::default(),
            event_handlers: EventHandlerRegistry::default(),
            debug_handler: DefaultDebugHandler,
        }
    }
}

impl<D: DebugHandler> DefaultHost<D> {
    pub fn load_mast_forest(&mut self, mast_forest: Arc<MastForest>) -> Result<(), ExecutionError> {
        self.store.insert(mast_forest);
        Ok(())
    }

    pub fn load_library(&mut self, library: &impl HostLibrary) -> Result<(), ExecutionError> {
        self.load_mast_forest(library.mast_forest())?;
        for (id, handler) in library.event_handlers() {
            self.event_handlers.register(id, handler)?;
        }
        Ok(())
    }

    /// Loads a single [`EventHandler`] into this [`Host`]. The handler can be either a closure or a
    /// free function accepting a `&mut ProcessState` and returning an `EventError`.
    pub fn load_handler(
        &mut self,
        id: u32,
        handler: impl EventHandler + 'static,
    ) -> Result<(), ExecutionError> {
        self.event_handlers.register(id, Box::new(handler))
    }

    /// Replace the current [`DebugHandler`] with a custom one.
    pub fn with_debug_handler<H: DebugHandler>(self, handler: H) -> DefaultHost<H> {
        DefaultHost {
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

impl<D: DebugHandler> Host for DefaultHost<D> {
    fn get_mast_forest(&self, node_digest: &Word) -> Option<Arc<MastForest>> {
        self.store.get(node_digest)
    }

    fn on_event(
        &mut self,
        process: &mut ProcessState,
        event_id: u32,
        err_ctx: &impl ErrorContext,
    ) -> Result<(), ExecutionError> {
        if self.event_handlers.handle_event(event_id, process, err_ctx)? {
            // the event was handled by the registered event handlers; just return
            return Ok(());
        }

        Err(ExecutionError::invalid_event_id_error(event_id, err_ctx))
    }

    fn on_debug(
        &mut self,
        process: &mut ProcessState,
        options: &DebugOptions,
    ) -> Result<(), ExecutionError> {
        self.debug_handler.on_debug(process, options)
    }

    fn on_trace(
        &mut self,
        process: &mut ProcessState,
        trace_id: u32,
    ) -> Result<(), ExecutionError> {
        self.debug_handler.on_trace(process, trace_id)
    }
}

// DEFAULT DEBUG HANDLER IMPLEMENTATION
// ================================================================================================

/// Concrete [`DebugHandler`] which re-uses the default `on_debug` and `on_trace` implementations.
#[derive(Clone, Default)]
pub struct DefaultDebugHandler;

impl DebugHandler for DefaultDebugHandler {}
