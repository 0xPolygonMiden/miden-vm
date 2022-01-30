use super::{Felt, FieldElement, StarkField, Word};
use vm_core::hasher::{apply_round, NUM_ROUNDS, STATE_WIDTH};

// TYPE ALIASES
// ================================================================================================

type HasherState = [Felt; STATE_WIDTH];

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
    pub fn permute(&mut self, mut state: HasherState) -> (Felt, HasherState) {
        for i in 0..NUM_ROUNDS {
            // TODO: record state into a trace
            apply_round(&mut state, i);
        }

        // TODO: return address of the hash table row
        (Felt::ZERO, state)
    }

    /// TODO: add docs
    pub fn build_merkle_root(&mut self, value: Word, path: &[Word], index: Felt) -> (Felt, Word) {
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
            root = [state[4], state[5], state[6], state[7]];
            index >>= 1;
        }

        // TODO: return address of the hash table row
        (Felt::ZERO, root)
    }

    pub fn update_merkle_root(
        &mut self,
        old_value: Word,
        new_value: Word,
        path: &[Word],
        index: Felt,
    ) -> (Felt, Word, Word) {
        let mut old_root = old_value;
        let mut new_root = new_value;
        let mut index = index.as_int();

        for sibling in path {
            let (mut old_state, mut new_state) = if index & 1 == 0 {
                (
                    build_merge_state(&old_root, sibling),
                    build_merge_state(&new_root, sibling),
                )
            } else {
                (
                    build_merge_state(sibling, &old_root),
                    build_merge_state(sibling, &new_root),
                )
            };

            for i in 0..NUM_ROUNDS {
                // TODO: record state into a trace
                apply_round(&mut old_state, i);
                apply_round(&mut new_state, i);
            }
            old_root = [old_state[4], old_state[5], old_state[6], old_state[7]];
            new_root = [new_state[4], new_state[5], new_state[6], new_state[7]];
            index >>= 1;
        }

        // TODO: return address of the hash table row
        (Felt::ZERO, old_root, new_root)
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn build_merge_state(a: &Word, b: &Word) -> HasherState {
    [
        Felt::new(8),
        Felt::ZERO,
        Felt::ZERO,
        Felt::ZERO,
        a[0],
        a[1],
        a[2],
        a[3],
        b[0],
        b[1],
        b[2],
        b[3],
    ]
}
