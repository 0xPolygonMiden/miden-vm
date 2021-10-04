use super::{AssemblyError, BaseElement, FieldElement, HintMap, OpCode, OpHint, StarkField};

// CONSTANTS
// ================================================================================================
const PUSH_OP_ALIGNMENT: usize = 8;
const HASH_OP_ALIGNMENT: usize = 16;

// CONTROL FLOW OPERATIONS
// ================================================================================================

/// Appends a NOOP operations to the program.
pub fn parse_noop(
    program: &mut Vec<OpCode>,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    program.push(OpCode::Noop);
    Ok(())
}

/// Appends either ASSERT or ASSERTEQ operations to the program.
pub fn parse_assert(
    program: &mut Vec<OpCode>,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    if op.len() > 2 {
        return Err(AssemblyError::extra_param(op, step));
    } else if op.len() == 1 {
        program.push(OpCode::Assert);
    } else if op[1] == "eq" {
        program.push(OpCode::AssertEq);
    } else {
        return Err(AssemblyError::invalid_param_reason(
            op,
            step,
            format!("parameter {} is invalid; allowed values are: [eq]", op[1]),
        ));
    }

    Ok(())
}

// INPUT OPERATIONS
// ================================================================================================

/// Appends a PUSH operation to the program.
pub fn parse_push(
    program: &mut Vec<OpCode>,
    hints: &mut HintMap,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    let value = read_value(op, step)?;
    append_push_op(program, hints, value);
    Ok(())
}

/// Makes sure PUSH operation alignment is correct and appends PUSH opcode to the program.
fn append_push_op(program: &mut Vec<OpCode>, hints: &mut HintMap, value: BaseElement) {
    // pad the program with NOOPs to make sure PUSH happens on steps which are multiples of 8
    let alignment = program.len() % PUSH_OP_ALIGNMENT;
    let pad_length = (PUSH_OP_ALIGNMENT - alignment) % PUSH_OP_ALIGNMENT;
    program.resize(program.len() + pad_length, OpCode::Noop);

    // read the value to be pushed onto the stack
    hints.insert(program.len(), OpHint::PushValue(value));

    // add PUSH opcode to the program
    program.push(OpCode::Push);
}

/// Appends either READ or READ2 operation to the program.
pub fn parse_read(
    program: &mut Vec<OpCode>,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    if op.len() > 2 {
        return Err(AssemblyError::extra_param(op, step));
    } else if op.len() == 1 || op[1] == "a" {
        program.push(OpCode::Read);
    } else if op[1] == "ab" {
        program.push(OpCode::Read2);
    } else {
        return Err(AssemblyError::invalid_param_reason(
            op,
            step,
            format!(
                "parameter {} is invalid; allowed values are: [a, ab]",
                op[1]
            ),
        ));
    }

    Ok(())
}

// STACK MANIPULATION OPERATIONS
// ================================================================================================

/// Appends a sequence of operations to the program to duplicate top n values of the stack.
pub fn parse_dup(program: &mut Vec<OpCode>, op: &[&str], step: usize) -> Result<(), AssemblyError> {
    let n = read_param(op, step)?;
    match n {
        1 => program.push(OpCode::Dup),
        2 => program.push(OpCode::Dup2),
        3 => program.extend_from_slice(&[OpCode::Dup4, OpCode::Roll4, OpCode::Drop]),
        4 => program.push(OpCode::Dup4),
        _ => {
            return Err(AssemblyError::invalid_param_reason(
                op,
                step,
                format!(
                    "parameter {} is invalid; allowed values are: [1, 2, 3, 4]",
                    n
                ),
            ))
        }
    };

    Ok(())
}

