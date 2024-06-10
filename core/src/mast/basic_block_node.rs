use core::fmt;

use alloc::vec::Vec;
use miden_crypto::{hash::rpo::RpoDigest, Felt, ZERO};
use miden_formatting::prettier::PrettyPrint;

use crate::{
    program::blocks::{batch_ops, OpBatch},
    DecoratorIterator, DecoratorList, MastForest, Operation,
};

use super::MerkleTreeNode;

/// Block for a linear sequence of operations (i.e., no branching or loops).
///
/// Executes its operations in order. Fails if any of the operations fails.
///
/// A basic block is composed of operation batches, operation batches are composed of operation
/// groups, operation groups encode the VM's operations and immediate values. These values are
/// created according to these rules:
///
/// - A basic block contains one or more batches.
/// - A batch contains exactly 8 groups.
/// - A group contains exactly 9 operations or 1 immediate value.
/// - NOOPs are used to fill a group or batch when necessary.
/// - An immediate value follows the operation that requires it, using the next available group in
///   the batch. If there are no batches available in the group, then both the operation and its
///   immediate are moved to the next batch.
///
/// Example: 8 pushes result in two operation batches:
///
/// - First batch: First group with 7 push opcodes and 2 zero-paddings packed together, followed by
///   7 groups with their respective immediate values.
/// - Second batch: First group with the last push opcode and 8 zero-paddings packed together,
///   followed by one immediate and 6 padding groups.
///
/// The hash of a basic block is:
///
/// > hash(batches, domain=BASIC_BLOCK_DOMAIN)
///
/// Where `batches` is the concatenation of each `batch` in the basic block, and each batch is 8
/// field elements (512 bits).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasicBlockNode {
    /// The primitive operations contained in this basic block.
    ///
    /// The operations are broken up into batches of 8 groups,
    /// with each group containing up to 9 operations, or a
    /// single immediates. Thus the maximum size of each batch
    /// is 72 operations. Multiple batches are used for blocks
    /// consisting of more than 72 operations.
    op_batches: Vec<OpBatch>,
    digest: RpoDigest,
    decorators: DecoratorList,
}

/// Constants
impl BasicBlockNode {
    /// The domain of the span block (used for control block hashing).
    pub const DOMAIN: Felt = ZERO;
}

/// Constructors
impl BasicBlockNode {
    /// Returns a new [`BasicBlockNode`] instantiated with the specified operations.
    ///
    /// # Errors (TODO)
    /// Returns an error if:
    /// - `operations` vector is empty.
    /// - `operations` vector contains any number of system operations.
    pub fn new(operations: Vec<Operation>) -> Self {
        assert!(!operations.is_empty()); // TODO: return error
        Self::with_decorators(operations, DecoratorList::new())
    }

    /// Returns a new [`BasicBlockNode`] instantiated with the specified operations and decorators.
    ///
    /// # Errors (TODO)
    /// Returns an error if:
    /// - `operations` vector is empty.
    /// - `operations` vector contains any number of system operations.
    pub fn with_decorators(operations: Vec<Operation>, decorators: DecoratorList) -> Self {
        assert!(!operations.is_empty()); // TODO: return error

        // validate decorators list (only in debug mode)
        #[cfg(debug_assertions)]
        validate_decorators(&operations, &decorators);

        let (op_batches, digest) = batch_ops(operations);
        Self {
            op_batches,
            digest,
            decorators,
        }
    }
}

/// Public accessors
impl BasicBlockNode {
    pub fn op_batches(&self) -> &[OpBatch] {
        &self.op_batches
    }

    /// Returns a [`DecoratorIterator`] which allows us to iterate through the decorator list of
    /// this span block while executing operation batches of this span block
    pub fn decorator_iter(&self) -> DecoratorIterator {
        DecoratorIterator::new(&self.decorators)
    }

    /// Returns a list of decorators in this span block
    pub fn decorators(&self) -> &DecoratorList {
        &self.decorators
    }
}

impl MerkleTreeNode for BasicBlockNode {
    fn digest(&self) -> RpoDigest {
        self.digest
    }

    fn to_display<'a>(&'a self, _mast_forest: &'a MastForest) -> impl fmt::Display + 'a {
        self
    }
}

/// Checks if a given decorators list is valid (only checked in debug mode)
/// - Assert the decorator list is in ascending order.
/// - Assert the last op index in decorator list is less than or equal to the number of operations.
#[cfg(debug_assertions)]
fn validate_decorators(operations: &[Operation], decorators: &DecoratorList) {
    if !decorators.is_empty() {
        // check if decorator list is sorted
        for i in 0..(decorators.len() - 1) {
            debug_assert!(decorators[i + 1].0 >= decorators[i].0, "unsorted decorators list");
        }
        // assert the last index in decorator list is less than operations vector length
        debug_assert!(
            operations.len() >= decorators.last().expect("empty decorators list").0,
            "last op index in decorator list should be less than or equal to the number of ops"
        );
    }
}

impl PrettyPrint for BasicBlockNode {
    #[rustfmt::skip]
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;

        // TODOP: Change `span` -> `basic_block`
        // e.g. `span a b c end`
        let single_line = const_text("span")
            + const_text(" ")
            + self
                .op_batches
                .iter()
                .flat_map(|batch| batch.ops().iter())
                .map(|p| p.render())
                .reduce(|acc, doc| acc + const_text(" ") + doc)
                .unwrap_or_default()
            + const_text(" ")
            + const_text("end");

        // e.g. `
        // span
        //     a
        //     b
        //     c
        // end
        // `

        // TODOP: Change these to `basic_block`
        let multi_line = indent(
            4,
            const_text("span")
                + nl()
                + self
                    .op_batches
                    .iter()
                    .flat_map(|batch| batch.ops().iter())
                    .map(|p| p.render())
                    .reduce(|acc, doc| acc + nl() + doc)
                    .unwrap_or_default(),
        ) + nl()
            + const_text("end");

        single_line | multi_line
    }
}

impl fmt::Display for BasicBlockNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::prettier::PrettyPrint;
        self.pretty_print(f)
    }
}
