use core::iter;

use vm_core::{code_blocks::CodeBlock, Felt, FieldElement, Operation::*};

use crate::{todo::SpanBuilder, AssemblerError};

// ARITHMETIC OPERATIONS
// ================================================================================================

pub(super) fn add_imm(
    imm: &Felt,
    span: &mut SpanBuilder,
) -> Result<Option<CodeBlock>, AssemblerError> {
    if imm == &Felt::ONE {
        span.add_op(Incr)
    } else {
        span.add_ops([Push(*imm), Add])
    }
}

pub(super) fn mul_imm(
    imm: &Felt,
    span: &mut SpanBuilder,
) -> Result<Option<CodeBlock>, AssemblerError> {
    if imm == &Felt::ONE {
        Ok(None)
    } else {
        span.add_ops([Push(*imm), Mul])
    }
}

pub(super) fn div_imm(
    imm: &Felt,
    span: &mut SpanBuilder,
) -> Result<Option<CodeBlock>, AssemblerError> {
    if imm == &Felt::ONE {
        Ok(None)
    } else {
        // TODO test if zero imm will panic this inversion
        span.add_ops([Push(imm.inv()), Mul])
    }
}

pub(super) fn pow2(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblerError> {
    let pre = [
        // push base 2 onto the stack: [exp, .....] -> [2, exp, ......]
        Push(2u64.into()),
        // introduce initial value of acc onto the stack: [2, exp, ....] -> [1, 2, exp, ....]
        Pad,
        Incr,
        // arrange the top of the stack for `EXPACC` instruction: [1, 2, exp, ....] -> [0, 2, 1, exp, ...]
        Swap,
        Pad,
    ];

    // calling expacc instruction 7 times.
    // TODO we are in fact calling it 6 times
    let expacc = iter::repeat(Expacc).take(6);

    // drop the top two elements bit and exp value of the latest bit.
    let drop = iter::repeat(Drop).take(2);

    // taking `b` to the top and asserting if it's equal to ZERO after all the right shifts.
    // TODO should we assert and not just perform the operation so the user can assert himself if
    // he wants to?
    let post = [Swap, Eqz, Assert];

    let chain = pre
        .into_iter()
        .chain(expacc)
        .chain(drop)
        .chain(post.into_iter());

    span.add_ops(chain)
}

pub(super) fn exp_imm(
    _imm: &Felt,
    _span: &mut SpanBuilder,
) -> Result<Option<CodeBlock>, AssemblerError> {
    todo!()
}

pub(super) fn exp_bits(
    _bit: &u8,
    _span: &mut SpanBuilder,
) -> Result<Option<CodeBlock>, AssemblerError> {
    todo!()
}

// COMPARISON OPERATIONS
// ================================================================================================

pub(super) fn eq_imm(
    imm: &Felt,
    span: &mut SpanBuilder,
) -> Result<Option<CodeBlock>, AssemblerError> {
    if imm == &Felt::ZERO {
        span.add_op(Eqz)
    } else {
        span.add_ops([Push(*imm), Eq])
    }
}

pub(super) fn eqw(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblerError> {
    span.add_ops([
        // duplicate first pair of for comparison(4th elements of each word) in reverse order
        // to avoid using dup.8 after stack shifting(dup.X where X > 7, takes more VM cycles )
        Dup7, Dup4, Eq,
        // continue comparison pair by pair using bitwise AND for EQ results
        Dup7, Dup4, Eq, And, Dup6, Dup3, Eq, And, Dup5, Dup2, Eq, And,
    ])
}

pub(super) fn neq_imm(
    imm: &Felt,
    span: &mut SpanBuilder,
) -> Result<Option<CodeBlock>, AssemblerError> {
    if imm == &Felt::ZERO {
        span.add_ops([Eqz, Not])
    } else {
        span.add_ops([Push(*imm), Eq, Not])
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Appends relevant operations to the span block for the computation of power of 2.
pub fn append_pow2_op(span: &mut SpanBuilder) {
    // push base 2 onto the stack: [exp, .....] -> [2, exp, ......]
    span.push_op(Push(Felt::new(2)));

    // introduce initial value of acc onto the stack: [2, exp, ....] -> [1, 2, exp, ....]
    span.push_op(Pad);
    span.push_op(Incr);

    // arrange the top of the stack for `EXPACC` instruction: [1, 2, exp, ....] -> [0, 2, 1, exp, ...]
    span.push_op(Swap);
    span.push_op(Pad);

    // calling expacc instruction 7 times.
    span.push_op_many(Expacc, 6);

    // drop the top two elements bit and exp value of the latest bit.
    span.push_op_many(Drop, 2);

    // taking `b` to the top and asserting if it's equal to ZERO after all the right shifts.
    span.push_op(Swap);
    span.push_op(Eqz);
    span.push_op(Assert);
}
