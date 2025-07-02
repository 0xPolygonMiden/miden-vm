use alloc::{sync::Arc, vec::Vec};

use vm_core::{DebugOptions, mast::MastForest};

use crate::{
    DebugHandler, ExecutionError, Host, MastForestStore, MemMastForestStore, ProcessState, Word,
};

// DEFAULT HOST IMPLEMENTATION
// ================================================================================================

/// A default [Host] implementation that provides the essential functionality required by the VM.
#[derive(Debug, Clone)]
pub struct DefaultHost<D: DebugHandler = DefaultDebugHandler> {
    mast_forests: Vec<Arc<MastForest>>,
    store: MemMastForestStore,
    debug_handler: D,
}

impl Default for DefaultHost {
    fn default() -> Self {
        Self {
            mast_forests: Vec::default(),
            store: MemMastForestStore::default(),
            debug_handler: DefaultDebugHandler,
        }
    }
}

impl<D: DebugHandler> DefaultHost<D> {
    pub fn load_mast_forest(&mut self, mast_forest: Arc<MastForest>) -> Result<(), ExecutionError> {
        self.mast_forests.push(mast_forest.clone());

        self.store.insert(mast_forest);
        Ok(())
    }

    /// Replace the current [`DebugHandler`] with a custom one.
    pub fn with_debug_handler<H: DebugHandler>(self, handler: H) -> DefaultHost<H> {
        DefaultHost {
            mast_forests: self.mast_forests,
            store: self.store,
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
    fn get_mast_forest(&mut self, node_digest: &Word) -> Option<Arc<MastForest>> {
        self.store.get(node_digest)
    }

    fn iter_mast_forests(&self) -> impl Iterator<Item = Arc<MastForest>> {
        self.mast_forests.iter().cloned()
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
