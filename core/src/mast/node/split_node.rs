use core::fmt;

use miden_crypto::{hash::rpo::RpoDigest, Felt};
use miden_formatting::prettier::PrettyPrint;

use crate::{
    chiplets::hasher,
    mast::{MastForest, MastNodeId},
    OPCODE_SPLIT,
};

// SPLIT NODE
// ================================================================================================

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
    pub fn new(branches: [MastNodeId; 2], mast_forest: &MastForest) -> Self {
        let digest = {
            let if_branch_hash = mast_forest[branches[0]].digest();
            let else_branch_hash = mast_forest[branches[1]].digest();

            hasher::merge_in_domain(&[if_branch_hash, else_branch_hash], Self::DOMAIN)
        };

        Self { branches, digest }
    }

    #[cfg(test)]
    pub fn new_test(branches: [MastNodeId; 2], digest: RpoDigest) -> Self {
        Self { branches, digest }
    }
}

/// Public accessors
impl SplitNode {
    pub fn on_true(&self) -> MastNodeId {
        self.branches[0]
    }

    pub fn on_false(&self) -> MastNodeId {
        self.branches[1]
    }

    pub fn digest(&self) -> RpoDigest {
        self.digest
    }

    pub fn to_display<'a>(&'a self, mast_forest: &'a MastForest) -> impl core::fmt::Display + 'a {
        SplitNodePrettyPrint {
            split_node: self,
            mast_forest,
        }
    }
}

// PRETTY PRINTING
// ================================================================================================

impl SplitNode {
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
