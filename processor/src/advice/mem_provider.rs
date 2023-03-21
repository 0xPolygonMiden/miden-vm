use super::{
    AdviceInputs, AdviceProvider, AdviceSource, BTreeMap, ExecutionError, Felt, IntoBytes,
    MerklePath, MerkleSet, NodeIndex, StarkField, Vec, Word,
};

// MEMORY ADVICE PROVIDER
// ================================================================================================

/// An in-memory `[AdviceProvider]` implementation to support program execution.
///
/// Uses `[BTreeMap]` as backend.
#[derive(Debug, Clone, Default)]
pub struct MemAdviceProvider {
    step: u32,
    stack: Vec<Felt>,
    map: BTreeMap<[u8; 32], Vec<Felt>>,
    sets: BTreeMap<[u8; 32], MerkleSet>,
}

impl From<AdviceInputs> for MemAdviceProvider {
    fn from(inputs: AdviceInputs) -> Self {
        let (mut stack, map, sets) = inputs.into_parts();
        stack.reverse();
        Self {
            step: 0,
            stack,
            map,
            sets,
        }
    }
}

impl AdviceProvider for MemAdviceProvider {
    // ADVICE STACK
    // --------------------------------------------------------------------------------------------

    fn pop_stack(&mut self) -> Result<Felt, ExecutionError> {
        self.stack.pop().ok_or(ExecutionError::AdviceStackReadFailed(self.step))
    }

    fn pop_stack_word(&mut self) -> Result<Word, ExecutionError> {
        if self.stack.len() < 4 {
            return Err(ExecutionError::AdviceStackReadFailed(self.step));
        }

        let idx = self.stack.len() - 4;
        let result =
            [self.stack[idx + 3], self.stack[idx + 2], self.stack[idx + 1], self.stack[idx]];

        self.stack.truncate(idx);

        Ok(result)
    }

    fn pop_stack_dword(&mut self) -> Result<[Word; 2], ExecutionError> {
        let word0 = self.pop_stack_word()?;
        let word1 = self.pop_stack_word()?;

        Ok([word0, word1])
    }

    fn push_stack(&mut self, source: AdviceSource) -> Result<(), ExecutionError> {
        match source {
            AdviceSource::Value(value) => {
                self.stack.push(value);
                Ok(())
            }

            AdviceSource::Map { key } => {
                let map = self
                    .map
                    .get(&key.into_bytes())
                    .ok_or(ExecutionError::AdviceKeyNotFound(key))?;
                self.stack.extend(map.iter().rev());
                Ok(())
            }
        }
    }

    fn insert_into_map(&mut self, key: Word, values: Vec<Felt>) -> Result<(), ExecutionError> {
        match self.map.insert(key.into_bytes(), values) {
            None => Ok(()),
            Some(_) => Err(ExecutionError::DuplicateAdviceKey(key)),
        }
    }

    // ADVISE SETS
    // --------------------------------------------------------------------------------------------

    fn get_tree_node(
        &self,
        root: Word,
        depth: &Felt,
        index: &Felt,
    ) -> Result<Word, ExecutionError> {
        // look up the merkle set and return an error if none is found
        let merkle_set = self
            .sets
            .get(&root.into_bytes())
            .ok_or_else(|| ExecutionError::MerkleSetNotFound(root.into_bytes()))?;

        // get the tree node from the merkle set based on depth and index
        let index = NodeIndex::from_elements(depth, index)
            .map_err(ExecutionError::MerkleSetLookupFailed)?;
        let node = merkle_set.get_node(index).map_err(ExecutionError::MerkleSetLookupFailed)?;

        Ok(node)
    }

    fn get_merkle_path(
        &self,
        root: Word,
        depth: &Felt,
        index: &Felt,
    ) -> Result<MerklePath, ExecutionError> {
        // look up the merkle set and return an error if none is found
        let merkle_set = self
            .sets
            .get(&root.into_bytes())
            .ok_or_else(|| ExecutionError::MerkleSetNotFound(root.into_bytes()))?;

        // get the Merkle path from the merkle set based on depth and index
        let index = NodeIndex::from_elements(depth, index)
            .map_err(ExecutionError::MerkleSetLookupFailed)?;
        let path = merkle_set.get_path(index).map_err(ExecutionError::MerkleSetLookupFailed)?;

        Ok(path)
    }

    fn update_merkle_leaf(
        &mut self,
        root: Word,
        index: Felt,
        leaf_value: Word,
        update_in_copy: bool,
    ) -> Result<MerklePath, ExecutionError> {
        // look up the merkle set and return error if none is found. if we are updating a copy,
        // clone the merkle set; otherwise remove it from the map because the root will change,
        // and we'll re-insert the set later under a different root.
        let mut merkle_set = if update_in_copy {
            // look up the merkle set and return an error if none is found
            self.sets
                .get(&root.into_bytes())
                .ok_or_else(|| ExecutionError::MerkleSetNotFound(root.into_bytes()))?
                .clone()
        } else {
            self.sets
                .remove(&root.into_bytes())
                .ok_or_else(|| ExecutionError::MerkleSetNotFound(root.into_bytes()))?
        };

        // get the Merkle path from the merkle set for the leaf at the specified index
        let index = NodeIndex::new(merkle_set.depth(), index.as_int());
        let path = merkle_set.get_path(index).map_err(ExecutionError::MerkleSetLookupFailed)?;

        // update the merkle set and re-insert it into the map
        merkle_set
            .update_leaf(index.value(), leaf_value)
            .map_err(ExecutionError::MerkleSetLookupFailed)?;
        self.sets.insert(merkle_set.root().into_bytes(), merkle_set);

        Ok(path)
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

    /// Returns true if the merkle set with the specified root is present in this advice provider.
    pub fn has_merkle_set(&self, root: Word) -> bool {
        self.sets.contains_key(&root.into_bytes())
    }
}
