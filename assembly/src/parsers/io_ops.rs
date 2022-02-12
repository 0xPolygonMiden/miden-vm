use vm_core::Felt;

use super::{
    parse_decimal_param, parse_element_param, parse_hex_param, parse_int_param, push_value,
    validate_op_len, AssemblyError, BaseElement, Operation, Token,
};

// CONSTANTS
// ================================================================================================

/// The maximum number of constant inputs allowed by `push` operation.
const MAX_CONST_INPUTS: usize = 16;

/// The required length of the hexadecimal representation for an input value when more than one hex
/// input is provided to `push` without period separators.
const HEX_CHUNK_SIZE: usize = 16;

/// The maximum number of elements that can be read from the advice tape in a single `push`
/// operation.
const ADVICE_READ_LIMIT: u32 = 16;

// PUSHING VALUES ONTO THE STACK (PUSH)
// ================================================================================================

/// Pushes constant, environment, memory, or non-deterministic (advice) inputs onto the stack as
/// specified by the operation variant and its parameter(s).
///
/// *CONSTANTS: `push.a`, `push.a.b`, `push.a.b.c...`*
/// Pushes immediate values `a`, `b`, `c`, etc onto the stack in the order in which they're
/// provided. Up to 16 values can be specified. All values must be valid field elements in decimal
/// (e.g. 123) or hexadecimal (e.g. 0x7b) representation. When specifying values in hexadecimal
/// format, it is possible to omit the periods between individual values as long as the total number
/// of specified bytes is a multiple of 8.
///
/// *NON-DETERMINISTIC (ADVICE): `push.adv.n`*
/// Removes the next `n` values from the advice tape and pushes them onto the stack. The number of
/// items that can be read from the advice tape is limited to 16.
///
/// *ENVIRONMENT: `push.env.{var}`*
/// Pushes the value of the specified environment variable onto the top of the stack. Currently, the
/// only environment input is `sdepth`.
///
/// *RANDOM ACCESS MEMORY: `push.mem`, `push.mem.a`*
/// Pushes the first element of the word at memory address `a` onto the stack. If no memory address
/// is specified, it is assumed to be on top of the stack.
///
/// *LOCAL PROCEDURE VARIABLES: `push.local.i`*
/// Pushes the first element of the word at the local memory address with index `i` onto the stack.
///
/// # Errors
///
/// Returns an `AssemblyError` if the op param is invalid, malformed, or doesn't match an expected
/// `push` instruction
pub fn parse_push(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    num_proc_locals: u32,
) -> Result<(), AssemblyError> {
    if op.num_parts() < 2 {
        return Err(AssemblyError::invalid_op(op));
    }
    if op.parts()[0] != "push" {
        return Err(AssemblyError::unexpected_token(
            op,
            "push.{adv.n|env.var|local.i|mem|mem.a|a|a.b|a.b.c...}",
        ));
    }

    match op.parts()[1] {
        "adv" => parse_push_adv(span_ops, op),
        "env" => parse_push_env(span_ops, op),
        "local" => parse_push_local(span_ops, op, num_proc_locals),
        "mem" => parse_push_mem(span_ops, op),
        _ => parse_push_constant(span_ops, op),
    }
}

/// Pushes a word (4 elements) onto the stack from an absolute location in random access memory or
/// from local procedure memory as specified by the operation variant and its parameter.
///
/// *RANDOM ACCESS MEMORY: `pushw.mem`, `pushw.mem.a`*
/// Reads a word (4 elements) from memory and pushes it onto the stack by appending `LOADW` and
/// required stack manipulations to the span block. If no memory address is specified, it is assumed
/// to be on top of the stack. Otherwise, the provided address will be pushed so it is on top of the
/// stack when `LOADW` is executed. The memory address will be removed from the stack by `LOADW`.
///
/// *LOCAL PROCEDURE VARIABLES: `pushw.local.i`*
/// Reads a word (4 elements) from local memory at index `i` and pushes it onto the stack.
///
/// # Errors
///
/// Returns an `AssemblyError` if the op param is invalid, malformed, or doesn't match an expected
/// `pushw` instruction.
pub fn parse_pushw(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    num_proc_locals: u32,
) -> Result<(), AssemblyError> {
    // validate op
    validate_op_len(op, 2, 0, 1)?;
    if op.parts()[0] != "pushw" {
        return Err(AssemblyError::unexpected_token(
            op,
            "pushw.{mem|mem.a|local.i}",
        ));
    }

    match op.parts()[1] {
        "mem" => parse_mem_read(span_ops, op, true),
        "local" => parse_local_read(span_ops, op, num_proc_locals, true),
        _ => Err(AssemblyError::invalid_op(op)),
    }
}

// REMOVING VALUES FROM THE STACK (POP)
// ================================================================================================

