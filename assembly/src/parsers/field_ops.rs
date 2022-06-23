use super::{
    super::validate_operation, parse_element_param, AssemblyError, Felt, FieldElement, Operation,
    Token, Vec,
};

// ASSERTIONS AND TESTS
// ================================================================================================

/// Appends ASSERT operation to the span block.
///
/// In cases when 'eq' parameter is specified, the sequence of appended operations is: EQ ASSERT
pub fn parse_assert(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        1 => span_ops.push(Operation::Assert),
        2 => {
            if op.parts()[1] == "eq" {
                span_ops.push(Operation::Eq);
                span_ops.push(Operation::Assert);
            } else {
                return Err(AssemblyError::invalid_param(op, 1));
            }
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }
    Ok(())
}

// ARITHMETIC OPERATIONS
// ================================================================================================

/// Appends ADD operation to the span block.
///
/// In cases when one of the parameters is provided via immediate value, the sequence of
/// operations is: PUSH(imm) ADD, unless the imm value is 1, then the operation is just: INCR
pub fn parse_add(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        1 => span_ops.push(Operation::Add),
        2 => {
            let imm = parse_element_param(op, 1)?;
            if imm == Felt::ONE {
                span_ops.push(Operation::Incr);
            } else {
                span_ops.push(Operation::Push(imm));
                span_ops.push(Operation::Add);
            }
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }
    Ok(())
}

/// Appends NEG ADD operations to the span block.
///
/// In cases when one of the parameters is provided via immediate value, the sequence of
/// operations is: PUSH(-imm) ADD
pub fn parse_sub(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        1 => {
            span_ops.push(Operation::Neg);
            span_ops.push(Operation::Add);
        }
        2 => {
            let imm = parse_element_param(op, 1)?;
            span_ops.push(Operation::Push(-imm));
            span_ops.push(Operation::Add);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }
    Ok(())
}

/// Appends MUL operation to the span block.
///
/// In cases when one of the parameters is provided via immediate value, the sequence of
/// operations is: PUSH(imm) MUL
pub fn parse_mul(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        1 => span_ops.push(Operation::Mul),
        2 => {
            let imm = parse_element_param(op, 1)?;
            span_ops.push(Operation::Push(imm));
            span_ops.push(Operation::Mul);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }
    Ok(())
}

/// Appends INV MUL operations to the span block.
///
/// In cases when one of the parameters is provided via immediate value, the sequence of
/// operations is: PUSH(imm) INV MUL
pub fn parse_div(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        1 => {
            span_ops.push(Operation::Inv);
            span_ops.push(Operation::Mul);
        }
        2 => {
            let imm = parse_element_param(op, 1)?;
            span_ops.push(Operation::Push(imm.inv()));
            span_ops.push(Operation::Mul);
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }
    Ok(())
}

/// Appends NEG operation to the span block.
pub fn parse_neg(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    span_ops.push(Operation::Neg);
    Ok(())
}

/// Appends INV operation to the span block.
pub fn parse_inv(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    span_ops.push(Operation::Inv);
    Ok(())
}

/// Translates pow2 assembly instruction to VM operations. Pow2 accepts an exponent value in the
/// range [0, 63]
///
/// This takes 1 cycle.
pub fn parse_pow2(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        0 => return Err(AssemblyError::missing_param(op)),
        1 => span_ops.push(Operation::Pow2),
        _ => return Err(AssemblyError::extra_param(op)),
    }

    Ok(())
}

// BOOLEAN OPERATIONS
// ================================================================================================

/// Appends NOT operation to the span block.
pub fn parse_not(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    span_ops.push(Operation::Not);
    Ok(())
}

/// Appends AND operation to the span block.
pub fn parse_and(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    span_ops.push(Operation::And);
    Ok(())
}

/// Appends OR operation to the span block.
pub fn parse_or(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    span_ops.push(Operation::Or);
    Ok(())
}

/// Appends a sequence of operations emulating an XOR operation to the span block.
///
/// The sequence is: DUP0 DUP2 OR MOVDN2 AND NOT AND
pub fn parse_xor(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    span_ops.extend_from_slice(&[
        Operation::Dup0,
        Operation::Dup2,
        Operation::Or,
        Operation::MovDn2,
        Operation::And,
        Operation::Not,
        Operation::And,
    ]);
    Ok(())
}

