use super::{parse_int_param, push_value, AssemblyError, Felt, Operation, Token};

// CONVERSIONS AND TESTS
// ================================================================================================

/// Translates u32test assembly instruction to VM operations.
///
/// Implemented as: `DUP U32SPLIT SWAP DROP EQZ` (5 VM cycles).
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

/// Translates u32testw assembly instruction to VM operations.
///
/// Implemented by executing DUP U32SPLIT SWAP DROP EQZ on each element in the word
/// and combining the results using AND operation (total of 23 VM cycles)
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

            // Test the third element
            span_ops.push(Operation::Dup3);
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::Drop);
            span_ops.push(Operation::Eqz);
            span_ops.push(Operation::And);

            // Test the second element
            span_ops.push(Operation::Dup2);
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::Drop);
            span_ops.push(Operation::Eqz);
            span_ops.push(Operation::And);

            // Test the first element
            span_ops.push(Operation::Dup1);
            span_ops.push(Operation::U32split);
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::Drop);
            span_ops.push(Operation::Eqz);
            span_ops.push(Operation::And);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32assert assembly instruction to VM operations.
///
/// u32assert, u32assert.1: Implemented as: `PAD U32ASSERT2 DROP` (3 VM cycles).
/// u32assert.2: Implemented as: `U32assert2` (1 VM cycles).
pub fn parse_u32assert(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => assert_u32(span_ops),
        2 => match op.parts()[1] {
            "1" => assert_u32(span_ops),
            "2" => span_ops.push(Operation::U32assert2),
            _ => return Err(AssemblyError::invalid_param(op, 1)),
        },
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32assert assembly instruction to VM operations.
///
/// Implemented by executing `U32ASSERT2` on each pair of elements in the word.
/// Total of 6 VM cycles.
pub fn parse_u32assertw(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            // Test the first and the second elements
            span_ops.push(Operation::U32assert2);

            // Move 3 and 4 to the top of the stack
            span_ops.push(Operation::MovUp3);
            span_ops.push(Operation::MovUp3);

            // Test them
            span_ops.push(Operation::U32assert2);

            // Move the elements back into place
            span_ops.push(Operation::MovUp3);
            span_ops.push(Operation::MovUp3);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32cast assembly instruction to VM operations.
///
/// Implemented as: `U32SPLIT DROP` (2 VM cycles).
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

/// Translates u32split assembly instruction to VM operations.
///
/// Implement as: `U32SPLIT` (1 VM cycle).
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

/// Translates u32add assembly instruction to VM operations.
///
/// The base operation is `U32ADD`, but depending on the mode, additional operations may be
/// inserted. Please refer to the docs of `handle_arithmetic_operation` for more details.
///
/// VM cycles per mode:
/// - u32add: 4 cycles
/// - u32add.b: 5 cycles
/// - u32add.full: 2 cycles
/// - u32add.unsafe: 1 cycle
pub fn parse_u32add(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    handle_arithmetic_operation(span_ops, op, Operation::U32add)
}

/// Translates u32add3 assembly instruction to VM operations.
///
/// In the unsafe mode this translates directly to `U32ADD3` operation. In the safe mode,
/// we also assert that all three inputs are u32 values.
///
/// VM cycles per mode:
/// - u32add3: 4 cycles
/// - u32add3.unsafe: 1 cycle
pub fn parse_u32add3(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            span_ops.push(Operation::U32assert2);
            span_ops.push(Operation::MovUp2);
            span_ops.push(Operation::U32assert2);
        }
        2 => {
            if op.parts()[1] != "unsafe" {
                return Err(AssemblyError::invalid_param(op, 1));
            }
        }
        _ => return Err(AssemblyError::extra_param(op)),
    };

    span_ops.push(Operation::U32add3);

    Ok(())
}

/// Translates u32sub assembly instruction to VM operations.
///
/// The base operation is `U32SUB`, but depending on the mode, additional operations may be
/// inserted. Please refer to the docs of `handle_arithmetic_operation` for more details.
///
/// VM cycles per mode:
/// - u32sub: 4 cycles
/// - u32sub.b: 5 cycles
/// - u32sub.full: 2 cycles
/// - u32sub.unsafe: 1 cycle
pub fn parse_u32sub(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    handle_arithmetic_operation(span_ops, op, Operation::U32sub)
}

