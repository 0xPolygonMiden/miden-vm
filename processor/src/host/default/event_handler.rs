use alloc::collections::BTreeMap;

use crate::{Box, ExecutionError, Vec, host::default::EventHandler};

/// Registry for maintaining event handlers
#[derive(Default)]
pub(super) struct EventHandlerRegistry<A> {
    handlers: BTreeMap<u32, Box<dyn EventHandler<A>>>,
}

impl<A> EventHandlerRegistry<A> {
    pub(super) fn new() -> Self {
        Self { handlers: BTreeMap::new() }
    }

    pub(super) fn register(
        &mut self,
        handler: Box<dyn EventHandler<A>>,
    ) -> Result<(), ExecutionError> {
        let id = handler.id();
        if self.handlers.contains_key(&id) {
            return Err(ExecutionError::DuplicateEventHandler { id });
        }
        self.handlers.insert(id, handler);
        Ok(())
    }

    pub(super) fn register_many(
        &mut self,
        handlers: Vec<Box<dyn EventHandler<A>>>,
    ) -> Result<(), ExecutionError> {
        for handler in handlers {
            self.register(handler)?;
        }
        Ok(())
    }

    pub(super) fn get(&mut self, id: u32) -> Option<&mut Box<dyn EventHandler<A>>> {
        self.handlers.get_mut(&id)
    }
}
