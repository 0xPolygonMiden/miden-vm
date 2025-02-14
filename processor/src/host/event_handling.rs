use alloc::{
    boxed::Box,
    collections::{btree_map::Entry, BTreeMap},
};
use core::error::Error;

use crate::{ExecutionError, ProcessState};

/// Defines an interface for handling events emitted by the VM.
pub trait EventHandler {
    /// Returns the ID of the event this handler is responsible for.
    fn id(&self) -> u32;

    /// Mutates the advice provider based on the event emitted by the VM.
    fn on_event(
        &mut self,
        process: ProcessState,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
}

/// A registry of event handlers, indexed by event id.
#[derive(Default)]
pub struct EventHandlerRegistry {
    handlers: BTreeMap<u32, Box<dyn EventHandler>>,
}

impl EventHandlerRegistry {
    /// Register an event handler with the registry.
    pub fn register_event_handler(
        &mut self,
        handler: Box<dyn EventHandler>,
    ) -> Result<(), ExecutionError> {
        match self.handlers.entry(handler.id()) {
            Entry::Occupied(_) => {
                Err(ExecutionError::EventHandlerAlreadyRegistered { event_id: handler.id() })
            },
            Entry::Vacant(entry) => {
                entry.insert(handler);
                Ok(())
            },
        }
    }

    /// Register a set of event handlers with the registry.
    pub fn register_event_handlers(
        &mut self,
        handlers: impl Iterator<Item = Box<dyn EventHandler>> + 'static,
    ) -> Result<(), ExecutionError> {
        for handler in handlers {
            self.register_event_handler(handler)?;
        }

        Ok(())
    }

    /// Returns a mutable reference to the event handler for the specified event ID.
    pub fn get_event_handler(&mut self, event_id: u32) -> Option<&mut Box<dyn EventHandler>> {
        self.handlers.get_mut(&event_id)
    }
}

// NOOP EVENT HANDLER
// ================================================================================================

/// An event handler that does nothing.
pub struct NoopEventHandler {
    id: u32,
}

impl NoopEventHandler
{
    /// Creates an event handler with the specified ID.
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    /// Creates an event handler with the specified ID.
    pub fn new_boxed(id: u32) -> Box<dyn EventHandler> {
        Box::new(Self { id })
    }
}

impl EventHandler for NoopEventHandler {
    fn id(&self) -> u32 {
        self.id
    }

    fn on_event(
        &mut self,
        _process: ProcessState,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        Ok(())
    }
}
