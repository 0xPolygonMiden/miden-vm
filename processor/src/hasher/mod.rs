use super::{BaseElement, FieldElement, StarkField, Word};
use vm_core::hasher::{apply_round, NUM_ROUNDS, STATE_WIDTH};

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

        // TODO: return address of the hash table row
        (BaseElement::ZERO, state)
    }

    /// TODO: add docs
    pub fn build_merkle_root(
        &mut self,
        value: Word,
        path: &[Word],
        index: BaseElement,
    ) -> (BaseElement, Word) {
        let mut root = value;
        let mut index = index.as_int();

        for sibling in path {
            let mut state = if index & 1 == 0 {
                build_merge_state(&root, sibling)
            } else {
                build_merge_state(sibling, &root)
            };

            for i in 0..NUM_ROUNDS {
                // TODO: record state into a trace
                apply_round(&mut state, i);
            }
            root = [state[0], state[1], state[2], state[3]];
            index >>= 1;
        }

        // TODO: return address of the hash table row
        (BaseElement::ZERO, root)
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn build_merge_state(a: &Word, b: &Word) -> HasherState {
    [
        a[0],
        a[1],
        a[2],
        a[3],
        b[0],
        b[1],
        b[2],
        b[3],
        BaseElement::ZERO,
        BaseElement::ZERO,
        BaseElement::ZERO,
        BaseElement::new(8),
    ]
}
