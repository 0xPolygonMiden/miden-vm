use super::{parse_element_param, AssemblyError, BaseElement, FieldElement, Operation, Token};

// CONSTANT INPUTS
// ================================================================================================

/// Appends a PUSH operation to the span block.
///
/// In cases when the immediate value is 0, PUSH operation is replaced with PAD. Also, in cases
/// when immediate value is 1, PUSH operation is replaced with PAD INCR because in most cases
/// this will be more efficient than doing a PUSH.
///
/// # Errors
///
/// This function expects an assembly op with exactly one immediate value that is a valid field
/// element in decimal or hexadecimal representation. It will return an error if the immediate
/// value is invalid or missing.
pub fn parse_push(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    validate_op_len(op, 2, 2)?;

    let value = parse_element_param(op, 1)?;
    push_value(span_ops, value);

    Ok(())
}

/// Appends PUSH operations to the span block until all elements of the provided word are pushed
/// onto the stack.
///
/// All immediate values are handled in the same way as for the single element "push" operation.
///
/// # Errors
///
/// This function expects an assembly op with 4 immediate values that are valid field elements
/// in decimal or hexadecimal representation. It will return an error if the assembly instruction's
/// immediate values are invalid.
pub fn parse_pushw(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    validate_op_len(op, 5, 5)?;

    for idx in 1..=4 {
        let value = parse_element_param(op, idx)?;
        push_value(span_ops, value);
    }

    Ok(())
}

/// This is a helper function that appends a PUSH operation to the span block which puts the
/// provided value parameter onto the stack.
///
/// When the value is 0, PUSH operation is replaced with PAD. When the value is 1, PUSH operation
/// is replaced with PAD INCR because in most cases this will be more efficient than doing a PUSH.
fn push_value(span_ops: &mut Vec<Operation>, value: BaseElement) {
    if value == BaseElement::ZERO {
        span_ops.push(Operation::Pad);
    } else if value == BaseElement::ONE {
        span_ops.push(Operation::Pad);
        span_ops.push(Operation::Incr);
    } else {
        span_ops.push(Operation::Push(value));
    }
}

// ENVIRONMENT INPUTS
// ================================================================================================

/// Appends machine operations to the current span block according to the requested environment
/// assembly instruction.
///
/// "env.sdepth" pushes the current depth of the stack onto the top of the stack, which is handled
/// directly by the SDEPTH operation.
///
/// # Errors
///
/// This function expects a valid assembly environment op that specifies the environment input to
/// be handled. It will return an error if the assembly instruction is malformed or the environment
/// input is unrecognized.
pub fn parse_env(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    validate_op_len(op, 2, 2)?;

    match op.parts()[1] {
        "sdepth" => {
            span_ops.push(Operation::SDepth);
        }
        _ => return Err(AssemblyError::invalid_op(op)),
    }

    Ok(())
}

// NON-DETERMINISTIC INPUTS
// ================================================================================================

