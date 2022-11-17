use vm_core::FieldElement;

use super::{
    field_ops::append_pow2_op,
    AssemblerError, CodeBlock, Felt,
    Operation::{self, *},
    SpanBuilder,
};

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
pub fn u32testw(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblerError> {
    // Test the fourth element
    span.push_op(Dup3);
    span.push_op(U32split);
    span.push_op(Swap);
    span.push_op(Drop);
    span.push_op(Eqz);

    // Test the third element
    span.push_op(Dup3);
    span.push_op(U32split);
    span.push_op(Swap);
    span.push_op(Drop);
    span.push_op(Eqz);
    span.push_op(And);

    // Test the second element
    span.push_op(Dup2);
    span.push_op(U32split);
    span.push_op(Swap);
    span.push_op(Drop);
    span.push_op(Eqz);
    span.push_op(And);

    // Test the first element
    span.push_op(Dup1);
    span.push_op(U32split);
    span.push_op(Swap);
    span.push_op(Drop);
    span.push_op(Eqz);
    span.push_op(And);
    Ok(None)
}

/// Translates u32assertw assembly instruction to VM operations.
///
/// Implemented by executing `U32ASSERT2` on each pair of elements in the word.
/// Total of 6 VM cycles.
pub fn u32assertw(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblerError> {
    // Test the first and the second elements
    span.push_op(U32assert2);

    // Move 3 and 4 to the top of the stack
    span.push_op(MovUp3);
    span.push_op(MovUp3);

    // Test them
    span.push_op(U32assert2);

    // Move the elements back into place
    span.push_op(MovUp3);
    span.push_op(MovUp3);

    Ok(None)
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
) -> Result<Option<CodeBlock>, AssemblerError> {
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
) -> Result<Option<CodeBlock>, AssemblerError> {
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
) -> Result<Option<CodeBlock>, AssemblerError> {
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
) -> Result<Option<CodeBlock>, AssemblerError> {
    handle_division(span, op_mode, imm)?;

    span.push_op(Drop);

    Ok(None)
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
) -> Result<Option<CodeBlock>, AssemblerError> {
    handle_division(span, op_mode, imm)?;

    span.push_op(Swap);
    span.push_op(Drop);

    Ok(None)
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
) -> Result<Option<CodeBlock>, AssemblerError> {
    handle_division(span, op_mode, imm)?;
    Ok(None)
}

// BITWISE OPERATIONS
// ================================================================================================

/// Translates u32checked_not assembly instruction to VM operations.
///
/// The reason this method works is because 2^32 -1 provides a bit mask of ones, which after
/// subtracting the element, flips the bits of the original value to perform a bitwise NOT.
///
/// This takes 5 VM cycles.
pub fn u32not(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblerError> {
    // perform the operation
    span.push_op(Push(Felt::new(2u64.pow(32) - 1)));
    span.push_op(U32assert2);
    span.push_op(Swap);
    span.push_op(U32sub);

    // Drop the underflow flag
    span.push_op(Drop);

    Ok(None)
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
) -> Result<Option<CodeBlock>, AssemblerError> {
    match imm {
        Some(imm) => match op_mode {
            U32OpMode::Checked => {
                if imm > 31 {
                    return Err(AssemblerError::imm_out_of_bounds(imm as u64, 0, 31));
                }
                span.push_op(Push(Felt::new(2u64.pow(imm as u32))));
                span.push_op(U32assert2);
            }
            U32OpMode::Unchecked => {
                span.push_op(Push(Felt::new(2u64.pow(imm as u32))));
            }
            _ => unreachable!("unsupported operation mode"),
        },
        None => match op_mode {
            U32OpMode::Checked => {
                append_pow2_op(span);
                span.push_op(U32assert2);
            }
            U32OpMode::Unchecked => append_pow2_op(span),
            _ => unreachable!("unsupported operation mode"),
        },
    }

    span.push_op(U32mul);
    span.push_op(Drop);

    Ok(None)
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
) -> Result<Option<CodeBlock>, AssemblerError> {
    match imm {
        Some(imm) => match op_mode {
            U32OpMode::Checked => {
                if imm > 31 {
                    return Err(AssemblerError::imm_out_of_bounds(imm as u64, 0, 31));
                }
                span.push_op(Push(Felt::new(2u64.pow(imm as u32))));
                span.push_op(U32assert2);
            }
            U32OpMode::Unchecked => {
                span.push_op(Push(Felt::new(2u64.pow(imm as u32))));
            }
            _ => unreachable!(),
        },
        None => match op_mode {
            U32OpMode::Checked => {
                // Assume the dynamic shift value b is on top of the stack.
                append_pow2_op(span);
                span.push_op(U32assert2);
            }
            U32OpMode::Unchecked => append_pow2_op(span),
            _ => unreachable!(),
        },
    };

    // Use division to shift right and then drop the remainder
    span.push_op(U32div);
    span.push_op(Drop);

    Ok(None)
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
) -> Result<Option<CodeBlock>, AssemblerError> {
    match imm {
        None => match op_mode {
            U32OpMode::Checked => {
                // Assume the dynamic shift value b is on top of the stack.
                append_pow2_op(span);
                span.push_op(U32assert2);
            }
            U32OpMode::Unchecked => {
                append_pow2_op(span);
            }
            _ => unreachable!("unsupported operation mode"),
        },
        Some(imm) => match op_mode {
            U32OpMode::Checked => {
                if imm > 31 {
                    return Err(AssemblerError::imm_out_of_bounds(imm as u64, 0, 31));
                }
                span.push_op(Push(Felt::new(2u64.pow(imm as u32))));
                span.push_op(U32assert2);
            }
            U32OpMode::Unchecked => {
                span.push_op(Push(Felt::new(2u64.pow(imm as u32))));
            }
            _ => unreachable!("unsupported operation mode"),
        },
    }

    span.push_op(U32mul);
    span.push_op(Add);

    Ok(None)
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
) -> Result<Option<CodeBlock>, AssemblerError> {
    match imm {
        None => match op_mode {
            U32OpMode::Checked => {
                // Verify both b and a are u32.
                span.push_op(U32assert2);
                // Calculate 32 - b and assert that the shift value b <= 31.
                span.push_op(Push(Felt::new(31)));
                span.push_op(Dup1);
                span.push_op(U32sub);
                span.push_op(Not);
                span.push_op(Assert);
                span.push_op(Incr);
                span.push_op(Dup1);
                // If 32-b = 32, replace it with 0.
                span.push_op(Eqz);
                span.push_op(Not);
                span.push_op(CSwap);
                span.push_op(Drop);
                append_pow2_op(span);
                span.push_op(Swap);
            }
            U32OpMode::Unchecked => {
                span.push_op(Push(Felt::new(32)));
                span.push_op(Swap);
                span.push_op(U32sub);
                span.push_op(Drop);
                append_pow2_op(span);
            }
            _ => unreachable!("unsupported operation mode"),
        },
        Some(imm) => match op_mode {
            U32OpMode::Checked => {
                // Assert the top of the stack is a u32 value.
                // NOTE: We cannot use U32Assert2 since we are potentially pushing a number larger
                // than u32 for b.
                // TODO: double check this logic: we shouldn't use U32MUL if both inputs are not
                // guaranteed to be u32 values
                span.push_op(Pad);
                span.push_op(U32assert2);
                span.push_op(Drop);

                if imm > 31 {
                    return Err(AssemblerError::imm_out_of_bounds(imm as u64, 0, 31));
                }
                span.push_op(Push(Felt::new(2u64.pow(32 - imm as u32))));
            }
            U32OpMode::Unchecked => {
                span.push_op(Push(Felt::new(2u64.pow(32 - imm as u32))));
            }
            _ => unreachable!("unsupported operation mode"),
        },
    }

    span.push_op(U32mul);
    span.push_op(Add);

    Ok(None)
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
) -> Result<Option<CodeBlock>, AssemblerError> {
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
        span.push_op(Eqz);
        span.push_op(Assert);
    } else if drop_high_bits {
        span.push_op(Drop);
    }

    Ok(None)
}

/// Handles common parts of u32div, u32mod, and u32divmod operations in checked and unchecked modes,
/// including handling of immediate parameters.
fn handle_division(
    span: &mut SpanBuilder,
    op_mode: U32OpMode,
    imm: Option<u32>,
) -> Result<(), AssemblerError> {
    if let Some(imm) = imm {
        if imm == 0 {
            return Err(AssemblerError::division_by_zero());
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

    span.push_op(U32div);

    Ok(())
}

fn push_u32_value(span: &mut SpanBuilder, value: u32) {
    if value == 0 {
        span.push_op(Pad);
    } else if value == 1 {
        span.push_op(Pad);
        span.push_op(Incr);
    } else {
        span.push_op(Push(Felt::from(value)));
    }
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
pub fn u32eq(
    span: &mut SpanBuilder,
    imm: Option<u32>,
) -> Result<Option<CodeBlock>, AssemblerError> {
    match imm {
        Some(imm) => assert_and_push_u32_param(span, imm, 0)?,
        None => span.push_op(U32assert2),
    }

    span.push_op(Eq);

    Ok(None)
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
) -> Result<Option<CodeBlock>, AssemblerError> {
    match imm {
        Some(imm) => assert_and_push_u32_param(span, imm, 0)?,
        None => span.push_op(U32assert2),
    }

    span.push_op(Eq);
    span.push_op(Not);

    Ok(None)
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
) -> Result<Option<CodeBlock>, AssemblerError> {
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
) -> Result<Option<CodeBlock>, AssemblerError> {
    handle_u32_and_unchecked_mode(span, op_mode);

    // Compute the lt with reversed number to get a gt check
    span.push_op(Swap);
    compute_lt(span);

    // Flip the final results to get the lte results.
    span.push_op(Not);

    Ok(None)
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
) -> Result<Option<CodeBlock>, AssemblerError> {
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
) -> Result<Option<CodeBlock>, AssemblerError> {
    handle_u32_and_unchecked_mode(span, op_mode);

    compute_lt(span);

    // Flip the final results to get the gte results.
    span.push_op(Not);

    Ok(None)
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
) -> Result<Option<CodeBlock>, AssemblerError> {
    compute_max_and_min(span, op_mode);

    // Drop the max and keep the min
    span.push_op(Drop);

    Ok(None)
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
) -> Result<Option<CodeBlock>, AssemblerError> {
    compute_max_and_min(span, op_mode);

    // Drop the min and keep the max
    span.push_op(Swap);
    span.push_op(Drop);

    Ok(None)
}

// COMPARISON OPERATIONS - HELPERS
// ================================================================================================

/// This is a helper function that appends a PUSH operation to the span block which puts the
/// provided value parameter onto the stack.
///
/// When the value is 0, PUSH operation is replaced with PAD. When the value is 1, PUSH operation
/// is replaced with PAD INCR because in most cases this will be more efficient than doing a PUSH.
fn push_value(span: &mut SpanBuilder, value: Felt) {
    if value == Felt::ZERO {
        span.push_op(Pad);
    } else if value == Felt::ONE {
        span.push_op(Pad);
        span.push_op(Incr);
    } else {
        span.push_op(Push(value));
    }
}

/// Asserts that the value on the top of the stack is a u32 and pushes the first param of the `op`
/// as a u32 value onto the stack.
///
/// This takes:
/// - 3 VM cycles when the param == 1.
/// - 2 VM cycle when the param != 1.
///
/// # Errors
/// Returns an error if the first parameter of the `op` is not a u32 value or is greater than
/// `lower_bound`.
fn assert_and_push_u32_param(
    span: &mut SpanBuilder,
    lower_bound: u32,
    value: u32,
) -> Result<(), AssemblerError> {
    // TODO: We should investigate special case handling adding 0 or 1.
    // check that the parameter is within the specified bounds
    if value < lower_bound {
        return Err(AssemblerError::imm_out_of_bounds(
            value as u64,
            lower_bound as u64,
            u32::MAX as u64,
        ));
    }

    push_value(span, Felt::new(value as u64));

    // Assert both numbers are u32.
    span.push_op(U32assert2);

    Ok(())
}

/// Handles u32 assertion and unchecked mode for any u32 operation.
fn handle_u32_and_unchecked_mode(span: &mut SpanBuilder, op_mode: U32OpMode) {
    if op_mode == U32OpMode::Checked {
        span.push_op(U32assert2);
    }
}

/// Inserts the VM operations to check if the second element is less than
/// the top element. This takes 5 cycles.
fn compute_lt(span: &mut SpanBuilder) {
    span.push_op(U32sub);
    span.push_op(Swap);
    span.push_op(Drop);

    // Check the underflow flag
    span.push_op(Eqz);
    span.push_op(Not);
}

/// Duplicate the top two elements in the stack and check both are u32, and determine the min
/// and max between them.
///
/// The maximum number will be at the top of the stack and minimum will be at the 2nd index.
fn compute_max_and_min(span: &mut SpanBuilder, op_mode: U32OpMode) {
    // Copy top two elements of the stack.
    span.push_op(Dup1);
    span.push_op(Dup1);
    if op_mode == U32OpMode::Checked {
        span.push_op(U32assert2);
    }

    span.push_op(U32sub);
    span.push_op(Swap);
    span.push_op(Drop);

    // Check the underflow flag, if it's zero
    // then the second number is equal or larger than the first.
    span.push_op(Eqz);
    span.push_op(CSwap);
}
