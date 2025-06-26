use alloc::vec::Vec;
use core::{fmt, mem};

use miden_crypto::{Felt, Word, ZERO};
use miden_formatting::prettier::PrettyPrint;

use crate::{
    Decorator, DecoratorIterator, DecoratorList, Operation, OperationId,
    chiplets::hasher,
    debuginfo::DebugInfo,
    mast::{DecoratorId, MastForest, MastForestError},
};

mod op_batch;
pub use op_batch::OpBatch;
use op_batch::OpBatchAccumulator;

use super::MastNodeExt;

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

/// Maximum number of operations per group.
pub const GROUP_SIZE: usize = 9;

/// Maximum number of groups per batch.
pub const BATCH_SIZE: usize = 8;

// BASIC BLOCK NODE
// ================================================================================================

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
    /// The operations are broken up into batches of 8 groups, with each group containing up to 9
    /// operations, or a single immediates. Thus the maximum size of each batch is 72 operations.
    /// Multiple batches are used for blocks consisting of more than 72 operations.
    op_batches: Vec<OpBatch>,
    digest: Word,
    decorators: DecoratorList,
}

// ------------------------------------------------------------------------------------------------
/// Constants
impl BasicBlockNode {
    /// The domain of the basic block node (used for control block hashing).
    pub const DOMAIN: Felt = ZERO;
}

// ------------------------------------------------------------------------------------------------
/// Constructors
impl BasicBlockNode {
    /// Returns a new [`BasicBlockNode`] instantiated with the specified operations and decorators.
    ///
    /// Returns an error if:
    /// - `operations` vector is empty.
    pub fn new(
        operations: Vec<Operation>,
        decorators: Option<DecoratorList>,
    ) -> Result<Self, MastForestError> {
        if operations.is_empty() {
            return Err(MastForestError::EmptyBasicBlock);
        }

        // None is equivalent to an empty list of decorators moving forward.
        let decorators = decorators.unwrap_or_default();

        // Validate decorators list (only in debug mode).
        #[cfg(debug_assertions)]
        validate_decorators(&operations, &decorators);

        let (op_batches, digest) = batch_and_hash_ops(operations);
        Ok(Self { op_batches, digest, decorators })
    }

    /// Returns a new [`BasicBlockNode`] from values that are assumed to be correct.
    /// Should only be used when the source of the inputs is trusted (e.g. deserialization).
    pub fn new_unsafe(operations: Vec<Operation>, decorators: DecoratorList, digest: Word) -> Self {
        assert!(!operations.is_empty());
        let op_batches = batch_ops(operations);
        Self { op_batches, digest, decorators }
    }

    /// Returns a new [`BasicBlockNode`] instantiated with the specified operations and decorators.
    #[cfg(test)]
    pub fn new_with_raw_decorators(
        operations: Vec<Operation>,
        decorators: Vec<(usize, crate::Decorator)>,
        mast_forest: &mut crate::mast::MastForest,
    ) -> Result<Self, MastForestError> {
        let mut decorator_list = Vec::new();
        for (idx, decorator) in decorators {
            decorator_list.push((idx, mast_forest.add_decorator(decorator)?));
        }

        Self::new(operations, Some(decorator_list))
    }
}

// ------------------------------------------------------------------------------------------------
/// Public accessors
impl BasicBlockNode {
    /// Returns a commitment to this basic block.
    pub fn digest(&self) -> Word {
        self.digest
    }

    /// Returns a reference to the operation batches in this basic block.
    pub fn op_batches(&self) -> &[OpBatch] {
        &self.op_batches
    }

    /// Returns the number of operation batches in this basic block.
    pub fn num_op_batches(&self) -> usize {
        self.op_batches.len()
    }

    /// Returns the total number of operation groups in this basic block.
    ///
    /// Then number of operation groups is computed as follows:
    /// - For all batches but the last one we set the number of groups to 8, regardless of the
    ///   actual number of groups in the batch. The reason for this is that when operation batches
    ///   are concatenated together each batch contributes 8 elements to the hash.
    /// - For the last batch, we take the number of actual groups and round it up to the next power
    ///   of two. The reason for rounding is that the VM always executes a number of operation
    ///   groups which is a power of two.
    pub fn num_op_groups(&self) -> usize {
        let last_batch_num_groups = self.op_batches.last().expect("no last group").num_groups();
        (self.op_batches.len() - 1) * BATCH_SIZE + last_batch_num_groups.next_power_of_two()
    }

