use vm_core::{chiplets::hasher::STATE_WIDTH, crypto::hash::Rpo256, utils::range, Felt};

use super::SpeedyGonzales;

impl<const N: usize> SpeedyGonzales<N> {
    /// Applies a permutation of the Rpo256 hash function to the top 12 elements of the stack.
    pub fn op_hperm(&mut self) {
        let hashed_state = {
            let mut input_state: [Felt; STATE_WIDTH] = self.stack
                [range(self.stack_top_idx - STATE_WIDTH, STATE_WIDTH)]
            .try_into()
            .unwrap();

            Rpo256::apply_permutation(&mut input_state);

            input_state
        };

        self.stack[range(self.stack_top_idx - STATE_WIDTH, STATE_WIDTH)]
            .copy_from_slice(&hashed_state);
    }
}
