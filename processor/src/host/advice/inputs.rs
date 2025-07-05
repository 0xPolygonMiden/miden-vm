use alloc::vec::Vec;

use vm_core::{
    AdviceMap, Felt, Word,
    crypto::merkle::{InnerNodeInfo, MerkleStore},
    errors::InputError,
    utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable},
};

// ADVICE INPUTS
// ================================================================================================

/// Inputs container to initialize advice provider for the execution of Miden VM programs.
///
/// The program may request nondeterministic advice inputs from the prover. These inputs are secret
/// inputs. This means that the prover does not need to share them with the verifier.
///
/// There are three types of advice inputs:
///
/// 1. Single advice stack which can contain any number of elements.
/// 2. Key-mapped element lists which can be pushed onto the advice stack.
/// 3. Merkle store, which is used to provide nondeterministic inputs for instructions that operates
///    with Merkle trees.
#[cfg(not(feature = "testing"))]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AdviceInputs {
    stack: Vec<Felt>,
    map: AdviceMap,
    store: MerkleStore,
}

impl AdviceInputs {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Attempts to extend the stack values with the given sequence of integers, returning an error
    /// if any of the numbers fails while converting to an element `[Felt]`.
    pub fn with_stack_values<I>(mut self, iter: I) -> Result<Self, InputError>
    where
        I: IntoIterator<Item = u64>,
    {
        let stack = iter
            .into_iter()
            .map(|v| Felt::try_from(v).map_err(|e| InputError::NotFieldElement(v, e)))
            .collect::<Result<Vec<_>, _>>()?;

        self.stack.extend(stack.iter());
        Ok(self)
    }

    /// Extends the stack with the given elements.
    pub fn with_stack<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = Felt>,
    {
        self.stack.extend(iter);
        self
    }

    /// Extends the map of values with the given argument, replacing previously inserted items.
    pub fn with_map<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = (Word, Vec<Felt>)>,
    {
        self.map.extend(iter);
        self
    }

    /// Replaces the [MerkleStore] with the provided argument.
    pub fn with_merkle_store(mut self, store: MerkleStore) -> Self {
        self.store = store;
        self
    }

    // PUBLIC MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Extends the stack with the given elements.
    pub fn extend_stack<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Felt>,
    {
        self.stack.extend(iter);
    }

    /// Extends the map of values with the given argument, replacing previously inserted items.
    pub fn extend_map<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (Word, Vec<Felt>)>,
    {
        self.map.extend(iter);
    }

    /// Extends the [MerkleStore] with the given nodes.
    pub fn extend_merkle_store<I>(&mut self, iter: I)
    where
        I: Iterator<Item = InnerNodeInfo>,
    {
        self.store.extend(iter);
    }

    /// Extends the contents of this instance with the contents of the other instance.
    pub fn extend(&mut self, other: Self) {
        self.stack.extend(other.stack);
        self.map.extend(other.map);
        self.store.extend(other.store.inner_nodes());
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a reference to the advice stack.
    pub fn stack(&self) -> &[Felt] {
        &self.stack
    }

    /// Fetch a values set mapped by the given key.
    pub fn mapped_values(&self, key: &Word) -> Option<&[Felt]> {
        self.map.get(key)
    }

    /// Returns the underlying [MerkleStore].
    pub const fn merkle_store(&self) -> &MerkleStore {
        &self.store
    }

    // DESTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Decomposes these `[Self]` into their raw components.
    #[allow(clippy::type_complexity)]
    pub(crate) fn into_parts(self) -> (Vec<Felt>, AdviceMap, MerkleStore) {
        let Self { stack, map, store } = self;
        (stack, map, store)
    }
}

impl Serializable for AdviceInputs {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let Self { stack, map, store } = self;
        stack.write_into(target);
        map.write_into(target);
        store.write_into(target);
    }
}

impl Deserializable for AdviceInputs {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let stack = Vec::<Felt>::read_from(source)?;
        let map = AdviceMap::read_from(source)?;
        let store = MerkleStore::read_from(source)?;
        Ok(Self { stack, map, store })
    }
}

// TESTING
// ================================================================================================

#[cfg(feature = "testing")]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AdviceInputs {
    pub stack: Vec<Felt>,
    pub map: AdviceMap,
    pub store: MerkleStore,
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use winter_utils::{Deserializable, Serializable};

    use crate::AdviceInputs;

    #[test]
    fn test_advice_inputs_eq() {
        let advice1 = AdviceInputs::default();
        let advice2 = AdviceInputs::default();

        assert_eq!(advice1, advice2);

        let advice1 = AdviceInputs::default().with_stack_values([1, 2, 3].iter().copied()).unwrap();
        let advice2 = AdviceInputs::default().with_stack_values([1, 2, 3].iter().copied()).unwrap();

        assert_eq!(advice1, advice2);
    }

    #[test]
    fn test_advice_inputs_serialization() {
        let advice1 = AdviceInputs::default().with_stack_values([1, 2, 3].iter().copied()).unwrap();
        let bytes = advice1.to_bytes();
        let advice2 = AdviceInputs::read_from_bytes(&bytes).unwrap();

        assert_eq!(advice1, advice2);
    }
}
