use crate::utils::{collections::*, ByteReader, Deserializable, DeserializationError};

use super::{ByteWriter, Felt, InputError, Serializable, ToElements};
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

    /// Attempts to create stack inputs from an iterator of numbers, failing if they do not
    /// represent a valid field element.
    pub fn try_from_values<I>(iter: I) -> Result<Self, InputError>
    where
        I: IntoIterator<Item = u64>,
    {
        let values = iter
            .into_iter()
            .map(|v| Felt::try_from(v).map_err(|e| InputError::NotFieldElement(v, e)))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(values))
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

impl ToElements<Felt> for StackInputs {
    fn to_elements(&self) -> Vec<Felt> {
        self.values.to_vec()
    }
}

// SERIALIZATION
// ================================================================================================

impl Serializable for StackInputs {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        // TODO the length of the stack, by design, will not be greater than `u32::MAX`. however,
        // we must define a common serialization format as we might diverge from the implementation
        // here and the one provided by default from winterfell.

        debug_assert!(self.values.len() <= u32::MAX as usize);
        target.write_u32(self.values.len() as u32);
        target.write_many(&self.values);
    }
}

impl Deserializable for StackInputs {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let count = source.read_u32()?;

        let values = source.read_many::<Felt>(count as usize)?;
        Ok(StackInputs { values })
    }
}
