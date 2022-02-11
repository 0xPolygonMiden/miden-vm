use super::{parse_element_param, validate_op_len, AssemblyError, Operation, Token};

// RANDOM ACCESS MEMORY
// ================================================================================================

/// Pushes the first element of the word at the specified memory address onto the stack. The
/// memory address may be provided directly as an immediate value or via the stack.
pub fn parse_push_mem(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    validate_op_len(op, 2, 0, 1)?;

    parse_read_mem(span_ops, op, false)?;

    for _ in 0..3 {
        span_ops.push(Operation::Drop);
    }

    Ok(())
}

/// Pops the top element off the stack and saves it at the specified memory address. The memory
/// address may be provided directly as an immediate value or via the stack.
pub fn parse_pop_mem(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    validate_op_len(op, 2, 0, 1)?;

    // pad to word length before calling STOREW
    for _ in 0..3 {
        span_ops.push(Operation::Pad);
    }

    // if the destination memory address was on top of the stack, restore it to the top
    if op.num_parts() == 2 {
        span_ops.push(Operation::MovUp3);
    }

    parse_write_mem(span_ops, op, false)
}

/// Translates the `pushw.mem` and `loadw.mem` assembly ops to the system's `LOADW` memory read
/// operation.
///
/// If the op provides an address (e.g. `pushw.mem.a`), it must be pushed to the stack directly
/// before the `LOADW` operation. For `loadw.mem`, `LOADW` can be used directly. For `pushw.mem`,
/// space for 4 new elements on the stack must be made first, using `PAD`. Then, if the memory
/// address was provided via the stack (not as part of the memory op) it must be moved to the top.
///
/// # Errors
///
/// This function expects a memory read assembly operation that has already been validated. If
/// called without validation, it could yield incorrect results or return an `AssemblyError`.
pub fn parse_read_mem(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    overwrite_stack_top: bool,
) -> Result<(), AssemblyError> {
    if !overwrite_stack_top {
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

/// Translates the `popw.mem` and `storew.mem` assembly ops to the system's `STOREW` memory write
/// operation.
///
/// If the op provides an address (e.g. `popw.mem.a`), it must be pushed to the stack directly
/// before the `STOREW` operation. For `storew.mem`, `STOREW` can be used directly. For `popw.mem`,
/// the stack must `DROP` the top 4 elements after they are written to memory.
///
/// # Errors
///
/// This function expects a memory write assembly operation that has already been validated. If
/// called without validation, it could yield incorrect results or return an `AssemblyError`.
pub fn parse_write_mem(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    retain_stack_top: bool,
) -> Result<(), AssemblyError> {
    if op.num_parts() == 3 {
        push_mem_addr(span_ops, op)?;
    }

    span_ops.push(Operation::StoreW);

    if !retain_stack_top {
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
/// This function will return an `AssemblyError` if the address parameter does not exist.
fn push_mem_addr(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    let address = parse_element_param(op, 2)?;
    span_ops.push(Operation::Push(address));

    Ok(())
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        super::{
            parse_loadw, parse_popw, parse_pushw, parse_storew, tests::get_parsing_error,
            BaseElement,
        },
        AssemblyError, Operation, Token,
    };
    use crate::parsers::FieldElement;

    // TESTS FOR PUSHING VALUES ONTO THE STACK (PUSH)
    // ============================================================================================

    #[test]
    fn push_mem_invalid() {
        test_parse_mem("push");
    }

    #[test]
    fn pushw_mem() {
        let num_proc_locals = 0;
        // reads a word from memory and pushes it onto the stack

        // test push with memory address on top of stack
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_push = Token::new("pushw.mem", 0);
        let expected = vec![
            Operation::Pad,
            Operation::Pad,
            Operation::Pad,
            Operation::Pad,
            Operation::MovUp4,
            Operation::LoadW,
        ];

        parse_pushw(&mut span_ops, &op_push, num_proc_locals).expect("Failed to parse pushw.mem");

        assert_eq!(&span_ops, &expected);

        // test push with memory address provided directly (address 0)
        let mut span_ops_addr: Vec<Operation> = Vec::new();
        let op_push_addr = Token::new("pushw.mem.0", 0);
        let expected_addr = vec![
            Operation::Pad,
            Operation::Pad,
            Operation::Pad,
            Operation::Pad,
            Operation::Push(BaseElement::ZERO),
            Operation::LoadW,
        ];

        parse_pushw(&mut span_ops_addr, &op_push_addr, num_proc_locals)
            .expect("Failed to parse pushw.mem.0 (address provided by op)");

        assert_eq!(&span_ops_addr, &expected_addr);
    }

    #[test]
    fn pushw_mem_invalid() {
        test_parse_mem("pushw");
    }

    // TESTS FOR REMOVING VALUES FROM THE STACK (POP)
    // ============================================================================================

    #[test]
    fn pop_mem_invalid() {
        test_parse_mem("pop");
    }

    #[test]
    fn popw_mem() {
        let num_proc_locals = 0;

        // stores the top 4 elements of the stack in memory
        // then removes those 4 elements from the top of the stack

        // test pop with memory address on top of the stack
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_mem_pop = Token::new("popw.mem", 0);
        let expected = vec![
            Operation::StoreW,
            Operation::Drop,
            Operation::Drop,
            Operation::Drop,
            Operation::Drop,
        ];
        parse_popw(&mut span_ops, &op_mem_pop, num_proc_locals).expect("Failed to parse popw.mem");
        assert_eq!(&span_ops, &expected);

        // test pop with memory address provided directly (address 0)
        let mut span_ops_addr: Vec<Operation> = Vec::new();
        let op_pop_addr = Token::new("popw.mem.0", 0);
        let expected_addr = vec![
            Operation::Push(BaseElement::ZERO),
            Operation::StoreW,
            Operation::Drop,
            Operation::Drop,
            Operation::Drop,
            Operation::Drop,
        ];

        parse_popw(&mut span_ops_addr, &op_pop_addr, num_proc_locals)
            .expect("Failed to parse popw.mem.0");

        assert_eq!(&span_ops_addr, &expected_addr);
    }

    #[test]
    fn popw_mem_invalid() {
        test_parse_mem("popw");
    }

    // TESTS FOR OVERWRITING VALUES ON THE STACK (LOAD)
    // ============================================================================================

    #[test]
    fn loadw_mem() {
        let num_proc_locals = 0;

        // reads a word from memory and overwrites the top 4 stack elements

        // test load with memory address on top of stack
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_push = Token::new("loadw.mem", 0);
        let expected = vec![Operation::LoadW];

        parse_loadw(&mut span_ops, &op_push, num_proc_locals).expect("Failed to parse loadw.mem");

        assert_eq!(&span_ops, &expected);

        // test load with memory address provided directly (address 0)
        let mut span_ops_addr: Vec<Operation> = Vec::new();
        let op_load_addr = Token::new("loadw.mem.0", 0);
        let expected_addr = vec![Operation::Push(BaseElement::ZERO), Operation::LoadW];

        parse_loadw(&mut span_ops_addr, &op_load_addr, num_proc_locals)
            .expect("Failed to parse loadw.mem.0 (address provided by op)");

        assert_eq!(&span_ops_addr, &expected_addr);
    }

    #[test]
    fn loadw_mem_invalid() {
        test_parse_mem("loadw");
    }

    // TESTS FOR SAVING STACK VALUES WITHOUT REMOVING THEM (STORE)
    // ============================================================================================

    #[test]
    fn storew_mem() {
        let num_proc_locals = 0;
        // stores the top 4 elements of the stack in memory

        // test store with memory address on top of the stack
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_store = Token::new("storew.mem", 0);
        let expected = vec![Operation::StoreW];

        parse_storew(&mut span_ops, &op_store, num_proc_locals)
            .expect("Failed to parse storew.mem");

        assert_eq!(&span_ops, &expected);

        // test store with memory address provided directly (address 0)
        let mut span_ops_addr: Vec<Operation> = Vec::new();
        let op_store_addr = Token::new("storew.mem.0", 0);
        let expected_addr = vec![Operation::Push(BaseElement::ZERO), Operation::StoreW];

        parse_storew(&mut span_ops_addr, &op_store_addr, num_proc_locals)
            .expect("Failed to parse storew.mem.0 with adddress (address provided by op)");

        assert_eq!(&span_ops_addr, &expected_addr);
    }

    #[test]
    fn storew_mem_invalid() {
        test_parse_mem("storew");
    }

    // TEST HELPERS
    // ============================================================================================

    /// Test that an instruction for an absolute memory operation is properly formed. It can be used
    /// to test parameter inputs for `push.mem`, `pushw.mem`, `pop.mem`, `popw.mem`, `loadw.mem`,
    /// and `storew.mem`.
    fn test_parse_mem(base_op: &str) {
        let num_proc_locals = 0;

        // fails when immediate values to a {push|pushw|pop|popw|loadw|storew}.mem.{a|} operation
        // are invalid or missing
        let pos = 0;

        // invalid value provided to mem variant
        let op_str = format!("{}.mem.abc", base_op);
        let op_val_invalid = Token::new(&op_str, pos);
        let expected = AssemblyError::invalid_param(&op_val_invalid, 2);
        assert_eq!(
            get_parsing_error(base_op, &op_val_invalid, num_proc_locals),
            expected
        );

        // extra value provided to mem variant
        let op_str = format!("{}.mem.0.1", base_op);
        let op_extra_val = Token::new(&op_str, pos);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(
            get_parsing_error(base_op, &op_extra_val, num_proc_locals),
            expected
        );
    }
}
