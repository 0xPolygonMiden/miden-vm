use super::{Felt, Word};

// ADVICE SOURCE
// ================================================================================================

/// Specifies the source of the value(s) to be pushed onto the advice stack.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AdviceSource {
    /// Puts a single value onto the advice stack.
    Value(Felt),

    /// Puts a word (4 elements) onto the stack.
    Word(Word),

    /// Fetches a list of elements under the specified key from the advice map and pushes them onto
    /// the advice stack.
    ///
    /// If `include_len` is set to true, this also pushes the number of elements onto the advice
    /// stack.
    ///
    /// Note: this operation doesn't consume the map element so it can be called multiple times
    /// for the same key.
    ///
    /// # Example
    /// Given an advice stack `[a, b, c, ...]`, and a map `x |-> [d, e, f]`:
    ///
    /// A call `push_stack(AdviceSource::Map { key: x, include_len: false })` will result in
    /// advice stack: `[d, e, f, a, b, c, ...]`.
    ///
    /// A call `push_stack(AdviceSource::Map { key: x, include_len: true })` will result in
    /// advice stack: `[3, d, e, f, a, b, c, ...]`.
    ///
    /// # Errors
    /// Returns an error if the key was not found in the key-value map.
    Map { key: Word, include_len: bool },
}
