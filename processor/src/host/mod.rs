use alloc::sync::Arc;

use vm_core::{DebugOptions, Felt, Word, mast::MastForest};

use crate::{DebugHandler, ExecutionError, ProcessState, errors::ErrorContext};

pub(super) mod advice;

#[cfg(feature = "std")]
mod debug;

pub mod default;
use default::DefaultDebugHandler;

pub mod handlers;

mod mast_forest_store;
pub use mast_forest_store::{MastForestStore, MemMastForestStore};

// HOST TRAIT
// ================================================================================================

/// Defines an interface by which the VM can interact with the host.
///
/// There are three main categories of interactions between the VM and the host:
/// 1. getting a library's MAST forest,
/// 2. handling advice events (which mutate the process' advice provider), and
/// 3. handling debug and trace events.
pub trait Host {
    // REQUIRED METHODS
    // --------------------------------------------------------------------------------------------

    /// Returns MAST forest corresponding to the specified digest, or None if the MAST forest for
    /// this digest could not be found in this [Host].
    fn get_mast_forest(&self, node_digest: &Word) -> Option<Arc<MastForest>>;

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
    ) -> Result<(), ExecutionError> {
        DefaultDebugHandler.on_debug(process, options)
    }

    /// Handles the trace emitted from the VM.
    fn on_trace(
        &mut self,
        process: &mut ProcessState,
        trace_id: u32,
    ) -> Result<(), ExecutionError> {
        DefaultDebugHandler.on_trace(process, trace_id)
    }

    /// Handles the failure of the assertion instruction.
    fn on_assert_failed(&mut self, _process: &mut ProcessState, _err_code: Felt) {}
}

impl<H> Host for &mut H
where
    H: Host,
{
    fn get_mast_forest(&self, node_digest: &Word) -> Option<Arc<MastForest>> {
        H::get_mast_forest(self, node_digest)
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
