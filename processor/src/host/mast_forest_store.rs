use alloc::{collections::BTreeMap, sync::Arc};
use vm_core::{crypto::hash::RpoDigest, mast::MastForest};

pub trait MastForestStore {
    fn get(&self, node_digest: &RpoDigest) -> Option<Arc<MastForest>>;
}

#[derive(Debug, Default, Clone)]
pub struct MemMastForestStore {
    mast_forests: BTreeMap<RpoDigest, Arc<MastForest>>,
}

impl MemMastForestStore {
    pub fn insert(&mut self, mast_forest: MastForest) {
        let mast_forest = Arc::new(mast_forest);

        for root in mast_forest.roots() {
            self.mast_forests.insert(root, mast_forest.clone());
        }
    }
}

impl MastForestStore for MemMastForestStore {
    fn get(&self, node_digest: &RpoDigest) -> Option<Arc<MastForest>> {
        self.mast_forests.get(node_digest).cloned()
    }
}
