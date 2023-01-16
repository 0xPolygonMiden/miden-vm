use super::{
    chiplets::hasher,
    errors::{AdviceSetError, InputError},
    utils::IntoBytes,
    Felt, FieldElement, Word,
};
use core::convert::TryInto;
use winter_utils::collections::{vec, BTreeMap, Vec};

mod advice;
pub use advice::AdviceSet;

mod stack;
pub use stack::StackInputs;

// PROGRAM INPUTS
// ================================================================================================

/// Input container for Miden VM programs.
///
/// Miden VM programs can receive inputs in two ways:
/// 1. The stack can be initialized to some set of values at the beginning of the program. These
///    inputs are public and must be shared with the verifier for them to verify a proof of the
///    correct execution of a Miden program. There is no limit to the number of elements at the top
///    of the stack which can receive an initial value.
/// 2. The program may request nondeterministic advice inputs from the prover. These inputs are
///    secret inputs. This means that the prover does not need to share them with the verifier.
///    There are two types of advice inputs: (1) a single advice tape which can contain any number
///    of elements and (2) a list of advice sets, which are used to provide nondeterministic
///    inputs for instructions which work with Merkle trees.
///
/// TODO: add more detailed explanation.
#[derive(Clone, Debug)]
pub struct ProgramInputs {
    advice_tape: Vec<Felt>,
    advice_map: BTreeMap<[u8; 32], Vec<Felt>>,
    advice_sets: BTreeMap<[u8; 32], AdviceSet>,
}

impl ProgramInputs {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------
    /// Returns [ProgramInputs] instantiated with the specified initial stack values, advice tape
    /// values, and advice sets.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The number initial stack values is greater than 16.
    /// - Any of the initial stack values or the advice tape values are not valid field elements.
    /// - Any of the advice sets have the same root.
    pub fn new(advice_tape: &[u64], advice_sets: Vec<AdviceSet>) -> Result<Self, InputError> {
        Self::with_advice_map(advice_tape, BTreeMap::new(), advice_sets)
    }

    /// Returns [ProgramInputs] instantiated with the specified initial stack values, advice tape,
    /// key-value advice map, and advice sets.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The number initial stack values is greater than 16.
    /// - Any of the initial stack values or the advice tape values are not valid field elements.
    /// - Any of the advice sets have the same root.
    pub fn with_advice_map(
        advice_tape: &[u64],
        advice_map: BTreeMap<[u8; 32], Vec<Felt>>,
        advice_sets: Vec<AdviceSet>,
    ) -> Result<Self, InputError> {
        // convert advice tape values into field elements
        let mut advice_tape_elements = Vec::with_capacity(advice_tape.len());
        for &value in advice_tape {
            let element = value
                .try_into()
                .map_err(|_| InputError::NotFieldElement(value, "advice tape value"))?;
            advice_tape_elements.push(element);
        }

        // put advice sets into a map
        let mut advice_sets_elements = BTreeMap::new();
        for advice_set in advice_sets {
            let key = advice_set.root().into_bytes();
            if advice_sets_elements.insert(key, advice_set).is_some() {
                return Err(InputError::DuplicateAdviceRoot(key));
            };
        }

        Ok(Self {
            advice_tape: advice_tape_elements,
            advice_map,
            advice_sets: advice_sets_elements,
        })
    }

    /// Returns [ProgramInputs] with empty values and sets.
    pub fn empty() -> Self {
        Self {
            advice_tape: vec![],
            advice_map: BTreeMap::new(),
            advice_sets: BTreeMap::new(),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a reference to the advice tape.
    pub fn advice_tape(&self) -> &[Felt] {
        &self.advice_tape
    }

    // DESTRUCTURING
    // --------------------------------------------------------------------------------------------

    /// Decomposes these [ProgramInputs] into their raw components.
    #[allow(clippy::type_complexity)]
    pub fn into_parts(
        self,
    ) -> (Vec<Felt>, BTreeMap<[u8; 32], Vec<Felt>>, BTreeMap<[u8; 32], AdviceSet>) {
        let Self {
            advice_tape,
            advice_map,
            advice_sets,
        } = self;

        (advice_tape, advice_map, advice_sets)
    }
}