/// Pops an element off the stack and saves it to the absolute or local memory location as
/// specified by the operation variant and its parameter.
///
/// *RANDOM ACCESS MEMORY: `pop.mem`, `pop.mem.a`*
/// Pops an element off the stack and saves it at memory address `a`. If no memory address is
/// specified as part of the operation, then it is assumed to be on top of the stack.
///
/// *LOCAL PROCEDURE VARIABLES: `pop.local.i`*
/// Pops an element off the stack and saves it to the local memory address with index `i`.
///
/// # Errors
///
/// Returns an `AssemblyError` if the op param is invalid, malformed, or doesn't match an expected
/// `pop` instruction
pub fn parse_pop(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    num_proc_locals: u32,
) -> Result<(), AssemblyError> {
    if op.num_parts() < 2 {
        return Err(AssemblyError::invalid_op(op));
    }
    if op.parts()[0] != "pop" {
        return Err(AssemblyError::unexpected_token(op, "pop.{mem|mem.a}"));
    }

    match op.parts()[1] {
        "local" => parse_pop_local(span_ops, op, num_proc_locals),
        "mem" => parse_pop_mem(span_ops, op),
        _ => Err(AssemblyError::invalid_op(op)),
    }
}

/// Pops a word (4 elements) from the stack and store it at an absolute memory location or in local
/// procedure memory as specified by the operation variant and its parameter.
///
/// *RANDOM ACCESS MEMORY: `popw.mem`, `popw.mem.a`*
/// Pops the top 4 elements off the stack and stores them at an absolute address in memory by
/// appending `STOREW` and required stack manipulations to the span block. If no memory address is
/// provided as a parameter, the address is assumed to be on top of the stack. Otherwise, the
/// provided address will be pushed so it is on top of the stack when `STOREW` is executed. The
/// memory address will be removed from the stack by `STOREW`.
///
/// *LOCAL PROCEDURE VARIABLES: `popw.local.i`*
/// Pops the top 4 elements off the stack and stores them in local memory at index `i`.
///
/// # Errors
///
/// Returns an `AssemblyError` if the op param is invalid, malformed, or doesn't match an expected
/// `popw` instruction.
pub fn parse_popw(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    num_proc_locals: u32,
) -> Result<(), AssemblyError> {
    // validate op
    validate_op_len(op, 2, 0, 1)?;
    if op.parts()[0] != "popw" {
        return Err(AssemblyError::unexpected_token(
            op,
            "popw.{mem|mem.a|local.i}",
        ));
    }

    match op.parts()[1] {
        "mem" => parse_mem_write(span_ops, op, false),
        "local" => parse_local_write(span_ops, op, num_proc_locals, false),
        _ => Err(AssemblyError::invalid_op(op)),
    }
}

// OVERWRITING VALUES ON THE STACK (LOAD)
// ================================================================================================

/// Overwrites the top 4 elements of the stack with a word (4 elements) loaded from either the
/// advice tape, an absolute location in random access memory, or procedure locals as specified by
/// the operation variant and its parameter.
///
/// *NON-DETERMINISTIC (ADVICE): `loadw.adv`*
/// Removes the next word (4 elements) from the advice tape and overwrites the top 4 elements of the
/// stack with it. Fails if the advice tape has fewer than 4 elements.
///
/// *RANDOM ACCESS MEMORY: `loadw.mem`, `loadw.mem.a`*
/// Reads a word (4 elements) from memory and overwrites the top 4 elements of the stack with it by
/// appending `LOADW` and required stack manipulations to the span block. If no memory address is
/// specified, the address is assumed to be on top of the stack. Otherwise, the provided address
/// will be pushed so it is on top of the stack when `LOADW` is executed. The memory address will be
/// removed from the stack by `LOADW`.
///
/// *LOCAL PROCEDURE VARIABLES: `loadw.local.i`*
/// Reads a word (4 elements) from local memory at index `i` and overwrites the top 4 elements of
/// the stack with it.
///
/// # Errors
///
/// Returns an `AssemblyError` if the op param is invalid, malformed, or doesn't match an expected
/// `loadw` instruction.
pub fn parse_loadw(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    num_proc_locals: u32,
) -> Result<(), AssemblyError> {
    // validate op
    validate_op_len(op, 2, 0, 1)?;
    if op.parts()[0] != "loadw" {
        return Err(AssemblyError::unexpected_token(
            op,
            "loadw.{adv|mem|mem.a|local.i}",
        ));
    }

    match op.parts()[1] {
        "adv" => {
            // ensure that no parameter exists
            if op.num_parts() > 2 {
                return Err(AssemblyError::extra_param(op));
            }

            // load a word from the advice tape
            span_ops.push(Operation::ReadW);
            Ok(())
        }
        "mem" => parse_mem_read(span_ops, op, false),
        "local" => parse_local_read(span_ops, op, num_proc_locals, false),
        _ => Err(AssemblyError::invalid_op(op)),
    }
}

// SAVING STACK VALUES WITHOUT REMOVING THEM (STORE)
// ================================================================================================

