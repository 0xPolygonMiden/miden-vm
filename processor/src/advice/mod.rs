use super::{ExecutionError, Felt, InputError, StarkField, Word};
use vm_core::{
    crypto::data::{KvMap, RecordingMap},
    crypto::merkle::{
        GenericMerkleStore, InnerNodeInfo, MerkleMap, MerkleMapT, MerklePath, MerkleStore,
        NodeIndex, RecordingMerkleMap, RecordingMerkleStore,
    },
    utils::{
        collections::{BTreeMap, Vec},
        IntoBytes,
    },
};

mod inputs;
pub use inputs::AdviceInputs;

mod mem_provider;
pub use mem_provider::MemAdviceProvider;

mod recorder;
pub use recorder::RecAdviceProvider;

mod source;
pub use source::AdviceSource;

// ADVICE PROVIDER
// ================================================================================================

/// Defines behavior of an advice provider.
///
/// An advice provider is a component through which the VM processor can interact with the host
/// environment. The processor can request nondeterministic inputs from the advice provider (i.e.,
/// result of a computation performed outside of the VM), as well as insert new data into the
/// advice provider.
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
pub trait AdviceProvider {
    // ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Creates a "by reference" advice provider for this instance.
    ///
    /// The returned adapter also implements [AdviceProvider] and will simply mutably borrow this
    /// instance.
    fn by_ref(&mut self) -> &mut Self {
        // this trait follows the same model as
        // [io::Read](https://doc.rust-lang.org/std/io/trait.Read.html#method.by_ref).
        //
        // this approach allows the flexibility to take an advice provider either as owned or by
        // mutable reference - both equally compatible with the trait requirements as we implement
        // `AdviceProvider` for mutable references of any type that also implements advice
        // provider.
        self
    }

    // ADVICE STACK
    // --------------------------------------------------------------------------------------------

    /// Pops an element from the advice stack and returns it.
    ///
    /// # Errors
    /// Returns an error if the advice stack is empty.
    fn pop_stack(&mut self) -> Result<Felt, ExecutionError>;

    /// Pops a word (4 elements) from the advice stack and returns it.
    ///
    /// Note: a word is popped off the stack element-by-element. For example, a `[d, c, b, a, ...]`
    /// stack (i.e., `d` is at the top of the stack) will yield `[d, c, b, a]`.
    ///
    /// # Errors
    /// Returns an error if the advice stack does not contain a full word.
    fn pop_stack_word(&mut self) -> Result<Word, ExecutionError>;

    /// Pops a double word (8 elements) from the advice stack and returns them.
    ///
    /// Note: words are popped off the stack element-by-element. For example, a
    /// `[h, g, f, e, d, c, b, a, ...]` stack (i.e., `h` is at the top of the stack) will yield
    /// two words: `[h, g, f,e ], [d, c, b, a]`.
    ///
    /// # Errors
    /// Returns an error if the advice stack does not contain two words.
    fn pop_stack_dword(&mut self) -> Result<[Word; 2], ExecutionError>;

    /// Pushes the value(s) specified by the source onto the advice stack.
    ///
    /// # Errors
    /// Returns an error if the value specified by the advice source cannot be obtained.
    fn push_stack(&mut self, source: AdviceSource) -> Result<(), ExecutionError>;

    /// Inserts the provided value into the advice map under the specified key.
    ///
    /// The values in the advice map can be moved onto the advice stack by invoking
    /// [AdviceProvider::push_stack()] method.
    ///
    /// If the specified key is already present in the advice map, the values under the key
    /// are replaced with the specified values.
    fn insert_into_map(&mut self, key: Word, values: Vec<Felt>) -> Result<(), ExecutionError>;

    // ADVISE SETS
    // --------------------------------------------------------------------------------------------

    /// Returns a node at the specified depth and index in a Merkle tree with the given root.
    ///
    /// # Errors
    /// Returns an error if:
    /// - A Merkle tree for the specified root cannot be found in this advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree
    ///   identified by the specified root.
    /// - Value of the node at the specified depth and index is not known to this advice provider.
    fn get_tree_node(&self, root: Word, depth: &Felt, index: &Felt)
        -> Result<Word, ExecutionError>;