/// Translates u32mul assembly instruction to VM operations.
///
/// The base operation is `U32MUL`, but depending on the mode, additional operations may be
/// inserted. Please refer to the docs of `handle_arithmetic_operation` for more details.
///
/// VM cycles per mode:
/// - u32mul: 4 cycles
/// - u32mul.b: 5 cycles
/// - u32mul.full: 2 cycles
/// - u32mul.unsafe: 1 cycle
pub fn parse_u32mul(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    handle_arithmetic_operation(span_ops, op, Operation::U32mul)
}

/// Translates u32madd assembly instruction to VM operations.
///
/// In the unsafe mode this translates directly to `U32MADD` operation. In the safe mode,
/// we also assert that all three inputs are u32 values.
///
/// VM cycles per mode:
/// - u32madd: 5 cycles
/// - u32madd.unsafe: 1 cycle
pub fn parse_u32madd(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            // make sure all 3 values at the top of the stack are u32 values; swapping the order
            // of the first two values is ok, but the 3rd value should remain in place.
            span_ops.push(Operation::U32assert2);
            span_ops.push(Operation::MovUp2);
            span_ops.push(Operation::U32assert2);
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

/// Translates u32div assembly instruction to VM operations.
///
/// The base operation is `U32DIV`, but depending on the mode, additional operations may be
/// inserted.
///
/// VM cycles per mode:
/// - u32div: 3 cycles
/// - u32div.b: 4 cycles
/// - u32div.full: 2 cycles
/// - u32div.unsafe: 1 cycle
pub fn parse_u32div(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    let drop_remainder = match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            span_ops.push(Operation::U32assert2);
            true
        }
        2 => match op.parts()[1] {
            "unsafe" => false,
            "full" => {
                span_ops.push(Operation::U32assert2);
                false
            }
            _ => {
                let divisor: u32 = parse_int_param(op, 1, 0, u32::MAX)?;
                if divisor == 0 {
                    return Err(AssemblyError::invalid_param_with_reason(
                        op,
                        1,
                        "division by 0",
                    ));
                }

                push_value(span_ops, Felt::new(divisor as u64));
                span_ops.push(Operation::U32assert2);
                true
            }
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    span_ops.push(Operation::U32div);

    if drop_remainder {
        span_ops.push(Operation::Drop);
    }

    Ok(())
}

/// Translates u32mod assembly instruction to VM operations.
///
/// In the unsafe mode this translates directly to `U32DIV SWAP DROP` operation. In the safe mode,
/// we also assert that both inputs are u32 values.
///
/// VM cycles per mode:
/// - u32mod: 4 cycles
/// - u32mod.b: 5 cycles
/// - u32mod.unsafe: 3 cycles
pub fn parse_u32mod(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    // prepare the stack for the operation and determine if we need to check for overflow
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32assert2),
        2 => match op.parts()[1] {
            // skip u32 check in unsafe mode
            "unsafe" => (),
            _ => {
                // for u32mod.n (where n is the immediate value), we need to push the immediate
                // value onto the stack, and make sure both operands are u32 values.
                let divisor: u32 = parse_int_param(op, 1, 0, u32::MAX)?;
                if divisor == 0 {
                    return Err(AssemblyError::invalid_param_with_reason(
                        op,
                        1,
                        "division by 0",
                    ));
                }

                push_value(span_ops, Felt::new(divisor as u64));
                span_ops.push(Operation::U32assert2);
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

/// Translates u32and assembly instruction to VM operation.
///
/// Implemented as: `U32AND` (1 VM cycle).
///
/// We don't need to assert that inputs are u32 values because the VM does these assertions
/// implicitly for `U32AND` operation.
pub fn parse_u32and(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32and),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32or assembly instruction to VM operation `U32OR`.
///
/// Implemented as: `U32OR` (1 VM cycle).
///
/// We don't need to assert that inputs are u32 values because the VM does these assertions
/// implicitly for `U32OR` operation.
pub fn parse_u32or(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32or),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32xor assembly instruction to VM operation `U32XOR`.
///
/// Implemented as: `U32XOR` (1 VM cycle).
///
/// We don't need to assert that inputs are u32 values because the VM does these assertions
/// implicitly for `U32XOR` operation.
pub fn parse_u32xor(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32xor),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32not assembly instruction to VM operations.
///
/// The operation is implemented as `PUSH(2^32 - 1) U32ASSERT2 SWAP U32SUB DROP`,
/// total to 5 cycles.
///
/// The reason this method works is because 2^32 provides a bit mask of ones, which after
/// subtracting the element, flips the bits of the original value to perform a bitwise NOT.
pub fn parse_u32not(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            // perform the operation
            span_ops.push(Operation::Push(Felt::new(2u64.pow(32) - 1)));
            span_ops.push(Operation::U32assert2);
            span_ops.push(Operation::Swap);
            span_ops.push(Operation::U32sub);

            // Drop the underflow flag
            span_ops.push(Operation::Drop);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

/// Translates u32shl assembly instruction to VM operations.
///
/// The operation is implemented by putting a power of 2 on the stack, then multiplying it with
/// the value to be shifted and splitting the result. Depending on the mode, other instructions may
/// be added, and the return value may or may not include an overflow result. For safe variations,
/// the shift value is asserted to be between 0-31 and the value to be shifted is asserted to be a
/// 32-bit value.
///
/// VM cycles per mode:
/// - u32shl: 4 cycles
/// - u32shl.b: 4 cycles
/// - u32shl.unsafe: 2 cycles
pub fn parse_u32shl(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    let drop_remainder = match op.num_parts() {
        0 => return Err(AssemblyError::invalid_op(op)),
        1 => {
            // Assume the dynamic shift value b is on top of the stack.
            span_ops.push(Operation::Pow2);
            span_ops.push(Operation::U32assert2);
            true
        }
        2 => match op.parts()[1] {
            "unsafe" => {
                span_ops.push(Operation::Pow2);
                false
            }
            _ => {
                let x = parse_int_param(op, 1, 0, 31)?;
                span_ops.push(Operation::Push(Felt::new(2u64.pow(x))));
                span_ops.push(Operation::U32assert2);
                true
            }
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    span_ops.push(Operation::U32mul);

    if drop_remainder {
        span_ops.push(Operation::Drop);
    }

    Ok(())
}

/// Translates u32shr assembly instruction to VM operations.
///
/// The safe modes of the operation are implemented by putting a power of 2 on the stack, then
/// dividing the value to be shifted by it and returning the quotient. For unsafe mode, a left shift
/// is implemented via multiplication and both the shifted value and the overflow shift are
/// returned. For safe variations, the shift value is asserted to be between 0-31 and the value to
/// be shifted is asserted to be a 32-bit value.
///
/// VM cycles per mode:
/// - u32shr: 4 cycles
/// - u32shr.b: 4 cycles
/// - u32shr.unsafe: 7 cycles
pub fn parse_u32shr(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    let drop_remainder = match op.num_parts() {
        0 => return Err(AssemblyError::invalid_op(op)),
        1 => {
            // Assume the dynamic shift value b is on top of the stack.
            span_ops.push(Operation::Pow2);
            span_ops.push(Operation::U32assert2);
            true
        }
        2 => match op.parts()[1] {
            "unsafe" => {
                // Use multiplication to shift left so the right-shifted result and the overflow
                // shift can both be returned.
                span_ops.push(Operation::Push(Felt::new(32)));
                span_ops.push(Operation::Swap);
                span_ops.push(Operation::U32sub);
                span_ops.push(Operation::Drop);
                span_ops.push(Operation::Pow2);
                span_ops.push(Operation::U32mul);
                span_ops.push(Operation::Swap);
                false
            }
            _ => {
                let x = parse_int_param(op, 1, 0, 31)?;
                span_ops.push(Operation::Push(Felt::new(2u64.pow(x))));
                span_ops.push(Operation::U32assert2);
                true
            }
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    if drop_remainder {
        // Use division to shift right when no overflow result is required and only a single
        // shifted value is returned. This excludes "unsafe" mode, which is done above via mul.
        span_ops.push(Operation::U32div);
        // drop the remainder and keep the quotient
        span_ops.push(Operation::Drop);
    }

    Ok(())
}

/// Translates u32rotl assembly instruction to VM operations.
///
/// The base operation is implemented by putting a power of 2 on the stack, then multiplying the
/// value to be shifted by it and adding the overflow limb to the shifted limb. Depending on the
/// mode, other instructions may be added. For safe variations, the shift value is asserted to be
/// between 0-31 and the value to be shifted is asserted to be a 32-bit value.
///
/// VM cycles per mode:
/// - u32rotl: 4 cycles
/// - u32rotl.b: 4 cycles
/// - u32rotl.unsafe: 3 cycles
pub fn parse_u32rotl(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::invalid_op(op)),
        1 => {
            // Assume the dynamic shift value b is on top of the stack.
            span_ops.push(Operation::Pow2);
            span_ops.push(Operation::U32assert2);
        }
        2 => match op.parts()[1] {
            "unsafe" => {
                span_ops.push(Operation::Pow2);
            }
            _ => {
                let x = parse_int_param(op, 1, 0, 31)?;
                span_ops.push(Operation::Push(Felt::new(2u64.pow(x))));
                span_ops.push(Operation::U32assert2);
            }
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    span_ops.push(Operation::U32mul);
    span_ops.push(Operation::Add);

    Ok(())
}

/// Translates u32rotr assembly instruction to VM operations.
///
/// The base operation is implemented by multiplying the value to be shifted by 2^(32-b), where b is
/// the shift amount, then adding the overflow limb to the shifted limb. Depending on the mode,
/// other instructions may be added. For safe variations, the shift value is asserted to be between
/// 0-31 and the value to be shifted is asserted to be a 32-bit value.
///
/// VM cycles per mode:
/// - u32rotr: 16 cycles
/// - u32rotr.b: 6 cycles
/// - u32rotr.unsafe: 7 cycles
pub fn parse_u32rotr(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::invalid_op(op)),
        1 => {
            // Verify both b and a are u32.
            span_ops.push(Operation::U32assert2);
            // Calculate 32 - b and assert that the shift value b <= 31.
            span_ops.push(Operation::Push(Felt::new(31)));
            span_ops.push(Operation::Dup1);
            span_ops.push(Operation::U32sub);
            span_ops.push(Operation::Not);
            span_ops.push(Operation::Assert);
            span_ops.push(Operation::Incr);
            span_ops.push(Operation::Dup1);
            // If 32-b = 32, replace it with 0.
            span_ops.push(Operation::Eqz);
            span_ops.push(Operation::Not);
            span_ops.push(Operation::CSwap);
            span_ops.push(Operation::Drop);
            span_ops.push(Operation::Pow2);
            span_ops.push(Operation::Swap);
        }
        2 => match op.parts()[1] {
            "unsafe" => {
                span_ops.push(Operation::Push(Felt::new(32)));
                span_ops.push(Operation::Swap);
                span_ops.push(Operation::U32sub);
                span_ops.push(Operation::Drop);
                span_ops.push(Operation::Pow2);
            }
            _ => {
                // Assert the top of the stack is a u32 value.
                // NOTE: We cannot use U32Assert2 since we are potentially pushing a number larger
                // than u32 for b.
                assert_u32(span_ops);

                let x = parse_int_param(op, 1, 0, 31)?;
                span_ops.push(Operation::Push(Felt::new(2u64.pow(32 - x))));
            }
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    span_ops.push(Operation::U32mul);
    span_ops.push(Operation::Add);

    Ok(())
}

// COMPARISON OPERATIONS
// ================================================================================================

/// Translates u32eq assembly instruction to VM operations.
///
/// Specifically we test the first two numbers to be u32, then perform a EQ to check the equality.
///
/// VM cycles per mode:
/// u32eq: 2 cycles
/// u32eq.b: 3 cycles
pub fn parse_u32eq(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32assert2),
        2 => assert_and_push_u32_param(span_ops, op, 0)?,
        _ => return Err(AssemblyError::extra_param(op)),
    }

    span_ops.push(Operation::Eq);

    Ok(())
}

/// Translates u32neq assembly instruction to VM operations.
///
/// Specifically we test the first two numbers to be u32, then perform a `EQ NOT` to check the
/// equality.
///
/// u32neq: 3 cycles
/// u32neq.b: 4 cycles
pub fn parse_u32neq(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::U32assert2),
        2 => assert_and_push_u32_param(span_ops, op, 0)?,
        _ => return Err(AssemblyError::extra_param(op)),
    }

    span_ops.push(Operation::Eq);
    span_ops.push(Operation::Not);

    Ok(())
}

/// Translates u32lt assembly instruction to VM operations.
///
/// Specifically we test the first two numbers to be u32, then perform a `U32SUB EQZ NOT` to check
/// the underflow flag.
///
/// VM cycles per mode:
/// u32lt: 6 cycles
/// u32lt.unsafe: 5 cycles
pub fn parse_u32lt(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    handle_u32_and_unsafe_check(span_ops, op)?;
    compute_lt(span_ops);

    Ok(())
}

/// Translates u32lte assembly instruction to VM operations.
///
/// Specifically we test the first two numbers to be u32, then perform a gt check and flip the
/// results.
///
/// VM cycles per mode:
/// u32lte: 8 cycles
/// u32lte.unsafe: 7 cycles
pub fn parse_u32lte(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    handle_u32_and_unsafe_check(span_ops, op)?;

    // Compute the lt with reversed number to get a gt check
    span_ops.push(Operation::Swap);
    compute_lt(span_ops);

    // Flip the final results to get the lte results.
    span_ops.push(Operation::Not);

    Ok(())
}

/// Translates u32gt assembly instruction to VM operations.
///
/// Specifically we test the first two numbers to be u32, then perform a lt check with the
/// numbers swapped.
///
/// VM cycles per mode:
/// u32gt: 7 cycles
/// u32gt.unsafe: 6 cycles
pub fn parse_u32gt(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    handle_u32_and_unsafe_check(span_ops, op)?;

    // Reverse the numbers so we can get a gt check.
    span_ops.push(Operation::Swap);

    compute_lt(span_ops);

    Ok(())
}

/// Translates u32gte assembly instruction to VM operations.
///
/// Specifically we test the first two numbers to be u32, then compute a lt check and flip the
/// results.
///
/// VM cycles per mode:
/// u32gte: 7 cycles
/// u32gte.unsafe: 6 cycles
pub fn parse_u32gte(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    handle_u32_and_unsafe_check(span_ops, op)?;

    compute_lt(span_ops);

    // Flip the final results to get the gte results.
    span_ops.push(Operation::Not);

    Ok(())
}

/// Translates u32min assembly instruction to VM operations.
///
/// Specifically, we test the first two numbers to be u32 (U32SPLIT NOT ASSERT), subtract the top
/// value from the second to the top value (U32SUB), check the underflow flag (EQZ), and perform a
/// conditional swap (CSWAP) to have the max number in front. Then we finally drop the top element
/// to keep the min.
///
/// VM cycles per mode:
/// u32min: 9 cycles
/// u32min.unsafe: 8 cycles
pub fn parse_u32min(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    let unsafe_mode = match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => false,
        2 => match op.parts()[1] {
            "unsafe" => true,
            _ => return Err(AssemblyError::invalid_param(op, 1)),
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    compute_max_and_min(span_ops, unsafe_mode);

    // Drop the max and keep the min
    span_ops.push(Operation::Drop);

    Ok(())
}

/// Translates u32max assembly instruction to VM operations.
///
/// Specifically, we test the first two values to be u32 (U32SPLIT NOT ASSERT), subtract the top
/// value from the second to the top value (U32SUB), check the underflow flag (EQZ), and perform
/// a conditional swap (CSWAP) to have the max number in front. then we finally drop the 2nd
/// element to keep the max.
///
/// VM cycles per mode:
/// u32max: 10 cycles
/// u32max.unsafe: 9 cycles
pub fn parse_u32max(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    let unsafe_mode = match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => false,
        2 => match op.parts()[1] {
            "unsafe" => true,
            _ => return Err(AssemblyError::invalid_param(op, 1)),
        },
        _ => return Err(AssemblyError::extra_param(op)),
    };

    compute_max_and_min(span_ops, unsafe_mode);

    // Drop the min and keep the max
    span_ops.push(Operation::Swap);
    span_ops.push(Operation::Drop);

    Ok(())
}

// HELPER FUNCTIONS
// ================================================================================================
/// Asserts that the value on the top of the stack is a u32.
///
/// Implemented as: `PAD U32ASSERT2 DROP` (takes 3 VM cycles).
fn assert_u32(span_ops: &mut Vec<Operation>) {
    span_ops.push(Operation::Pad);
    span_ops.push(Operation::U32assert2);
    span_ops.push(Operation::Drop);
}

/// Asserts that the value on the top of the stack is a u32 and pushes the first param of the `op`
/// as a u32 value onto the stack.
///
/// # Errors
/// Returns an error if the first parameter of the `op` is not a u32 value or is greater than
/// `lower_bound`.
fn assert_and_push_u32_param(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    lower_bound: u32,
) -> Result<(), AssemblyError> {
    // TODO: We should investigate special case handling adding 0 or 1.
    let value = parse_int_param(op, 1, lower_bound, u32::MAX)?;
    push_value(span_ops, Felt::new(value as u64));

    // Assert both nmbers are u32.
    span_ops.push(Operation::U32assert2);

    Ok(())
}

/// Duplicate the top two elements in the stack and check both are u32, and determine the min
/// and max between them.
///
/// The maximum number will be at the top of the stack and minimum will be at the 2nd index.
fn compute_max_and_min(span_ops: &mut Vec<Operation>, unsafe_mode: bool) {
    // Copy top two elements of the stack.
    span_ops.push(Operation::Dup1);
    span_ops.push(Operation::Dup1);
    if !unsafe_mode {
        span_ops.push(Operation::U32assert2);
    }

    span_ops.push(Operation::U32sub);
    span_ops.push(Operation::Swap);
    span_ops.push(Operation::Drop);

    // Check the underflow flag, if it's zero
    // then the second number is equal or larger than the first.
    span_ops.push(Operation::Eqz);
    span_ops.push(Operation::CSwap);
}

/// Inserts the VM operations to check if the second element is less than
/// the top element. This takes 5 cycles.
fn compute_lt(span_ops: &mut Vec<Operation>) {
    span_ops.push(Operation::U32sub);
    span_ops.push(Operation::Swap);
    span_ops.push(Operation::Drop);

    // Check the underflow flag
    span_ops.push(Operation::Eqz);
    span_ops.push(Operation::Not);
}

/// Handles u32 assertion and unsafe mode for any u32 operation.
fn handle_u32_and_unsafe_check(
    span_ops: &mut Vec<Operation>,
    op: &Token,
) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            span_ops.push(Operation::U32assert2);
        }
        2 => {
            if op.parts()[1] != "unsafe" {
                return Err(AssemblyError::invalid_param(op, 1));
            }
        }
        _ => return Err(AssemblyError::extra_param(op)),
    };

    Ok(())
}

/// Handles arithmetic operation that needs support for unsafe, full, operation and operation.n
/// modes.
///
/// Specifically handles these specific inputs per the spec.
/// - Zero argument: assert the top two elements are u32 and push the result after  to the stack.
/// - Single argument:
///   - "unsafe" skips the assert check and directly performs the operation
///   - "full" checks both numbers are u32 and perform the same operations as "unsafe"
///   - Any number argument gets pushed to the stack, checked if both are u32 and performs the
///     operation.
///
/// According to the spec this is currently U32ADD, U32SUB, U32MUL.
fn handle_arithmetic_operation(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    arithmetic_op: Operation,
) -> Result<(), AssemblyError> {
    // prepare the stack for the operation and determine if we need to check for overflow
    let assert_u32_result = match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => {
            // for simple arithmetic operation we need to make sure operands are u32 values,
            // and we need to make sure that the result will be a u32 value as well
            span_ops.push(Operation::U32assert2);
            true
        }
        2 => match op.parts()[1] {
            "unsafe" => false,
            "full" => {
                // for the full  mode we need to make sure operands are u32 values, but we don't
                // need to check the result for overflow because we return both high and low bits
                // of the result
                span_ops.push(Operation::U32assert2);
                false
            }
            _ => {
                // for operation.n (where n is the immediate value), we need to push the immediate
                // value onto the stack, and make sure both operands are u32 values. we also want
                // to make sure the result is a u32 value.
                assert_and_push_u32_param(span_ops, op, 0)?;
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
