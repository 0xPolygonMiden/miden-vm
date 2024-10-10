use alloc::vec::Vec;
use core::fmt;

use miden_crypto::{hash::rpo::RpoDigest, Felt};
use miden_formatting::prettier::{const_text, nl, Document, PrettyPrint};

use crate::{
    mast::{DecoratorId, MastForest},
    OPCODE_DYN,
};

// DYN NODE
// ================================================================================================

/// A Dyn node specifies that the node to be executed next is defined dynamically via the stack.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DynNode {
    before_enter: Vec<DecoratorId>,
    after_exit: Vec<DecoratorId>,
}

/// Constants
impl DynNode {
    /// The domain of the Dyn block (used for control block hashing).
    pub const DOMAIN: Felt = Felt::new(OPCODE_DYN as u64);
}

/// Public accessors
impl DynNode {
    /// Returns a commitment to a Dyn node.
    ///
    /// The commitment is computed by hashing two empty words ([ZERO; 4]) in the domain defined
    /// by [Self::DOMAIN], i.e.:
    ///
    /// ```
    /// # use miden_core::mast::DynNode;
    /// # use miden_crypto::{hash::rpo::{RpoDigest as Digest, Rpo256 as Hasher}};
    /// Hasher::merge_in_domain(&[Digest::default(), Digest::default()], DynNode::DOMAIN);
    /// ```
    pub fn digest(&self) -> RpoDigest {
        RpoDigest::new([
            Felt::new(8115106948140260551),
            Felt::new(13491227816952616836),
            Felt::new(15015806788322198710),
            Felt::new(16575543461540527115),
        ])
    }

    /// Returns the decorators to be executed before this node is executed.
    pub fn before_enter(&self) -> &[DecoratorId] {
        &self.before_enter
    }

    /// Returns the decorators to be executed after this node is executed.
    pub fn after_exit(&self) -> &[DecoratorId] {
        &self.after_exit
    }
}

/// Mutators
impl DynNode {
    /// Sets the list of decorators to be executed before this node.
    pub fn set_before_enter(&mut self, decorator_ids: Vec<DecoratorId>) {
        self.before_enter = decorator_ids;
    }

    /// Sets the list of decorators to be executed after this node.
    pub fn set_after_exit(&mut self, decorator_ids: Vec<DecoratorId>) {
        self.after_exit = decorator_ids;
    }
}

// PRETTY PRINTING
// ================================================================================================

impl DynNode {
    pub(super) fn to_display<'a>(&'a self, mast_forest: &'a MastForest) -> impl fmt::Display + 'a {
        DynNodePrettyPrint { node: self, mast_forest }
    }

    pub(super) fn to_pretty_print<'a>(
        &'a self,
        mast_forest: &'a MastForest,
    ) -> impl PrettyPrint + 'a {
        DynNodePrettyPrint { node: self, mast_forest }
    }
}

struct DynNodePrettyPrint<'a> {
    node: &'a DynNode,
    mast_forest: &'a MastForest,
}

impl DynNodePrettyPrint<'_> {
    /// Concatenates the provided decorators in a single line. If the list of decorators is not
    /// empty, prepends `prepend` and appends `append` to the decorator document.
    fn concatenate_decorators(
        &self,
        decorator_ids: &[DecoratorId],
        prepend: Document,
        append: Document,
    ) -> Document {
        let decorators = decorator_ids
            .iter()
            .map(|&decorator_id| self.mast_forest[decorator_id].render())
            .reduce(|acc, doc| acc + const_text(" ") + doc)
            .unwrap_or_default();

        if decorators.is_empty() {
            decorators
        } else {
            prepend + decorators + append
        }
    }

    fn single_line_pre_decorators(&self) -> Document {
        self.concatenate_decorators(self.node.before_enter(), Document::Empty, const_text(" "))
    }

    fn single_line_post_decorators(&self) -> Document {
        self.concatenate_decorators(self.node.after_exit(), const_text(" "), Document::Empty)
    }

    fn multi_line_pre_decorators(&self) -> Document {
        self.concatenate_decorators(self.node.before_enter(), Document::Empty, nl())
    }

    fn multi_line_post_decorators(&self) -> Document {
        self.concatenate_decorators(self.node.after_exit(), nl(), Document::Empty)
    }
}

impl crate::prettier::PrettyPrint for DynNodePrettyPrint<'_> {
    fn render(&self) -> crate::prettier::Document {
        let dyn_text = const_text("dyn");

        let single_line = self.single_line_pre_decorators()
            + dyn_text.clone()
            + self.single_line_post_decorators();
        let multi_line =
            self.multi_line_pre_decorators() + dyn_text + self.multi_line_post_decorators();

        single_line | multi_line
    }
}

impl fmt::Display for DynNodePrettyPrint<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print(f)
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use miden_crypto::hash::rpo::Rpo256;

    use super::*;

    /// Ensures that the hash of `DynNode` is indeed the hash of 2 empty words, in the `DynNode`
    /// domain.
    #[test]
    pub fn test_dyn_node_digest() {
        assert_eq!(
            DynNode::default().digest(),
            Rpo256::merge_in_domain(&[RpoDigest::default(), RpoDigest::default()], DynNode::DOMAIN)
        );
    }
}
