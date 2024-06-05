use miden_crypto::{hash::rpo::RpoDigest, Felt};

use crate::{chiplets::hasher, Operation};

use super::{MastForest, MastNodeId, MerkleTreeNode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplitNode {
    branches: [MastNodeId; 2],
    digest: RpoDigest,
}

/// Constants
impl SplitNode {
    /// The domain of the split node (used for control block hashing).
    pub const DOMAIN: Felt = Felt::new(Operation::Split.op_code() as u64);
}

/// Constructors
impl SplitNode {
    pub fn new(branches: [MastNodeId; 2], mast_forest: &MastForest) -> Self {
        let digest = {
            let if_branch_hash = mast_forest.get_node_by_id(branches[0]).digest();
            let else_branch_hash = mast_forest.get_node_by_id(branches[1]).digest();

            hasher::merge_in_domain(&[if_branch_hash, else_branch_hash], Self::DOMAIN)
        };

        Self { branches, digest }
    }
}

impl SplitNode {
    pub fn on_true(&self) -> MastNodeId {
        self.branches[0]
    }

    pub fn on_false(&self) -> MastNodeId {
        self.branches[1]
    }
}

impl MerkleTreeNode for SplitNode {
    fn digest(&self) -> RpoDigest {
        self.digest
    }
}
