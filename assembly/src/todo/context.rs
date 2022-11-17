use super::{AssemblerError, Kernel, Procedure, ProcedureId, String, ToString, Vec};
use crate::MODULE_PATH_DELIM;

// ASSEMBLY
// ================================================================================================

pub struct AssemblyContext {
    modules: Vec<ModuleContext>,
    is_kernel: bool,
    kernel: Option<Kernel>,
}

impl AssemblyContext {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------
    ///
    pub fn new(is_kernel: bool) -> Self {
        let modules = if is_kernel {
            Vec::new()
        } else {
            vec![ModuleContext::new(MODULE_PATH_DELIM.to_string())]
        };

        Self {
            modules,
            is_kernel,
            kernel: None,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns module path for the module which is currently being compiled.
    pub fn module_path(&self) -> &str {
        self.modules.last().expect("no modules").module_path()
    }

    /// Returns true if this context is used for compiling a kernel.
    pub fn is_kernel(&self) -> bool {
        self.is_kernel
    }

    /// Returns a [Procedure] located at the specified index in the module.
    ///
    /// # Error
    /// Returns an error if there is no compiled procedure at the specified index.
    pub fn get_local_proc(&self, index: u16) -> Result<&Procedure, AssemblerError> {
        self.modules
            .last()
            .expect("no modules")
            .get_local_proc(index)
    }

    /// Returns a [Procedure] with the specified ID, or None if a procedure with such ID could not
    /// be found in this context.
    pub fn find_local_proc(&self, proc_id: &ProcedureId) -> Option<&Procedure> {
        self.modules
            .last()
            .expect("no modules")
            .find_local_proc(proc_id)
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    ///
    pub fn begin_module(&mut self, module_path: &str) {
        // TODO: check for circular references
        self.modules
            .push(ModuleContext::new(module_path.to_string()));
    }

    ///
    pub fn complete_module(&mut self) -> Vec<Procedure> {
        let procs = self.modules.pop().expect("no modules").into_local_procs();
        if self.is_kernel && self.modules.is_empty() {
            // if we are compiling a kernel and this is the last module on the module stack, then
            // it must be the Kernel module; thus, we build a Kernel struct from the procedures
            // exported from the kernel module
            let hashes = procs
                .iter()
                .filter(|proc| proc.is_export())
                .map(|proc| proc.code_root().hash())
                .collect::<Vec<_>>();
            self.kernel = Some(Kernel::new(&hashes));
        }
        procs
    }

    /// Adds a compiled procedure to this context.
    pub fn add_local_proc(&mut self, procedure: Procedure) {
        self.modules
            .last_mut()
            .expect("no modules")
            .add_local_proc(procedure);
    }

    pub fn into_kernel(self) -> Kernel {
        self.kernel.expect("no kernel")
    }
}

// MODULE CONTEXT
// ================================================================================================

/// TODO: add comments
struct ModuleContext {
    local_procs: Vec<Procedure>,
    module_path: String,
}

impl ModuleContext {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Returns a new [ModuleContext] instantiated with the specified module path and an empty
    /// vector of local procedures.
    pub fn new(module_path: String) -> Self {
        Self {
            local_procs: Vec::new(),
            module_path,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns module path for the module which is currently being compiled.
    pub fn module_path(&self) -> &str {
        &self.module_path
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