/// Stores the top 4 elements of the stack at an absolute memory location or in local procedure
/// memory, as specified by the operation variant and its parameter. If a memory address is provided
/// via the stack, it will be removed first. At the end of the operation, all elements will remain
/// on the stack.
///
/// *RANDOM ACCESS MEMORY: `storew.mem`, `storew.mem.a`*
/// Stores the top 4 elements of the stack at an absolute address in memory by appending `STOREW`
/// and required stack manipulations to the span block. If no memory address is provided as a
/// parameter, the address is assumed to be on top of the stack. Otherwise, the provided address
/// will be pushed so it is on top of the stack when `STOREW` is executed.  The memory address will
/// be removed from the stack by `STOREW`.
///
/// *LOCAL PROCEDURE VARIABLES: `storew.local.i`*
/// Stores the top 4 elements of the stack in local memory at index `i`.
///
/// # Errors
///
/// Returns an `AssemblyError` if the op param is invalid, malformed, or doesn't match an expected
/// `storew` instruction.
pub fn parse_storew(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    num_proc_locals: u32,
) -> Result<(), AssemblyError> {
    // validate op
    validate_op_len(op, 2, 0, 1)?;
    if op.parts()[0] != "storew" {
        return Err(AssemblyError::unexpected_token(
            op,
            "storew.{mem|mem.a|local.i}",
        ));
    }

    match op.parts()[1] {
        "mem" => parse_mem_write(span_ops, op, true),
        "local" => parse_local_write(span_ops, op, num_proc_locals, true),
        _ => Err(AssemblyError::invalid_op(op)),
    }
}

// HELPERS - CONSTANT INPUTS
// ================================================================================================

/// Appends `PUSH` operations to the span block to push one or more provided constant values onto
/// the stack, up to a maximum of 16 values.
///
/// Constant values may be specified in one of 2 formats:
/// 1. A series of 1-16 valid field elements in decimal or hexadecimal representation separated by
///    periods, e.g. push.0x1234.0xabcd
/// 2. A hexadecimal string without period separators that represents a series of 1-16 elements
///    where the total number of specified bytes is a multiple of 8, e.g.
///    push.0x0000000000001234000000000000abcd
///
/// In cases when the immediate value is 0, `PUSH` operation is replaced with `PAD`. Also, in cases
/// when immediate value is 1, `PUSH` operation is replaced with `PAD INCR` because in most cases
/// this will be more efficient than doing a `PUSH`.
///
/// # Errors
///
/// It will return an error if no immediate value is provided or if any of parameter formats are
/// invalid. It will also return an error if the op token is malformed or doesn't match the expected
/// instruction.
fn parse_push_constant(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    let param_idx = 1;
    validate_op_len(op, param_idx, 1, MAX_CONST_INPUTS)?;

    let param_count = op.num_parts() - param_idx;
    // for multiple input parameters, parse & push each one onto the stack in order, then return
    if param_count > 1 {
        for param_idx in param_idx..=param_count {
            let value = parse_element_param(op, param_idx)?;
            push_value(span_ops, value);
        }
        return Ok(());
    }

    // for a single input, there could be one value or there could be a series of many hexadecimal
    // values without separators
    let param_str = op.parts()[param_idx];
    if let Some(param_str) = param_str.strip_prefix("0x") {
        // parse 1 or more hexadecimal values
        let values = parse_hex_params(op, param_idx, param_str)?;
        // push each value onto the stack in order
        for &value in values.iter() {
            push_value(span_ops, value);
        }
    } else {
        // parse 1 decimal value and push it onto the stack
        let value = parse_decimal_param(op, param_idx, param_str)?;
        push_value(span_ops, value);
    }

    Ok(())
}

/// Parses a hexadecimal string into a vector of 1 or more field elements, up to 16 total. If more
/// than one value is specified, then the total number of specified bytes must be a multiple of 8.
fn parse_hex_params(
    op: &Token,
    param_idx: usize,
    param_str: &str,
) -> Result<Vec<BaseElement>, AssemblyError> {
    // handle error cases where the hex string is poorly formed
    let is_single_element = if param_str.len() <= HEX_CHUNK_SIZE {
        if param_str.len() % 2 != 0 {
            // parameter string is not a valid hex representation
            return Err(AssemblyError::invalid_param(op, param_idx));
        }
        true
    } else {
        if param_str.len() % HEX_CHUNK_SIZE != 0 {
            // hex string doesn't contain a valid number of bytes
            return Err(AssemblyError::invalid_param(op, param_idx));
        } else if param_str.len() > HEX_CHUNK_SIZE * MAX_CONST_INPUTS {
            // hex string contains more than the maximum number of inputs
            return Err(AssemblyError::extra_param(op));
        }
        false
    };

    // parse the hex string into one or more valid field elements
    if is_single_element {
        // parse a single element in hex representation
        let parsed_param = parse_hex_param(op, param_idx, param_str)?;
        Ok(vec![parsed_param])
    } else {
        // iterate over the multi-value hex string and parse each 8-byte chunk into a valid element
        (0..param_str.len())
            .step_by(HEX_CHUNK_SIZE)
            .map(|i| parse_hex_param(op, param_idx, &param_str[i..i + HEX_CHUNK_SIZE]))
            .collect()
    }
}

