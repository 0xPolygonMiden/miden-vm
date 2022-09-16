use super::{
    super::validate_operation, parse_decimal_param, parse_element_param, parse_hex_param,
    parse_u32_param, push_value, AssemblyError, Felt, Operation, Token, Vec,
};
use vm_core::{AdviceInjector, Decorator, DecoratorList};

mod adv_ops;
mod constant_ops;
mod env_ops;
mod mem_ops;
use adv_ops::*;
use constant_ops::*;
pub use env_ops::*;
pub use mem_ops::*;

// PUSHING VALUES ONTO THE STACK (PUSH)
// ================================================================================================

/// Pushes constant or non-deterministic (advice) inputs onto the stack as
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
/// # Errors
///
/// Returns an `AssemblyError` if the op param is invalid, malformed, or doesn't match an expected
/// `push` instruction
pub fn parse_push(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    debug_assert_eq!(op.parts()[0], "push");
    if op.num_parts() < 2 {
        return Err(AssemblyError::invalid_op(op));
    }

    match op.parts()[1] {
        "adv" => parse_push_adv(span_ops, op),
        _ => parse_push_constant(span_ops, op),
    }
}

// OVERWRITING VALUES ON THE STACK (LOAD)
// ================================================================================================

/// Overwrites the top 4 elements of the stack with a word (4 elements) loaded from the
/// advice tape.
///
/// *NON-DETERMINISTIC (ADVICE): `loadw.adv`*
/// Removes the next word (4 elements) from the advice tape and overwrites the top 4 elements of the
/// stack with it. Fails if the advice tape has fewer than 4 elements.
///
/// # Errors
///
/// Returns an `AssemblyError` if the op param is invalid, malformed, or doesn't match an expected
/// `loadw` instruction.
pub fn parse_loadw_advice(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    validate_operation!(op, "loadw.adv");

    match op.parts()[1] {
        "adv" => parse_loadw_adv(span_ops, op),
        _ => Err(AssemblyError::invalid_op(op)),
    }
}

// ADVICE INJECTORS
// ================================================================================================

/// Appends the appropriate advice injector operation to `span_ops`.
///
/// Advice injector operations insert one or more values at the head of the advice tape, but do
/// not modify the VM state and do not advance the clock cycles. Currently, the following advice
/// injectors can be invoked explicitly:
/// - adv.u64div: this operation interprets four elements at the top of the stack as two 64-bit
///   values (represented by 32-bit limbs), divides one value by another, and injects the quotient
///   and the remainder into the advice tape.
pub fn parse_adv_inject(
    span_ops: &mut [Operation],
    op: &Token,
    decorators: &mut DecoratorList,
) -> Result<(), AssemblyError> {
    validate_operation!(op, "adv.u64div");
    match op.parts()[1] {
        "u64div" => decorators.push((
            span_ops.len(),
            Decorator::Advice(AdviceInjector::DivResultU64),
        )),
        _ => return Err(AssemblyError::invalid_op(op)),
    };

    Ok(())
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        parse_loadw_advice, parse_locaddr, parse_mem_read, parse_mem_write, parse_push,
        parse_sdepth, AssemblyError, Operation, Token,
    };

    // TEST HELPERS
    // ============================================================================================

    /// Attempts to parse the specified operation, which is expected to be invalid, and returns the
    /// parsing error.
    pub fn get_parsing_error(
        base_op: &str,
        invalid_op: &Token,
        num_proc_locals: u32,
    ) -> AssemblyError {
        let mut span_ops: Vec<Operation> = Vec::new();

        match base_op {
            "push" => parse_push(&mut span_ops, invalid_op).unwrap_err(),
            "sdepth" => parse_sdepth(&mut span_ops, invalid_op).unwrap_err(),
            "locaddr" => parse_locaddr(&mut span_ops, invalid_op, num_proc_locals).unwrap_err(),

            "mem_load" => {
                parse_mem_read(&mut span_ops, invalid_op, num_proc_locals, false, true).unwrap_err()
            }
            "loc_load" => {
                parse_mem_read(&mut span_ops, invalid_op, num_proc_locals, true, true).unwrap_err()
            }

            "mem_loadw" => parse_mem_read(&mut span_ops, invalid_op, num_proc_locals, false, false)
                .unwrap_err(),
            "loc_loadw" => {
                parse_mem_read(&mut span_ops, invalid_op, num_proc_locals, true, false).unwrap_err()
            }

            "mem_store" => parse_mem_write(&mut span_ops, invalid_op, num_proc_locals, false, true)
                .unwrap_err(),
            "loc_store" => {
                parse_mem_write(&mut span_ops, invalid_op, num_proc_locals, true, true).unwrap_err()
            }

            "mem_storew" => {
                parse_mem_write(&mut span_ops, invalid_op, num_proc_locals, false, false)
                    .unwrap_err()
            }
            "loc_storew" => {
                parse_mem_write(&mut span_ops, invalid_op, num_proc_locals, true, false)
                    .unwrap_err()
            }

            "loadw" => parse_loadw_advice(&mut span_ops, invalid_op).unwrap_err(),
            _ => panic!(),
        }
    }
}
