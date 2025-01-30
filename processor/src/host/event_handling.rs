use alloc::boxed::Box;
use std::collections::BTreeMap;

use crate::{ExecutionError, ProcessState};

pub trait EventHandler<A> {
    fn id(&self) -> u32;

    fn on_event(
        &mut self,
        process: ProcessState,
        advice_provider: &mut A,
    ) -> Result<(), ExecutionError>;
}

#[derive(Default)]
pub struct EventHandlerRegistry<A> {
    handlers: BTreeMap<u32, Box<dyn EventHandler<A>>>,
}

impl<A> EventHandlerRegistry<A> {
    pub fn register_event_handler(&mut self, handler: Box<dyn EventHandler<A>>) {
        self.handlers.insert(handler.id(), handler);
    }

    pub fn register_event_handlers(
        &mut self,
        handlers: impl Iterator<Item = Box<dyn EventHandler<A>>>,
    ) {
        for handler in handlers {
            self.register_event_handler(handler);
        }
    }

    pub fn get_event_handler(&mut self, event_id: u32) -> Option<&mut Box<dyn EventHandler<A>>> {
        self.handlers.get_mut(&event_id)
    }
}
