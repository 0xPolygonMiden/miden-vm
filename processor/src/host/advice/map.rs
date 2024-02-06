use super::{BTreeMap, Felt, Vec};
use vm_core::utils::collections::btree_map::IntoIter;

use vm_core::crypto::hash::RpoDigest;

//////////////////////////////////
/// ADVICE MAP
//////////////////////////////////
/// This is one of the advice input types
/// it holds private advice inputs that can be used by the prover
/// it is a key mapped element list which can be pushed onto the advice stack
#[derive(Debug, Clone, Default)]
pub struct AdviceMap(BTreeMap<RpoDigest, Vec<Felt>>);

impl AdviceMap {
    /// Creates a new advice map
    pub fn new() -> Self {
        Self(BTreeMap::<RpoDigest, Vec<Felt>>::new())
    }

    /// Gets the value associated with the key in the advice map
    /// returns an option type
    pub fn get(&self, key: &RpoDigest) -> Option<&[Felt]> {
        self.0.get(key).map(|v| v.as_slice())
    }

    /// Inserts a key value pair in the advice map
    /// returns the value inserted
    pub fn insert(&mut self, key: RpoDigest, value: Vec<Felt>) -> Option<Vec<Felt>> {
        self.0.insert(key, value)
    }

    /// Removes the value associated with the key
    /// returns the value that was removed
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
