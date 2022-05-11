use super::{super::DecoderTrace, Felt, Operation, NUM_OP_BITS};
use crate::{ExecutionTrace, Process, ProgramInputs};
use core::ops::Range;
use vm_core::{
    program::blocks::CodeBlock, utils::range, FieldElement, StarkField, DECODER_TRACE_RANGE,
};

// CONSTANTS
// ================================================================================================

/// TODO: move to core?
const OP_BITS_OFFSET: usize = 1;
const OP_BITS_RANGE: Range<usize> = range(OP_BITS_OFFSET, NUM_OP_BITS);

const IN_SPAN_IDX: usize = 8;

// TESTS
// ================================================================================================

#[test]
fn join_block() {
    let value1 = Felt::new(3);
    let value2 = Felt::new(5);
    let span1 = CodeBlock::new_span(vec![Operation::Push(value1), Operation::Mul]);
    let span2 = CodeBlock::new_span(vec![Operation::Push(value2), Operation::Add]);
    let program = CodeBlock::new_join([span1, span2]);

    let (trace, _trace_len) = build_trace(&[], &program);

    // --- test op bits columns -----------------------------------------------

    // opcodes should be: JOIN SPAN PUSH END SPAN DROP END END
    assert!(contains_op(&trace, 0, Operation::Join));
    assert!(contains_op(&trace, 1, Operation::Span));
    assert!(contains_op(&trace, 2, Operation::Push(value1)));
    assert!(contains_op(&trace, 3, Operation::Mul));
    assert!(contains_op(&trace, 4, Operation::End));
    assert!(contains_op(&trace, 5, Operation::Span));
    assert!(contains_op(&trace, 6, Operation::Push(value2)));
    assert!(contains_op(&trace, 7, Operation::Add));
    assert!(contains_op(&trace, 8, Operation::End));
    assert!(contains_op(&trace, 9, Operation::End));
}

// SPAN BLOCK TESTS
// ================================================================================================

#[test]
fn span_block() {
    let program = CodeBlock::new_span(vec![
        Operation::Push(Felt::new(1)),
        Operation::Push(Felt::new(2)),
        Operation::Push(Felt::new(3)),
        Operation::Pad,
        Operation::Mul,
        Operation::Add,
        Operation::Drop,
        Operation::Push(Felt::new(4)),
        Operation::Push(Felt::new(5)),
        Operation::Mul,
        Operation::Add,
        Operation::Inv,
    ]);

    let (trace, trace_len) = build_trace(&[], &program);

    for i in 0..20 {
        print_row(&trace, i);
    }

    // --- test op bits columns -----------------------------------------------
    // two NOOPs are inserted by the processor:
    // - after PUSH(4) to make sure the first group doesn't end with a PUSH
    // - before the END to pad the last group with a single NOOP
    assert!(contains_op(&trace, 0, Operation::Span));
    assert!(contains_op(&trace, 1, Operation::Push(Felt::new(1))));
    assert!(contains_op(&trace, 2, Operation::Push(Felt::new(2))));
    assert!(contains_op(&trace, 3, Operation::Push(Felt::new(3))));
    assert!(contains_op(&trace, 4, Operation::Pad));
    assert!(contains_op(&trace, 5, Operation::Mul));
    assert!(contains_op(&trace, 6, Operation::Add));
    assert!(contains_op(&trace, 7, Operation::Drop));
    assert!(contains_op(&trace, 8, Operation::Push(Felt::new(4))));
    assert!(contains_op(&trace, 9, Operation::Noop));
    assert!(contains_op(&trace, 10, Operation::Push(Felt::new(5))));
    assert!(contains_op(&trace, 11, Operation::Mul));
    assert!(contains_op(&trace, 12, Operation::Add));
    assert!(contains_op(&trace, 13, Operation::Inv));
    assert!(contains_op(&trace, 14, Operation::Noop));
    assert!(contains_op(&trace, 15, Operation::End));
    for i in 16..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
    }

    // --- test is_span column ------------------------------------------------
    assert_eq!(Felt::ZERO, trace[IN_SPAN_IDX][0]);
    for i in 1..15 {
        assert_eq!(Felt::ONE, trace[IN_SPAN_IDX][i]);
    }
    for i in 15..trace_len {
        assert_eq!(Felt::ZERO, trace[IN_SPAN_IDX][i]);
    }
}

