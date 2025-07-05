use alloc::{collections::BTreeMap, vec::Vec};

use vm_core::{
    AdviceMap, Felt, Word,
    crypto::merkle::{MerklePath, MerkleStore, NodeIndex, StoreNode},
};

mod inputs;
pub use inputs::AdviceInputs;

mod errors;
pub use errors::AdviceError;

// TYPE ALIASES
// ================================================================================================

type SimpleMerkleMap = BTreeMap<Word, StoreNode>;

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
    pub map: AdviceMap,
    pub store: MerkleStore<SimpleMerkleMap>,
}

impl AdviceProvider {
    // ADVICE STACK
    // --------------------------------------------------------------------------------------------

    /// Pops an element from the advice stack and returns it.
    ///
    /// # Errors
    /// Returns an error if the advice stack is empty.
    pub fn pop_stack(&mut self) -> Result<Felt, AdviceError> {
        self.stack.pop().ok_or(AdviceError::StackReadFailed)
    }

    /// Pops a word (4 elements) from the advice stack and returns it.
    ///
    /// Note: a word is popped off the stack element-by-element. For example, a `[d, c, b, a, ...]`
    /// stack (i.e., `d` is at the top of the stack) will yield `[d, c, b, a]`.
    ///
    /// # Errors
    /// Returns an error if the advice stack does not contain a full word.
    pub fn pop_stack_word(&mut self) -> Result<Word, AdviceError> {
        if self.stack.len() < 4 {
            return Err(AdviceError::StackReadFailed);
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
    pub fn pop_stack_dword(&mut self) -> Result<[Word; 2], AdviceError> {
        let word0 = self.pop_stack_word()?;
        let word1 = self.pop_stack_word()?;

        Ok([word0, word1])
    }

    /// Pushes a single value onto the advice stack.
    pub fn push_stack(&mut self, value: Felt) {
        self.stack.push(value)
    }

    /// Pushes a word (4 elements) onto the stack.
    pub fn push_stack_word(&mut self, word: &Word) {
        self.stack.extend(word.iter().rev())
    }

    /// Fetches a list of elements under the specified key from the advice map and pushes them onto
    /// the advice stack.
    ///
    /// If `include_len` is set to true, this also pushes the number of elements onto the advice
    /// stack.
    ///
    /// Note: this operation doesn't consume the map element so it can be called multiple times
    /// for the same key.
    ///
    /// # Example
    /// Given an advice stack `[a, b, c, ...]`, and a map `x |-> [d, e, f]`:
    ///
    /// A call `push_stack(AdviceSource::Map { key: x, include_len: false })` will result in
    /// advice stack: `[d, e, f, a, b, c, ...]`.
    ///
    /// A call `push_stack(AdviceSource::Map { key: x, include_len: true })` will result in
    /// advice stack: `[3, d, e, f, a, b, c, ...]`.
    ///
    /// # Errors
    /// Returns an error if the key was not found in the key-value map.
    pub fn push_from_map(&mut self, key: Word, include_len: bool) -> Result<(), AdviceError> {
        let values = self.map.get(&key).ok_or(AdviceError::MapKeyNotFound { key })?;

        self.stack.extend(values.iter().rev());
        if include_len {
            self.stack
                .push(Felt::try_from(values.len() as u64).expect("value length too big"));
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
    pub fn get_mapped_values(&self, key: &Word) -> Result<&[Felt], AdviceError> {
        self.map.get(key).ok_or(AdviceError::MapKeyNotFound { key: *key })
    }

    /// Inserts the provided value into the advice map under the specified key.
    ///
    /// The values in the advice map can be moved onto the advice stack by invoking
    /// the [AdviceProvider::push_stack()] method.
    ///
    /// Returns an error if the specified key is already present in the advice map.
    pub fn insert_into_map(&mut self, key: Word, values: Vec<Felt>) {
        self.map.insert(key, values);
    }

    /// Merges all entries from the given [`AdviceMap`] into the current advice map.
    ///
    /// Returns an error if any new entry already exists with the same key but a different value
    /// than the one currently stored. The current map remains unchanged.
    pub fn merge_advice_map(&mut self, other: &AdviceMap) -> Result<(), AdviceError> {
        self.map.merge_advice_map(other).map_err(|((key, prev_values), new_values)| {
            AdviceError::MapKeyAlreadyPresent { key, prev_values, new_values }
        })
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
    ) -> Result<Word, AdviceError> {
        let index = NodeIndex::from_elements(depth, index).map_err(|_| {
            AdviceError::InvalidMerkleTreeNodeIndex { depth: *depth, index: *index }
        })?;
        self.store.get_node(root, index).map_err(AdviceError::MerkleStoreLookupFailed)
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
    ) -> Result<MerklePath, AdviceError> {
        let index = NodeIndex::from_elements(depth, index).map_err(|_| {
            AdviceError::InvalidMerkleTreeNodeIndex { depth: *depth, index: *index }
        })?;
        self.store
            .get_path(root, index)
            .map(|value| value.path)
            .map_err(AdviceError::MerkleStoreLookupFailed)
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
    ) -> Result<(MerklePath, Word), AdviceError> {
        let node_index = NodeIndex::from_elements(depth, index).map_err(|_| {
            AdviceError::InvalidMerkleTreeNodeIndex { depth: *depth, index: *index }
        })?;
        self.store
            .set_node(root, node_index, value)
            .map(|root| (root.path, root.root))
            .map_err(AdviceError::MerkleStoreUpdateFailed)
    }

    /// Creates a new Merkle tree in the advice provider by combining Merkle trees with the
    /// specified roots. The root of the new tree is defined as `hash(left_root, right_root)`.
    ///
    /// After the operation, both the original trees and the new tree remains in the advice
    /// provider (i.e., the input trees are not removed).
    ///
    /// It is not checked whether a Merkle tree for either of the specified roots can be found in
    /// this advice provider.
    pub fn merge_roots(&mut self, lhs: Word, rhs: Word) -> Result<Word, AdviceError> {
        self.store.merge_roots(lhs, rhs).map_err(AdviceError::MerkleStoreMergeFailed)
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
        Self { stack, map, store }
    }
}
