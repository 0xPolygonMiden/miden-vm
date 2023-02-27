use super::{
    build_op_group, AuxTraceHints, BlockHashTableRow, BlockStackTableRow, BlockTableUpdate,
    OpGroupTableRow, OpGroupTableUpdate,
};
use crate::{
    decoder::block_stack::ExecutionContextInfo, utils::get_trace_len, ExecutionTrace, Felt, Kernel,
    MemAdviceProvider, Operation, Process, StackInputs, Word,
};
use rand_utils::rand_value;
use vm_core::{
    code_blocks::{CodeBlock, Span, OP_BATCH_SIZE},
    decoder::{
        ADDR_COL_IDX, GROUP_COUNT_COL_IDX, HASHER_STATE_RANGE, IN_SPAN_COL_IDX, NUM_HASHER_COLUMNS,
        NUM_OP_BATCH_FLAGS, NUM_OP_BITS, OP_BATCH_1_GROUPS, OP_BATCH_2_GROUPS, OP_BATCH_4_GROUPS,
        OP_BATCH_8_GROUPS, OP_BATCH_FLAGS_RANGE, OP_BITS_OFFSET, OP_BIT_EXTRA_COL_IDX,
        OP_INDEX_COL_IDX,
    },
    utils::collections::Vec,
    CodeBlockTable, StarkField, CTX_COL_IDX, DECODER_TRACE_RANGE, DECODER_TRACE_WIDTH, FMP_COL_IDX,
    FN_HASH_RANGE, IN_SYSCALL_COL_IDX, ONE, SYS_TRACE_RANGE, SYS_TRACE_WIDTH, ZERO,
};

// CONSTANTS
// ================================================================================================

const TWO: Felt = Felt::new(2);
const THREE: Felt = Felt::new(3);
const EIGHT: Felt = Felt::new(8);

const INIT_ADDR: Felt = ONE;
const FMP_MIN: Felt = Felt::new(crate::FMP_MIN);
const SYSCALL_FMP_MIN: Felt = Felt::new(crate::SYSCALL_FMP_MIN);

// TYPE ALIASES
// ================================================================================================

type SystemTrace = [Vec<Felt>; SYS_TRACE_WIDTH];
type DecoderTrace = [Vec<Felt>; DECODER_TRACE_WIDTH];

// SPAN BLOCK TESTS
// ================================================================================================

#[test]
fn span_block_one_group() {
    let ops = vec![Operation::Pad, Operation::Add, Operation::Mul];
    let span = Span::new(ops.clone());
    let program = CodeBlock::new_span(ops.clone());

    let (trace, aux_hints, trace_len) = build_trace(&[], &program);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    check_op_decoding(&trace, 0, ZERO, Operation::Span, 1, 0, 0);
    check_op_decoding(&trace, 1, INIT_ADDR, Operation::Pad, 0, 0, 1);
    check_op_decoding(&trace, 2, INIT_ADDR, Operation::Add, 0, 1, 1);
    check_op_decoding(&trace, 3, INIT_ADDR, Operation::Mul, 0, 2, 1);
    check_op_decoding(&trace, 4, INIT_ADDR, Operation::End, 0, 0, 0);
    check_op_decoding(&trace, 5, ZERO, Operation::Halt, 0, 0, 0);

    // --- check hasher state columns -------------------------------------------------------------
    let program_hash: Word = program.hash().into();
    check_hasher_state(
        &trace,
        vec![
            span.op_batches()[0].groups().to_vec(), // first group should contain op batch
            vec![build_op_group(&ops[1..])],
            vec![build_op_group(&ops[2..])],
            vec![],
            program_hash.to_vec(), // last row should contain program hash
        ],
    );

    // HALT opcode and program hash gets propagated to the last row
    for i in 6..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
        assert_eq!(ONE, trace[OP_BIT_EXTRA_COL_IDX][i]);
        assert_eq!(program_hash, get_hasher_state1(&trace, i));
    }

    // --- check op_group table hints -------------------------------------------------------------
    // op_group table should not have been touched
    assert!(&aux_hints.op_group_table_hints().is_empty());
    assert!(aux_hints.op_group_table_rows().is_empty());

    // --- check block execution hints ------------------------------------------------------------
    let expected_hints =
        vec![(0, BlockTableUpdate::BlockStarted(0)), (4, BlockTableUpdate::BlockEnded(false))];
    assert_eq!(expected_hints, aux_hints.block_exec_hints());

    // --- check block stack table hints ----------------------------------------------------------
    let expected_rows = vec![BlockStackTableRow::new_test(INIT_ADDR, ZERO, false)];
    assert_eq!(expected_rows, aux_hints.block_stack_table_rows());

    // --- check block hash table hints ----------------------------------------------------------
    let expected_rows = vec![BlockHashTableRow::from_program_hash(program_hash)];
    assert_eq!(expected_rows, aux_hints.block_hash_table_rows());
}

#[test]
fn span_block_small() {
    let iv = [ONE, TWO];
    let ops = vec![Operation::Push(iv[0]), Operation::Push(iv[1]), Operation::Add];
    let span = Span::new(ops.clone());
    let program = CodeBlock::new_span(ops.clone());

    let (trace, aux_hints, trace_len) = build_trace(&[], &program);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    check_op_decoding(&trace, 0, ZERO, Operation::Span, 4, 0, 0);
    check_op_decoding(&trace, 1, INIT_ADDR, Operation::Push(iv[0]), 3, 0, 1);
    check_op_decoding(&trace, 2, INIT_ADDR, Operation::Push(iv[1]), 2, 1, 1);
    check_op_decoding(&trace, 3, INIT_ADDR, Operation::Add, 1, 2, 1);
    // starting new group: NOOP group is inserted by the processor to make sure number of groups
    // is a power of two
    check_op_decoding(&trace, 4, INIT_ADDR, Operation::Noop, 0, 0, 1);
    check_op_decoding(&trace, 5, INIT_ADDR, Operation::End, 0, 0, 0);
    check_op_decoding(&trace, 6, ZERO, Operation::Halt, 0, 0, 0);

    // --- check hasher state columns -------------------------------------------------------------
    let program_hash: Word = program.hash().into();
    check_hasher_state(
        &trace,
        vec![
            span.op_batches()[0].groups().to_vec(),
            vec![build_op_group(&ops[1..])],
            vec![build_op_group(&ops[2..])],
            vec![],
            vec![],
            program_hash.to_vec(), // last row should contain program hash
        ],
    );

    // HALT opcode and program hash gets propagated to the last row
    for i in 7..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
        assert_eq!(ONE, trace[OP_BIT_EXTRA_COL_IDX][i]);
        assert_eq!(program_hash, get_hasher_state1(&trace, i));
    }

    // --- check op_group table hints -------------------------------------------------------------

    // 3 op groups should be inserted at cycle 0, and removed one by one in subsequent cycles
    let expected_ogt_hints = vec![
        (0, OpGroupTableUpdate::InsertRows(3)),
        (1, OpGroupTableUpdate::RemoveRow),
        (2, OpGroupTableUpdate::RemoveRow),
        (3, OpGroupTableUpdate::RemoveRow),
    ];
    assert_eq!(&expected_ogt_hints, aux_hints.op_group_table_hints());

    // the groups are imm(1), imm(2), and op group with a single NOOP
    let expected_ogt_rows = vec![
        OpGroupTableRow::new(INIT_ADDR, Felt::new(3), iv[0]),
        OpGroupTableRow::new(INIT_ADDR, TWO, iv[1]),
        OpGroupTableRow::new(INIT_ADDR, ONE, ZERO),
    ];
    assert_eq!(expected_ogt_rows, aux_hints.op_group_table_rows());

    // --- check block execution hints ------------------------------------------------------------
    let expected_hints =
        vec![(0, BlockTableUpdate::BlockStarted(0)), (5, BlockTableUpdate::BlockEnded(false))];
    assert_eq!(expected_hints, aux_hints.block_exec_hints());

    // --- check block stack table hints ----------------------------------------------------------
    let expected_rows = vec![BlockStackTableRow::new_test(INIT_ADDR, ZERO, false)];
    assert_eq!(expected_rows, aux_hints.block_stack_table_rows());

    // --- check block hash table hints ----------------------------------------------------------
    let expected_rows = vec![BlockHashTableRow::from_program_hash(program_hash)];
    assert_eq!(expected_rows, aux_hints.block_hash_table_rows());
}

