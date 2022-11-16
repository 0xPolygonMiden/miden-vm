use super::{AssemblerError, Procedure, ProcedureId, String, ToString, Vec};
use crate::MODULE_PATH_DELIM;

// ASSEMBLER CONTEXT
// ================================================================================================

/// TODO: add comments
pub struct AssemblerContext {
    local_procs: Vec<Procedure>,
    module_path: String,
}

impl AssemblerContext {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Returns an [AssemblerContext] used for compiling executable programs.
    ///
    /// When compiling a program, module path is set to "::". This, the path of a local procedure
    /// in a program will be "::proc_index".
    pub fn for_program() -> Self {
        Self {
            local_procs: Vec::new(),
            module_path: MODULE_PATH_DELIM.to_string(),
        }
    }

    /// Returns an [AssemblerContext] used for compiling modules.
    pub fn for_module(module_path: String) -> Self {
        Self {
            local_procs: Vec::new(),
            module_path,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    pub fn module_path(&self) -> &str {
        &self.module_path
    }

    pub fn get_local_proc(&self, index: u16) -> Result<&Procedure, AssemblerError> {
        self.local_procs
            .get(index as usize)
            .ok_or_else(|| AssemblerError::undefined_proc(index))
    }

    pub fn find_local_proc(&self, proc_id: &ProcedureId) -> Option<&Procedure> {
        self.local_procs.iter().find(|proc| proc.id() == proc_id)
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    pub fn add_local_proc(&mut self, procedure: Procedure) {
        self.local_procs.push(procedure);
    }

    pub fn into_local_procs(self) -> Vec<Procedure> {
        self.local_procs
    }
}
