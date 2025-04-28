use vm_core::{Felt, chiplets::hasher::STATE_WIDTH, crypto::hash::Rpo256, utils::range};

use super::FastProcessor;
use crate::{AdviceProvider, ExecutionError, Host};

impl FastProcessor {
    /// Applies a permutation of the Rpo256 hash function to the top 12 elements of the stack.
    ///
    /// Analogous to `Process::op_hperm`.
    pub fn op_hperm(&mut self) {
        let state_range = range(self.stack_top_idx - STATE_WIDTH, STATE_WIDTH);
        let hashed_state = {
            let mut input_state: [Felt; STATE_WIDTH] =
                self.stack[state_range.clone()].try_into().unwrap();

            Rpo256::apply_permutation(&mut input_state);

            input_state
        };

        self.stack[state_range].copy_from_slice(&hashed_state);
    }

    /// Analogous to `Process::op_mpverify`.
    pub fn op_mpverify(
        &mut self,
        err_code: u32,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        // read node value, depth, index and root value from the stack
        let node = self.stack_get_word(0);
        let depth = self.stack_get(4);
        let index = self.stack_get(5);
        let root = self.stack_get_word(6);

        // get a Merkle path from the advice provider for the specified root and node index
        let path = host.advice_provider_mut().get_merkle_path(root, &depth, &index)?;

        // verify the path
        match path.verify(index.as_int(), node.into(), &root.into()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ExecutionError::MerklePathVerificationFailed {
                value: node,
                index,
                root: root.into(),
                err_code,
            }),
        }
    }

    /// Analogous to `Process::op_mrupdate`.
    pub fn op_mrupdate(&mut self, host: &mut impl Host) -> Result<(), ExecutionError> {
        // read old node value, depth, index, tree root and new node values from the stack
        let old_node = self.stack_get_word(0);
        let depth = self.stack_get(4);
        let index = self.stack_get(5);
        let old_root = self.stack_get_word(6);
        let new_node = self.stack_get_word(10);

        // update the node at the specified index in the Merkle tree specified by the old root, and
        // get a Merkle path to it. The length of the returned path is expected to match the
        // specified depth. If the new node is the root of a tree, this instruction will append the
        // whole sub-tree to this node.
        let (path, new_root) = host
            .advice_provider_mut()
            .update_merkle_node(old_root, &depth, &index, new_node)?;

        assert_eq!(path.len(), depth.as_int() as usize);

        // verify that the old node is consistent with the Merkle path
        if path.verify(index.as_int(), old_node.into(), &old_root.into()).is_err() {
            return Err(ExecutionError::MerklePathVerificationFailed {
                value: old_node,
                index,
                root: old_root.into(),
                err_code: 0,
            });
        }

        // Replace the old node value with computed new root; everything else remains the same.
        self.stack_write_word(0, &new_root);

        Ok(())
    }
}