#[test]
fn span_block() {
    let iv = [ONE, TWO, Felt::new(3), Felt::new(4), Felt::new(5)];
    let ops = vec![
        Operation::Push(iv[0]),
        Operation::Push(iv[1]),
        Operation::Push(iv[2]),
        Operation::Pad,
        Operation::Mul,
        Operation::Add,
        Operation::Drop,
        Operation::Push(iv[3]),
        Operation::Push(iv[4]),
        Operation::Mul,
        Operation::Add,
        Operation::Inv,
    ];
    let span = Span::new(ops.clone());
    let program = CodeBlock::new_span(ops.clone());
    let (trace, aux_hints, trace_len) = build_trace(&[], &program);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    check_op_decoding(&trace, 0, ZERO, Operation::Span, 8, 0, 0);
    check_op_decoding(&trace, 1, INIT_ADDR, Operation::Push(iv[0]), 7, 0, 1);
    check_op_decoding(&trace, 2, INIT_ADDR, Operation::Push(iv[1]), 6, 1, 1);
    check_op_decoding(&trace, 3, INIT_ADDR, Operation::Push(iv[2]), 5, 2, 1);
    check_op_decoding(&trace, 4, INIT_ADDR, Operation::Pad, 4, 3, 1);
    check_op_decoding(&trace, 5, INIT_ADDR, Operation::Mul, 4, 4, 1);
    check_op_decoding(&trace, 6, INIT_ADDR, Operation::Add, 4, 5, 1);
    check_op_decoding(&trace, 7, INIT_ADDR, Operation::Drop, 4, 6, 1);
    check_op_decoding(&trace, 8, INIT_ADDR, Operation::Push(iv[3]), 4, 7, 1);
    // NOOP inserted by the processor to make sure the group doesn't end with a PUSH
    check_op_decoding(&trace, 9, INIT_ADDR, Operation::Noop, 3, 8, 1);
    // starting new operation group
    check_op_decoding(&trace, 10, INIT_ADDR, Operation::Push(iv[4]), 2, 0, 1);
    check_op_decoding(&trace, 11, INIT_ADDR, Operation::Mul, 1, 1, 1);
    check_op_decoding(&trace, 12, INIT_ADDR, Operation::Add, 1, 2, 1);
    check_op_decoding(&trace, 13, INIT_ADDR, Operation::Inv, 1, 3, 1);
    // NOOP inserted by the processor to make sure the number of groups is a power of two
    check_op_decoding(&trace, 14, INIT_ADDR, Operation::Noop, 0, 0, 1);
    check_op_decoding(&trace, 15, INIT_ADDR, Operation::End, 0, 0, 0);
    check_op_decoding(&trace, 16, ZERO, Operation::Halt, 0, 0, 0);

    // --- check hasher state columns -------------------------------------------------------------
    let program_hash: Word = program.hash().into();
    check_hasher_state(
        &trace,
        vec![
            span.op_batches()[0].groups().to_vec(),
            vec![build_op_group(&ops[1..8])], // first group starts
            vec![build_op_group(&ops[2..8])],
            vec![build_op_group(&ops[3..8])],
            vec![build_op_group(&ops[4..8])],
            vec![build_op_group(&ops[5..8])],
            vec![build_op_group(&ops[6..8])],
            vec![build_op_group(&ops[7..8])],
            vec![], // NOOP inserted after push
            vec![],
            vec![build_op_group(&ops[9..])], // next group starts
            vec![build_op_group(&ops[10..])],
            vec![build_op_group(&ops[11..])],
            vec![],
            vec![],                // a group with single NOOP added at the end
            program_hash.to_vec(), // last row should contain program hash
        ],
    );

    // HALT opcode and program hash gets propagated to the last row
    for i in 17..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
        assert_eq!(ONE, trace[OP_BIT_EXTRA_COL_IDX][i]);
        assert_eq!(program_hash, get_hasher_state1(&trace, i));
    }

    // --- check op_group table hints -------------------------------------------------------------

    let expected_ogt_hints = vec![
        (0, OpGroupTableUpdate::InsertRows(7)),
        (1, OpGroupTableUpdate::RemoveRow),
        (2, OpGroupTableUpdate::RemoveRow),
        (3, OpGroupTableUpdate::RemoveRow),
        (8, OpGroupTableUpdate::RemoveRow),
        (9, OpGroupTableUpdate::RemoveRow),
        (10, OpGroupTableUpdate::RemoveRow),
        (13, OpGroupTableUpdate::RemoveRow),
    ];
    assert_eq!(&expected_ogt_hints, aux_hints.op_group_table_hints());

    let batch0_groups = &span.op_batches()[0].groups();
    let expected_ogt_rows = vec![
        OpGroupTableRow::new(INIT_ADDR, Felt::new(7), batch0_groups[1]),
        OpGroupTableRow::new(INIT_ADDR, Felt::new(6), batch0_groups[2]),
        OpGroupTableRow::new(INIT_ADDR, Felt::new(5), batch0_groups[3]),
        OpGroupTableRow::new(INIT_ADDR, Felt::new(4), batch0_groups[4]),
        OpGroupTableRow::new(INIT_ADDR, Felt::new(3), batch0_groups[5]),
        OpGroupTableRow::new(INIT_ADDR, TWO, batch0_groups[6]),
        OpGroupTableRow::new(INIT_ADDR, ONE, batch0_groups[7]),
    ];
    assert_eq!(expected_ogt_rows, aux_hints.op_group_table_rows());

    // --- check block execution hints ------------------------------------------------------------
    let expected_hints = vec![
        (0, BlockTableUpdate::BlockStarted(0)),
        (15, BlockTableUpdate::BlockEnded(false)),
    ];
    assert_eq!(expected_hints, aux_hints.block_exec_hints());

    // --- check block stack table hints ----------------------------------------------------------
    let expected_rows = vec![BlockStackTableRow::new_test(INIT_ADDR, ZERO, false)];
    assert_eq!(expected_rows, aux_hints.block_stack_table_rows());

    // --- check block hash table hints ----------------------------------------------------------
    let expected_rows = vec![BlockHashTableRow::from_program_hash(program_hash)];
    assert_eq!(expected_rows, aux_hints.block_hash_table_rows());
}

#[test]
fn span_block_with_respan() {
    let iv = [
        ONE,
        TWO,
        Felt::new(3),
        Felt::new(4),
        Felt::new(5),
        Felt::new(6),
        Felt::new(7),
        EIGHT,
        Felt::new(9),
    ];

    let ops = vec![
        Operation::Push(iv[0]),
        Operation::Push(iv[1]),
        Operation::Push(iv[2]),
        Operation::Push(iv[3]),
        Operation::Push(iv[4]),
        Operation::Push(iv[5]),
        Operation::Push(iv[6]),
        Operation::Push(iv[7]),
        Operation::Add,
        Operation::Push(iv[8]),
    ];
    let span = Span::new(ops.clone());
    let program = CodeBlock::new_span(ops.clone());
    let (trace, aux_hints, trace_len) = build_trace(&[], &program);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    check_op_decoding(&trace, 0, ZERO, Operation::Span, 12, 0, 0);
    check_op_decoding(&trace, 1, INIT_ADDR, Operation::Push(iv[0]), 11, 0, 1);
    check_op_decoding(&trace, 2, INIT_ADDR, Operation::Push(iv[1]), 10, 1, 1);
    check_op_decoding(&trace, 3, INIT_ADDR, Operation::Push(iv[2]), 9, 2, 1);
    check_op_decoding(&trace, 4, INIT_ADDR, Operation::Push(iv[3]), 8, 3, 1);
    check_op_decoding(&trace, 5, INIT_ADDR, Operation::Push(iv[4]), 7, 4, 1);
    check_op_decoding(&trace, 6, INIT_ADDR, Operation::Push(iv[5]), 6, 5, 1);
    check_op_decoding(&trace, 7, INIT_ADDR, Operation::Push(iv[6]), 5, 6, 1);
    // NOOP inserted by the processor to make sure the group doesn't end with a PUSH
    check_op_decoding(&trace, 8, INIT_ADDR, Operation::Noop, 4, 7, 1);
    // RESPAN since the previous batch is full
    let batch1_addr = INIT_ADDR + EIGHT;
    check_op_decoding(&trace, 9, INIT_ADDR, Operation::Respan, 4, 0, 0);
    check_op_decoding(&trace, 10, batch1_addr, Operation::Push(iv[7]), 3, 0, 1);
    check_op_decoding(&trace, 11, batch1_addr, Operation::Add, 2, 1, 1);
    check_op_decoding(&trace, 12, batch1_addr, Operation::Push(iv[8]), 2, 2, 1);
    // NOOP inserted by the processor to make sure the group doesn't end with a PUSH
    check_op_decoding(&trace, 13, batch1_addr, Operation::Noop, 1, 3, 1);
    // NOOP inserted by the processor to make sure the number of groups is a power of two
    check_op_decoding(&trace, 14, batch1_addr, Operation::Noop, 0, 0, 1);
    check_op_decoding(&trace, 15, batch1_addr, Operation::End, 0, 0, 0);
    check_op_decoding(&trace, 16, ZERO, Operation::Halt, 0, 0, 0);

    // --- check hasher state columns -------------------------------------------------------------
    let program_hash: Word = program.hash().into();
    check_hasher_state(
        &trace,
        vec![
            span.op_batches()[0].groups().to_vec(),
            vec![build_op_group(&ops[1..7])], // first group starts
            vec![build_op_group(&ops[2..7])],
            vec![build_op_group(&ops[3..7])],
            vec![build_op_group(&ops[4..7])],
            vec![build_op_group(&ops[5..7])],
            vec![build_op_group(&ops[6..7])],
            vec![],
            vec![], // a NOOP inserted after last PUSH
            span.op_batches()[1].groups().to_vec(),
            vec![build_op_group(&ops[8..])], // next group starts
            vec![build_op_group(&ops[9..])],
            vec![],
            vec![],                // a NOOP is inserted after last PUSH
            vec![],                // a group with single NOOP added at the end
            program_hash.to_vec(), // last row should contain program hash
        ],
    );

    // HALT opcode and program hash gets propagated to the last row
    for i in 17..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
        assert_eq!(ONE, trace[OP_BIT_EXTRA_COL_IDX][i]);
        assert_eq!(program_hash, get_hasher_state1(&trace, i));
    }

    // --- check op_group table hints -------------------------------------------------------------

    let expected_ogt_hints = vec![
        (0, OpGroupTableUpdate::InsertRows(7)),
        (1, OpGroupTableUpdate::RemoveRow),
        (2, OpGroupTableUpdate::RemoveRow),
        (3, OpGroupTableUpdate::RemoveRow),
        (4, OpGroupTableUpdate::RemoveRow),
        (5, OpGroupTableUpdate::RemoveRow),
        (6, OpGroupTableUpdate::RemoveRow),
        (7, OpGroupTableUpdate::RemoveRow),
        (9, OpGroupTableUpdate::InsertRows(3)),
        (10, OpGroupTableUpdate::RemoveRow),
        (12, OpGroupTableUpdate::RemoveRow),
        (13, OpGroupTableUpdate::RemoveRow),
    ];
    assert_eq!(&expected_ogt_hints, aux_hints.op_group_table_hints());

    let batch0_groups = &span.op_batches()[0].groups();
    let batch1_groups = &span.op_batches()[1].groups();
    let expected_ogt_rows = vec![
        OpGroupTableRow::new(INIT_ADDR, Felt::new(11), batch0_groups[1]),
        OpGroupTableRow::new(INIT_ADDR, Felt::new(10), batch0_groups[2]),
        OpGroupTableRow::new(INIT_ADDR, Felt::new(9), batch0_groups[3]),
        OpGroupTableRow::new(INIT_ADDR, EIGHT, batch0_groups[4]),
        OpGroupTableRow::new(INIT_ADDR, Felt::new(7), batch0_groups[5]),
        OpGroupTableRow::new(INIT_ADDR, Felt::new(6), batch0_groups[6]),
        OpGroupTableRow::new(INIT_ADDR, Felt::new(5), batch0_groups[7]),
        // skipping the first group of batch 1
        OpGroupTableRow::new(batch1_addr, Felt::new(3), batch1_groups[1]),
        OpGroupTableRow::new(batch1_addr, TWO, batch1_groups[2]),
        OpGroupTableRow::new(batch1_addr, ONE, batch1_groups[3]),
    ];
    assert_eq!(expected_ogt_rows, aux_hints.op_group_table_rows());

    // --- check block execution hints ------------------------------------------------------------
    let expected_hints = vec![
        (0, BlockTableUpdate::BlockStarted(0)),
        (9, BlockTableUpdate::SpanExtended),
        (15, BlockTableUpdate::BlockEnded(false)),
    ];
    assert_eq!(expected_hints, aux_hints.block_exec_hints());

    // --- check block stack table hints ----------------------------------------------------------
    let expected_rows = vec![
        BlockStackTableRow::new_test(INIT_ADDR, ZERO, false),
        BlockStackTableRow::new_test(batch1_addr, ZERO, false),
    ];
    assert_eq!(expected_rows, aux_hints.block_stack_table_rows());

    // --- check block hash table hints ----------------------------------------------------------
    let expected_rows = vec![BlockHashTableRow::from_program_hash(program_hash)];
    assert_eq!(expected_rows, aux_hints.block_hash_table_rows());
}

