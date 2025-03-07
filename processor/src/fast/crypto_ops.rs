use vm_core::{chiplets::hasher::STATE_WIDTH, crypto::hash::Rpo256, utils::range, Felt};

use super::SpeedyGonzales;
use crate::{AdviceProvider, ExecutionError, Host};

impl SpeedyGonzales {
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

    pub fn op_mpverify(
        &mut self,
        err_code: u32,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        // read node value, depth, index and root value from the stack
        let node = [
            self.stack[self.stack_top_idx - 4],
            self.stack[self.stack_top_idx - 3],
            self.stack[self.stack_top_idx - 2],
            self.stack[self.stack_top_idx - 1],
        ];
        let depth = self.stack[self.stack_top_idx - 5];
        let index = self.stack[self.stack_top_idx - 6];
        let root = [
            self.stack[self.stack_top_idx - 10],
            self.stack[self.stack_top_idx - 9],
            self.stack[self.stack_top_idx - 8],
            self.stack[self.stack_top_idx - 7],
        ];

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

    pub fn op_mrupdate(&mut self, host: &mut impl Host) -> Result<(), ExecutionError> {
        // read old node value, depth, index, tree root and new node values from the stack
        let old_node = [
            self.stack[self.stack_top_idx - 4],
            self.stack[self.stack_top_idx - 3],
            self.stack[self.stack_top_idx - 2],
            self.stack[self.stack_top_idx - 1],
        ];
        let depth = self.stack[self.stack_top_idx - 5];
        let index = self.stack[self.stack_top_idx - 6];
        let old_root = [
            self.stack[self.stack_top_idx - 10],
            self.stack[self.stack_top_idx - 9],
            self.stack[self.stack_top_idx - 8],
            self.stack[self.stack_top_idx - 7],
        ];
        let new_node = [
            self.stack[self.stack_top_idx - 14],
            self.stack[self.stack_top_idx - 13],
            self.stack[self.stack_top_idx - 12],
            self.stack[self.stack_top_idx - 11],
        ];

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
        self.stack[self.stack_top_idx - 1] = new_root[3];
        self.stack[self.stack_top_idx - 2] = new_root[2];
        self.stack[self.stack_top_idx - 3] = new_root[1];
        self.stack[self.stack_top_idx - 4] = new_root[0];

        Ok(())
    }
}
