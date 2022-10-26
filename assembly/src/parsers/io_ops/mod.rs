use super::{push_value, AssemblyError, Felt, Operation, Vec};
pub use adv_ops::{parse_adv_loadw, parse_adv_push};
pub use constant_ops::parse_push;
pub use env_ops::{parse_locaddr, parse_sdepth};
pub use mem_ops::{parse_mem_global, parse_mem_local};
use vm_core::{AdviceInjector, Decorator, DecoratorList};

mod adv_ops;
mod constant_ops;
mod env_ops;
mod mem_ops;

// ADVICE INJECTORS
// ================================================================================================

// Appends the appropriate advice injector operation to `span_ops`.
//
// Advice injector operations insert one or more values at the head of the advice tape, but do
// not modify the VM state and do not advance the clock cycles.

/// Handles the adv.u64div operation.
/// This operation interprets four elements at the top of the stack as two 64-bit
/// values (represented by 32-bit limbs), divides one value by another, and injects the quotient
/// and the remainder into the advice tape.
pub fn parse_adv_inject_u64div(
    span_ops: &mut [Operation],
    decorators: &mut DecoratorList,
) -> Result<(), AssemblyError> {
    decorators.push((
        span_ops.len(),
        Decorator::Advice(AdviceInjector::DivResultU64),
    ));
    Ok(())
}

/// Handles the adv.keyval operation.
/// This operation reads four elements at the top of the stack, uses it as a key to
/// take vector from key-value map and injects elements of this vector into the advice tape.
pub fn parse_adv_inject_keyval(
    span_ops: &mut [Operation],
    decorators: &mut DecoratorList,
) -> Result<(), AssemblyError> {
    decorators.push((span_ops.len(), Decorator::Advice(AdviceInjector::MapValue)));
    Ok(())
}
