use super::{ExecutionError, Process};

// CRYPTOGRAPHIC OPERATIONS
// ================================================================================================

impl Process {
    // HASHING OPERATIONS
    // --------------------------------------------------------------------------------------------
    /// Applies Rescue Prime permutation to the top 12 elements of the stack. The outer part of the
    /// state is assumed to be at the top of the stack.
    ///
    /// # Errors
    /// Returns an error if the stack contains fewer than 12 elements.
    pub(super) fn op_rpperm(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(12, "RPPERM")?;

        let input_state = [
            self.stack.get(0),
            self.stack.get(1),
            self.stack.get(2),
            self.stack.get(3),
            self.stack.get(4),
            self.stack.get(5),
            self.stack.get(6),
            self.stack.get(7),
            self.stack.get(8),
            self.stack.get(9),
            self.stack.get(10),
            self.stack.get(11),
        ];

        let (_addr, output_state) = self.hasher.permute(input_state);

        for (i, &value) in output_state.iter().enumerate() {
            self.stack.set(i, value);
        }
        self.stack.copy_state(12);
        Ok(())
    }

    // MERKLE TREES
    // --------------------------------------------------------------------------------------------

    /// Computes a root of a Merkle path for the specified node. The stack is expected to be
    /// arranged as follows (from the top):
    /// - depth of the path, 1 element.
    /// - index of the node, 1 element.
    /// - value of the node, 4 elements.
    /// - root of the tree, 4 elements.
    ///
    /// To perform the operation we do the following:
    /// 1. Look up the Merkle path in the advice provider for the specified tree root.
    /// 2. Use the hasher to compute the root of the Merkle path for the specified node.
    /// 3. Replace the node value with the computed root.
    /// 4. Pop the depth value off the stack.
    ///
    /// If the correct Merkle path was provided, the computed root and the provided root must be
    /// the same. This can be checked via subsequent operations.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The stack contains fewer than 10 elements.
    /// - Merkle tree for the specified root cannot be found in the advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree
    ///   identified by the specified root.
    /// - Path to the node at the specified depth and index is not known to the advice provider.
    pub(super) fn op_mpverify(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(10, "MPVERIFY")?;

        // read depth, index, node value, and root value from the stack
        let depth = self.stack.get(0);
        let index = self.stack.get(1);
        let node = [
            self.stack.get(5),
            self.stack.get(4),
            self.stack.get(3),
            self.stack.get(2),
        ];
        let provided_root = [
            self.stack.get(9),
            self.stack.get(8),
            self.stack.get(7),
            self.stack.get(6),
        ];

        // get a Merkle path from the advice provider for the specified root and node index.
        // the path is expected to be of the specified depth.
        let path = self.advice.get_merkle_path(provided_root, depth, index)?;

        // use hasher to compute the Merkle root of the path
        let (_addr, computed_root) = self.hasher.build_merkle_root(node, &path, index);

        // this can happen only if the advice provider returns a Merkle path inconsistent with
        // the specified root. in general, programs using this operations should check that the
        // computed root and the provided root are the same.
        debug_assert_eq!(
            provided_root, computed_root,
            "inconsistent Merkle tree root"
        );

        // pop the depth off the stack, replace the node value with the computed root, and shift
        // the rest of the stack by one item to the left
        self.stack.set(0, index);
        for (i, &value) in computed_root.iter().rev().enumerate() {
            self.stack.set(i + 1, value);
        }
        self.stack.shift_left(6);
        Ok(())
    }

    /// Computes a new root of a Merkle tree where a node at the specified position is updated to
    /// the specified value. The stack is expected to be arranged as follows (from the top):
    /// - depth of the node, 1 element
    /// - index of the node, 1 element
    /// - old value of the node, 4 element
    /// - new value of the node, 4 element
    /// - current root of the tree, 4 elements
    ///
    /// To perform the operation we do the following:
    /// 1. Look up the Merkle path in the advice provider for the specified tree root.
    /// 2. Use the hasher to update the root of the Merkle path for the specified node. For this
    ///    we need to provide the old and the new node value.
    /// 3. Replace the node value with the computed root.
    /// 4. Pop the depth value off the stack.
    ///
    /// The Merkle path for the node is expected to be provided by the prover non-deterministically
    /// (via advice sets). At the end of the operation, the old node value is replaced with the
    /// old root value computed based on the provided path, the new node value is replaced by the
    /// new root value computed based on the same path. Everything else remains the same.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The stack contains fewer than 14 elements.
    /// - Merkle tree for the specified root cannot be found in the advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree
    ///   identified by the specified root.
    /// - Path to the node at the specified depth and index is not known to the advice provider.
    pub(super) fn op_mrupdate(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(14, "MRUPDATE")?;

        // read depth, index, old and new node values, and tree root value from the stack
        let depth = self.stack.get(0);
        let index = self.stack.get(1);
        let old_node = [
            self.stack.get(5),
            self.stack.get(4),
            self.stack.get(3),
            self.stack.get(2),
        ];
        let new_node = [
            self.stack.get(9),
            self.stack.get(8),
            self.stack.get(7),
            self.stack.get(6),
        ];
        let old_root = [
            self.stack.get(13),
            self.stack.get(12),
            self.stack.get(11),
            self.stack.get(10),
        ];

        // get a Merkle path from the advice provider for the old root and node index.
        // the path is expected to be of the specified depth.
        let path = self.advice.get_merkle_path(old_root, depth, index)?;

        // use hasher to update the Merkle root
        let (_addr, computed_old_root, new_root) = self
            .hasher
            .update_merkle_root(old_node, new_node, &path, index);

        // this can happen only if the advice provider returns a Merkle path inconsistent with
        // the specified root. in general, programs using this operations should check that the
        // computed old root and the provided old root are the same.
        debug_assert_eq!(old_root, computed_old_root, "inconsistent Merkle tree root");

        // replace the node values with computed old and new roots; everything else stays the same
        self.stack.set(0, depth);
        self.stack.set(1, index);
        for (i, &value) in computed_old_root.iter().rev().enumerate() {
            self.stack.set(i + 2, value);
        }
        for (i, &value) in new_root.iter().rev().enumerate() {
            self.stack.set(i + 6, value);
        }
        self.stack.copy_state(10);

        Ok(())
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        super::{init_stack_with, BaseElement, FieldElement, Operation, StarkField},
        Process,
    };
    use crate::Word;
    use rand_utils::rand_vector;
    use vm_core::{AdviceSet, ProgramInputs};
    use winterfell::crypto::{hashers::Rp64_256, ElementHasher};

