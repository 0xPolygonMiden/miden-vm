use miden_crypto::{hash::rpo::RpoDigest, Felt};

use crate::{chiplets::hasher, Operation};

use super::{MastForest, MastNodeId, MerkleTreeNode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallNode {
    callee: MastNodeId,
    is_syscall: bool,
    digest: RpoDigest,
}

/// Constants
impl CallNode {
    /// The domain of the call block (used for control block hashing).
    pub const CALL_DOMAIN: Felt = Felt::new(Operation::Call.op_code() as u64);
    /// The domain of the syscall block (used for control block hashing).
    pub const SYSCALL_DOMAIN: Felt = Felt::new(Operation::SysCall.op_code() as u64);
}

/// Constructors
impl CallNode {
    /// Returns a new [`CallNode`] instantiated with the specified callee.
    pub fn new(callee: MastNodeId, mast_forest: &MastForest) -> Self {
        let digest = {
            let callee_digest = mast_forest.get_node_by_id(callee).digest();

            hasher::merge_in_domain(&[callee_digest, RpoDigest::default()], Self::CALL_DOMAIN)
        };

        Self {
            callee,
            is_syscall: false,
            digest,
        }
    }

    /// Returns a new [`CallNode`] instantiated with the specified callee and marked as a kernel
    /// call.
    pub fn new_syscall(callee: MastNodeId, mast_forest: &MastForest) -> Self {
        let digest = {
            let callee_digest = mast_forest.get_node_by_id(callee).digest();

            hasher::merge_in_domain(&[callee_digest, RpoDigest::default()], Self::SYSCALL_DOMAIN)
        };

        Self {
            callee,
            is_syscall: true,
            digest,
        }
    }
}

impl CallNode {
    pub fn callee(&self) -> MastNodeId {
        self.callee
    }

    pub fn is_syscall(&self) -> bool {
        self.is_syscall
    }

    /// Returns the domain of the call node.
    pub fn hash_domain(&self) -> Felt {
        if self.is_syscall() {
            Self::SYSCALL_DOMAIN
        } else {
            Self::CALL_DOMAIN
        }
    }
}

impl MerkleTreeNode for CallNode {
    fn digest(&self) -> RpoDigest {
        self.digest
    }
}