// COMPARISON OPERATIONS
// ================================================================================================

/// Appends EQ operation to the span block.
///
/// In cases when an immediate values is supplied:
/// - If the immediate value is zero, the appended operation is EQZ
/// - Otherwise, the appended operations are: PUSH(imm) EQ
pub fn parse_eq(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        1 => span_ops.push(Operation::Eq),
        2 => {
            let imm = parse_element_param(op, 1)?;
            if imm == Felt::ZERO {
                span_ops.push(Operation::Eqz);
            } else {
                span_ops.push(Operation::Push(imm));
                span_ops.push(Operation::Eq);
            }
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }
    Ok(())
}

/// Appends EQ NOT operation to the span block.
///
/// In cases when an immediate values is supplied:
/// - If the immediate value is zero, the appended operations are: EQZ NOT
/// - Otherwise, the appended operations are: PUSH(imm) EQ NOT
pub fn parse_neq(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    match op.num_parts() {
        1 => span_ops.push(Operation::Eq),
        2 => {
            let imm = parse_element_param(op, 1)?;
            if imm == Felt::ZERO {
                span_ops.push(Operation::Eqz);
            } else {
                span_ops.push(Operation::Push(imm));
                span_ops.push(Operation::Eq);
            }
        }
        _ => return Err(AssemblyError::extra_param(op)),
    }
    span_ops.push(Operation::Not);
    Ok(())
}

/// Appends the EQW operation to the span block to do an element-wise comparison of the top 2 words
/// on the stack and push a value of 1 if they're equal or 0 otherwise. The original words are left
/// on the stack.
///
/// # Errors
/// Returns an error if the assembly operation token is malformed or incorrect.
pub fn parse_eqw(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    validate_operation!(op, "eqw", 0);

    span_ops.push(Operation::Eqw);

    Ok(())
}

/// Appends operations to the span block to pop the top 2 elements off the stack and do a "less
/// than" comparison. The stack is expected to be arranged as [b, a, ...] (from the top). A value
/// of 1 is pushed onto the stack if a < b. Otherwise, 0 is pushed.
///
/// This operation takes 17 VM cycles.
///
/// # Errors
/// Returns an error if the assembly operation token is malformed or incorrect.
pub fn parse_lt(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    validate_operation!(op, "lt", 0);

    // Split both elements into high and low bits
    // 3 cycles
    split_elements(span_ops);

    // compare the high bit values and put comparison result flags on the stack for eq and lt
    // then reorder in preparation for the low-bit comparison (a_lo < b_lo)
    // 9 cycles
    check_lt_high_bits(span_ops);

    // check a_lo < b_lo, resulting in 1 if true and 0 otherwise
    // 3 cycles
    check_lt(span_ops);

    // combine low-bit and high-bit results
    // 2 cycles
    set_result(span_ops);

    Ok(())
}

/// Appends operations to the span block to pop the top 2 elements off the stack and do a "less
/// than or equal" comparison. The stack is expected to be arranged as [b, a, ...] (from the top).
/// A value of 1 is pushed onto the stack if a <= b. Otherwise, 0 is pushed.
///
/// This operation takes 18 VM cycles.
///
/// # Errors
/// Returns an error if the assembly operation token is malformed or incorrect.
pub fn parse_lte(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    validate_operation!(op, "lte", 0);

    // Split both elements into high and low bits
    // 3 cycles
    split_elements(span_ops);

    // compare the high bit values and put comparison result flags on the stack for eq and lt
    // then reorder in preparation for the low-bit comparison (a_lo <= b_lo)
    // 9 cycles
    check_lt_high_bits(span_ops);

    // check a_lo <= b_lo, resulting in 1 if true and 0 otherwise
    // 4 cycles
    check_lte(span_ops);

    // combine low-bit and high-bit results
    // 2 cycles
    set_result(span_ops);

    Ok(())
}

