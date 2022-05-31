use super::{super::DecoderTrace, Felt, Operation, Word, HASHER_WIDTH, NUM_OP_BITS};
use crate::{ExecutionTrace, Process, ProgramInputs};
use core::ops::Range;
use vm_core::{
    program::blocks::{CodeBlock, Span},
    utils::range,
    FieldElement, StarkField, DECODER_TRACE_RANGE,
};

// CONSTANTS
// ================================================================================================

const ONE: Felt = Felt::ONE;
const ZERO: Felt = Felt::ZERO;

const ADDR_IDX: usize = 0;

/// TODO: move to core?
const OP_BITS_OFFSET: usize = 1;
const OP_BITS_RANGE: Range<usize> = range(OP_BITS_OFFSET, NUM_OP_BITS);

const IN_SPAN_IDX: usize = 8;
const GROUP_COUNT_IDX: usize = 17;
const OP_INDEX_IDX: usize = 18;

const HASHER_STATE_OFFSET: usize = 9;
const HASHER_STATE_RANGE: Range<usize> = range(HASHER_STATE_OFFSET, HASHER_WIDTH);

const INIT_ADDR: Felt = Felt::ZERO;

// SPAN BLOCK TESTS
// ================================================================================================

#[test]
fn span_block_one_group() {
    let ops = vec![Operation::Pad, Operation::Add, Operation::Mul];
    let span = Span::new(ops.clone());
    let program = CodeBlock::new_span(ops.clone());

    let (trace, _trace_len) = build_trace(&[], &program);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    check_op_decoding(&trace, 0, ZERO, Operation::Span, 8, 0, 0);
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
            vec![build_group(&ops[1..])],
            vec![build_group(&ops[2..])],
            vec![],
            program_hash.to_vec(), // last row should contain program hash
        ],
    );
}

#[test]
fn span_block_small() {
    let iv = [Felt::new(1), Felt::new(2)];
    let ops = vec![
        Operation::Push(iv[0]),
        Operation::Push(iv[1]),
        Operation::Add,
    ];
    let span = Span::new(ops.clone());
    let program = CodeBlock::new_span(ops.clone());

    let (trace, _trace_len) = build_trace(&[], &program);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    check_op_decoding(&trace, 0, ZERO, Operation::Span, 8, 0, 0);
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
            vec![build_group(&ops[1..])],
            vec![build_group(&ops[2..])],
            vec![],
            vec![],
            program_hash.to_vec(), // last row should contain program hash
        ],
    );
}

#[test]
fn span_block() {
    let iv = [
        Felt::new(1),
        Felt::new(2),
        Felt::new(3),
        Felt::new(4),
        Felt::new(5),
    ];
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
    let (trace, _trace_len) = build_trace(&[], &program);

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
            vec![build_group(&ops[1..8])], // first group starts
            vec![build_group(&ops[2..8])],
            vec![build_group(&ops[3..8])],
            vec![build_group(&ops[4..8])],
            vec![build_group(&ops[5..8])],
            vec![build_group(&ops[6..8])],
            vec![build_group(&ops[7..8])],
            vec![], // NOOP inserted after push
            vec![],
            vec![build_group(&ops[9..])], // next group starts
            vec![build_group(&ops[10..])],
            vec![build_group(&ops[11..])],
            vec![],
            vec![],                // a group with single NOOP added at the end
            program_hash.to_vec(), // last row should contain program hash
        ],
    );
}

