use core::fmt;

use miden_crypto::{hash::rpo::RpoDigest, Felt};
use miden_formatting::prettier::PrettyPrint;

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

    pub(super) fn to_pretty_print<'a>(
        &'a self,
        mast_forest: &'a MastForest,
    ) -> impl PrettyPrint + 'a {
        LoopNodePrettyPrint {
            loop_node: self,
            mast_forest,
        }
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

    fn to_display<'a>(&'a self, mast_forest: &'a MastForest) -> impl fmt::Display + 'a {
        LoopNodePrettyPrint {
            loop_node: self,
            mast_forest,
        }
    }
}

struct LoopNodePrettyPrint<'a> {
    loop_node: &'a LoopNode,
    mast_forest: &'a MastForest,
}

impl<'a> crate::prettier::PrettyPrint for LoopNodePrettyPrint<'a> {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;

        let loop_body = self
            .mast_forest
            .get_node_by_id(self.loop_node.body)
            .to_pretty_print(self.mast_forest);

        indent(4, const_text("while.true") + nl() + loop_body.render()) + nl() + const_text("end")
    }
}

impl<'a> fmt::Display for LoopNodePrettyPrint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::prettier::PrettyPrint;
        self.pretty_print(f)
    }
}