/// Appends a sequence of operations to the program to pad the stack with n zeros.
pub fn parse_pad(program: &mut Vec<OpCode>, op: &[&str], step: usize) -> Result<(), AssemblyError> {
    let n = read_param(op, step)?;
    match n {
        1 => program.extend_from_slice(&[OpCode::Pad2, OpCode::Drop]),
        2 => program.push(OpCode::Pad2),
        3 => program.extend_from_slice(&[OpCode::Pad2, OpCode::Pad2, OpCode::Drop]),
        4 => program.extend_from_slice(&[OpCode::Pad2, OpCode::Pad2]),
        5 => program.extend_from_slice(&[OpCode::Pad2, OpCode::Pad2, OpCode::Pad2, OpCode::Drop]),
        6 => program.extend_from_slice(&[OpCode::Pad2, OpCode::Pad2, OpCode::Pad2]),
        7 => program.extend_from_slice(&[OpCode::Pad2, OpCode::Pad2, OpCode::Dup4, OpCode::Drop]),
        8 => program.extend_from_slice(&[OpCode::Pad2, OpCode::Pad2, OpCode::Dup4]),
        _ => {
            return Err(AssemblyError::invalid_param_reason(
                op,
                step,
                format!(
                    "parameter {} is invalid; allowed values are: [1, 2, 3, 4, 5, 6, 7, 8]",
                    n
                ),
            ))
        }
    }

    Ok(())
}

/// Appends a sequence of operations to the program to copy n-th item to the top of the stack.
pub fn parse_pick(
    program: &mut Vec<OpCode>,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    let n = read_param(op, step)?;
    match n {
        1 => program.extend_from_slice(&[OpCode::Dup2, OpCode::Drop]),
        2 => program.extend_from_slice(&[
            OpCode::Dup4,
            OpCode::Roll4,
            OpCode::Drop,
            OpCode::Drop,
            OpCode::Drop,
        ]),
        3 => program.extend_from_slice(&[OpCode::Dup4, OpCode::Drop, OpCode::Drop, OpCode::Drop]),
        _ => {
            return Err(AssemblyError::invalid_param_reason(
                op,
                step,
                format!("parameter {} is invalid; allowed values are: [1, 2, 3]", n),
            ))
        }
    };

    Ok(())
}

/// Appends a sequence of operations to the program to remove top n values from the stack.
pub fn parse_drop(
    program: &mut Vec<OpCode>,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    let n = read_param(op, step)?;
    match n {
        1 => program.push(OpCode::Drop),
        2 => program.extend_from_slice(&[OpCode::Drop, OpCode::Drop]),
        3 => program.extend_from_slice(&[OpCode::Dup, OpCode::Drop4]),
        4 => program.push(OpCode::Drop4),
        5 => program.extend_from_slice(&[OpCode::Drop, OpCode::Drop4]),
        6 => program.extend_from_slice(&[OpCode::Drop, OpCode::Drop, OpCode::Drop4]),
        7 => program.extend_from_slice(&[OpCode::Dup, OpCode::Drop4, OpCode::Drop4]),
        8 => program.extend_from_slice(&[OpCode::Drop4, OpCode::Drop4]),
        _ => {
            return Err(AssemblyError::invalid_param_reason(
                op,
                step,
                format!(
                    "parameter {} is invalid; allowed values are: [1, 2, 3, 4, 5, 6, 7, 8]",
                    n
                ),
            ))
        }
    }

    Ok(())
}

/// Appends a sequence of operations to the program to swap n values at the top of the stack
/// with the following n values.
pub fn parse_swap(
    program: &mut Vec<OpCode>,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    let n = read_param(op, step)?;
    match n {
        1 => program.push(OpCode::Swap),
        2 => program.push(OpCode::Swap2),
        4 => program.push(OpCode::Swap4),
        _ => {
            return Err(AssemblyError::invalid_param_reason(
                op,
                step,
                format!("parameter {} is invalid; allowed values are: [1, 2, 4]", n),
            ))
        }
    }

    Ok(())
}

/// Appends either ROLL4 or ROLL8 operation to the program.
pub fn parse_roll(
    program: &mut Vec<OpCode>,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    let n = read_param(op, step)?;
    match n {
        4 => program.push(OpCode::Roll4),
        8 => program.push(OpCode::Roll8),
        _ => {
            return Err(AssemblyError::invalid_param_reason(
                op,
                step,
                format!("parameter {} is invalid; allowed values are: [4, 8]", n),
            ))
        }
    }

    Ok(())
}

// ARITHMETIC AND BOOLEAN OPERATIONS
// ================================================================================================

/// Appends ADD operation to the program.
pub fn parse_add(program: &mut Vec<OpCode>, op: &[&str], step: usize) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    program.push(OpCode::Add);
    Ok(())
}

