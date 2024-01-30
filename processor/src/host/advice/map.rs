use super::{BTreeMap, Felt, Vec};

extern crate alloc;
use alloc::collections::btree_map::IntoIter;

use vm_core::crypto::hash::RpoDigest;

// ADVICE MAP
#[derive(Debug, Clone, Default)]
pub struct AdviceMap(BTreeMap<RpoDigest, Vec<Felt>>);

impl AdviceMap {
    pub fn new() -> Self {
        Self(BTreeMap::<RpoDigest, Vec<Felt>>::new())
    }

    pub fn get(&self, key: &RpoDigest) -> Option<&Vec<Felt>> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: RpoDigest, value: Vec<Felt>) -> Option<Vec<Felt>> {
        self.0.insert(key, value)
    }

    pub fn remove(&mut self, key: RpoDigest) -> Option<Vec<Felt>> {
        self.0.remove(&key)
    }
}

impl IntoIterator for AdviceMap {
    type Item = (RpoDigest, Vec<Felt>);
    type IntoIter = IntoIter<RpoDigest, Vec<Felt>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Extend<(RpoDigest, Vec<Felt>)> for AdviceMap {
    fn extend<T: IntoIterator<Item = (RpoDigest, Vec<Felt>)>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}
