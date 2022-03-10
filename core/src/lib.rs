pub mod hasher;
pub mod program;
pub mod utils;
pub use math::{fields::f64::BaseElement as Felt, FieldElement, StarkField};

mod operations;
pub use operations::{AdviceInjector, DebugOptions, Operation};

mod inputs;
pub use inputs::{AdviceSet, ProgramInputs};

pub mod errors;

// TYPE ALIASES
// ================================================================================================

pub type Word = [Felt; 4];

// CONSTANTS
// ================================================================================================

/// The minimum stack depth enforced by the VM. This is also the number of stack registers which can
/// be accessed by the VM directly.
pub const MIN_STACK_DEPTH: usize = 16;