#[test]
fn span_block_with_respan() {
    let iv = [
        Felt::new(1),
        Felt::new(2),
        Felt::new(3),
        Felt::new(4),
        Felt::new(5),
        Felt::new(6),
        Felt::new(7),
        Felt::new(8),
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
    let (trace, _trace_len) = build_trace(&[], &program);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    check_op_decoding(&trace, 0, ZERO, Operation::Span, 16, 0, 0);
    check_op_decoding(&trace, 1, INIT_ADDR, Operation::Push(iv[0]), 15, 0, 1);
    check_op_decoding(&trace, 2, INIT_ADDR, Operation::Push(iv[1]), 14, 1, 1);
    check_op_decoding(&trace, 3, INIT_ADDR, Operation::Push(iv[2]), 13, 2, 1);
    check_op_decoding(&trace, 4, INIT_ADDR, Operation::Push(iv[3]), 12, 3, 1);
    check_op_decoding(&trace, 5, INIT_ADDR, Operation::Push(iv[4]), 11, 4, 1);
    check_op_decoding(&trace, 6, INIT_ADDR, Operation::Push(iv[5]), 10, 5, 1);
    check_op_decoding(&trace, 7, INIT_ADDR, Operation::Push(iv[6]), 9, 6, 1);
    // NOOP inserted by the processor to make sure the group doesn't end with a PUSH
    check_op_decoding(&trace, 8, INIT_ADDR, Operation::Noop, 8, 7, 1);
    // RESPAN since the previous batch is full
    let next_addr = INIT_ADDR + Felt::new(8);
    check_op_decoding(&trace, 9, INIT_ADDR, Operation::Respan, 8, 0, 1);
    check_op_decoding(&trace, 10, next_addr, Operation::Push(iv[7]), 3, 0, 1);
    check_op_decoding(&trace, 11, next_addr, Operation::Add, 2, 1, 1);
    check_op_decoding(&trace, 12, next_addr, Operation::Push(iv[8]), 2, 2, 1);
    // NOOP inserted by the processor to make sure the group doesn't end with a PUSH
    check_op_decoding(&trace, 13, next_addr, Operation::Noop, 1, 3, 1);
    // NOOP inserted by the processor to make sure the number of groups is a power of two
    check_op_decoding(&trace, 14, next_addr, Operation::Noop, 0, 0, 1);
    check_op_decoding(&trace, 15, next_addr, Operation::End, 0, 0, 0);
    check_op_decoding(&trace, 16, ZERO, Operation::Halt, 0, 0, 0);

    // --- check hasher state columns -------------------------------------------------------------
    let program_hash: Word = program.hash().into();
    check_hasher_state(
        &trace,
        vec![
            span.op_batches()[0].groups().to_vec(),
            vec![build_group(&ops[1..7])], // first group starts
            vec![build_group(&ops[2..7])],
            vec![build_group(&ops[3..7])],
            vec![build_group(&ops[4..7])],
            vec![build_group(&ops[5..7])],
            vec![build_group(&ops[6..7])],
            vec![],
            vec![], // a NOOP inserted after last PUSH
            span.op_batches()[1].groups().to_vec(),
            vec![build_group(&ops[8..])], // next group starts
            vec![build_group(&ops[9..])],
            vec![],
            vec![],                // a NOOP is inserted after last PUSH
            vec![],                // a group with single NOOP added at the end
            program_hash.to_vec(), // last row should contain program hash
        ],
    );
}

// JOIN BLOCK TESTS
// ================================================================================================

#[test]
fn join_block() {
    let span1 = CodeBlock::new_span(vec![Operation::Mul]);
    let span2 = CodeBlock::new_span(vec![Operation::Add]);
    let program = CodeBlock::new_join([span1.clone(), span2.clone()]);

    let (trace, _trace_len) = build_trace(&[], &program);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    check_op_decoding(&trace, 0, ZERO, Operation::Join, 0, 0, 0);
    // starting first span
    let span1_addr = INIT_ADDR + Felt::new(8);
    check_op_decoding(&trace, 1, INIT_ADDR, Operation::Span, 8, 0, 0);
    check_op_decoding(&trace, 2, span1_addr, Operation::Mul, 0, 0, 1);
    check_op_decoding(&trace, 3, span1_addr, Operation::End, 0, 0, 0);
    // starting second span
    let span2_addr = INIT_ADDR + Felt::new(16);
    check_op_decoding(&trace, 4, INIT_ADDR, Operation::Span, 8, 0, 0);
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
}

// SPLIT BLOCK TESTS
// ================================================================================================

#[test]
fn split_block_true() {
    let span1 = CodeBlock::new_span(vec![Operation::Mul]);
    let span2 = CodeBlock::new_span(vec![Operation::Add]);
    let program = CodeBlock::new_split(span1.clone(), span2.clone());

    let (trace, _trace_len) = build_trace(&[1], &program);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    let span_addr = INIT_ADDR + Felt::new(8);
    check_op_decoding(&trace, 0, ZERO, Operation::Split, 0, 0, 0);
    check_op_decoding(&trace, 1, INIT_ADDR, Operation::Span, 8, 0, 0);
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
}

#[test]
fn split_block_false() {
    let span1 = CodeBlock::new_span(vec![Operation::Mul]);
    let span2 = CodeBlock::new_span(vec![Operation::Add]);
    let program = CodeBlock::new_split(span1.clone(), span2.clone());

    let (trace, _trace_len) = build_trace(&[0], &program);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    let span_addr = INIT_ADDR + Felt::new(8);
    check_op_decoding(&trace, 0, ZERO, Operation::Split, 0, 0, 0);
    check_op_decoding(&trace, 1, INIT_ADDR, Operation::Span, 8, 0, 0);
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
}

// LOOP BLOCK TESTS
// ================================================================================================

#[test]
fn loop_block() {
    let loop_body = CodeBlock::new_span(vec![Operation::Pad, Operation::Drop]);
    let program = CodeBlock::new_loop(loop_body.clone());

    let (trace, _trace_len) = build_trace(&[0, 1], &program);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    let body_addr = INIT_ADDR + Felt::new(8);
    check_op_decoding(&trace, 0, ZERO, Operation::Loop, 0, 0, 0);
    check_op_decoding(&trace, 1, INIT_ADDR, Operation::Span, 8, 0, 0);
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
}

#[test]
fn loop_block_skip() {
    let loop_body = CodeBlock::new_span(vec![Operation::Pad, Operation::Drop]);
    let program = CodeBlock::new_loop(loop_body.clone());

    let (trace, _trace_len) = build_trace(&[0], &program);

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
    // we didn't entre the loop's body
    let program_hash: Word = program.hash().into();
    assert_eq!(program_hash, get_hasher_state1(&trace, 1));
    assert_eq!([ZERO, ZERO, ZERO, ZERO], get_hasher_state2(&trace, 1));
}

#[test]
fn loop_block_repeat() {
    let loop_body = CodeBlock::new_span(vec![Operation::Pad, Operation::Drop]);
    let program = CodeBlock::new_loop(loop_body.clone());

    let (trace, _trace_len) = build_trace(&[0, 1, 1], &program);

    // --- check block address, op_bits, group count, op_index, and in_span columns ---------------
    let iter1_addr = INIT_ADDR + Felt::new(8);
    let iter2_addr = INIT_ADDR + Felt::new(16);

    check_op_decoding(&trace, 0, ZERO, Operation::Loop, 0, 0, 0);
    check_op_decoding(&trace, 1, INIT_ADDR, Operation::Span, 8, 0, 0);
    check_op_decoding(&trace, 2, iter1_addr, Operation::Pad, 0, 0, 1);
    check_op_decoding(&trace, 3, iter1_addr, Operation::Drop, 0, 1, 1);
    check_op_decoding(&trace, 4, iter1_addr, Operation::End, 0, 0, 0);
    // start second iteration
    check_op_decoding(&trace, 5, INIT_ADDR, Operation::Repeat, 0, 0, 0);
    check_op_decoding(&trace, 6, INIT_ADDR, Operation::Span, 8, 0, 0);
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
}

// HELPER FUNCTIONS
// ================================================================================================

fn build_trace(stack: &[u64], program: &CodeBlock) -> (DecoderTrace, usize) {
    let inputs = ProgramInputs::new(stack, &[], vec![]).unwrap();
    let mut process = Process::new(inputs);
    process.execute_code_block(&program).unwrap();

    let trace = ExecutionTrace::test_finalize_trace(process);
    let trace_len = trace.len() - ExecutionTrace::NUM_RAND_ROWS;

    (
        trace[DECODER_TRACE_RANGE]
            .to_vec()
            .try_into()
            .expect("failed to convert vector to array"),
        trace_len,
    )
}

fn contains_op(trace: &DecoderTrace, row_idx: usize, op: Operation) -> bool {
    op.op_code().unwrap() == read_opcode(trace, row_idx)
}

fn read_opcode(trace: &DecoderTrace, row_idx: usize) -> u8 {
    let mut result = 0;
    for (i, column) in trace
        .iter()
        .skip(OP_BITS_OFFSET)
        .take(NUM_OP_BITS)
        .enumerate()
    {
        let op_bit = column[row_idx].as_int();
        assert!(op_bit <= 1, "invalid op bit");
        result += op_bit << i;
    }
    result as u8
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
    for (result, column) in result
        .iter_mut()
        .zip(trace[HASHER_STATE_RANGE].iter().skip(4))
    {
        *result = column[row_idx];
    }
    result
}

fn get_hasher_state(trace: &DecoderTrace, row_idx: usize) -> [Felt; 8] {
    let mut result = [ZERO; 8];
    for (result, column) in result.iter_mut().zip(trace[HASHER_STATE_RANGE].iter()) {
        *result = column[row_idx];
    }
    result
}

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

fn check_hasher_state(trace: &DecoderTrace, expected: Vec<Vec<Felt>>) {
    for (i, expected) in expected.iter().enumerate() {
        let expected = build_expected_hasher_state(expected);
        assert_eq!(expected, get_hasher_state(trace, i));
    }
}

fn build_expected_hasher_state(values: &[Felt]) -> [Felt; 8] {
    let mut result = [ZERO; 8];
    for (i, value) in values.iter().enumerate() {
        result[i] = *value;
    }
    result
}

fn check_op_decoding(
    trace: &DecoderTrace,
    row_idx: usize,
    addr: Felt,
    op: Operation,
    group_count: u64,
    op_idx: u64,
    in_span: u64,
) {
    assert_eq!(trace[ADDR_IDX][row_idx], addr);
    assert!(contains_op(&trace, row_idx, op));
    assert_eq!(trace[GROUP_COUNT_IDX][row_idx], Felt::new(group_count));
    assert_eq!(trace[OP_INDEX_IDX][row_idx], Felt::new(op_idx));
    assert_eq!(trace[IN_SPAN_IDX][row_idx], Felt::new(in_span));
}

#[allow(dead_code)]
fn print_row(trace: &DecoderTrace, idx: usize) {
    let mut row = Vec::new();
    for column in trace.iter() {
        row.push(column[idx].as_int());
    }
    println!(
        "{}\t{}\t{:?} {} {: <16x?} {: <16x?} {} {}",
        idx,
        row[0],
        &row[OP_BITS_RANGE],
        row[8],
        &row[9..13],
        &row[13..17],
        row[17],
        row[18]
    );
}
