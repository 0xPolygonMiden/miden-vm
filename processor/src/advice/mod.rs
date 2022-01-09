use super::{BaseElement, ExecutionError, ProgramInputs, Word};
use std::collections::BTreeMap;
use vm_core::{utils::IntoBytes, AdviceSet, StarkField};

/// TODO: add docs
pub struct AdviceProvider {
    step: usize,
    tape: Vec<BaseElement>,
    sets: BTreeMap<[u8; 32], AdviceSet>,
}

impl AdviceProvider {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// TODO: add docs
    pub fn new(inputs: ProgramInputs) -> Self {
        let (_, mut advice_tape, advice_sets) = inputs.into_parts();

        // reverse the advice tape so that we can pop elements off the end
        advice_tape.reverse();

        // put advice sets into a map
        let mut advice_map = BTreeMap::new();
        for merkle_set in advice_sets {
            advice_map.insert(merkle_set.root().into_bytes(), merkle_set);
        }

        Self {
            step: 0,
            tape: advice_tape,
            sets: advice_map,
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
    pub fn get_merkle_path(
        &mut self,
        root: Word,
        depth: BaseElement,
        index: BaseElement,
    ) -> Result<Vec<Word>, ExecutionError> {
        // TODO: return error if not found
        let merkle_set = self.sets.get(&root.into_bytes()).unwrap();

        Ok(merkle_set.get_path(depth.as_int() as u32, index.as_int()))
    }

    // CONTEXT MANAGEMENT
    // --------------------------------------------------------------------------------------------

    /// Increments the clock cycle.
    pub fn advance_clock(&mut self) {
        self.step += 1;
    }
}
