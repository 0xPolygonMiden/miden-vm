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

    /// TODO: add docs
    pub(super) fn op_mpverify(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(10, "MPVERIFY")?;

        // read depth, index, leaf value, and root value from the stack
        let depth = self.stack.get(0);
        let index = self.stack.get(1);
        let leaf = [
            self.stack.get(2),
            self.stack.get(3),
            self.stack.get(4),
            self.stack.get(5),
        ];
        let provided_root = [
            self.stack.get(6),
            self.stack.get(7),
            self.stack.get(8),
            self.stack.get(9),
        ];

        // get a Merkle path from the advice provider for the specified root and leaf index.
        // the path is expected to be of the specified depth.
        let path = self.advice.get_merkle_path(provided_root, depth, index)?;

        // use hasher to compute the Merkle root of the path
        let (_addr, computed_root) = self.hasher.build_merkle_root(leaf, &path, index);

        // pop the depth off the stack, replace the leaf value with the computed root, and shift
        // the rest of the stack by one item to the left
        self.stack.set(0, index);
        for (i, &value) in computed_root.iter().enumerate() {
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
        super::{init_stack_with, BaseElement, FieldElement, Operation},
        Process,
    };
    use rand_utils::rand_vector;
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
}
