use crate::utils::string::String;
use core::fmt;

// ASSEMBLY OP
// ================================================================================================

/// Contains information corresponding to an assembly instruction (only applicable in debug mode).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssemblyOp {
    context_name: String,
    num_cycles: u8,
    op: String,
    should_break: bool,
}

impl AssemblyOp {
    /// Returns [AssemblyOp] instantiated with the specified assembly instruction string and number
    /// of cycles it takes to execute the assembly instruction.
    pub fn new(context_name: String, num_cycles: u8, op: String, should_break: bool) -> Self {
        Self {
            context_name,
            num_cycles,
            op,
            should_break,
        }
    }

    /// Returns the context name for this operation.
    pub fn context_name(&self) -> &str {
        &self.context_name
    }

    /// Returns the number of VM cycles taken to execute the assembly instruction of this decorator.
    pub const fn num_cycles(&self) -> u8 {
        self.num_cycles
    }

    /// Returns the assembly instruction corresponding to this decorator.
    pub fn op(&self) -> &str {
        &self.op
    }

    /// Returns `true` if there is a breakpoint for the current operation.
    pub const fn should_break(&self) -> bool {
        self.should_break
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Change cycles corresponding to an AsmOp decorator to the specified number of cycles.
    pub fn set_num_cycles(&mut self, num_cycles: u8) {
        self.num_cycles = num_cycles;
    }
}

impl fmt::Display for AssemblyOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "context={}, operation={}, cost={}",
            self.context_name, self.op, self.num_cycles,
        )
    }
}
