use super::{Felt, Word};

// ADVICE SOURCE
// ================================================================================================

/// Specifies the source of the value(s) to be pushed onto the advice stack.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AdviceSource {
    /// Puts a single value onto the advice stack.
    Value(Felt),

    /// Fetches a list of elements under the specified key from the advice map and pushes them onto
    /// the advice stack.
    ///
    /// Note: this operation doesn't consume the map element so it can be called multiple times
    /// for the same key.
    ///
    /// # Example
    /// Given an advice stack `[a, b, c, ...]`, and a map `x |-> [d, e, f]`, a call
    /// `push_stack(AdviceSource::Map { key: x })` will result in  `[f, e, d, a, b, c, ...]` for
    /// the advice stack, and will preserve `x |-> [d, e, f]` in the advice map.
    ///
    /// # Errors
    /// Returns an error if the key was not found in the key-value map.
    Map { key: Word },
}
