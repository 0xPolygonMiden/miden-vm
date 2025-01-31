use alloc::{boxed::Box, collections::btree_map::Entry};
use core::error::Error;
use std::collections::BTreeMap;

use crate::{ExecutionError, ProcessState};

pub trait EventHandler<A> {
    fn id(&self) -> u32;

    // TODO(plafer): `ProcessState` is a processor type, which can't be moved to core as-is. But we
    // want `EventHandler` to be in core. How to fix this? The solution is probably to provide a
    // `ProcessState` trait in core.
    fn on_event(
        &mut self,
        process: ProcessState,
        advice_provider: &mut A,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
}

#[derive(Default)]
pub struct EventHandlerRegistry<A> {
    handlers: BTreeMap<u32, Box<dyn EventHandler<A>>>,
}

impl<A> EventHandlerRegistry<A> {
    pub fn register_event_handler(
        &mut self,
        handler: Box<dyn EventHandler<A>>,
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

    pub fn register_event_handlers(
        &mut self,
        handlers: impl Iterator<Item = Box<dyn EventHandler<A>>> + 'static,
    ) -> Result<(), ExecutionError> {
        for handler in handlers {
            self.register_event_handler(handler)?;
        }

        Ok(())
    }

    pub fn get_event_handler(&mut self, event_id: u32) -> Option<&mut Box<dyn EventHandler<A>>> {
        self.handlers.get_mut(&event_id)
    }
}
