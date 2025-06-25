use alloc::vec::Vec;
use core::fmt;

use miden_crypto::{Felt, Word};
use miden_formatting::prettier::PrettyPrint;

use super::MastNodeExt;
use crate::{
    OPCODE_LOOP,
    chiplets::hasher,
    mast::{DecoratorId, MastForest, MastForestError, MastNodeId, Remapping},
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
    digest: Word,
    before_enter: Vec<DecoratorId>,
    after_exit: Vec<DecoratorId>,
}

/// Constants
impl LoopNode {
    /// The domain of the loop node (used for control block hashing).
    pub const DOMAIN: Felt = Felt::new(OPCODE_LOOP as u64);
}

/// Constructors
impl LoopNode {
    /// Returns a new [`LoopNode`] instantiated with the specified body node.
    pub fn new(body: MastNodeId, mast_forest: &MastForest) -> Result<Self, MastForestError> {
        if body.as_usize() >= mast_forest.nodes.len() {
            return Err(MastForestError::NodeIdOverflow(body, mast_forest.nodes.len()));
        }
        let digest = {
            let body_hash = mast_forest[body].digest();

            hasher::merge_in_domain(&[body_hash, Word::default()], Self::DOMAIN)
        };

        Ok(Self {
            body,
            digest,
            before_enter: Vec::new(),
            after_exit: Vec::new(),
        })
    }

    /// Returns a new [`LoopNode`] from values that are assumed to be correct.
    /// Should only be used when the source of the inputs is trusted (e.g. deserialization).
    pub fn new_unsafe(body: MastNodeId, digest: Word) -> Self {
        Self {
            body,
            digest,
            before_enter: Vec::new(),
            after_exit: Vec::new(),
        }
    }
}

impl LoopNode {
    /// Returns a commitment to this Loop node.
    ///
    /// The commitment is computed as a hash of the loop body and an empty word ([ZERO; 4]) in
    /// the domain defined by [Self::DOMAIN] - i..e,:
    /// ```
    /// # use miden_core::mast::LoopNode;
    /// # use miden_crypto::{Word, hash::rpo::Rpo256 as Hasher};
    /// # let body_digest = Word::default();
    /// Hasher::merge_in_domain(&[body_digest, Word::default()], LoopNode::DOMAIN);
    /// ```
    pub fn digest(&self) -> Word {
        self.digest
    }

    /// Returns the ID of the node presenting the body of the loop.
    pub fn body(&self) -> MastNodeId {
        self.body
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
impl LoopNode {
    pub fn remap_children(&self, remapping: &Remapping) -> Self {
        let mut node = self.clone();
        node.body = node.body.remap(remapping);
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

impl MastNodeExt for LoopNode {
    fn decorators(&self) -> impl Iterator<Item = (usize, DecoratorId)> {
        self.before_enter.iter().chain(&self.after_exit).copied().enumerate()
    }
}

// PRETTY PRINTING
// ================================================================================================

impl LoopNode {
    pub(super) fn to_display<'a>(&'a self, mast_forest: &'a MastForest) -> impl fmt::Display + 'a {
        LoopNodePrettyPrint { loop_node: self, mast_forest }
    }

    pub(super) fn to_pretty_print<'a>(
        &'a self,
        mast_forest: &'a MastForest,
    ) -> impl PrettyPrint + 'a {
        LoopNodePrettyPrint { loop_node: self, mast_forest }
    }
}

struct LoopNodePrettyPrint<'a> {
    loop_node: &'a LoopNode,
    mast_forest: &'a MastForest,
}

impl crate::prettier::PrettyPrint for LoopNodePrettyPrint<'_> {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;

        let pre_decorators = {
            let mut pre_decorators = self
                .loop_node
                .before_enter()
                .iter()
                .map(|&decorator_id| self.mast_forest[decorator_id].render())
                .reduce(|acc, doc| acc + const_text(" ") + doc)
                .unwrap_or_default();
            if !pre_decorators.is_empty() {
                pre_decorators += nl();
            }

            pre_decorators
        };

        let post_decorators = {
            let mut post_decorators = self
                .loop_node
                .after_exit()
                .iter()
                .map(|&decorator_id| self.mast_forest[decorator_id].render())
                .reduce(|acc, doc| acc + const_text(" ") + doc)
                .unwrap_or_default();
            if !post_decorators.is_empty() {
                post_decorators = nl() + post_decorators;
            }

            post_decorators
        };

        let loop_body = self.mast_forest[self.loop_node.body].to_pretty_print(self.mast_forest);

        pre_decorators
            + indent(4, const_text("while.true") + nl() + loop_body.render())
            + nl()
            + const_text("end")
            + post_decorators
    }
}

impl fmt::Display for LoopNodePrettyPrint<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::prettier::PrettyPrint;
        self.pretty_print(f)
    }
}
