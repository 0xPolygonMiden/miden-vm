use super::{AdviceInjector, ExecutionError, Process};

// DECORATORS
// ================================================================================================

impl Process {
    // DEBUGGING
    // --------------------------------------------------------------------------------------------

    /// TODO: implement
    pub fn op_debug(&self) -> Result<(), ExecutionError> {
        unimplemented!();
    }

    // ADVICE INJECTION
    // --------------------------------------------------------------------------------------------

    /// Process the specified advice injector.
    pub fn op_advice(&mut self, injector: AdviceInjector) -> Result<(), ExecutionError> {
        match injector {
            AdviceInjector::MerkleNode => self.inject_merkle_node(),
        }
    }

    // INJECTOR HELPERS
    // --------------------------------------------------------------------------------------------

    /// Injects a node of the Merkle tree specified by the values on the stack at the head of the
    /// advice tape. The stack is expected to be arranged as follows (from the top):
    /// - depth of the node, 1 element
    /// - index of the node, 1 element
    /// - root of the tree, 4 elements
    ///
    /// # Errors
    /// Returns an error if:
    /// - The stack contains fewer than 6 elements.
    /// - Merkle tree for the specified root cannot be found in the advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree
    ///   identified by the specified root.
    /// - Value of the node at the specified depth and index is not known to the advice provider.
    fn inject_merkle_node(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(6, "INJMKNODE")?;

        // read node depth, node index, and tree root from the stack
        let depth = self.stack.get(0);
        let index = self.stack.get(1);
        let root = [
            self.stack.get(5),
            self.stack.get(4),
            self.stack.get(3),
            self.stack.get(2),
        ];

        // look up the node in the advice provider
        let node = self.advice.get_tree_node(root, depth, index)?;

        // write the node into the advice tape with first element written last so that it can be
        // removed first
        self.advice.write_tape(node[3]);
        self.advice.write_tape(node[2]);
        self.advice.write_tape(node[1]);
        self.advice.write_tape(node[0]);

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
    use crate::Word;

    use vm_core::{AdviceInjector, AdviceSet, ProgramInputs};

    #[test]
    fn inject_merkle_node() {
        let leaves = [init_leaf(1), init_leaf(2), init_leaf(3), init_leaf(4)];

        let tree = AdviceSet::new_merkle_tree(leaves.to_vec()).unwrap();
        let inti_stack = [
            tree.depth() as u64,
            1,
            tree.root()[3].as_int(),
            tree.root()[2].as_int(),
            tree.root()[1].as_int(),
            tree.root()[0].as_int(),
        ];

        let inputs = ProgramInputs::new(&inti_stack, &[], vec![tree.clone()]).unwrap();
        let mut process = Process::new(inputs);

        // inject the node into the advice tape
        process
            .execute_op(Operation::Advice(AdviceInjector::MerkleNode))
            .unwrap();
        // read the node from the tape onto the stack
        process.execute_op(Operation::Read).unwrap();
        process.execute_op(Operation::Read).unwrap();
        process.execute_op(Operation::Read).unwrap();
        process.execute_op(Operation::Read).unwrap();

        let expected = build_expected(&[
            leaves[1][3],
            leaves[1][2],
            leaves[1][1],
            leaves[1][0],
            Felt::new(2),
            Felt::new(1),
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
        [Felt::new(value), Felt::ZERO, Felt::ZERO, Felt::ZERO]
    }

    fn build_expected(values: &[Felt]) -> [Felt; 16] {
        let mut expected = [Felt::ZERO; 16];
        for (&value, result) in values.iter().zip(expected.iter_mut()) {
            *result = value
        }
        expected
    }
}
