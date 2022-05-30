use super::{super::DecoderTrace, Felt, Operation, Word, HASHER_WIDTH, NUM_OP_BITS};
use crate::{ExecutionTrace, Process, ProgramInputs};
use core::ops::Range;
use vm_core::{
    program::blocks::CodeBlock, utils::range, FieldElement, StarkField, DECODER_TRACE_RANGE,
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

const HASHER_STATE_OFFSET: usize = 9;
const HASHER_STATE_RANGE: Range<usize> = range(HASHER_STATE_OFFSET, HASHER_WIDTH);

// SPAN BLOCK TESTS
// ================================================================================================

#[test]
fn span_block_small() {
    let program = CodeBlock::new_span(vec![
        Operation::Push(Felt::new(1)),
        Operation::Push(Felt::new(2)),
        Operation::Add,
    ]);

    let (trace, _trace_len) = build_trace(&[], &program);

    // --- test op bits columns -------------------------------------------------------------------
    assert!(contains_op(&trace, 0, Operation::Span));
    assert!(contains_op(&trace, 1, Operation::Push(Felt::new(1))));
    assert!(contains_op(&trace, 2, Operation::Push(Felt::new(2))));
    assert!(contains_op(&trace, 3, Operation::Add));
    assert!(contains_op(&trace, 4, Operation::Noop));
    assert!(contains_op(&trace, 5, Operation::End));
}

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
    ]);

    let (trace, _trace_len) = build_trace(&[], &program);

    // --- test op bits columns -----------------------------------------------
    assert!(contains_op(&trace, 0, Operation::Span));
}

// JOIN BLOCK TESTS
// ================================================================================================

