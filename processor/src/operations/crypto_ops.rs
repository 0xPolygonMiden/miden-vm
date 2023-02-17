use super::{AdviceProvider, ExecutionError, Operation, Process};
use vm_core::StarkField;

// CRYPTOGRAPHIC OPERATIONS
// ================================================================================================

impl<A> Process<A>
where
    A: AdviceProvider,
{
    // HASHING OPERATIONS
    // --------------------------------------------------------------------------------------------
    /// Applies a permutation of Rescue Prime Optimized to the top 12 elements of the stack. The
    /// stack is assumed to be arranged so that the 8 elements of the rate are at the top of the
    /// stack. The capacity word follows, with the element that specifies the padding rule at the
    /// deepest position in stack[11]. For an RPO permutation of [A, B, C] where A is the capacity,
    /// the stack should be arranged (from the top) as [C, B, A, ...].
    pub(super) fn op_hperm(&mut self) -> Result<(), ExecutionError> {
        let input_state = [
            self.stack.get(11),
            self.stack.get(10),
            self.stack.get(9),
            self.stack.get(8),
            self.stack.get(7),
            self.stack.get(6),
            self.stack.get(5),
            self.stack.get(4),
            self.stack.get(3),
            self.stack.get(2),
            self.stack.get(1),
            self.stack.get(0),
        ];

        let (_addr, output_state) = self.chiplets.permute(input_state);

        for (i, &value) in output_state.iter().rev().enumerate() {
            self.stack.set(i, value);
        }
        self.stack.copy_state(12);
        Ok(())
    }

    // MERKLE TREES
    // --------------------------------------------------------------------------------------------

    /// Verifies that a Merkle path from the specified node resolves to the specified root. The
    /// stack is expected to be arranged as follows (from the top):
    /// - value of the node, 4 elements.
    /// - depth of the node, 1 element; this is expected to be the depth of the Merkle tree
    /// - index of the node, 1 element.
    /// - root of the tree, 4 elements.
    ///
    /// To perform the operation we do the following:
    /// 1. Look up the Merkle path in the advice provider for the specified tree root.
    /// 2. Use the hasher to compute the root of the Merkle path for the specified node.
    /// 3. Verify that the computed root is equal to the root provided via the stack.
    /// 4. Copy the stack state over to the next clock cycle with no changes.
    ///
    /// # Errors
    /// Returns an error if:
    /// - Merkle tree for the specified root cannot be found in the advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree
    ///   identified by the specified root.
    /// - Path to the node at the specified depth and index is not known to the advice provider.
    ///
    /// # Panics
    /// Panics if the computed root does not match the root provided via the stack.
    pub(super) fn op_mpverify(&mut self) -> Result<(), ExecutionError> {
        // read node value, depth, index and root value from the stack
        let node = [self.stack.get(3), self.stack.get(2), self.stack.get(1), self.stack.get(0)];
        let depth = self.stack.get(4);
        let index = self.stack.get(5);
        let provided_root =
            [self.stack.get(9), self.stack.get(8), self.stack.get(7), self.stack.get(6)];

        // get a Merkle path from the advice provider for the specified root and node index.
        // the path is expected to be of the specified depth.
        let path = self.advice_provider.get_merkle_path(provided_root, depth, index)?;

        // use hasher to compute the Merkle root of the path
        let (addr, computed_root) = self.chiplets.build_merkle_root(node, &path, index);

        // save address(r) of the hasher trace from when the computation starts in the decoder
        // helper registers.
        self.decoder.set_user_op_helpers(Operation::MpVerify, &[addr]);

        // Asserting the computed root of the merkle path from the advice provider is consistent with
        // the input root.
        assert_eq!(provided_root, computed_root, "inconsistent Merkle tree root");

        // The same state is copied over to the next clock cycle with no changes.
        self.stack.copy_state(0);
        Ok(())
    }

    /// Computes a new root of a Merkle tree where a leaf at the specified index is updated to
    /// the specified value. The stack is expected to be arranged as follows (from the top):
    /// - old value of the node, 4 elements.
    /// - depth of the node, 1 element; this is expected to be the depth of the Merkle tree.
    /// - index of the node, 1 element.
    /// - current root of the tree, 4 elements.
    /// - new value of the node, 4 elements.
    ///
    /// To perform the operation we do the following:
    /// 1. Update the leaf node at the specified index in the advice provider with the specified
    ///    root, and get the Merkle path to this leaf. If `copy` is set to true, we make a copy
    ///    of the advice set before updating it.
    /// 2. Use the hasher to update the root of the Merkle path for the specified node. For this
    ///    we need to provide the old and the new node value.
    /// 3. Verify that the computed old root is equal to the input root provided via the stack.
    /// 4. Replace the old node value with the computed new root.
    ///
    /// The Merkle path for the node is expected to be provided by the prover non-deterministically
    /// (via advice sets). At the end of the operation, the old node value is replaced with the
    /// new root value computed based on the provided path. Everything else on the stack remains the
    /// same.
    ///
    /// If `copy` is set to true, at the end of the operation the advice provide will keep both,
    /// the old and the new advice sets. Otherwise, the old advice set is removed from the
    /// provider.
    ///
    ///
    /// # Errors
    /// Returns an error if:
    /// - Merkle tree for the specified root cannot be found in the advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree
    ///   identified by the specified root.
    /// - Path to the node at the specified depth and index is not known to the advice provider.
    ///
    /// # Panics
    /// Panics if the computed old root does not match the input root provided via the stack.
    pub(super) fn op_mrupdate(&mut self, copy: bool) -> Result<(), ExecutionError> {
        // read old node value, depth, index, tree root and new node values from the stack
        let old_node = [self.stack.get(3), self.stack.get(2), self.stack.get(1), self.stack.get(0)];
        let depth = self.stack.get(4);
        let index = self.stack.get(5);
        let old_root = [self.stack.get(9), self.stack.get(8), self.stack.get(7), self.stack.get(6)];
        let new_node =
            [self.stack.get(13), self.stack.get(12), self.stack.get(11), self.stack.get(10)];

        // update the leaf at the specified index in the advice set specified by the old root, and
        // get a Merkle path to the specified leaf. the length of the returned path is expected to
        // match the specified depth.
        // TODO: in the future, we should be able to replace sub-trees and not just the leaves,
        // and, thus, the assert on depth would not be needed.
        let path = self.advice_provider.update_merkle_leaf(old_root, index, new_node, copy)?;
        assert_eq!(path.len(), depth.as_int() as usize);

        let merkle_tree_update = self.chiplets.update_merkle_root(old_node, new_node, &path, index);

        // Asserts the computed old root of the merkle path from the advice provider is consistent
        // with the input root provided via the stack. This will panic only if the advice provider
        // returns a Merkle path inconsistent with the specified root.
        assert_eq!(old_root, merkle_tree_update.get_old_root(), "inconsistent Merkle tree root");

        // save address(r) of the hasher trace from when the computation starts in the decoder
        // helper registers.
        self.decoder
            .set_user_op_helpers(Operation::MrUpdate(copy), &[merkle_tree_update.get_address()]);

        // Replace the old node value with computed new root; everything else remains the same.
        for (i, &value) in merkle_tree_update.get_new_root().iter().rev().enumerate() {
            self.stack.set(i, value);
        }
        self.stack.copy_state(4);

        Ok(())
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        super::{Felt, FieldElement, Operation, StarkField},
        Process,
    };
    use crate::{AdviceInputs, StackInputs, Word};
    use rand_utils::rand_vector;
    use vm_core::{
        chiplets::hasher::{apply_permutation, STATE_WIDTH},
        AdviceSet,
    };

    #[test]
    fn op_hperm() {
        // --- test hashing [ONE, ONE] ------------------------------------------------------------
        let inputs: [u64; STATE_WIDTH] = [2, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0];
        let stack = StackInputs::try_from_values(inputs).unwrap();
        let mut process = Process::new_dummy(stack);

        let expected: [Felt; STATE_WIDTH] = build_expected_perm(&inputs);
        process.execute_op(Operation::HPerm).unwrap();
        assert_eq!(expected, &process.stack.trace_state()[0..12]);

        // --- test hashing 8 random values -------------------------------------------------------
        let values = rand_vector::<u64>(8);
        let mut inputs: Vec<u64> = vec![values.len() as u64, 0, 0, 0];
        inputs.extend_from_slice(&values);
        let stack = StackInputs::try_from_values(inputs.clone()).unwrap();
        let mut process = Process::new_dummy(stack);

        // add the capacity to prepare the input vector
        let expected: [Felt; STATE_WIDTH] = build_expected_perm(&inputs);
        process.execute_op(Operation::HPerm).unwrap();
        assert_eq!(expected, &process.stack.trace_state()[0..12]);

        // --- test that the rest of the stack isn't affected -------------------------------------
        let mut inputs: Vec<u64> = vec![1, 2, 3, 4];
        let expected = inputs.iter().rev().map(|&v| Felt::new(v)).collect::<Vec<Felt>>();
        let values: Vec<u64> = vec![2, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0];
        inputs.extend_from_slice(&values);

        let stack = StackInputs::try_from_values(inputs).unwrap();
        let mut process = Process::new_dummy(stack);
        process.execute_op(Operation::HPerm).unwrap();
        assert_eq!(expected, &process.stack.trace_state()[12..16]);
    }

    #[test]
    fn op_mpverify() {
        let index = 5usize;
        let leaves = init_leaves(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let tree = AdviceSet::new_merkle_tree(leaves.to_vec()).unwrap();

        let stack_inputs = [
            tree.root()[0].as_int(),
            tree.root()[1].as_int(),
            tree.root()[2].as_int(),
            tree.root()[3].as_int(),
            index as u64,
            tree.depth() as u64,
            leaves[index][0].as_int(),
            leaves[index][1].as_int(),
            leaves[index][2].as_int(),
            leaves[index][3].as_int(),
        ];

        let advice_inputs = AdviceInputs::default().with_merkle_sets(vec![tree.clone()]).unwrap();
        let stack_inputs = StackInputs::try_from_values(stack_inputs).unwrap();
        let mut process =
            Process::new_dummy_with_inputs_and_decoder_helpers(stack_inputs, advice_inputs);

        process.execute_op(Operation::MpVerify).unwrap();
        let expected_stack = build_expected(&[
            leaves[index][3],
            leaves[index][2],
            leaves[index][1],
            leaves[index][0],
            Felt::new(tree.depth() as u64),
            Felt::new(index as u64),
            tree.root()[3],
            tree.root()[2],
            tree.root()[1],
            tree.root()[0],
        ]);
        assert_eq!(expected_stack, process.stack.trace_state());
    }

    #[test]
    fn op_mrupdate_move() {
        let leaves = init_leaves(&[1, 2, 3, 4, 5, 6, 7, 8]);

        let node_index = 1usize;
        let new_node = init_leaf(9);
        let mut new_leaves = leaves.clone();
        new_leaves[node_index] = new_node;

        let tree = AdviceSet::new_merkle_tree(leaves.clone()).unwrap();
        let new_tree = AdviceSet::new_merkle_tree(new_leaves).unwrap();

        let stack_inputs = [
            new_node[0].as_int(),
            new_node[1].as_int(),
            new_node[2].as_int(),
            new_node[3].as_int(),
            tree.root()[0].as_int(),
            tree.root()[1].as_int(),
            tree.root()[2].as_int(),
            tree.root()[3].as_int(),
            node_index as u64,
            tree.depth() as u64,
            leaves[node_index][0].as_int(),
            leaves[node_index][1].as_int(),
            leaves[node_index][2].as_int(),
            leaves[node_index][3].as_int(),
        ];

        let advice_inputs = AdviceInputs::default().with_merkle_sets(vec![tree.clone()]).unwrap();
        let stack_inputs = StackInputs::try_from_values(stack_inputs).unwrap();
        let mut process =
            Process::new_dummy_with_inputs_and_decoder_helpers(stack_inputs, advice_inputs);

        // update the Merkle tree and discard the old copy
        process.execute_op(Operation::MrUpdate(false)).unwrap();
        let expected_stack = build_expected(&[
            new_tree.root()[3],
            new_tree.root()[2],
            new_tree.root()[1],
            new_tree.root()[0],
            Felt::new(tree.depth() as u64),
            Felt::new(node_index as u64),
            tree.root()[3],
            tree.root()[2],
            tree.root()[1],
            tree.root()[0],
            new_node[3],
            new_node[2],
            new_node[1],
            new_node[0],
        ]);
        assert_eq!(expected_stack, process.stack.trace_state());

        // make sure the old Merkle tree was discarded
        assert!(!process.advice_provider.has_advice_set(tree.root()));
        assert!(process.advice_provider.has_advice_set(new_tree.root()));
    }

    #[test]
    fn op_mrupdate_copy() {
        let leaves = init_leaves(&[1, 2, 3, 4, 5, 6, 7, 8]);

        let node_index = 5usize;
        let new_node = init_leaf(9);
        let mut new_leaves = leaves.clone();
        new_leaves[node_index] = new_node;

        let tree = AdviceSet::new_merkle_tree(leaves.clone()).unwrap();
        let new_tree = AdviceSet::new_merkle_tree(new_leaves).unwrap();

        let stack_inputs = [
            new_node[0].as_int(),
            new_node[1].as_int(),
            new_node[2].as_int(),
            new_node[3].as_int(),
            tree.root()[0].as_int(),
            tree.root()[1].as_int(),
            tree.root()[2].as_int(),
            tree.root()[3].as_int(),
            node_index as u64,
            tree.depth() as u64,
            leaves[node_index][0].as_int(),
            leaves[node_index][1].as_int(),
            leaves[node_index][2].as_int(),
            leaves[node_index][3].as_int(),
        ];

        let advice_inputs = AdviceInputs::default().with_merkle_sets(vec![tree.clone()]).unwrap();
        let stack_inputs = StackInputs::try_from_values(stack_inputs).unwrap();
        let mut process =
            Process::new_dummy_with_inputs_and_decoder_helpers(stack_inputs, advice_inputs);

        // update the Merkle tree but keep the old copy
        process.execute_op(Operation::MrUpdate(true)).unwrap();
        let expected_stack = build_expected(&[
            new_tree.root()[3],
            new_tree.root()[2],
            new_tree.root()[1],
            new_tree.root()[0],
            Felt::new(tree.depth() as u64),
            Felt::new(node_index as u64),
            tree.root()[3],
            tree.root()[2],
            tree.root()[1],
            tree.root()[0],
            new_node[3],
            new_node[2],
            new_node[1],
            new_node[0],
        ]);
        assert_eq!(expected_stack, process.stack.trace_state());

        // make sure both Merkle trees are still in the advice set
        assert!(process.advice_provider.has_advice_set(tree.root()));
        assert!(process.advice_provider.has_advice_set(new_tree.root()));
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------
    fn init_leaves(values: &[u64]) -> Vec<Word> {
        values.iter().map(|&v| init_leaf(v)).collect()
    }

    fn init_leaf(value: u64) -> Word {
        [Felt::new(value), Felt::ZERO, Felt::ZERO, Felt::ZERO]
    }

    fn build_expected(values: &[Felt]) -> [Felt; 16] {
        let mut expected = [Felt::ZERO; 16];
        for (&value, result) in values.iter().zip(expected.iter_mut()) {
            *result = value;
        }
        expected
    }

    fn build_expected_perm(values: &[u64]) -> [Felt; STATE_WIDTH] {
        let mut expected = [Felt::ZERO; STATE_WIDTH];
        for (&value, result) in values.iter().zip(expected.iter_mut()) {
            *result = Felt::new(value);
        }
        apply_permutation(&mut expected);
        expected.reverse();

        expected
    }
}
