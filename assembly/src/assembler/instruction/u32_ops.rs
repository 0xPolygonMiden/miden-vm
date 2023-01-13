use super::{
    field_ops::append_pow2_op,
    push_u32_value, validate_param, AssemblyError, CodeBlock, Felt,
    Operation::{self, *},
    SpanBuilder,
};
use crate::{MAX_U32_ROTATE_VALUE, MAX_U32_SHIFT_VALUE};

// ENUMS
// ================================================================================================

/// This enum is intended to determine the mode of operation passed to the parsing function
#[derive(PartialEq, Eq)]
pub enum U32OpMode {
    Checked,
    Unchecked,
    Wrapping,
    Overflowing,
}

// CONVERSIONS AND TESTS
// ================================================================================================

/// Translates u32testw assembly instruction to VM operations.
///
/// Implemented by executing DUP U32SPLIT SWAP DROP EQZ on each element in the word
/// and combining the results using AND operation (total of 23 VM cycles)
pub fn u32testw(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    #[rustfmt::skip]
    let ops = [
         // Test the fourth element
        Dup3, U32split, Swap, Drop, Eqz,

        // Test the third element
        Dup3, U32split, Swap, Drop, Eqz, And,

         // Test the second element
        Dup2, U32split, Swap, Drop, Eqz, And,

        // Test the first element
        Dup1, U32split, Swap, Drop, Eqz, And,
    ];
    span.add_ops(ops)
}

/// Translates u32assertw assembly instruction to VM operations.
///
/// Implemented by executing `U32ASSERT2` on each pair of elements in the word.
/// Total of 6 VM cycles.
pub fn u32assertw(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    #[rustfmt::skip]
    let ops = [
        // Test the first and the second elements
        U32assert2,

        // Move 3 and 4 to the top of the stack
        MovUp3, MovUp3,

        // Test them
        U32assert2,

        // Move the elements back into place
        MovUp3, MovUp3,
    ];
    span.add_ops(ops)
}

// ARITHMETIC OPERATIONS
// ================================================================================================

/// Translates u32add assembly instructions to VM operations.
///
/// The base operation is `U32ADD`, but depending on the mode, additional operations may be
/// inserted. Please refer to the docs of `handle_arithmetic_operation` for more details.
///
/// VM cycles per mode:
/// - u32checked_add: 4 cycles
/// - u32checked_add.b:
///    - 6 cycles if b = 1
///    - 5 cycles if b != 1
/// - u32wrapping_add: 2 cycles
/// - u32wrapping_add.b: 3 cycles
/// - u32overflowing_add: 1 cycles
/// - u32overflowing_add.b: 2 cycles
pub fn u32add(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
    imm: Option<u32>,
) -> Result<Option<CodeBlock>, AssemblyError> {
    handle_arithmetic_operation(span, U32add, op_mode, imm)
}

/// Translates u32sub assembly instructions to VM operations.
///
/// The base operation is `U32SUB`, but depending on the mode, additional operations may be
/// inserted. Please refer to the docs of `handle_arithmetic_operation` for more details.
///
/// VM cycles per mode:
/// - u32checked_sub: 4 cycles
/// - u32checked_sub.b:
///    - 6 cycles if b = 1
///    - 5 cycles if b != 1
/// - u32wrapping_sub: 2 cycles
/// - u32wrapping_sub.b: 3 cycles
/// - u32overflowing_sub: 1 cycles
/// - u32overflowing_sub.b: 2 cycles
pub fn u32sub(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
    imm: Option<u32>,
) -> Result<Option<CodeBlock>, AssemblyError> {
    handle_arithmetic_operation(span, U32sub, op_mode, imm)
}

