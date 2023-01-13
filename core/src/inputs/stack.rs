use super::{vec, Felt, InputError, Vec};
use core::slice;

// STACK INPUTS
// ================================================================================================

/// Initial state of the stack to support program execution.
///
/// The program execution expects the inputs to be a stack on the VM, and it will be stored in
/// reversed order on this struct.
#[derive(Clone, Debug, Default)]
pub struct StackInputs {
    values: Vec<Felt>,
}

impl StackInputs {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Returns `[StackInputs]` from a list of values, reversing them into a stack.
    pub fn new(mut values: Vec<Felt>) -> Self {
        values.reverse();
        Self { values }
    }

    /// Returns `[StackInputs]` without values.
    pub const fn empty() -> Self {
        Self { values: vec![] }
    }

    /// Attempts to create stack inputs from an iterator of numbers, failing if they do not
    /// represent a valid field element.
    pub fn try_from_values<I>(iter: I) -> Result<Self, InputError>
    where
        I: IntoIterator<Item = u64>,
    {
        iter.into_iter()
            .map(|v| {
                Felt::try_from(v).map_err(|_| {
                    InputError::NotFieldElement(v, "the provided value isn't a valid field element")
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Self::new)
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the initial stack values in stack/reversed order.
    pub fn values(&self) -> &[Felt] {
        &self.values
    }
}

impl<'a> IntoIterator for &'a StackInputs {
    type Item = &'a Felt;
    type IntoIter = slice::Iter<'a, Felt>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl IntoIterator for StackInputs {
    type Item = Felt;
    type IntoIter = vec::IntoIter<Felt>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}
