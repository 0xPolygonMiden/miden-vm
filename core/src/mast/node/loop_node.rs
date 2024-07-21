use core::fmt;

use miden_crypto::{hash::rpo::RpoDigest, Felt};
use miden_formatting::prettier::PrettyPrint;

use crate::{
    chiplets::hasher,
    mast::{MastForest, MastNodeId},
    OPCODE_LOOP,
};

// LOOP NODE
// ================================================================================================

/// A Loop node defines condition-controlled iterative execution. When the VM encounters a Loop
/// node, it will keep executing the body of the loop as long as the top of the stack is `1``.
///
/// The loop is exited when at the end of executing the loop body the top of the stack is `0``.
/// If the top of the stack is neither `0` nor `1` when the condition is checked, the execution
/// fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopNode {
    body: MastNodeId,
    digest: RpoDigest,
}

/// Constants
impl LoopNode {
    /// The domain of the loop node (used for control block hashing).
    pub const DOMAIN: Felt = Felt::new(OPCODE_LOOP as u64);
}

/// Constructors
impl LoopNode {
    pub fn new(body: MastNodeId, mast_forest: &MastForest) -> Self {
        let digest = {
            let body_hash = mast_forest[body].digest();

            hasher::merge_in_domain(&[body_hash, RpoDigest::default()], Self::DOMAIN)
        };

        Self { body, digest }
    }
}

impl LoopNode {
    /// Returns a commitment to this Loop node.
    ///
    /// The commitment is computed as a hash of the loop body and an empty word ([ZERO; 4]) in
    /// the domain defined by [Self::DOMAIN] - i..e,:
    ///
    /// hasher::merge_in_domain(&[on_true_digest, Digest::default()], LoopNode::DOMAIN)
    pub fn digest(&self) -> RpoDigest {
        self.digest
    }

    /// Returns the ID of the node presenting the body of the loop.
    pub fn body(&self) -> MastNodeId {
        self.body
    }
}

// PRETTY PRINTING
// ================================================================================================

impl LoopNode {
    pub(super) fn to_display<'a>(&'a self, mast_forest: &'a MastForest) -> impl fmt::Display + 'a {
        LoopNodePrettyPrint {
            loop_node: self,
            mast_forest,
        }
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

struct LoopNodePrettyPrint<'a> {
    loop_node: &'a LoopNode,
    mast_forest: &'a MastForest,
}

impl<'a> crate::prettier::PrettyPrint for LoopNodePrettyPrint<'a> {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;

        let loop_body = self.mast_forest[self.loop_node.body].to_pretty_print(self.mast_forest);

        indent(4, const_text("while.true") + nl() + loop_body.render()) + nl() + const_text("end")
    }
}

impl<'a> fmt::Display for LoopNodePrettyPrint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::prettier::PrettyPrint;
        self.pretty_print(f)
    }
}
