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

/// Appends operations to the span block to pop the top 2 elements off the stack and do a "less
/// than" comparison. The stack is expected to be arranged as [b, a, ...] (from the top). A value
/// of 1 is pushed onto the stack if a < b. Otherwise, 0 is pushed.
///
/// This operation takes 17 VM cycles.
///
/// # Errors
/// Returns an error if the assembly operation token is malformed or incorrect.
pub(super) fn lt(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblerError> {
    // Split both elements into high and low bits
    // 3 cycles
    split_elements(span);

    // compare the high bit values and put comparison result flags on the stack for eq and lt
    // then reorder in preparation for the low-bit comparison (a_lo < b_lo)
    // 9 cycles
    check_lt_high_bits(span);

    // check a_lo < b_lo, resulting in 1 if true and 0 otherwise
    // 3 cycles
    check_lt(span);

    // combine low-bit and high-bit results
    // 2 cycles
    set_result(span);

    Ok(None)
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

/// Appends operations to the span block to pop the top 2 elements off the stack and do a "less
/// than or equal" comparison. The stack is expected to be arranged as [b, a, ...] (from the top).
/// A value of 1 is pushed onto the stack if a <= b. Otherwise, 0 is pushed.
///
/// This operation takes 18 VM cycles.
///
/// # Errors
/// Returns an error if the assembly operation token is malformed or incorrect.
pub(super) fn lte(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblerError> {
    // Split both elements into high and low bits
    // 3 cycles
    split_elements(span);

    // compare the high bit values and put comparison result flags on the stack for eq and lt
    // then reorder in preparation for the low-bit comparison (a_lo <= b_lo)
    // 9 cycles
    check_lt_high_bits(span);

    // check a_lo <= b_lo, resulting in 1 if true and 0 otherwise
    // 4 cycles
    check_lte(span);

    // combine low-bit and high-bit results
    // 2 cycles
    set_result(span);

    Ok(None)
}

/// Appends operations to the span block to pop the top 2 elements off the stack and do a "greater
/// than" comparison. The stack is expected to be arranged as [b, a, ...] (from the top). A value
/// of 1 is pushed onto the stack if a > b. Otherwise, 0 is pushed.
///
/// This operation takes 18 VM cycles.
///
/// # Errors
/// Returns an error if the assembly operation token is malformed or incorrect.
pub(super) fn gt(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblerError> {
    // Split both elements into high and low bits
    // 3 cycles
    split_elements(span);

    // compare the high bit values and put comparison result flags on the stack for eq and gt
    // then reorder in preparation for the low-bit comparison (b_lo < a_lo)
    // 10 cycles
    check_gt_high_bits(span);

    // check b_lo < a_lo, resulting in 1 if true and 0 otherwise
    // 3 cycles
    check_lt(span);

    // combine low-bit and high-bit results
    // 2 cycles
    set_result(span);

    Ok(None)
}

/// Appends operations to the span block to pop the top 2 elements off the stack and do a "greater
/// than or equal" comparison. The stack is expected to be arranged as [b, a, ...] (from the top).
/// A value of 1 is pushed onto the stack if a >= b. Otherwise, 0 is pushed.
///
/// This operation takes 19 VM cycles.
///
/// # Errors
/// Returns an error if the assembly operation token is malformed or incorrect.
pub(super) fn gte(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblerError> {
    // Split both elements into high and low bits
    // 3 cycles
    split_elements(span);

    // compare the high bit values and put comparison result flags on the stack for eq and gt
    // then reorder in preparation for the low-bit comparison (b_lo <= a_lo)
    // 10 cycles
    check_gt_high_bits(span);

    // check b_lo <= a_lo, resulting in 1 if true and 0 otherwise
    // 4 cycles
    check_lte(span);

    // combine low-bit and high-bit results
    // 2 cycles
    set_result(span);

    Ok(None)
}

// COMPARISON OPERATION HELPER FUNCTIONS
// ================================================================================================

/// Splits the top 2 elements on the stack into low and high 32-bit values and swaps their order.
/// The expected starting state of the stack (from the top) is: [b, a, ...].
///
/// After these operations, the stack state will be: [a_hi, a_lo, b_hi, b_lo, ...].
///
/// This function takes 3 cycles.
fn split_elements(span: &mut SpanBuilder) {
    // stack: [b, a, ...] => [b_hi, b_lo, a, ...]
    span.push_op(U32split);
    // => [a, b_hi, b_lo, ...]
    span.push_op(MovUp2);
    // => [a_hi, a_lo, b_hi, b_lo, ...]
    span.push_op(U32split);
}

/// Appends operations to the span block to simultaneously check both the "less than" condition
/// (a < b) and equality (a = b) and push a separate flag onto the stack for each result.
///
/// The expected stack (from the top) is: [b, a, ...].
/// - Pushes 1 on the stack if a < b and 0 otherwise.
/// - Pushes 1 on the stack if a = b and 0 otherwise.
///
/// The resulting stack after this operation is: [eq_flag, lt_flag, ...].
///
/// This function takes 6 cycles.
fn check_lt_and_eq(span: &mut SpanBuilder) {
    // calculate a - b
    // stack: [b, a, ...] => [underflow_flag, result, ...]
    span.push_op(U32sub);

    // Put 1 on the stack if the underflow flag was not set (there was no underflow)
    span.push_op(Dup0);
    span.push_op(Not);

    // move the result to the top of the stack and check if it was zero
    span.push_op(MovUp2);
    span.push_op(Eqz);

    // set the equality flag to 1 if there was no underflow and the result was zero
    span.push_op(And);
}

/// This is a helper function for comparison operations that perform a less-than check a < b
/// between two field elements a and b. It expects both elements to be already split into upper and
/// lower 32-bit values and arranged on the stack (from the top) as:
/// [a_hi, a_lo, bi_hi, b_lo, ...].
///
/// It pops the high bit values of both elements, compares them, and pushes 2 flags: one for
/// less-than and one for equality. Then it moves the flags down the stack, leaving the low bits at
/// the top of the stack in the orientation required for a less-than check of the low bit values
/// (a_lo < b_lo).
///
/// After this operation, the stack will look as follows (from the top):
/// - b_lo
/// - a_lo
/// - hi_flag_eq: 1 if the high bit values were equal; 0 otherwise
/// - hi_flag_lt: 1 if a's high-bit values were less than b's (a_hi < b_hi); 0 otherwise
///
/// This function takes 9 cycles.
fn check_lt_high_bits(span: &mut SpanBuilder) {
    // reorder the stack to check a_hi < b_hi
    span.push_op(MovUp2);

    // simultaneously check a_hi < b_hi and a_hi = b_hi, resulting in:
    // - an equality flag of 1 if a_hi = b_hi and 0 otherwise (at stack[0])
    // - a less-than flag of 1 if a_hi > b_hi and 0 otherwise (at stack[1])
    check_lt_and_eq(span);

    // reorder the stack to prepare for low-bit comparison (a_lo < b_lo)
    span.push_op(MovUp2);
    span.push_op(MovUp3);
}

/// Appends operations to the span block to emulate a "less than" conditional and check that a < b
/// for a starting stack of [b, a, ...]. Pops both elements and leaves 1 on the stack if a < b and
/// 0 otherwise.
///
/// This is implemented with the VM's ```U32sub``` op, which performs a subtraction and leaves the
/// result and an underflow flag on the stack. When a < b, a - b will underflow, so the less-than
/// condition will be true if the underflow flag is set.
///
/// This function takes 3 cycles.
fn check_lt(span: &mut SpanBuilder) {
    // calculate a - b
    // stack: [b, a, ...] => [underflow_flag, result, ...]
    span.push_op(U32sub);

    // drop the result, since it's not needed
    span.push_op(Swap);
    span.push_op(Drop);
}

/// This is a helper function to combine the high-bit and low-bit comparison checks into a single
/// result flag.
///
/// Since we're working with a 64-bit field modulus, we need to ensure that valid field elements
/// represented by > 32 bits are still compared properly. High bit comparisons take precedence, so
/// we only care about the result of the low-bit value comparison when the high bits were equal.
///
/// The stack is expected to be arranged as follows (from the top):
/// - low-bit comparison flag: 1 if the lt/lte/gt/gte condition being checked was true; 0 otherwise
/// - high-bit equality flag: 1 if the high bits were equal; 0 otherwise
/// - high-bit comparison flag: 1 if the lt/gt condition being checked was true; 0 otherwise
///
/// This function takes 2 cycles.
fn set_result(span: &mut SpanBuilder) {
    // check if high bits are equal AND low bit comparison condition was true
    span.push_op(And);

    // Set the result flag if the above check passed OR the high-bit comparison was true
    span.push_op(Or);
}

/// Appends operations to the span block to emulate a "less than or equal" conditional and check
/// that a <= b for a starting stack of [b, a, ...]. Pops both elements and leaves 1 on the stack
/// if a <= b and 0 otherwise.
///
/// This is implemented with the VM's ```U32sub``` op, which performs a subtraction and leaves the
/// result and an underflow flag on the stack. When a < b, a - b will underflow, so the less-than
/// condition will be true if the underflow flag is set. The equal condition will be true if
/// there was no underflow and the result is 0.
///
/// This function takes 4 cycles.
fn check_lte(span: &mut SpanBuilder) {
    // calculate a - b
    // stack: [b, a, ...] => [underflow_flag, result, ...]
    span.push_op(U32sub);

    // check the result
    span.push_op(Swap);
    span.push_op(Eqz);

    // set the lte flag if the underflow flag was set or the result was 0
    span.push_op(Or);
}

/// This is a helper function for comparison operations that perform a greater-than check a > b
/// between two field elements a and b. It expects both elements to be already split into upper and
/// lower 32-bit values and arranged on the stack (from the top) as:
/// [a_hi, a_lo, bi_hi, b_lo, ...].
///
/// It pops the high bit values of both elements, compares them, and pushes 2 flags: one for
/// greater-than and one for equality. Then it moves the flags down the stack, leaving the low bits at
/// the top of the stack in the orientation required for a greater-than check of the low bit values
/// (a_lo > b_lo).
///
/// After this operation, the stack will look as follows (from the top):
/// - a_lo
/// - b_lo
/// - hi_flag_eq: 1 if the high bit values were equal; 0 otherwise
/// - hi_flag_gt: 1 if a's high-bit values were greater than b's (a_hi > b_hi); 0 otherwise
///
/// This function takes 10 cycles.
fn check_gt_high_bits(span: &mut SpanBuilder) {
    // reorder the stack to check b_hi < a_hi
    span.push_op(Swap);
    span.push_op(MovDn2);

    // simultaneously check b_hi < a_hi and b_hi = a_hi, resulting in:
    // - an equality flag of 1 if a_hi = b_hi and 0 otherwise (at stack[0])
    // - a greater-than flag of 1 if a_hi > b_hi and 0 otherwise (at stack[1])
    check_lt_and_eq(span);

    // reorder the stack to prepare for low-bit comparison (b_lo < a_lo)
    span.push_op(MovUp3);
    span.push_op(MovUp3);
}
