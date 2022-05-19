use super::{fmt, hasher, Digest, Felt, FieldElement, Operation};
use winter_utils::flatten_slice_elements;

// CONSTANTS
// ================================================================================================

/// Maximum number of operations per group.
const GROUP_SIZE: usize = 9;

/// Maximum number of groups per batch.
const BATCH_SIZE: usize = 8;

/// Maximum number of operations which can fit into a single operation batch.
const MAX_OPS_PER_BATCH: usize = GROUP_SIZE * BATCH_SIZE;

// SPAN BLOCK
// ================================================================================================
/// A code block used to describe a linear sequence of operations (i.e., no branching or loops).
///
/// When the VM executes a Span block, it breaks the sequence of operations into batches and
/// groups according to the following rules:
/// - A group may contain up to 9 operations or a single immediate value.
/// - A batch may contain up to 8 groups.
/// - There is no limit on the number of batches contained within a single span.
///
/// Thus, for example, executing 8 pushes in a row will result in two operation batches:
/// - The first batch will contain 8 groups, with the first group containing 7 push opcodes,
///   and the remaining 7 groups containing immediate values for each of the push operations.
/// - The second batch will contain 2 groups, with the first group containing a single push opcode,
///   and the second group containing the immediate value for the last push operation.
///
/// If a sequence of operations does not have any operations which carry immediate values, then
/// up to 72 operations can fit into a single batch.
///
/// From the user's perspective, all operations are executed in order, however, the assembler may
/// insert NOOPs to ensure proper alignment of all operations in the sequence.
///
/// TODO: describe how Span hash is computed.
#[derive(Clone, Debug)]
pub struct Span {
    op_batches: Vec<OpBatch>,
    hash: Digest,
}

impl Span {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [Span] block instantiated with the specified operations.
    ///
    /// # Errors (TODO)
    /// Returns an error if:
    /// - `operations` vector is empty.
    /// - `operations` vector contains any number of system operations.
    pub fn new(operations: Vec<Operation>) -> Self {
        assert!(!operations.is_empty()); // TODO: return error
        let (op_batches, hash) = batch_ops(operations);
        Self { op_batches, hash }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a hash of this code block.
    pub fn hash(&self) -> Digest {
        self.hash
    }

    /// Returns list of operation batches contained in this span block.
    pub fn op_batches(&self) -> &[OpBatch] {
        &self.op_batches
    }

    /// Returns a list of operations contained in this span block.
    pub fn get_ops(&self) -> Vec<Operation> {
        let mut ops = Vec::with_capacity(self.op_batches.len() * MAX_OPS_PER_BATCH);
        for batch in self.op_batches.iter() {
            ops.extend_from_slice(&batch.ops);
        }
        ops
    }

    // SPAN MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Returns a new [Span] block instantiated with operations from this block repeated the
    /// specified number of times.
    #[must_use]
    pub fn replicate(&self, num_copies: usize) -> Self {
        let own_ops = self.get_ops();
        let mut ops = Vec::with_capacity(own_ops.len() * num_copies);
        for _ in 0..num_copies {
            ops.extend_from_slice(&own_ops);
        }
        Self::new(ops)
    }

    /// Appends the operations from the provided [Span] to this [Span].
    pub fn append(&mut self, other: Self) {
        let mut ops = self.get_ops();
        for batch in other.op_batches {
            ops.extend_from_slice(&batch.ops);
        }
        let (op_batches, hash) = batch_ops(ops);
        self.op_batches = op_batches;
        self.hash = hash;
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "span")?;
        for batch in self.op_batches.iter() {
            for op in batch.ops.iter() {
                write!(f, " {}", op)?;
            }
        }
        write!(f, " end")
    }
}

// OPERATION BATCH
// ================================================================================================

/// A batch of operations in a [Span] block.
///
/// An operation batch consists of up to 8 operation groups, with each group containing up to 9
/// operations or a single immediate value.
#[derive(Clone, Debug)]
pub struct OpBatch {
    ops: Vec<Operation>,
    groups: [Felt; BATCH_SIZE],
}

