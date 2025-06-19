use alloc::boxed::Box;
use core::error::Error;

use vm_core::DebugOptions;

use crate::{AdviceProvider, ExecutionError, ProcessState};

// HANDLER TRAIT
// ================================================================================================

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
    ) -> Result<(), EventError>;
}

// DEBUG HANDLER
// ================================================================================================

/// Handler for debug and trace operations
pub trait DebugHandler<A: AdviceProvider> {
    /// TODO: What kind of error should we return
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
    /// TODO: What kind of error should we return
    fn on_trace(
        &mut self,
        advice: &A,
        process: ProcessState,
        trace_id: u32,
    ) -> Result<(), ExecutionError>;
}

// STATELESS HANDLER
// ================================================================================================

pub type StatelessHandlerFunc<A> = fn(&mut A, ProcessState) -> Result<(), EventError>;

pub struct StatelessEventHandler<A> {
    id: u32,
    handler: StatelessHandlerFunc<A>,
}

impl<A> StatelessEventHandler<A> {
    pub const fn new(id: u32, handler: StatelessHandlerFunc<A>) -> Self {
        Self { id, handler }
    }
}

impl<A> EventHandler<A> for StatelessEventHandler<A> {
    fn id(&self) -> u32 {
        self.id
    }

    fn on_event(
        &mut self,
        advice_provider: &mut A,
        process: ProcessState,
    ) -> Result<(), EventError> {
        (self.handler)(advice_provider, process)
    }
}

// EVENT ERROR
// ================================================================================================

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct EventError(Box<dyn Error + Send + Sync + 'static>);

impl EventError {
    pub fn from<E: Error + Send + Sync + 'static>(value: E) -> Self {
        Self(Box::new(value))
    }
}
