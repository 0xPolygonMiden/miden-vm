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
