use super::{BaseElement, ExecutionError, ProgramInputs, Word};

pub struct AdviceProvider {
    step: usize,
    tape: Vec<BaseElement>,
}

impl AdviceProvider {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// TODO: add docs
    pub fn new(inputs: &ProgramInputs) -> Self {
        // the advice tape is reversed so that we can pop elements off the end
        Self {
            step: 0,
            tape: inputs.advice_tape().iter().rev().cloned().collect(),
        }
    }

    // ADVICE TAPE
    // --------------------------------------------------------------------------------------------

    /// Removes the next element from the advice tape and returns it.
    ///
    /// # Errors
    /// Returns an error if the advice tape is empty.
    pub fn read_tape(&mut self) -> Result<BaseElement, ExecutionError> {
        self.tape
            .pop()
            .ok_or(ExecutionError::EmptyAdviceTape(self.step))
    }

    // MERKLE PATHS
    // --------------------------------------------------------------------------------------------

    /// Returns a Merkle path to a leaf at the specified index in a Merkle tree with the specified
    /// root.
    ///
    /// # Errors
    /// Returns an error if:
    /// * A Merkle path provider for the specified root cannot be found in this advice provider.
    /// * The specified depth is greater than the depth of the Merkle tree identified by the
    ///   specified root.
    /// * The Merkle path provider for the specified root does not contain a Merkle path for a
    ///   leaf at the specified depth and index.
    #[allow(unused)]
    pub fn get_merkle_path(
        &mut self,
        _root: Word,
        _depth: BaseElement,
        _index: BaseElement,
    ) -> Result<Vec<Word>, ExecutionError> {
        // TODO: implement
        unimplemented!()
    }

    // CONTEXT MANAGEMENT
    // --------------------------------------------------------------------------------------------

    /// Increments the clock cycle.
    pub fn advance_clock(&mut self) {
        self.step += 1;
    }
}
