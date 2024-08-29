use super::{
    errors::{InputError, OutputError},
    Felt, ToElements,
};
use crate::utils::{ByteWriter, Serializable};

mod inputs;
pub use inputs::StackInputs;

mod outputs;
pub use outputs::StackOutputs;

// CONSTANTS
// ================================================================================================

/// Represents:
/// - Number of elements that can be initialized at the start of execution and remain populated at
///   the end of execution.
/// - Number of elements that can be accessed directly via instructions.
/// - Number of elements that remain visible to the callee when the context is switched via `call`
///   or `syscall` instructions.
/// - Number of elements below which the depth of the stack never drops.
pub const MIN_STACK_DEPTH: usize = 16;

// HELPER FUNCTIONS
// ================================================================================================

/// Get the number of non-zero stack elements.
fn get_stack_values_num(values: &[Felt]) -> u8 {
    let mut acc = 0;
    for v in values.iter().rev() {
        if v.as_int() == 0 {
            acc += 1;
        } else {
            break;
        }
    }
    (MIN_STACK_DEPTH - acc) as u8
}
