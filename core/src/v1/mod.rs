pub mod program;
pub use math::{fields::f62::BaseElement, FieldElement, StarkField};

// CONSTANTS
// ================================================================================================

/// Number of stack registers which can be accesses by the VM directly.
pub const STACK_TOP_SIZE: usize = 16;
