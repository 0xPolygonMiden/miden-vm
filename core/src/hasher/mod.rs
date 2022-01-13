use super::BaseElement;
use crypto::{hashers::Rp64_256, ElementHasher, Hasher};

// TYPES ALIASES
// ================================================================================================

pub type Digest = <Rp64_256 as Hasher>::Digest;

// CONSTANTS
// ================================================================================================

/// TODO: add docs
pub const STATE_WIDTH: usize = Rp64_256::STATE_WIDTH;

/// TODO: add docs
pub const NUM_ROUNDS: usize = Rp64_256::NUM_ROUNDS;

// PASS-THROUGH FUNCTIONS
// ================================================================================================

/// TODO: add docs
#[inline(always)]
pub fn merge(values: &[Digest; 2]) -> Digest {
    Rp64_256::merge(values)
}

/// TODO: add docs
#[inline(always)]
pub fn hash_elements(elements: &[BaseElement]) -> Digest {
    Rp64_256::hash_elements(elements)
}

/// Rescue-XLIX round function.
#[inline(always)]
pub fn apply_round(state: &mut [BaseElement; STATE_WIDTH], round: usize) {
    Rp64_256::apply_round(state, round)
}
