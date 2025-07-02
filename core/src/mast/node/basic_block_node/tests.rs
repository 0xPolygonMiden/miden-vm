use super::*;
use crate::ONE;

#[test]
fn batch_ops() {
    // --- one operation ----------------------------------------------------------------------
    let ops = vec![Operation::Add];
    let (batches, hash) = super::batch_and_hash_ops(ops.clone());
    assert_eq!(1, batches.len());

    let batch = &batches[0];
    assert_eq!(ops, batch.ops);
    assert_eq!(1, batch.num_groups());

    let mut batch_groups = [ZERO; BATCH_SIZE];
    batch_groups[0] = build_group(&ops);

    assert_eq!(batch_groups, batch.groups);
    assert_eq!([1_usize, 0, 0, 0, 0, 0, 0, 0], batch.op_counts);
    assert_eq!(hasher::hash_elements(&batch_groups), hash);

    // --- two operations ---------------------------------------------------------------------
    let ops = vec![Operation::Add, Operation::Mul];
    let (batches, hash) = super::batch_and_hash_ops(ops.clone());
    assert_eq!(1, batches.len());

    let batch = &batches[0];
    assert_eq!(ops, batch.ops);
    assert_eq!(1, batch.num_groups());

    let mut batch_groups = [ZERO; BATCH_SIZE];
    batch_groups[0] = build_group(&ops);

    assert_eq!(batch_groups, batch.groups);
    assert_eq!([2_usize, 0, 0, 0, 0, 0, 0, 0], batch.op_counts);
    assert_eq!(hasher::hash_elements(&batch_groups), hash);

    // --- one group with one immediate value -------------------------------------------------
    let ops = vec![Operation::Add, Operation::Push(Felt::new(12345678))];
    let (batches, hash) = super::batch_and_hash_ops(ops.clone());
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

    // --- one group with 7 immediate values --------------------------------------------------
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
    let (batches, hash) = super::batch_and_hash_ops(ops.clone());
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

    // --- two groups with 7 immediate values; the last push overflows to the second batch ----
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
    let (batches, hash) = super::batch_and_hash_ops(ops.clone());
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

    // --- immediate values in-between groups -------------------------------------------------
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

    let (batches, hash) = super::batch_and_hash_ops(ops.clone());
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
    let (batches, hash) = super::batch_and_hash_ops(ops.clone());
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
    let (batches, hash) = super::batch_and_hash_ops(ops.clone());
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

    let (batches, hash) = super::batch_and_hash_ops(ops.clone());
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

    let batch1_groups = [build_group(&ops[17..]), Felt::new(6), ZERO, ZERO, ZERO, ZERO, ZERO, ZERO];
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