/// Appends NEG ADD operations to the program.
pub fn parse_sub(program: &mut Vec<OpCode>, op: &[&str], step: usize) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    program.extend_from_slice(&[OpCode::Neg, OpCode::Add]);
    Ok(())
}

/// Appends MUL operation to the program.
pub fn parse_mul(program: &mut Vec<OpCode>, op: &[&str], step: usize) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    program.push(OpCode::Mul);
    Ok(())
}

/// Appends INV MUL operations to the program.
pub fn parse_div(program: &mut Vec<OpCode>, op: &[&str], step: usize) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    program.extend_from_slice(&[OpCode::Inv, OpCode::Mul]);
    Ok(())
}

/// Appends NEG operation to the program.
pub fn parse_neg(program: &mut Vec<OpCode>, op: &[&str], step: usize) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    program.push(OpCode::Neg);
    Ok(())
}

/// Appends INV operation to the program.
pub fn parse_inv(program: &mut Vec<OpCode>, op: &[&str], step: usize) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    program.push(OpCode::Inv);
    Ok(())
}

/// Appends NOT operation to the program.
pub fn parse_not(program: &mut Vec<OpCode>, op: &[&str], step: usize) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    program.push(OpCode::Not);
    Ok(())
}

/// Appends AND operation to the program.
pub fn parse_and(program: &mut Vec<OpCode>, op: &[&str], step: usize) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    program.push(OpCode::And);
    Ok(())
}

/// Appends OR operation to the program.
pub fn parse_or(program: &mut Vec<OpCode>, op: &[&str], step: usize) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    program.push(OpCode::Or);
    Ok(())
}

// COMPARISON OPERATIONS
// ================================================================================================

/// Appends a sequence of operations to the the program to determine whether the top value on the
/// stack is equal to the following value.
pub fn parse_eq(
    program: &mut Vec<OpCode>,
    hints: &mut HintMap,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    hints.insert(program.len(), OpHint::EqStart);
    program.extend_from_slice(&[OpCode::Read, OpCode::Eq]);
    Ok(())
}

/// Appends a sequence of operations to the the program to determine whether the top value on the
/// stack is not equal to the following value.
pub fn parse_ne(
    program: &mut Vec<OpCode>,
    hints: &mut HintMap,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    if op.len() > 1 {
        return Err(AssemblyError::extra_param(op, step));
    }
    hints.insert(program.len(), OpHint::EqStart);
    program.extend_from_slice(&[OpCode::Read, OpCode::Eq, OpCode::Not]);
    Ok(())
}

/// Appends a sequence of operations to the program to determine whether the top value on the
/// stack is greater than the following value.
pub fn parse_gt(
    program: &mut Vec<OpCode>,
    hints: &mut HintMap,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    // n is the number of bits sufficient to represent each value; if either of the
    // values does not fit into n bits, the operation fill fail.
    let n = read_param(op, step)?;
    if !(4..=128).contains(&n) {
        return Err(AssemblyError::invalid_param_reason(
            op,
            step,
            format!(
                "parameter {} is invalid; value must be between 4 and 128",
                n
            ),
        ));
    }

    // prepare the stack
    program.extend_from_slice(&[OpCode::Pad2, OpCode::Pad2, OpCode::Pad2, OpCode::Dup]);
    let power_of_two = BaseElement::new(2).exp(n as u128 - 1);
    append_push_op(program, hints, power_of_two);

    // add a hint indicating that value comparison is about to start
    hints.insert(program.len(), OpHint::CmpStart(n));

    // append CMP operations
    program.resize(program.len() + (n as usize), OpCode::Cmp);

    // compare binary aggregation values with the original values, and drop everything
    // but the GT value from the stack
    program.extend_from_slice(&[
        OpCode::Drop4,
        OpCode::Pad2,
        OpCode::Swap4,
        OpCode::Roll4,
        OpCode::AssertEq,
        OpCode::AssertEq,
        OpCode::Roll4,
        OpCode::Dup,
        OpCode::Drop4,
    ]);
    Ok(())
}