    /// Returns the number of operations in this basic block.
    pub fn num_operations(&self) -> u32 {
        let num_ops: usize = self.op_batches.iter().map(|batch| batch.ops().len()).sum();
        num_ops.try_into().expect("basic block contains more than 2^32 operations")
    }

    /// Returns a list of decorators in this basic block node.
    ///
    /// Each decorator is accompanied by the operation index specifying the operation prior to
    /// which the decorator should be executed.
    pub fn decorators(&self) -> &DecoratorList {
        &self.decorators
    }

    /// Returns a [`DecoratorIterator`] which allows us to iterate through the decorator list of
    /// this basic block node while executing operation batches of this basic block node.
    pub fn decorator_iter(&self) -> DecoratorIterator<'_> {
        DecoratorIterator::new(&self.decorators)
    }

    /// Returns an iterator over the operations in the order in which they appear in the program.
    pub fn operations(&self) -> impl Iterator<Item = &Operation> {
        self.op_batches.iter().flat_map(|batch| batch.ops())
    }

    /// Returns the total number of operations and decorators in this basic block.
    pub fn num_operations_and_decorators(&self) -> u32 {
        let num_ops: usize = self.num_operations() as usize;
        let num_decorators = self.decorators.len();

        (num_ops + num_decorators)
            .try_into()
            .expect("basic block contains more than 2^32 operations and decorators")
    }

    /// Returns an iterator over all operations and decorator, in the order in which they appear in
    /// the program.
    pub fn iter<'a>(
        &'a self,
        node_id: usize,
        debug_info: &'a DebugInfo,
    ) -> impl Iterator<Item = OperationOrDecorator<'a>> {
        OperationOrDecoratorIterator::new(self, node_id, debug_info)
    }
}

/// Mutators
impl BasicBlockNode {
    /// Sets the provided list of decorators to be executed before all existing decorators.
    pub fn prepend_decorators(&mut self, decorator_ids: &[DecoratorId]) {
        let mut new_decorators: DecoratorList =
            decorator_ids.iter().map(|decorator_id| (0, *decorator_id)).collect();
        new_decorators.extend(mem::take(&mut self.decorators));

        self.decorators = new_decorators;
    }

    /// Sets the provided list of decorators to be executed after all existing decorators.
    pub fn append_decorators(&mut self, decorator_ids: &[DecoratorId]) {
        let after_last_op_idx = self.num_operations() as usize;

        self.decorators
            .extend(decorator_ids.iter().map(|&decorator_id| (after_last_op_idx, decorator_id)));
    }

    /// Used to initialize decorators for the [`BasicBlockNode`]. Replaces the existing decorators
    /// with the given ['DecoratorList'].
    pub fn set_decorators(&mut self, decorator_list: DecoratorList) {
        self.decorators = decorator_list;
    }

    /// Clears the decorators.
    pub fn clear_decorators(&mut self) {
        self.decorators.clear();
    }
}

impl MastNodeExt for BasicBlockNode {
    fn decorators(&self) -> impl Iterator<Item = (usize, DecoratorId)> {
        self.decorators.iter().copied()
    }
}

// PRETTY PRINTING
// ================================================================================================

impl BasicBlockNode {
    pub(super) fn to_display<'a>(
        &'a self,
        mast_forest: &'a MastForest,
        node_id: usize,
    ) -> impl fmt::Display + 'a {
        BasicBlockNodePrettyPrint { block_node: self, mast_forest, node_id }
    }

    pub(super) fn to_pretty_print<'a>(
        &'a self,
        mast_forest: &'a MastForest,
        node_id: usize,
    ) -> impl PrettyPrint + 'a {
        BasicBlockNodePrettyPrint { block_node: self, mast_forest, node_id }
    }
}

struct BasicBlockNodePrettyPrint<'a> {
    node_id: usize,
    block_node: &'a BasicBlockNode,
    mast_forest: &'a MastForest,
}

impl PrettyPrint for BasicBlockNodePrettyPrint<'_> {
    #[rustfmt::skip]
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;

        // e.g. `basic_block a b c end`
        let single_line = const_text("basic_block")
            + const_text(" ")
            + self.
                block_node
                .iter(self.node_id, &self.mast_forest.debug_info)
                .map(|op_or_dec| match op_or_dec {
                    OperationOrDecorator::Operation(op) => op.render(),
                    OperationOrDecorator::Decorator(decorator) => decorator.render(),
                })
                .reduce(|acc, doc| acc + const_text(" ") + doc)
                .unwrap_or_default()
            + const_text(" ")
            + const_text("end");

        // e.g. `
        // basic_block
        //     a
        //     b
        //     c
        // end
        // `

        let multi_line = indent(
            4,
            const_text("basic_block")
                + nl()
                + self
                    .block_node
                .iter(self.node_id, &self.mast_forest.debug_info)
                    .map(|op_or_dec| match op_or_dec {
                        OperationOrDecorator::Operation(op) => op.render(),
                        OperationOrDecorator::Decorator(decorator) => decorator.render(),
                    })
                    .reduce(|acc, doc| acc + nl() + doc)
                    .unwrap_or_default(),
        ) + nl()
            + const_text("end");

        single_line | multi_line
    }
}

