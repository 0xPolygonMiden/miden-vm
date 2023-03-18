use super::{ExecutionError, Felt, InputError, Word};
use vm_core::{
    crypto::merkle::{MerklePath, MerkleStore, NodeIndex},
    utils::{
        collections::{BTreeMap, Vec},
        IntoBytes,
    },
};

mod inputs;
pub use inputs::AdviceInputs;

mod mem_provider;
pub use mem_provider::MemAdviceProvider;

mod source;
pub use source::AdviceSource;

// ADVICE PROVIDER
// ================================================================================================

// TODO elaborate on why this is non-deterministic (seems it is a simple state machine).

// TODO the `advance_clock` seems to be an internal of the VM and shouldn't necessarily be here.

// TODO this will likely suffer a breaking change as we introduce a generic storage for big Merkle
// sets.
// The purpose of this clock is to keep the feedback of the provided in sync with the execution
// trace of the VM, but on the other hand the VM can control the calls/traces itself, without the
// need for the advice provider (or any other external component) to keep track of it. If we
// delegate this control to the advice provider - especially if it is a trait, the user might
// implement it incorrectly, creating undefined behavior on the VM side (that expects the counter
// to incrementally increase).

/// Common behavior of advice providers for program execution.
///
/// An advice provider supplies non-deterministic inputs to the processor.
///
/// 1. Provide a stack functionality that yields elements as a stack (last in, first out). These can
///    be yielded as elements, words or double words.
/// 2. Provide a map functionality that will store temporary stacks that can be appended to the main
///    stack. This operation should not allow key overwrite; that is: if a given key exists, the
///    implementation should error if the user attempts to insert this key again, instead of the
///    common behavior of the maps to simply override the previous contents. This is a design
///    decision to increase the runtime robustness of the execution.
/// 3. Provide merkle tree interfaces, backed by a [MerkleStore].
pub trait AdviceProvider {
    // ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Creates a "by reference" advice provider for this instance.
    ///
    /// The returned adapter also implements `AdviceProvider` and will simply mutably borrow this
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
    /// Note: a word is always stored as little-endian. A `[...,a,b,c,d]` stack will yield
    /// `[d,c,b,a]`.
    ///
    /// # Errors
    /// Returns an error if the advice stack does not contain a full word.
    fn pop_stack_word(&mut self) -> Result<Word, ExecutionError>;

    /// Pops a double word (8 elements) from the advice stack and returns them.
    ///
    /// Note: a double word is always stored as little-endian. A `[...,a,b,c,d,e,f,g,h]` stack will
    /// yield `[h,g,f,e],[,d,c,b,a]`.
    ///
    /// # Errors
    /// Returns an error if the advice stack does not contain two words.
    fn pop_stack_dword(&mut self) -> Result<[Word; 2], ExecutionError>;

    /// Writes values specified by the source to the head of the advice stack.
    fn write_stack(&mut self, source: AdviceSource) -> Result<(), ExecutionError>;

    /// Maps a key to a value list to be yielded by `write_stack` with the [AdviceSource::Map]
    /// variant.
    ///
    /// # Errors
    /// Returns an error if the key is already present in the advice map.
    fn insert_into_map(&mut self, key: Word, map: Vec<Felt>) -> Result<(), ExecutionError>;

    // ADVISE SETS
    // --------------------------------------------------------------------------------------------

    /// Returns a node/leaf for the given depth and index in a Merkle tree with the given root.
    ///
    /// # Errors
    /// Returns an error if:
    /// - A Merkle tree for the specified root cannot be found in this advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree
    ///   identified by the specified root.
    /// - Value of the node at the specified depth and index is not known to this advice provider.
    fn get_tree_node(&self, root: Word, depth: &Felt, index: &Felt)
        -> Result<Word, ExecutionError>;

    /// Returns a path to a node at the specified index in a Merkle tree with the specified root.
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

    /// Updates a leaf at the specified index on an existing Merkle tree with the specified root;
    /// returns the Merkle path from the updated leaf to the new root.
    ///
    /// If `update_in_copy` is set to true, retains both the tree prior to the update (i.e. with
    /// the original root), and the new updated tree. Otherwise, the old merkle set is removed from
    /// this provider.
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

    // CONTEXT MANAGEMENT
    // --------------------------------------------------------------------------------------------

    /// Increments the clock cycle.
    ///
    /// This is used to keep the state of the VM in sync with the state of the advice provider, and
    /// should be incrementally updated when called.
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

    fn write_stack(&mut self, source: AdviceSource) -> Result<(), ExecutionError> {
        T::write_stack(self, source)
    }

    fn insert_into_map(&mut self, key: Word, map: Vec<Felt>) -> Result<(), ExecutionError> {
        T::insert_into_map(self, key, map)
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

    fn update_merkle_node(
        &mut self,
        root: Word,
        depth: &Felt,
        index: &Felt,
        value: Word,
    ) -> Result<MerklePath, ExecutionError> {
        T::update_merkle_node(self, root, depth, index, value)
    }

    fn advance_clock(&mut self) {
        T::advance_clock(self)
    }
}