#[test]
fn span_block_with_respan() {
    let program = CodeBlock::new_span(vec![
        Operation::Pad,
        Operation::Pow2,
        Operation::Push(Felt::new(1)),
        Operation::Pow2,
        Operation::Push(Felt::new(2)),
        Operation::Pow2,
        Operation::Push(Felt::new(3)),
        Operation::Pow2,
        Operation::Push(Felt::new(4)),
        Operation::Pow2,
        Operation::Push(Felt::new(5)),
        Operation::Pow2,
        Operation::Push(Felt::new(6)),
        Operation::Pow2,
        Operation::Push(Felt::new(7)),
        Operation::Pow2,
        Operation::Push(Felt::new(8)),
        Operation::Pow2,
        Operation::Push(Felt::new(9)),
        Operation::Pow2,
        Operation::Push(Felt::new(63)),
        Operation::Pow2,
    ]);

    let (trace, _trace_len) = build_trace(&[], &program);

    // --- test op bits columns -----------------------------------------------
    assert!(contains_op(&trace, 0, Operation::Span));
}

// LOOP BLOCK TESTS
// ================================================================================================

#[test]
fn loop_block() {
    let loop_body = CodeBlock::new_span(vec![Operation::Pad, Operation::Drop]);
    let program = CodeBlock::new_loop(loop_body);

    let (trace, trace_len) = build_trace(&[0, 1], &program);

    // --- test op bits columns -----------------------------------------------
    assert!(contains_op(&trace, 0, Operation::Loop));
    assert!(contains_op(&trace, 1, Operation::Span));
    assert!(contains_op(&trace, 2, Operation::Pad));
    assert!(contains_op(&trace, 3, Operation::Drop));
    assert!(contains_op(&trace, 4, Operation::End));
    assert!(contains_op(&trace, 5, Operation::End));
    for i in 6..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
    }
}

#[test]
fn loop_block_skip() {
    let loop_body = CodeBlock::new_span(vec![Operation::Pad, Operation::Drop]);
    let program = CodeBlock::new_loop(loop_body);

    let (trace, trace_len) = build_trace(&[0], &program);

    // --- test op bits columns -----------------------------------------------
    assert!(contains_op(&trace, 0, Operation::Loop));
    assert!(contains_op(&trace, 1, Operation::End));
    for i in 2..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
    }
}

#[test]
fn loop_block_repeat() {
    let loop_body = CodeBlock::new_span(vec![Operation::Pad, Operation::Drop]);
    let program = CodeBlock::new_loop(loop_body);

    let (trace, trace_len) = build_trace(&[0, 1, 1], &program);

    // --- test op bits columns -----------------------------------------------
    assert!(contains_op(&trace, 0, Operation::Loop));
    assert!(contains_op(&trace, 1, Operation::Span));
    assert!(contains_op(&trace, 2, Operation::Pad));
    assert!(contains_op(&trace, 3, Operation::Drop));
    assert!(contains_op(&trace, 4, Operation::End));
    assert!(contains_op(&trace, 5, Operation::Repeat));
    assert!(contains_op(&trace, 6, Operation::Span));
    assert!(contains_op(&trace, 7, Operation::Pad));
    assert!(contains_op(&trace, 8, Operation::Drop));
    assert!(contains_op(&trace, 9, Operation::End));
    assert!(contains_op(&trace, 10, Operation::End));
    for i in 11..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
    }

    //for i in 0..12 {
    //    print_row(&trace, i);
    //}
    //assert!(false, "all good!");
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
