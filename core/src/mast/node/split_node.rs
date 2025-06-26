use alloc::vec::Vec;
use core::fmt;

use miden_crypto::{Felt, Word};
use miden_formatting::prettier::PrettyPrint;

use super::MastNodeExt;
use crate::{
    OPCODE_SPLIT, OperationId,
    chiplets::hasher,
    mast::{DecoratorId, MastForest, MastForestError, MastNodeId, Remapping},
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
    digest: Word,
    before_enter: Vec<DecoratorId>,
    after_exit: Vec<DecoratorId>,
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
        if branches[0].as_usize() >= forest_len {
            return Err(MastForestError::NodeIdOverflow(branches[0], forest_len));
        } else if branches[1].as_usize() >= forest_len {
            return Err(MastForestError::NodeIdOverflow(branches[1], forest_len));
        }
        let digest = {
            let if_branch_hash = mast_forest[branches[0]].digest();
            let else_branch_hash = mast_forest[branches[1]].digest();

            hasher::merge_in_domain(&[if_branch_hash, else_branch_hash], Self::DOMAIN)
        };

        Ok(Self {
            branches,
            digest,
            before_enter: Vec::new(),
            after_exit: Vec::new(),
        })
    }

    /// Returns a new [`SplitNode`] from values that are assumed to be correct.
    /// Should only be used when the source of the inputs is trusted (e.g. deserialization).
    pub fn new_unsafe(branches: [MastNodeId; 2], digest: Word) -> Self {
        Self {
            branches,
            digest,
            before_enter: Vec::new(),
            after_exit: Vec::new(),
        }
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
    /// # use miden_crypto::{Word, hash::rpo::Rpo256 as Hasher};
    /// # let on_true_digest = Word::default();
    /// # let on_false_digest = Word::default();
    /// Hasher::merge_in_domain(&[on_true_digest, on_false_digest], SplitNode::DOMAIN);
    /// ```
    pub fn digest(&self) -> Word {
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

    /// Returns the decorators to be executed before this node is executed.
    pub fn before_enter(&self) -> &[DecoratorId] {
        &self.before_enter
    }

    /// Returns the decorators to be executed after this node is executed.
    pub fn after_exit(&self) -> &[DecoratorId] {
        &self.after_exit
    }

    /// Clears the decorators.
    pub fn clear_decorators(&mut self) {
        self.before_enter.clear();
        self.after_exit.clear();
    }
}

/// Mutators
impl SplitNode {
    pub fn remap_children(&self, remapping: &Remapping) -> Self {
        let mut node = self.clone();
        node.branches[0] = node.branches[0].remap(remapping);
        node.branches[1] = node.branches[1].remap(remapping);
        node
    }

    /// Sets the list of decorators to be executed before this node.
    pub fn append_before_enter(&mut self, decorator_ids: &[DecoratorId]) {
        self.before_enter.extend_from_slice(decorator_ids);
    }

    /// Sets the list of decorators to be executed after this node.
    pub fn append_after_exit(&mut self, decorator_ids: &[DecoratorId]) {
        self.after_exit.extend_from_slice(decorator_ids);
    }
}

impl MastNodeExt for SplitNode {
    fn decorators(&self) -> impl Iterator<Item = (usize, DecoratorId)> {
        self.before_enter.iter().chain(&self.after_exit).copied().enumerate()
    }
}

// PRETTY PRINTING
// ================================================================================================

impl SplitNode {
    pub(super) fn to_display<'a>(
        &'a self,
        mast_forest: &'a MastForest,
        node_id: usize,
    ) -> impl fmt::Display + 'a {
        SplitNodePrettyPrint { split_node: self, mast_forest, node_id }
    }

    pub(super) fn to_pretty_print<'a>(
        &'a self,
        mast_forest: &'a MastForest,
        node_id: usize,
    ) -> impl PrettyPrint + 'a {
        SplitNodePrettyPrint { split_node: self, mast_forest, node_id }
    }
}

struct SplitNodePrettyPrint<'a> {
    node_id: usize,
    split_node: &'a SplitNode,
    mast_forest: &'a MastForest,
}

impl PrettyPrint for SplitNodePrettyPrint<'_> {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;

        let op_id = OperationId {
            node: self.node_id,
            batch_idx: 0,
            op_id_in_batch: 0,
        };

        let pre_decorators = {
            if let Some(decorator_ids) =
                self.mast_forest.debug_info.get_decorator_ids_before(&op_id)
            {
                let mut pre_decorators = decorator_ids
                    .iter()
                    .map(|&decorator_id| {
                        self.mast_forest.debug_info.decorators[decorator_id].render()
                    })
                    .reduce(|acc, doc| acc + const_text(" ") + doc)
                    .unwrap_or_default();
                if !pre_decorators.is_empty() {
                    pre_decorators += nl();
                }

                pre_decorators
            } else {
                Document::default()
            }
        };

        let post_decorators = {
            if let Some(decorator_ids) = self.mast_forest.debug_info.get_decorator_ids_after(&op_id)
            {
                let mut post_decorators = decorator_ids
                    .iter()
                    .map(|&decorator_id| {
                        self.mast_forest.debug_info.decorators[decorator_id].render()
                    })
                    .reduce(|acc, doc| acc + const_text(" ") + doc)
                    .unwrap_or_default();
                if !post_decorators.is_empty() {
                    post_decorators = nl() + post_decorators;
                }

                post_decorators
            } else {
                Document::default()
            }
        };

        let true_branch = self.mast_forest[self.split_node.on_true()]
            .to_pretty_print(self.mast_forest, self.split_node.on_true().into());
        let false_branch = self.mast_forest[self.split_node.on_false()]
            .to_pretty_print(self.mast_forest, self.split_node.on_false().into());

        let mut doc = pre_decorators;
        doc += indent(4, const_text("if.true") + nl() + true_branch.render()) + nl();
        doc += indent(4, const_text("else") + nl() + false_branch.render());
        doc += nl() + const_text("end");
        doc + post_decorators
    }
}

impl fmt::Display for SplitNodePrettyPrint<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::prettier::PrettyPrint;
        self.pretty_print(f)
    }
}
