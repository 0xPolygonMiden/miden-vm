pub mod hasher;
pub mod program;
pub mod utils;
pub use math::{fields::f64::BaseElement, FieldElement, StarkField};

mod inputs;
pub use inputs::{ProgramInputs, AdviceSet};

// CONSTANTS
// ================================================================================================

/// Number of stack registers which can be accesses by the VM directly.
pub const STACK_TOP_SIZE: usize = 16;