/// Translates u32mul assembly instructions to VM operations.
///
/// The base operation is `U32MUL`, but depending on the mode, additional operations may be
/// inserted. Please refer to the docs of `handle_arithmetic_operation` for more details.
///
/// VM cycles per mode:
/// - u32checked_mul: 4 cycles
/// - u32checked_mul.b:
///    - 6 cycles if b = 1
///    - 5 cycles if b != 1
/// - u32wrapping_mul: 2 cycles
/// - u32wrapping_mul.b: 3 cycles
/// - u32overflowing_mul: 1 cycles
/// - u32overflowing_mul.b: 2 cycles
pub fn u32mul(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
    imm: Option<u32>,
) -> Result<Option<CodeBlock>, AssemblyError> {
    handle_arithmetic_operation(span, U32mul, op_mode, imm)
}

/// Translates u32div assembly instructions to VM operations.
///
/// VM cycles per mode:
/// - u32checked_div: 3 cycles
/// - u32checked_div.b:
///    - 5 cycles if b is 1
///    - 4 cycles if b is not 1
/// - u32unchecked_div: 2 cycles
/// - u32unchecked_div.b:
///    - 4 cycles if b is 1
///    - 3 cycles if b is not 1
pub fn u32div(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
    imm: Option<u32>,
) -> Result<Option<CodeBlock>, AssemblyError> {
    handle_division(span, op_mode, imm)?;
    span.add_op(Drop)
}

/// Translates u32mod assembly instructions to VM operations.
///
/// VM cycles per mode:
/// - u32checked_mod: 4 cycles
/// - u32checked_mod.b:
///    - 6 cycles if b is 1
///    - 5 cycles if b is not 1
/// - u32unchecked_mod: 3 cycle
/// - u32unchecked_mod.b:
///    - 5 cycles if b is 1
///    - 4 cycles if b is not 1
pub fn u32mod(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
    imm: Option<u32>,
) -> Result<Option<CodeBlock>, AssemblyError> {
    handle_division(span, op_mode, imm)?;
    span.add_ops([Swap, Drop])
}

/// Translates u32divmod assembly instructions to VM operations.
///
/// VM cycles per mode:
/// - u32checked_divmod: 2 cycles
/// - u32checked_divmod.b:
///    - 4 cycles if b is 1
///    - 3 cycles if b is not 1
/// - u32unchecked_divmod: 1 cycle
/// - u32unchecked_divmod.b:
///    - 3 cycles if b is 1
///    - 2 cycles if b is not 1
pub fn u32divmod(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
    imm: Option<u32>,
) -> Result<Option<CodeBlock>, AssemblyError> {
    handle_division(span, op_mode, imm)
}

// BITWISE OPERATIONS
// ================================================================================================

/// Translates u32checked_not assembly instruction to VM operations.
///
/// The reason this method works is because 2^32 -1 provides a bit mask of ones, which after
/// subtracting the element, flips the bits of the original value to perform a bitwise NOT.
///
/// This takes 5 VM cycles.
pub fn u32not(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    #[rustfmt::skip]
    let ops = [
        // Perform the operation
        Push(Felt::from(u32::MAX)),
        U32assert2,
        Swap,
        U32sub,

        // Drop the underflow flag
        Drop,
    ];
    span.add_ops(ops)
}

/// Translates u32shl assembly instructions to VM operations.
///
/// The operation is implemented by putting a power of 2 on the stack, then multiplying it with
/// the value to be shifted and splitting the result. For checked variants, the shift value is
/// asserted to be between 0-31 and the value to be shifted is asserted to be a 32-bit value.
///
/// VM cycles per mode:
/// - u32checked_shl: 19 cycles
/// - u32checked_shl.b: 4 cycles
/// - u32unchecked_shl: 18 cycles
/// - u32unchecked_shl.b: 3 cycles
pub fn u32shl(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
    imm: Option<u8>,
) -> Result<Option<CodeBlock>, AssemblyError> {
    prepare_bitwise::<MAX_U32_SHIFT_VALUE>(span, imm, op_mode, [U32mul, Drop])
}

