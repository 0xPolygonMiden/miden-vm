use super::{BTreeMap, Felt, Vec};

extern crate alloc;
use alloc::collections::btree_map::IterMut;

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

    pub fn get_mut(&mut self, key: &RpoDigest) -> Option<&mut Vec<Felt>> {
        self.0.get_mut(key)
    }

    pub fn insert(&mut self, key: RpoDigest, value: Vec<Felt>) -> Option<Vec<Felt>> {
        self.0.insert(key, value)
    }

    pub fn remove(&mut self, key: RpoDigest) -> Option<Vec<Felt>> {
        self.0.remove(&key)
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, RpoDigest, Vec<Felt>> {
        self.0.iter_mut()
    }

    pub fn get_map(&self) -> &BTreeMap<RpoDigest, Vec<Felt>> {
        &self.0
    }

    pub fn get_map_mut(&mut self) -> &mut BTreeMap<RpoDigest, Vec<Felt>> {
        &mut self.0
    }
}
