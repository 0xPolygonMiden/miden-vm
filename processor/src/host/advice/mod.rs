use alloc::{collections::BTreeMap, vec::Vec};

use vm_core::{
    Felt,
    crypto::merkle::{MerklePath, MerkleStore, NodeIndex, StoreNode},
};

use crate::{ErrorContext, ExecutionError, ProcessState, Word};

mod inputs;
pub use inputs::AdviceInputs;

mod source;
pub use source::AdviceSource;

// TYPE ALIASES
// ================================================================================================

type SimpleMerkleMap = BTreeMap<Word, StoreNode>;
type SimpleAdviceMap = BTreeMap<Word, Vec<Felt>>;

// ADVICE PROVIDER
// ================================================================================================

/// An advice provider is a component through which the host can interact with the advice provider.
/// The host can request nondeterministic inputs from the advice provider (i.e., result of a
/// computation performed outside of the VM), as well as insert new data into the advice provider.
///
/// An advice provider consists of the following components:
/// 1. Advice stack, which is a LIFO data structure. The processor can move the elements from the
///    advice stack onto the operand stack, as well as push new elements onto the advice stack.
/// 2. Advice map, which is a key-value map where keys are words (4 field elements) and values are
///    vectors of field elements. The processor can push the values from the map onto the advice
///    stack, as well as insert new values into the map.
/// 3. Merkle store, which contains structured data reducible to Merkle paths. The VM can request
///    Merkle paths from the store, as well as mutate it by updating or merging nodes contained in
///    the store.
///
/// Advice data is store in-memory using [BTreeMap]s as its backing storage.
#[derive(Debug, Clone, Default)]
pub struct AdviceProvider {
    pub stack: Vec<Felt>,
    pub map: SimpleAdviceMap,
    pub store: MerkleStore<SimpleMerkleMap>,
}

impl AdviceProvider {
    // ADVICE STACK
    // --------------------------------------------------------------------------------------------

    /// Pops an element from the advice stack and returns it.
    ///
    /// # Errors
    /// Returns an error if the advice stack is empty.
    pub fn pop_stack(
        &mut self,
        process: ProcessState,
        err_ctx: &impl ErrorContext,
    ) -> Result<Felt, ExecutionError> {
        self.stack
            .pop()
            .ok_or(ExecutionError::advice_stack_read_failed(process.clk(), err_ctx))
    }

    /// Pops a word (4 elements) from the advice stack and returns it.
    ///
    /// Note: a word is popped off the stack element-by-element. For example, a `[d, c, b, a, ...]`
    /// stack (i.e., `d` is at the top of the stack) will yield `[d, c, b, a]`.
    ///
    /// # Errors
    /// Returns an error if the advice stack does not contain a full word.
    pub fn pop_stack_word(
        &mut self,
        process: ProcessState,
        err_ctx: &impl ErrorContext,
    ) -> Result<Word, ExecutionError> {
        if self.stack.len() < 4 {
            return Err(ExecutionError::advice_stack_read_failed(process.clk(), err_ctx));
        }

        let idx = self.stack.len() - 4;
        let result =
            [self.stack[idx + 3], self.stack[idx + 2], self.stack[idx + 1], self.stack[idx]];

        self.stack.truncate(idx);

        Ok(result.into())
    }

    /// Pops a double word (8 elements) from the advice stack and returns them.
    ///
    /// Note: words are popped off the stack element-by-element. For example, a
    /// `[h, g, f, e, d, c, b, a, ...]` stack (i.e., `h` is at the top of the stack) will yield
    /// two words: `[h, g, f,e ], [d, c, b, a]`.
    ///
    /// # Errors
    /// Returns an error if the advice stack does not contain two words.
    pub fn pop_stack_dword(
        &mut self,
        process: ProcessState,
        err_ctx: &impl ErrorContext,
    ) -> Result<[Word; 2], ExecutionError> {
        let word0 = self.pop_stack_word(process, err_ctx)?;
        let word1 = self.pop_stack_word(process, err_ctx)?;

        Ok([word0, word1])
    }

    /// Pushes the value(s) specified by the source onto the advice stack.
    ///
    /// # Errors
    /// Returns an error if the value specified by the advice source cannot be obtained.
    pub fn push_stack(
        &mut self,
        source: AdviceSource,
        err_ctx: &impl ErrorContext,
    ) -> Result<(), ExecutionError> {
        match source {
            AdviceSource::Value(value) => {
                self.stack.push(value);
            },
            AdviceSource::Word(word) => {
                self.stack.extend(word.iter().rev());
            },
            AdviceSource::Map { key, include_len } => {
                let values = self
                    .map
                    .get(&key)
                    .ok_or(ExecutionError::advice_map_key_not_found(key, err_ctx))?;

                self.stack.extend(values.iter().rev());
                if include_len {
                    self.stack
                        .push(Felt::try_from(values.len() as u64).expect("value length too big"));
                }
            },
        }

        Ok(())
    }

    /// Returns a slice of length `length` from the top of the advice stack.
    /// If length = 0 returns the whole advice stack.
    pub fn peek_stack(&self, length: usize) -> &[Felt] {
        if length == 0 {
            &self.stack
        } else {
            &self.stack[0..length]
        }
    }

