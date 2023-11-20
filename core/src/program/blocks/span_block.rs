use super::{fmt, hasher, Digest, Felt, Operation, Vec};
use crate::{DecoratorIterator, DecoratorList, ZERO};
use winter_utils::flatten_slice_elements;

// CONSTANTS
// ================================================================================================

/// Maximum number of operations per group.
pub const GROUP_SIZE: usize = 9;

/// Maximum number of groups per batch.
pub const BATCH_SIZE: usize = 8;

/// Maximum number of operations which can fit into a single operation batch.
const MAX_OPS_PER_BATCH: usize = GROUP_SIZE * BATCH_SIZE;

// SPAN BLOCK
// ================================================================================================
/// Block for a linear sequence of operations (i.e., no branching or loops).
///
/// Executes its operations in order. Fails if any of the operations fails.
///
/// A span is composed of operation batches, operation batches are composed of operation groups,
/// operation groups encode the VM's operations and immediate values. These values are created
/// according to these rules:
///
/// - A span contains one or more batches.
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
/// The hash of a span block is:
///
/// > hash(batches, domain=SPAN_DOMAIN)
///
/// Where `batches` is the concatenation of each `batch` in the span, and each batch is 8 field
/// elements (512 bits).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span {
    op_batches: Vec<OpBatch>,
    hash: Digest,
    decorators: DecoratorList,
}