#[test]
fn join_block() {
    let span1 = CodeBlock::new_span(vec![Operation::Mul]);
    let span2 = CodeBlock::new_span(vec![Operation::Add]);
    let program = CodeBlock::new_join([span1.clone(), span2.clone()]);

    let (trace, trace_len) = build_trace(&[], &program);

    // --- check block address column -------------------------------------------------------------
    let init_addr = ZERO;

    assert_eq!(trace[ADDR_IDX][0], ZERO);
    assert_eq!(trace[ADDR_IDX][1], init_addr); // SPAN
    assert_eq!(trace[ADDR_IDX][2], init_addr + Felt::new(8));
    assert_eq!(trace[ADDR_IDX][3], init_addr + Felt::new(8));
    assert_eq!(trace[ADDR_IDX][4], init_addr); // SPAN
    assert_eq!(trace[ADDR_IDX][5], init_addr + Felt::new(16));
    assert_eq!(trace[ADDR_IDX][6], init_addr + Felt::new(16));
    assert_eq!(trace[ADDR_IDX][7], init_addr);
    for i in 8..trace_len {
        assert_eq!(trace[ADDR_IDX][i], ZERO);
    }

    // --- test op bits columns -------------------------------------------------------------------

    // opcodes should be: JOIN SPAN MUL END SPAN ADD END END
    assert!(contains_op(&trace, 0, Operation::Join));
    assert!(contains_op(&trace, 1, Operation::Span));
    assert!(contains_op(&trace, 2, Operation::Mul));
    assert!(contains_op(&trace, 3, Operation::End));
    assert!(contains_op(&trace, 4, Operation::Span));
    assert!(contains_op(&trace, 5, Operation::Add));
    assert!(contains_op(&trace, 6, Operation::End));
    assert!(contains_op(&trace, 7, Operation::End));
    for i in 8..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
    }

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

    let (trace, trace_len) = build_trace(&[1], &program);

    // --- check block address column -------------------------------------------------------------
    let init_addr = ZERO;

    assert_eq!(trace[ADDR_IDX][0], ZERO);
    assert_eq!(trace[ADDR_IDX][1], init_addr); // SPAN
    assert_eq!(trace[ADDR_IDX][2], init_addr + Felt::new(8));
    assert_eq!(trace[ADDR_IDX][3], init_addr + Felt::new(8));
    assert_eq!(trace[ADDR_IDX][4], init_addr);
    for i in 5..trace_len {
        assert_eq!(trace[ADDR_IDX][i], ZERO);
    }

    // --- test op bits columns -------------------------------------------------------------------

    // opcodes should be: SPLIT SPAN MUL END END
    assert!(contains_op(&trace, 0, Operation::Split));
    assert!(contains_op(&trace, 1, Operation::Span));
    assert!(contains_op(&trace, 2, Operation::Mul));
    assert!(contains_op(&trace, 3, Operation::End));
    assert!(contains_op(&trace, 4, Operation::End));
    for i in 5..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
    }

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

    let (trace, trace_len) = build_trace(&[0], &program);

    // --- check block address column -------------------------------------------------------------
    let init_addr = ZERO;

    assert_eq!(trace[ADDR_IDX][0], ZERO);
    assert_eq!(trace[ADDR_IDX][1], init_addr); // SPAN
    assert_eq!(trace[ADDR_IDX][2], init_addr + Felt::new(8));
    assert_eq!(trace[ADDR_IDX][3], init_addr + Felt::new(8));
    assert_eq!(trace[ADDR_IDX][4], init_addr);
    for i in 5..trace_len {
        assert_eq!(trace[ADDR_IDX][i], ZERO);
    }

    // --- test op bits columns -------------------------------------------------------------------

    // opcodes should be: SPLIT SPAN MUL END END
    assert!(contains_op(&trace, 0, Operation::Split));
    assert!(contains_op(&trace, 1, Operation::Span));
    assert!(contains_op(&trace, 2, Operation::Add));
    assert!(contains_op(&trace, 3, Operation::End));
    assert!(contains_op(&trace, 4, Operation::End));
    for i in 5..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
    }

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

    let (trace, trace_len) = build_trace(&[0, 1], &program);

    // --- check block address column -------------------------------------------------------------
    let init_addr = ZERO;

    assert_eq!(trace[ADDR_IDX][0], ZERO);
    assert_eq!(trace[ADDR_IDX][1], init_addr);
    assert_eq!(trace[ADDR_IDX][2], init_addr + Felt::new(8));
    assert_eq!(trace[ADDR_IDX][3], init_addr + Felt::new(8));
    assert_eq!(trace[ADDR_IDX][4], init_addr + Felt::new(8));
    assert_eq!(trace[ADDR_IDX][5], init_addr);
    for i in 6..trace_len {
        assert_eq!(trace[ADDR_IDX][i], ZERO);
    }

    // --- check op bits columns ------------------------------------------------------------------

    assert!(contains_op(&trace, 0, Operation::Loop));
    assert!(contains_op(&trace, 1, Operation::Span));
    assert!(contains_op(&trace, 2, Operation::Pad));
    assert!(contains_op(&trace, 3, Operation::Drop));
    assert!(contains_op(&trace, 4, Operation::End));
    assert!(contains_op(&trace, 5, Operation::End));
    for i in 6..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
    }

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

    let (trace, trace_len) = build_trace(&[0], &program);

    // --- check block address column -------------------------------------------------------------
    let init_addr = ZERO;

    assert_eq!(trace[ADDR_IDX][0], ZERO);
    assert_eq!(trace[ADDR_IDX][1], init_addr);
    assert_eq!(trace[ADDR_IDX][2], init_addr);
    for i in 2..trace_len {
        assert_eq!(trace[ADDR_IDX][i], ZERO);
    }

    // --- test op bits columns -------------------------------------------------------------------
    assert!(contains_op(&trace, 0, Operation::Loop));
    assert!(contains_op(&trace, 1, Operation::End));
    for i in 2..trace_len {
        assert!(contains_op(&trace, i, Operation::Halt));
    }

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

    let (trace, trace_len) = build_trace(&[0, 1, 1], &program);

    // --- check block address column -------------------------------------------------------------
    let init_addr = ZERO;

    assert_eq!(trace[ADDR_IDX][0], ZERO);
    assert_eq!(trace[ADDR_IDX][1], init_addr);
    assert_eq!(trace[ADDR_IDX][2], init_addr + Felt::new(8));
    assert_eq!(trace[ADDR_IDX][3], init_addr + Felt::new(8));
    assert_eq!(trace[ADDR_IDX][4], init_addr + Felt::new(8));
    assert_eq!(trace[ADDR_IDX][5], init_addr); // REPEAT
    assert_eq!(trace[ADDR_IDX][6], init_addr);
    assert_eq!(trace[ADDR_IDX][7], init_addr + Felt::new(16));
    assert_eq!(trace[ADDR_IDX][8], init_addr + Felt::new(16));
    assert_eq!(trace[ADDR_IDX][9], init_addr + Felt::new(16));
    assert_eq!(trace[ADDR_IDX][10], init_addr);
    for i in 11..trace_len {
        assert_eq!(trace[ADDR_IDX][i], ZERO);
    }

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