/// TODO: implement
pub fn parse_read(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

/// TODO: implement
pub fn parse_readw(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

// RANDOM ACCESS MEMORY
// ================================================================================================

/// Appends STOREW or LOADW and required stack manipulations to the span block, as specified by the
/// memory operation.
///
/// If the op does not contain an address, the memory address is assumed to be
/// on top of the stack. Otherwise, the provided address will be pushed so it is on top of the
/// stack when STOREW or LOADW is executed.
///
/// "mem.push" reads a word (4 elements) from memory and pushes it onto the stack.
/// "mem.load" reads a word from memory and overwrites the top 4 elements of the stack.
/// "mem.pop" is a write operation that pops the top 4 elements off the stack and saves them to
/// memory.
/// "mem.store" is a write operation that saves the top 4 elements of the stack to memory and
/// leaves them on the stack.
pub fn parse_mem(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    validate_op_len(op, 2, 3)?;

    match op.parts()[1] {
        "push" | "load" => parse_mem_read(span_ops, op),
        "pop" | "store" => parse_mem_write(span_ops, op),
        _ => Err(AssemblyError::invalid_op(op)),
    }
}

/// Translates the mem.push and mem.load assembly ops to the system's LOADW memory read operation.
///
/// If the op provides an address (e.g. mem.push.a), it must be pushed to the stack directly
/// before the LOADW operation. For "mem.load", LOADW can be used directly. For "mem.push", space
/// for 4 new elements on the stack must be made first, using PAD. Then, if the memory address was
/// provided via the stack (not as part of the memory op) it must be moved to the top.
///
/// # Errors
///
/// This function expects a memory read assembly operation that has already been validated. If
/// called without validation, it could yield incorrect results or return an AssemblyError.
fn parse_mem_read(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.parts()[1] == "push" {
        // make space for the new elements
        for _ in 0..4 {
            span_ops.push(Operation::Pad);
        }

        // put the memory address on top of the stack
        if op.num_parts() == 2 {
            // move the memory address to the top of the stack
            span_ops.push(Operation::MovUp4);
        } else {
            // parse the provided memory address and push it onto the stack
            let address = parse_element_param(op, 2)?;
            span_ops.push(Operation::Push(address));
        }
    } else if op.num_parts() == 3 {
        push_mem_addr(span_ops, op)?;
    }

    // load from the memory address on top of the stack
    span_ops.push(Operation::LoadW);

    Ok(())
}

/// Translates the mem.pop and mem.store assembly ops to the system's STOREW memory write
/// operation.
///
/// If the op provides an address (e.g. mem.pop.a), it must be pushed to the stack directly before
/// the STOREW operation. For "mem.store", STOREW can be used directly. For "mem.pop", the stack
/// must DROP the top 4 elements after they are written to memory.
///
/// # Errors
///
/// This function expects a memory write assembly operation that has already been validated. If
/// called without validation, it could yield incorrect results or return an AssemblyError.
fn parse_mem_write(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    if op.num_parts() == 3 {
        push_mem_addr(span_ops, op)?;
    }

    span_ops.push(Operation::StoreW);

    if op.parts()[1] == "pop" {
        for _ in 0..4 {
            span_ops.push(Operation::Drop);
        }
    }

    Ok(())
}

/// Parses a provided memory address and pushes it onto the stack.
///
/// # Errors
///
/// This function will return an AssemblyError if the address parameter does not exist.
fn push_mem_addr(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    let address = parse_element_param(op, 2)?;
    span_ops.push(Operation::Push(address));

    Ok(())
}

// HELPERS
// ================================================================================================

/// This is a helper function that validates the length of the assembly io instruction and returns
/// an error if params are missing or there are extras.
///
/// The minimum and maximum number of valid instruction parts for the op are provided by the
/// caller.
///
/// # Errors
///
/// This function will return an AssemblyError if the assembly op has too many or too few parts.
fn validate_op_len(op: &Token, min_parts: usize, max_parts: usize) -> Result<(), AssemblyError> {
    match op.num_parts() {
        too_few if too_few < min_parts => Err(AssemblyError::missing_param(op)),
        too_many if too_many > max_parts => Err(AssemblyError::extra_param(op)),
        _ => Ok(()),
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push() {
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_0 = Token::new("push.0", 0);
        let op_1 = Token::new("push.1", 0);
        let op_dec = Token::new("push.135", 0);
        let op_hex = Token::new("push.0x7b", 0);
        let expected = vec![
            Operation::Pad,
            Operation::Pad,
            Operation::Incr,
            Operation::Push(BaseElement::new(135)),
            Operation::Push(BaseElement::new(123)),
        ];

        parse_push(&mut span_ops, &op_0).expect("Failed to parse push.0");
        parse_push(&mut span_ops, &op_1).expect("Failed to parse push.1");
        parse_push(&mut span_ops, &op_dec).expect("Failed to parse push of decimal element 123");
        parse_push(&mut span_ops, &op_hex).expect("Failed to parse push of hex element 0x7b");

        assert_eq!(span_ops, expected);
    }

    #[test]
    fn push_invalid() {
        // fails when immediate value is invalid or missing
        let mut span_ops: Vec<Operation> = Vec::new();
        let param_idx = 0;

        // value missing
        let op_no_val = Token::new("push", param_idx);
        let expected = AssemblyError::missing_param(&op_no_val);
        assert_eq!(parse_push(&mut span_ops, &op_no_val).unwrap_err(), expected);

        // invalid value
        let op_val_invalid = Token::new("push.abc", param_idx);
        let expected = AssemblyError::invalid_param(&op_val_invalid, 1);
        assert_eq!(
            parse_push(&mut span_ops, &op_val_invalid).unwrap_err(),
            expected
        );

        // extra value
        let op_extra_val = Token::new("push.0.1", param_idx);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(
            parse_push(&mut span_ops, &op_extra_val).unwrap_err(),
            expected
        );

        // wrong operation passed to parsing function
        let op_mismatch = Token::new("pushw.0", param_idx);
        let expected = AssemblyError::unexpected_token(&op_mismatch, "push.{param}");
        assert_eq!(
            parse_push(&mut span_ops, &op_mismatch).unwrap_err(),
            expected
        )
    }

    #[test]
    fn pushw() {
        // pushes a word of 4 immediate values in decimal or hexadecimal onto the stack
        let mut span_ops: Vec<Operation> = Vec::new();
        let op = Token::new("pushw.1.23.0x1C8.0", 0);
        let expected = vec![
            Operation::Pad,
            Operation::Incr,
            Operation::Push(BaseElement::new(23)),
            Operation::Push(BaseElement::new(456)),
            Operation::Pad,
        ];
        parse_pushw(&mut span_ops, &op).expect("Failed to parse pushw");

        assert_eq!(span_ops, expected);
    }

    #[test]
    fn pushw_invalid() {
        // fails when immediate values are invalid or missing
        let mut span_ops: Vec<Operation> = Vec::new();
        let param_idx = 0;

        // no values
        let op_no_vals = Token::new("pushw", param_idx);
        let expected = AssemblyError::missing_param(&op_no_vals);
        assert_eq!(
            parse_pushw(&mut span_ops, &op_no_vals).unwrap_err(),
            expected
        );

        // insufficient values provided
        let op_val_missing = Token::new("pushw.0.1.2", param_idx);
        let expected = AssemblyError::missing_param(&op_val_missing);
        assert_eq!(
            parse_pushw(&mut span_ops, &op_val_missing).unwrap_err(),
            expected
        );

        // invalid value
        let op_val_invalid = Token::new("pushw.0.1.2.abc", param_idx);
        let expected = AssemblyError::invalid_param(&op_val_invalid, 4);
        assert_eq!(
            parse_pushw(&mut span_ops, &op_val_invalid).unwrap_err(),
            expected
        );

        // extra value
        let op_extra_val = Token::new("pushw.0.1.2.3.4", param_idx);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(
            parse_pushw(&mut span_ops, &op_extra_val).unwrap_err(),
            expected
        );

        // wrong operation passed to parsing function
        let op_mismatch = Token::new("push.0.1.2.3", param_idx);
        let expected =
            AssemblyError::unexpected_token(&op_mismatch, "pushw.{param}.{param}.{param}.{param}");
        assert_eq!(
            parse_pushw(&mut span_ops, &op_mismatch).unwrap_err(),
            expected
        )
    }

    #[test]
    fn env_sdepth() {
        // pushes the current depth of the stack onto the top of the stack
        let mut span_ops = vec![Operation::Push(BaseElement::ONE); 8];
        let op = Token::new("env.sdepth", 0);
        let mut expected = span_ops.clone();
        expected.push(Operation::SDepth);

        parse_env(&mut span_ops, &op).expect("Failed to parse env.sdepth with empty stack");
        assert_eq!(span_ops, expected);
    }

    #[test]
    fn env_invalid() {
        // fails when env op variant is invalid or missing or has too many immediate values
        let mut span_ops: Vec<Operation> = Vec::new();
        let param_idx = 0;

        // missing env var
        let op_no_val = Token::new("env", param_idx);
        let expected = AssemblyError::invalid_op(&op_no_val);
        assert_eq!(parse_env(&mut span_ops, &op_no_val).unwrap_err(), expected);

        // invalid env var
        let op_val_invalid = Token::new("env.invalid", param_idx);
        let expected = AssemblyError::invalid_op(&op_val_invalid);
        assert_eq!(
            parse_env(&mut span_ops, &op_val_invalid).unwrap_err(),
            expected
        );

        // extra value
        let op_extra_val = Token::new("env.sdepth.0", param_idx);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(
            parse_env(&mut span_ops, &op_extra_val).unwrap_err(),
            expected
        );

        // wrong operation passed to parsing function
        let op_mismatch = Token::new("push.sdepth", param_idx);
        let expected = AssemblyError::unexpected_token(&op_mismatch, "env.{param}");
        assert_eq!(
            parse_env(&mut span_ops, &op_mismatch).unwrap_err(),
            expected
        )
    }

    #[test]
    fn mem_pop() {
        // stores the top 4 elements of the stack in memory
        // then removes those 4 elements from the top of the stack

        // test pop with memory address on top of the stack
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_mem_pop = Token::new("mem.pop", 0);
        let expected = vec![
            Operation::StoreW,
            Operation::Drop,
            Operation::Drop,
            Operation::Drop,
            Operation::Drop,
        ];
        parse_mem(&mut span_ops, &op_mem_pop).expect("Failed to parse mem.pop");
        assert_eq!(&span_ops, &expected);

        // test pop with memory address provided directly (address 0)
        let mut span_ops_addr: Vec<Operation> = Vec::new();
        let op_pop_addr = Token::new("mem.pop.0", 0);
        let expected_addr = vec![
            Operation::Push(BaseElement::ZERO),
            Operation::StoreW,
            Operation::Drop,
            Operation::Drop,
            Operation::Drop,
            Operation::Drop,
        ];

        parse_mem(&mut span_ops_addr, &op_pop_addr).expect("Failed to parse mem.pop.0");

        assert_eq!(&span_ops_addr, &expected_addr);
    }

    #[test]
    fn mem_store() {
        // stores the top 4 elements of the stack in memory

        // test store with memory address on top of the stack
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_store = Token::new("mem.store", 0);
        let expected = vec![Operation::StoreW];

        parse_mem(&mut span_ops, &op_store).expect("Failed to parse mem.store");

        assert_eq!(&span_ops, &expected);

        // test store with memory address provided directly (address 0)
        let mut span_ops_addr: Vec<Operation> = Vec::new();
        let op_store_addr = Token::new("mem.store.0", 0);
        let expected_addr = vec![Operation::Push(BaseElement::ZERO), Operation::StoreW];

        parse_mem(&mut span_ops_addr, &op_store_addr)
            .expect("Failed to parse mem.store.0 with adddress (address provided by op)");

        assert_eq!(&span_ops_addr, &expected_addr);
    }

    #[test]
    fn mem_push() {
        // reads a word from memory and pushes it onto the stack

        // test push with memory address on top of stack
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_push = Token::new("mem.push", 0);
        let expected = vec![
            Operation::Pad,
            Operation::Pad,
            Operation::Pad,
            Operation::Pad,
            Operation::MovUp4,
            Operation::LoadW,
        ];

        parse_mem(&mut span_ops, &op_push).expect("Failed to parse mem.push");

        assert_eq!(&span_ops, &expected);

        // test push with memory address provided directly (address 0)
        let mut span_ops_addr: Vec<Operation> = Vec::new();
        let op_push_addr = Token::new("mem.push.0", 0);
        let expected_addr = vec![
            Operation::Pad,
            Operation::Pad,
            Operation::Pad,
            Operation::Pad,
            Operation::Push(BaseElement::ZERO),
            Operation::LoadW,
        ];

        parse_mem(&mut span_ops_addr, &op_push_addr)
            .expect("Failed to parse mem.push.0 (address provided by op)");

        assert_eq!(&span_ops_addr, &expected_addr);
    }

    #[test]
    fn mem_load() {
        // reads a word from memory and overwrites the top 4 stack elements

        // test load with memory address on top of stack
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_push = Token::new("mem.load", 0);
        let expected = vec![Operation::LoadW];

        parse_mem(&mut span_ops, &op_push).expect("Failed to parse mem.load");

        assert_eq!(&span_ops, &expected);

        // test load with memory address provided directly (address 0)
        let mut span_ops_addr: Vec<Operation> = Vec::new();
        let op_load_addr = Token::new("mem.load.0", 0);
        let expected_addr = vec![Operation::Push(BaseElement::ZERO), Operation::LoadW];

        parse_mem(&mut span_ops_addr, &op_load_addr)
            .expect("Failed to parse mem.load.0 (address provided by op)");

        assert_eq!(&span_ops_addr, &expected_addr);
    }

    #[test]
    fn mem_invalid() {
        // fails when mem op variant is invalid or missing or has too many immediate values
        let mut span_ops: Vec<Operation> = Vec::new();
        let param_idx = 0;

        // missing variant
        let op_missing = Token::new("mem", param_idx);
        let expected = AssemblyError::invalid_op(&op_missing);
        assert_eq!(parse_mem(&mut span_ops, &op_missing).unwrap_err(), expected);

        // invalid variant
        let op_invalid = Token::new("mem.abc", param_idx);
        let expected = AssemblyError::invalid_op(&op_invalid);
        assert_eq!(parse_mem(&mut span_ops, &op_invalid).unwrap_err(), expected);

        // invalid param
        let op_val_invalid = Token::new("mem.push.a", param_idx);
        let expected = AssemblyError::invalid_param(&op_val_invalid, 2);
        assert_eq!(
            parse_mem(&mut span_ops, &op_val_invalid).unwrap_err(),
            expected
        );

        // extra value
        let op_extra_val = Token::new("mem.push.0.1", param_idx);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(
            parse_mem(&mut span_ops, &op_extra_val).unwrap_err(),
            expected
        );

        // wrong operation passed to parsing function
        let op_mismatch = Token::new("adv.push.0", param_idx);
        let expected = AssemblyError::unexpected_token(&op_mismatch, "mem.{push|load|pop|store}");
        assert_eq!(
            parse_mem(&mut span_ops, &op_mismatch).unwrap_err(),
            expected
        );
    }
}
