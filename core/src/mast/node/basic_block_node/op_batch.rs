use alloc::vec::Vec;

use super::{BATCH_SIZE, Felt, OP_GROUP_SIZE, Operation, ZERO};

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
    /// A list of operations in this batch.
    ///
    /// This list is redundant with opcodes stored in the operation groups.
    ops: Vec<Operation>,
    /// The batch's groups, of which there are 2 types:
    /// 1. operation groups, which encode the opcodes of the operations,
    /// 2. immediate values of operations in the preceding operation group.
    groups: Vec<u64>,
    /// Number of operations in each operation group. Operation count for groups with immediate
    /// values is set to 0.
    op_counts: [usize; BATCH_SIZE],
    /// Index in `groups` of the operation group that is currently being filled.
    current_op_group_idx: usize,
    /// Number of operations in the operation group currently being filled.
    current_op_group_size: usize,
    /// True if the last operation in the current operation group has an immediate value.
    last_op_has_imm: bool,
}

impl OpBatchAccumulator {
    /// Returns an empty [OpBatchAccumulator].
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
            last_op_has_imm: false,
        }
    }

    /// Returns true if this accumulator does not contain any operations.
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

    /// Adds the specified operation to this accumulator, and returns a batch if the accumulator was
    /// full prior to adding the operation.
    pub fn add_op(&mut self, op: Operation) -> Option<OpBatch> {
        // If the accumulator is full, we extract batch and reset the accumulator.
        let maybe_batch = if !self.can_accept_op(op) {
            self.extract_batch_and_reset()
        } else {
            None
        };

        self.add_op_impl(op);

        maybe_batch
    }

    /// Converts the accumulator into an [OpBatch] if the accumulator is not empty.
    pub fn into_batch(mut self) -> Option<OpBatch> {
        self.extract_batch_and_reset()
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Returns true if this accumulator can accept the specified operation.
    ///
    /// An accumulator may not be able accept an operation for the following reasons:
    /// - There is no more space in the underlying batch (e.g., the 8th group of the batch already
    ///   contains 9 operations).
    /// - There is no space for the immediate value carried by the operation (e.g., the 8th group is
    ///   only partially full, but we are trying to add a PUSH operation).
    /// - The alignment rules require that the operation overflows into the next group, and if this
    ///   happens, there will be no space for the operation or its immediate value.
    fn can_accept_op(&self, op: Operation) -> bool {
        let num_groups_post_insertion = {
            let num_new_imm_groups: usize = op.imm_value().is_some() as usize;
            let num_new_op_groups: usize = if op.imm_value().is_some() {
                // an operation carrying an immediate value cannot be the last one in a group; so,
                // if it were to be the last one in the group, we need to add it to
                // a new group.
                (self.current_op_group_size >= OP_GROUP_SIZE - 1) as usize
            } else {
                // we need a new op group if the current group is full
                (self.current_op_group_size == OP_GROUP_SIZE) as usize
            };

            self.groups.len() + num_new_imm_groups + num_new_op_groups
        };

        num_groups_post_insertion <= BATCH_SIZE
    }

    /// Adds the specified operation to this accumulator. It is expected that
    /// (can_accept_op())[OpBatchAccumulator::can_accept_op] is called before this function to make
    /// sure that the specified operation can be added to the accumulator.
    fn add_op_impl(&mut self, op: Operation) {
        // if the group is full, finalize it and start a new group
        if self.current_op_group_size == OP_GROUP_SIZE {
            self.finalize_and_start_new_op_group();
        }

        if let Some(imm) = op.imm_value() {
            // since an operation with an immediate value cannot be the last one in a group, if the
            // operation would be the last one in the group, we insert a no-op and start a new group
            if self.current_op_group_size == OP_GROUP_SIZE - 1 {
                self.insert_op_into_current_group(Operation::Noop);
                self.finalize_and_start_new_op_group();
            }

            debug_assert!(
                self.current_op_group_size < OP_GROUP_SIZE - 1,
                "finalize_op_group() did not reset the group size"
            );
            // save the immediate value by appending it as a group in the batch
            self.groups.push(imm.into());
        }

        // add the opcode to the group and increment the op index pointer
        self.insert_op_into_current_group(op);
    }

    /// Returns a new batch with the current operations and groups if the accumulator is not empty,
    /// and resets the accumulator.
    fn extract_batch_and_reset(&mut self) -> Option<OpBatch> {
        if self.is_empty() {
            return None;
        }

        // if the last group ends with an immediate value, we need to insert a NOOP operation.
        if self.last_op_has_imm {
            debug_assert!(self.current_op_group_size < OP_GROUP_SIZE);
            self.insert_op_into_current_group(Operation::Noop);
        }

        // make sure the last group gets added to the group array; we also check the op_idx to
        // handle the case when a group contains a single NOOP operation.
        self.finalize_op_group();

        while self.groups.len() != self.groups.len().next_power_of_two() {
            self.start_new_op_group();
            self.insert_op_into_current_group(Operation::Noop);
            self.finalize_op_group();
        }

        let op_batch = {
            let num_groups = self.groups.len();

            // Convert groups to [Felt; BATCH_SIZE], padding with 0s if necessary
            let groups = {
                let mut padded_groups = [ZERO; BATCH_SIZE];
                for (i, group) in self.groups.iter().enumerate() {
                    padded_groups[i] = Felt::new(*group);
                }
                padded_groups
            };

            let ops = core::mem::take(&mut self.ops);
            let op_counts = self.op_counts;

            OpBatch { ops, groups, op_counts, num_groups }
        };

        *self = Self::new();
        Some(op_batch)
    }

    /// Adds the opcode to the current group and increment the op index pointer
    fn insert_op_into_current_group(&mut self, op: Operation) {
        debug_assert!(
            self.current_op_group_size < OP_GROUP_SIZE,
            "Cannot add operation to a full group"
        );

        let opcode = op.op_code() as u64;
        self.groups[self.current_op_group_idx] |=
            opcode << (Operation::OP_BITS * self.current_op_group_size);
        self.ops.push(op);
        self.current_op_group_size += 1;
        self.last_op_has_imm = op.imm_value().is_some();
    }

    /// Finalizes the current operation group, and starts a new one.
    fn finalize_and_start_new_op_group(&mut self) {
        self.finalize_op_group();
        self.start_new_op_group();
    }

    /// Finalizes the current operation group.
    fn finalize_op_group(&mut self) {
        // Store the size of the current group in the op_counts array and reset the group size
        self.op_counts[self.current_op_group_idx] = self.current_op_group_size;
        self.current_op_group_size = 0;
    }

    /// Starts a new operation group.
    fn start_new_op_group(&mut self) {
        // Start a new group
        let new_op_group = 0;
        self.groups.push(new_op_group);
        self.current_op_group_idx = self.groups.len() - 1;
    }
}
