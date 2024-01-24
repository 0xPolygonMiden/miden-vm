use super::{BTreeMap, Felt};
use vm_core::crypto::hash::RpoDigest;

// ADVICE MAP
#[derive(Debug, Clone, Default)]
pub struct AdviceMap(BTreeMap<RpoDigest, Vec<Felt>>);

impl AdviceMap {
    pub fn get_inner_map(&self) -> &BTreeMap<RpoDigest, Vec<Felt>> {
        &self.0
    }

    pub fn get_inner_map_mut(&mut self) -> &mut BTreeMap<RpoDigest, Vec<Felt>> {
        &mut self.0
    }
}
