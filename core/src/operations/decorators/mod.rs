use crate::utils::collections::Vec;
use core::fmt;

mod advice;
pub use advice::{AdviceExtractor, AdviceFunction, AdviceInjector};

mod assembly_op;
pub use assembly_op::AssemblyOp;

mod debug;
pub use debug::DebugOptions;

mod host_function;
pub use host_function::{HostFunction, HostResult};

// DECORATORS
// ================================================================================================

/// A set of decorators which can be executed by the VM.
///
/// Executing a decorator does not affect the state of the main VM components such as operand stack
/// and memory. However, decorators may modify the host.
///
/// Executing decorators does not advance the VM clock. As such, many decorators can be executed in
/// a single VM cycle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Decorator {
    /// Executes a host function against the host.
    HostFunction(HostFunction),
    /// Adds information about the assembly instruction at a particular index (only applicable in
    /// debug mode).
    AsmOp(AssemblyOp),
}

impl fmt::Display for Decorator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AsmOp(assembly_op) => {
                write!(f, "asmOp({}, {})", assembly_op.op(), assembly_op.num_cycles())
            }
            Self::HostFunction(host_function) => write!(f, "{}", host_function),
        }
    }
}

/// Vector consisting of a tuple of operation index (within a span block) and decorator at that index
pub type DecoratorList = Vec<(usize, Decorator)>;

/// Iterator used to iterate through the decorator list of a span block
/// while executing operation batches of a span block.
pub struct DecoratorIterator<'a> {
    decorators: &'a DecoratorList,
    idx: usize,
}

impl<'a> DecoratorIterator<'a> {
    /// Returns a new instance of decorator iterator instantiated with the provided decorator list.
    pub fn new(decorators: &'a DecoratorList) -> Self {
        Self { decorators, idx: 0 }
    }

    /// Returns the next decorator but only if its position matches the specified position,
    /// otherwise, None is returned.
    #[inline(always)]
    pub fn next_filtered(&mut self, pos: usize) -> Option<&Decorator> {
        if self.idx < self.decorators.len() && self.decorators[self.idx].0 == pos {
            self.idx += 1;
            Some(&self.decorators[self.idx - 1].1)
        } else {
            None
        }
    }
}

impl<'a> Iterator for DecoratorIterator<'a> {
    type Item = &'a Decorator;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.decorators.len() {
            self.idx += 1;
            Some(&self.decorators[self.idx - 1].1)
        } else {
            None
        }
    }
}