/// Appends a sequence of operations to the program to determine whether the top value on the
/// stack is less than the following value.
pub fn parse_lt(
    program: &mut Vec<OpCode>,
    hints: &mut HintMap,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    // n is the number of bits sufficient to represent each value; if either of the
    // values does not fit into n bits, the operation fill fail.
    let n = read_param(op, step)?;
    if !(4..=128).contains(&n) {
        return Err(AssemblyError::invalid_param_reason(
            op,
            step,
            format!(
                "parameter {} is invalid; value must be between 4 and 128",
                n
            ),
        ));
    }

    // prepare the stack
    program.extend_from_slice(&[OpCode::Pad2, OpCode::Pad2, OpCode::Pad2, OpCode::Dup]);
    let power_of_two = BaseElement::new(2).exp(n as u128 - 1);
    append_push_op(program, hints, power_of_two);

    // add a hint indicating that value comparison is about to start
    hints.insert(program.len(), OpHint::CmpStart(n));

    // append CMP operations
    program.resize(program.len() + (n as usize), OpCode::Cmp);

    // compare binary aggregation values with the original values, and drop everything
    // but the LT value from the stack
    program.extend_from_slice(&[
        OpCode::Drop4,
        OpCode::Pad2,
        OpCode::Swap4,
        OpCode::Roll4,
        OpCode::AssertEq,
        OpCode::AssertEq,
        OpCode::Dup,
        OpCode::Drop4,
    ]);
    Ok(())
}

/// Appends a sequence of operations to the program to determine whether the top value on the
/// stack can be represented with n bits.
pub fn parse_rc(
    program: &mut Vec<OpCode>,
    hints: &mut HintMap,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    // n is the number of bits against which to test the binary decomposition
    let n = read_param(op, step)?;
    if !(4..=128).contains(&n) {
        return Err(AssemblyError::invalid_param_reason(
            op,
            step,
            format!(
                "parameter {} is invalid; value must be between 4 and 128",
                n
            ),
        ));
    }

    // prepare the stack
    program.push(OpCode::Pad2);
    append_push_op(program, hints, BaseElement::ONE);
    program.extend_from_slice(&[OpCode::Swap, OpCode::Dup]);

    // add a hint indicating that range-checking is about to start
    hints.insert(program.len(), OpHint::RcStart(n));

    // append BINACC operations
    program.resize(program.len() + (n as usize), OpCode::BinAcc);

    // compare binary aggregation value with the original value
    program.extend_from_slice(&[OpCode::Dup, OpCode::Drop4]);
    hints.insert(program.len(), OpHint::EqStart);
    program.extend_from_slice(&[OpCode::Read, OpCode::Eq]);
    Ok(())
}

/// Appends a sequence of operations to the program to determine whether the top value on the
/// stack is odd.
pub fn parse_isodd(
    program: &mut Vec<OpCode>,
    hints: &mut HintMap,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    // n is the number of bits sufficient to represent top stack value;
    // if the values does not fit into n bits, the operation fill fail.
    let n = read_param(op, step)?;
    if !(4..=128).contains(&n) {
        return Err(AssemblyError::invalid_param_reason(
            op,
            step,
            format!(
                "parameter {} is invalid; value must be between 4 and 128",
                n
            ),
        ));
    }

    // prepare the stack
    program.push(OpCode::Pad2);
    append_push_op(program, hints, BaseElement::ONE);
    program.extend_from_slice(&[OpCode::Swap, OpCode::Dup]);

    // add a hint indicating that range-checking is about to start
    hints.insert(program.len(), OpHint::RcStart(n));

    // read the first bit and make sure it is saved at the end of the stack
    program.extend_from_slice(&[OpCode::BinAcc, OpCode::Swap2, OpCode::Roll4, OpCode::Dup]);

    // append remaining BINACC operations
    let n = n - 1;
    program.resize(program.len() + (n as usize), OpCode::BinAcc);

    // compare binary aggregation value with the original value and drop all values used in
    // computations except for the least significant bit of the value we saved previously
    program.extend_from_slice(&[
        OpCode::Drop,
        OpCode::Drop,
        OpCode::Swap,
        OpCode::Roll4,
        OpCode::AssertEq,
        OpCode::Drop,
    ]);
    Ok(())
}

// SELECTOR OPERATIONS
// ================================================================================================

