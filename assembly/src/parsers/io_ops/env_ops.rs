use super::{parse_u32_param, push_value, AssemblyError, Felt, Operation, Token, Vec};

// ENVIRONMENT INPUTS
// ================================================================================================

/// Appends `locaddr.i` operation to the span block to push the absolute address of the local
/// variable at index `i` onto the stack.
///
/// # Errors
///
/// It will return an error if the assembly instruction is malformed or it has inappropriate
/// parameter value according to the number of local variables of the procedurethe
pub fn parse_locaddr(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    num_proc_locals: u32,
) -> Result<(), AssemblyError> {
    debug_assert_eq!(op.parts()[0], "locaddr");

    if num_proc_locals == 0 {
        return Err(AssemblyError::invalid_op_with_reason(
            op,
            "no procedure locals available in current context",
        ));
    }

    let index = match op.num_parts() {
        0 | 1 => return Err(AssemblyError::missing_param(op)),
        2 => parse_u32_param(op, 1, 0, num_proc_locals - 1)?,
        _ => return Err(AssemblyError::extra_param(op)),
    };

    push_value(span_ops, -Felt::from(index));
    span_ops.push(Operation::FmpAdd);

    Ok(())
}

/// Appends `sdepth` operation to the current span block to push the current depth of the stack
/// onto the top of the stack. `sdepth` is handled directly by the `SDEPTH` operation.
///
/// # Errors
///
/// It will return an error if the assembly instruction is malformed.
pub fn parse_sdepth(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    debug_assert_eq!(op.parts()[0], "sdepth");

    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }

    span_ops.push(Operation::SDepth);

    Ok(())
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        super::tests::get_parsing_error,
        super::{super::FieldElement, parse_locaddr, parse_sdepth, AssemblyError, Felt},
        Operation, Token,
    };

    // TESTS FOR PUSHING VALUES ONTO THE STACK (PUSH)
    // ============================================================================================

    #[test]
    fn sdepth() {
        // --- pushes the current depth of the stack onto the top of the stack --------------------
        let mut span_ops = vec![Operation::Push(Felt::ONE); 8];
        let op = Token::new("sdepth", 0);
        let mut expected = span_ops.clone();
        expected.push(Operation::SDepth);

        parse_sdepth(&mut span_ops, &op).expect("Failed to parse sdepth with empty stack");
        assert_eq!(span_ops, expected);
    }

    #[test]
    fn locaddr() {
        let asm_op = "locaddr";
        let num_proc_locals = 2;
        let mut span_ops: Vec<Operation> = Vec::new();

        let op_str = format!("{}.{}", asm_op, 1);
        let op = Token::new(&op_str, 0);
        assert!(parse_locaddr(&mut span_ops, &op, num_proc_locals).is_ok());
    }

    #[test]
    #[should_panic]
    fn sdepth_invalid_panic() {
        let mut span_ops: Vec<Operation> = Vec::new();
        let pos = 0;

        // --- invalid env var --------------------------------------------------------------------
        let op_val_invalid = Token::new("invalid", pos);
        let expected = AssemblyError::unexpected_token(&op_val_invalid, "sdepth");
        assert_eq!(
            parse_sdepth(&mut span_ops, &op_val_invalid).unwrap_err(),
            expected
        );
    }

    #[test]
    fn sdepth_invalid() {
        // fails when env op variant has too many immediate values
        let mut span_ops: Vec<Operation> = Vec::new();
        let pos = 0;

        // --- extra param ------------------------------------------------------------------------
        let op_extra_val = Token::new("sdepth.0", pos);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(
            parse_sdepth(&mut span_ops, &op_extra_val).unwrap_err(),
            expected
        );
    }

    #[test]
    #[should_panic]
    fn locaddr_invalid_panic() {
        let num_proc_locals = 2;
        let mut span_ops: Vec<Operation> = Vec::new();
        let pos = 0;

        // --- invalid env var --------------------------------------------------------------------
        let op_val_invalid = Token::new("invalid", pos);
        parse_locaddr(&mut span_ops, &op_val_invalid, num_proc_locals).unwrap_err();
    }

    #[test]
    fn locaddr_invalid() {
        let asm_op = "locaddr";
        let num_proc_locals = 2;
        let mut span_ops: Vec<Operation> = Vec::new();
        let pos = 0;

        // --- missing param ----------------------------------------------------------------------
        let op_missing_param = Token::new(asm_op, pos);
        let expected = AssemblyError::missing_param(&op_missing_param);
        assert_eq!(
            parse_locaddr(&mut span_ops, &op_missing_param, num_proc_locals).unwrap_err(),
            expected
        );

        // --- provided local index is outside of the declared bounds of the procedure locals -----
        let op_str = format!("{}.{}", asm_op, 2);
        let op_val_invalid = Token::new(&op_str, pos);
        let expected = AssemblyError::invalid_param_with_reason(
            &op_val_invalid,
            1,
            format!(
                "parameter value must be greater than or equal to 0 and less than or equal to {}",
                num_proc_locals - 1
            )
            .as_str(),
        );
        assert_eq!(
            get_parsing_error("locaddr", &op_val_invalid, num_proc_locals),
            expected
        );

        // --- no procedure locals in context -----------------------------------------------------
        let op_str = format!("{}.{}", asm_op, 1);
        let op_context_invalid = Token::new(&op_str, pos);
        let expected = AssemblyError::invalid_op_with_reason(
            &op_context_invalid,
            "no procedure locals available in current context",
        );
        assert_eq!(
            get_parsing_error("locaddr", &op_context_invalid, 0),
            expected
        );
    }
}
