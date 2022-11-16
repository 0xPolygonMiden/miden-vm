use super::{AssemblerError, Procedure, Vec};

// ASSEMBLER CONTEXT
// ================================================================================================

pub struct AssemblerContext {
    local_procs: Vec<Procedure>,
}

impl AssemblerContext {
    pub fn new() -> Self {
        Self {
            local_procs: Vec::new(),
        }
    }

    pub fn add_local_procedure(&mut self, procedure: Procedure) {
        self.local_procs.push(procedure);
    }

    pub fn into_local_procs(self) -> Vec<Procedure> {
        self.local_procs
    }

    pub fn get_local_proc(&self, index: u16) -> Result<&Procedure, AssemblerError> {
        self.local_procs
            .get(index as usize)
            .ok_or_else(|| AssemblerError::undefined_proc(index))
    }
}
