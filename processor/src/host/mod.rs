use alloc::sync::Arc;

use vm_core::{
    DebugOptions,
    crypto::hash::RpoDigest,
    mast::{MastForest, MastNodeExt},
};

use super::{ExecutionError, ProcessState};
use crate::{Felt, errors::ErrorContext};

pub(super) mod advice;
use advice::AdviceProvider;

mod mast_forest_store;
pub use mast_forest_store::{MastForestStore, MemMastForestStore};

pub(super) mod default;

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
    fn on_event(
        &mut self,
        process: ProcessState,
        event_id: u32,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Result<(), ExecutionError>;

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
    fn on_assert_failed(&mut self, _process: ProcessState, _err_code: Felt) {}
}

// DEBUG HANDLER
// ================================================================================================

/// Handler for debug and trace operations
pub trait DebugHandler<A: AdviceProvider> {
    fn on_debug(
        &mut self,
        advice: &A,
        process: ProcessState,
        options: &DebugOptions,
    ) -> Result<(), ExecutionError>;
}

// TRACE HANDLER
// ================================================================================================

pub trait TraceHandler<A: AdviceProvider> {
    fn on_trace(
        &mut self,
        advice: &A,
        process: ProcessState,
        trace_id: u32,
    ) -> Result<(), ExecutionError>;
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