// JOIN BLOCK TESTS
// ================================================================================================

#[test]
fn join_block() {
    let span1 = CodeBlock::new_span(vec![Operation::Mul]);
    let span2 = CodeBlock::new_span(vec![Operation::Add]);
    let program = CodeBlock::new_join([span1.clone(), span2.clone()]);

    let (trace, aux_hints, trace_len) = build_trace(&[], &program);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    check_op_decoding(&trace, 0, ZERO, Operation::Join, 0, 0, 0);
    // starting first span
    let span1_addr = INIT_ADDR + EIGHT;
    check_op_decoding(&trace, 1, INIT_ADDR, Operation::Span, 1, 0, 0);
    check_op_decoding(&trace, 2, span1_addr, Operation::Mul, 0, 0, 1);
    check_op_decoding(&trace, 3, span1_addr, Operation::End, 0, 0, 0);
    // starting second span
    let span2_addr = INIT_ADDR + Felt::new(16);
    check_op_decoding(&trace, 4, INIT_ADDR, Operation::Span, 1, 0, 0);
    check_op_decoding(&trace, 5, span2_addr, Operation::Add, 0, 0, 1);
    check_op_decoding(&trace, 6, span2_addr, Operation::End, 0, 0, 0);
    check_op_decoding(&trace, 7, INIT_ADDR, Operation::End, 0, 0, 0);
    check_op_decoding(&trace, 8, ZERO, Operation::Halt, 0, 0, 0);

    // --- check hasher state columns -------------------------------------------------------------

    // in the first row, the hasher state is set to hashes of both child nodes
    let span1_hash: Word = span1.hash().into();
    let span2_hash: Word = span2.hash().into();
    assert_eq!(span1_hash, get_hasher_state1(&trace, 0));
    assert_eq!(span2_hash, get_hasher_state2(&trace, 0));

    // at the end of the first SPAN, the hasher state is set to the hash of the first child
    assert_eq!(span1_hash, get_hasher_state1(&trace, 3));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&trace, 3));

    // at the end of the second SPAN, the hasher state is set to the hash of the second child
    assert_eq!(span2_hash, get_hasher_state1(&trace, 6));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&trace, 6));

    // at the end of the program, the hasher state is set to the hash of the entire program
    let program_hash: Word = program.hash().into();
    assert_eq!(program_hash, get_hasher_state1(&trace, 7));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&trace, 7));

    // HALT opcode and program hash gets propagated to the last row
    for i in 9..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
        assert_eq!(ONE, trace[OP_BIT_EXTRA_COL_IDX][i]);
        assert_eq!(program_hash, get_hasher_state1(&trace, i));
    }

    // --- check op_group table hints -------------------------------------------------------------
    // op_group table should not have been touched
    assert!(&aux_hints.op_group_table_hints().is_empty());
    assert!(aux_hints.op_group_table_rows().is_empty());

    // --- check block execution hints ------------------------------------------------------------
    let expected_hints = vec![
        (0, BlockTableUpdate::BlockStarted(2)),
        (1, BlockTableUpdate::BlockStarted(0)),
        (3, BlockTableUpdate::BlockEnded(true)),
        (4, BlockTableUpdate::BlockStarted(0)),
        (6, BlockTableUpdate::BlockEnded(false)),
        (7, BlockTableUpdate::BlockEnded(false)),
    ];
    assert_eq!(expected_hints, aux_hints.block_exec_hints());

    // --- check block stack table hints ----------------------------------------------------------
    let expected_rows = vec![
        BlockStackTableRow::new_test(INIT_ADDR, ZERO, false),
        BlockStackTableRow::new_test(span1_addr, INIT_ADDR, false),
        BlockStackTableRow::new_test(span2_addr, INIT_ADDR, false),
    ];
    assert_eq!(expected_rows, aux_hints.block_stack_table_rows());

    // --- check block hash table hints ----------------------------------------------------------
    let expected_rows = vec![
        BlockHashTableRow::from_program_hash(program_hash),
        BlockHashTableRow::new_test(INIT_ADDR, span1_hash, true, false),
        BlockHashTableRow::new_test(INIT_ADDR, span2_hash, false, false),
    ];
    assert_eq!(expected_rows, aux_hints.block_hash_table_rows());
}

// SPLIT BLOCK TESTS
// ================================================================================================

#[test]
fn split_block_true() {
    let span1 = CodeBlock::new_span(vec![Operation::Mul]);
    let span2 = CodeBlock::new_span(vec![Operation::Add]);
    let program = CodeBlock::new_split(span1.clone(), span2.clone());

    let (trace, aux_hints, trace_len) = build_trace(&[1], &program);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    let span_addr = INIT_ADDR + EIGHT;
    check_op_decoding(&trace, 0, ZERO, Operation::Split, 0, 0, 0);
    check_op_decoding(&trace, 1, INIT_ADDR, Operation::Span, 1, 0, 0);
    check_op_decoding(&trace, 2, span_addr, Operation::Mul, 0, 0, 1);
    check_op_decoding(&trace, 3, span_addr, Operation::End, 0, 0, 0);
    check_op_decoding(&trace, 4, INIT_ADDR, Operation::End, 0, 0, 0);
    check_op_decoding(&trace, 5, ZERO, Operation::Halt, 0, 0, 0);

    // --- check hasher state columns -------------------------------------------------------------

    // in the first row, the hasher state is set to hashes of both child nodes
    let span1_hash: Word = span1.hash().into();
    let span2_hash: Word = span2.hash().into();
    assert_eq!(span1_hash, get_hasher_state1(&trace, 0));
    assert_eq!(span2_hash, get_hasher_state2(&trace, 0));

    // at the end of the SPAN, the hasher state is set to the hash of the first child
    assert_eq!(span1_hash, get_hasher_state1(&trace, 3));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&trace, 3));

    // at the end of the program, the hasher state is set to the hash of the entire program
    let program_hash: Word = program.hash().into();
    assert_eq!(program_hash, get_hasher_state1(&trace, 4));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&trace, 4));

    // HALT opcode and program hash gets propagated to the last row
    for i in 6..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
        assert_eq!(ONE, trace[OP_BIT_EXTRA_COL_IDX][i]);
        assert_eq!(program_hash, get_hasher_state1(&trace, i));
    }

    // --- check op_group table hints -------------------------------------------------------------
    // op_group table should not have been touched
    assert!(&aux_hints.op_group_table_hints().is_empty());
    assert!(aux_hints.op_group_table_rows().is_empty());

    // --- check block execution hints ------------------------------------------------------------
    let expected_hints = vec![
        (0, BlockTableUpdate::BlockStarted(1)),
        (1, BlockTableUpdate::BlockStarted(0)),
        (3, BlockTableUpdate::BlockEnded(false)),
        (4, BlockTableUpdate::BlockEnded(false)),
    ];
    assert_eq!(expected_hints, aux_hints.block_exec_hints());

    // --- check block stack table hints ----------------------------------------------------------
    let expected_rows = vec![
        BlockStackTableRow::new_test(INIT_ADDR, ZERO, false),
        BlockStackTableRow::new_test(span_addr, INIT_ADDR, false),
    ];
    assert_eq!(expected_rows, aux_hints.block_stack_table_rows());

    // --- check block hash table hints ----------------------------------------------------------
    let expected_rows = vec![
        BlockHashTableRow::from_program_hash(program_hash),
        BlockHashTableRow::new_test(INIT_ADDR, span1_hash, false, false),
    ];
    assert_eq!(expected_rows, aux_hints.block_hash_table_rows());
}