    /// Returns a path to a node at the specified depth and index in a Merkle tree with the
    /// specified root.
    ///
    /// # Errors
    /// Returns an error if:
    /// - A Merkle tree for the specified root cannot be found in this advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree
    ///   identified by the specified root.
    /// - Path to the node at the specified depth and index is not known to this advice provider.
    fn get_merkle_path(
        &self,
        root: Word,
        depth: &Felt,
        index: &Felt,
    ) -> Result<MerklePath, ExecutionError>;

    /// Reconstructs a path from the root until a leaf or empty node and returns its depth.
    ///
    /// For more information, check [MerkleStore::get_leaf_depth].
    ///
    /// # Errors
    /// Will return an error if:
    /// - The provided `tree_depth` doesn't fit `u8`.
    /// - The conditions of [MerkleStore::get_leaf_depth] aren't met.
    fn get_leaf_depth(
        &self,
        root: Word,
        tree_depth: &Felt,
        index: &Felt,
    ) -> Result<u8, ExecutionError>;

    /// Updates a node at the specified depth and index in a Merkle tree with the specified root;
    /// returns the Merkle path from the updated node to the new root.
    ///
    /// The tree is cloned prior to the update. Thus, the advice provider retains the original and
    /// the updated tree.
    ///
    /// # Errors
    /// Returns an error if:
    /// - A Merkle tree for the specified root cannot be found in this advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree
    ///   identified by the specified root.
    /// - Path to the leaf at the specified index in the specified Merkle tree is not known to this
    ///   advice provider.
    fn update_merkle_node(
        &mut self,
        root: Word,
        depth: &Felt,
        index: &Felt,
        value: Word,
    ) -> Result<MerklePath, ExecutionError>;

    /// Creates a new Merkle tree in the advice provider by combining Merkle trees with the
    /// specified roots. The root of the new tree is defined as `hash(left_root, right_root)`.
    ///
    /// After the operation, both the original trees and the new tree remains in the advice
    /// provider (i.e., the input trees are not removed).
    ///
    /// # Errors
    /// Returns an error if a Merkle tree for either of the specified roots cannot be found in this
    /// advice provider.
    fn merge_roots(&mut self, lhs: Word, rhs: Word) -> Result<Word, ExecutionError>;

    // CONTEXT MANAGEMENT
    // --------------------------------------------------------------------------------------------

    /// Increments the clock cycle.
    ///
    /// This is used to keep the state of the VM in sync with the state of the advice provider, and
    /// should be incrementally updated when called.
    ///
    /// TODO: keeping track of the clock cycle is used primarily for attaching clock cycle to error
    /// messages generated by the advice provider; consider refactoring.
    fn advance_clock(&mut self);
}

// STACK / MAP / STORE PROVIDER
// ================================================================================================
/// A trait that defines the interface for an [AdviceProvider] that consists of a stack
/// (`Vec<Felt>`), a map (`KvMap<[u8; 32], Vec<Felt>>`) and a Merkle store
/// (`GenericMerkleStore<MerkleMapT>`).
pub trait StackMapStoreProvider {
    /// The type of the map used to store Merkle data.
    type Map: MerkleMapT;

    /// Returns the current execution step.
    fn get_step(&self) -> u32;

    /// Returns a mutable reference to the current execution step.
    fn get_step_mut(&mut self) -> &mut u32;

    /// Returns a reference to the stack.
    fn get_stack(&self) -> &[Felt];

    /// Returns a mutable reference to the stack.
    fn get_stack_mut(&mut self) -> &mut Vec<Felt>;

    /// Returns a reference to the map.
    fn get_map(&self) -> &dyn KvMap<[u8; 32], Vec<Felt>>;

    /// Returns a mutable reference to the map.
    fn get_map_mut(&mut self) -> &mut dyn KvMap<[u8; 32], Vec<Felt>>;