/// Translates u32shr assembly instructions to VM operations.
///
/// The operation is implemented by putting a power of 2 on the stack, then dividing the value to
/// be shifted by it and returning the quotient. For checked variants, the shift value is asserted
/// to be between 0-31 and the value to be shifted is asserted to be a 32-bit value.
///
/// VM cycles per mode:
/// - u32checked_shr: 19 cycles
/// - u32checked_shr.b: 4 cycles
/// - u32unchecked_shr: 18 cycles
/// - u32unchecked_shr.b: 3 cycles
pub fn u32shr(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
    imm: Option<u8>,
) -> Result<Option<CodeBlock>, AssemblyError> {
    prepare_bitwise::<MAX_U32_SHIFT_VALUE>(span, imm, op_mode, [U32div, Drop])
}

/// Translates u32rotl assembly instructions to VM operations.
///
/// The base operation is implemented by putting a power of 2 on the stack, then multiplying the
/// value to be shifted by it and adding the overflow limb to the shifted limb. For the checked
/// variants, the shift value is asserted to be between 0-31 and the value to be shifted is
/// asserted to be a 32-bit value.
///
/// VM cycles per mode:
/// - u32checked_rotl: 19 cycles
/// - u32checked_rotl.b: 4 cycles
/// - u32unchecked_rotl: 18 cycles
/// - u32unchecked_rotl.b: 3 cycles
pub fn u32rotl(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
    imm: Option<u8>,
) -> Result<Option<CodeBlock>, AssemblyError> {
    prepare_bitwise::<MAX_U32_ROTATE_VALUE>(span, imm, op_mode, [U32mul, Add])
}

/// Translates u32rotr assembly instructions to VM operations.
///
/// The base operation is implemented by multiplying the value to be shifted by 2^(32-b), where
/// b is the shift amount, then adding the overflow limb to the shifted limb. For the checked
/// variants, the shift value is asserted to be between 0-31 and the value to be shifted is
/// asserted to be a 32-bit value.
///
/// VM cycles per mode:
/// - u32checked_rotr: 31 cycles
/// - u32checked_rotr.b: 6 cycles
/// - u32unchecked_rotr: 22 cycles
/// - u32unchecked_rotr.b: 3 cycles
pub fn u32rotr(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
    imm: Option<u8>,
) -> Result<Option<CodeBlock>, AssemblyError> {
    match (imm, op_mode) {
        (Some(imm), U32OpMode::Checked) if imm == 0 => {
            // if rotation is performed by 0, just verify that stack top is u32
            span.push_ops([Pad, U32assert2, Drop]);
            return Ok(None);
        }
        (Some(imm), U32OpMode::Checked) => {
            validate_param(imm, 1..=MAX_U32_ROTATE_VALUE)?;
            span.push_ops([Push(Felt::new(1 << (32 - imm))), U32assert2]);
        }
        (Some(imm), U32OpMode::Unchecked) if imm == 0 => {
            // if rotation is performed by 0, do nothing (Noop)
            span.push_op(Noop);
            return Ok(None);
        }
        (Some(imm), U32OpMode::Unchecked) => {
            validate_param(imm, 1..=MAX_U32_ROTATE_VALUE)?;
            span.push_op(Push(Felt::new(1 << (32 - imm))));
        }
        (None, U32OpMode::Checked) => {
            #[rustfmt::skip]
            span.push_ops([
                // Verify both b and a are u32.
                U32assert2,

                // Calculate 32 - b and assert that the shift value b <= 31.
                Push(Felt::from(MAX_U32_ROTATE_VALUE)), Dup1, U32sub, Not, Assert, Incr, Dup1,

                // If 32-b = 32, replace it with 0.
                Eqz, Not, CSwap, Drop,
            ]);
            append_pow2_op(span);
            span.push_op(Swap);
        }
        (None, U32OpMode::Unchecked) => {
            span.push_ops([Push(Felt::new(32)), Swap, U32sub, Drop]);
            append_pow2_op(span);
        }
        _ => unreachable!("unsupported operation mode"),
    }
    span.add_ops([U32mul, Add])
}