#[test]
fn split_block_false() {
    let span1 = CodeBlock::new_span(vec![Operation::Mul]);
    let span2 = CodeBlock::new_span(vec![Operation::Add]);
    let program = CodeBlock::new_split(span1.clone(), span2.clone());

    let (trace, aux_hints, trace_len) = build_trace(&[0], &program);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    let span_addr = INIT_ADDR + EIGHT;
    check_op_decoding(&trace, 0, ZERO, Operation::Split, 0, 0, 0);
    check_op_decoding(&trace, 1, INIT_ADDR, Operation::Span, 1, 0, 0);
    check_op_decoding(&trace, 2, span_addr, Operation::Add, 0, 0, 1);
    check_op_decoding(&trace, 3, span_addr, Operation::End, 0, 0, 0);
    check_op_decoding(&trace, 4, INIT_ADDR, Operation::End, 0, 0, 0);
    check_op_decoding(&trace, 5, ZERO, Operation::Halt, 0, 0, 0);

    // --- check hasher state columns -------------------------------------------------------------

    // in the first row, the hasher state is set to hashes of both child nodes
    let span1_hash: Word = span1.hash().into();
    let span2_hash: Word = span2.hash().into();
    assert_eq!(span1_hash, get_hasher_state1(&trace, 0));
    assert_eq!(span2_hash, get_hasher_state2(&trace, 0));

    // at the end of the SPAN, the hasher state is set to the hash of the second child
    assert_eq!(span2_hash, get_hasher_state1(&trace, 3));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&trace, 3));

    // at the end of the program, the hasher state is set to the hash of the entire program
    let program_hash: Word = program.hash().into();
    assert_eq!(program_hash, get_hasher_state1(&trace, 4));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&trace, 4));

    // HALT opcode and program hash gets propagated to the last row
    for i in 6..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
        assert_eq!(ONE, trace[OP_BIT_EXTRA_COL_IDX][i]);
        assert_eq!(program_hash, get_hasher_state1(&trace, i));
    }

    // --- check op_group table hints -------------------------------------------------------------
    // op_group table should not have been touched
    assert!(&aux_hints.op_group_table_hints().is_empty());
    assert!(aux_hints.op_group_table_rows().is_empty());

    // --- check block execution hints ------------------------------------------------------------
    let expected_hints = vec![
        (0, BlockTableUpdate::BlockStarted(1)),
        (1, BlockTableUpdate::BlockStarted(0)),
        (3, BlockTableUpdate::BlockEnded(false)),
        (4, BlockTableUpdate::BlockEnded(false)),
    ];
    assert_eq!(expected_hints, aux_hints.block_exec_hints());

    // --- check block stack table hints ----------------------------------------------------------
    let expected_rows = vec![
        BlockStackTableRow::new_test(INIT_ADDR, ZERO, false),
        BlockStackTableRow::new_test(span_addr, INIT_ADDR, false),
    ];
    assert_eq!(expected_rows, aux_hints.block_stack_table_rows());

    // --- check block hash table hints ----------------------------------------------------------
    let expected_rows = vec![
        BlockHashTableRow::from_program_hash(program_hash),
        BlockHashTableRow::new_test(INIT_ADDR, span2_hash, false, false),
    ];
    assert_eq!(expected_rows, aux_hints.block_hash_table_rows());
}

// LOOP BLOCK TESTS
// ================================================================================================

#[test]
fn loop_block() {
    let loop_body = CodeBlock::new_span(vec![Operation::Pad, Operation::Drop]);
    let program = CodeBlock::new_loop(loop_body.clone());

    let (trace, aux_hints, trace_len) = build_trace(&[0, 1], &program);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    let body_addr = INIT_ADDR + EIGHT;
    check_op_decoding(&trace, 0, ZERO, Operation::Loop, 0, 0, 0);
    check_op_decoding(&trace, 1, INIT_ADDR, Operation::Span, 1, 0, 0);
    check_op_decoding(&trace, 2, body_addr, Operation::Pad, 0, 0, 1);
    check_op_decoding(&trace, 3, body_addr, Operation::Drop, 0, 1, 1);
    check_op_decoding(&trace, 4, body_addr, Operation::End, 0, 0, 0);
    check_op_decoding(&trace, 5, INIT_ADDR, Operation::End, 0, 0, 0);
    check_op_decoding(&trace, 6, ZERO, Operation::Halt, 0, 0, 0);

    // --- check hasher state columns -------------------------------------------------------------

    // in the first row, the hasher state is set to the hash of the loop's body
    let loop_body_hash: Word = loop_body.hash().into();
    assert_eq!(loop_body_hash, get_hasher_state1(&trace, 0));
    assert_eq!([ZERO; 4], get_hasher_state2(&trace, 0));

    // at the end of the SPAN block, the hasher state is also set to the hash of the loops body,
    // and is_loop_body flag is also set to ONE
    assert_eq!(loop_body_hash, get_hasher_state1(&trace, 4));
    assert_eq!([ONE, ZERO, ZERO, ZERO], get_hasher_state2(&trace, 4));

    // the hash of the program is located in the last END row; this row should also have is_loop
    // flag set to ONE
    let program_hash: Word = program.hash().into();
    assert_eq!(program_hash, get_hasher_state1(&trace, 5));
    assert_eq!([ZERO, ONE, ZERO, ZERO], get_hasher_state2(&trace, 5));

    // HALT opcode and program hash gets propagated to the last row
    for i in 7..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
        assert_eq!(ONE, trace[OP_BIT_EXTRA_COL_IDX][i]);
        assert_eq!(program_hash, get_hasher_state1(&trace, i));
    }

    // --- check op_group table hints -------------------------------------------------------------
    // op_group table should not have been touched
    assert!(&aux_hints.op_group_table_hints().is_empty());
    assert!(aux_hints.op_group_table_rows().is_empty());

    // --- check block execution hints ------------------------------------------------------------
    let expected_hints = vec![
        (0, BlockTableUpdate::BlockStarted(1)),
        (1, BlockTableUpdate::BlockStarted(0)),
        (4, BlockTableUpdate::BlockEnded(false)),
        (5, BlockTableUpdate::BlockEnded(false)),
    ];
    assert_eq!(expected_hints, aux_hints.block_exec_hints());

    // --- check block stack table hints ----------------------------------------------------------
    let expected_rows = vec![
        BlockStackTableRow::new_test(INIT_ADDR, ZERO, true),
        BlockStackTableRow::new_test(body_addr, INIT_ADDR, false),
    ];
    assert_eq!(expected_rows, aux_hints.block_stack_table_rows());

    // --- check block hash table hints ----------------------------------------------------------
    let expected_rows = vec![
        BlockHashTableRow::from_program_hash(program_hash),
        BlockHashTableRow::new_test(INIT_ADDR, loop_body_hash, false, true),
    ];
    assert_eq!(expected_rows, aux_hints.block_hash_table_rows());
}

#[test]
fn loop_block_skip() {
    let loop_body = CodeBlock::new_span(vec![Operation::Pad, Operation::Drop]);
    let program = CodeBlock::new_loop(loop_body.clone());

    let (trace, aux_hints, trace_len) = build_trace(&[0], &program);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    check_op_decoding(&trace, 0, ZERO, Operation::Loop, 0, 0, 0);
    check_op_decoding(&trace, 1, INIT_ADDR, Operation::End, 0, 0, 0);
    check_op_decoding(&trace, 2, ZERO, Operation::Halt, 0, 0, 0);

    // --- check hasher state columns -------------------------------------------------------------

    // in the first row, the hasher state is set to the hash of the loop's body
    let loop_body_hash: Word = loop_body.hash().into();
    assert_eq!(loop_body_hash, get_hasher_state1(&trace, 0));
    assert_eq!([ZERO; 4], get_hasher_state2(&trace, 0));

    // the hash of the program is located in the last END row; is_loop is not set to ONE because
    // we didn't enter the loop's body
    let program_hash: Word = program.hash().into();
    assert_eq!(program_hash, get_hasher_state1(&trace, 1));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&trace, 1));

    // HALT opcode and program hash gets propagated to the last row
    for i in 3..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
        assert_eq!(ONE, trace[OP_BIT_EXTRA_COL_IDX][i]);
        assert_eq!(program_hash, get_hasher_state1(&trace, i));
    }

    // --- check op_group table hints -------------------------------------------------------------
    // op_group table should not have been touched
    assert!(&aux_hints.op_group_table_hints().is_empty());
    assert!(aux_hints.op_group_table_rows().is_empty());

    // --- check block execution hints ------------------------------------------------------------
    let expected_hints =
        vec![(0, BlockTableUpdate::BlockStarted(0)), (1, BlockTableUpdate::BlockEnded(false))];
    assert_eq!(expected_hints, aux_hints.block_exec_hints());

    // --- check block stack table hints ----------------------------------------------------------
    let expected_rows = vec![BlockStackTableRow::new_test(INIT_ADDR, ZERO, false)];
    assert_eq!(expected_rows, aux_hints.block_stack_table_rows());

    // --- check block hash table hints ----------------------------------------------------------
    let expected_rows = vec![BlockHashTableRow::from_program_hash(program_hash)];
    assert_eq!(expected_rows, aux_hints.block_hash_table_rows());
}

