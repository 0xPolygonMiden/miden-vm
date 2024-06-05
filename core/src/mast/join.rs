use miden_crypto::{hash::rpo::RpoDigest, Felt};

use crate::{chiplets::hasher, Operation};

use super::{MastForest, MastNodeId, MerkleTreeNode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinNode {
    children: [MastNodeId; 2],
    digest: RpoDigest,
}

/// Constants
impl JoinNode {
    /// The domain of the join block (used for control block hashing).
    pub const DOMAIN: Felt = Felt::new(Operation::Join.op_code() as u64);
}

/// Constructors
impl JoinNode {
    /// Returns a new [`JoinNode`] instantiated with the specified children nodes.
    pub fn new(children: [MastNodeId; 2], mast_forest: &MastForest) -> Self {
        let digest = {
            let left_child_hash = mast_forest.get_node_by_id(children[0]).digest();
            let right_child_hash = mast_forest.get_node_by_id(children[1]).digest();

            hasher::merge_in_domain(&[left_child_hash, right_child_hash], Self::DOMAIN)
        };

        Self { children, digest }
    }
}

/// Accessors
impl JoinNode {
    pub fn first(&self) -> MastNodeId {
        self.children[0]
    }

    pub fn second(&self) -> MastNodeId {
        self.children[1]
    }
}

impl MerkleTreeNode for JoinNode {
    fn digest(&self) -> RpoDigest {
        self.digest
    }
}
