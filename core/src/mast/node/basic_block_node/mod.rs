use core::fmt;

use alloc::vec::Vec;
use miden_crypto::{hash::rpo::RpoDigest, Felt, ZERO};
use miden_formatting::prettier::PrettyPrint;
use winter_utils::flatten_slice_elements;

use crate::{
    chiplets::hasher,
    mast::{MastForest, MerkleTreeNode},
    Decorator, DecoratorIterator, DecoratorList, Operation,
};

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
    digest: RpoDigest,
    decorators: DecoratorList,
}

/// Constants
impl BasicBlockNode {
    /// The domain of the basic block node (used for control block hashing).
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
    pub fn num_operations_and_decorators(&self) -> u32 {
        let num_ops: usize = self.op_batches.iter().map(|batch| batch.ops().len()).sum();
        let num_decorators = self.decorators.len();

        (num_ops + num_decorators)
            .try_into()
            .expect("basic block contains more than 2^32 operations and decorators")
    }

    pub fn op_batches(&self) -> &[OpBatch] {
        &self.op_batches
    }

    /// Returns an iterator over all operations and decorator, in the order in which they appear in
    /// the program.
    pub fn iter(&self) -> impl Iterator<Item = OperationOrDecorator> {
        OperationOrDecoratorIterator::new(self)
    }

    /// Returns a [`DecoratorIterator`] which allows us to iterate through the decorator list of
    /// this basic block node while executing operation batches of this basic block node.
    pub fn decorator_iter(&self) -> DecoratorIterator {
        DecoratorIterator::new(&self.decorators)
    }

    /// Returns a list of decorators in this basic block node.
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

        // e.g. `basic_block a b c end`
        let single_line = const_text("basic_block")
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

// OPERATION OR DECORATOR
// ================================================================================================

// TODOP: Document
pub enum OperationOrDecorator<'a> {
    Operation(&'a Operation),
    Decorator(&'a Decorator),
}

struct OperationOrDecoratorIterator<'a> {
    node: &'a BasicBlockNode,

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
    fn new(node: &'a BasicBlockNode) -> Self {
        Self {
            node,
            batch_index: 0,
            op_index_in_batch: 0,
            op_index: 0,
            decorator_list_next_index: 0,
        }
    }
}

impl<'a> Iterator for OperationOrDecoratorIterator<'a> {
    type Item = OperationOrDecorator<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // check if there's a decorator to execute
        if let Some((op_index, decorator)) =
            self.node.decorators.get(self.decorator_list_next_index)
        {
            if *op_index == self.op_index {
                self.decorator_list_next_index += 1;
                return Some(OperationOrDecorator::Decorator(decorator));
            }
        }

        // If no decorator needs to be executed, then execute the operation
        if let Some(batch) = self.node.op_batches.get(self.batch_index) {
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

// OPERATION BATCH
// ================================================================================================

/// A batch of operations in a [Span] block.
///
/// An operation batch consists of up to 8 operation groups, with each group containing up to 9
/// operations or a single immediate value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpBatch {
    ops: Vec<Operation>,
    groups: [Felt; BATCH_SIZE],
    op_counts: [usize; BATCH_SIZE],
    num_groups: usize,
}

impl OpBatch {
    /// Returns a list of operations contained in this batch.
    pub fn ops(&self) -> &[Operation] {
        &self.ops
    }

    /// Returns a list of operation groups contained in this batch.
    ///
    /// Each group is represented by a single field element.
    pub fn groups(&self) -> &[Felt; BATCH_SIZE] {
        &self.groups
    }

    /// Returns the number of non-decorator operations for each operation group.
    ///
    /// Number of operations for groups containing immediate values is set to 0.
    pub fn op_counts(&self) -> &[usize; BATCH_SIZE] {
        &self.op_counts
    }

    /// Returns the number of groups in this batch.
    pub fn num_groups(&self) -> usize {
        self.num_groups
    }
}

/// An accumulator used in construction of operation batches.
struct OpBatchAccumulator {
    /// A list of operations in this batch, including decorators.
    ops: Vec<Operation>,
    /// Values of operation groups, including immediate values.
    groups: [Felt; BATCH_SIZE],
    /// Number of non-decorator operations in each operation group. Operation count for groups
    /// with immediate values is set to 0.
    op_counts: [usize; BATCH_SIZE],
    /// Value of the currently active op group.
    group: u64,
    /// Index of the next opcode in the current group.
    op_idx: usize,
    /// index of the current group in the batch.
    group_idx: usize,
    // Index of the next free group in the batch.
    next_group_idx: usize,
}

impl OpBatchAccumulator {
    /// Returns a blank [OpBatchAccumulator].
    pub fn new() -> Self {
        Self {
            ops: Vec::new(),
            groups: [ZERO; BATCH_SIZE],
            op_counts: [0; BATCH_SIZE],
            group: 0,
            op_idx: 0,
            group_idx: 0,
            next_group_idx: 1,
        }
    }

    /// Returns true if this accumulator does not contain any operations.
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

