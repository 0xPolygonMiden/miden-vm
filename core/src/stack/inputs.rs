use winter_utils::{ByteReader, Deserializable, DeserializationError};

use super::{vec, ByteWriter, Felt, InputError, Serializable, ToElements, Vec};
use core::slice;

// STACK INPUTS
// ================================================================================================

const MAX_STACK_INPUTS_SIZE: usize = u16::MAX as usize;

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
    pub fn new<I>(values: I) -> Result<Self, InputError>
    where
        I: IntoIterator<Item = Felt>,
    {
        let mut values: Vec<Felt> = values.into_iter().collect();

        if values.len() > MAX_STACK_INPUTS_SIZE {
            Err(InputError::StackTooBig(values.len()))
        } else {
            values.reverse();
            Ok(Self { values })
        }
    }

    /// Attempts to create stack inputs from an iterator of numbers, failing if they do not
    /// represent a valid field element.
    pub fn try_from_values<I>(iter: I) -> Result<Self, InputError>
    where
        I: IntoIterator<Item = u64>,
    {
        Self::new(iter.into_iter().map(Felt::from))
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
        debug_assert!(self.values.len() <= MAX_STACK_INPUTS_SIZE);
        target.write_u32(self.values.len() as u32);
        self.values.write_into(target);
    }
}

impl Deserializable for StackInputs {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let count: usize = source.read_u32()?.try_into().expect("u32 must fit in a usize");
        let values = Felt::read_batch_from(source, count)?;
        Ok(Self { values })
    }
}
