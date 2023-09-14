use core::fmt;

// DEBUG OPTIONS
// ================================================================================================

/// Options of the `Debug` decorator.
///
/// These options define the debug info which gets printed out when the Debug decorator is
/// executed.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DebugOptions {
    /// Print out the entire contents of the stack for the current execution context.
    StackAll,
    /// Prints out the top n items of the stack for the current context.
    StackTop(u16),
}

impl fmt::Display for DebugOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StackAll => write!(f, "stack"),
            Self::StackTop(n) => write!(f, "stack.{n}"),
        }
    }
}
