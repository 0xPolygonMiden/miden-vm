use core::fmt;

// ADVICE EXTRACTORS
// ================================================================================================

/// Defines a set of actions which can be initiated from the VM to extract data from the advice
/// provider. These actions can only modify the advice stack.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AdviceExtractor {
    /// Pops an element from the advice stack and returns it.
    ///
    /// # Errors
    /// Returns an error if the advice stack is empty.
    ///
    /// Inputs:
    ///  Operand stack: [...]
    ///  Advice stack: [value, ...]
    ///  Advice map: {...}
    ///  Merkle store: {...}
    ///
    /// Outputs:
    ///  Operand stack: [...]
    ///  Advice stack: [...]
    ///  Advice map: {...}
    ///  Merkle store: {...}
    ///  Return: \[value\]
    PopStack,

    /// Pops a word (4 elements) from the advice stack and returns it.
    ///
    /// Note: a word is popped off the stack element-by-element. For example, a `[d, c, b, a, ...]`
    /// stack (i.e., `d` is at the top of the stack) will yield `[d, c, b, a]`.
    ///
    /// # Errors
    /// Returns an error if the advice stack does not contain a full word.
    ///
    /// Inputs:
    ///  Operand stack: [...]
    ///  Advice stack: [d, c, b, a, ...]
    ///  Advice map: {...}
    ///  Merkle store: {...}
    ///
    /// Outputs:
    ///  Operand stack: [...]
    ///  Advice stack: [...]
    ///  Advice map: {...}
    ///  Merkle store: {...}
    ///  Return: [a, b, c, d]
    PopStackWord,

    /// Pops a double word (8 elements) from the advice stack and returns them.
    ///
    /// Note: words are popped off the stack element-by-element. For example, a
    /// `[h, g, f, e, d, c, b, a, ...]` stack (i.e., `h` is at the top of the stack) will yield
    /// two words: `[h, g, f,e ], [d, c, b, a]`.
    ///
    /// # Errors
    /// Returns an error if the advice stack does not contain two words.
    ///
    /// Inputs:
    ///  Operand stack: [...]
    ///  Advice stack: [h, g, f, e, d, c, b, a, ...]
    ///  Advice map: {...}
    ///  Merkle store: {...}
    ///
    /// Outputs:
    ///  Operand stack: [...]
    ///  Advice stack: [...]
    ///  Advice map: {...}
    ///  Merkle store: {...}
    ///  Return: [a, b, c, d, e, f, g, h]
    PopStackDWord,

    /// Extracts a Merkle path for the node specified by the values at the top of the operand stack
    /// and returns it to the caller.
    ///
    /// # Errors
    /// Returns an error if the Merkle store does not contain the specified Merkle path.
    ///
    /// Inputs:
    ///  Operand stack: [WORD, depth, index, ROOT, ...]
    ///  Advice stack: [...]
    ///  Advice map: {...}
    ///  Merkle store: {path, ...}
    ///
    /// Outputs:
    ///  Operand stack: [WORD, depth, index, ROOT, ...]
    ///  Advice stack: [...]
    ///  Advice map: {...}
    ///  Merkle store: {path, ...}
    ///  Return: \[path\]
    GetMerklePath,
}

impl fmt::Display for AdviceExtractor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PopStack => write!(f, "pop_stack"),
            Self::PopStackWord => write!(f, "pop_stack_word"),
            Self::PopStackDWord => write!(f, "pop_stack_dword"),
            Self::GetMerklePath => {
                write!(f, "get_merkle_path")
            }
        }
    }
}
