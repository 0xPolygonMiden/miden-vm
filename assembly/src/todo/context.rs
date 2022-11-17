use super::{AssemblerError, Procedure, ProcedureId, String, ToString, Vec};
use crate::MODULE_PATH_DELIM;

// ASSEMBLER CONTEXT
// ================================================================================================

/// TODO: add comments
pub struct ModuleContext {
    local_procs: Vec<Procedure>,
    module_path: String,
    is_kernel: bool,
}

impl ModuleContext {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Returns an [ModuleContext] used for compiling executable modules.
    ///
    /// When compiling a an executable, module path is set to "::". Thus, the path of a local
    /// procedure in a program will be "::proc_index".
    pub fn for_program() -> Self {
        Self {
            local_procs: Vec::new(),
            module_path: MODULE_PATH_DELIM.to_string(),
            is_kernel: false,
        }
    }

    /// Returns an [ModuleContext] used for compiling library modules.
    pub fn for_module(module_path: String, is_kernel: bool) -> Self {
        Self {
            local_procs: Vec::new(),
            module_path,
            is_kernel,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns module path for the module which is currently being compiled.
    pub fn module_path(&self) -> &str {
        &self.module_path
    }

    /// Returns true if this context is used for compiling a kernel module.
    pub fn is_kernel(&self) -> bool {
        self.is_kernel
    }

    /// Returns a [Procedure] located at the specified index in the module.
    ///
    /// # Error
    /// Returns an error if there is no compiled procedure at the specified index.
    pub fn get_local_proc(&self, index: u16) -> Result<&Procedure, AssemblerError> {
        self.local_procs
            .get(index as usize)
            .ok_or_else(|| AssemblerError::undefined_proc(index))
    }

    /// Returns a [Procedure] with the specified ID, or None if a procedure with such ID could not
    /// be found in this context.
    pub fn find_local_proc(&self, proc_id: &ProcedureId) -> Option<&Procedure> {
        self.local_procs.iter().find(|proc| proc.id() == proc_id)
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Adds a compiled procedure to this context.
    pub fn add_local_proc(&mut self, procedure: Procedure) {
        self.local_procs.push(procedure);
    }

    /// Converts this context into a list of compiled procedures.
    pub fn into_local_procs(self) -> Vec<Procedure> {
        self.local_procs
    }
}