/// Appends either CHOOSE or CHOOSE2 operation to the program.
pub fn parse_choose(
    program: &mut Vec<OpCode>,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    let n = read_param(op, step)?;
    match n {
        1 => program.push(OpCode::Choose),
        2 => program.push(OpCode::Choose2),
        _ => {
            return Err(AssemblyError::invalid_param_reason(
                op,
                step,
                format!("parameter {} is invalid; allowed values are: [1, 2]", n),
            ))
        }
    }
    Ok(())
}

// CRYPTO OPERATIONS
// ================================================================================================

/// Appends a sequence of operations to the program to hash top n values of the stack.
pub fn parse_hash(
    program: &mut Vec<OpCode>,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    let n = read_param(op, step)?;
    match n {
        1 => program.extend_from_slice(&[OpCode::Pad2, OpCode::Pad2, OpCode::Pad2, OpCode::Drop]),
        2 => program.extend_from_slice(&[OpCode::Pad2, OpCode::Pad2]),
        3 => program.extend_from_slice(&[OpCode::Pad2, OpCode::Pad2, OpCode::Drop]),
        4 => program.push(OpCode::Pad2),
        _ => {
            return Err(AssemblyError::invalid_param_reason(
                op,
                step,
                format!(
                    "parameter {} is invalid; allowed values are: [1, 2, 3, 4]",
                    n
                ),
            ))
        }
    }

    // pad with NOOPs to make sure hashing starts on a step which is a multiple of 16
    let alignment = program.len() % HASH_OP_ALIGNMENT;
    let pad_length = (HASH_OP_ALIGNMENT - alignment) % HASH_OP_ALIGNMENT;
    program.resize(program.len() + pad_length, OpCode::Noop);

    // append operations to execute 10 rounds of Rescue
    program.extend_from_slice(&[
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
    ]);

    // truncate the state
    program.push(OpCode::Drop4);

    Ok(())
}

/// Appends a sequence of operations to the program to compute the root of Merkle authentication
/// path for a tree of depth n. Leaf index is expected to be provided via input tapes A and B.
pub fn parse_smpath(
    program: &mut Vec<OpCode>,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    let n = read_param(op, step)?;
    if !(2..=256).contains(&n) {
        return Err(AssemblyError::invalid_param_reason(
            op,
            step,
            format!(
                "parameter {} is invalid; value must be between 2 and 256",
                n
            ),
        ));
    }

    // move the first bit of the leaf's index and the first node in the Merkle onto the stack,
    // position them correctly, and pad the stack to prepare it for hashing.
    program.extend_from_slice(&[
        OpCode::Read2,
        OpCode::Swap2,
        OpCode::Read2,
        OpCode::CSwap2,
        OpCode::Pad2,
    ]);

    // pad with NOOPs to make sure hashing starts on a step which is a multiple of 16
    let alignment = program.len() % HASH_OP_ALIGNMENT;
    let pad_length = (HASH_OP_ALIGNMENT - alignment) % HASH_OP_ALIGNMENT;
    program.resize(program.len() + pad_length, OpCode::Noop);

    // repeat the following cycle of operations once for each remaining node:
    // 1. compute hash of the 2 nodes on the stack
    // 2. read the index of the next node in the authentication path
    // 3. read the next node in the authentication path
    // 4. base on position index bit = 1, swaps the nodes on the stack (using cswap2 instruction)
    // 5. pad the stack to prepare it for the next round of hashing
    const SUB_CYCLE: [OpCode; 16] = [
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::Drop4,
        OpCode::Read2,
        OpCode::Swap2,
        OpCode::Read2,
        OpCode::CSwap2,
        OpCode::Pad2,
    ];

    for _ in 0..(n - 2) {
        program.extend_from_slice(&SUB_CYCLE);
    }

    // at the end, use the same cycle except for the last 5 operations
    // since there is no need to read in any additional nodes
    program.extend_from_slice(&SUB_CYCLE[..11]);

    Ok(())
}

