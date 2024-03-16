use alloc::vec::Vec;

use miden_crypto::{Word, ZERO};

use super::{
    ByteWriter, Felt, OutputError, Serializable, StackTopState, ToElements, STACK_TOP_SIZE,
};
use crate::utils::{range, ByteReader, Deserializable, DeserializationError};

// STACK OUTPUTS
// ================================================================================================

/// Output container for Miden VM programs.
///
/// Miden program outputs contain the full state of the stack at the end of execution as well as the
/// addresses in the overflow table which are required to reconstruct the table (when combined with
/// the overflow values from the stack state).
///
/// `stack` is expected to be ordered as if the elements were popped off the stack one by one.
/// Thus, the value at the top of the stack is expected to be in the first position, and the order
/// of the rest of the output elements will also match the order on the stack.
///
/// `overflow_addrs` is expected to start with the `prev` address value from the first row in the
/// overflow table (the row representing the deepest element in the stack) and then be followed by
/// the address (`clk` value) of each row in the table starting from the deepest element in the
/// stack and finishing with the row which was added to the table last.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StackOutputs {
    /// The elements on the stack at the end of execution.
    stack: Vec<Felt>,
    /// The overflow table row addresses required to reconstruct the final state of the table.
    overflow_addrs: Vec<Felt>,
}

impl StackOutputs {
    // CONSTANTS
    // --------------------------------------------------------------------------------------------

    pub const MAX_LEN: usize = u16::MAX as usize;

    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Constructs a new [StackOutputs] struct from the provided stack elements and overflow
    /// addresses.
    ///
    /// # Errors
    ///  Returns an error if the number of stack elements is greater than `STACK_TOP_SIZE` (16) and
    /// `overflow_addrs` does not contain exactly `stack.len() + 1 - STACK_TOP_SIZE` elements.
    pub fn new(mut stack: Vec<Felt>, overflow_addrs: Vec<Felt>) -> Result<Self, OutputError> {
        // validate stack length
        if stack.len() > Self::MAX_LEN {
            return Err(OutputError::OutputSizeTooBig(stack.len()));
        }

        // get overflow_addrs length
        let expected_overflow_addrs_len = get_overflow_addrs_len(stack.len());

        // validate overflow_addrs length
        if overflow_addrs.len() != expected_overflow_addrs_len {
            return Err(OutputError::InvalidOverflowAddressLength(
                overflow_addrs.len(),
                expected_overflow_addrs_len,
            ));
        }

        // pad stack to the `STACK_TOP_SIZE`
        if stack.len() < STACK_TOP_SIZE {
            stack.resize(STACK_TOP_SIZE, ZERO);
        }

        Ok(Self {
            stack,
            overflow_addrs,
        })
    }