// HELPERS - ENVIRONMENT INPUTS
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
fn parse_push_env(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    // validate the operation
    validate_op_len(op, 3, 0, 0)?;
    if op.parts()[1] != "env" {
        return Err(AssemblyError::unexpected_token(op, "push.env.{var}"));
    }

    // update the span block
    match op.parts()[2] {
        "sdepth" => {
            span_ops.push(Operation::SDepth);
        }
        _ => return Err(AssemblyError::invalid_op(op)),
    }

    Ok(())
}

// HELPERS - NON-DETERMINISTIC INPUTS
// ================================================================================================

/// Appends the number of `READ` operations specified by the operation's immediate value
/// to the span block. This pushes the specified number of items from the advice tape onto the
/// stack. It limits the number of items that can be read from the advice tape at a time to 16.
///
/// # Errors
///
/// Returns an `AssemblyError` if the instruction is invalid, malformed, missing a required
/// parameter, or does not match the expected operation. Returns an `invalid_param` `AssemblyError`
/// if the parameter for `push.adv` is not a decimal value.
fn parse_push_adv(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    // do basic validation common to all advice operations
    validate_op_len(op, 2, 1, 1)?;
    if op.parts()[1] != "adv" {
        return Err(AssemblyError::unexpected_token(op, "push.adv.n"));
    }

    // parse and validate the parameter as the number of items to read from the advice tape
    // it must be between 1 and ADVICE_READ_LIMIT, inclusive, since adv.push.0 is a no-op
    let n = parse_int_param(op, 2, 1, ADVICE_READ_LIMIT)?;

    // read n items from the advice tape and push then onto the stack
    for _ in 0..n {
        span_ops.push(Operation::Read);
    }

    Ok(())
}

// HELPERS - RANDOM ACCESS MEMORY
// ================================================================================================

/// Pushes the first element of the word at the specified memory address onto the stack. The
/// memory address may be provided directly as an immediate value or via the stack.
fn parse_push_mem(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    validate_op_len(op, 2, 0, 1)?;

    parse_mem_read(span_ops, op, true)?;

    for _ in 0..3 {
        span_ops.push(Operation::Drop);
    }

    Ok(())
}