    #[test]
    fn op_rpperm() {
        // --- test hashing [ONE, ONE] ----------------------------------------
        let expected = Rp64_256::hash_elements(&[BaseElement::ONE, BaseElement::ONE]);

        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1]);
        process.execute_op(Operation::RpPerm).unwrap();
        assert_eq!(expected.as_elements(), &process.stack.trace_state()[..4]);

        // --- test hashing 8 random values -----------------------------------
        let mut values = rand_vector::<u64>(8);
        let expected = Rp64_256::hash_elements(
            &values
                .iter()
                .map(|&v| BaseElement::new(v))
                .collect::<Vec<_>>(),
        );

        let mut process = Process::new_dummy();
        values.extend_from_slice(&[0, 0, 0, 8]);
        // reverse the values so that the outer part of the state is at the top of the stack
        values.reverse();
        init_stack_with(&mut process, &values);
        process.execute_op(Operation::RpPerm).unwrap();
        assert_eq!(expected.as_elements(), &process.stack.trace_state()[..4]);
    }

    #[test]
    fn op_mpverify() {
        let index = 5usize;
        let leaves = inti_leaves(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let tree = AdviceSet::new_merkle_tree(leaves.to_vec()).unwrap();

        let inti_stack = [
            tree.depth() as u64,
            index as u64,
            leaves[index][3].as_int(),
            leaves[index][2].as_int(),
            leaves[index][1].as_int(),
            leaves[index][0].as_int(),
            tree.root()[3].as_int(),
            tree.root()[2].as_int(),
            tree.root()[1].as_int(),
            tree.root()[0].as_int(),
        ];

        let inputs = ProgramInputs::new(&inti_stack, &[], vec![tree.clone()]).unwrap();
        let mut process = Process::new(inputs);

        process.execute_op(Operation::MpVerify).unwrap();
        let expected = build_expected(&[
            BaseElement::new(index as u64),
            tree.root()[3],
            tree.root()[2],
            tree.root()[1],
            tree.root()[0],
            tree.root()[3],
            tree.root()[2],
            tree.root()[1],
            tree.root()[0],
        ]);
        assert_eq!(expected, process.stack.trace_state());
    }

    #[test]
    fn op_mrupdate() {
        let leaves = inti_leaves(&[1, 2, 3, 4, 5, 6, 7, 8]);

        let node_index = 1usize;
        let new_node = init_leaf(9);
        let mut new_leaves = leaves.clone();
        new_leaves[node_index] = new_node;

        let tree = AdviceSet::new_merkle_tree(leaves.clone()).unwrap();
        let new_tree = AdviceSet::new_merkle_tree(new_leaves).unwrap();

        let inti_stack = [
            tree.depth() as u64,
            node_index as u64,
            leaves[node_index][3].as_int(),
            leaves[node_index][2].as_int(),
            leaves[node_index][1].as_int(),
            leaves[node_index][0].as_int(),
            new_node[3].as_int(),
            new_node[2].as_int(),
            new_node[1].as_int(),
            new_node[0].as_int(),
            tree.root()[3].as_int(),
            tree.root()[2].as_int(),
            tree.root()[1].as_int(),
            tree.root()[0].as_int(),
        ];

        let inputs = ProgramInputs::new(&inti_stack, &[], vec![tree.clone()]).unwrap();
        let mut process = Process::new(inputs);

        process.execute_op(Operation::MrUpdate).unwrap();
        let expected = build_expected(&[
            BaseElement::new(tree.depth() as u64),
            BaseElement::new(node_index as u64),
            tree.root()[3],
            tree.root()[2],
            tree.root()[1],
            tree.root()[0],
            new_tree.root()[3],
            new_tree.root()[2],
            new_tree.root()[1],
            new_tree.root()[0],
            tree.root()[3],
            tree.root()[2],
            tree.root()[1],
            tree.root()[0],
        ]);
        assert_eq!(expected, process.stack.trace_state());
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------
    fn inti_leaves(values: &[u64]) -> Vec<Word> {
        values.iter().map(|&v| init_leaf(v)).collect()
    }

    fn init_leaf(value: u64) -> Word {
        [
            BaseElement::new(value),
            BaseElement::ZERO,
            BaseElement::ZERO,
            BaseElement::ZERO,
        ]
    }

    fn build_expected(values: &[BaseElement]) -> [BaseElement; 16] {
        let mut expected = [BaseElement::ZERO; 16];
        for (&value, result) in values.iter().zip(expected.iter_mut()) {
            *result = value
        }
        expected
    }
}