/// Translates u32popcnt assembly instructions to VM operations.
///
/// VM cycles per mode:
/// - u32checked_popcnt: 36 cycles
/// - u32unchecked_popcnt: 33 cycles
pub fn u32popcnt(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
) -> Result<Option<CodeBlock>, AssemblyError> {
    match op_mode {
        U32OpMode::Checked => span.push_ops([Pad, U32assert2, Drop]),
        U32OpMode::Unchecked => (),
        _ => unreachable!("unsupported operation mode"),
    }
    #[rustfmt::skip]
    let ops = [
        // i = i - ((i >> 1) & 0x55555555);
        Dup0,
        Push(Felt::new(1 << 1)), U32div, Drop,
        Push(Felt::new(0x55555555)),
        U32and,
        U32sub, Drop,
        // i = (i & 0x33333333) + ((i >> 2) & 0x33333333);
        Dup0,
        Push(Felt::new(1 << 2)), U32div, Drop,
        Push(Felt::new(0x33333333)),
        U32and,
        Swap,
        Push(Felt::new(0x33333333)),
        U32and,
        U32add, Drop,
        // i = (i + (i >> 4)) & 0x0F0F0F0F;
        Dup0,
        Push(Felt::new(1 << 4)), U32div, Drop,
        U32add, Drop,
        Push(Felt::new(0x0F0F0F0F)),
        U32and,
        // return (i * 0x01010101) >> 24;
        Push(Felt::new(0x01010101)),
        U32mul, Drop,
        Push(Felt::new(1 << 24)), U32div, Drop
    ];
    span.add_ops(ops)
}

/// Handles U32ADD, U32SUB, and U32MUL operations in checked, wrapping, and overflowing modes,
/// including handling of immediate parameters.
///
/// Specifically handles these specific inputs per the spec.
/// - Checked: fails if either of the inputs or the output is not a u32 value.
/// - Wrapping: does not check if the inputs are u32 values; overflow or underflow bits are
///   discarded.
/// - Overflowing: does not check if the inputs are u32 values; overflow or underflow bits are
///   pushed onto the stack.
fn handle_arithmetic_operation(
    span: &mut SpanBuilder,
    op: Operation,
    op_mode: U32OpMode,
    imm: Option<u32>,
) -> Result<Option<CodeBlock>, AssemblyError> {
    let mut drop_high_bits = false;
    let mut assert_u32_res = false;

    if let Some(imm) = imm {
        push_u32_value(span, imm);
    }

    match op_mode {
        U32OpMode::Checked => {
            span.push_op(U32assert2);
            assert_u32_res = true;
        }
        U32OpMode::Wrapping => {
            drop_high_bits = true;
        }
        U32OpMode::Overflowing => {}
        _ => unreachable!("unsupported operation mode"),
    }

    span.push_op(op);

    if assert_u32_res {
        span.add_ops([Eqz, Assert])
    } else if drop_high_bits {
        span.add_op(Drop)
    } else {
        Ok(None)
    }
}

/// Handles common parts of u32div, u32mod, and u32divmod operations in checked and unchecked modes,
/// including handling of immediate parameters.
fn handle_division(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
    imm: Option<u32>,
) -> Result<Option<CodeBlock>, AssemblyError> {
    if let Some(imm) = imm {
        if imm == 0 {
            return Err(AssemblyError::division_by_zero());
        }
        push_u32_value(span, imm);
    }

    match op_mode {
        U32OpMode::Checked => {
            span.push_op(U32assert2);
        }
        U32OpMode::Unchecked => {}
        _ => unreachable!("unsupported operation mode"),
    }

    span.add_op(U32div)
}

// BITWISE OPERATIONS - HELPERS
// ================================================================================================

