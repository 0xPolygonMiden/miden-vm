use core::ops::Index;

use alloc::collections::BTreeMap;
use vm_core::{
    crypto::hash::RpoDigest,
    mast::{MastForest, MastNode, MastNodeId, MerkleTreeNode},
};

/// Builder for a [`MastForest`].
#[derive(Clone, Debug, Default)]
pub struct MastForestBuilder {
    mast_forest: MastForest,
    node_id_by_hash: BTreeMap<RpoDigest, MastNodeId>,
}

impl MastForestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> MastForest {
        self.mast_forest
    }
}

/// Accessors
impl MastForestBuilder {
    /// Returns the underlying [`MastForest`] being built
    pub fn forest(&self) -> &MastForest {
        &self.mast_forest
    }

    /// Returns the [`MastNodeId`] of the procedure associated with a given digest, if any.
    #[inline(always)]
    pub fn find_procedure_root(&self, digest: RpoDigest) -> Option<MastNodeId> {
        self.mast_forest.find_procedure_root(digest)
    }
}

/// Mutators
impl MastForestBuilder {
    /// Adds a node to the forest, and returns the [`MastNodeId`] associated with it.
    ///
    /// If a [`MastNode`] which is equal to the current node was previously added, the previously
    /// returned [`MastNodeId`] will be returned. This enforces this invariant that equal
    /// [`MastNode`]s have equal [`MastNodeId`]s.
    pub fn ensure_node(&mut self, node: MastNode) -> MastNodeId {
        let node_digest = node.digest();

        if let Some(node_id) = self.node_id_by_hash.get(&node_digest) {
            // node already exists in the forest; return previously assigned id
            *node_id
        } else {
            let new_node_id = self.mast_forest.add_node(node);
            self.node_id_by_hash.insert(node_digest, new_node_id);

            new_node_id
        }
    }

    /// Marks the given [`MastNodeId`] as being the root of a procedure.
    pub fn make_root(&mut self, new_root_id: MastNodeId) {
        self.mast_forest.make_root(new_root_id)
    }
}

impl Index<MastNodeId> for MastForestBuilder {
    type Output = MastNode;

    #[inline(always)]
    fn index(&self, node_id: MastNodeId) -> &Self::Output {
        &self.mast_forest[node_id]
    }
}