    /// Returns true if this accumulator can accept the specified operation.
    ///
    /// An accumulator may not be able accept an operation for the following reasons:
    /// - There is no more space in the underlying batch (e.g., the 8th group of the batch already
    ///   contains 9 operations).
    /// - There is no space for the immediate value carried by the operation (e.g., the 8th group is
    ///   only partially full, but we are trying to add a PUSH operation).
    /// - The alignment rules require that the operation overflows into the next group, and if this
    ///   happens, there will be no space for the operation or its immediate value.
    pub fn can_accept_op(&self, op: Operation) -> bool {
        if op.imm_value().is_some() {
            // an operation carrying an immediate value cannot be the last one in a group; so, we
            // check if we need to move the operation to the next group. in either case, we need
            // to make sure there is enough space for the immediate value as well.
            if self.op_idx < GROUP_SIZE - 1 {
                self.next_group_idx < BATCH_SIZE
            } else {
                self.next_group_idx + 1 < BATCH_SIZE
            }
        } else {
            // check if there is space for the operation in the current group, or if there isn't,
            // whether we can add another group
            self.op_idx < GROUP_SIZE || self.next_group_idx < BATCH_SIZE
        }
    }

    /// Adds the specified operation to this accumulator. It is expected that the specified
    /// operation is not a decorator and that (can_accept_op())[OpBatchAccumulator::can_accept_op]
    /// is called before this function to make sure that the specified operation can be added to
    /// the accumulator.
    pub fn add_op(&mut self, op: Operation) {
        // if the group is full, finalize it and start a new group
        if self.op_idx == GROUP_SIZE {
            self.finalize_op_group();
        }

        // for operations with immediate values, we need to do a few more things
        if let Some(imm) = op.imm_value() {
            // since an operation with an immediate value cannot be the last one in a group, if
            // the operation would be the last one in the group, we need to start a new group
            if self.op_idx == GROUP_SIZE - 1 {
                self.finalize_op_group();
            }

            // save the immediate value at the next group index and advance the next group pointer
            self.groups[self.next_group_idx] = imm;
            self.next_group_idx += 1;
        }

        // add the opcode to the group and increment the op index pointer
        let opcode = op.op_code() as u64;
        self.group |= opcode << (Operation::OP_BITS * self.op_idx);
        self.ops.push(op);
        self.op_idx += 1;
    }

    /// Convert the accumulator into an [OpBatch].
    pub fn into_batch(mut self) -> OpBatch {
        // make sure the last group gets added to the group array; we also check the op_idx to
        // handle the case when a group contains a single NOOP operation.
        if self.group != 0 || self.op_idx != 0 {
            self.groups[self.group_idx] = Felt::new(self.group);
            self.op_counts[self.group_idx] = self.op_idx;
        }

        OpBatch {
            ops: self.ops,
            groups: self.groups,
            op_counts: self.op_counts,
            num_groups: self.next_group_idx,
        }
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Saves the current group into the group array, advances current and next group pointers,
    /// and resets group content.
    fn finalize_op_group(&mut self) {
        self.groups[self.group_idx] = Felt::new(self.group);
        self.op_counts[self.group_idx] = self.op_idx;

        self.group_idx = self.next_group_idx;
        self.next_group_idx = self.group_idx + 1;

        self.op_idx = 0;
        self.group = 0;
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Groups the provided operations into batches as described in the docs for this module (i.e.,
/// up to 9 operations per group, and 8 groups per batch).
///
/// After the operations have been grouped, computes the hash of the block.
fn batch_ops(ops: Vec<Operation>) -> (Vec<OpBatch>, RpoDigest) {
    let mut batch_acc = OpBatchAccumulator::new();
    let mut batches = Vec::<OpBatch>::new();
    let mut batch_groups = Vec::<[Felt; BATCH_SIZE]>::new();

    for op in ops {
        // if the operation cannot be accepted into the current accumulator, add the contents of
        // the accumulator to the list of batches and start a new accumulator
        if !batch_acc.can_accept_op(op) {
            let batch = batch_acc.into_batch();
            batch_acc = OpBatchAccumulator::new();

            batch_groups.push(*batch.groups());
            batches.push(batch);
        }

        // add the operation to the accumulator
        batch_acc.add_op(op);
    }

    // make sure we finished processing the last batch
    if !batch_acc.is_empty() {
        let batch = batch_acc.into_batch();
        batch_groups.push(*batch.groups());
        batches.push(batch);
    }

    // compute the hash of all operation groups
    let op_groups = &flatten_slice_elements(&batch_groups);
    let hash = hasher::hash_elements(op_groups);

    (batches, hash)
}

/// Returns the total number of operation groups in a span defined by the provides list of
/// operation batches.
///
/// Then number of operation groups is computed as follows:
/// - For all batches but the last one we set the number of groups to 8, regardless of the actual
///   number of groups in the batch. The reason for this is that when operation batches are
///   concatenated together each batch contributes 8 elements to the hash.
/// - For the last batch, we take the number of actual batches and round it up to the next power of
///   two. The reason for rounding is that the VM always executes a number of operation groups which
///   is a power of two.
pub fn get_span_op_group_count(op_batches: &[OpBatch]) -> usize {
    let last_batch_num_groups = op_batches.last().expect("no last group").num_groups();
    (op_batches.len() - 1) * BATCH_SIZE + last_batch_num_groups.next_power_of_two()
}
