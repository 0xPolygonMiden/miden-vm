use alloc::vec::Vec;
use core::slice;

use super::{
    super::ZERO, get_stack_values_num, ByteWriter, Felt, InputError, Serializable, ToElements,
    MIN_STACK_DEPTH,
};
use crate::utils::{ByteReader, Deserializable, DeserializationError};

// STACK INPUTS
// ================================================================================================

/// Defines the initial state of the VM's operand stack.
///
/// The values in the struct are stored in the "stack order" - i.e., the last input is at the top
/// of the stack (in position 0).
#[derive(Clone, Debug, Default)]
pub struct StackInputs {
    elements: [Felt; MIN_STACK_DEPTH],
}

impl StackInputs {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Returns [StackInputs] from a list of values, reversing them into a stack.
    ///
    /// # Errors
    /// Returns an error if the number of input values exceeds the allowed maximum.
    pub fn new(mut values: Vec<Felt>) -> Result<Self, InputError> {
        if values.len() > MIN_STACK_DEPTH {
            return Err(InputError::InputLengthExceeded(MIN_STACK_DEPTH, values.len()));
        }
        values.reverse();
        values.resize(MIN_STACK_DEPTH, ZERO);

        Ok(Self { elements: values.try_into().unwrap() })
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

    /// Returns the initial stack elements in stack/reversed order.
    pub fn elements(&self) -> &[Felt] {
        &self.elements
    }
}

impl<'a> IntoIterator for &'a StackInputs {
    type Item = &'a Felt;
    type IntoIter = slice::Iter<'a, Felt>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl IntoIterator for StackInputs {
    type Item = Felt;
    type IntoIter = core::array::IntoIter<Felt, 16>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl ToElements<Felt> for StackInputs {
    fn to_elements(&self) -> Vec<Felt> {
        self.elements.to_vec()
    }
}

// SERIALIZATION
// ================================================================================================

impl Serializable for StackInputs {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8(get_stack_values_num(self.elements()));
        target.write_many(self.elements);
    }
}

impl Deserializable for StackInputs {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let elements_num = source.read_u8()?;
        let mut elements = source.read_many::<Felt>(elements_num.into())?;

        elements.resize(MIN_STACK_DEPTH, ZERO);

        Ok(StackInputs { elements: elements.try_into().unwrap() })
    }
}
