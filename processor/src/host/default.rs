// DEFAULT HOST IMPLEMENTATION
// ================================================================================================

use alloc::{sync::Arc, vec::Vec};

use vm_core::mast::MastForest;

use crate::{
    ErrorContext, ExecutionError, Host, MastForestStore, MemMastForestStore, ProcessState, Word,
};

/// A default [Host] implementation that provides the essential functionality required by the VM.
#[derive(Debug, Clone, Default)]
pub struct DefaultHost {
    mast_forests: Vec<Arc<MastForest>>,
    store: MemMastForestStore,
}

impl DefaultHost {
    pub fn load_mast_forest(&mut self, mast_forest: Arc<MastForest>) -> Result<(), ExecutionError> {
        self.mast_forests.push(mast_forest.clone());

        self.store.insert(mast_forest);
        Ok(())
    }
}

impl Host for DefaultHost {
    fn get_mast_forest(&mut self, node_digest: &Word) -> Option<Arc<MastForest>> {
        self.store.get(node_digest)
    }

    fn iter_mast_forests(&self) -> impl Iterator<Item = Arc<MastForest>> {
        self.mast_forests.iter().cloned()
    }

    fn on_event(
        &mut self,
        process: &mut ProcessState,
        event_id: u32,
        err_ctx: &impl ErrorContext,
    ) -> Result<(), ExecutionError> {
        let _ = (&process, event_id, err_ctx);
        #[cfg(feature = "std")]
        std::println!(
            "Event with id {} emitted at step {} in context {}",
            event_id,
            process.clk(),
            process.ctx()
        );
        Ok(())
    }
}
