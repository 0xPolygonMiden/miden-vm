use crate::{ExecutionError, Felt, Process, StarkField};
use core::fmt;
use vm_core::Word;

/// VmState holds a current process state information at a specific clock cycle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VmState {
    pub clk: usize,
    pub fmp: Felt,
    pub stack: Vec<Felt>,
    pub memory: Vec<(u64, Word)>,
}

impl fmt::Display for VmState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stack: Vec<u64> = self.stack.iter().map(|x| x.as_int()).collect();
        let memory: Vec<(u64, [u64; 4])> = self
            .memory
            .iter()
            .map(|x| (x.0, word_to_ints(&x.1)))
            .collect();
        write!(
            f,
            "clk={}, fmp={}, stack={:?}, memory={:?}",
            self.clk, self.fmp, stack, memory
        )
    }
}

/// Iterator that iterates through vm state at each step of the execution.
/// This allows debugging or replaying ability to view various process state
/// at each clock cycle.
/// If the execution returned an error, it returns that error on the clock cycle
/// it stopped.
pub struct VmStateIterator {
    process: Process,
    error: Option<ExecutionError>,
    clk: usize,
}

impl VmStateIterator {
    pub(super) fn new(process: Process, result: Result<(), ExecutionError>) -> Self {
        Self {
            process,
            error: result.err(),
            clk: 0,
        }
    }
}

impl Iterator for VmStateIterator {
    type Item = Result<VmState, ExecutionError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.clk > self.process.system.clk() {
            match &self.error {
                Some(_) => {
                    let error = core::mem::take(&mut self.error);
                    return Some(Err(error.unwrap()));
                }
                None => return None,
            }
        }

        let result = Some(Ok(VmState {
            clk: self.clk,
            fmp: self.process.system.get_fmp_at(self.clk),
            stack: self.process.stack.get_state_at(self.clk),
            memory: self
                .process
                .memory
                .get_values_at(0..=u64::MAX, self.clk as u64),
        }));

        self.clk += 1;

        result
    }
}

// HELPER FUNCTIONS
// =================================================================
fn word_to_ints(word: &Word) -> [u64; 4] {
    [
        word[0].as_int(),
        word[1].as_int(),
        word[2].as_int(),
        word[3].as_int(),
    ]
}
