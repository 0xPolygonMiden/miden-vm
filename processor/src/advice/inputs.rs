use super::{utils::IntoBytes, AdviceSet, BTreeMap, Felt, InputError, Vec};

// ADVICE INPUTS
// ================================================================================================

/// Inputs container to initialize advice provider for the execution of Miden VM programs.
///
/// The program may request nondeterministic advice inputs from the prover. These inputs are secret
/// inputs. This means that the prover does not need to share them with the verifier.
///
/// There are three types of advice inputs:
///
/// 1. Single advice tape which can contain any number of elements.
/// 2. Multiple advice tapes that can be appended to the main tape, and are mapped by 32 bytes keys.
/// 3. Advice sets list, which are used to provide nondeterministic inputs for instructions that
///    operates with Merkle trees.
#[derive(Clone, Debug, Default)]
pub struct AdviceInputs {
    tape: Vec<Felt>,
    values_map: BTreeMap<[u8; 32], Vec<Felt>>,
    merkle_sets: BTreeMap<[u8; 32], AdviceSet>,
}

impl AdviceInputs {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Attempts to extend the tape values with the given sequence of integers, returning an error
    /// if any of the numbers fails while converting to an element `[Felt]`.
    pub fn with_tape_values<I>(mut self, iter: I) -> Result<Self, InputError>
    where
        I: IntoIterator<Item = u64>,
    {
        let tape = iter
            .into_iter()
            .map(|v| {
                Felt::try_from(v).map_err(|_| {
                    InputError::NotFieldElement(v, "the provided value isn't a valid field element")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.tape.extend(tape);
        Ok(self)
    }

    /// Extends the tape with the given elements.
    pub fn with_tape<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = Felt>,
    {
        self.tape.extend(iter);
        self
    }

    /// Extends the map of values with the given argument, replacing previously inserted items.
    pub fn with_values_map<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = ([u8; 32], Vec<Felt>)>,
    {
        self.values_map.extend(iter);
        self
    }

    /// Attempts to extend the Merkle sets with the given argument, failing if a duplicated root is
    /// provided.
    pub fn with_merkle_sets<I>(mut self, iter: I) -> Result<Self, InputError>
    where
        I: IntoIterator<Item = AdviceSet>,
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

    /// Returns a reference to the advice tape.
    pub fn tape(&self) -> &[Felt] {
        &self.tape
    }

    /// Fetch a values set mapped by the given key.
    pub fn mapped_values(&self, key: &[u8; 32]) -> Option<&[Felt]> {
        self.values_map.get(key).map(Vec::as_slice)
    }

    /// Fetch a Merkle set mapped by the given key.
    pub fn merkle_set(&self, key: &[u8; 32]) -> Option<&AdviceSet> {
        self.merkle_sets.get(key)
    }

    // DESTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Decomposes these `[Self]` into their raw components.
    #[allow(clippy::type_complexity)]
    pub(crate) fn into_parts(
        self,
    ) -> (Vec<Felt>, BTreeMap<[u8; 32], Vec<Felt>>, BTreeMap<[u8; 32], AdviceSet>) {
        let Self {
            tape,
            values_map,
            merkle_sets,
        } = self;
        (tape, values_map, merkle_sets)
    }
}
