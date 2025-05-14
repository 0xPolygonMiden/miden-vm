use alloc::vec::Vec;

use super::{BATCH_SIZE, Felt, GROUP_SIZE, Operation, ZERO};

// OPERATION BATCH
// ================================================================================================

/// A batch of operations in a span block.
///
/// An operation batch consists of up to 8 operation groups, with each group containing up to 9
/// operations or a single immediate value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpBatch {
    pub(super) ops: Vec<Operation>,
    pub(super) groups: [Felt; BATCH_SIZE],
    pub(super) op_counts: [usize; BATCH_SIZE],
    pub(super) num_groups: usize,
}

impl OpBatch {
    /// Returns a list of operations contained in this batch.
    ///
    /// Note: the processor will insert NOOP operations to fill out the groups, so the true number
    /// of operations in the batch may be larger than the number of operations reported by this
    /// method.
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

// OPERATION BATCH ACCUMULATOR
// ================================================================================================

/// An accumulator used in construction of operation batches.
pub(super) struct OpBatchAccumulator {
    /// A list of operations in this batch, including decorators.
    ops: Vec<Operation>,
    /// Values of operation groups, including immediate values.
    groups: Vec<u64>,
    /// Number of non-decorator operations in each operation group. Operation count for groups
    /// with immediate values is set to 0.
    op_counts: [usize; BATCH_SIZE],
    /// Index of the op group that is currently being filled.
    current_op_group_idx: usize,
    /// Index of the next opcode in the current group.
    current_op_group_size: usize,
}

impl OpBatchAccumulator {
    /// Returns a blank [OpBatchAccumulator].
    pub fn new() -> Self {
        let groups = {
            let current_op_group = 0;
            vec![current_op_group]
        };

        Self {
            ops: Vec::new(),
            groups,
            op_counts: [0; BATCH_SIZE],
            current_op_group_idx: 0,
            current_op_group_size: 0,
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
            if self.current_op_group_size < GROUP_SIZE - 1 {
                self.groups.len() < BATCH_SIZE
            } else {
                self.groups.len() + 1 < BATCH_SIZE
            }
        } else {
            // check if there is space for the operation in the current group, or if there isn't,
            // whether we can add another group
            self.current_op_group_size < GROUP_SIZE || self.groups.len() < BATCH_SIZE
        }
    }

    /// Adds the specified operation to this accumulator. It is expected that the specified
    /// operation is not a decorator and that (can_accept_op())[OpBatchAccumulator::can_accept_op]
    /// is called before this function to make sure that the specified operation can be added to
    /// the accumulator.
    pub fn add_op(&mut self, op: Operation) {
        // if the group is full, finalize it and start a new group
        if self.current_op_group_size == GROUP_SIZE {
            self.finalize_op_group();
        }

        // for operations with immediate values, we need to do a few more things
        if let Some(imm) = op.imm_value() {
            // since an operation with an immediate value cannot be the last one in a group, if
            // the operation would be the last one in the group, we need to start a new group
            if self.current_op_group_size == GROUP_SIZE - 1 {
                self.finalize_op_group();
            }

            // save the immediate value by appending it as a group in the batch
            self.groups.push(imm.into());
        }

        // add the opcode to the group and increment the op index pointer
        let opcode = op.op_code() as u64;
        self.groups[self.current_op_group_idx] |=
            opcode << (Operation::OP_BITS * self.current_op_group_size);
        self.ops.push(op);
        self.current_op_group_size += 1;
    }

    /// Convert the accumulator into an [OpBatch].
    pub fn into_batch(mut self) -> OpBatch {
        // make sure the last group gets added to the group array; we also check the op_idx to
        // handle the case when a group contains a single NOOP operation.
        if self.current_op_group_idx != 0 || self.current_op_group_size != 0 {
            self.op_counts[self.current_op_group_idx] = self.current_op_group_size;
        }

        let num_groups = self.groups.len();

        // Convert groups to [Felt; BATCH_SIZE], padding with 0s if necessary
        let groups = {
            let mut padded_groups = [ZERO; BATCH_SIZE];
            for (i, group) in self.groups.into_iter().enumerate() {
                padded_groups[i] = Felt::new(group);
            }
            padded_groups
        };

        OpBatch {
            ops: self.ops,
            groups,
            op_counts: self.op_counts,
            num_groups,
        }
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Finalizes the current operation group and starts a new one.
    pub(super) fn finalize_op_group(&mut self) {
        // Store the size of the current group in the op_counts array and reset the group size
        self.op_counts[self.current_op_group_idx] = self.current_op_group_size;
        self.current_op_group_size = 0;

        // Start a new group
        let new_op_group = 0;
        self.groups.push(new_op_group);
        self.current_op_group_idx = self.groups.len() - 1;
    }
}
