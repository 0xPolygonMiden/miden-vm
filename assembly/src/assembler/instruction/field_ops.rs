use super::{
    validate_param, AssemblyError, CodeBlock, Felt, FieldElement, Operation::*, SpanBuilder,
    StarkField, ONE, ZERO,
};
use crate::MAX_EXP_BITS;

/// Field element representing TWO in the base field of the VM.
const TWO: Felt = Felt::new(2);

// ASSERTIONS
// ================================================================================================

/// Asserts that the top two words in the stack are equal.
///
/// VM cycles: 11 cycles
pub fn assertw(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    span.add_ops([MovUp4, Eq, Assert, MovUp3, Eq, Assert, MovUp2, Eq, Assert, Eq, Assert])
}

// BASIC ARITHMETIC OPERATIONS
// ================================================================================================

/// Appends a sequence of operations to add an immediate value to the value at the top of the
/// stack. Specifically, the sequences are:
/// - if imm = 0: NOOP
/// - else if imm = 1: INCR
/// - else if imm = 2: INCR INCR
/// - otherwise: PUSH(imm) ADD
pub fn add_imm(span: &mut SpanBuilder, imm: Felt) -> Result<Option<CodeBlock>, AssemblyError> {
    if imm == ZERO {
        span.add_op(Noop)
    } else if imm == ONE {
        span.add_op(Incr)
    } else if imm == TWO {
        span.add_ops([Incr, Incr])
    } else {
        span.add_ops([Push(imm), Add])
    }
}

/// Appends a sequence of operations to subtract an immediate value from the value at the top of the
/// stack. Specifically, the sequences are:
/// - if imm = 0: NOOP
/// - otherwise: PUSH(-imm) ADD
pub fn sub_imm(span: &mut SpanBuilder, imm: Felt) -> Result<Option<CodeBlock>, AssemblyError> {
    if imm == ZERO {
        span.add_op(Noop)
    } else {
        span.add_ops([Push(-imm), Add])
    }
}

/// Appends a sequence of operations to multiply the value at the top of the stack by an immediate
/// value. Specifically, the sequences are:
/// - if imm = 0: DROP PAD
/// - else if imm = 1: NOOP
/// - otherwise: PUSH(imm) MUL
pub fn mul_imm(span: &mut SpanBuilder, imm: Felt) -> Result<Option<CodeBlock>, AssemblyError> {
    if imm == ZERO {
        span.add_ops([Drop, Pad])
    } else if imm == ONE {
        span.add_op(Noop)
    } else {
        span.add_ops([Push(imm), Mul])
    }
}

/// Appends a sequence of operations to divide the value at the top of the stack by an immediate
/// value. Specifically, the sequences are:
/// - if imm = 0: Returns an error
/// - else if imm = 1: NOOP
/// - otherwise: PUSH(1/imm) MUL
///
/// # Errors
/// Returns an error if the immediate value is ZERO.
pub fn div_imm(span: &mut SpanBuilder, imm: Felt) -> Result<Option<CodeBlock>, AssemblyError> {
    if imm == ZERO {
        Err(AssemblyError::division_by_zero())
    } else if imm == ONE {
        span.add_op(Noop)
    } else {
        span.add_ops([Push(imm.inv()), Mul])
    }
}

// POWER OF TWO OPERATION
// ================================================================================================

/// Appends a sequence of operations to raise value 2 to the power specified by the element at the
/// top of the stack.
///
/// VM cycles: 16 cycles
pub fn pow2(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    append_pow2_op(span);
    Ok(None)
}

/// Appends relevant operations to the span block for the computation of power of 2.
///
/// VM cycles: 16 cycles
pub fn append_pow2_op(span: &mut SpanBuilder) {
    // push base 2 onto the stack: [exp, ...] -> [2, exp, ...]
    span.push_op(Push(2_u8.into()));
    // introduce initial value of acc onto the stack: [2, exp, ...] -> [1, 2, exp, ...]
    span.push_ops([Pad, Incr]);
    // arrange the top of the stack for EXPACC operation: [1, 2, exp, ...] -> [0, 2, 1, exp, ...]
    span.push_ops([Swap, Pad]);
    // calling expacc instruction 6 times
    span.push_ops([Expacc, Expacc, Expacc, Expacc, Expacc, Expacc]);
    // drop the top two elements bit and exp value of the latest bit.
    span.push_ops([Drop, Drop]);
    // taking `b` to the top and asserting if it's equal to ZERO after all the right shifts.
    span.push_ops([Swap, Eqz, Assert]);
}

// EXPONENTIATION OPERATION
// ================================================================================================