    /// Attempts to create [StackOutputs] struct from the provided stack elements and overflow
    /// addresses represented as vectors of `u64` values.
    ///
    /// # Errors
    /// Returns an error if:
    /// - Any of the provided stack elements are invalid field elements.
    /// - Any of the provided overflow addresses are invalid field elements.
    pub fn try_from_ints(stack: Vec<u64>, overflow_addrs: Vec<u64>) -> Result<Self, OutputError> {
        // Validate stack elements
        let stack = stack
            .iter()
            .map(|v| Felt::try_from(*v))
            .collect::<Result<Vec<Felt>, _>>()
            .map_err(OutputError::InvalidStackElement)?;

        // Validate overflow address elements
        let overflow_addrs = overflow_addrs
            .iter()
            .map(|v| Felt::try_from(*v))
            .collect::<Result<Vec<Felt>, _>>()
            .map_err(OutputError::InvalidOverflowAddress)?;

        Self::new(stack, overflow_addrs)
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the element located at the specified position on the stack or `None` if out of
    /// bounds.
    pub fn get_stack_item(&self, idx: usize) -> Option<Felt> {
        self.stack.get(idx).cloned()
    }

    /// Returns the word located starting at the specified Felt position on the stack or `None` if
    /// out of bounds. For example, passing in `0` returns the word at the top of the stack, and
    /// passing in `4` returns the word starting at element index `4`.
    pub fn get_stack_word(&self, idx: usize) -> Option<Word> {
        let word_elements: Word = {
            let word_elements: Vec<Felt> = range(idx, 4)
                .map(|idx| self.get_stack_item(idx))
                // Elements need to be reversed, since a word `[a, b, c, d]` will be stored on the
                // stack as `[d, c, b, a]`
                .rev()
                .collect::<Option<_>>()?;

            word_elements.try_into().expect("a Word contains 4 elements")
        };

        Some(word_elements)
    }

    /// Returns the stack outputs, which is state of the stack at the end of execution converted to
    /// integers.
    pub fn stack(&self) -> &[Felt] {
        &self.stack
    }

    /// Returns the number of requested stack outputs or returns the full stack if fewer than the
    /// requested number of stack values exist.
    pub fn stack_truncated(&self, num_outputs: usize) -> &[Felt] {
        let len = self.stack.len().min(num_outputs);
        &self.stack[..len]
    }

    /// Returns the state of the top of the stack at the end of execution.
    pub fn stack_top(&self) -> StackTopState {
        self.stack
            .iter()
            .take(STACK_TOP_SIZE)
            .cloned()
            .collect::<Vec<_>>()
            .try_into()
            .expect("failed to convert vector to array")
    }

    /// Returns the overflow address outputs, which are the addresses required to reconstruct the
    /// overflow table (when combined with the stack overflow values) converted to integers.
    pub fn overflow_addrs(&self) -> &[Felt] {
        &self.overflow_addrs
    }

    /// Returns true if the overflow table outputs are non-empty.
    pub fn has_overflow(&self) -> bool {
        !self.overflow_addrs.is_empty()
    }

    /// Returns the previous address `prev` for the first row in the stack overflow table
    pub fn overflow_prev(&self) -> Felt {
        self.overflow_addrs[0]
    }

    /// Returns (address, value) for all rows which were on the overflow table at the end of
    /// execution in the order in which they were added to the table (deepest stack item first).
    pub fn stack_overflow(&self) -> Vec<(Felt, Felt)> {
        let mut overflow = Vec::with_capacity(self.overflow_addrs.len() - 1);
        for (addr, val) in self
            .overflow_addrs
            .iter()
            .skip(1)
            .zip(self.stack.iter().skip(STACK_TOP_SIZE).rev())
        {
            overflow.push((*addr, *val));
        }

        overflow
    }

    // PUBLIC MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Returns mutable access to the stack outputs, to be used for testing or running examples.
    /// TODO: this should be marked with #[cfg(test)] attribute, but that currently won't work with
    /// the integration test handler util.
    pub fn stack_mut(&mut self) -> &mut [Felt] {
        &mut self.stack
    }
}

// HELPER FUNCTIONS
// ================================================================================================

impl ToElements<Felt> for StackOutputs {
    fn to_elements(&self) -> Vec<Felt> {
        self.stack.iter().chain(self.overflow_addrs.iter()).cloned().collect()
    }
}

/// Returs the number of overflow addresses based on the lenght of the stack.
fn get_overflow_addrs_len(stack_len: usize) -> usize {
    if stack_len > STACK_TOP_SIZE {
        stack_len + 1 - STACK_TOP_SIZE
    } else {
        0
    }
}

// SERIALIZATION
// ================================================================================================

impl Serializable for StackOutputs {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        debug_assert!(self.stack.len() <= Self::MAX_LEN);
        target.write_usize(self.stack.len());
        target.write_many(&self.stack);

        target.write_many(&self.overflow_addrs);
    }
}

impl Deserializable for StackOutputs {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let count = source.read_usize()?;
        if count > Self::MAX_LEN {
            return Err(DeserializationError::InvalidValue(format!(
                "Number of values on the output stack can not be more than {}, but {} was found",
                Self::MAX_LEN,
                count
            )));
        }
        let stack = source.read_many::<Felt>(count)?;

        let count = get_overflow_addrs_len(stack.len());
        let overflow_addrs = source.read_many::<Felt>(count)?;

        Ok(Self {
            stack,
            overflow_addrs,
        })
    }
}
