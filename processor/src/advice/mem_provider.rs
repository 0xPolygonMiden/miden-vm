use super::{
    AdviceInputs, AdviceProvider, AdviceSource, BTreeMap, ExecutionError, Felt, IntoBytes,
    MerklePath, MerkleStore, NodeIndex, Vec, Word,
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
    store: MerkleStore,
}

impl From<AdviceInputs> for MemAdviceProvider {
    fn from(inputs: AdviceInputs) -> Self {
        let (mut stack, map, store) = inputs.into_parts();
        stack.reverse();
        Self {
            step: 0,
            stack,
            map,
            store,
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
        let index = NodeIndex::from_elements(depth, index).map_err(|_| {
            ExecutionError::InvalidNodeIndex {
                depth: *depth,
                value: *index,
            }
        })?;
        self.store
            .get_node(root, index)
            .map_err(ExecutionError::MerkleStoreLookupFailed)
    }

    fn get_merkle_path(
        &self,
        root: Word,
        depth: &Felt,
        index: &Felt,
    ) -> Result<MerklePath, ExecutionError> {
        let index = NodeIndex::from_elements(depth, index).map_err(|_| {
            ExecutionError::InvalidNodeIndex {
                depth: *depth,
                value: *index,
            }
        })?;
        self.store
            .get_path(root, index)
            .map(|value| value.path)
            .map_err(ExecutionError::MerkleStoreLookupFailed)
    }

    fn update_merkle_node(
        &mut self,
        root: Word,
        depth: &Felt,
        index: &Felt,
        value: Word,
    ) -> Result<MerklePath, ExecutionError> {
        let node_index = NodeIndex::from_elements(depth, index).map_err(|_| {
            ExecutionError::InvalidNodeIndex {
                depth: *depth,
                value: *index,
            }
        })?;
        self.store
            .set_node(root, node_index, value)
            .map(|root| root.path)
            .map_err(ExecutionError::MerkleStoreUpdateFailed)
    }

    fn merge_roots(&mut self, lhs: Word, rhs: Word) -> Result<Word, ExecutionError> {
        self.store.merge_roots(lhs, rhs).map_err(ExecutionError::MerkleStoreMergeFailed)
    }

    // CONTEXT MANAGEMENT
    // --------------------------------------------------------------------------------------------

    fn advance_clock(&mut self) {
        self.step += 1;
    }
}

impl MemAdviceProvider {
    // ADVISE SETS TEST HELPERS
    // --------------------------------------------------------------------------------------------

    /// Returns true if the Merkle root exists for the advice provider Merkle store.
    #[cfg(test)]
    pub fn has_merkle_root(&self, root: Word) -> bool {
        self.store.get_node(root, NodeIndex::root()).is_ok()
    }
}
