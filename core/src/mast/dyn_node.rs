use miden_crypto::{hash::rpo::RpoDigest, Felt};

use crate::Operation;

use super::MerkleTreeNode;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DynNode;

/// Constants
impl DynNode {
    /// The domain of the Dyn block (used for control block hashing).
    pub const DOMAIN: Felt = Felt::new(Operation::Dyn.op_code() as u64);
}

impl MerkleTreeNode for DynNode {
    fn digest(&self) -> RpoDigest {
        // The Dyn node is represented by a constant, which is set to be the hash of two empty
        // words ([ZERO, ZERO, ZERO, ZERO]) with a domain value of `DYN_DOMAIN`, i.e.
        // hasher::merge_in_domain(&[Digest::default(), Digest::default()], DynNode::DOMAIN)
        RpoDigest::new([
            Felt::new(8115106948140260551),
            Felt::new(13491227816952616836),
            Felt::new(15015806788322198710),
            Felt::new(16575543461540527115),
        ])
    }
}