#[test]
fn loop_block_repeat() {
    let loop_body = CodeBlock::new_span(vec![Operation::Pad, Operation::Drop]);
    let program = CodeBlock::new_loop(loop_body.clone());

    let (trace, aux_hints, trace_len) = build_trace(&[0, 1, 1], &program);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    let iter1_addr = INIT_ADDR + EIGHT;
    let iter2_addr = INIT_ADDR + Felt::new(16);

    check_op_decoding(&trace, 0, ZERO, Operation::Loop, 0, 0, 0);
    check_op_decoding(&trace, 1, INIT_ADDR, Operation::Span, 1, 0, 0);
    check_op_decoding(&trace, 2, iter1_addr, Operation::Pad, 0, 0, 1);
    check_op_decoding(&trace, 3, iter1_addr, Operation::Drop, 0, 1, 1);
    check_op_decoding(&trace, 4, iter1_addr, Operation::End, 0, 0, 0);
    // start second iteration
    check_op_decoding(&trace, 5, INIT_ADDR, Operation::Repeat, 0, 0, 0);
    check_op_decoding(&trace, 6, INIT_ADDR, Operation::Span, 1, 0, 0);
    check_op_decoding(&trace, 7, iter2_addr, Operation::Pad, 0, 0, 1);
    check_op_decoding(&trace, 8, iter2_addr, Operation::Drop, 0, 1, 1);
    check_op_decoding(&trace, 9, iter2_addr, Operation::End, 0, 0, 0);
    check_op_decoding(&trace, 10, INIT_ADDR, Operation::End, 0, 0, 0);
    check_op_decoding(&trace, 11, ZERO, Operation::Halt, 0, 0, 0);

    // --- check hasher state columns -------------------------------------------------------------

    // in the first row, the hasher state is set to the hash of the loop's body
    let loop_body_hash: Word = loop_body.hash().into();
    assert_eq!(loop_body_hash, get_hasher_state1(&trace, 0));
    assert_eq!([ZERO; 4], get_hasher_state2(&trace, 0));

    // at the end of the first iteration, the hasher state is also set to the hash of the loops
    // body, and is_loop_body flag is also set to ONE
    assert_eq!(loop_body_hash, get_hasher_state1(&trace, 4));
    assert_eq!([ONE, ZERO, ZERO, ZERO], get_hasher_state2(&trace, 4));

    // at the RESPAN row hasher state is copied over from the previous row
    assert_eq!(loop_body_hash, get_hasher_state1(&trace, 5));
    assert_eq!([ONE, ZERO, ZERO, ZERO], get_hasher_state2(&trace, 5));

    // at the end of the second iteration, the hasher state is again set to the hash of the loops
    // body, and is_loop_body flag is also set to ONE
    assert_eq!(loop_body_hash, get_hasher_state1(&trace, 9));
    assert_eq!([ONE, ZERO, ZERO, ZERO], get_hasher_state2(&trace, 9));

    // the hash of the program is located in the last END row; this row should also have is_loop
    // flag set to ONE
    let program_hash: Word = program.hash().into();
    assert_eq!(program_hash, get_hasher_state1(&trace, 10));
    assert_eq!([ZERO, ONE, ZERO, ZERO], get_hasher_state2(&trace, 10));

    // HALT opcode and program hash gets propagated to the last row
    for i in 12..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
        assert_eq!(ONE, trace[OP_BIT_EXTRA_COL_IDX][i]);
        assert_eq!(program_hash, get_hasher_state1(&trace, i));
    }

    // --- check op_group table hints -------------------------------------------------------------
    // op_group table should not have been touched
    assert!(&aux_hints.op_group_table_hints().is_empty());
    assert!(aux_hints.op_group_table_rows().is_empty());

    // --- check block execution hints ------------------------------------------------------------
    let expected_hints = vec![
        (0, BlockTableUpdate::BlockStarted(1)),
        (1, BlockTableUpdate::BlockStarted(0)),
        (4, BlockTableUpdate::BlockEnded(false)),
        (5, BlockTableUpdate::LoopRepeated),
        (6, BlockTableUpdate::BlockStarted(0)),
        (9, BlockTableUpdate::BlockEnded(false)),
        (10, BlockTableUpdate::BlockEnded(false)),
    ];
    assert_eq!(expected_hints, aux_hints.block_exec_hints());

    // --- check block stack table hints ----------------------------------------------------------
    let expected_rows = vec![
        BlockStackTableRow::new_test(INIT_ADDR, ZERO, true),
        BlockStackTableRow::new_test(iter1_addr, INIT_ADDR, false),
        BlockStackTableRow::new_test(iter2_addr, INIT_ADDR, false),
    ];
    assert_eq!(expected_rows, aux_hints.block_stack_table_rows());

    // --- check block hash table hints ----------------------------------------------------------
    let expected_rows = vec![
        BlockHashTableRow::from_program_hash(program_hash),
        BlockHashTableRow::new_test(INIT_ADDR, loop_body_hash, false, true),
    ];
    assert_eq!(expected_rows, aux_hints.block_hash_table_rows());
}

// CALL BLOCK TESTS
// ================================================================================================

#[test]
#[rustfmt::skip]
fn call_block() {
    // build a program which looks like this:
    //
    // proc.foo
    //     fmp <- fmp + 1
    // end
    //
    // begin
    //    fmp <- fmp + 2
    //    call.foo
    //    stack[0] <- fmp
    // end

    let first_span = CodeBlock::new_span(vec![
        Operation::Push(TWO),
        Operation::FmpUpdate,
        Operation::Pad,
    ]);
    let foo_root = CodeBlock::new_span(vec![Operation::Push(ONE), Operation::FmpUpdate]);
    let last_span = CodeBlock::new_span(vec![Operation::FmpAdd]);

    let foo_call = CodeBlock::new_call(foo_root.hash());
    let join1 = CodeBlock::new_join([first_span.clone(), foo_call.clone()]);
    let program = CodeBlock::new_join([join1.clone(), last_span.clone()]);

    let (sys_trace, dec_trace, aux_hints, trace_len) =
        build_call_trace(&program, foo_root.clone(), None);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    check_op_decoding(&dec_trace, 0, ZERO, Operation::Join, 0, 0, 0);
    // starting the internal JOIN block
    let join1_addr = INIT_ADDR + EIGHT;
    check_op_decoding(&dec_trace, 1, INIT_ADDR, Operation::Join, 0, 0, 0);
    // starting first SPAN block
    let first_span_addr = join1_addr + EIGHT;
    check_op_decoding(&dec_trace, 2, join1_addr, Operation::Span, 2, 0, 0);
    check_op_decoding(&dec_trace, 3, first_span_addr, Operation::Push(TWO), 1, 0, 1);
    check_op_decoding(&dec_trace, 4, first_span_addr, Operation::FmpUpdate, 0, 1, 1);
    // as PAD operation is executed, the last item from the stack top moves to the overflow table.
    // thus, the overflow address for the top row in the table will be set to the clock cycle at
    // which PAD was executed - which is 5.
    let overflow_addr_after_pad = Felt::new(5);
    check_op_decoding(&dec_trace, 5, first_span_addr, Operation::Pad, 0, 2, 1);
    check_op_decoding(&dec_trace, 6, first_span_addr, Operation::End, 0, 0, 0);
    // starting CALL block
    let foo_call_addr = first_span_addr + EIGHT;
    check_op_decoding(&dec_trace, 7, join1_addr, Operation::Call, 0, 0, 0);
    // starting second SPAN block
    let foo_root_addr = foo_call_addr + EIGHT;
    check_op_decoding(&dec_trace, 8, foo_call_addr, Operation::Span, 2, 0, 0);
    check_op_decoding(&dec_trace, 9, foo_root_addr, Operation::Push(ONE), 1, 0, 1);
    check_op_decoding(&dec_trace, 10, foo_root_addr, Operation::FmpUpdate, 0, 1, 1);
    check_op_decoding(&dec_trace, 11, foo_root_addr, Operation::End, 0, 0, 0);
    // ending CALL block
    check_op_decoding(&dec_trace, 12, foo_call_addr, Operation::End, 0, 0, 0);
    // ending internal JOIN block
    check_op_decoding(&dec_trace, 13, join1_addr, Operation::End, 0, 0, 0);
    // starting the 3rd SPAN block
    let last_span_addr = foo_root_addr + EIGHT;
    check_op_decoding(&dec_trace, 14, INIT_ADDR, Operation::Span, 1, 0, 0);
    check_op_decoding(&dec_trace, 15, last_span_addr, Operation::FmpAdd, 0, 0, 1);
    check_op_decoding(&dec_trace, 16, last_span_addr, Operation::End, 0, 0, 0);
    // ending the program
    check_op_decoding(&dec_trace, 17, INIT_ADDR, Operation::End, 0, 0, 0);
    check_op_decoding(&dec_trace, 18, ZERO, Operation::Halt, 0, 0, 0);

    // --- check hasher state columns -------------------------------------------------------------
    // in the first row, the hasher state is set to hashes of (join1, span3)
    let join1_hash: Word = join1.hash().into();
    let last_span_hash: Word = last_span.hash().into();
    assert_eq!(join1_hash, get_hasher_state1(&dec_trace, 0));
    assert_eq!(last_span_hash, get_hasher_state2(&dec_trace, 0));

    // in the second row, the hasher state is set to hashes of (span1, fn_block)
    let first_span_hash: Word = first_span.hash().into();
    let foo_call_hash: Word = foo_call.hash().into();
    assert_eq!(first_span_hash, get_hasher_state1(&dec_trace, 1));
    assert_eq!(foo_call_hash, get_hasher_state2(&dec_trace, 1));

    // at the end of the first SPAN, the hasher state is set to the hash of the first child
    assert_eq!(first_span_hash, get_hasher_state1(&dec_trace, 6));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&dec_trace, 6));

    // in the 7th row, we start the CALL block which hash span2 as its only child
    let foo_root_hash: Word = foo_root.hash().into();
    assert_eq!(foo_root_hash, get_hasher_state1(&dec_trace, 7));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&dec_trace, 7));

    // span2 ends in the 11th row
    assert_eq!(foo_root_hash, get_hasher_state1(&dec_trace, 11));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&dec_trace, 11));

    // CALL block ends in the 12th row; the second to last element of the hasher state
    // is set to ONE because we are exiting the CALL block
    assert_eq!(foo_call_hash, get_hasher_state1(&dec_trace, 12));
    assert_eq!([ZERO, ZERO, ONE, ZERO], get_hasher_state2(&dec_trace, 12));

    // internal JOIN block ends in the 13th row
    assert_eq!(join1_hash, get_hasher_state1(&dec_trace, 13));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&dec_trace, 13));

    // span3 ends in the 14th row
    assert_eq!(last_span_hash, get_hasher_state1(&dec_trace, 16));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&dec_trace, 16));

    // the program ends in the 17th row
    let program_hash: Word = program.hash().into();
    assert_eq!(program_hash, get_hasher_state1(&dec_trace, 17));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&dec_trace, 17));

    // HALT opcode and program hash gets propagated to the last row
    for i in 18..trace_len {
        assert!(contains_op(&dec_trace, i, Operation::Halt));
        assert_eq!(ONE, dec_trace[OP_BIT_EXTRA_COL_IDX][i]);
        assert_eq!(program_hash, get_hasher_state1(&dec_trace, i));
    }

    // --- check the ctx column -------------------------------------------------------------------

    // for the first 7 cycles, we are in the root context
    for i in 0..8 {
        assert_eq!(sys_trace[CTX_COL_IDX][i], ZERO);
    }

    // when CALL operation is executed, we switch to the new context
    for i in 8..13 {
        assert_eq!(sys_trace[CTX_COL_IDX][i], EIGHT);
    }

    // once the CALL block exited, we go back to the root context
    for i in 13..trace_len {
        assert_eq!(sys_trace[CTX_COL_IDX][i], ZERO);
    }

    // --- check the fmp column -------------------------------------------------------------------

    // for the first 5 cycles fmp stays at initial value
    for i in 0..5 {
        assert_eq!(sys_trace[FMP_COL_IDX][i], FMP_MIN);
    }

    // when the first FmpUpdate is executed, fmp gets gets incremented by 2
    for i in 5..8 {
        assert_eq!(sys_trace[FMP_COL_IDX][i], FMP_MIN + TWO);
    }

    // when CALL operation is executed, fmp gets reset to the initial value
    for i in 8..11 {
        assert_eq!(sys_trace[FMP_COL_IDX][i], FMP_MIN);
    }

    // when the second FmpUpdate is executed, fmp gets gets incremented by 1
    for i in 11..13 {
        assert_eq!(sys_trace[FMP_COL_IDX][i], FMP_MIN + ONE);
    }

    // once the CALL block exited, fmp gets reset back to FMP_MIN + 2, and it remains unchanged
    // until the end of the trace
    for i in 13..trace_len {
        assert_eq!(sys_trace[FMP_COL_IDX][i], FMP_MIN + TWO);
    }

    // --- check the in_syscall column ------------------------------------------------------------

    // since no syscalls were made, values in the syscall flag column should be all ZEROs
    assert_eq!(
        &sys_trace[IN_SYSCALL_COL_IDX][..trace_len],
        vec![ZERO; trace_len]
    );

    // --- check fn hash columns ------------------------------------------------------------------

    // before the CALL operation is executed, we are in a root context and thus fn_hash is ZEROs.
    for i in 0..8 {
        assert_eq!(get_fn_hash(&sys_trace, i), [ZERO; 4]);
    }

    // inside the CALL block fn hash is set to the hash of the foo procedure
    for i in 8..13 {
        assert_eq!(get_fn_hash(&sys_trace, i), foo_root_hash);
    }

    // after the CALL block is ended, we are back in the root context
    for i in 13..trace_len {
        assert_eq!(get_fn_hash(&sys_trace, i), [ZERO; 4]);
    }

    // --- check block execution hints ------------------------------------------------------------
    let expected_hints = vec![
        (0, BlockTableUpdate::BlockStarted(2)),
        (1, BlockTableUpdate::BlockStarted(2)),
        (2, BlockTableUpdate::BlockStarted(0)),
        (6, BlockTableUpdate::BlockEnded(true)),
        (7, BlockTableUpdate::BlockStarted(1)),
        (8, BlockTableUpdate::BlockStarted(0)),
        (11, BlockTableUpdate::BlockEnded(false)),
        (12, BlockTableUpdate::BlockEnded(false)),
        (13, BlockTableUpdate::BlockEnded(true)),
        (14, BlockTableUpdate::BlockStarted(0)),
        (16, BlockTableUpdate::BlockEnded(false)),
        (17, BlockTableUpdate::BlockEnded(false)),
    ];
    assert_eq!(expected_hints, aux_hints.block_exec_hints());

    // --- check block stack table rows -----------------------------------------------------------
    let call_ctx =
        ExecutionContextInfo::new(0, [ZERO; 4], FMP_MIN + TWO, 17, overflow_addr_after_pad);
    let expected_rows = vec![
        BlockStackTableRow::new_test(INIT_ADDR, ZERO, false),
        BlockStackTableRow::new_test(join1_addr, INIT_ADDR, false),
        BlockStackTableRow::new_test(first_span_addr, join1_addr, false),
        BlockStackTableRow::new_test_with_ctx(foo_call_addr, join1_addr, false, call_ctx),
        BlockStackTableRow::new_test(foo_root_addr, foo_call_addr, false),
        BlockStackTableRow::new_test(last_span_addr, INIT_ADDR, false),
    ];
    assert_eq!(expected_rows, aux_hints.block_stack_table_rows());

    // --- check block hash table hints ----------------------------------------------------------
    let expected_rows = vec![
        BlockHashTableRow::from_program_hash(program_hash),
        BlockHashTableRow::new_test(INIT_ADDR, join1_hash, true, false),
        BlockHashTableRow::new_test(INIT_ADDR, last_span_hash, false, false),
        BlockHashTableRow::new_test(join1_addr, first_span_hash, true, false),
        BlockHashTableRow::new_test(join1_addr, foo_call_hash, false, false),
        BlockHashTableRow::new_test(foo_call_addr, foo_root_hash, false, false),
    ];
    assert_eq!(expected_rows, aux_hints.block_hash_table_rows());
}

