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

/// The number of stack registers which can be accessed by the VM directly. This is also the
/// minimum stack depth enforced by the VM.
pub const STACK_TOP_SIZE: usize = 16;

/// Maximum number of elements allowed for the input and output stack.
pub const STACK_DEPTH: usize = 16;