/// Pops the top element off the stack and saves it at the specified memory address. The memory
/// address may be provided directly as an immediate value or via the stack.
fn parse_pop_mem(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    validate_op_len(op, 2, 0, 1)?;

    // pad to word length before calling STOREW
    for _ in 0..3 {
        span_ops.push(Operation::Pad);
    }

    // if the destination memory address was on top of the stack, restore it to the top
    if op.num_parts() == 2 {
        span_ops.push(Operation::MovUp3);
    }

    parse_mem_write(span_ops, op, false)
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
fn parse_mem_read(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    preserve_stack: bool,
) -> Result<(), AssemblyError> {
    if preserve_stack {
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
fn parse_mem_write(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    preserve_stack: bool,
) -> Result<(), AssemblyError> {
    if op.num_parts() == 3 {
        push_mem_addr(span_ops, op)?;
    }

    span_ops.push(Operation::StoreW);

    if !preserve_stack {
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

// HELPERS - LOCAL MEMORY FOR PROCEDURE VARIABLES
// ================================================================================================

/// Pushes the first element of the word at the specified local procedure memory index onto the
/// stack.
fn parse_push_local(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    num_proc_locals: u32,
) -> Result<(), AssemblyError> {
    validate_op_len(op, 2, 1, 1)?;

    parse_local_read(span_ops, op, num_proc_locals, true)?;

    for _ in 0..3 {
        span_ops.push(Operation::Drop);
    }

    Ok(())
}

/// Pops the top element off the stack and saves it at the specified local procedure memory index.
fn parse_pop_local(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    num_proc_locals: u32,
) -> Result<(), AssemblyError> {
    validate_op_len(op, 2, 1, 1)?;

    // pad to word length before calling STOREW
    for _ in 0..3 {
        span_ops.push(Operation::Pad);
    }

    parse_local_write(span_ops, op, num_proc_locals, false)
}

fn parse_local_read(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    num_proc_locals: u32,
    preserve_stack: bool,
) -> Result<(), AssemblyError> {
    // check for parameter
    if op.num_parts() < 3 {
        return Err(AssemblyError::missing_param(op));
    }

    if preserve_stack {
        // make space for the new elements
        for _ in 0..4 {
            span_ops.push(Operation::Pad);
        }
    }

    push_local_addr(span_ops, op, num_proc_locals)?;
    span_ops.push(Operation::LoadW);

    Ok(())
}

fn parse_local_write(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    num_proc_locals: u32,
    preserve_stack: bool,
) -> Result<(), AssemblyError> {
    // check for parameter
    if op.num_parts() < 3 {
        return Err(AssemblyError::missing_param(op));
    }

    push_local_addr(span_ops, op, num_proc_locals)?;
    span_ops.push(Operation::StoreW);

    if !preserve_stack {
        for _ in 0..4 {
            span_ops.push(Operation::Drop);
        }
    }

    Ok(())
}

fn push_local_addr(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    num_proc_locals: u32,
) -> Result<(), AssemblyError> {
    if num_proc_locals == 0 {
        // if no procedure locals were declared, then no local mem ops are allowed
        return Err(AssemblyError::invalid_op_with_reason(
            op,
            "no procedure locals were declared",
        ));
    }

    // parse the provided local memory index
    let index = parse_int_param(op, 2, 0, num_proc_locals - 1)?;

    // put the absolute memory address on the stack
    // negate the value to use it as an offset from the fmp
    // since the fmp value was incremented when locals were allocated
    push_value(span_ops, -Felt::new(index as u64));
    span_ops.push(Operation::FmpAdd);

    Ok(())
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::{BaseElement, FieldElement};

    // TESTS FOR PUSHING VALUES ONTO THE STACK (PUSH)
    // ============================================================================================

    #[test]
    fn push_one() {
        let num_proc_locals = 0;

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

        parse_push(&mut span_ops, &op_0, num_proc_locals).expect("Failed to parse push.0");
        parse_push(&mut span_ops, &op_1, num_proc_locals).expect("Failed to parse push.1");
        parse_push(&mut span_ops, &op_dec, num_proc_locals)
            .expect("Failed to parse push of decimal element 123");
        parse_push(&mut span_ops, &op_hex, num_proc_locals)
            .expect("Failed to parse push of hex element 0x7b");

        assert_eq!(span_ops, expected);
    }

    #[test]
    fn push_many() {
        let num_proc_locals = 0;

        // --- push 4 decimal values --------------------------------------------------------------
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_4_dec = Token::new("push.4.5.6.7", 0);
        let mut expected = Vec::with_capacity(4);
        for a in 4..8 {
            expected.push(Operation::Push(BaseElement::new(a)));
        }
        parse_push(&mut span_ops, &op_4_dec, num_proc_locals)
            .expect("Failed to parse push.4.5.6.7");
        assert_eq!(span_ops, expected);

        // --- push the maximum number of decimal values (16) -------------------------------------
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_16_dec = Token::new("push.16.17.18.19.20.21.22.23.24.25.26.27.28.29.30.31", 0);
        let mut expected = Vec::with_capacity(16);
        for a in 16..32 {
            expected.push(Operation::Push(BaseElement::new(a)));
        }
        parse_push(&mut span_ops, &op_16_dec, num_proc_locals)
            .expect("Failed to parse push.16.17.18.19.20.21.22.23.24.25.26.27.28.29.30.31");
        assert_eq!(span_ops, expected);

        // --- push hexadecimal values with period separators between values ----------------------
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_5_hex = Token::new("push.0xA.0x64.0x3E8.0x2710.0x186A0", 0);
        let mut expected = Vec::with_capacity(5);
        for i in 1..=5 {
            expected.push(Operation::Push(BaseElement::new(10_u64.pow(i))));
        }
        parse_push(&mut span_ops, &op_5_hex, num_proc_locals)
            .expect("Failed to parse push.0xA.0x64.0x3EB.0x2710.0x186A0");
        assert_eq!(span_ops, expected);

        // --- push a mixture of decimal and single-element hexadecimal values --------------------
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_8_dec_hex = Token::new("push.2.4.8.0x10.0x20.0x40.128.0x100", 0);
        let mut expected = Vec::with_capacity(8);
        for i in 1_u32..=8 {
            expected.push(Operation::Push(BaseElement::new(2_u64.pow(i))));
        }
        parse_push(&mut span_ops, &op_8_dec_hex, num_proc_locals)
            .expect("Failed to parse push.2.4.8.0x10.0x20.0x40.128.0x100");
        assert_eq!(span_ops, expected);
    }

    #[test]
    fn push_without_separator() {
        let num_proc_locals = 0;

        // --- push hexadecimal values with no period separators ----------------------------------
        let mut span_ops: Vec<Operation> = Vec::new();
        let mut expected: Vec<Operation> = Vec::new();
        let op_sep = Token::new("push.0x1234.0xabcd", 0);
        let op_no_sep = Token::new("push.0x0000000000001234000000000000abcd", 0);
        parse_push(&mut expected, &op_sep, num_proc_locals)
            .expect("Failed to parse push.0x1234.0xabcd");
        parse_push(&mut span_ops, &op_no_sep, num_proc_locals)
            .expect("Failed to parse push.0x0000000000001234000000000000abcd");
        assert_eq!(span_ops, expected);

        // --- push the maximum number of hexadecimal values without separators (16) --------------
        let mut span_ops: Vec<Operation> = Vec::new();
        let mut expected: Vec<Operation> = Vec::new();
        let op_16_dec = Token::new("push.0.1.2.3.4.5.6.7.8.9.10.11.12.13.14.15", 0);
        let op_16_no_sep = Token::new("push.0x0000000000000000000000000000000100000000000000020000000000000003000000000000000400000000000000050000000000000006000000000000000700000000000000080000000000000009000000000000000A000000000000000B000000000000000C000000000000000D000000000000000E000000000000000F", 0);
        parse_push(&mut expected, &op_16_dec, num_proc_locals)
            .expect("Failed to parse push.0.1.2.3.4.5.6.7.8.9.10.11.12.13.14.15");
        parse_push(&mut span_ops, &op_16_no_sep,num_proc_locals)
            .expect("Failed to parse push.0x0000000000000000000000000000000100000000000000020000000000000003000000000000000400000000000000050000000000000006000000000000000700000000000000080000000000000009000000000000000A000000000000000B000000000000000C000000000000000D000000000000000E000000000000000F");
        assert_eq!(span_ops, expected);
    }

    #[test]
    fn push_invalid() {
        let num_proc_locals = 0;

        // fails when immediate value is invalid or missing
        let mut span_ops: Vec<Operation> = Vec::new();
        let pos = 0;

        // missing value or variant
        let op_no_val = Token::new("push", pos);
        let expected = AssemblyError::invalid_op(&op_no_val);
        assert_eq!(
            parse_push(&mut span_ops, &op_no_val, num_proc_locals).unwrap_err(),
            expected
        );

        // invalid value
        let op_val_invalid = Token::new("push.abc", pos);
        let expected = AssemblyError::invalid_param(&op_val_invalid, 1);
        assert_eq!(
            parse_push(&mut span_ops, &op_val_invalid, num_proc_locals).unwrap_err(),
            expected
        );

        // --- separators for single values cannot be combined with multi-element hex inputs ------
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_mixed_sep = Token::new("push.4.5.0x0000000000001234000000000000abcd", 0);
        let expected = AssemblyError::invalid_param(&op_mixed_sep, 3);
        assert_eq!(
            parse_push(&mut span_ops, &op_mixed_sep, num_proc_locals).unwrap_err(),
            expected
        );

        let mut span_ops: Vec<Operation> = Vec::new();
        let op_mixed_sep = Token::new("push.0x0000000000001234000000000000abcd.4.5", 0);
        let expected = AssemblyError::invalid_param(&op_mixed_sep, 1);
        assert_eq!(
            parse_push(&mut span_ops, &op_mixed_sep, num_proc_locals).unwrap_err(),
            expected
        );

        // extra value
        let op_extra_val = Token::new("push.0.1.2.3.4.5.6.7.8.9.10.11.12.13.14.15.16", pos);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(
            parse_push(&mut span_ops, &op_extra_val, num_proc_locals).unwrap_err(),
            expected
        );

        // wrong operation passed to parsing function
        let op_mismatch = Token::new("pushw.0", pos);
        let expected = AssemblyError::unexpected_token(
            &op_mismatch,
            "push.{adv.n|env.var|local.i|mem|mem.a|a|a.b|a.b.c...}",
        );
        assert_eq!(
            parse_push(&mut span_ops, &op_mismatch, num_proc_locals).unwrap_err(),
            expected
        )
    }

    #[test]
    fn push_invalid_hex() {
        let num_proc_locals = 0;

        // --- no separators, and total number of bytes is not a multiple of 8 --------------------
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_bad_hex = Token::new("push.0x00000000000012340000abcd", 0);
        let expected = AssemblyError::invalid_param(&op_bad_hex, 1);
        assert_eq!(
            parse_push(&mut span_ops, &op_bad_hex, num_proc_locals).unwrap_err(),
            expected
        );

        // --- some period separators, but total number of bytes is not a multiple of 8 --------------------
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_bad_hex = Token::new("push.0x00001234000000000000abcd.0x5678.0x9ef0", 0);
        let expected = AssemblyError::invalid_param(&op_bad_hex, 1);
        assert_eq!(
            parse_push(&mut span_ops, &op_bad_hex, num_proc_locals).unwrap_err(),
            expected
        );

        // --- too many values provided in hex format without separators --------------
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_extra_val = Token::new("push.0x0000000000000000000000000000000100000000000000020000000000000003000000000000000400000000000000050000000000000006000000000000000700000000000000080000000000000009000000000000000A000000000000000B000000000000000C000000000000000D000000000000000E000000000000000F0000000000000010", 0);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(
            parse_push(&mut span_ops, &op_extra_val, num_proc_locals).unwrap_err(),
            expected
        );
    }

    #[test]
    fn push_env_sdepth() {
        let num_proc_locals = 0;

        // pushes the current depth of the stack onto the top of the stack
        let mut span_ops = vec![Operation::Push(BaseElement::ONE); 8];
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
        let expected = AssemblyError::invalid_op(&op_val_invalid);
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

    #[test]
    fn push_adv() {
        let num_proc_locals = 0;

        // remove n items from the advice tape and push them onto the stack
        let mut span_ops: Vec<Operation> = Vec::new();
        let op = Token::new("push.adv.4", 0);
        let expected = vec![Operation::Read; 4];

        parse_push(&mut span_ops, &op, num_proc_locals).expect("Failed to parse push.adv.4");
        assert_eq!(span_ops, expected);
    }

    #[test]
    fn push_adv_invalid() {
        let num_proc_locals = 0;

        // fails when the instruction is malformed or unrecognized
        let mut span_ops: Vec<Operation> = Vec::new();
        let pos = 0;

        // missing value
        let op_no_val = Token::new("push.adv", pos);
        let expected = AssemblyError::missing_param(&op_no_val);
        assert_eq!(
            parse_push(&mut span_ops, &op_no_val, num_proc_locals).unwrap_err(),
            expected
        );

        // extra value to push
        let op_extra_val = Token::new("push.adv.2.2", pos);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(
            parse_push(&mut span_ops, &op_extra_val, num_proc_locals).unwrap_err(),
            expected
        );

        // invalid value - char
        let op_invalid_char = Token::new("push.adv.a", pos);
        let expected = AssemblyError::invalid_param(&op_invalid_char, 2);
        assert_eq!(
            parse_push(&mut span_ops, &op_invalid_char, num_proc_locals).unwrap_err(),
            expected
        );

        // invalid value - hexadecimal
        let op_invalid_hex = Token::new("push.adv.0x10", pos);
        let expected = AssemblyError::invalid_param(&op_invalid_hex, 2);
        assert_eq!(
            parse_push(&mut span_ops, &op_invalid_hex, num_proc_locals).unwrap_err(),
            expected
        );

        // parameter out of bounds
        let reason = format!(
            "parameter value must be greater than or equal to {} and less than or equal to {}",
            1, ADVICE_READ_LIMIT
        );
        // less than lower bound
        let op_lower_bound = Token::new("push.adv.0", pos);
        let expected = AssemblyError::invalid_param_with_reason(&op_lower_bound, 2, &reason);
        assert_eq!(
            parse_push(&mut span_ops, &op_lower_bound, num_proc_locals).unwrap_err(),
            expected
        );

        // greater than upper bound
        let inst_str = format!("push.adv.{}", ADVICE_READ_LIMIT + 1);
        let op_upper_bound = Token::new(&inst_str, pos);
        let expected = AssemblyError::invalid_param_with_reason(&op_upper_bound, 2, &reason);
        assert_eq!(
            parse_push(&mut span_ops, &op_upper_bound, num_proc_locals).unwrap_err(),
            expected
        );
    }

    #[test]
    fn push_mem_invalid() {
        test_parse_mem("push");
    }

    #[test]
    fn push_local_invalid() {
        test_parse_local("push");
    }

    #[test]
    fn pushw_invalid() {
        test_parsew_base("pushw", "pushw.{mem|mem.a|local.i}");
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

    #[test]
    fn pushw_local_invalid() {
        test_parse_local("pushw");
    }

    // TESTS FOR REMOVING VALUES FROM THE STACK (POP)
    // ============================================================================================

    #[test]
    fn pop_mem_invalid() {
        test_parse_mem("pop");
    }

    #[test]
    fn pop_local_invalid() {
        test_parse_local("pop");
    }

    #[test]
    fn popw_invalid() {
        test_parsew_base("popw", "popw.{mem|mem.a|local.i}");
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

    #[test]
    fn popw_local_invalid() {
        test_parse_local("popw");
    }

    // TESTS FOR OVERWRITING VALUES ON THE STACK (LOAD)
    // ============================================================================================

    #[test]
    fn loadw_invalid() {
        test_parsew_base("loadw", "loadw.{adv|mem|mem.a|local.i}");
    }

    #[test]
    fn loadw_adv() {
        let num_proc_locals = 0;

        // replace the top 4 elements of the stack with 4 elements from the advice tape
        let mut span_ops: Vec<Operation> = Vec::new();
        let op = Token::new("loadw.adv", 0);
        let expected = vec![Operation::ReadW];

        parse_loadw(&mut span_ops, &op, num_proc_locals).expect("Failed to parse loadw.adv");
        assert_eq!(span_ops, expected);
    }

    #[test]
    fn loadw_adv_invalid() {
        let num_proc_locals = 0;

        // fails when the instruction is malformed or unrecognized
        let mut span_ops: Vec<Operation> = Vec::new();
        let pos = 0;

        // extra value to loadw
        let op_extra_val = Token::new("loadw.adv.0", pos);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(
            parse_loadw(&mut span_ops, &op_extra_val, num_proc_locals).unwrap_err(),
            expected
        );
    }

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

    #[test]
    fn loadw_local_invalid() {
        test_parse_local("loadw");
    }

    // TESTS FOR SAVING STACK VALUES WITHOUT REMOVING THEM (STORE)
    // ============================================================================================

    #[test]
    fn storew_invalid() {
        test_parsew_base("storew", "storew.{mem|mem.a|local.i}");
    }

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

    #[test]
    fn storew_local_invalid() {
        test_parse_local("storew");
    }

    // TEST HELPERS
    // ============================================================================================

    /// Test that the core part of an instruction for an operation on a word is properly formed.
    /// It can be used to test operation and variant validity for `pushw`, `popw`, `loadw`, and
    /// `storew`.
    fn test_parsew_base(base_op: &str, expected_token: &str) {
        let num_proc_locals = 0;

        // fails when required variants are invalid or missing or the wrong operation is provided
        let pos = 0;

        // missing variant
        let op_no_variant = Token::new(base_op, pos);
        let expected = AssemblyError::invalid_op(&op_no_variant);
        assert_eq!(
            get_parsing_error(base_op, &op_no_variant, num_proc_locals),
            expected
        );

        // invalid variant
        let op_str = format!("{}.invalid", base_op);
        let op_invalid = Token::new(&op_str, pos);
        let expected = AssemblyError::invalid_op(&op_invalid);
        assert_eq!(
            get_parsing_error(base_op, &op_invalid, num_proc_locals),
            expected
        );

        // wrong operation passed to parsing function
        let op_mismatch = Token::new("none.mem", pos);
        let expected = AssemblyError::unexpected_token(&op_mismatch, expected_token);
        assert_eq!(
            get_parsing_error(base_op, &op_mismatch, num_proc_locals),
            expected
        )
    }

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

    /// Test that an instruction for a local memory operation is properly formed. It can be used to
    /// test parameter inputs for pushw.mem, popw.mem, loadw.mem, and storew.mem.
    fn test_parse_local(base_op: &str) {
        let num_proc_locals = 1;

        // fails when immediate values to a {push|pushw|pop|popw|loadw|storew}.local.i operation are
        // invalid or missing
        let pos = 0;

        // insufficient values provided
        let op_str = format!("{}.local", base_op);
        let op_val_missing = Token::new(&op_str, pos);
        let expected = AssemblyError::missing_param(&op_val_missing);
        assert_eq!(
            get_parsing_error(base_op, &op_val_missing, num_proc_locals),
            expected
        );

        // invalid value provided to local variant
        let op_str = format!("{}.local.abc", base_op);
        let op_val_invalid = Token::new(&op_str, pos);
        let expected = AssemblyError::invalid_param(&op_val_invalid, 2);
        assert_eq!(
            get_parsing_error(base_op, &op_val_invalid, num_proc_locals),
            expected
        );

        // no procedure locals declared
        let op_str = format!("{}.local.{}", base_op, 1);
        let op_no_locals = Token::new(&op_str, pos);
        let expected = AssemblyError::invalid_op_with_reason(
            &op_no_locals,
            "no procedure locals were declared",
        );
        assert_eq!(get_parsing_error(base_op, &op_no_locals, 0), expected);

        // provided local index is outside of the declared bounds of the procedure locals
        let op_str = format!("{}.local.{}", base_op, num_proc_locals);
        let op_val_invalid = Token::new(&op_str, pos);
        let expected = AssemblyError::invalid_param_with_reason(
            &op_val_invalid,
            2,
            format!(
                "parameter value must be greater than or equal to 0 and less than or equal to {}",
                num_proc_locals - 1
            )
            .as_str(),
        );
        assert_eq!(
            get_parsing_error(base_op, &op_val_invalid, num_proc_locals),
            expected
        );

        // extra value provided to local variant
        let op_str = format!("{}.local.0.1", base_op);
        let op_extra_val = Token::new(&op_str, pos);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(
            get_parsing_error(base_op, &op_extra_val, num_proc_locals),
            expected
        );
    }

    /// Attempts to parse the specified operation, which is expected to be invalid, and returns the
    /// parsing error.
    fn get_parsing_error(base_op: &str, invalid_op: &Token, num_proc_locals: u32) -> AssemblyError {
        let mut span_ops: Vec<Operation> = Vec::new();

        match base_op {
            "push" => parse_push(&mut span_ops, invalid_op, num_proc_locals).unwrap_err(),
            "pushw" => parse_pushw(&mut span_ops, invalid_op, num_proc_locals).unwrap_err(),
            "pop" => parse_pop(&mut span_ops, invalid_op, num_proc_locals).unwrap_err(),
            "popw" => parse_popw(&mut span_ops, invalid_op, num_proc_locals).unwrap_err(),
            "loadw" => parse_loadw(&mut span_ops, invalid_op, num_proc_locals).unwrap_err(),
            "storew" => parse_storew(&mut span_ops, invalid_op, num_proc_locals).unwrap_err(),
            _ => panic!(),
        }
    }
}