impl OpBatch {
    /// Returns a list of operations contained in this batch.
    pub fn ops(&self) -> &[Operation] {
        &self.ops
    }

    /// Returns a list of operation groups contained in this batch.
    ///
    /// Each group is represented by a single field element.
    pub fn groups(&self) -> [Felt; BATCH_SIZE] {
        self.groups
    }
}

/// An accumulator used in construction of operation batches.
struct OpBatchAccumulator {
    ops: Vec<Operation>,
    groups: [Felt; BATCH_SIZE],
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
            groups: [Felt::ZERO; BATCH_SIZE],
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
    pub fn can_accept_op(&self, op: Operation) -> bool {
        // if the current group is full and the batch is full, return false
        let next_group_idx = if self.op_idx == GROUP_SIZE {
            if self.next_group_idx >= BATCH_SIZE {
                return false;
            }
            self.next_group_idx + 1
        } else {
            self.next_group_idx
        };

        // if the operation carries an immediate value, there must be enough room for it
        !(op.imm_value().is_some() && next_group_idx >= BATCH_SIZE)
    }

    /// Adds the specified operation to this accumulator. It is expected that the specified
    /// operation is not a decorator and that (can_accept_op())[OpBatchAccumulator::can_accept_op]
    /// is called before this function to make sure that the specified operation can be added to
    /// the accumulator.
    pub fn add_op(&mut self, op: Operation) {
        debug_assert!(!op.is_decorator(), "must not be a decorator");

        // if the current op group is full, save it to the list of op groups, advance current
        // and next group pointers, and reset the group content.
        if self.op_idx == GROUP_SIZE {
            self.groups[self.group_idx] = Felt::new(self.group);
            self.group_idx = self.next_group_idx;
            self.next_group_idx = self.group_idx + 1;

            self.op_idx = 0;
            self.group = 0;
        }

        // if the operation contains immediate value, save it at the next group pointer
        if let Some(imm) = op.imm_value() {
            self.groups[self.next_group_idx] = imm;
            self.next_group_idx += 1;

            // operation carrying an immediate value cannot be the first one in the group except
            // for the first group of a batch; so, we just add a noop in front of it
            if self.op_idx == 0 && self.group_idx != 0 {
                self.ops.push(Operation::Noop);
                self.op_idx += 1;
            }
        }

        // add the opcode to the group
        let opcode = op.op_code().expect("no opcode") as u64;
        self.group |= opcode << (Operation::OP_BITS * self.op_idx);
        self.ops.push(op);
        self.op_idx += 1;
    }

    /// Adds the specified operation to the list of operations without accumulating the operation
    /// into op groups. It is expected that the operation is a decorator.
    pub fn add_decorator(&mut self, op: Operation) {
        debug_assert!(op.is_decorator(), "must be a decorator");
        self.ops.push(op);
    }

