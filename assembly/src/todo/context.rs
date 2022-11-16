use super::{Procedure, Vec};

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

    pub fn local_procs(&self) -> &[Procedure] {
        &self.local_procs
    }

    pub fn add_local_procedure(&mut self, procedure: Procedure) {
        self.local_procs.push(procedure);
    }

    pub fn into_local_procs(self) -> Vec<Procedure> {
        self.local_procs
    }
}
