use super::{parse_u32_param, AssemblyError, Operation, Token, Vec};
use vm_core::utils::PushMany;

// CONSTANTS
// ================================================================================================

/// The maximum number of elements that can be read from the advice tape in a single `push`
/// operation.
const ADVICE_READ_LIMIT: u32 = 16;

// NON-DETERMINISTIC (ADVICE) INPUTS
// ================================================================================================

/// Appends the number of `READ` operations specified by the operation's immediate value to the
/// span block. This removes the specified number of items from the advice tape and pushes them
/// onto the stack. The number of items that can be read from the advice tape is limited to 16.
///
/// # Errors
///
/// Returns an `AssemblyError` if the instruction is invalid, malformed, missing a required
/// parameter, or does not match the expected operation. Returns an `invalid_param` `AssemblyError`
/// if the parameter for `adv_push` is not a decimal value.
pub fn parse_adv_push(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    debug_assert_eq!(op.parts()[0], "adv_push");

    // parse and validate the parameter as the number of items to read from the advice tape
    // it must be between 1 and ADVICE_READ_LIMIT, inclusive, since adv.push.0 is a no-op
    let n = match op.num_parts() {
        0 | 1 => return Err(AssemblyError::missing_param(op)),
        2 => parse_u32_param(op, 1, 1, ADVICE_READ_LIMIT)?,
        _ => return Err(AssemblyError::extra_param(op)),
    };

    // read n items from the advice tape and push then onto the stack
    span_ops.push_many(Operation::Read, n as usize);

    Ok(())
}

/// Removes the next word (4 elements) from the advice tape and overwrites the top 4 elements of
/// the stack with it. Fails if the advice tape has fewer than 4 elements. After validation, this
/// operation uses the `READW` machine operation directly.
pub fn parse_adv_loadw(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    debug_assert_eq!(op.parts()[0], "adv_loadw");

    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }

    // load a word from the advice tape
    span_ops.push(Operation::ReadW);

    Ok(())
}

/// Appends operations to the span block to execute `adv_pipe` instruction. The sequence of
/// operations is: PIPE RPPERM.
///
/// This instruction requires 2 VM cycles to execute.
///
/// # Errors
/// This function will return an `AssemblyError` if the `adv_pipe` instruction is malformed.
pub fn parse_adv_pipe(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    debug_assert_eq!(op.parts()[0], "adv_pipe");
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }

    span_ops.push(Operation::Pipe);
    span_ops.push(Operation::RpPerm);

    Ok(())
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        super::{parse_adv_loadw, parse_adv_push},
        AssemblyError, Operation, Token, ADVICE_READ_LIMIT,
    };

    // TESTS FOR PUSHING VALUES ONTO THE STACK (PUSH)
    // ============================================================================================

    #[test]
    fn adv_push() {
        // remove n items from the advice tape and push them onto the stack
        let mut span_ops: Vec<Operation> = Vec::new();
        let op = Token::new("adv_push.4", 0);
        let expected = vec![Operation::Read; 4];

        parse_adv_push(&mut span_ops, &op).expect("Failed to parse adv_push.4");
        assert_eq!(span_ops, expected);
    }

    #[test]
    fn adv_push_invalid() {
        // fails when the instruction is malformed or unrecognized
        let mut span_ops: Vec<Operation> = Vec::new();
        let pos = 0;

        // missing value
        let op_no_val = Token::new("adv_push", pos);
        let expected = AssemblyError::missing_param(&op_no_val);
        assert_eq!(
            parse_adv_push(&mut span_ops, &op_no_val).unwrap_err(),
            expected
        );

        // extra value to push
        let op_extra_val = Token::new("adv_push.2.2", pos);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(
            parse_adv_push(&mut span_ops, &op_extra_val).unwrap_err(),
            expected
        );

        // invalid value - char
        let op_invalid_char = Token::new("adv_push.a", pos);
        let expected = AssemblyError::invalid_param(&op_invalid_char, 1);
        assert_eq!(
            parse_adv_push(&mut span_ops, &op_invalid_char).unwrap_err(),
            expected
        );

        // invalid value - hexadecimal
        let op_invalid_hex = Token::new("adv_push.0x10", pos);
        let expected = AssemblyError::invalid_param(&op_invalid_hex, 1);
        assert_eq!(
            parse_adv_push(&mut span_ops, &op_invalid_hex).unwrap_err(),
            expected
        );

        // parameter out of bounds
        let reason = format!(
            "parameter value must be greater than or equal to {} and less than or equal to {}",
            1, ADVICE_READ_LIMIT
        );
        // less than lower bound
        let op_lower_bound = Token::new("adv_push.0", pos);
        let expected = AssemblyError::invalid_param_with_reason(&op_lower_bound, 1, &reason);
        assert_eq!(
            parse_adv_push(&mut span_ops, &op_lower_bound).unwrap_err(),
            expected
        );

        // greater than upper bound
        let inst_str = format!("adv_push.{}", ADVICE_READ_LIMIT + 1);
        let op_upper_bound = Token::new(&inst_str, pos);
        let expected = AssemblyError::invalid_param_with_reason(&op_upper_bound, 1, &reason);
        assert_eq!(
            parse_adv_push(&mut span_ops, &op_upper_bound).unwrap_err(),
            expected
        );
    }

    // TESTS FOR OVERWRITING VALUES ON THE STACK (LOAD)
    // ============================================================================================
    #[test]
    fn loadw_adv() {
        // replace the top 4 elements of the stack with 4 elements from the advice tape
        let mut span_ops: Vec<Operation> = Vec::new();
        let op = Token::new("adv_loadw", 0);
        let expected = vec![Operation::ReadW];
        parse_adv_loadw(&mut span_ops, &op).expect("Failed to parse adv_loadw");
        assert_eq!(span_ops, expected);
    }

    #[test]
    fn loadw_adv_invalid() {
        // fails when the instruction is malformed or unrecognized
        let mut span_ops: Vec<Operation> = Vec::new();
        let pos = 0;

        // extra value to loadw
        let op_extra_val = Token::new("adv_loadw.0", pos);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(
            parse_adv_loadw(&mut span_ops, &op_extra_val).unwrap_err(),
            expected
        );
    }
}
