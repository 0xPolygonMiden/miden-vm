use super::{utils, ExecutionError, Felt, InputError, Word};
use vm_core::{
    utils::{
        collections::{BTreeMap, Vec},
        IntoBytes,
    },
    AdviceSet, StarkField,
};

mod inputs;
pub use inputs::AdviceInputs;

mod mem_provider;
pub use mem_provider::MemAdviceProvider;

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
/// 1. Provide a tape functionality that yields elements as a stack (last in, first out). These can
///    be yielded as elements, words or double words.
/// 2. Provide a map functionality that will store temporary tapes that can be appended to the main
///    tape. This operation should not allow key overwrite; that is: if a given key exists, the
///    implementation should error if the user attempts to insert this key again, instead of the
///    common behavior of the maps to simply override the previous contents. This is a design
///    decision to increase the runtime robustness of the execution.
/// 3. Provide advice sets, that are mappings from a Merkle root its tree. The tree should yield
///    nodes & leaves, and will provide a Merkle path if a leaf is updated.
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

    // ADVICE TAPE
    // --------------------------------------------------------------------------------------------

    /// Pops an element from the advice tape and returns it.
    ///
    /// # Errors
    /// Returns an error if the advice tape is empty.
    fn read_tape(&mut self) -> Result<Felt, ExecutionError>;

    /// Pops a word (4 elements) from the advice tape and returns it.
    ///
    /// Note: a word is always stored as little-endian. A `[...,a,b,c,d]` tape will yield
    /// `[d,c,b,a]`.
    ///
    /// # Errors
    /// Returns an error if the advice tape does not contain a full word.
    fn read_tape_w(&mut self) -> Result<Word, ExecutionError>;

    /// Pops a double word (8 elements) from the advice tape and returns them.
    ///
    /// Note: a double word is always stored as little-endian. A `[...,a,b,c,d,e,f,g,h]` tape will
    /// yield `[h,g,f,e],[,d,c,b,a]`.
    ///
    /// # Errors
    /// Returns an error if the advice tape does not contain two words.
    fn read_tape_dw(&mut self) -> Result<[Word; 2], ExecutionError>;

    /// Writes the provided value at the head of the advice tape.
    fn write_tape(&mut self, value: Felt);

    /// Fetch a keyed tape from the values map, reversing and appending it to the advice tape.
    ///
    /// Note: this operation shouldn't consume the map element so it can be called multiple times
    /// for the same key.
    ///
    /// # Example
    /// Given an advice map `[a,b,c]`, and a map `x |-> [d,e,f]`, a call `write_tape_from_map(x)`
    /// will result in `[a,b,c,f,e,d]` for the advice tape, and will preserve `x |-> [d,e,f]`.
    ///
    /// # Errors
    /// Returns an error if the key was not found in a key-value map.
    fn write_tape_from_map(&mut self, key: Word) -> Result<(), ExecutionError>;

    /// Maps a key to a value list to be yielded by `write_tape_from_map`.
    ///
    /// # Errors
    /// Returns an error if the key is already present in the advice map.
    fn insert_into_map(&mut self, key: Word, values: Vec<Felt>) -> Result<(), ExecutionError>;

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
    fn get_tree_node(&self, root: Word, depth: Felt, index: Felt) -> Result<Word, ExecutionError>;

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
        depth: Felt,
        index: Felt,
    ) -> Result<Vec<Word>, ExecutionError>;

    /// Updates a leaf at the specified index on an existing Merkle tree with the specified root;
    /// returns the Merkle path from the updated leaf to the new root.
    ///
    /// If `update_in_copy` is set to true, retains both the tree prior to the update (i.e. with
    /// the original root), and the new updated tree. Otherwise, the old advice set is removed from
    /// this provider.
    ///
    /// # Errors
    /// Returns an error if:
    /// - A Merkle tree for the specified root cannot be found in this advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree
    ///   identified by the specified root.
    /// - Path to the leaf at the specified index in the specified Merkle tree is not known to this
    ///   advice provider.
    fn update_merkle_leaf(
        &mut self,
        root: Word,
        index: Felt,
        leaf_value: Word,
        update_in_copy: bool,
    ) -> Result<Vec<Word>, ExecutionError>;

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
    fn read_tape(&mut self) -> Result<Felt, ExecutionError> {
        T::read_tape(self)
    }

    fn read_tape_w(&mut self) -> Result<Word, ExecutionError> {
        T::read_tape_w(self)
    }

    fn read_tape_dw(&mut self) -> Result<[Word; 2], ExecutionError> {
        T::read_tape_dw(self)
    }

    fn write_tape(&mut self, value: Felt) {
        T::write_tape(self, value)
    }

    fn write_tape_from_map(&mut self, key: Word) -> Result<(), ExecutionError> {
        T::write_tape_from_map(self, key)
    }

    fn insert_into_map(&mut self, key: Word, values: Vec<Felt>) -> Result<(), ExecutionError> {
        T::insert_into_map(self, key, values)
    }

    fn get_tree_node(&self, root: Word, depth: Felt, index: Felt) -> Result<Word, ExecutionError> {
        T::get_tree_node(self, root, depth, index)
    }

    fn get_merkle_path(
        &self,
        root: Word,
        depth: Felt,
        index: Felt,
    ) -> Result<Vec<Word>, ExecutionError> {
        T::get_merkle_path(self, root, depth, index)
    }

    fn update_merkle_leaf(
        &mut self,
        root: Word,
        index: Felt,
        leaf_value: Word,
        update_in_copy: bool,
    ) -> Result<Vec<Word>, ExecutionError> {
        T::update_merkle_leaf(self, root, index, leaf_value, update_in_copy)
    }

    fn advance_clock(&mut self) {
        T::advance_clock(self)
    }
}
