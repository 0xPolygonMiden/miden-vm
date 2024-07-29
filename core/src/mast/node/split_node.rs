use core::fmt;

use miden_crypto::{hash::rpo::RpoDigest, Felt};
use miden_formatting::prettier::PrettyPrint;

use crate::{
    chiplets::hasher,
    mast::{MastForest, MastForestError, MastNodeId},
    OPCODE_SPLIT,
};

// SPLIT NODE
// ================================================================================================

/// A Split node defines conditional execution. When the VM encounters a Split node it executes
/// either the `on_true` child or `on_false` child.
///
/// Which child is executed is determined based on the top of the stack. If the value is `1`, then
/// the `on_true` child is executed. If the value is `0`, then the `on_false` child is executed. If
/// the value is neither `0` nor `1`, the execution fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplitNode {
    branches: [MastNodeId; 2],
    digest: RpoDigest,
}

/// Constants
impl SplitNode {
    /// The domain of the split node (used for control block hashing).
    pub const DOMAIN: Felt = Felt::new(OPCODE_SPLIT as u64);
}

/// Constructors
impl SplitNode {
    pub fn new(
        branches: [MastNodeId; 2],
        mast_forest: &MastForest,
    ) -> Result<Self, MastForestError> {
        let forest_len = mast_forest.nodes.len();
        if usize::from(branches[0]) >= forest_len {
            return Err(MastForestError::NodeIdOverflow(branches[0], forest_len));
        } else if usize::from(branches[1]) >= forest_len {
            return Err(MastForestError::NodeIdOverflow(branches[1], forest_len));
        }
        let digest = {
            let if_branch_hash = mast_forest[branches[0]].digest();
            let else_branch_hash = mast_forest[branches[1]].digest();

            hasher::merge_in_domain(&[if_branch_hash, else_branch_hash], Self::DOMAIN)
        };

        Ok(Self { branches, digest })
    }

    #[cfg(test)]
    pub fn new_test(branches: [MastNodeId; 2], digest: RpoDigest) -> Self {
        Self { branches, digest }
    }
}

/// Public accessors
impl SplitNode {
    /// Returns a commitment to this Split node.
    ///
    /// The commitment is computed as a hash of the `on_true` and `on_false` child nodes in the
    /// domain defined by [Self::DOMAIN] - i..e,:
    /// ```
    /// # use miden_core::mast::SplitNode;
    /// # use miden_crypto::{hash::rpo::{RpoDigest as Digest, Rpo256 as Hasher}};
    /// # let on_true_digest = Digest::default();
    /// # let on_false_digest = Digest::default();
    /// Hasher::merge_in_domain(&[on_true_digest, on_false_digest], SplitNode::DOMAIN);
    /// ```
    pub fn digest(&self) -> RpoDigest {
        self.digest
    }

    /// Returns the ID of the node which is to be executed if the top of the stack is `1`.
    pub fn on_true(&self) -> MastNodeId {
        self.branches[0]
    }

    /// Returns the ID of the node which is to be executed if the top of the stack is `0`.
    pub fn on_false(&self) -> MastNodeId {
        self.branches[1]
    }
}

// PRETTY PRINTING
// ================================================================================================

impl SplitNode {
    pub(super) fn to_display<'a>(&'a self, mast_forest: &'a MastForest) -> impl fmt::Display + 'a {
        SplitNodePrettyPrint {
            split_node: self,
            mast_forest,
        }
    }

    pub(super) fn to_pretty_print<'a>(
        &'a self,
        mast_forest: &'a MastForest,
    ) -> impl PrettyPrint + 'a {
        SplitNodePrettyPrint {
            split_node: self,
            mast_forest,
        }
    }
}

struct SplitNodePrettyPrint<'a> {
    split_node: &'a SplitNode,
    mast_forest: &'a MastForest,
}

impl<'a> PrettyPrint for SplitNodePrettyPrint<'a> {
    #[rustfmt::skip]
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;

        let true_branch = self.mast_forest[self.split_node.on_true()].to_pretty_print(self.mast_forest);
        let false_branch = self.mast_forest[self.split_node.on_false()].to_pretty_print(self.mast_forest);

        let mut doc = indent(4, const_text("if.true") + nl() + true_branch.render()) + nl();
        doc += indent(4, const_text("else") + nl() + false_branch.render());
        doc + nl() + const_text("end")
    }
}

impl<'a> fmt::Display for SplitNodePrettyPrint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::prettier::PrettyPrint;
        self.pretty_print(f)
    }
}
