use super::{
    super::validate_operation, parse_decimal_param, parse_element_param, parse_hex_param,
    parse_u32_param, push_value, AssemblyError, Felt, Operation, Token, Vec,
};
pub use adv_ops::{parse_adv_loadw, parse_adv_push};
pub use constant_ops::parse_push;
pub use env_ops::{parse_locaddr, parse_sdepth};
pub use mem_ops::{parse_mem_read, parse_mem_write};
use vm_core::{AdviceInjector, Decorator, DecoratorList};

mod adv_ops;
mod constant_ops;
mod env_ops;
mod mem_ops;

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
/// - adv.keyval: this operation reads four elements at the top of the stack, uses it as a key to
///   take vector from key-value map and injects elements of this vector into the advice tape.
pub fn parse_adv_inject(
    span_ops: &mut [Operation],
    op: &Token,
    decorators: &mut DecoratorList,
) -> Result<(), AssemblyError> {
    match op.parts()[1] {
        "u64div" => decorators.push((
            span_ops.len(),
            Decorator::Advice(AdviceInjector::DivResultU64),
        )),
        "keyval" => decorators.push((span_ops.len(), Decorator::Advice(AdviceInjector::MapValue))),
        _ => return Err(AssemblyError::invalid_op(op)),
    };

    Ok(())
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        adv_ops::parse_adv_push, parse_adv_loadw, parse_locaddr, parse_mem_read, parse_mem_write,
        parse_push, parse_sdepth, AssemblyError, Operation, Token,
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

            "adv_loadw" => parse_adv_loadw(&mut span_ops, invalid_op).unwrap_err(),
            "adv_push" => parse_adv_push(&mut span_ops, invalid_op).unwrap_err(),
            _ => panic!(),
        }
    }
}
