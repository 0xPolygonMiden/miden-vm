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

        // pop the depth off the stack, replace the leaf value with the computed root, and shift
        // the rest of the stack by one item to the left
        self.stack.set(0, index);
        for (i, &value) in computed_root.iter().rev().enumerate() {
            self.stack.set(i + 1, value);
        }
        self.stack.shift_left(6);
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
        let leaves = [init_leaf(1), init_leaf(2), init_leaf(3), init_leaf(4)];

        let tree = AdviceSet::new_merkle_tree(leaves.to_vec()).unwrap();
        let inti_stack = [
            tree.depth() as u64,
            0,
            leaves[0][3].as_int(),
            leaves[0][2].as_int(),
            leaves[0][1].as_int(),
            leaves[0][0].as_int(),
            tree.root()[3].as_int(),
            tree.root()[2].as_int(),
            tree.root()[1].as_int(),
            tree.root()[0].as_int(),
        ];

        let inputs = ProgramInputs::new(&inti_stack, &[], vec![tree.clone()]).unwrap();
        let mut process = Process::new(inputs);

        process.execute_op(Operation::MpVerify).unwrap();
        let expected = build_expected(&[
            BaseElement::new(0),
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

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------
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