/// Appends a sequence of operations to compute b^e where e is the value at the top of the stack
/// and b is the value second from the top of the stack.
///
/// num_pow_bits parameter is expected to contain the number of bits needed to encode value e. If
/// this assumption is not satisfied, the operation will fail at runtime.
///
/// VM cycles: 9 + num_pow_bits
///
/// # Errors
/// Returns an error if num_pow_bits is greater than 64.
pub fn exp(span: &mut SpanBuilder, num_pow_bits: u8) -> Result<Option<CodeBlock>, AssemblyError> {
    validate_param(num_pow_bits, 0..=MAX_EXP_BITS)?;

    // arranging the stack to prepare it for expacc instruction.
    span.push_ops([Pad, Incr, MovUp2, Pad]);

    // calling expacc instruction n times.
    span.push_op_many(Expacc, num_pow_bits as usize);

    // drop the top two elements bit and exp value of the latest bit.
    span.push_ops([Drop, Drop]);

    // taking `b` to the top and asserting if it's equal to ZERO after all the right shifts.
    span.push_ops([Swap, Eqz, Assert]);
    Ok(None)
}

/// Appends a sequence of operations to compute b^pow where b is the value at the top of the stack.
///
/// VM cycles per mode:
/// - pow = 0: 3 cycles
/// - pow = 1: 1 cycles
/// - pow = 2: 2 cycles
/// - pow = 3: 4 cycles
/// - pow = 4: 6 cycles
/// - pow = 5: 8 cycles
/// - pow = 6: 10 cycles
/// - pow = 7: 12 cycles
/// - pow > 7: 9 + Ceil(log2(pow))
pub fn exp_imm(span: &mut SpanBuilder, pow: Felt) -> Result<Option<CodeBlock>, AssemblyError> {
    if pow.as_int() <= 7 {
        perform_exp_for_small_power(span, pow.as_int());
        Ok(None)
    } else {
        // compute the bits length of the exponent
        let num_pow_bits = (64 - pow.as_int().leading_zeros()) as u8;

        // pushing the exponent onto the stack.
        span.push_op(Push(pow));

        exp(span, num_pow_bits)
    }
}

/// If the immediate value of the `exp` instruction is less than 8, then, it is be cheaper to
/// compute the exponentiation of the base to the power imm using `dup` and `mul` instructions.
///
/// The expected starting state of the stack (from the top) is: [b, ...].
///
/// After these operations, the stack state will be: [b^pow, ...], where b is the immediate value and b
/// is less than 8.
///
/// VM cycles per mode:
/// - pow = 0: 3 cycles
/// - pow = 1: 1 cycles
/// - pow = 2: 2 cycles
/// - pow = 3: 4 cycles
/// - pow = 4: 6 cycles
/// - pow = 5: 8 cycles
/// - pow = 6: 10 cycles
/// - pow = 7: 12 cycles
fn perform_exp_for_small_power(span: &mut SpanBuilder, pow: u64) {
    match pow {
        0 => {
            span.push_op(Drop);
            span.push_op(Pad);
            span.push_op(Incr);
        }
        1 => span.push_op(Noop), // TODO: show warning?
        2 => {
            span.push_op(Dup0);
            span.push_op(Mul);
        }
        3 => {
            span.push_op_many(Dup0, 2);
            span.push_op_many(Mul, 2);
        }
        4 => {
            span.push_op_many(Dup0, 3);
            span.push_op_many(Mul, 3);
        }
        5 => {
            span.push_op_many(Dup0, 4);
            span.push_op_many(Mul, 4);
        }
        6 => {
            span.push_op_many(Dup0, 5);
            span.push_op_many(Mul, 5);
        }
        7 => {
            span.push_op_many(Dup0, 6);
            span.push_op_many(Mul, 6);
        }
        _ => unreachable!("pow must be less than 8"),
    }
}

// COMPARISON OPERATIONS
// ================================================================================================

/// Appends a sequence of operations to check equality between the value at the top of the stack
/// and the provided immediate value. Specifically, the sequences are:
/// - if imm = 0: EQZ
/// - otherwise: PUSH(imm) EQ
pub fn eq_imm(span: &mut SpanBuilder, imm: Felt) -> Result<Option<CodeBlock>, AssemblyError> {
    if imm == ZERO {
        span.add_op(Eqz)
    } else {
        span.add_ops([Push(imm), Eq])
    }
}

/// Appends a sequence of operations to check inequality between the value at the top of the stack
/// and the provided immediate value. Specifically, the sequences are:
/// - if imm = 0: EQZ NOT
/// - otherwise: PUSH(imm) EQ NOT
pub fn neq_imm(span: &mut SpanBuilder, imm: Felt) -> Result<Option<CodeBlock>, AssemblyError> {
    if imm == ZERO {
        span.add_ops([Eqz, Not])
    } else {
        span.add_ops([Push(imm), Eq, Not])
    }
}

/// Appends a sequence of operations to check equality between two words at the top of the stack.
///
/// This operation takes 15 VM cycles.
pub fn eqw(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    span.add_ops([
        // duplicate first pair of for comparison(4th elements of each word) in reverse order
        // to avoid using dup.8 after stack shifting(dup.X where X > 7, takes more VM cycles )
        Dup7, Dup4, Eq,
        // continue comparison pair by pair using bitwise AND for EQ results
        Dup7, Dup4, Eq, And, Dup6, Dup3, Eq, And, Dup5, Dup2, Eq, And,
    ])
}

