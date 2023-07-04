#[cfg(test)]
mod tests {
    use crate::{ExecutionOptions, Kernel, MemAdviceProvider, Operation, Process, StackInputs};

    // Check that process returns an error if a maximum number of cycles is exceeded.
    #[test]
    fn cycles_num_exceeded() {
        let stack = StackInputs::default();
        let advice_provider = MemAdviceProvider::default();
        let mut process = Process::new(
            Kernel::default(),
            stack,
            advice_provider,
            ExecutionOptions::new(Some(64), 64).unwrap(),
        );
        for _ in 0..64 {
            process.execute_op(Operation::Noop).unwrap();
        }
        assert!(process.execute_op(Operation::Noop).is_err());
    }
}
