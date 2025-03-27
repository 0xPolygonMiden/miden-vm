use alloc::{string::String, sync::Arc};
use core::fmt;

use crate::debuginfo::{Location, SourceFile, SourceManager, SourceSpan};

// ASSEMBLY OP
// ================================================================================================

/// Contains information corresponding to an assembly instruction (only applicable in debug mode).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssemblyOp {
    location: Option<Location>,
    context_name: String,
    op: String,
    num_cycles: u8,
    should_break: bool,
}

impl AssemblyOp {
    /// Returns [AssemblyOp] instantiated with the specified assembly instruction string and number
    /// of cycles it takes to execute the assembly instruction.
    pub fn new(
        location: Option<Location>,
        context_name: String,
        num_cycles: u8,
        op: String,
        should_break: bool,
    ) -> Self {
        Self {
            location,
            context_name,
            op,
            num_cycles,
            should_break,
        }
    }

    /// Returns the [Location] for this operation, if known
    pub fn location(&self) -> Option<&Location> {
        self.location.as_ref()
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

    pub fn to_label_and_source_file(
        &self,
        source_manager: &dyn SourceManager,
    ) -> (Option<SourceSpan>, Option<Arc<SourceFile>>) {
        let label = self
            .location
            .clone()
            .and_then(|location| source_manager.location_to_span(location));

        let source_file = self
            .location
            .as_ref()
            .and_then(|location| source_manager.get_by_path(&location.path));

        (label, source_file)
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Change cycles corresponding to an AsmOp decorator to the specified number of cycles.
    pub fn set_num_cycles(&mut self, num_cycles: u8) {
        self.num_cycles = num_cycles;
    }

    /// Change the [Location] of this [AssemblyOp]
    pub fn set_location(&mut self, location: Location) {
        self.location = Some(location);
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