/// Appends a sequence of operations to the program to compute the root of Merkle authentication
/// path for a tree of depth n. Leaf index is expected to be 3rd item from the top of the stack.
pub fn parse_pmpath(
    program: &mut Vec<OpCode>,
    hints: &mut HintMap,
    op: &[&str],
    step: usize,
) -> Result<(), AssemblyError> {
    let n = read_param(op, step)?;
    if !(2..=256).contains(&n) {
        return Err(AssemblyError::invalid_param_reason(
            op,
            step,
            format!(
                "parameter {} is invalid; value must be between 2 and 256",
                n
            ),
        ));
    }

    // add a hint indicating that pmpath macro is about to begin
    hints.insert(program.len(), OpHint::PmpathStart(n));

    // read the first node and its index onto the stack and make sure nodes are arranged
    // correctly. Also, set initial value of binary multiplier to 1.
    program.extend_from_slice(&[OpCode::Read2, OpCode::Pad2]);
    append_push_op(program, hints, BaseElement::ONE);
    program.extend_from_slice(&[
        OpCode::Swap,
        OpCode::Dup,
        OpCode::BinAcc,
        OpCode::Swap4,
        OpCode::CSwap2,
        OpCode::Pad2,
    ]);

    // pad with NOOPs to make sure hashing starts on a step which is a multiple of 16
    let alignment = program.len() % HASH_OP_ALIGNMENT;
    let pad_length = (HASH_OP_ALIGNMENT - alignment) % HASH_OP_ALIGNMENT;
    program.resize(program.len() + pad_length, OpCode::Noop);

    // repeat the following cycle of operations once for each remaining node:
    // 1. compute hash of the 2 nodes on the stack
    // 2. read the index of the next node in the authentication path (using binacc instruction)
    // 3. read the next node in the authentication path
    // 4. base on position index bit = 1, swap the nodes on the stack (using cswap2 instruction)
    // 5. pad the stack to prepare it for the next round of hashing
    const SUB_CYCLE: [OpCode; 32] = [
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::RescR,
        OpCode::Drop4,
        OpCode::Pad2,
        OpCode::Swap2,
        OpCode::Read2,
        OpCode::Swap4,
        OpCode::BinAcc,
        OpCode::Swap4,
        OpCode::CSwap2,
        OpCode::Pad2,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
    ];

    for _ in 0..(n - 2) {
        program.extend_from_slice(&SUB_CYCLE);
    }

    // at the end, use the first 11 operations from the cycle since there is nothing else to read;
    // then make sure the accumulated value of index is indeed equal to the leaf index
    program.extend_from_slice(&SUB_CYCLE[..11]);
    program.extend_from_slice(&[OpCode::Swap2, OpCode::Drop, OpCode::Roll4, OpCode::AssertEq]);

    Ok(())
}

// HELPER FUNCTIONS
// ================================================================================================

fn read_param(op: &[&str], step: usize) -> Result<u32, AssemblyError> {
    if op.len() == 1 {
        // if no parameters were provided, assume parameter value 1
        return Ok(1);
    } else if op.len() > 2 {
        return Err(AssemblyError::extra_param(op, step));
    }

    // try to parse the parameter value
    let result = match op[1].parse::<u32>() {
        Ok(i) => i,
        Err(_) => return Err(AssemblyError::invalid_param(op, step)),
    };

    // parameter value 0 is never valid
    if result == 0 {
        return Err(AssemblyError::invalid_param_reason(
            op,
            step,
            "parameter value must be greater than 0".to_string(),
        ));
    }

    Ok(result)
}

fn read_value(op: &[&str], step: usize) -> Result<BaseElement, AssemblyError> {
    // make sure exactly 1 parameter was supplied
    if op.len() == 1 {
        return Err(AssemblyError::missing_param(op, step));
    } else if op.len() > 2 {
        return Err(AssemblyError::extra_param(op, step));
    }

    let result = if op[1].starts_with("0x") {
        // parse hexadecimal number
        match u128::from_str_radix(&op[1][2..], 16) {
            Ok(i) => i,
            Err(_) => return Err(AssemblyError::invalid_param(op, step)),
        }
    } else {
        // parse decimal number
        match op[1].parse::<u128>() {
            Ok(i) => i,
            Err(_) => return Err(AssemblyError::invalid_param(op, step)),
        }
    };

    // make sure the value is a valid field element
    if result >= BaseElement::MODULUS {
        return Err(AssemblyError::invalid_param_reason(
            op,
            step,
            format!(
                "parameter value must be smaller than {}",
                BaseElement::MODULUS
            ),
        ));
    }

    Ok(BaseElement::new(result))
}
