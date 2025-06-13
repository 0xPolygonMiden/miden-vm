use miden_air::trace::decoder::NUM_USER_OP_HELPERS;
use vm_core::{Felt, ZERO};

use super::CoreTraceFragmentGenerator;
use crate::processor::Processor;

impl CoreTraceFragmentGenerator {
    /// Performs a hash permutation operation.
    /// Applies Rescue Prime Optimized permutation to the top 12 elements of the stack.
    pub(crate) fn op_hperm(&mut self) -> [Felt; NUM_USER_OP_HELPERS] {
        let (addr, computed_hash) = self.state.hasher.replay_permutation();

        // Put the result back on the stack (in reverse order)
        for (i, &value) in computed_hash.iter().rev().enumerate() {
            self.stack_write(i, value);
        }

        [addr, ZERO, ZERO, ZERO, ZERO, ZERO]
    }

    /// Verifies a Merkle path.
    ///
    /// In this implementation, we don't actually verify the path.
    pub(crate) fn op_mpverify(&mut self) -> [Felt; NUM_USER_OP_HELPERS] {
        let depth = self.stack_get(4);
        let index = self.stack_get(5);
        let root = self.stack_get_word(6);

        // Replay the Merkle path retrieval from the advice provider
        let _path = self.state.advice.replay_merkle_path(root, depth, index);
        let (addr, computed_root) = self.state.hasher.replay_build_merkle_root();

        debug_assert_eq!(root, computed_root, "Merkle root mismatch");

        [addr, ZERO, ZERO, ZERO, ZERO, ZERO]
    }

    /// Updates the Merkle root.
    pub(crate) fn op_mrupdate(&mut self) {
        todo!()
    }

    /// Performs FRI extension fold operation.
    pub(crate) fn op_fri_ext2fold4(&mut self) {
        todo!()
    }

    /// Evaluates a polynomial using Horner's method (base field).
    pub(crate) fn op_horner_eval_base(&mut self) {
        todo!()
    }

    /// Evaluates a polynomial using Horner's method (extension field).
    pub(crate) fn op_horner_eval_ext(&mut self) {
        todo!()
    }

    /// Evaluates an arithmetic circuit.
    pub(crate) fn op_arithmetic_circuit_eval(&mut self) {
        todo!()
    }
}