    /// Convert the accumulator into an [OpBatch].
    pub fn into_batch(mut self) -> OpBatch {
        // make sure the last group gets added to the group array
        if self.group != 0 {
            self.groups[self.group_idx] = Felt::new(self.group);
        }

        OpBatch {
            ops: self.ops,
            groups: self.groups,
        }
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn batch_ops(ops: Vec<Operation>) -> (Vec<OpBatch>, Digest) {
    let mut batch_acc = OpBatchAccumulator::new();
    let mut batches = Vec::<OpBatch>::new();
    let mut batch_groups = Vec::<[Felt; BATCH_SIZE]>::new();

    for op in ops {
        // if the operator is a decorator we add it to the accumulator as a decorator, but don't
        // process it further (i.e., the operation will not affect program hash).
        if op.is_decorator() {
            batch_acc.add_decorator(op);
            continue;
        }

        // if the operation cannot be accepted into the current accumulator, add the contents of
        // the accumulator to the list of batches and start a new accumulator
        if !batch_acc.can_accept_op(op) {
            let batch = batch_acc.into_batch();
            batch_acc = OpBatchAccumulator::new();

            batch_groups.push(batch.groups());
            batches.push(batch);
        }

        // add the operation to the accumulator
        batch_acc.add_op(op);
    }

    // make sure we finished processing the last batch
    if !batch_acc.is_empty() {
        let batch = batch_acc.into_batch();
        batch_groups.push(batch.groups());
        batches.push(batch);
    }

    let hash = hasher::hash_elements(flatten_slice_elements(&batch_groups));

    (batches, hash)
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{hasher, Felt, FieldElement, Operation, BATCH_SIZE};
    use crate::DebugOptions;

    #[test]
    fn batch_ops() {
        // one operation
        let ops = vec![Operation::Add];
        let (batches, hash) = super::batch_ops(ops.clone());
        assert_eq!(1, batches.len());
        let batch = &batches[0];
        assert_eq!(ops, batch.ops);
        let mut batch_groups = [Felt::ZERO; BATCH_SIZE];
        batch_groups[0] = build_group(&ops);
        assert_eq!(batch_groups, batch.groups);
        assert_eq!(hasher::hash_elements(&batch_groups), hash);

        // two operations
        let ops = vec![Operation::Add, Operation::Mul];
        let (batches, hash) = super::batch_ops(ops.clone());
        assert_eq!(1, batches.len());
        let batch = &batches[0];
        assert_eq!(ops, batch.ops);
        let mut batch_groups = [Felt::ZERO; BATCH_SIZE];
        batch_groups[0] = build_group(&ops);
        assert_eq!(batch_groups, batch.groups);
        assert_eq!(hasher::hash_elements(&batch_groups), hash);

        // one group with one immediate value
        let ops = vec![Operation::Add, Operation::Push(Felt::new(12345678))];
        let (batches, hash) = super::batch_ops(ops.clone());
        assert_eq!(1, batches.len());
        let batch = &batches[0];
        assert_eq!(ops, batch.ops);
        let mut batch_groups = [Felt::ZERO; BATCH_SIZE];
        batch_groups[0] = build_group(&ops);
        batch_groups[1] = Felt::new(12345678);
        assert_eq!(batch_groups, batch.groups);
        assert_eq!(hasher::hash_elements(&batch_groups), hash);

        // one group with 7 immediate values
        let ops = vec![
            Operation::Add,
            Operation::Push(Felt::new(1)),
            Operation::Push(Felt::new(2)),
            Operation::Push(Felt::new(3)),
            Operation::Push(Felt::new(4)),
            Operation::Push(Felt::new(5)),
            Operation::Push(Felt::new(6)),
            Operation::Push(Felt::new(7)),
        ];
        let (batches, hash) = super::batch_ops(ops.clone());
        assert_eq!(1, batches.len());
        let batch = &batches[0];
        assert_eq!(ops, batch.ops);
        let batch_groups = [
            build_group(&ops),
            Felt::new(1),
            Felt::new(2),
            Felt::new(3),
            Felt::new(4),
            Felt::new(5),
            Felt::new(6),
            Felt::new(7),
        ];
        assert_eq!(batch_groups, batch.groups);
        assert_eq!(hasher::hash_elements(&batch_groups), hash);

        // two groups with 7 immediate values; the last push overflows to the second batch
        let ops = vec![
            Operation::Add,
            Operation::Mul,
            Operation::Add,
            Operation::Push(Felt::new(1)),
            Operation::Push(Felt::new(2)),
            Operation::Push(Felt::new(3)),
            Operation::Push(Felt::new(4)),
            Operation::Push(Felt::new(5)),
            Operation::Push(Felt::new(6)),
            Operation::Push(Felt::new(7)),
        ];
        let (batches, hash) = super::batch_ops(ops.clone());
        assert_eq!(2, batches.len());
        let batch0 = &batches[0];
        assert_eq!(ops[..9], batch0.ops);

        let batch0_groups = [
            build_group(&ops[..9]),
            Felt::new(1),
            Felt::new(2),
            Felt::new(3),
            Felt::new(4),
            Felt::new(5),
            Felt::new(6),
            Felt::ZERO,
        ];
        assert_eq!(batch0_groups, batch0.groups);

        let batch1 = &batches[1];
        assert_eq!(vec![ops[9]], batch1.ops);
        let mut batch1_groups = [Felt::ZERO; BATCH_SIZE];
        batch1_groups[0] = build_group(&[ops[9]]);
        batch1_groups[1] = Felt::new(7);
        assert_eq!(batch1_groups, batch1.groups);

        let all_groups = [batch0_groups, batch1_groups].concat();
        assert_eq!(hasher::hash_elements(&all_groups), hash);

        // immediate values in-between groups
        let ops = vec![
            Operation::Add,
            Operation::Mul,
            Operation::Add,
            Operation::Push(Felt::new(7)),
            Operation::Add,
            Operation::Add,
            Operation::Push(Felt::new(11)),
            Operation::Mul,
            Operation::Mul,
            Operation::Add,
        ];

        let (batches, hash) = super::batch_ops(ops.clone());
        assert_eq!(1, batches.len());
        let batch = &batches[0];
        assert_eq!(ops, batch.ops);

        let batch_groups = [
            build_group(&ops[..9]),
            Felt::new(7),
            Felt::new(11),
            build_group(&ops[9..]),
            Felt::ZERO,
            Felt::ZERO,
            Felt::ZERO,
            Felt::ZERO,
        ];
        assert_eq!(batch_groups, batch.groups);
        assert_eq!(hasher::hash_elements(&batch_groups), hash);

        // push at start of second group; assembler inserts a NOOP in front of PUSH
        let ops = vec![
            Operation::Add,
            Operation::Mul,
            Operation::Add,
            Operation::Add,
            Operation::Add,
            Operation::Mul,
            Operation::Mul,
            Operation::Add,
            Operation::Add,
            Operation::Push(Felt::new(11)),
        ];
        let (batches, hash) = super::batch_ops(ops.clone());
        assert_eq!(1, batches.len());
        let batch = &batches[0];
        let mut expected_ops = ops.clone();
        expected_ops.insert(9, Operation::Noop);
        assert_eq!(expected_ops, batch.ops);
        let batch_groups = [
            build_group(&ops[..9]),
            build_group(&[Operation::Noop, ops[9]]),
            Felt::new(11),
            Felt::ZERO,
            Felt::ZERO,
            Felt::ZERO,
            Felt::ZERO,
            Felt::ZERO,
        ];
        assert_eq!(batch_groups, batch.groups);
        assert_eq!(hasher::hash_elements(&batch_groups), hash);
    }

    #[test]
    fn batch_ops_with_decorator() {
        let ops = vec![
            Operation::Push(Felt::ONE),
            Operation::Add,
            Operation::Debug(DebugOptions::All),
            Operation::Mul,
        ];
        let (batches, hash) = super::batch_ops(ops.clone());
        assert_eq!(1, batches.len());
        let batch = &batches[0];
        assert_eq!(ops, batch.ops);
        let mut batch_groups = [Felt::ZERO; BATCH_SIZE];
        batch_groups[0] = build_group(&ops);
        batch_groups[1] = Felt::ONE;
        assert_eq!(batch_groups, batch.groups);
        assert_eq!(hasher::hash_elements(&batch_groups), hash);
    }

    // TEST HELPERS
    // --------------------------------------------------------------------------------------------

    fn build_group(ops: &[Operation]) -> Felt {
        let mut group = 0u64;
        let mut i = 0;
        for op in ops.iter() {
            if !op.is_decorator() {
                group |= (op.op_code().unwrap() as u64) << (Operation::OP_BITS * i);
                i += 1;
            }
        }
        Felt::new(group)
    }
}
