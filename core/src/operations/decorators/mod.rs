use alloc::{string::ToString, vec::Vec};
use core::fmt;

use miden_crypto::hash::blake::Blake3_256;
use num_traits::ToBytes;

mod assembly_op;
pub use assembly_op::AssemblyOp;

mod debug;
pub use debug::DebugOptions;

use crate::mast::{DecoratorFingerprint, DecoratorId};

// DECORATORS
// ================================================================================================

/// A set of decorators which can be executed by the VM.
///
/// Executing a decorator does not affect the state of the main VM components such as operand stack
/// and memory.
///
/// Executing decorators does not advance the VM clock. As such, many decorators can be executed in
/// a single VM cycle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Decorator {
    /// Adds information about the assembly instruction at a particular index (only applicable in
    /// debug mode).
    AsmOp(AssemblyOp),
    /// Prints out information about the state of the VM based on the specified options. This
    /// decorator is executed only in debug mode.
    Debug(DebugOptions),
    /// Emits a trace to the host.
    Trace(u32),
}

impl Decorator {
    pub fn fingerprint(&self) -> DecoratorFingerprint {
        match self {
            Self::AsmOp(asm_op) => {
                let mut bytes_to_hash = Vec::new();
                if let Some(location) = asm_op.location() {
                    bytes_to_hash.extend(location.uri.as_str().as_bytes());
                    bytes_to_hash.extend(location.start.to_u32().to_le_bytes());
                    bytes_to_hash.extend(location.end.to_u32().to_le_bytes());
                }
                bytes_to_hash.extend(asm_op.context_name().as_bytes());
                bytes_to_hash.extend(asm_op.op().as_bytes());
                bytes_to_hash.push(asm_op.num_cycles());
                bytes_to_hash.push(asm_op.should_break() as u8);

                Blake3_256::hash(&bytes_to_hash)
            },
            Self::Debug(debug) => Blake3_256::hash(debug.to_string().as_bytes()),
            Self::Trace(trace) => Blake3_256::hash(&trace.to_le_bytes()),
        }
    }
}

impl crate::prettier::PrettyPrint for Decorator {
    fn render(&self) -> crate::prettier::Document {
        crate::prettier::display(self)
    }
}

impl fmt::Display for Decorator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AsmOp(assembly_op) => {
                write!(f, "asmOp({}, {})", assembly_op.op(), assembly_op.num_cycles())
            },
            Self::Debug(options) => write!(f, "debug({options})"),
            Self::Trace(trace_id) => write!(f, "trace({trace_id})"),
        }
    }
}

/// Vector consisting of a tuple of operation index (within a span block) and decorator at that
/// index.
///
/// Note: for `AssemblyOp` decorators, when an instruction compiles down to multiple operations,
/// only the first operation is associated with the assembly op.
pub type DecoratorList = Vec<(usize, DecoratorId)>;

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
    pub fn next_filtered(&mut self, pos: usize) -> Option<&DecoratorId> {
        if self.idx < self.decorators.len() && self.decorators[self.idx].0 == pos {
            self.idx += 1;
            Some(&self.decorators[self.idx - 1].1)
        } else {
            None
        }
    }
}

impl<'a> Iterator for DecoratorIterator<'a> {
    type Item = &'a DecoratorId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.decorators.len() {
            self.idx += 1;
            Some(&self.decorators[self.idx - 1].1)
        } else {
            None
        }
    }
}