    // ADVICE MAP
    // --------------------------------------------------------------------------------------------

    /// Returns a reference to the value(s) associated with the specified key in the advice map.
    pub fn get_mapped_values(&self, key: &Word) -> Option<&[Felt]> {
        self.map.get(key).map(|v| v.as_slice())
    }

    /// Inserts the provided value into the advice map under the specified key.
    ///
    /// The values in the advice map can be moved onto the advice stack by invoking
    /// [AdviceProvider::push_stack()] method.
    ///
    /// If the specified key is already present in the advice map, the values under the key
    /// are replaced with the specified values.
    pub fn insert_into_map(&mut self, key: Word, values: Vec<Felt>) {
        self.map.insert(key, values);
    }

    // MERKLE STORE
    // --------------------------------------------------------------------------------------------

    /// Returns a node at the specified depth and index in a Merkle tree with the given root.
    ///
    /// # Errors
    /// Returns an error if:
    /// - A Merkle tree for the specified root cannot be found in this advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree identified
    ///   by the specified root.
    /// - Value of the node at the specified depth and index is not known to this advice provider.
    pub fn get_tree_node(
        &self,
        root: Word,
        depth: &Felt,
        index: &Felt,
        err_ctx: &impl ErrorContext,
    ) -> Result<Word, ExecutionError> {
        let index = NodeIndex::from_elements(depth, index)
            .map_err(|_| ExecutionError::invalid_merkle_tree_node_index(*depth, *index, err_ctx))?;
        self.store
            .get_node(root, index)
            .map_err(|err| ExecutionError::merkle_store_lookup_failed(err, err_ctx))
    }

    /// Returns a path to a node at the specified depth and index in a Merkle tree with the
    /// specified root.
    ///
    /// # Errors
    /// Returns an error if:
    /// - A Merkle tree for the specified root cannot be found in this advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree identified
    ///   by the specified root.
    /// - Path to the node at the specified depth and index is not known to this advice provider.
    pub fn get_merkle_path(
        &self,
        root: Word,
        depth: &Felt,
        index: &Felt,
        err_ctx: &impl ErrorContext,
    ) -> Result<MerklePath, ExecutionError> {
        let index = NodeIndex::from_elements(depth, index)
            .map_err(|_| ExecutionError::invalid_merkle_tree_node_index(*depth, *index, err_ctx))?;
        self.store
            .get_path(root, index)
            .map(|value| value.path)
            .map_err(|err| ExecutionError::merkle_store_lookup_failed(err, err_ctx))
    }

    /// Updates a node at the specified depth and index in a Merkle tree with the specified root;
    /// returns the Merkle path from the updated node to the new root, together with the new root.
    ///
    /// The tree is cloned prior to the update. Thus, the advice provider retains the original and
    /// the updated tree.
    ///
    /// # Errors
    /// Returns an error if:
    /// - A Merkle tree for the specified root cannot be found in this advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree identified
    ///   by the specified root.
    /// - Path to the leaf at the specified index in the specified Merkle tree is not known to this
    ///   advice provider.
    pub fn update_merkle_node(
        &mut self,
        root: Word,
        depth: &Felt,
        index: &Felt,
        value: Word,
        err_ctx: &impl ErrorContext,
    ) -> Result<(MerklePath, Word), ExecutionError> {
        let node_index = NodeIndex::from_elements(depth, index)
            .map_err(|_| ExecutionError::invalid_merkle_tree_node_index(*depth, *index, err_ctx))?;
        self.store
            .set_node(root, node_index, value)
            .map(|root| (root.path, root.root))
            .map_err(|err| ExecutionError::merkle_store_update_failed(err, err_ctx))
    }

    /// Creates a new Merkle tree in the advice provider by combining Merkle trees with the
    /// specified roots. The root of the new tree is defined as `hash(left_root, right_root)`.
    ///
    /// After the operation, both the original trees and the new tree remains in the advice
    /// provider (i.e., the input trees are not removed).
    ///
    /// It is not checked whether a Merkle tree for either of the specified roots can be found in
    /// this advice provider.
    pub fn merge_roots(
        &mut self,
        lhs: Word,
        rhs: Word,
        err_ctx: &impl ErrorContext,
    ) -> Result<Word, ExecutionError> {
        self.store
            .merge_roots(lhs, rhs)
            .map_err(|err| ExecutionError::merkle_store_merge_failed(err, err_ctx))
    }

    /// Returns true if the Merkle root exists for the advice provider Merkle store.
    pub fn has_merkle_root(&self, root: Word) -> bool {
        self.store.get_node(root, NodeIndex::root()).is_ok()
    }
}

impl From<AdviceInputs> for AdviceProvider {
    fn from(inputs: AdviceInputs) -> Self {
        let (mut stack, map, store) = inputs.into_parts();
        stack.reverse();
        Self {
            stack,
            map: map.into_iter().collect(),
            store: store.inner_nodes().collect(),
        }
    }
}
