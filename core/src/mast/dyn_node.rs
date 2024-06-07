use core::fmt;

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
    
    fn to_display<'a>(&'a self, _mast_forest: &'a crate::MastForest) -> impl fmt::Display + 'a {
        self
    }
}

impl crate::prettier::PrettyPrint for DynNode {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;
        const_text("dyn")
    }
}

impl fmt::Display for DynNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use miden_formatting::prettier::PrettyPrint;
        self.pretty_print(f)
    }
}
