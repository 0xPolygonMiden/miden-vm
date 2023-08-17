use super::{ExecutionError, Felt, InputError, StarkField, Word};
use core::borrow::Borrow;
use vm_core::{
    crypto::{
        hash::RpoDigest,
        merkle::{InnerNodeInfo, MerklePath, MerkleStore, NodeIndex, StoreNode},
    },
    utils::{
        collections::{BTreeMap, KvMap, RecordingMap, Vec},
        IntoBytes,
    },
};

mod inputs;
pub use inputs::AdviceInputs;

mod providers;
pub use providers::{MemAdviceProvider, RecAdviceProvider};

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

    // ADVICE MAP
    // --------------------------------------------------------------------------------------------

    /// Returns a reference to the value(s) associated with the specified key in the advice map.
    fn get_mapped_values(&self, key: &[u8; 32]) -> Option<&[Felt]>;

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
    /// returns the Merkle path from the updated node to the new root, together with the new root.
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
    ) -> Result<(MerklePath, Word), ExecutionError>;

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

    /// Returns a subset of this Merkle store such that the returned Merkle store contains all
    /// nodes which are descendants of the specified roots.
    ///
    /// The roots for which no descendants exist in this Merkle store are ignored.
    fn get_store_subset<I, R>(&self, roots: I) -> MerkleStore
    where
        I: Iterator<Item = R>,
        R: Borrow<RpoDigest>;

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

impl<'a, T> AdviceProvider for &'a mut T
where
    T: AdviceProvider,
{
    fn pop_stack(&mut self) -> Result<Felt, ExecutionError> {
        T::pop_stack(self)
    }

    fn pop_stack_word(&mut self) -> Result<Word, ExecutionError> {
        T::pop_stack_word(self)
    }

    fn pop_stack_dword(&mut self) -> Result<[Word; 2], ExecutionError> {
        T::pop_stack_dword(self)
    }

    fn push_stack(&mut self, source: AdviceSource) -> Result<(), ExecutionError> {
        T::push_stack(self, source)
    }

    fn insert_into_map(&mut self, key: Word, values: Vec<Felt>) -> Result<(), ExecutionError> {
        T::insert_into_map(self, key, values)
    }

    fn get_mapped_values(&self, key: &[u8; 32]) -> Option<&[Felt]> {
        T::get_mapped_values(self, key)
    }

    fn get_tree_node(
        &self,
        root: Word,
        depth: &Felt,
        index: &Felt,
    ) -> Result<Word, ExecutionError> {
        T::get_tree_node(self, root, depth, index)
    }

    fn get_merkle_path(
        &self,
        root: Word,
        depth: &Felt,
        index: &Felt,
    ) -> Result<MerklePath, ExecutionError> {
        T::get_merkle_path(self, root, depth, index)
    }

    fn get_leaf_depth(
        &self,
        root: Word,
        tree_depth: &Felt,
        index: &Felt,
    ) -> Result<u8, ExecutionError> {
        T::get_leaf_depth(self, root, tree_depth, index)
    }

    fn update_merkle_node(
        &mut self,
        root: Word,
        depth: &Felt,
        index: &Felt,
        value: Word,
    ) -> Result<(MerklePath, Word), ExecutionError> {
        T::update_merkle_node(self, root, depth, index, value)
    }

    fn merge_roots(&mut self, lhs: Word, rhs: Word) -> Result<Word, ExecutionError> {
        T::merge_roots(self, lhs, rhs)
    }

    fn get_store_subset<I, R>(&self, roots: I) -> MerkleStore
    where
        I: Iterator<Item = R>,
        R: Borrow<RpoDigest>,
    {
        T::get_store_subset(self, roots)
    }

    fn advance_clock(&mut self) {
        T::advance_clock(self)
    }
}