// SYSCALL BLOCK TESTS
// ================================================================================================

#[test]
#[rustfmt::skip]
fn syscall_block() {
    // build a program which looks like this:
    //
    // --- kernel ---
    // export.foo
    //     fmp <- fmp + 3
    // end
    //
    // --- program ---
    // proc.bar
    //     fmp <- fmp + 2
    //     syscall.foo
    // end
    //
    // begin
    //    fmp <- fmp + 1
    //    syscall.bar
    //    stack[0] <- fmp
    // end

    // build foo procedure body
    let foo_root = CodeBlock::new_span(vec![Operation::Push(THREE), Operation::FmpUpdate]);

    // build bar procedure body
    let bar_span = CodeBlock::new_span(vec![Operation::Push(TWO), Operation::FmpUpdate]);
    let foo_call = CodeBlock::new_syscall(foo_root.hash());
    let bar_root = CodeBlock::new_join([bar_span.clone(), foo_call.clone()]);

    // build the program
    let first_span = CodeBlock::new_span(vec![
        Operation::Push(ONE),
        Operation::FmpUpdate,
        Operation::Pad,
    ]);
    let last_span = CodeBlock::new_span(vec![Operation::FmpAdd]);

    let bar_call = CodeBlock::new_call(bar_root.hash());
    let inner_join = CodeBlock::new_join([first_span.clone(), bar_call.clone()]);
    let program = CodeBlock::new_join([inner_join.clone(), last_span.clone()]);

    let (sys_trace, dec_trace, aux_hints, trace_len) =
        build_call_trace(&program, bar_root.clone(), Some(foo_root.clone()));

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    check_op_decoding(&dec_trace, 0, ZERO, Operation::Join, 0, 0, 0);
    // starting the internal JOIN block
    let inner_join_addr = INIT_ADDR + EIGHT;
    check_op_decoding(&dec_trace, 1, INIT_ADDR, Operation::Join, 0, 0, 0);
    // starting first SPAN block
    let first_span_addr = inner_join_addr + EIGHT;
    check_op_decoding(&dec_trace, 2, inner_join_addr, Operation::Span, 2, 0, 0);
    check_op_decoding(&dec_trace, 3, first_span_addr, Operation::Push(TWO), 1, 0, 1);
    check_op_decoding(&dec_trace, 4, first_span_addr, Operation::FmpUpdate, 0, 1, 1);
    // as PAD operation is executed, the last item from the stack top moves to the overflow table.
    // thus, the overflow address for the top row in the table will be set to the clock cycle at
    // which PAD was executed - which is 5.
    let overflow_addr_after_pad = Felt::new(5);
    check_op_decoding(&dec_trace, 5, first_span_addr, Operation::Pad, 0, 2, 1);
    check_op_decoding(&dec_trace, 6, first_span_addr, Operation::End, 0, 0, 0);

    // starting CALL block for bar
    let call_addr = first_span_addr + EIGHT;
    check_op_decoding(&dec_trace, 7, inner_join_addr, Operation::Call, 0, 0, 0);
    // starting JOIN block inside bar
    let bar_join_addr = call_addr + EIGHT;
    check_op_decoding(&dec_trace, 8, call_addr, Operation::Join, 0, 0, 0);
    // starting SPAN block inside bar
    let bar_span_addr = bar_join_addr + EIGHT;
    check_op_decoding(&dec_trace, 9, bar_join_addr, Operation::Span, 2, 0, 0);
    check_op_decoding(&dec_trace, 10, bar_span_addr, Operation::Push(ONE), 1, 0, 1);
    check_op_decoding(&dec_trace, 11, bar_span_addr, Operation::FmpUpdate, 0, 1, 1);
    check_op_decoding(&dec_trace, 12, bar_span_addr, Operation::End, 0, 0, 0);

    // starting SYSCALL block for bar
    let syscall_addr = bar_span_addr + EIGHT;
    check_op_decoding(&dec_trace, 13, bar_join_addr, Operation::SysCall, 0, 0, 0);
    // starting SPAN block within syscall
    let syscall_span_addr = syscall_addr + EIGHT;
    check_op_decoding(&dec_trace, 14, syscall_addr, Operation::Span, 2, 0, 0);
    check_op_decoding(&dec_trace, 15, syscall_span_addr, Operation::Push(THREE), 1, 0, 1);
    check_op_decoding(&dec_trace, 16, syscall_span_addr, Operation::FmpUpdate, 0, 1, 1);
    check_op_decoding(&dec_trace, 17, syscall_span_addr, Operation::End, 0, 0, 0);
    // ending SYSCALL block
    check_op_decoding(&dec_trace, 18, syscall_addr, Operation::End, 0, 0, 0);

    // ending CALL block
    check_op_decoding(&dec_trace, 19, bar_join_addr, Operation::End, 0, 0, 0);
    check_op_decoding(&dec_trace, 20, call_addr, Operation::End, 0, 0, 0);

    // ending the inner JOIN block
    check_op_decoding(&dec_trace, 21, inner_join_addr, Operation::End, 0, 0, 0);

    // starting the last SPAN block
    let last_span_addr = syscall_span_addr + EIGHT;
    check_op_decoding(&dec_trace, 22, INIT_ADDR, Operation::Span, 1, 0, 0);
    check_op_decoding(&dec_trace, 23, last_span_addr, Operation::FmpAdd, 0, 0, 1);
    check_op_decoding(&dec_trace, 24, last_span_addr, Operation::End, 0, 0, 0);

    // ending the program
    check_op_decoding(&dec_trace, 25, INIT_ADDR, Operation::End, 0, 0, 0);
    check_op_decoding(&dec_trace, 26, ZERO, Operation::Halt, 0, 0, 0);

    // --- check hasher state columns -------------------------------------------------------------
    // in the first row, the hasher state is set to hashes of (inner_join, last_span)
    let inner_join_hash: Word = inner_join.hash().into();
    let last_span_hash: Word = last_span.hash().into();
    assert_eq!(inner_join_hash, get_hasher_state1(&dec_trace, 0));
    assert_eq!(last_span_hash, get_hasher_state2(&dec_trace, 0));

    // in the second row, the hasher state is set to hashes of (first_span, bar_call)
    let first_span_hash: Word = first_span.hash().into();
    let bar_call_hash: Word = bar_call.hash().into();
    assert_eq!(first_span_hash, get_hasher_state1(&dec_trace, 1));
    assert_eq!(bar_call_hash, get_hasher_state2(&dec_trace, 1));

    // at the end of the first SPAN, the hasher state is set to the hash of the first child
    assert_eq!(first_span_hash, get_hasher_state1(&dec_trace, 6));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&dec_trace, 6));

    // in the 7th row, we start the CALL block which has bar_join as its only child
    let bar_root_hash: Word = bar_root.hash().into();
    assert_eq!(bar_root_hash, get_hasher_state1(&dec_trace, 7));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&dec_trace, 7));

    // in the 8th row, the hasher state is set to hashes of (bar_span, foo_call)
    let bar_span_hash: Word = bar_span.hash().into();
    let foo_call_hash: Word = foo_call.hash().into();
    assert_eq!(bar_span_hash, get_hasher_state1(&dec_trace, 8));
    assert_eq!(foo_call_hash, get_hasher_state2(&dec_trace, 8));

    // at the end of the bar_span, the hasher state is set to the hash of the first child
    assert_eq!(bar_span_hash, get_hasher_state1(&dec_trace, 12));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&dec_trace, 12));

    // in the 13th row, we start the SYSCALL block which has foo_span as its only child
    let foo_root_hash: Word = foo_root.hash().into();
    assert_eq!(foo_root_hash, get_hasher_state1(&dec_trace, 13));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&dec_trace, 13));

    // at the end of the foo_span_hash, the hasher state is set to the hash of the first child
    assert_eq!(foo_root_hash, get_hasher_state1(&dec_trace, 17));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&dec_trace, 17));

    // SYSCALL block ends in the 18th row; the last element of the hasher state
    // is set to ONE because we are exiting a SYSCALL block
    assert_eq!(foo_call_hash, get_hasher_state1(&dec_trace, 18));
    assert_eq!([ZERO, ZERO, ZERO, ONE], get_hasher_state2(&dec_trace, 18));

    // internal bar_join block ends in the 19th row
    assert_eq!(bar_root_hash, get_hasher_state1(&dec_trace, 19));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&dec_trace, 19));

    // CALL block ends in the 20th row; the second to last element of the hasher state
    // is set to ONE because we are exiting a CALL block
    assert_eq!(bar_call_hash, get_hasher_state1(&dec_trace, 20));
    assert_eq!([ZERO, ZERO, ONE, ZERO], get_hasher_state2(&dec_trace, 20));

    // internal JOIN block ends in the 21st row
    assert_eq!(inner_join_hash, get_hasher_state1(&dec_trace, 21));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&dec_trace, 21));

    // last span ends in the 24th row
    assert_eq!(last_span_hash, get_hasher_state1(&dec_trace, 24));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&dec_trace, 24));

    // the program ends in the 25th row
    let program_hash: Word = program.hash().into();
    assert_eq!(program_hash, get_hasher_state1(&dec_trace, 25));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&dec_trace, 25));

    // HALT opcode and program hash gets propagated to the last row
    for i in 26..trace_len {
        assert!(contains_op(&dec_trace, i, Operation::Halt));
        assert_eq!(ONE, dec_trace[OP_BIT_EXTRA_COL_IDX][i]);
        assert_eq!(program_hash, get_hasher_state1(&dec_trace, i));
    }

    // --- check the ctx column -------------------------------------------------------------------

    // for the first 7 cycles, we are in the root context
    for i in 0..8 {
        assert_eq!(sys_trace[CTX_COL_IDX][i], ZERO);
    }

    // when CALL operation is executed, we switch to the new context; the ID of this context is 8
    // because we switch to it at the 8th cycle
    for i in 8..14 {
        assert_eq!(sys_trace[CTX_COL_IDX][i], EIGHT);
    }

    // when SYSCALL operation is executed, we switch back to the root context (0)
    for i in 14..18 {
        assert_eq!(sys_trace[CTX_COL_IDX][i], ZERO);
    }

    // when SYSCALL ends, we return to the context of the CALL block
    for i in 19..21 {
        assert_eq!(sys_trace[CTX_COL_IDX][i], EIGHT);
    }

    // once the CALL block exited, we go back to the root context
    for i in 21..trace_len {
        assert_eq!(sys_trace[CTX_COL_IDX][i], ZERO);
    }

    // --- check the fmp column -------------------------------------------------------------------

    // for the first 5 cycles fmp stays at initial value
    for i in 0..5 {
        assert_eq!(sys_trace[FMP_COL_IDX][i], FMP_MIN);
    }

    // when the first FmpUpdate is executed, fmp gets gets incremented by 1
    for i in 5..8 {
        assert_eq!(sys_trace[FMP_COL_IDX][i], FMP_MIN + ONE);
    }

    // when CALL operation is executed, fmp gets reset to the initial value
    for i in 8..12 {
        assert_eq!(sys_trace[FMP_COL_IDX][i], FMP_MIN);
    }

    // when the second FmpUpdate is executed, fmp gets gets incremented by 2
    for i in 12..14 {
        assert_eq!(sys_trace[FMP_COL_IDX][i], FMP_MIN + TWO);
    }

    // when SYSCALL operation is executed, fmp gets reset to the initial value for syscalls
    for i in 14..17 {
        assert_eq!(sys_trace[FMP_COL_IDX][i], SYSCALL_FMP_MIN);
    }

    // when the third FmpUpdate is executed, fmp gets gets incremented by 3
    for i in 17..19 {
        assert_eq!(sys_trace[FMP_COL_IDX][i], SYSCALL_FMP_MIN + THREE);
    }

    // once the SYSCALL block exited, fmp gets reset back to FMP_MIN + 2
    for i in 19..21 {
        assert_eq!(sys_trace[FMP_COL_IDX][i], FMP_MIN + TWO);
    }

    // once the CALL block exited, fmp gets reset back to FMP_MIN + 1, and it remains unchanged
    // until the end of the trace
    for i in 21..trace_len {
        assert_eq!(sys_trace[FMP_COL_IDX][i], FMP_MIN + ONE);
    }

    // --- check the is_syscall column ------------------------------------------------------------

    // before the SYSCALL block, syscall flag values should be set to 0
    for i in 0..14 {
        assert_eq!(sys_trace[IN_SYSCALL_COL_IDX][i], ZERO);
    }

    // within the SYSCALL block, syscall flag values should be set to 1
    for i in 14..19 {
        assert_eq!(sys_trace[IN_SYSCALL_COL_IDX][i], ONE);
    }

    // after the SYSCALL block, syscall flag values should be set to 0 again
    for i in 19..trace_len {
        assert_eq!(sys_trace[IN_SYSCALL_COL_IDX][i], ZERO);
    }

    // --- check fn hash columns ------------------------------------------------------------------

    // before the CALL operation is executed, we are in a root context and thus fn_hash is ZEROs.
    for i in 0..8 {
        assert_eq!(get_fn_hash(&sys_trace, i), [ZERO; 4]);
    }

    // inside the CALL block (and the invoked from it SYSCALL block), fn hash is set to the hash
    // of the bar procedure
    for i in 8..21 {
        assert_eq!(get_fn_hash(&sys_trace, i), bar_root_hash);
    }

    // after the CALL block is ended, we are back in the root context
    for i in 21..trace_len {
        assert_eq!(get_fn_hash(&sys_trace, i), [ZERO; 4]);
    }

    // --- check block execution hints ------------------------------------------------------------
    let expected_hints = vec![
        (0, BlockTableUpdate::BlockStarted(2)),    // join0
        (1, BlockTableUpdate::BlockStarted(2)),    // join1
        (2, BlockTableUpdate::BlockStarted(0)),    // span0
        (6, BlockTableUpdate::BlockEnded(true)),   // end span0
        (7, BlockTableUpdate::BlockStarted(1)),    // call
        (8, BlockTableUpdate::BlockStarted(2)),    // join2
        (9, BlockTableUpdate::BlockStarted(0)),    // span1
        (12, BlockTableUpdate::BlockEnded(true)),  // end span1
        (13, BlockTableUpdate::BlockStarted(1)),   // syscall
        (14, BlockTableUpdate::BlockStarted(0)),   // span2
        (17, BlockTableUpdate::BlockEnded(false)), // end span2
        (18, BlockTableUpdate::BlockEnded(false)), // end syscall
        (19, BlockTableUpdate::BlockEnded(false)), // end join2
        (20, BlockTableUpdate::BlockEnded(false)), // end join1
        (21, BlockTableUpdate::BlockEnded(true)),  // end join0
        (22, BlockTableUpdate::BlockStarted(0)),   // span3
        (24, BlockTableUpdate::BlockEnded(false)), // end span3
        (25, BlockTableUpdate::BlockEnded(false)), // end program
    ];
    assert_eq!(expected_hints, aux_hints.block_exec_hints());

    // --- check block stack table rows -----------------------------------------------------------
    let call_ctx =
        ExecutionContextInfo::new(0, [ZERO; 4], FMP_MIN + ONE, 17, overflow_addr_after_pad);
    let syscall_ctx = ExecutionContextInfo::new(8, bar_root_hash, FMP_MIN + TWO, 16, ZERO);
    let expected_rows = vec![
        BlockStackTableRow::new_test(INIT_ADDR, ZERO, false),
        BlockStackTableRow::new_test(inner_join_addr, INIT_ADDR, false),
        BlockStackTableRow::new_test(first_span_addr, inner_join_addr, false),
        BlockStackTableRow::new_test_with_ctx(call_addr, inner_join_addr, false, call_ctx),
        BlockStackTableRow::new_test(bar_join_addr, call_addr, false),
        BlockStackTableRow::new_test(bar_span_addr, bar_join_addr, false),
        BlockStackTableRow::new_test_with_ctx(syscall_addr, bar_join_addr, false, syscall_ctx),
        BlockStackTableRow::new_test(syscall_span_addr, syscall_addr, false),
        BlockStackTableRow::new_test(last_span_addr, INIT_ADDR, false),
    ];
    assert_eq!(expected_rows, aux_hints.block_stack_table_rows());

    // --- check block hash table hints ----------------------------------------------------------
    let expected_rows = vec![
        BlockHashTableRow::from_program_hash(program_hash),
        BlockHashTableRow::new_test(INIT_ADDR, inner_join_hash, true, false),
        BlockHashTableRow::new_test(INIT_ADDR, last_span_hash, false, false),
        BlockHashTableRow::new_test(inner_join_addr, first_span_hash, true, false),
        BlockHashTableRow::new_test(inner_join_addr, bar_call_hash, false, false),
        BlockHashTableRow::new_test(call_addr, bar_root_hash, false, false),
        BlockHashTableRow::new_test(bar_join_addr, bar_span_hash, true, false),
        BlockHashTableRow::new_test(bar_join_addr, foo_call_hash, false, false),
        BlockHashTableRow::new_test(syscall_addr, foo_root_hash, false, false),
    ];
    assert_eq!(expected_rows, aux_hints.block_hash_table_rows());
}

