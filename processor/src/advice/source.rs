use super::{Felt, Word};

// ADVICE SOURCE
// ================================================================================================

/// Placeholder for advice provider tape mutation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AdviceSource {
    /// Writes a single value to the head of the advice tape.
    Value(Felt),

    /// Fetch a keyed tape from the values map, reversing and appending it to the advice tape.
    ///
    /// Note: this operation shouldn't consume the map element so it can be called multiple times
    /// for the same key.
    ///
    /// # Example
    /// Given an advice tape `[a,b,c]`, and a map `x |-> [d,e,f]`, a call `write_tape_from_map(x)`
    /// will result in `[a,b,c,f,e,d]` for the advice tape, and will preserve `x |-> [d,e,f]`.
    ///
    /// # Errors
    /// Returns an error if the key was not found in the key-value map.
    Map { key: Word },
}