/// Mutate the first two elements of the stack from `[b, a, ..]` into `[2^b, a, ..]`, with `b`
/// either as a provided immediate value, or as an element that already exists in the stack.
///
/// If the used mode is `checked`, the function will assert that both `[b, a]` are valid `u32`.
/// This function is equivalent to a bit shift operation, so the exponent shouldn't cause a number
/// to be greater than `u32::MAX`; therefore, the maximum valid value must be `31`, as defined in
/// the helper constants.
///
/// This function supports only checked and unchecked modes; if some other mode is provided, it
/// will panic.
fn prepare_bitwise<const MAX_VALUE: u8>(
    span: &mut SpanBuilder,
    imm: Option<u8>,
    op_mode: U32OpMode,
    final_ops: [Operation; 2],
) -> Result<Option<CodeBlock>, AssemblyError> {
    match (imm, op_mode) {
        (Some(imm), U32OpMode::Checked) if imm == 0 => {
            // if shift/rotation is performed by 0, just verify that stack top is u32
            span.push_ops([Pad, U32assert2, Drop]);
            return Ok(None);
        }
        (Some(imm), U32OpMode::Checked) => {
            validate_param(imm, 1..=MAX_VALUE)?;
            span.push_ops([Push(Felt::new(1 << imm)), U32assert2]);
        }
        (Some(imm), U32OpMode::Unchecked) if imm == 0 => {
            // if shift/rotation is performed by 0, do nothing (Noop)
            span.push_op(Noop);
            return Ok(None);
        }
        (Some(imm), U32OpMode::Unchecked) => {
            span.push_op(Push(Felt::new(1 << imm)));
        }
        (None, U32OpMode::Checked) => {
            // Assume the dynamic shift value b is on top of the stack.
            append_pow2_op(span);
            span.push_op(U32assert2);
        }
        (None, U32OpMode::Unchecked) => append_pow2_op(span),
        _ => unreachable!("unsupported operation mode"),
    }
    span.add_ops(final_ops)
}

// COMPARISON OPERATIONS
// ================================================================================================

/// Translates u32checked_eq assembly instruction to VM operations.
///
/// Specifically we test the first two numbers to be u32, then perform a EQ to check the equality.
///
/// VM cycles per mode:
/// - u32checked_eq: 2 cycles
/// - u32checked_eq.b: 3 cycles
pub fn u32eq(span: &mut SpanBuilder, imm: Option<u32>) -> Result<Option<CodeBlock>, AssemblyError> {
    if let Some(imm) = imm {
        push_u32_value(span, imm);
    }

    span.add_ops([U32assert2, Eq])
}

/// Translates u32checked_neq assembly instruction to VM operations.
///
/// Specifically we test the first two numbers to be u32, then perform a `EQ NOT` to check the
/// equality.
///
/// VM cycles per mode:
/// - u32checked_neq: 3 cycles
/// - u32checked_neq.b: 4 cycles
pub fn u32neq(
    span: &mut SpanBuilder,
    imm: Option<u32>,
) -> Result<Option<CodeBlock>, AssemblyError> {
    if let Some(imm) = imm {
        push_u32_value(span, imm);
    }

    span.add_ops([U32assert2, Eq, Not])
}

/// Translates u32lt assembly instructions to VM operations.
///
/// Specifically we test the first two numbers to be u32, then perform a `U32SUB EQZ NOT` to check
/// the underflow flag.
///
/// VM cycles per mode:
/// - u32checked_lt: 6 cycles
/// - u32unchecked_lt 5 cycles
pub fn u32lt(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
) -> Result<Option<CodeBlock>, AssemblyError> {
    handle_u32_and_unchecked_mode(span, op_mode);
    compute_lt(span);

    Ok(None)
}

/// Translates u32lte assembly instructions to VM operations.
///
/// Specifically we test the first two numbers to be u32, then perform a gt check and flip the
/// results.
///
/// VM cycles per mode:
/// - u32checked_lte: 8 cycles
/// - u32unchecked_lte: 7 cycles
pub fn u32lte(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
) -> Result<Option<CodeBlock>, AssemblyError> {
    handle_u32_and_unchecked_mode(span, op_mode);

    // Compute the lt with reversed number to get a gt check
    span.push_op(Swap);
    compute_lt(span);

    // Flip the final results to get the lte results.
    span.add_op(Not)
}