// HELPER REGISTERS TESTS
// ================================================================================================
#[test]
fn set_user_op_helpers_many() {
    // --- user operation with 4 helper values ----------------------------------------------------
    let program = CodeBlock::new_span(vec![Operation::U32div]);
    let a = rand_value::<u32>();
    let b = rand_value::<u32>();
    let (dividend, divisor) = if a > b { (a, b) } else { (b, a) };
    let (trace, ..) = build_trace(&[dividend as u64, divisor as u64], &program);
    let hasher_state = get_hasher_state(&trace, 1);

    // Check the hasher state of the user operation which was executed.
    // h2 to h5 are expected to hold the values for range checks.
    let quot = dividend / divisor;
    let rem = dividend - quot * divisor;
    let check_1 = dividend - quot;
    let check_2 = divisor - rem - 1;
    let expected = build_expected_hasher_state(&[
        ZERO,
        ZERO,
        Felt::new((check_1 as u16).into()),
        Felt::new(((check_1 >> 16) as u16).into()),
        Felt::new((check_2 as u16).into()),
        Felt::new(((check_2 >> 16) as u16).into()),
    ]);

    assert_eq!(expected, hasher_state);
}

// HELPER FUNCTIONS
// ================================================================================================

fn build_trace(stack_inputs: &[u64], program: &CodeBlock) -> (DecoderTrace, AuxTraceHints, usize) {
    let stack_inputs = StackInputs::try_from_values(stack_inputs.iter().copied()).unwrap();
    let advice_provider = MemAdviceProvider::default();
    let mut process = Process::new(Kernel::default(), stack_inputs, advice_provider);
    process.execute_code_block(program, &CodeBlockTable::default()).unwrap();

    let (trace, aux_hints) = ExecutionTrace::test_finalize_trace(process);
    let trace_len = get_trace_len(&trace) - ExecutionTrace::NUM_RAND_ROWS;

    (
        trace[DECODER_TRACE_RANGE]
            .to_vec()
            .try_into()
            .expect("failed to convert vector to array"),
        aux_hints.decoder,
        trace_len,
    )
}

