use alloc::vec::Vec;
use core::slice;

use super::{ByteWriter, Felt, InputError, Serializable, ToElements};
use crate::utils::{ByteReader, Deserializable, DeserializationError};

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
    // CONSTANTS
    // --------------------------------------------------------------------------------------------

    pub const MAX_LEN: usize = u16::MAX as usize;

    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Returns [StackInputs] from a list of values, reversing them into a stack.
    ///
    /// # Errors
    /// Returns an error if the number of input values exceeds the allowed maximum.
    pub fn new(mut values: Vec<Felt>) -> Result<Self, InputError> {
        if values.len() > Self::MAX_LEN {
            return Err(InputError::InputLengthExceeded(Self::MAX_LEN, values.len()));
        }
        values.reverse();

        Ok(Self { values })
    }

    /// Attempts to create stack inputs from an iterator of integers.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The values do not represent a valid field element.
    /// - Number of values in the iterator exceeds the allowed maximum number of input values.
    pub fn try_from_ints<I>(iter: I) -> Result<Self, InputError>
    where
        I: IntoIterator<Item = u64>,
    {
        let values = iter
            .into_iter()
            .map(|v| Felt::try_from(v).map_err(|e| InputError::NotFieldElement(v, e)))
            .collect::<Result<Vec<_>, _>>()?;

        Self::new(values)
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
    type IntoIter = alloc::vec::IntoIter<Felt>;

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

        debug_assert!(self.values.len() <= Self::MAX_LEN);
        target.write_usize(self.values.len());
        target.write_many(&self.values);
    }
}

impl Deserializable for StackInputs {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let count = source.read_usize()?;
        if count > Self::MAX_LEN {
            return Err(DeserializationError::InvalidValue(format!(
                "Number of values on the input stack can not be more than {}, but {} was found",
                Self::MAX_LEN,
                count
            )));
        }
        let values = source.read_many::<Felt>(count)?;
        Ok(StackInputs { values })
    }
}
