use super::{fmt, Digest, ElementHasher, Operation, Rp62_248};
use math::{fields::f62::BaseElement, FieldElement};
use winter_utils::flatten_slice_elements;

// CONSTANTS
// ================================================================================================

/// Maximum number of operations per group.
const GROUP_SIZE: usize = 9;

/// Maximum number of groups per batch.
const BATCH_SIZE: usize = 8;

// SPAN BLOCK
// ================================================================================================
/// A code block used to describe a linear sequence of operations.
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
/// From the user's perspective, all operations are executed in order, however, the VM may insert
/// NOOPs to ensure proper alignment of all operations in the sequence.
///
/// TODO: describe how Span hash is computed.
#[derive(Clone, Debug)]
pub struct Span {
    ops: Vec<Operation>,
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
        let op_batches = batch_ops(&operations);
        let hash = Rp62_248::hash_elements(flatten_slice_elements(&op_batches));
        Self {
            ops: operations,
            hash,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a hash of this code block.
    pub fn hash(&self) -> Digest {
        self.hash
    }

    /// Returns a new [Span] block instantiated with operations from this block repeated the
    /// specified number of times.
    pub fn replicate(&self, num_copies: usize) -> Self {
        let mut ops = Vec::with_capacity(self.ops.len() * num_copies);
        for _ in 0..num_copies {
            ops.extend_from_slice(&self.ops);
        }
        Self::new(ops)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "span {}", self.ops[0])?;
        for op in self.ops.iter().skip(1) {
            write!(f, " {}", op)?;
        }
        write!(f, " end")
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn batch_ops(ops: &[Operation]) -> Vec<[BaseElement; BATCH_SIZE]> {
    // encodes a group of opcodes; up to 7 opcodes can fit into one group
    let mut op_group = 0u64;

    // index of the next opcode in the current group; valued range of values is [0, 8)
    let mut op_idx = 0;

    // indexes of the current group in the batch, and the next free group in the batch; these
    // could be different by more than 1 if some operations in the batch carry immediate values.
    let mut group_idx = 0;
    let mut next_group_idx = 1;

    // current batch of operations; each batch can contain a combination of op groups and immediate
    // values; total number of slots in a batch is 8.
    let mut batch = [BaseElement::ZERO; BATCH_SIZE];
    let mut batches = Vec::<[BaseElement; BATCH_SIZE]>::new();

    for op in ops {
        // if the operation carries immediate value, process it first
        if let Some(imm) = op.imm_value() {
            // operation carrying an immediate value cannot be the first one in the group;
            // so, we just add a noop in front of it
            if op_idx == 0 {
                op_group = Operation::Noop.op_code() as u64;
                op_idx += 1;
            }

            // check if the batch has room for the immediate value, and if not start another batch
            if next_group_idx == BATCH_SIZE {
                batches.push(batch);
                batch = [BaseElement::ZERO; BATCH_SIZE];
                op_group = 0;
                group_idx = 0;
                next_group_idx = 1;
                op_idx = 0;
            }

            // put the immediate value into the next available slot
            batch[next_group_idx] = imm;
            next_group_idx += 1;
        }

        // add the opcode to the group
        op_group |= (op.op_code() as u64) << (Operation::OP_BITS * op_idx);
        op_idx += 1;

        // if the group is full, put it into the batch and start another group
        if op_idx == GROUP_SIZE {
            batch[group_idx] = BaseElement::new(op_group);
            op_idx = 0;
            op_group = 0;
            group_idx = next_group_idx;
            next_group_idx += 1;

            // if the batch is full, start another batch
            if next_group_idx == BATCH_SIZE {
                batches.push(batch);
                batch = [BaseElement::ZERO; BATCH_SIZE];
                group_idx = 0;
                next_group_idx = 1;
            }
        }
    }

    // make sure we finished processing the last batch
    if group_idx != 0 || op_group != 0 {
        if op_group != 0 {
            batch[group_idx] = BaseElement::new(op_group);
        }
        batches.push(batch);
    }

    batches
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{BaseElement, FieldElement, Operation, GROUP_SIZE};

    #[test]
    fn batch_ops() {
        // one operation
        let ops = [Operation::Add];
        let batches = super::batch_ops(&ops);
        assert_eq!(1, batches.len());
        assert_eq!(BaseElement::from(Operation::Add.op_code()), batches[0][0]);
        for &group in batches[0].iter().skip(1) {
            assert_eq!(BaseElement::ZERO, group);
        }

        // two operations
        let ops = [Operation::Add, Operation::Mul];
        let batches = super::batch_ops(&ops);
        assert_eq!(1, batches.len());
        assert_eq!(build_group(&ops), batches[0][0]);
        for &group in batches[0].iter().skip(1) {
            assert_eq!(BaseElement::ZERO, group);
        }

        // one group with one immediate value
        let ops = [Operation::Add, Operation::Push(BaseElement::new(12345678))];
        let batches = super::batch_ops(&ops);
        assert_eq!(1, batches.len());
        let batch = batches[0];
        assert_eq!(build_group(&ops), batch[0]);
        assert_eq!(BaseElement::new(12345678), batch[1]);
        for &group in batch.iter().skip(2) {
            assert_eq!(BaseElement::ZERO, group);
        }

        // one group with 7 immediate values
        let ops = [
            Operation::Add,
            Operation::Push(BaseElement::new(1)),
            Operation::Push(BaseElement::new(2)),
            Operation::Push(BaseElement::new(3)),
            Operation::Push(BaseElement::new(4)),
            Operation::Push(BaseElement::new(5)),
            Operation::Push(BaseElement::new(6)),
            Operation::Push(BaseElement::new(7)),
        ];
        let batches = super::batch_ops(&ops);
        assert_eq!(1, batches.len());
        let batch = batches[0];
        assert_eq!(build_group(&ops), batch[0]);
        for (i, &group) in batch.iter().enumerate().skip(1) {
            assert_eq!(BaseElement::new(i as u64), group);
        }

        // two groups with 7 immediate values; the last push overflows to the second batch
        let ops = [
            Operation::Add,
            Operation::Mul,
            Operation::Add,
            Operation::Push(BaseElement::new(1)),
            Operation::Push(BaseElement::new(2)),
            Operation::Push(BaseElement::new(3)),
            Operation::Push(BaseElement::new(4)),
            Operation::Push(BaseElement::new(5)),
            Operation::Push(BaseElement::new(6)),
            Operation::Push(BaseElement::new(7)),
        ];
        let batches = super::batch_ops(&ops);
        assert_eq!(2, batches.len());
        let batch0 = batches[0];
        assert_eq!(build_group(&ops[..9]), batch0[0]);
        for (i, &group) in batch0.iter().enumerate().skip(1).take(6) {
            assert_eq!(BaseElement::new(i as u64), group);
        }
        assert_eq!(BaseElement::ZERO, batch0[7]);

        let batch1 = batches[1];
        assert_eq!(build_group(&[Operation::Noop, ops[9]]), batch1[0]);
        assert_eq!(BaseElement::new(7), batch1[1]);
        for &group in batch1.iter().skip(2) {
            assert_eq!(BaseElement::ZERO, group);
        }

        // immediate values in-between groups
        let ops = [
            Operation::Add,
            Operation::Mul,
            Operation::Add,
            Operation::Push(BaseElement::new(7)),
            Operation::Add,
            Operation::Add,
            Operation::Push(BaseElement::new(11)),
            Operation::Mul,
            Operation::Mul,
            Operation::Add,
        ];

        let batches = super::batch_ops(&ops);
        assert_eq!(1, batches.len());
        let batch = batches[0];
        assert_eq!(build_group(&ops[..9]), batch[0]);
        assert_eq!(BaseElement::new(7), batch[1]);
        assert_eq!(BaseElement::new(11), batch[2]);
        assert_eq!(build_group(&ops[9..]), batch[3]);
    }

    // TEST HELPERS
    // --------------------------------------------------------------------------------------------

    fn build_group(ops: &[Operation]) -> BaseElement {
        assert!(ops.len() <= GROUP_SIZE);
        let mut group = 0u64;
        for (i, op) in ops.iter().enumerate() {
            group |= (op.op_code() as u64) << (Operation::OP_BITS * i);
        }
        BaseElement::new(group)
    }
}
