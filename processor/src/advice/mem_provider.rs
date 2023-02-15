use vm_core::Insertion;

use super::{
    AdviceInputs, AdviceProvider, AdviceSet, AdviceSource, BTreeMap, ExecutionError, Felt,
    IntoBytes, StarkField, Vec, Word,
};


// MEMORY ADVICE PROVIDER
// ================================================================================================

/// An in-memory `[AdviceProvider]` implementation to support program execution.
///
/// Uses `[BTreeMap]` as backend.
#[derive(Debug, Clone, Default)]
pub struct MemAdviceProvider {
    step: u32,
    tape: Vec<Felt>,
    values: BTreeMap<[u8; 32], Vec<Felt>>,
    sets: BTreeMap<[u8; 32], AdviceSet>,
}

impl From<AdviceInputs> for MemAdviceProvider {
    fn from(inputs: AdviceInputs) -> Self {
        let (mut tape, values, sets) = inputs.into_parts();
        tape.reverse();
        Self {
            step: 0,
            tape,
            values,
            sets,
        }
    }
}

impl AdviceProvider for MemAdviceProvider {
    // ADVICE TAPE
    // --------------------------------------------------------------------------------------------

    fn read_tape(&mut self) -> Result<Felt, ExecutionError> {
        self.tape.pop().ok_or(ExecutionError::AdviceTapeReadFailed(self.step))
    }

    fn read_tape_w(&mut self) -> Result<Word, ExecutionError> {
        if self.tape.len() < 4 {
            return Err(ExecutionError::AdviceTapeReadFailed(self.step));
        }

        let idx = self.tape.len() - 4;
        let result = [self.tape[idx + 3], self.tape[idx + 2], self.tape[idx + 1], self.tape[idx]];

        self.tape.truncate(idx);

        Ok(result)
    }

    fn read_tape_dw(&mut self) -> Result<[Word; 2], ExecutionError> {
        let word0 = self.read_tape_w()?;
        let word1 = self.read_tape_w()?;

        Ok([word0, word1])
    }

    fn write_tape(&mut self, source: AdviceSource) -> Result<(), ExecutionError> {
        match source {
            AdviceSource::Value(value) => {
                self.tape.push(value);
                Ok(())
            }

            AdviceSource::Map { key } => {
                let values = self
                    .values
                    .get(&key.into_bytes())
                    .ok_or(ExecutionError::AdviceKeyNotFound(key))?;
                self.tape.extend(values.iter().rev());
                Ok(())
            }
        }
    }

    fn insert_into_map(&mut self, key: Word, values: Vec<Felt>) -> Result<(), ExecutionError> {
        match self.values.insert(key.into_bytes(), values) {
            None => Ok(()),
            Some(_) => Err(ExecutionError::DuplicateAdviceKey(key)),
        }
    }

    // ADVISE SETS
    // --------------------------------------------------------------------------------------------

    fn get_tree_node(&self, root: Word, depth: Felt, index: Felt) -> Result<Word, ExecutionError> {
        // look up the advice set and return an error if none is found
        let advice_set = self
            .sets
            .get(&root.into_bytes())
            .ok_or_else(|| ExecutionError::AdviceSetNotFound(root.into_bytes()))?;

        // get the tree node from the advice set based on depth and index
        let node = advice_set
            .get_node(depth.as_int() as u32, index.as_int())
            .map_err(ExecutionError::AdviceSetLookupFailed)?;

        Ok(node)
    }

    fn get_merkle_path(
        &self,
        root: Word,
        depth: Felt,
        index: Felt,
    ) -> Result<Vec<Word>, ExecutionError> {
        // look up the advice set and return an error if none is found
        let advice_set = self
            .sets
            .get(&root.into_bytes())
            .ok_or_else(|| ExecutionError::AdviceSetNotFound(root.into_bytes()))?;

        // get the Merkle path from the advice set based on depth and index
        let path = advice_set
            .get_path(depth.as_int() as u32, index.as_int())
            .map_err(ExecutionError::AdviceSetLookupFailed)?;

        Ok(path)
    }

    fn update_merkle_leaf(
        &mut self,
        root: Word,
        index: Felt,
        leaf_value: Word,
        update_in_copy: bool,
    ) -> Result<Vec<Word>, ExecutionError> {
        // look up the advice set and return error if none is found. if we are updating a copy,
        // clone the advice set; otherwise remove it from the map because the root will change,
        // and we'll re-insert the set later under a different root.
        let mut advice_set = if update_in_copy {
            // look up the advice set and return an error if none is found
            self.sets
                .get(&root.into_bytes())
                .ok_or_else(|| ExecutionError::AdviceSetNotFound(root.into_bytes()))?
                .clone()
        } else {
            self.sets
                .remove(&root.into_bytes())
                .ok_or_else(|| ExecutionError::AdviceSetNotFound(root.into_bytes()))?
        };

        // get the Merkle path from the advice set for the leaf at the specified index
        let path = advice_set
            .get_path(advice_set.depth(), index.as_int())
            .map_err(ExecutionError::AdviceSetLookupFailed)?;

        // update the advice set and re-insert it into the map
        advice_set
            .update_leaf(index.as_int(), leaf_value)
            .map_err(ExecutionError::AdviceSetLookupFailed)?;
        println!("advice_set.root().into_bytes() is {:?}",advice_set.root().into_bytes());
        self.sets.insert(advice_set.root().into_bytes(), advice_set);

        Ok(path)
    }

    fn set_smt_depth(&mut self, root: Word, depth: u32) -> Result<(), ExecutionError> {
        println!("root at set_smt_depth is {:?}", root.into_bytes());
        //println!("sets are {:?}", self.sets);
        let set = self.sets.get_mut(&root.into_bytes()).unwrap();
        match set {
            AdviceSet::TieredSMT(ref mut tree) => {
                tree.set_depth(depth);
                Ok(())
            }
            _ => Err(ExecutionError::CallerNotInSyscall),
        }
    }

    fn pre_insert_tiered_smt(&mut self, root: Word, key: Word, value: Word) -> Result<Insertion, ExecutionError> {
        let tiered_smt = self.sets.get_mut(&root.into_bytes()).unwrap();
        match tiered_smt{
            AdviceSet::TieredSMT(ref mut tiered_smt) => {
                let insertion = tiered_smt.pre_insert(key, value).unwrap();
                match insertion {
                    Insertion::Simple { index, depth } => tiered_smt.set_depth(depth),
                    Insertion::Complex { index, depth, index0, depth0, index1, depth1, key, value } => tiered_smt.set_depth(depth), 
                }
                
                Ok(insertion)
            }
            _ => Err(ExecutionError::CallerNotInSyscall)
        }
    }

    // CONTEXT MANAGEMENT
    // --------------------------------------------------------------------------------------------

    fn advance_clock(&mut self) {
        self.step += 1;
    }
}

#[cfg(test)]
impl MemAdviceProvider {
    // ADVISE SETS
    // --------------------------------------------------------------------------------------------

    /// Returns true if the advice set with the specified root is present in this advice provider.
    pub fn has_advice_set(&self, root: Word) -> bool {
        self.sets.contains_key(&root.into_bytes())
    }
}
