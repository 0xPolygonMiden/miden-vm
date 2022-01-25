use super::{parse_int_param, push_value, AssemblyError, BaseElement, Operation, Token};

// CONVERSIONS AND TESTS
// ================================================================================================

/// Translates u32test assembly instruction to VM operation DUP U32SPLIT SWAP DROP EQZ.
pub fn parse_u32test(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            span_ops.push(Operation::Dup0);
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::Drop);
            span_ops.push(Operation::Eqz);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32testw assembly instruction to VM operation
/// with a series of DUP U32SPLIT SWAP DROP EQZ on each element in the word.
pub fn parse_u32testw(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            // Test the fourth element
            span_ops.push(Operation::Dup3);
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::Drop);
            span_ops.push(Operation::Eqz);
            span_ops.push(Operation::Not);

            // Test the third element
            span_ops.push(Operation::Dup3);
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::Drop);
            span_ops.push(Operation::Eqz);
            span_ops.push(Operation::Not);
            span_ops.push(Operation::Or);

            // Test the second element
            span_ops.push(Operation::Dup2);
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::Drop);
            span_ops.push(Operation::Eqz);
            span_ops.push(Operation::Or);

            // Test the first element
            span_ops.push(Operation::Dup1);
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::Drop);
            span_ops.push(Operation::Eqz);
            span_ops.push(Operation::Not);
            span_ops.push(Operation::Or);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32assert assembly instruction to VM operation U32SPLIT EQZ ASSERT.
