use alloc::collections::BTreeMap;
use core::error::Error;

use crate::{Box, ExecutionError, ProcessState, Vec};

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

// REGISTRY
// ================================================================================================

/// Registry for maintaining event handlers
#[derive(Default)]
pub struct EventHandlerRegistry<A> {
    handlers: BTreeMap<u32, Box<dyn EventHandler<A>>>,
}

impl<A> EventHandlerRegistry<A> {
    pub fn new() -> Self {
        Self { handlers: BTreeMap::new() }
    }

    pub fn register(&mut self, handler: Box<dyn EventHandler<A>>) -> Result<(), ExecutionError> {
        let id = handler.id();
        if self.handlers.contains_key(&id) {
            return Err(ExecutionError::DuplicateEventHandler { id });
        }
        self.handlers.insert(id, handler);
        Ok(())
    }

    pub fn register_many(
        &mut self,
        handlers: Vec<Box<dyn EventHandler<A>>>,
    ) -> Result<(), ExecutionError> {
        for handler in handlers {
            self.register(handler)?;
        }
        Ok(())
    }

    pub fn get(&mut self, id: u32) -> Option<&mut Box<dyn EventHandler<A>>> {
        self.handlers.get_mut(&id)
    }
}

// REGISTRY
// ================================================================================================

type BoxedEventError = Box<dyn Error + Send + Sync + 'static>;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct EventError(BoxedEventError);

impl EventError {
    pub fn from<E: Error + Send + Sync + 'static>(value: E) -> Self {
        Self(Box::new(value))
    }
}
