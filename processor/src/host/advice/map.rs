use alloc::{
    collections::{btree_map::IntoIter, BTreeMap},
    vec::Vec,
};

use miden_air::DeserializationError;
use vm_core::crypto::hash::RpoDigest;
use winter_utils::{ByteReader, ByteWriter, Deserializable, Serializable};

use super::Felt;

// ADVICE MAP
// ================================================================================================

/// Defines a set of non-deterministic (advice) inputs which the VM can access by their keys.
///
/// Each key maps to one or more field element. To access the elements, the VM can move the values
/// associated with a given key onto the advice stack using `adv.push_mapval` instruction. The VM
/// can also insert new values into the advice map during execution.
#[derive(Debug, Clone, Default)]
pub struct AdviceMap(BTreeMap<RpoDigest, Vec<Felt>>);

impl AdviceMap {
    /// Creates a new advice map.
    pub fn new() -> Self {
        Self(BTreeMap::<RpoDigest, Vec<Felt>>::new())
    }

    /// Returns the values associated with given key.
    pub fn get(&self, key: &RpoDigest) -> Option<&[Felt]> {
        self.0.get(key).map(|v| v.as_slice())
    }

    /// Inserts a key value pair in the advice map and returns the inserted value.
    pub fn insert(&mut self, key: RpoDigest, value: Vec<Felt>) -> Option<Vec<Felt>> {
        self.0.insert(key, value)
    }

    /// Removes the value associated with the key and returns the removed element.
    pub fn remove(&mut self, key: RpoDigest) -> Option<Vec<Felt>> {
        self.0.remove(&key)
    }
}

impl From<BTreeMap<RpoDigest, Vec<Felt>>> for AdviceMap {
    fn from(value: BTreeMap<RpoDigest, Vec<Felt>>) -> Self {
        Self(value)
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

impl Serializable for AdviceMap {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u32(self.0.len() as u32);
        for (key, values) in self.0.iter() {
            key.write_into(target);
            values.write_into(target);
        }
    }
}

impl Deserializable for AdviceMap {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let mut map = BTreeMap::new();
        let count = source.read_u32()?;
        for _ in 0..count {
            let key = RpoDigest::read_from(source)?;
            let values = Vec::<Felt>::read_from(source)?;
            map.insert(key, values);
        }
        Ok(Self(map))
    }
}