/// Translates u32gt assembly instructions to VM operations.
///
/// Specifically we test the first two numbers to be u32, then perform a lt check with the
/// numbers swapped.
///
/// VM cycles per mode:
/// - u32checked_gt: 7 cycles
/// - u32unchecked_gt: 6 cycles
pub fn u32gt(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
) -> Result<Option<CodeBlock>, AssemblyError> {
    handle_u32_and_unchecked_mode(span, op_mode);

    // Reverse the numbers so we can get a gt check.
    span.push_op(Swap);

    compute_lt(span);

    Ok(None)
}

/// Translates u32gte assembly instructions to VM operations.
///
/// Specifically we test the first two numbers to be u32, then compute a lt check and flip the
/// results.
///
/// VM cycles per mode:
/// - u32checked_gte: 7 cycles
/// - u32unchecked_gte: 6 cycles
pub fn u32gte(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
) -> Result<Option<CodeBlock>, AssemblyError> {
    handle_u32_and_unchecked_mode(span, op_mode);

    compute_lt(span);

    // Flip the final results to get the gte results.
    span.add_op(Not)
}

/// Translates u32min assembly instructions to VM operations.
///
/// Specifically, we test the first two numbers to be u32 (U32SPLIT NOT ASSERT), subtract the top
/// value from the second to the top value (U32SUB), check the underflow flag (EQZ), and perform a
/// conditional swap (CSWAP) to have the max number in front. Then we finally drop the top element
/// to keep the min.
///
/// VM cycles per mode:
/// - u32checked_min: 9 cycles
/// - u32unchecked_min: 8 cycles
pub fn u32min(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
) -> Result<Option<CodeBlock>, AssemblyError> {
    compute_max_and_min(span, op_mode);

    // Drop the max and keep the min
    span.add_op(Drop)
}

/// Translates u32max assembly instructions to VM operations.
///
/// Specifically, we test the first two values to be u32 (U32SPLIT NOT ASSERT), subtract the top
/// value from the second to the top value (U32SUB), check the underflow flag (EQZ), and perform
/// a conditional swap (CSWAP) to have the max number in front. then we finally drop the 2nd
/// element to keep the max.
///
/// VM cycles per mode:
/// - u32checked_max: 10 cycles
/// - u32unchecked_max: 9 cycles
pub fn u32max(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
) -> Result<Option<CodeBlock>, AssemblyError> {
    compute_max_and_min(span, op_mode);

    // Drop the min and keep the max
    span.add_ops([Swap, Drop])
}

// COMPARISON OPERATIONS - HELPERS
// ================================================================================================

/// Handles u32 assertion and unchecked mode for any u32 operation.
fn handle_u32_and_unchecked_mode(span: &mut SpanBuilder, op_mode: U32OpMode) {
    if op_mode == U32OpMode::Checked {
        span.push_op(U32assert2);
    }
}

/// Inserts the VM operations to check if the second element is less than
/// the top element. This takes 5 cycles.
fn compute_lt(span: &mut SpanBuilder) {
    span.push_ops([
        U32sub, Swap, Drop, // Perform the operations
        Eqz, Not, // Check the underflow flag
    ])
}

/// Duplicate the top two elements in the stack and check both are u32, and determine the min
/// and max between them.
///
/// The maximum number will be at the top of the stack and minimum will be at the 2nd index.
fn compute_max_and_min(span: &mut SpanBuilder, op_mode: U32OpMode) {
    // Copy top two elements of the stack.
    span.push_ops([Dup1, Dup1]);
    if op_mode == U32OpMode::Checked {
        span.push_op(U32assert2);
    }

    #[rustfmt::skip]
    span.push_ops([
        U32sub, Swap, Drop,

        // Check the underflow flag, if it's zero
        // then the second number is equal or larger than the first.
        Eqz, CSwap,
    ]);
}
