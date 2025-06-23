use alloc::boxed::Box;
use core::error::Error;

use vm_core::DebugOptions;

use crate::{ExecutionError, ProcessState};

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
/// TODO: Should we merge into a single handler?
pub trait DebugHandler<A> {
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

pub trait TraceHandler<A> {
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

#[derive(Clone)]
pub struct StatelessHandler<F> {
    id: u32,
    handler: F,
}

impl<F> StatelessHandler<F> {
    pub fn new(id: u32, handler: F) -> Self {
        Self { id, handler }
    }
}

pub fn trivial_handler<A: 'static>(id: u32) -> Box<dyn EventHandler<A>> {
    pub fn trivial_event_handler<A>(
        _advice: &mut A,
        _process: ProcessState,
    ) -> Result<(), EventError> {
        Ok(())
    }
    Box::new(StatelessHandler::new(id, trivial_event_handler::<A>))
}

impl<A, F> EventHandler<A> for StatelessHandler<F>
where
    F: Fn(&mut A, ProcessState) -> Result<(), EventError> + 'static,
{
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
#[error(transparent)]
pub struct EventError(#[from] Box<dyn Error + Send + Sync + 'static>);

impl EventError {
    pub fn from<E: Error + Send + Sync + 'static>(value: E) -> Self {
        Self(Box::new(value))
    }
}