/// Appends operations to the span block to pop the top 2 elements off the stack and do a "greater
/// than" comparison. The stack is expected to be arranged as [b, a, ...] (from the top). A value
/// of 1 is pushed onto the stack if a > b. Otherwise, 0 is pushed.
///
/// This operation takes 18 VM cycles.
///
/// # Errors
/// Returns an error if the assembly operation token is malformed or incorrect.
pub fn parse_gt(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    validate_operation!(op, "gt", 0);

    // Split both elements into high and low bits
    // 3 cycles
    split_elements(span_ops);

    // compare the high bit values and put comparison result flags on the stack for eq and gt
    // then reorder in preparation for the low-bit comparison (b_lo < a_lo)
    // 10 cycles
    check_gt_high_bits(span_ops);

    // check b_lo < a_lo, resulting in 1 if true and 0 otherwise
    // 3 cycles
    check_lt(span_ops);

    // combine low-bit and high-bit results
    // 2 cycles
    set_result(span_ops);

    Ok(())
}

/// Appends operations to the span block to pop the top 2 elements off the stack and do a "greater
/// than or equal" comparison. The stack is expected to be arranged as [b, a, ...] (from the top).
/// A value of 1 is pushed onto the stack if a >= b. Otherwise, 0 is pushed.
///
/// This operation takes 19 VM cycles.
///
/// # Errors
/// Returns an error if the assembly operation token is malformed or incorrect.
pub fn parse_gte(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    validate_operation!(op, "gte", 0);

    // Split both elements into high and low bits
    // 3 cycles
    split_elements(span_ops);

    // compare the high bit values and put comparison result flags on the stack for eq and gt
    // then reorder in preparation for the low-bit comparison (b_lo <= a_lo)
    // 10 cycles
    check_gt_high_bits(span_ops);

    // check b_lo <= a_lo, resulting in 1 if true and 0 otherwise
    // 4 cycles
    check_lte(span_ops);

    // combine low-bit and high-bit results
    // 2 cycles
    set_result(span_ops);

    Ok(())
}

// COMPARISON OPERATION HELPER FUNCTIONS
// ================================================================================================

