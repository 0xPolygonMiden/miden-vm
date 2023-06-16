use super::{
    AdviceInputs, BTreeMap, Felt, KvMap, MerkleMap, MerkleStore, StackMapStoreProvider, Vec,
};

#[cfg(test)]
use super::{NodeIndex, Word};

// MEMORY ADVICE PROVIDER
// ================================================================================================

/// An in-memory `[AdviceProvider]` implementation to support program execution.
///
/// Uses `[BTreeMap]` as backend.
#[cfg(not(any(test, feature = "internals")))]
#[derive(Debug, Clone, Default)]
pub struct MemAdviceProvider {
    step: u32,
    stack: Vec<Felt>,
    map: BTreeMap<[u8; 32], Vec<Felt>>,
    store: MerkleStore,
}

impl From<AdviceInputs> for MemAdviceProvider {
    fn from(inputs: AdviceInputs) -> Self {
        let (mut stack, map, store) = inputs.into_parts();
        stack.reverse();
        Self {
            step: 0,
            stack,
            map,
            store,
        }
    }
}

impl StackMapStoreProvider for MemAdviceProvider {
    type Map = MerkleMap;

    fn get_step(&self) -> u32 {
        self.step
    }

    fn get_step_mut(&mut self) -> &mut u32 {
        &mut self.step
    }

    fn get_stack(&self) -> &[Felt] {
        &self.stack
    }

    fn get_stack_mut(&mut self) -> &mut Vec<Felt> {
        &mut self.stack
    }

    fn get_map(&self) -> &dyn KvMap<[u8; 32], Vec<Felt>> {
        &self.map
    }

    // ADVISE SETS
    // --------------------------------------------------------------------------------------------

    fn get_tree_node(
        &self,
        root: Word,
        depth: &Felt,
        index: &Felt,
    ) -> Result<Word, ExecutionError> {
        let index = NodeIndex::from_elements(depth, index).map_err(|_| {
            ExecutionError::InvalidTreeNodeIndex {
                depth: *depth,
                value: *index,
            }
        })?;
        self.store
            .get_node(root.into(), index)
            .map(|v| v.into())
            .map_err(ExecutionError::MerkleStoreLookupFailed)
    }

    fn get_merkle_path(
        &self,
        root: Word,
        depth: &Felt,
        index: &Felt,
    ) -> Result<MerklePath, ExecutionError> {
        let index = NodeIndex::from_elements(depth, index).map_err(|_| {
            ExecutionError::InvalidTreeNodeIndex {
                depth: *depth,
                value: *index,
            }
        })?;
        self.store
            .get_path(root.into(), index)
            .map(|value| value.path)
            .map_err(ExecutionError::MerkleStoreLookupFailed)
    }

    fn get_leaf_depth(
        &self,
        root: Word,
        tree_depth: &Felt,
        index: &Felt,
    ) -> Result<u8, ExecutionError> {
        let tree_depth = u8::try_from(tree_depth.as_int())
            .map_err(|_| ExecutionError::InvalidTreeDepth { depth: *tree_depth })?;
        self.store
            .get_leaf_depth(root.into(), tree_depth, index.as_int())
            .map_err(ExecutionError::MerkleStoreLookupFailed)
    }

    fn update_merkle_node(
        &mut self,
        root: Word,
        depth: &Felt,
        index: &Felt,
        value: Word,
    ) -> Result<MerklePath, ExecutionError> {
        let node_index = NodeIndex::from_elements(depth, index).map_err(|_| {
            ExecutionError::InvalidTreeNodeIndex {
                depth: *depth,
                value: *index,
            }
        })?;
        self.store
            .set_node(root.into(), node_index, value.into())
            .map(|root| root.path)
            .map_err(ExecutionError::MerkleStoreUpdateFailed)
    }

    fn merge_roots(&mut self, lhs: Word, rhs: Word) -> Result<Word, ExecutionError> {
        self.store
            .merge_roots(lhs.into(), rhs.into())
            .map(|v| v.into())
            .map_err(ExecutionError::MerkleStoreMergeFailed)
    }

    // CONTEXT MANAGEMENT
    // --------------------------------------------------------------------------------------------

    fn advance_clock(&mut self) {
        self.step += 1;
    }
    
    fn get_map_mut(&mut self) -> &mut dyn KvMap<[u8; 32], Vec<Felt>> {
        &mut self.map
    }

    fn get_store(&self) -> &MerkleStore {
        &self.store
    }

    fn get_store_mut(&mut self) -> &mut MerkleStore {
        &mut self.store
    }
}

impl MemAdviceProvider {
    // ADVISE SETS TEST HELPERS
    // --------------------------------------------------------------------------------------------

    /// Returns true if the Merkle root exists for the advice provider Merkle store.
    #[cfg(test)]
    pub fn has_merkle_root(&self, root: crate::crypto::RpoDigest) -> bool {
        self.store.get_node(root, NodeIndex::root()).is_ok()
    }
}

// INTERNALS
// ================================================================================================

#[cfg(any(test, feature = "internals"))]
#[derive(Debug, Clone, Default)]
pub struct MemAdviceProvider {
    pub step: u32,
    pub stack: Vec<Felt>,
    pub map: BTreeMap<[u8; 32], Vec<Felt>>,
    pub store: MerkleStore,
}