impl Span {
    // CONSTANTS
    // --------------------------------------------------------------------------------------------
    /// The domain of the span block (used for control block hashing).
    pub const DOMAIN: Felt = ZERO;

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
        Self::with_decorators(operations, DecoratorList::new())
    }

    /// Returns a new [Span] block instantiated with the specified operations and decorators.
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

        let (op_batches, hash) = batch_ops(operations);
        Self {
            op_batches,
            hash,
            decorators,
        }
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

    // SPAN MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Returns a new [Span] block instantiated with operations from this block repeated the
    /// specified number of times.
    #[must_use]
    pub fn replicate(&self, num_copies: usize) -> Self {
        let own_ops = self.get_ops();
        let own_decorators = &self.decorators;
        let mut ops = Vec::with_capacity(own_ops.len() * num_copies);
        let mut decorators = DecoratorList::new();

        for i in 0..num_copies {
            // replicate decorators of a span block
            for decorator in own_decorators {
                decorators.push((own_ops.len() * i + decorator.0, decorator.1.clone()))
            }
            ops.extend_from_slice(&own_ops);
        }
        Self::with_decorators(ops, decorators)
    }

    /// Returns a list of decorators in this span block
    pub fn decorators(&self) -> &DecoratorList {
        &self.decorators
    }

    /// Returns a [DecoratorIterator] which allows us to iterate through the decorator list of this span
    /// block while executing operation batches of this span block
    pub fn decorator_iter(&self) -> DecoratorIterator {
        DecoratorIterator::new(&self.decorators)
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Returns a list of operations contained in this span block.
    fn get_ops(&self) -> Vec<Operation> {
        let mut ops = Vec::with_capacity(self.op_batches.len() * MAX_OPS_PER_BATCH);
        for batch in self.op_batches.iter() {
            ops.extend_from_slice(&batch.ops);
        }
        ops
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "span")?;
        for batch in self.op_batches.iter() {
            for op in batch.ops.iter() {
                write!(f, " {op}")?;
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
#[derive(Clone, Debug)]
struct OpBatchAccumulator {
    /// All groups that contain operations. New operations are appended to the last group.
    op_groups: Vec<OpGroup>,
}

impl OpBatchAccumulator {
    pub fn new() -> Self {
        let mut op_groups = Vec::with_capacity(BATCH_SIZE);
        op_groups.push(OpGroup::new());

        Self { op_groups }
    }

    /// Returns true if this accumulator does not contain any operations.
    pub fn is_empty(&self) -> bool {
        self.op_groups.len() == 1 && self.op_groups[0].is_empty()
    }

    /// A batch can accept a new operation if the batch doesn't exceed capacity as a result
    pub fn can_accept_op(&self, op: Operation) -> bool {
        let total_groups_after_accepting_op = {
            // number of op groups after accepting op
            let new_op_groups_len = {
                // if current_group is full, so we'll need another group for the new opcode
                // (0 or 1)
                let new_op_groups_count: usize = self.current_op_group().can_accept_op(op).into();

                self.op_groups.len() + new_op_groups_count
            };

            // current number of immediate values
            let current_num_imm_values: usize =
                self.op_groups.iter().map(OpGroup::num_immediate_values).sum::<usize>();

            // number of immediate values added (0 or 1)
            let num_new_imm_values: usize = op.imm_value().is_some().into();

            new_op_groups_len + current_num_imm_values + num_new_imm_values
        };

        total_groups_after_accepting_op <= BATCH_SIZE
    }

    pub fn add_op(&mut self, op: Operation) {
        if !self.current_op_group().can_accept_op(op) {
            self.op_groups.push(OpGroup::new());
        }

        self.current_op_group_mut().add_op(op);
    }

    // HELPERS
    // ---------------------------------------------------------------------------------------------
    fn current_op_group(&self) -> &OpGroup {
        self.op_groups.last().expect("op_groups is never empty")
    }

    fn current_op_group_mut(&mut self) -> &mut OpGroup {
        self.op_groups.last_mut().expect("op_groups is never empty")
    }
}

impl From<OpBatchAccumulator> for OpBatch {
    fn from(acc: OpBatchAccumulator) -> Self {
        let ops: Vec<Operation> = acc
            .op_groups
            .clone()
            .into_iter()
            .flat_map(|op_group| op_group.operations)
            .collect();

        let (groups, op_counts, num_groups): ([Felt; BATCH_SIZE], [usize; BATCH_SIZE], usize) = {
            let mut batch_groups: Vec<Felt> = Vec::with_capacity(BATCH_SIZE);
            let mut op_counts: Vec<usize> = Vec::with_capacity(BATCH_SIZE);

            for op_group in acc.op_groups {
                let immediate_values =
                    op_group.operations.clone().into_iter().filter_map(|op| op.imm_value());

                let op_count = op_group.operations.len();

                batch_groups.push(op_group.into());
                batch_groups.extend(immediate_values.clone());

                op_counts.push(op_count);
                // All immediate values form a new group which contain no operations
                op_counts.extend(immediate_values.map(|_| 0));
            }

            let num_groups = batch_groups.len();

            // padding
            op_counts.extend((batch_groups.len()..BATCH_SIZE).map(|_| 0));
            batch_groups.extend((batch_groups.len()..BATCH_SIZE).map(|_| ZERO));

            (
                batch_groups.try_into().expect(
                "`OpBatchAccumulator::can_accept_op()` accepted an operation it wasn't supposed to"),
                op_counts.try_into().expect(
                "`OpBatchAccumulator::can_accept_op()` accepted an operation it wasn't supposed to"),
                num_groups
            )
        };

        OpBatch {
            ops,
            groups,
            op_counts,
            num_groups,
        }
    }
}

/// A group that contains operations (i.e. no immediate values)
#[derive(Clone, Debug)]
struct OpGroup {
    operations: Vec<Operation>,
}

impl OpGroup {
    fn new() -> Self {
        Self {
            operations: Vec::with_capacity(GROUP_SIZE),
        }
    }

    fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    fn num_immediate_values(&self) -> usize {
        self.operations.iter().filter_map(|op| op.imm_value()).count()
    }

    fn can_accept_op(&self, op: Operation) -> bool {
        let op_has_imm_value = op.imm_value().is_some();

        if op_has_imm_value {
            // an operation carrying an immediate value cannot be the last one in a group
            self.operations.len() < GROUP_SIZE - 1
        } else {
            self.operations.len() < GROUP_SIZE
        }
    }

    fn add_op(&mut self, op: Operation) {
        self.operations.push(op)
    }
}

impl From<OpGroup> for Felt {
    fn from(op_group: OpGroup) -> Self {
        let mut group: u64 = 0;

        for (op_idx, op) in op_group.operations.into_iter().enumerate() {
            let opcode = op.op_code() as u64;
            group |= opcode << (Operation::OP_BITS * op_idx);
        }

        group.into()
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Groups the provided operations into batches as described in the docs for this module (i.e.,
/// up to 9 operations per group, and 8 groups per batch).
///
/// After the operations have been grouped, computes the hash of the block.
fn batch_ops(ops: Vec<Operation>) -> (Vec<OpBatch>, Digest) {
    let mut batch_acc = OpBatchAccumulator::new();
    let mut batches = Vec::<OpBatch>::new();
    let mut batch_groups = Vec::<[Felt; BATCH_SIZE]>::new();

    for op in ops {
        // if the operation cannot be accepted into the current accumulator, add the contents of
        // the accumulator to the list of batches and start a new accumulator
        if !batch_acc.can_accept_op(op) {
            let batch: OpBatch = batch_acc.into();
            batch_acc = OpBatchAccumulator::new();

            batch_groups.push(*batch.groups());
            batches.push(batch);
        }

        // add the operation to the accumulator
        batch_acc.add_op(op);
    }

    // make sure we finished processing the last batch
    if !batch_acc.is_empty() {
        let batch: OpBatch = batch_acc.into();
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
/// - For all batches but the last one we set the number of groups to 8, regardless of the
///   actual number of groups in the batch. The reason for this is that when operation
///   batches are concatenated together each batch contributes 8 elements to the hash.
/// - For the last batch, we take the number of actual batches and round it up to the next
///   power of two. The reason for rounding is that the VM always executes a number of
///   operation groups which is a power of two.
pub fn get_span_op_group_count(op_batches: &[OpBatch]) -> usize {
    let last_batch_num_groups = op_batches.last().expect("no last group").num_groups();
    (op_batches.len() - 1) * BATCH_SIZE + last_batch_num_groups.next_power_of_two()
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

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{hasher, Felt, Operation, BATCH_SIZE, ZERO};
    use crate::ONE;
    use miden_crypto::hash::rpo::RpoDigest;

    #[test]
    fn batch_ops_zero_operations() {
        let ops = Vec::new();
        let (batches, hash) = super::batch_ops(ops.clone());
        assert_eq!(0, batches.len());

        assert_eq!(RpoDigest::from([ZERO, ZERO, ZERO, ZERO]), hash);
    }

    #[test]
    fn batch_ops_one_operation() {
        let ops = vec![Operation::Add];
        let (batches, hash) = super::batch_ops(ops.clone());
        assert_eq!(1, batches.len());

        let batch = &batches[0];
        assert_eq!(ops, batch.ops);
        assert_eq!(1, batch.num_groups());

        let mut batch_groups = [ZERO; BATCH_SIZE];
        batch_groups[0] = build_group(&ops);

        assert_eq!(batch_groups, batch.groups);
        assert_eq!([1_usize, 0, 0, 0, 0, 0, 0, 0], batch.op_counts);
        assert_eq!(hasher::hash_elements(&batch_groups), hash);
    }

    #[test]
    fn batch_ops_two_operations() {
        let ops = vec![Operation::Add, Operation::Mul];
        let (batches, hash) = super::batch_ops(ops.clone());
        assert_eq!(1, batches.len());

        let batch = &batches[0];
        assert_eq!(ops, batch.ops);
        assert_eq!(1, batch.num_groups());

        let mut batch_groups = [ZERO; BATCH_SIZE];
        batch_groups[0] = build_group(&ops);

        assert_eq!(batch_groups, batch.groups);
        assert_eq!([2_usize, 0, 0, 0, 0, 0, 0, 0], batch.op_counts);
        assert_eq!(hasher::hash_elements(&batch_groups), hash);
    }

    #[test]
    fn batch_ops_one_group_with_imm_value() {
        let ops = vec![Operation::Add, Operation::Push(Felt::new(12345678))];
        let (batches, hash) = super::batch_ops(ops.clone());
        assert_eq!(1, batches.len());

        let batch = &batches[0];
        assert_eq!(ops, batch.ops);
        assert_eq!(2, batch.num_groups());

        let mut batch_groups = [ZERO; BATCH_SIZE];
        batch_groups[0] = build_group(&ops);
        batch_groups[1] = Felt::new(12345678);

        assert_eq!(batch_groups, batch.groups);
        assert_eq!([2_usize, 0, 0, 0, 0, 0, 0, 0], batch.op_counts);
        assert_eq!(hasher::hash_elements(&batch_groups), hash);
    }

    #[test]
    fn batch_ops_one_group_with_7_imm_values() {
        let ops = vec![
            Operation::Push(ONE),
            Operation::Push(Felt::new(2)),
            Operation::Push(Felt::new(3)),
            Operation::Push(Felt::new(4)),
            Operation::Push(Felt::new(5)),
            Operation::Push(Felt::new(6)),
            Operation::Push(Felt::new(7)),
            Operation::Add,
        ];
        let (batches, hash) = super::batch_ops(ops.clone());
        assert_eq!(1, batches.len());

        let batch = &batches[0];
        assert_eq!(ops, batch.ops);
        assert_eq!(8, batch.num_groups());

        let batch_groups = [
            build_group(&ops),
            ONE,
            Felt::new(2),
            Felt::new(3),
            Felt::new(4),
            Felt::new(5),
            Felt::new(6),
            Felt::new(7),
        ];

        assert_eq!(batch_groups, batch.groups);
        assert_eq!([8_usize, 0, 0, 0, 0, 0, 0, 0], batch.op_counts);
        assert_eq!(hasher::hash_elements(&batch_groups), hash);
    }

    #[test]
    fn batch_ops_two_groups_with_7_imm_values_v1() {
        // --- the last push overflows to the second batch. first group full before push ----
        let ops = vec![
            Operation::Add,
            Operation::Mul,
            Operation::Push(ONE),
            Operation::Push(Felt::new(2)),
            Operation::Push(Felt::new(3)),
            Operation::Push(Felt::new(4)),
            Operation::Push(Felt::new(5)),
            Operation::Push(Felt::new(6)),
            Operation::Add,
            Operation::Push(Felt::new(7)),
        ];
        let (batches, hash) = super::batch_ops(ops.clone());
        assert_eq!(2, batches.len());

        let batch0 = &batches[0];
        assert_eq!(ops[..9], batch0.ops);
        assert_eq!(7, batch0.num_groups());

        let batch0_groups = [
            build_group(&ops[..9]),
            ONE,
            Felt::new(2),
            Felt::new(3),
            Felt::new(4),
            Felt::new(5),
            Felt::new(6),
            ZERO,
        ];

        assert_eq!(batch0_groups, batch0.groups);
        assert_eq!([9_usize, 0, 0, 0, 0, 0, 0, 0], batch0.op_counts);

        let batch1 = &batches[1];
        assert_eq!(vec![ops[9]], batch1.ops);
        assert_eq!(2, batch1.num_groups());

        let mut batch1_groups = [ZERO; BATCH_SIZE];
        batch1_groups[0] = build_group(&[ops[9]]);
        batch1_groups[1] = Felt::new(7);

        assert_eq!([1_usize, 0, 0, 0, 0, 0, 0, 0], batch1.op_counts);
        assert_eq!(batch1_groups, batch1.groups);

        let all_groups = [batch0_groups, batch1_groups].concat();
        assert_eq!(hasher::hash_elements(&all_groups), hash);
    }

    #[test]
    fn batch_ops_two_groups_with_7_imm_values_v2() {
        // --- the last push overflows to the second batch; first group NOT full before push ----
        let ops = vec![
            // batch 1
            Operation::Add,
            Operation::Mul,
            Operation::Push(ONE),
            Operation::Push(Felt::new(2)),
            Operation::Push(Felt::new(3)),
            Operation::Push(Felt::new(4)),
            Operation::Push(Felt::new(5)),
            Operation::Push(Felt::new(6)),
            // batch 2
            Operation::Push(Felt::new(7)),
        ];
        let (batches, hash) = super::batch_ops(ops.clone());
        assert_eq!(2, batches.len());

        let batch0 = &batches[0];
        assert_eq!(ops[..8], batch0.ops);
        assert_eq!(7, batch0.num_groups());

        let batch0_groups = [
            build_group(&ops[..8]),
            ONE,
            Felt::new(2),
            Felt::new(3),
            Felt::new(4),
            Felt::new(5),
            Felt::new(6),
            ZERO,
        ];

        assert_eq!(batch0_groups, batch0.groups);
        assert_eq!([8_usize, 0, 0, 0, 0, 0, 0, 0], batch0.op_counts);

        let batch1 = &batches[1];
        assert_eq!(vec![ops[8]], batch1.ops);
        assert_eq!(2, batch1.num_groups());

        let mut batch1_groups = [ZERO; BATCH_SIZE];
        batch1_groups[0] = build_group(&[ops[8]]);
        batch1_groups[1] = Felt::new(7);

        assert_eq!([1_usize, 0, 0, 0, 0, 0, 0, 0], batch1.op_counts);
        assert_eq!(batch1_groups, batch1.groups);

        let all_groups = [batch0_groups, batch1_groups].concat();
        assert_eq!(hasher::hash_elements(&all_groups), hash);
    }

    #[test]
    fn batch_ops_imm_values_in_between_groups() {
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
        assert_eq!(4, batch.num_groups());

        let batch_groups = [
            build_group(&ops[..9]),
            Felt::new(7),
            Felt::new(11),
            build_group(&ops[9..]),
            ZERO,
            ZERO,
            ZERO,
            ZERO,
        ];

        assert_eq!([9_usize, 0, 0, 1, 0, 0, 0, 0], batch.op_counts);
        assert_eq!(batch_groups, batch.groups);
        assert_eq!(hasher::hash_elements(&batch_groups), hash);
    }

    #[test]
    fn batch_ops_push_end_of_group_v1() {
        // --- push at the end of a group is moved into the next group ----------------------------
        let ops = vec![
            Operation::Add,
            Operation::Mul,
            Operation::Add,
            Operation::Add,
            Operation::Add,
            Operation::Mul,
            Operation::Mul,
            Operation::Add,
            Operation::Push(Felt::new(11)),
        ];
        let (batches, hash) = super::batch_ops(ops.clone());
        assert_eq!(1, batches.len());

        let batch = &batches[0];
        assert_eq!(ops, batch.ops);
        assert_eq!(3, batch.num_groups());

        let batch_groups = [
            build_group(&ops[..8]),
            build_group(&[ops[8]]),
            Felt::new(11),
            ZERO,
            ZERO,
            ZERO,
            ZERO,
            ZERO,
        ];

        assert_eq!(batch_groups, batch.groups);
        assert_eq!([8_usize, 1, 0, 0, 0, 0, 0, 0], batch.op_counts);
        assert_eq!(hasher::hash_elements(&batch_groups), hash);
    }

    #[test]
    fn batch_ops_push_end_of_group_v2() {
        // --- push at the end of a group is moved into the next group ----------------------------
        let ops = vec![
            Operation::Add,
            Operation::Mul,
            Operation::Add,
            Operation::Add,
            Operation::Add,
            Operation::Mul,
            Operation::Mul,
            Operation::Push(ONE),
            Operation::Push(Felt::new(2)),
        ];
        let (batches, hash) = super::batch_ops(ops.clone());
        assert_eq!(1, batches.len());

        let batch = &batches[0];
        assert_eq!(ops, batch.ops);
        assert_eq!(4, batch.num_groups());

        let batch_groups = [
            build_group(&ops[..8]),
            ONE,
            build_group(&[ops[8]]),
            Felt::new(2),
            ZERO,
            ZERO,
            ZERO,
            ZERO,
        ];

        assert_eq!(batch_groups, batch.groups);
        assert_eq!([8_usize, 0, 1, 0, 0, 0, 0, 0], batch.op_counts);
        assert_eq!(hasher::hash_elements(&batch_groups), hash);
    }

    #[test]
    fn batch_ops_push_end_of_7th_group() {
        // --- push at the end of the 7th group overflows to the next batch -----------------------
        let ops = vec![
            Operation::Add,
            Operation::Mul,
            Operation::Push(ONE),
            Operation::Push(Felt::new(2)),
            Operation::Push(Felt::new(3)),
            Operation::Push(Felt::new(4)),
            Operation::Push(Felt::new(5)),
            Operation::Add,
            Operation::Mul,
            Operation::Add,
            Operation::Mul,
            Operation::Add,
            Operation::Mul,
            Operation::Add,
            Operation::Mul,
            Operation::Add,
            Operation::Mul,
            Operation::Push(Felt::new(6)),
            Operation::Pad,
        ];

        let (batches, hash) = super::batch_ops(ops.clone());
        assert_eq!(2, batches.len());

        let batch0 = &batches[0];
        assert_eq!(ops[..17], batch0.ops);
        assert_eq!(7, batch0.num_groups());

        let batch0_groups = [
            build_group(&ops[..9]),
            ONE,
            Felt::new(2),
            Felt::new(3),
            Felt::new(4),
            Felt::new(5),
            build_group(&ops[9..17]),
            ZERO,
        ];

        assert_eq!(batch0_groups, batch0.groups);
        assert_eq!([9_usize, 0, 0, 0, 0, 0, 8, 0], batch0.op_counts);

        let batch1 = &batches[1];
        assert_eq!(ops[17..], batch1.ops);
        assert_eq!(2, batch1.num_groups());

        let batch1_groups =
            [build_group(&ops[17..]), Felt::new(6), ZERO, ZERO, ZERO, ZERO, ZERO, ZERO];
        assert_eq!(batch1_groups, batch1.groups);
        assert_eq!([2_usize, 0, 0, 0, 0, 0, 0, 0], batch1.op_counts);

        let all_groups = [batch0_groups, batch1_groups].concat();
        assert_eq!(hasher::hash_elements(&all_groups), hash);
    }

    // TEST HELPERS
    // --------------------------------------------------------------------------------------------

    fn build_group(ops: &[Operation]) -> Felt {
        let mut group = 0u64;
        for (i, op) in ops.iter().enumerate() {
            group |= (op.op_code() as u64) << (Operation::OP_BITS * i);
        }
        Felt::new(group)
    }
}