fn build_call_trace(
    program: &CodeBlock,
    fn_block: CodeBlock,
    kernel_proc: Option<CodeBlock>,
) -> (SystemTrace, DecoderTrace, AuxTraceHints, usize) {
    let kernel = match kernel_proc {
        Some(ref proc) => Kernel::new(&[proc.hash()]),
        None => Kernel::default(),
    };
    let advice_provider = MemAdviceProvider::default();
    let stack_inputs = crate::StackInputs::default();
    let mut process = Process::new(kernel, stack_inputs, advice_provider);

    // build code block table
    let mut cb_table = CodeBlockTable::default();
    cb_table.insert(fn_block);
    if let Some(proc) = kernel_proc {
        cb_table.insert(proc);
    }

    process.execute_code_block(program, &cb_table).unwrap();

    let (trace, aux_hints) = ExecutionTrace::test_finalize_trace(process);
    let trace_len = get_trace_len(&trace) - ExecutionTrace::NUM_RAND_ROWS;

    let sys_trace = trace[SYS_TRACE_RANGE]
        .to_vec()
        .try_into()
        .expect("failed to convert vector to array");

    let decoder_trace = trace[DECODER_TRACE_RANGE]
        .to_vec()
        .try_into()
        .expect("failed to convert vector to array");

    (sys_trace, decoder_trace, aux_hints.decoder, trace_len)
}

// OPCODES
// ------------------------------------------------------------------------------------------------

fn check_op_decoding(
    trace: &DecoderTrace,
    row_idx: usize,
    addr: Felt,
    op: Operation,
    group_count: u64,
    op_idx: u64,
    in_span: u64,
) {
    let opcode = read_opcode(trace, row_idx);

    assert_eq!(trace[ADDR_COL_IDX][row_idx], addr);
    assert_eq!(op.op_code(), opcode);
    assert_eq!(trace[IN_SPAN_COL_IDX][row_idx], Felt::new(in_span));
    assert_eq!(trace[GROUP_COUNT_COL_IDX][row_idx], Felt::new(group_count));
    assert_eq!(trace[OP_INDEX_COL_IDX][row_idx], Felt::new(op_idx));

    let expected_batch_flags = if op == Operation::Span || op == Operation::Respan {
        let num_groups = core::cmp::min(OP_BATCH_SIZE, group_count as usize);
        build_op_batch_flags(num_groups)
    } else {
        [ZERO, ZERO, ZERO]
    };

    for (i, flag_value) in OP_BATCH_FLAGS_RANGE.zip(expected_batch_flags) {
        assert_eq!(trace[i][row_idx], flag_value);
    }

    // make sure op bit extra column is set to the product of the two most significant opcode bits
    let bit6 = Felt::from((opcode >> 6) & 1);
    let bit5 = Felt::from((opcode >> 5) & 1);
    assert_eq!(trace[OP_BIT_EXTRA_COL_IDX][row_idx], bit6 * bit5);
}

fn contains_op(trace: &DecoderTrace, row_idx: usize, op: Operation) -> bool {
    op.op_code() == read_opcode(trace, row_idx)
}

fn read_opcode(trace: &DecoderTrace, row_idx: usize) -> u8 {
    let mut result = 0;
    for (i, column) in trace.iter().skip(OP_BITS_OFFSET).take(NUM_OP_BITS).enumerate() {
        let op_bit = column[row_idx].as_int();
        assert!(op_bit <= 1, "invalid op bit");
        result += op_bit << i;
    }
    result as u8
}

fn build_op_batch_flags(num_groups: usize) -> [Felt; NUM_OP_BATCH_FLAGS] {
    match num_groups {
        1 => OP_BATCH_1_GROUPS,
        2 => OP_BATCH_2_GROUPS,
        4 => OP_BATCH_4_GROUPS,
        8 => OP_BATCH_8_GROUPS,
        _ => panic!("invalid num groups: {num_groups}"),
    }
}

// SYSTEM REGISTERS
// ------------------------------------------------------------------------------------------------

fn get_fn_hash(trace: &SystemTrace, row_idx: usize) -> Word {
    let mut result = [ZERO; 4];
    let trace = &trace[FN_HASH_RANGE];
    for (element, column) in result.iter_mut().zip(trace) {
        *element = column[row_idx];
    }
    result
}

// HASHER STATE
// ------------------------------------------------------------------------------------------------

fn check_hasher_state(trace: &DecoderTrace, expected: Vec<Vec<Felt>>) {
    for (i, expected) in expected.iter().enumerate() {
        let expected = build_expected_hasher_state(expected);
        assert_eq!(expected, get_hasher_state(trace, i));
    }
}

fn get_hasher_state(trace: &DecoderTrace, row_idx: usize) -> [Felt; NUM_HASHER_COLUMNS] {
    let mut result = [ZERO; NUM_HASHER_COLUMNS];
    for (result, column) in result.iter_mut().zip(trace[HASHER_STATE_RANGE].iter()) {
        *result = column[row_idx];
    }
    result
}

fn get_hasher_state1(trace: &DecoderTrace, row_idx: usize) -> Word {
    let mut result = [ZERO; 4];
    for (result, column) in result.iter_mut().zip(trace[HASHER_STATE_RANGE].iter()) {
        *result = column[row_idx];
    }
    result
}

fn get_hasher_state2(trace: &DecoderTrace, row_idx: usize) -> Word {
    let mut result = [ZERO; 4];
    for (result, column) in result.iter_mut().zip(trace[HASHER_STATE_RANGE].iter().skip(4)) {
        *result = column[row_idx];
    }
    result
}

fn build_expected_hasher_state(values: &[Felt]) -> [Felt; NUM_HASHER_COLUMNS] {
    let mut result = [ZERO; NUM_HASHER_COLUMNS];
    for (i, value) in values.iter().enumerate() {
        result[i] = *value;
    }
    result
}
