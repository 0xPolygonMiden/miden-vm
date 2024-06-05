use miden_crypto::{hash::rpo::RpoDigest, Felt};

use crate::{chiplets::hasher, Operation};

use super::{MastForest, MastNodeId, MerkleTreeNode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopNode {
    body: MastNodeId,
    digest: RpoDigest,
}

/// Constants
impl LoopNode {
    /// The domain of the loop block (used for control block hashing).
    pub const DOMAIN: Felt = Felt::new(Operation::Loop.op_code() as u64);
}

/// Constructors
impl LoopNode {
    pub fn new(body: MastNodeId, mast_forest: &MastForest) -> Self {
        let digest = {
            let body_hash = mast_forest.get_node_by_id(body).digest();

            hasher::merge_in_domain(&[body_hash, RpoDigest::default()], Self::DOMAIN)
        };

        Self { body, digest }
    }
}

impl LoopNode {
    pub fn body(&self) -> MastNodeId {
        self.body
    }
}

impl MerkleTreeNode for LoopNode {
    fn digest(&self) -> RpoDigest {
        self.digest
    }
}
