use super::{utils::IntoBytes, BTreeMap, Felt, InputError, MerkleSet, Vec};

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
/// 2. Key-mapped stacks set that can be pushed onto the operand stack.
/// 3. Merkle sets list, which are used to provide nondeterministic inputs for instructions that
///    operates with Merkle trees.
#[derive(Clone, Debug, Default)]
pub struct AdviceInputs {
    stack: Vec<Felt>,
    map: BTreeMap<[u8; 32], Vec<Felt>>,
    merkle_sets: BTreeMap<[u8; 32], MerkleSet>,
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

    /// Attempts to extend the Merkle sets with the given argument, failing if a duplicated root is
    /// provided.
    pub fn with_merkle_sets<I>(mut self, iter: I) -> Result<Self, InputError>
    where
        I: IntoIterator<Item = MerkleSet>,
    {
        for set in iter.into_iter() {
            let key = set.root().into_bytes();
            if self.merkle_sets.contains_key(&key) {
                return Err(InputError::DuplicateAdviceRoot(key));
            }
            self.merkle_sets.insert(key, set);
        }
        Ok(self)
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

    /// Fetch a Merkle set mapped by the given key.
    pub fn merkle_set(&self, key: &[u8; 32]) -> Option<&MerkleSet> {
        self.merkle_sets.get(key)
    }

    // DESTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Decomposes these `[Self]` into their raw components.
    #[allow(clippy::type_complexity)]
    pub(crate) fn into_parts(
        self,
    ) -> (Vec<Felt>, BTreeMap<[u8; 32], Vec<Felt>>, BTreeMap<[u8; 32], MerkleSet>) {
        let Self {
            stack,
            map,
            merkle_sets,
        } = self;
        (stack, map, merkle_sets)
    }
}
