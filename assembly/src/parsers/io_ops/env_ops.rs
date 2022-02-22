use super::{validate_operation, AssemblyError, Operation, Token};

// ENVIRONMENT INPUTS
// ================================================================================================

/// Appends machine operations to the current span block according to the requested environment
/// assembly instruction.
///
/// `push.env.sdepth` pushes the current depth of the stack onto the top of the stack, which is
/// handled directly by the `SDEPTH` operation.
///
/// # Errors
///
/// This function expects a valid assembly environment op that specifies the environment input to
/// be handled. It will return an error if the assembly instruction is malformed or the environment
/// input is unrecognized.
pub fn parse_push_env(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    validate_operation!(op, "push.env.sdepth", 0);

    // update the span block
    match op.parts()[2] {
        "sdepth" => {
            span_ops.push(Operation::SDepth);
        }
        _ => return Err(AssemblyError::invalid_op(op)),
    }

    Ok(())
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        super::{super::FieldElement, parse_push, AssemblyError, Felt},
        Operation, Token,
    };

    // TESTS FOR PUSHING VALUES ONTO THE STACK (PUSH)
    // ============================================================================================

    #[test]
    fn push_env_sdepth() {
        let num_proc_locals = 0;

        // pushes the current depth of the stack onto the top of the stack
        let mut span_ops = vec![Operation::Push(Felt::ONE); 8];
        let op = Token::new("push.env.sdepth", 0);
        let mut expected = span_ops.clone();
        expected.push(Operation::SDepth);

        parse_push(&mut span_ops, &op, num_proc_locals)
            .expect("Failed to parse push.env.sdepth with empty stack");
        assert_eq!(span_ops, expected);
    }

    #[test]
    fn push_env_invalid() {
        let num_proc_locals = 0;

        // fails when env op variant is invalid or missing or has too many immediate values
        let mut span_ops: Vec<Operation> = Vec::new();
        let pos = 0;

        // missing env var
        let op_no_val = Token::new("push.env", pos);
        let expected = AssemblyError::invalid_op(&op_no_val);
        assert_eq!(
            parse_push(&mut span_ops, &op_no_val, num_proc_locals).unwrap_err(),
            expected
        );

        // invalid env var
        let op_val_invalid = Token::new("push.env.invalid", pos);
        let expected = AssemblyError::unexpected_token(&op_val_invalid, "push.env.sdepth");
        assert_eq!(
            parse_push(&mut span_ops, &op_val_invalid, num_proc_locals).unwrap_err(),
            expected
        );

        // extra value
        let op_extra_val = Token::new("push.env.sdepth.0", pos);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(
            parse_push(&mut span_ops, &op_extra_val, num_proc_locals).unwrap_err(),
            expected
        );
    }
}
