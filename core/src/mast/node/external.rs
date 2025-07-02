use alloc::vec::Vec;
use core::fmt;

use miden_crypto::Word;
use miden_formatting::{
    hex::ToHex,
    prettier::{Document, PrettyPrint, const_text, nl, text},
};

use super::MastNodeExt;
use crate::{
    OperationId,
    mast::{DecoratorId, MastForest},
};

// EXTERNAL NODE
// ================================================================================================

/// Node for referencing procedures not present in a given [`MastForest`] (hence "external").
///
/// External nodes can be used to verify the integrity of a program's hash while keeping parts of
/// the program secret. They also allow a program to refer to a well-known procedure that was not
/// compiled with the program (e.g. a procedure in the standard library).
///
/// The hash of an external node is the hash of the procedure it represents, such that an external
/// node can be swapped with the actual subtree that it represents without changing the MAST root.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternalNode {
    digest: Word,
    before_enter: Vec<DecoratorId>,
    after_exit: Vec<DecoratorId>,
}

impl ExternalNode {
    /// Returns a new [`ExternalNode`] instantiated with the specified procedure hash.
    pub fn new(procedure_hash: Word) -> Self {
        Self {
            digest: procedure_hash,
            before_enter: Vec::new(),
            after_exit: Vec::new(),
        }
    }
}

impl ExternalNode {
    /// Returns the commitment to the MAST node referenced by this external node.
    pub fn digest(&self) -> Word {
        self.digest
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
impl ExternalNode {
    /// Sets the list of decorators to be executed before this node.
    pub fn append_before_enter(&mut self, decorator_ids: &[DecoratorId]) {
        self.before_enter.extend_from_slice(decorator_ids);
    }

    /// Sets the list of decorators to be executed after this node.
    pub fn append_after_exit(&mut self, decorator_ids: &[DecoratorId]) {
        self.after_exit.extend_from_slice(decorator_ids);
    }
}

impl MastNodeExt for ExternalNode {
    fn decorators(&self) -> impl Iterator<Item = (usize, DecoratorId)> {
        self.before_enter.iter().chain(&self.after_exit).copied().enumerate()
    }
}

// PRETTY PRINTING
// ================================================================================================

impl ExternalNode {
    pub(super) fn to_display<'a>(
        &'a self,
        mast_forest: &'a MastForest,
        node_id: usize,
    ) -> impl fmt::Display + 'a {
        ExternalNodePrettyPrint { node: self, mast_forest, node_id }
    }

    pub(super) fn to_pretty_print<'a>(
        &'a self,
        mast_forest: &'a MastForest,
        node_id: usize,
    ) -> impl PrettyPrint + 'a {
        ExternalNodePrettyPrint { node: self, mast_forest, node_id }
    }
}

struct ExternalNodePrettyPrint<'a> {
    node_id: usize,
    node: &'a ExternalNode,
    mast_forest: &'a MastForest,
}

impl ExternalNodePrettyPrint<'_> {
    /// Concatenates the provided decorators in a single line. If the list of decorators is not
    /// empty, prepends `prepend` and appends `append` to the decorator document.
    fn concatenate_decorators(
        &self,
        before: bool,
        prepend: Document,
        append: Document,
    ) -> Document {
        let op_id = OperationId {
            node: self.node_id,
            batch_idx: 0,
            op_id_in_batch: 0,
        };

        let decorator_ids = if before {
            self.mast_forest.debug_info.get_decorator_ids_before(&op_id)
        } else {
            self.mast_forest.debug_info.get_decorator_ids_after(&op_id)
        };
        let decorators = if let Some(decorator_ids) = decorator_ids {
            decorator_ids
                .iter()
                .map(|&decorator_id| self.mast_forest.debug_info.decorators[decorator_id].render())
                .reduce(|acc, doc| acc + const_text(" ") + doc)
                .unwrap_or_default()
        } else {
            Document::default()
        };
        if decorators.is_empty() {
            decorators
        } else {
            prepend + decorators + append
        }
    }

    fn single_line_pre_decorators(&self) -> Document {
        self.concatenate_decorators(true, Document::Empty, const_text(" "))
    }

    fn single_line_post_decorators(&self) -> Document {
        self.concatenate_decorators(false, const_text(" "), Document::Empty)
    }

    fn multi_line_pre_decorators(&self) -> Document {
        self.concatenate_decorators(true, Document::Empty, nl())
    }

    fn multi_line_post_decorators(&self) -> Document {
        self.concatenate_decorators(false, nl(), Document::Empty)
    }
}

impl crate::prettier::PrettyPrint for ExternalNodePrettyPrint<'_> {
    fn render(&self) -> crate::prettier::Document {
        let external = const_text("external")
            + const_text(".")
            + text(self.node.digest.as_bytes().to_hex_with_prefix());

        let single_line = self.single_line_pre_decorators()
            + external.clone()
            + self.single_line_post_decorators();
        let multi_line =
            self.multi_line_pre_decorators() + external + self.multi_line_post_decorators();

        single_line | multi_line
    }
}

impl fmt::Display for ExternalNodePrettyPrint<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::prettier::PrettyPrint;
        self.pretty_print(f)
    }
}
