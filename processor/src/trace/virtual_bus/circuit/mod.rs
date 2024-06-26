use alloc::vec::Vec;
use vm_core::FieldElement;

mod error;
mod prover;
pub use prover::prove;

/// Represents a claim to be proven by a subsequent call to the sum-check protocol.
#[derive(Debug)]
pub struct GkrClaim<E: FieldElement> {
    pub evaluation_point: Vec<E>,
    pub claimed_evaluation: (E, E),
}