impl fmt::Display for BasicBlockNodePrettyPrint<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::prettier::PrettyPrint;
        self.pretty_print(f)
    }
}

// OPERATION OR DECORATOR
// ================================================================================================

/// Encodes either an [`Operation`] or a [`crate::Decorator`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OperationOrDecorator<'a> {
    Operation(&'a Operation),
    Decorator(&'a Decorator),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct OperationOrDecoratorIterator<'a> {
    debug_info: &'a DebugInfo,
    node: &'a BasicBlockNode,
    node_id: usize,
    /// The index of the current batch
    batch_index: usize,

    /// The index of the operation in the current batch
    op_index_in_batch: usize,

    /// The index of the current operation across all batches
    op_index: usize,

    /// The index of the next element in `node.decorator_list`. This list is assumed to be sorted.
    decorator_list_next_index: usize,
}

impl<'a> OperationOrDecoratorIterator<'a> {
    fn new(node: &'a BasicBlockNode, node_id: usize, debug_info: &'a DebugInfo) -> Self {
        Self {
            debug_info,
            node,
            node_id,
            batch_index: 0,
            op_index_in_batch: 0,
            op_index: 0,
            decorator_list_next_index: 0,
        }
    }
}

impl<'a> Iterator for OperationOrDecoratorIterator<'a> {
    type Item = OperationOrDecorator<'a>;

    // here there is some logic
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(batch) = self.node.op_batches.get(self.batch_index) {
            // check if there's a decorator to execute
            let op_id = OperationId {
                node: self.node_id,
                batch_idx: 0,
                op_id_in_batch: self.batch_index + self.op_index_in_batch,
            };
            if let Some(decorator_ids) = self.debug_info.get_decorator_ids_before(&op_id) {
                if self.decorator_list_next_index < decorator_ids.len() {
                    let decorator_id = decorator_ids[self.decorator_list_next_index];
                    self.decorator_list_next_index += 1;
                    return Some(OperationOrDecorator::Decorator(
                        &self.debug_info.decorators[decorator_id],
                    ));
                } else {
                    // if the decorators are finished, reset the counter
                    self.decorator_list_next_index = 0
                }
            }

            // If no decorator needs to be executed, then execute the operation
            if let Some(operation) = batch.ops.get(self.op_index_in_batch) {
                self.op_index_in_batch += 1;
                self.op_index += 1;

                Some(OperationOrDecorator::Operation(operation))
            } else {
                self.batch_index += 1;
                self.op_index_in_batch = 0;

                self.next()
            }
        } else {
            None
        }
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Groups the provided operations into batches and computes the hash of the block.
fn batch_and_hash_ops(ops: Vec<Operation>) -> (Vec<OpBatch>, Word) {
    // Group the operations into batches.
    let batches = batch_ops(ops);

    // Compute the hash of all operation groups.
    let op_groups: Vec<Felt> = batches.iter().flat_map(|batch| batch.groups).collect();
    let hash = hasher::hash_elements(&op_groups);

    (batches, hash)
}

/// Groups the provided operations into batches as described in the docs for this module (i.e., up
/// to 9 operations per group, and 8 groups per batch).
fn batch_ops(ops: Vec<Operation>) -> Vec<OpBatch> {
    let mut batches = Vec::<OpBatch>::new();
    let mut batch_acc = OpBatchAccumulator::new();

    for op in ops {
        // If the operation cannot be accepted into the current accumulator, add the contents of
        // the accumulator to the list of batches and start a new accumulator.
        if !batch_acc.can_accept_op(op) {
            let batch = batch_acc.into_batch();
            batch_acc = OpBatchAccumulator::new();

            batches.push(batch);
        }

        // Add the operation to the accumulator.
        batch_acc.add_op(op);
    }

    // Make sure we finished processing the last batch.
    if !batch_acc.is_empty() {
        let batch = batch_acc.into_batch();
        batches.push(batch);
    }

    batches
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
