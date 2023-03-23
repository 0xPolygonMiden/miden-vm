use super::{BTreeMap, Felt, InputError, MerkleStore, Vec};

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
/// 3. Merkle store, which is used to provide nondeterministic inputs for instructions that
///    operates with Merkle trees.
#[derive(Clone, Debug, Default)]
pub struct AdviceInputs {
    stack: Vec<Felt>,
    map: BTreeMap<[u8; 32], Vec<Felt>>,
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
            .map(|v| {
                Felt::try_from(v).map_err(|_| {
                    InputError::NotFieldElement(v, "the provided value isn't a valid field element")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.stack.extend(stack);
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
        I: IntoIterator<Item = ([u8; 32], Vec<Felt>)>,
    {
        self.map.extend(iter);
        self
    }

    /// Replaces the [MerkleStore] with the provided argument.
    pub fn with_merkle_store(mut self, store: MerkleStore) -> Self {
        self.store = store;
        self
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a reference to the advice stack.
    pub fn stack(&self) -> &[Felt] {
        &self.stack
    }

    /// Fetch a values set mapped by the given key.
    pub fn mapped_values(&self, key: &[u8; 32]) -> Option<&[Felt]> {
        self.map.get(key).map(Vec::as_slice)
    }

    /// Returns the underlying [MerkleStore].
    pub const fn merkle_store(&self) -> &MerkleStore {
        &self.store
    }

    // DESTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Decomposes these `[Self]` into their raw components.
    #[allow(clippy::type_complexity)]
    pub(crate) fn into_parts(self) -> (Vec<Felt>, BTreeMap<[u8; 32], Vec<Felt>>, MerkleStore) {
        let Self { stack, map, store } = self;
        (stack, map, store)
    }
}
