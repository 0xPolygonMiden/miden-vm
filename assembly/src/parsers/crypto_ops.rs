use super::{validate_op_len, AssemblyError, BaseElement, Operation, Token};

// HASHING
// ================================================================================================
// The number of elements to be hashed by the rphash operation
const RPHASH_NUM_ELEMENTS: u64 = 8;

/// Appends RPPERM and stack manipulation operations to the span block as required to compute a
/// 2-to-1 Rescue Prime hash. The top of the stack is expected to be arranged with 2 words
/// (8 elements) to be hashed: [B, A, ...].
///
/// This assembly operation uses the VM operation RPPERM at its core, which permutes the top 12
/// elements of the stack.
///
/// To perform the operation, we do the following:
/// 1. Prepare the stack with 12 elements for RPPERM by pushing 4 more elements, the last of which
///    must equal the number of elements to be hashed (8).
/// 2. Append the RPPERM operation, which performs a Rescue Prime permutation on the top 12
///    elements and leaves an output of [E, D, C, ...] on the stack. E is our 2-to-1 hash result.
/// 3. Prepare to drop D and C by moving E further down the stack. This can be achieved by
///    swapping E and C with the SWAPW2 operation.
/// 4. Drop the top 8 elements from the stack, leaving our hash result at the top: [E, ...].
///
/// # Errors
/// Returns an AssemblyError if:
/// - the operation is malformed.
/// - an unrecognized operation is received (anything other than rphash).
pub fn parse_rphash(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    // validate the operation
    validate_op_len(op, 1, 0, 0)?;
    if op.parts()[0] != "rphash" {
        return Err(AssemblyError::unexpected_token(op, "rphash"));
    }

    // Add 4 elements to the stack to prepare for the Rescue Prime permutation
    // The element on top of the stack should be the number of elements to be hashed
    for _ in 0..3 {
        span_ops.push(Operation::Pad);
    }
    span_ops.push(Operation::Push(BaseElement::new(RPHASH_NUM_ELEMENTS)));

    // Do the Rescue Prime permutation on the top 12 elements in the stack
    span_ops.push(Operation::RpPerm);

    // Swap the top word (our result) with the 3rd word so we can easily drop words 2 and 3
    span_ops.push(Operation::SwapW2);

    // Drop 8 elements
    for _ in 0..8 {
        span_ops.push(Operation::Drop);
    }

    Ok(())
}

/// Appends an RPPERM operation to the span block, which performs a Rescue Prime permutation on the
/// top 12 elements of the stack.
///
/// # Errors
/// Returns an AssemblyError if:
/// - the operation is malformed.
/// - an unrecognized operation is received (anything other than rpperm).
pub fn parse_rpperm(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    // validate the operation
    validate_op_len(op, 1, 0, 0)?;
    if op.parts()[0] != "rpperm" {
        return Err(AssemblyError::unexpected_token(op, "rpperm"));
    }

    // append the machine op to the span block
    span_ops.push(Operation::RpPerm);

    Ok(())
}

// MERKLE TREES
// ================================================================================================

/// TODO: implement
pub fn parse_mtree(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rpperm() {
        let mut span_ops: Vec<Operation> = Vec::new();
        let op = Token::new("rpperm", 0);
        let expected = vec![Operation::RpPerm];

        parse_rpperm(&mut span_ops, &op).expect("Failed to parse rpperm");

        assert_eq!(span_ops, expected);
    }

    #[test]
    fn rpperm_invalid() {
        // parse_rpperm should return an error if called with an invalid or incorrect operation
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_pos = 0;

        let op_too_long = Token::new("rpperm.12", op_pos);
        let expected = AssemblyError::extra_param(&op_too_long);
        assert_eq!(
            parse_rpperm(&mut span_ops, &op_too_long).unwrap_err(),
            expected
        );

        let op_mismatch = Token::new("rphash", op_pos);
        let expected = AssemblyError::unexpected_token(&op_mismatch, "rpperm");
        assert_eq!(
            parse_rpperm(&mut span_ops, &op_mismatch).unwrap_err(),
            expected
        );
    }

    #[test]
    fn rphash() {
        // adds a word to the stack specifying the number of elements to be hashed (8)
        // does a rescue prime permutation
        // keeps the top word as the result but drops the other 8 elements
        let mut span_ops: Vec<Operation> = Vec::new();
        let op = Token::new("rphash", 0);

        // state of stack before permutation
        let mut expected = vec![
            Operation::Pad,
            Operation::Pad,
            Operation::Pad,
            Operation::Push(BaseElement::new(8)),
        ];
        // rp permutation leaves stack as [A, B, C,...]
        expected.push(Operation::RpPerm);
        // swap A and C, since A is the result we want --> gives [C, B, A, ...]
        expected.push(Operation::SwapW2);
        // drop C, B
        let drop8 = vec![Operation::Drop; 8];
        expected.extend_from_slice(&drop8);

        parse_rphash(&mut span_ops, &op).expect("Failed to parse rphash");
        assert_eq!(span_ops, expected);
    }

    #[test]
    fn rphash_invalid() {
        // parse_rphash should return an error if called with an invalid or incorrect operation
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_pos = 0;

        let op_too_long = Token::new("rphash.12", op_pos);
        let expected = AssemblyError::extra_param(&op_too_long);
        assert_eq!(
            parse_rphash(&mut span_ops, &op_too_long).unwrap_err(),
            expected
        );

        let op_mismatch = Token::new("rpperm", op_pos);
        let expected = AssemblyError::unexpected_token(&op_mismatch, "rphash");
        assert_eq!(
            parse_rphash(&mut span_ops, &op_mismatch).unwrap_err(),
            expected
        );
    }
}