pub fn parse_u32assert(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            assert_u32(span_ops);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32assert assembly instruction to VM operation
/// into a series of U32SPLIT EQZ ASSERT on each element in the word.
pub fn parse_u32assertw(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            // Test the first element
            assert_u32(span_ops);

            // Test the second element
            span_ops.push(Operation::Swap);
            assert_u32(span_ops);

            // Test the third element
            span_ops.push(Operation::MovUp2);
            assert_u32(span_ops);

            // Test the fourth element
            span_ops.push(Operation::MovUp3);
            assert_u32(span_ops);

            // Move the elements back into place
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::MovUp2);
            span_ops.push(Operation::MovUp3);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32cast assembly instruction to VM operation U32SPLIT DROP.
pub fn parse_u32cast(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Drop);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32split assembly instruction to VM operation U32SPLIT.
pub fn parse_u32split(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32split),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

// ARITHMETIC OPERATIONS
// ================================================================================================

/// Translates u32add assembly instruction to VM operation U32ADD.
/// Depending on the different mode, additional instructions will be inserted.
/// Please refer to the docs of `handle_arthimetic_operation` for more details.
/// VM cycles per mode:
/// - u32add: 10 cycles
/// - u32add.b: 7 cycles
/// - u32add.full: 8 cycles
/// - u32add.unsafe: 1 cycle
pub fn parse_u32add(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    handle_arthimetic_operation(span_ops, op, Operation::U32add, false)
}

/// Translates u32addc assembly instruction to VM operation U32ADDC.
/// VM cycles per mode:
/// - u32addc: 8 cycles
/// - u32addc.unsafe: 1 cycle
pub fn parse_u32addc(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            assert_u32(span_ops);
            span_ops.push(Operation::Swap);
            assert_u32(span_ops);
        }
        2 => {
            if op.parts()[1] != "unsafe" {
                return Err(AssemblyError::invalid_param(op, 1));
            }
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    span_ops.push(Operation::U32addc);

    Ok(())
}

/// Translates u32sub assembly instruction to VM operation U32SUB.
/// Depending on the different mode, additional instructions will be inserted.
/// Please refer to the docs of `handle_arthimetic_operation` for more details.
/// VM cycles per mode:
/// - u32sub: 11 cycles
/// - u32sub.b: 7 cycles
/// - u32sub.full: 9 cycles
/// - u32sub.unsafe: 1 cycle
pub fn parse_u32sub(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    handle_arthimetic_operation(span_ops, op, Operation::U32sub, true)
}

/// Translates u32mul assembly instruction to VM operation U32MUL.
/// Depending on the different mode, additional instructions will be inserted.
/// Please refer to the docs of `handle_arthimetic_operation` for more details.
/// VM cycles per mode:
/// - u32mul: 10 cycles
/// - u32mul.b: 7 cycles
/// - u32mul.full: 8 cycles
/// - u32mul.unsafe: 1 cycle
pub fn parse_u32mul(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    handle_arthimetic_operation(span_ops, op, Operation::U32mul, false)
}

/// Translates u32madd assembly instruction to VM operation U32MADD.
/// In safe mode, we assert all three nummbers are u32.
/// VM cycles per mode:
/// - u32madd: 12 cycles
/// - u32madd.unsafe: 1 cycle
pub fn parse_u32madd(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            assert_u32(span_ops);
            span_ops.push(Operation::Swap);
            assert_u32(span_ops);
            span_ops.push(Operation::MovUp2);
            assert_u32(span_ops);
            span_ops.push(Operation::MovDn2);
        }
        2 => {
            if op.parts()[1] != "unsafe" {
                return Err(AssemblyError::invalid_param(op, 1));
            }
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    span_ops.push(Operation::U32madd);

    Ok(())
}

/// Translates u32addc assembly instruction to VM operation U32DIV.
/// Depending on the different mode, additional instructions will be inserted.
/// Please refer to the docs of `handle_arthimetic_operation` for more details.
/// VM cycles per mode:
/// - u32div: 11 cycles
/// - u32div.b: 7 cycles
/// - u32div.full: 9 cycles
/// - u32div.unsafe: 1 cycle
pub fn parse_u32div(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    handle_arthimetic_operation(span_ops, op, Operation::U32div, true)
}

/// Translates u32mod assembly instruction to VM operation U32DIV SWAP DROP.
/// VM cycles per mode:
/// - u32mod: 12 cycles
/// - u32mod.b: 8 cycles
/// - u32mod.unsafe: 3 cycles
pub fn parse_u32mod(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    // prepare the stack for the operation and determine if we need to check for overflow
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            // for u32mod we need to make sure operands are u32 values,
            assert_u32(span_ops);
            span_ops.push(Operation::Swap);
            assert_u32(span_ops);
            span_ops.push(Operation::Swap);
        }
        2 => match op.parts()[1] {
            // skip u32 check in unsafe mode
            "unsafe" => (),
            _ => {
                // for u32mod.n (where n is the immediate value), we need to push the immediate
                // value onto the stack, and make sure both operands are u32 values.
                push_u32_param(span_ops, op, 1)?;
            }
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    // perform the mod
    span_ops.push(Operation::U32div);
    span_ops.push(Operation::Swap);
    span_ops.push(Operation::Drop);

    Ok(())
}

// BITWISE OPERATIONS
// ================================================================================================

/// Translates u32and assembly instruction to VM operation U32AND.
pub fn parse_u32and(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32and),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32or assembly instruction to VM operation U32OR.
pub fn parse_u32or(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32or),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32xor assembly instruction to VM operation U32XOR.
pub fn parse_u32xor(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32xor),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32not assembly instruction to VM operation PUSH(2^32) SWAP INCR U32SUB.
/// The reason this works is because 2^32 provides a bit mask of ones, which after
/// subtracting the element, flips the bits of the original value to perform a bitwise NOT.
pub fn parse_u32not(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            // Assert the value is u32
            assert_u32(span_ops);

            span_ops.push(Operation::Push(BaseElement::new(2u64.pow(32))));
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::Incr);
            span_ops.push(Operation::U32sub);

            // Drop the underflow flag
            span_ops.push(Operation::Drop);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32shl.x assembly instruction to VM operation PUSH(2^x) MUL U32SPLIT DROP.
pub fn parse_u32shl(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0..=1 => return Err(AssemblyError::missing_param(op)),
        2 => {
            // Assert the value is u32
            assert_u32(span_ops);

            let x = parse_int_param(op, 1, 1, 31)?;
            span_ops.push(Operation::Push(BaseElement::new(2u64.pow(x))));
            span_ops.push(Operation::Mul);
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Drop);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32shl.x assembly instruction to VM operation PUSH(2^x) U32DIV SWAP DROP.
pub fn parse_u32shr(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0..=1 => return Err(AssemblyError::missing_param(op)),
        2 => {
            assert_u32(span_ops);

            let x = parse_int_param(op, 1, 1, 31)?;
            span_ops.push(Operation::Push(BaseElement::new(2u64.pow(x))));
            span_ops.push(Operation::U32div);
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::Drop);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32rotl.x assembly instruction to VM operation PUSH(2^x) MUL U32SPLIT ADD.
pub fn parse_u32rotl(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0..=1 => return Err(AssemblyError::missing_param(op)),
        2 => {
            assert_u32(span_ops);

            let x = parse_int_param(op, 1, 1, 31)?;
            span_ops.push(Operation::Push(BaseElement::new(2u64.pow(x))));
            span_ops.push(Operation::Mul);
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Add);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32rotr.x assembly instruction to VM operation PUSH(2^(32-x)) MUL U32SPLIT ADD.
pub fn parse_u32rotr(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0..=1 => return Err(AssemblyError::missing_param(op)),
        2 => {
            assert_u32(span_ops);

            let x = parse_int_param(op, 1, 1, 31)?;
            span_ops.push(Operation::Push(BaseElement::new(2u64.pow(32 - x))));
            span_ops.push(Operation::Mul);
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Add);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

// COMPARISON OPERATIONS
// ================================================================================================

/// Translates u32eq assembly instruction to VM operations.
/// Specifically we test the first two numbers to be u32,
/// then perform a EQ to check the equality.
pub fn parse_u32eq(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            // Check first number is u32
            assert_u32(span_ops);

            span_ops.push(Operation::Swap);

            // Check second number is u32
            assert_u32(span_ops);
        }
        2 => {
            push_u32_param(span_ops, op, 0)?;
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    span_ops.push(Operation::Eq);

    Ok(())
}

/// Translates u32neq assembly instruction to VM operations.
/// Specifically we test the first two numbers to be u32 (U32SPLIT NOT ASSERT),
/// then perform a EQ NOT to check the equality.
pub fn parse_u32neq(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            // Check first number is u32
            assert_u32(span_ops);

            span_ops.push(Operation::Swap);

            // Check second number is u32
            assert_u32(span_ops);
        }
        2 => {
            push_u32_param(span_ops, op, 0)?;
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    span_ops.push(Operation::Eq);
    span_ops.push(Operation::Not);

    Ok(())
}

/// Translates u32lt assembly instruction to VM operations.
/// Specifically we test the first two numbers to be u32 (U32SPLIT NOT ASSERT),
/// then perform a U32SUB EQZ NOT to check the underflow flag.
pub fn parse_u32lt(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            // Check first number is u32
            assert_u32(span_ops);
            span_ops.push(Operation::Swap);

            // Check second number is u32
            assert_u32(span_ops);
            span_ops.push(Operation::Swap);

            compute_lt(span_ops);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32lte assembly instruction to VM operations.
/// Specifically we test the first two numbers to be u32 (U32SPLIT NOT ASSERT),
/// then perform a gt check and flip the results.
pub fn parse_u32lte(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            // Check first number is u32
            assert_u32(span_ops);
            span_ops.push(Operation::Swap);

            // Check second number is u32
            assert_u32(span_ops);

            // Compute the lt with reversed number to get a gt check
            compute_lt(span_ops);

            // Flip the final results to get the lte results.
            span_ops.push(Operation::Not);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32gt assembly instruction to VM operations.
/// Specifically we test the first two numbers to be u32 (U32SPLIT NOT ASSERT),
/// then perform a lt check with the numbers swapped.
pub fn parse_u32gt(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            // Check first number is u32
            assert_u32(span_ops);

            span_ops.push(Operation::Swap);

            // Check second number is u32
            assert_u32(span_ops);

            // We skip the swap which reverses the order of the numbers,
            // so a lt check here becomes gt.
            compute_lt(span_ops);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32gte assembly instruction to VM operations.
/// Specifically we test the first two numbers to be u32 (U32SPLIT NOT ASSERT),
/// then compute a lt check and flip the results.
pub fn parse_u32gte(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            // Check first number is u32
            assert_u32(span_ops);
            span_ops.push(Operation::Swap);

            // Check second number is u32
            assert_u32(span_ops);
            span_ops.push(Operation::Swap);

            compute_lt(span_ops);
            span_ops.push(Operation::Not);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32min assembly instruction to VM operations.
/// Specifically we test the first two numbers to be u32 (U32SPLIT NOT ASSERT),
/// and subtract both numbers (U32SUB), check the underflow flag (EQZ),
/// and perform a conditional swap (CSWAP) to have the max number in front,
/// then we finally drop the top element to keep the min.
pub fn parse_u32min(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            compute_max_and_min(span_ops);
            // Drop the max and keep the min
            span_ops.push(Operation::Drop);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32min assembly instruction to VM operations.
/// Specifically we test the first two numbers to be u32 (U32SPLIT NOT ASSERT),
/// and subtract both numbers (U32SUB), check the underflow flag (EQZ),
/// and perform a conditional swap (CSWAP) to have the max number in front,
/// then we finally drop the 2nd element to keep the max.
pub fn parse_u32max(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            compute_max_and_min(span_ops);

            // Drop the min and keep the max
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::Drop);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

// HELPER FUNCTIONS
// ================================================================================================
/// Assert if the current number is u32.
fn assert_u32(span_ops: &mut Vec<Operation>) {
    span_ops.push(Operation::U32split);
    span_ops.push(Operation::Eqz);
    span_ops.push(Operation::Assert);
}

/// Duplicate the first two numbers in the stack, check they are both u32,
/// and determine the min and max between them.
/// The maximum number will be at the top of the stack and  minimum will be at the 2nd index.
fn compute_max_and_min(span_ops: &mut Vec<Operation>) {
    // Check second number is u32
    span_ops.push(Operation::Dup1);
    assert_u32(span_ops);

    // Check first number is u32
    span_ops.push(Operation::Dup1);
    assert_u32(span_ops);

    span_ops.push(Operation::U32sub);
    span_ops.push(Operation::Swap);
    span_ops.push(Operation::Drop);
    // Check the underflow flag, if it's zero
    // then the second number is equal or larger than the first.
    span_ops.push(Operation::Eqz);
    span_ops.push(Operation::CSwap);
}

/// Inserts the VM operations to check if the second element is less than
/// the top element.
fn compute_lt(span_ops: &mut Vec<Operation>) {
    span_ops.push(Operation::U32sub);
    span_ops.push(Operation::Swap);
    span_ops.push(Operation::Drop);

    // Check the underflow flag
    span_ops.push(Operation::Eqz);
    span_ops.push(Operation::Not);
}

/// Pushes the first param as a u32 int into the top of the stack
fn push_u32_param(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    lower_bound: u32,
) -> Result<(), AssemblyError> {
    // assert first number is u32.
    assert_u32(span_ops);
    // TODO: We should investigate special case handling adding 0 or 1.
    let value = parse_int_param(op, 1, lower_bound, u32::MAX)?;
    push_value(span_ops, BaseElement::new(value as u64));

    Ok(())
}

/// Handles arthimetic operation that needs support for unsafe, full, operation and
/// operation.n modes.
/// Specifically handles these specific inputs per the spec.
/// - Zero argument: Assert the top two elemenets are u32 and push the result after  to the stack
/// - Single argument:
///   - "unsafe" skips the assert check and direclty performs the operation
///   - "full" checks both numbers are u32 and perform the same operations as "unsafe"
///   - Any number argument gets pushed to the stack, checked if both are u32 and performs the operation.
///
/// According to the spec this is currently U32ADD, U32SUB, U32DIV, U32MUL.
fn handle_arthimetic_operation(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    arithmetic_op: Operation,
    preseve_order: bool,
) -> Result<(), AssemblyError> {
    // prepare the stack for the operation and determine if we need to check for overflow
    let assert_u32_result = match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            // for simple arthimetic operation we need to make sure operands are u32 values,
            // and we need to make sure that the result will be a u32 value as well
            assert_u32(span_ops);
            span_ops.push(Operation::Swap);
            assert_u32(span_ops);
            if preseve_order {
                span_ops.push(Operation::Swap);
            }
            true
        }
        2 => match op.parts()[1] {
            "unsafe" => false,
            "full" => {
                // for full we need to make sure operands are u32 values, but we don't
                // need to check the result for overflow because we return both high and low bits
                // of the result
                assert_u32(span_ops);
                span_ops.push(Operation::Swap);
                assert_u32(span_ops);
                if preseve_order {
                    span_ops.push(Operation::Swap);
                }
                false
            }
            _ => {
                // for operation.n (where n is the immediate value), we need to push the immediate
                // value onto the stack, and make sure both operands are u32 values. we also want
                // to make sure the result is a u32 value.
                push_u32_param(span_ops, op, 0)?;
                true
            }
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    // perform the operation
    span_ops.push(arithmetic_op);

    // make sure the result is a u32 value, and drop the high bits
    if assert_u32_result {
        span_ops.push(Operation::Eqz);
        span_ops.push(Operation::Assert);
    }

    Ok(())
}
