use super::{validate_op_len, AssemblyError, Operation, Token};

// HASHING
// ================================================================================================

/// TODO: implement
pub fn parse_rphash(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
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
}