    /// Returns a reference to the store of.
    fn get_store(&self) -> &GenericMerkleStore<Self::Map>;

    /// Returns a mutable reference to the store.
    fn get_store_mut(&mut self) -> &mut GenericMerkleStore<Self::Map>;
}

/// Blanket implementation of [AdviceProvider] for types that implement the [StackMapStoreProvider]
/// trait.
impl<T> AdviceProvider for T
where
    T: StackMapStoreProvider,
{
    // ADVICE STACK
    // --------------------------------------------------------------------------------------------

    fn pop_stack(&mut self) -> Result<Felt, ExecutionError> {
        self.get_stack_mut()
            .pop()
            .ok_or(ExecutionError::AdviceStackReadFailed(self.get_step()))
    }

    fn pop_stack_word(&mut self) -> Result<Word, ExecutionError> {
        if self.get_stack().len() < 4 {
            return Err(ExecutionError::AdviceStackReadFailed(self.get_step()));
        }

        let idx = self.get_stack().len() - 4;
        let result = [
            self.get_stack()[idx + 3],
            self.get_stack()[idx + 2],
            self.get_stack()[idx + 1],
            self.get_stack()[idx],
        ];

        self.get_stack_mut().truncate(idx);

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
                self.get_stack_mut().push(value);
                Ok(())
            }

            AdviceSource::Map { key, include_len } => {
                let values = self
                    .get_map()
                    .get(&key.into_bytes())
                    .cloned()
                    .ok_or(ExecutionError::AdviceKeyNotFound(key))?;

                self.get_stack_mut().extend(values.iter().rev());
                if include_len {
                    self.get_stack_mut().push(Felt::from(values.len() as u64));
                }
                Ok(())
            }
        }
    }

    fn insert_into_map(&mut self, key: Word, values: Vec<Felt>) -> Result<(), ExecutionError> {
        self.get_map_mut().insert(key.into_bytes(), values);
        Ok(())
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
            ExecutionError::InvalidTreeNodeIndex {
                depth: *depth,
                value: *index,
            }
        })?;
        self.get_store()
            .get_node(root.into(), index)
            .map(|value| value.into())
            .map_err(ExecutionError::MerkleStoreLookupFailed)
    }

    fn get_merkle_path(
        &self,
        root: Word,
        depth: &Felt,
        index: &Felt,
    ) -> Result<MerklePath, ExecutionError> {
        let index = NodeIndex::from_elements(depth, index).map_err(|_| {
            ExecutionError::InvalidTreeNodeIndex {
                depth: *depth,
                value: *index,
            }
        })?;
        self.get_store()
            .get_path(root.into(), index)
            .map(|value| value.path)
            .map_err(ExecutionError::MerkleStoreLookupFailed)
    }

    fn get_leaf_depth(
        &self,
        root: Word,
        tree_depth: &Felt,
        index: &Felt,
    ) -> Result<u8, ExecutionError> {
        let tree_depth = u8::try_from(tree_depth.as_int())
            .map_err(|_| ExecutionError::InvalidTreeDepth { depth: *tree_depth })?;
        self.get_store()
            .get_leaf_depth(root.into(), tree_depth, index.as_int())
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
            ExecutionError::InvalidTreeNodeIndex {
                depth: *depth,
                value: *index,
            }
        })?;
        self.get_store_mut()
            .set_node(root.into(), node_index, value.into())
            .map(|root| root.path)
            .map_err(ExecutionError::MerkleStoreUpdateFailed)
    }

    fn merge_roots(&mut self, lhs: Word, rhs: Word) -> Result<Word, ExecutionError> {
        self.get_store_mut()
            .merge_roots(lhs.into(), rhs.into())
            .map(|value| value.into())
            .map_err(ExecutionError::MerkleStoreMergeFailed)
    }

    // CONTEXT MANAGEMENT
    // --------------------------------------------------------------------------------------------

    fn advance_clock(&mut self) {
        *self.get_step_mut() += 1;
    }
}
