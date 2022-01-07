use super::{BaseElement, FieldElement};
use vm_core::v1::hasher::{apply_round, NUM_ROUNDS, STATE_WIDTH};

// TYPE ALIASES
// ================================================================================================

type HasherState = [BaseElement; STATE_WIDTH];

// HASHER
// ================================================================================================

/// TODO: add docs
pub struct Hasher {}

impl Hasher {
    /// TODO: add docs
    pub fn new() -> Self {
        Self {}
    }

    /// TODO: add docs
    pub fn permute(&mut self, mut state: HasherState) -> (BaseElement, HasherState) {
        for i in 0..NUM_ROUNDS {
            // TODO: record state into a trace
            apply_round(&mut state, i);
        }

        (BaseElement::ZERO, state)
    }
}
