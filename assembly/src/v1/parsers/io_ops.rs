use super::{parse_element_param, AssemblyError, BaseElement, FieldElement, Operation, Token};

// CONSTANT INPUTS
// ================================================================================================

/// Appends a PUSH operation to the span block.
///
/// In cases when the immediate value is 0, PUSH operation is replaced with PAD. Also, in cases
/// when immediate value is 1, PUSH operation is replaced with PAD INCR because in most cases
/// this will be more efficient than doing a PUSH.
pub fn parse_push(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    let value = parse_element_param(op, 1)?;
    if value == BaseElement::ZERO {
        span_ops.push(Operation::Pad);
    } else if value == BaseElement::ONE {
        span_ops.push(Operation::Pad);
        span_ops.push(Operation::Incr);
    } else {
        span_ops.push(Operation::Push(value));
    }
    Ok(())
}

/// TODO: implement
pub fn parse_pushw(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
}

// ENVIRONMENT INPUTS
// ================================================================================================

/// TODO: implement
pub fn parse_env(_span_ops: &mut Vec<Operation>, _op: &Token) -> Result<(), AssemblyError> {
    unimplemented!()
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
    match op.num_parts() {
        0 | 1 => Err(AssemblyError::missing_param(op)),
        2 | 3 => match op.parts()[1] {
            "push" | "load" => parse_mem_read(span_ops, op),
            "pop" | "store" => parse_mem_write(span_ops, op),
            _ => Err(AssemblyError::invalid_op(op)),
        },
        _ => Err(AssemblyError::extra_param(op)),
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
        let op_other = Token::new("push.2", 0);
        let expected = vec![
            Operation::Pad,
            Operation::Pad,
            Operation::Incr,
            Operation::Push(BaseElement::new(2)),
        ];

        parse_push(&mut span_ops, &op_0).expect("Failed to parse push.0");
        parse_push(&mut span_ops, &op_1).expect("Failed to parse push.1");
        parse_push(&mut span_ops, &op_other).expect("Failed to parse push.2");

        assert_eq!(span_ops, expected);
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
}
