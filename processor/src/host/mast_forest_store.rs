use alloc::{collections::BTreeMap, sync::Arc, vec::Vec};

use vm_core::{Word, mast::MastForest};

/// A set of [`MastForest`]s available to the prover that programs may refer to (by means of an
/// [`vm_core::mast::ExternalNode`]).
///
/// For example, a program's kernel and standard library would most likely not be compiled directly
/// with the program, and instead be provided separately to the prover. This has the benefit of
/// reducing program binary size. The store could also be much more complex, such as accessing a
/// centralized registry of [`MastForest`]s when it doesn't find one locally.
pub trait MastForestStore {
    /// Returns a [`MastForest`] which is guaranteed to contain a procedure with the provided
    /// procedure hash as one of its procedure, if any.
    fn get(&self, procedure_hash: &Word) -> Option<Arc<MastForest>>;
}

/// A simple [`MastForestStore`] where all known [`MastForest`]s are held in memory.
#[derive(Debug, Default, Clone)]
pub struct MemMastForestStore {
    mast_forests: BTreeMap<Word, Arc<MastForest>>,
    unique_forests: Vec<Arc<MastForest>>,
}

impl MemMastForestStore {
    /// Inserts all the procedures of the provided MAST forest in the store.
    pub fn insert(&mut self, mast_forest: Arc<MastForest>) {
        // do not insert a forest if it has already been added.
        if self.unique_forests.contains(&mast_forest) {
            return;
        }
        // only register the procedures which are local to this forest
        for proc_digest in mast_forest.local_procedure_digests() {
            self.mast_forests.insert(proc_digest, mast_forest.clone());
        }
        // store the forest
        self.unique_forests.push(mast_forest);
    }

    /// Returns a list of all unique [`MastForest`]s inserted into the store.
    pub fn mast_forests(&self) -> &[Arc<MastForest>] {
        &self.unique_forests
    }
}

impl MastForestStore for MemMastForestStore {
    fn get(&self, procedure_hash: &Word) -> Option<Arc<MastForest>> {
        self.mast_forests.get(procedure_hash).cloned()
    }
}
