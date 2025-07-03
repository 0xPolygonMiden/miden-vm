use alloc::{boxed::Box, sync::Arc, vec::Vec};

use vm_core::mast::MastForest;

use crate::EventHandler;

pub trait HostLibrary {
    fn event_handlers(&self) -> Vec<(u32, Box<dyn EventHandler>)> {
        Vec::default()
    }

    fn mast_forest(&self) -> Arc<MastForest>;
}

impl HostLibrary for Arc<MastForest> {
    fn mast_forest(&self) -> Arc<MastForest> {
        self.clone()
    }
}