/// Splits the top 2 elements on the stack into low and high 32-bit values and swaps their order.
/// The expected starting state of the stack (from the top) is: [b, a, ...].
///
/// After these operations, the stack state will be: [a_hi, a_lo, b_hi, b_lo, ...].
///
/// This function takes 3 cycles.
fn split_elements(span_ops: &mut Vec<Operation>) {
    // stack: [b, a, ...] => [b_hi, b_lo, a, ...]
    span_ops.push(Operation::U32split);
    // => [a, b_hi, b_lo, ...]
    span_ops.push(Operation::MovUp2);
    // => [a_hi, a_lo, b_hi, b_lo, ...]
    span_ops.push(Operation::U32split);
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
fn check_lt_high_bits(span_ops: &mut Vec<Operation>) {
    // reorder the stack to check a_hi < b_hi
    span_ops.push(Operation::MovUp2);

    // simultaneously check a_hi < b_hi and a_hi = b_hi, resulting in:
    // - an equality flag of 1 if a_hi = b_hi and 0 otherwise (at stack[0])
    // - a less-than flag of 1 if a_hi > b_hi and 0 otherwise (at stack[1])
    check_lt_and_eq(span_ops);

    // reorder the stack to prepare for low-bit comparison (a_lo < b_lo)
    span_ops.push(Operation::MovUp2);
    span_ops.push(Operation::MovUp3);
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
fn check_gt_high_bits(span_ops: &mut Vec<Operation>) {
    // reorder the stack to check b_hi < a_hi
    span_ops.push(Operation::Swap);
    span_ops.push(Operation::MovDn2);

    // simultaneously check b_hi < a_hi and b_hi = a_hi, resulting in:
    // - an equality flag of 1 if a_hi = b_hi and 0 otherwise (at stack[0])
    // - a greater-than flag of 1 if a_hi > b_hi and 0 otherwise (at stack[1])
    check_lt_and_eq(span_ops);

    // reorder the stack to prepare for low-bit comparison (b_lo < a_lo)
    span_ops.push(Operation::MovUp3);
    span_ops.push(Operation::MovUp3);
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
fn check_lt(span_ops: &mut Vec<Operation>) {
    // calculate a - b
    // stack: [b, a, ...] => [underflow_flag, result, ...]
    span_ops.push(Operation::U32sub);

    // drop the result, since it's not needed
    span_ops.push(Operation::Swap);
    span_ops.push(Operation::Drop);
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
fn check_lte(span_ops: &mut Vec<Operation>) {
    // calculate a - b
    // stack: [b, a, ...] => [underflow_flag, result, ...]
    span_ops.push(Operation::U32sub);

    // check the result
    span_ops.push(Operation::Swap);
    span_ops.push(Operation::Eqz);

    // set the lte flag if the underflow flag was set or the result was 0
    span_ops.push(Operation::Or);
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
fn check_lt_and_eq(span_ops: &mut Vec<Operation>) {
    // calculate a - b
    // stack: [b, a, ...] => [underflow_flag, result, ...]
    span_ops.push(Operation::U32sub);

    // Put 1 on the stack if the underflow flag was not set (there was no underflow)
    span_ops.push(Operation::Dup0);
    span_ops.push(Operation::Not);

    // move the result to the top of the stack and check if it was zero
    span_ops.push(Operation::MovUp2);
    span_ops.push(Operation::Eqz);

    // set the equality flag to 1 if there was no underflow and the result was zero
    span_ops.push(Operation::And)
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
fn set_result(span_ops: &mut Vec<Operation>) {
    // check if high bits are equal AND low bit comparison condition was true
    span_ops.push(Operation::And);

    // Set the result flag if the above check passed OR the high-bit comparison was true
    span_ops.push(Operation::Or);
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eqw() {
        // parse_eqw should return an error if called with an invalid or incorrect operation
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_pos = 0;

        let op_too_long = Token::new("eqw.12", op_pos);
        let expected = AssemblyError::extra_param(&op_too_long);
        assert_eq!(
            parse_eqw(&mut span_ops, &op_too_long).unwrap_err(),
            expected
        );

        let op_mismatch = Token::new("eq", op_pos);
        let expected = AssemblyError::unexpected_token(&op_mismatch, "eqw");
        assert_eq!(
            parse_eqw(&mut span_ops, &op_mismatch).unwrap_err(),
            expected
        );
    }

    #[test]
    fn lt() {
        // parse_lt should return an error if called with an invalid or incorrect operation
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_pos = 0;

        let op_too_long = Token::new("lt.1", op_pos);
        let expected = AssemblyError::extra_param(&op_too_long);
        assert_eq!(parse_lt(&mut span_ops, &op_too_long).unwrap_err(), expected);

        let op_mismatch = Token::new("eq", op_pos);
        let expected = AssemblyError::unexpected_token(&op_mismatch, "lt");
        assert_eq!(parse_lt(&mut span_ops, &op_mismatch).unwrap_err(), expected);
    }

    #[test]
    fn lte() {
        // parse_lte should return an error if called with an invalid or incorrect operation
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_pos = 0;

        let op_too_long = Token::new("lte.5", op_pos);
        let expected = AssemblyError::extra_param(&op_too_long);
        assert_eq!(
            parse_lte(&mut span_ops, &op_too_long).unwrap_err(),
            expected
        );

        let op_mismatch = Token::new("lt", op_pos);
        let expected = AssemblyError::unexpected_token(&op_mismatch, "lte");
        assert_eq!(
            parse_lte(&mut span_ops, &op_mismatch).unwrap_err(),
            expected
        );
    }

    #[test]
    fn gt() {
        // parse_gt should return an error if called with an invalid or incorrect operation
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_pos = 0;

        let op_too_long = Token::new("gt.0x10", op_pos);
        let expected = AssemblyError::extra_param(&op_too_long);
        assert_eq!(parse_gt(&mut span_ops, &op_too_long).unwrap_err(), expected);

        let op_mismatch = Token::new("lt", op_pos);
        let expected = AssemblyError::unexpected_token(&op_mismatch, "gt");
        assert_eq!(parse_gt(&mut span_ops, &op_mismatch).unwrap_err(), expected);
    }

    #[test]
    fn gte() {
        // parse_gte should return an error if called with an invalid or incorrect operation
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_pos = 0;

        let op_too_long = Token::new("gte.25", op_pos);
        let expected = AssemblyError::extra_param(&op_too_long);
        assert_eq!(
            parse_gte(&mut span_ops, &op_too_long).unwrap_err(),
            expected
        );

        let op_mismatch = Token::new("lt", op_pos);
        let expected = AssemblyError::unexpected_token(&op_mismatch, "gte");
        assert_eq!(
            parse_gte(&mut span_ops, &op_mismatch).unwrap_err(),
            expected
        );
    }
}
