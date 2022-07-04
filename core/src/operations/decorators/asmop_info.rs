use crate::utils::string::String;

// ASMOP INFO
// ================================================================================================

/// Contains information corresponsing to an assembly instruction (only applicable in debug mode)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsmOpInfo {
    op: String,
    cycles: u8,
}

impl AsmOpInfo {
    /// Returns [AsmOpInfo] instantiated with the specified assembly instruction string and number
    /// of cycles it takes to execute the assembly instruction.
    pub fn new(op: String, cycles: u8) -> Self {
        Self { op, cycles }
    }

    /// Returns the assembly instruction corresponding to this decorator.
    pub fn get_op(&self) -> &String {
        &self.op
    }

    /// Returns the number of VM cycles taken to execute the assembly instruction of this decorator.
    pub fn get_cycles(&self) -> u8 {
        self.cycles
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Change cycles corresponding to an AsmOp decorator to the specified number of cycles.
    pub fn set_cycles(&mut self, cycles: u8) {
        self.cycles = cycles;
    }
}
