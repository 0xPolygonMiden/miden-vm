use alloc::{sync::Arc, vec::Vec};

use vm_core::{DebugOptions, Felt, Word, mast::MastForest};

use crate::{ExecutionError, ProcessState, errors::ErrorContext};

pub(super) mod advice;

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

    /// Returns MAST forest corresponding to the specified digest, or None if the MAST forest for
    /// this digest could not be found in this [Host].
    fn get_mast_forest(&mut self, node_digest: &Word) -> Option<Arc<MastForest>>;

    /// TODO: Docs and find better name.
    fn iter_mast_forests(&self) -> impl Iterator<Item = Arc<MastForest>>;

    // PROVIDED METHODS
    // --------------------------------------------------------------------------------------------

    /// Handles the event emitted from the VM.
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

    /// Handles the debug request from the VM.
    fn on_debug(
        &mut self,
        process: &mut ProcessState,
        options: &DebugOptions,
    ) -> Result<(), ExecutionError>
    where
        Self: Sized,
    {
        let _ = (&process, options);
        #[cfg(feature = "std")]
        debug::print_debug_info(process, options);
        Ok(())
    }

    /// Handles the trace emitted from the VM.
    fn on_trace(
        &mut self,
        process: &mut ProcessState,
        trace_id: u32,
    ) -> Result<(), ExecutionError> {
        let _ = (&process, trace_id);
        #[cfg(feature = "std")]
        std::println!(
            "Trace with id {} emitted at step {} in context {}",
            trace_id,
            process.clk(),
            process.ctx()
        );
        Ok(())
    }

    /// Handles the failure of the assertion instruction.
    fn on_assert_failed(&mut self, _process: &mut ProcessState, _err_code: Felt) {}
}

impl<H> Host for &mut H
where
    H: Host,
{
    fn get_mast_forest(&mut self, node_digest: &Word) -> Option<Arc<MastForest>> {
        H::get_mast_forest(self, node_digest)
    }

    fn iter_mast_forests(&self) -> impl Iterator<Item = Arc<MastForest>> {
        H::iter_mast_forests(self)
    }

    fn on_event(
        &mut self,
        process: &mut ProcessState,
        event_id: u32,
        err_ctx: &impl ErrorContext,
    ) -> Result<(), ExecutionError> {
        H::on_event(self, process, event_id, err_ctx)
    }

    fn on_debug(
        &mut self,
        process: &mut ProcessState,
        options: &DebugOptions,
    ) -> Result<(), ExecutionError> {
        H::on_debug(self, process, options)
    }

    fn on_trace(
        &mut self,
        process: &mut ProcessState,
        trace_id: u32,
    ) -> Result<(), ExecutionError> {
        H::on_trace(self, process, trace_id)
    }

    fn on_assert_failed(&mut self, process: &mut ProcessState, err_code: Felt) {
        H::on_assert_failed(self, process, err_code)
    }
}

// DEFAULT HOST IMPLEMENTATION
// ================================================================================================

/// A default [Host] implementation that provides the essential functionality required by the VM.
#[derive(Debug, Clone, Default)]
pub struct DefaultHost {
    mast_forsts: Vec<Arc<MastForest>>,
    store: MemMastForestStore,
}

impl DefaultHost {
    pub fn load_mast_forest(&mut self, mast_forest: Arc<MastForest>) -> Result<(), ExecutionError> {
        self.mast_forsts.push(mast_forest.clone());

        self.store.insert(mast_forest);
        Ok(())
    }
}

impl Host for DefaultHost {
    fn get_mast_forest(&mut self, node_digest: &Word) -> Option<Arc<MastForest>> {
        self.store.get(node_digest)
    }

    fn iter_mast_forests(&self) -> impl Iterator<Item = Arc<MastForest>> {
        self.mast_forsts.iter().cloned()
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
