use super::{BaseElement, ExecutionError, ProgramInputs, Word};
use vm_core::{utils::IntoBytes, AdviceSet, StarkField};
use winter_utils::collections::{BTreeMap, Vec};

// ADVICE PROVIDER
// ================================================================================================

/// An advice provider supplies non-deterministic inputs to the processor during program execution.
///
/// The provider manages two types of inputs:
/// 1. An advice tape, from which the program can read elements sequentially. Once read, the
///    element is removed from the tape.
/// 2. Advise sets, which can be identified by their roots. Advise sets are views into Merkle
///    trees and can be used to provide Merkle paths.
///
/// An advice provider can be instantiated from [ProgramInputs].
pub struct AdviceProvider {
    step: usize,
    tape: Vec<BaseElement>,
    sets: BTreeMap<[u8; 32], AdviceSet>,
}

impl AdviceProvider {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new advice provider instantiated from the specified program inputs.
    pub fn new(inputs: ProgramInputs) -> Self {
        let (_, mut advice_tape, advice_sets) = inputs.into_parts();

        // reverse the advice tape so that we can pop elements off the end
        advice_tape.reverse();

        Self {
            step: 0,
            tape: advice_tape,
            sets: advice_sets,
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

    // ADVISE SETS
    // --------------------------------------------------------------------------------------------

    /// Returns a node at the specified index in a Merkle tree with the specified root.
    ///
    /// # Errors
    /// Returns an error if:
    /// - A Merkle tree for the specified root cannot be found in this advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree
    ///   identified by the specified root.
    /// - Value of the node at the specified depth and index is not known to this advice provider.
    #[allow(dead_code)]
    pub fn get_tree_node(
        &mut self,
        root: Word,
        depth: BaseElement,
        index: BaseElement,
    ) -> Result<Word, ExecutionError> {
        // look up the advise set and return an error if none is found
        let advise_set = self
            .sets
            .get(&root.into_bytes())
            .ok_or_else(|| ExecutionError::AdviceSetNotFound(root.into_bytes()))?;

        // get the tree node from the advise set based on depth and index
        let node = advise_set
            .get_node(depth.as_int() as u32, index.as_int())
            .map_err(ExecutionError::AdviseSetLookupFailed)?;

        Ok(node)
    }

    /// Returns a path to a node at the specified index in a Merkle tree with the specified root.
    ///
    /// # Errors
    /// Returns an error if:
    /// - A Merkle tree for the specified root cannot be found in this advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree
    ///   identified by the specified root.
    /// - Path to the node at the specified depth and index is not known to this advice provider.
    pub fn get_merkle_path(
        &mut self,
        root: Word,
        depth: BaseElement,
        index: BaseElement,
    ) -> Result<Vec<Word>, ExecutionError> {
        // look up the advise set and return an error if none is found
        let advise_set = self
            .sets
            .get(&root.into_bytes())
            .ok_or_else(|| ExecutionError::AdviceSetNotFound(root.into_bytes()))?;

        // get the Merkle path from the advise set based on depth and index
        let path = advise_set
            .get_path(depth.as_int() as u32, index.as_int())
            .map_err(ExecutionError::AdviseSetLookupFailed)?;

        Ok(path)
    }

    // CONTEXT MANAGEMENT
    // --------------------------------------------------------------------------------------------

    /// Increments the clock cycle.
    pub fn advance_clock(&mut self) {
        self.step += 1;
    }
}