/// Appends a sequence of operations to to pop the top 2 elements off the stack and do a "less
/// than" comparison. The stack is expected to be arranged as [b, a, ...] (from the top). A value
/// of 1 is pushed onto the stack if a < b. Otherwise, 0 is pushed.
///
/// This operation takes 17 VM cycles.
pub fn lt(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
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

/// Appends a sequence of operations to pop the top 2 elements off the stack and do a "less
/// than or equal" comparison. The stack is expected to be arranged as [b, a, ...] (from the top).
/// A value of 1 is pushed onto the stack if a <= b. Otherwise, 0 is pushed.
///
/// This operation takes 18 VM cycles.
pub fn lte(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
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

/// Appends a sequence of operations to pop the top 2 elements off the stack and do a "greater
/// than" comparison. The stack is expected to be arranged as [b, a, ...] (from the top). A value
/// of 1 is pushed onto the stack if a > b. Otherwise, 0 is pushed.
///
/// This operation takes 18 VM cycles.
pub fn gt(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
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

/// Appends a sequence of operations to pop the top 2 elements off the stack and do a "greater
/// than or equal" comparison. The stack is expected to be arranged as [b, a, ...] (from the top).
/// A value of 1 is pushed onto the stack if a >= b. Otherwise, 0 is pushed.
///
/// This operation takes 19 VM cycles.
pub fn gte(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
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

/// Checks if the top element in the stack is an odd number or not.
///
/// Vm cycles: 5
pub fn is_odd(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    span.add_ops([U32split, Drop, Pad, Incr, U32and])
}

// COMPARISON OPERATION HELPER FUNCTIONS
// ================================================================================================

/// Splits the top 2 elements on the stack into low and high 32-bit values and swaps their order.
/// The expected starting state of the stack (from the top) is: [b, a, ...].
///
/// After these operations, the stack state will be: [a_hi, a_lo, b_hi, b_lo, ...].
///
/// This operation takes 3 cycles.
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
/// This operation takes 6 cycles.
fn check_lt_and_eq(span: &mut SpanBuilder) {
    // calculate a - b
    // stack: [b, a, ...] => [underflow_flag, result, ...]
    span.push_op(U32sub);
    // Put 1 on the stack if the underflow flag was not set (there was no underflow)
    span.push_ops([Dup0, Not]);
    // move the result to the top of the stack and check if it was zero
    span.push_ops([MovUp2, Eqz]);
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
/// This operation takes 9 cycles.
fn check_lt_high_bits(span: &mut SpanBuilder) {
    // reorder the stack to check a_hi < b_hi
    span.push_op(MovUp2);

    // simultaneously check a_hi < b_hi and a_hi = b_hi, resulting in:
    // - an equality flag of 1 if a_hi = b_hi and 0 otherwise (at stack[0])
    // - a less-than flag of 1 if a_hi > b_hi and 0 otherwise (at stack[1])
    check_lt_and_eq(span);

    // reorder the stack to prepare for low-bit comparison (a_lo < b_lo)
    span.push_ops([MovUp2, MovUp3]);
}

/// Appends operations to the span block to emulate a "less than" conditional and check that a < b
/// for a starting stack of [b, a, ...]. Pops both elements and leaves 1 on the stack if a < b and
/// 0 otherwise.
///
/// This is implemented with the VM's U32SUB op, which performs a subtraction and leaves the
/// result and an underflow flag on the stack. When a < b, a - b will underflow, so the less-than
/// condition will be true if the underflow flag is set.
///
/// This operation takes 3 cycles.
fn check_lt(span: &mut SpanBuilder) {
    // calculate a - b
    // stack: [b, a, ...] => [underflow_flag, result, ...]
    span.push_op(U32sub);

    // drop the result, since it's not needed
    span.push_ops([Swap, Drop]);
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
/// This is implemented with the VM's U32SUB op, which performs a subtraction and leaves the
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
    span.push_ops([Swap, Eqz]);

    // set the lte flag if the underflow flag was set or the result was 0
    span.push_op(Or);
}

/// This is a helper function for comparison operations that perform a greater-than check a > b
/// between two field elements a and b. It expects both elements to be already split into upper and
/// lower 32-bit values and arranged on the stack (from the top) as:
/// [a_hi, a_lo, bi_hi, b_lo, ...].
///
/// We pop the high bit values of both elements, compare them, and push 2 flags: one for
/// greater-than and one for equality. Then we move the flags down the stack, leaving the low bits
/// at the top of the stack in the orientation required for a greater-than check of the low bit
/// values (a_lo > b_lo).
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
    span.push_ops([Swap, MovDn2]);

    // simultaneously check b_hi < a_hi and b_hi = a_hi, resulting in:
    // - an equality flag of 1 if a_hi = b_hi and 0 otherwise (at stack[0])
    // - a greater-than flag of 1 if a_hi > b_hi and 0 otherwise (at stack[1])
    check_lt_and_eq(span);

    // reorder the stack to prepare for low-bit comparison (b_lo < a_lo)
    span.push_ops([MovUp3, MovUp3]);
}
