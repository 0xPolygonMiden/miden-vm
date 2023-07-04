#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionOptions {
    max_cycles: Option<u32>,
    expected_cycles: u32,
}

impl Default for ExecutionOptions {
    fn default() -> Self {
        ExecutionOptions {
            max_cycles: None,
            expected_cycles: 64,
        }
    }
}

impl ExecutionOptions {
    pub fn new(max_cycles: Option<u32>, expected_cycles: u32) -> Self {
        ExecutionOptions {
            max_cycles,
            expected_cycles,
        }
    }
}
