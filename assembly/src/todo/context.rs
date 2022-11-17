use super::{AssemblerError, Kernel, Procedure, ProcedureId, String, ToString, Vec};
use crate::MODULE_PATH_DELIM;

// ASSEMBLY CONTEXT
// ================================================================================================

/// TODO: add comments
pub struct AssemblyContext {
    modules: Vec<ModuleContext>,
    is_kernel: bool,
    kernel: Option<Kernel>,
}

impl AssemblyContext {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------
    /// Returns a new [AssemblyContext]. When is_kernel is set to true, the context is instantiated
    /// for compiling a kernel module. Otherwise, the context is instantiated for compiling
    /// executable programs.
    pub fn new(is_kernel: bool) -> Self {
        let modules = if is_kernel {
            Vec::new()
        } else {
            // for executable programs we initialize the module stack with the context of the
            // executable module itself
            vec![ModuleContext::new(MODULE_PATH_DELIM)]
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
        &self.modules.last().expect("no modules").module_path
    }

    /// Returns true if this context is used for compiling a kernel.
    pub fn is_kernel(&self) -> bool {
        self.is_kernel
    }

    /// Returns a [Procedure] located at the specified index in the module which is currently being
    /// compiled.
    ///
    /// # Error
    /// Returns an error if there is no compiled procedure at the specified index in the current
    /// module.
    pub fn get_local_proc(&self, index: u16) -> Result<&Procedure, AssemblerError> {
        self.modules
            .last()
            .expect("no modules")
            .get_local_proc(index)
    }

    /// Returns a [Procedure] with the specified ID, or None if a procedure with such ID could not
    /// be found in the module which is currently being compiled.
    pub fn find_local_proc(&self, proc_id: &ProcedureId) -> Option<&Procedure> {
        self.modules
            .last()
            .expect("no modules")
            .find_local_proc(proc_id)
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Initiates compilation of a new module.
    ///
    /// This puts a new module onto the module stack.
    ///
    /// # Errors
    /// Returns an error if a module with the same path already exists in the module stack.
    pub fn begin_module(&mut self, module_path: &str) -> Result<(), AssemblerError> {
        if self.is_kernel && self.modules.is_empty() {
            // a kernel context must be initialized with a kernel module path
            debug_assert_eq!(
                module_path,
                ProcedureId::KERNEL_PATH,
                "kernel context not initialized with kernel module"
            );
        }

        // make sure this module is not in the chain of modules which are currently being compiled
        if self.modules.iter().any(|m| m.module_path == module_path) {
            let dep_chain = self
                .modules
                .iter()
                .map(|m| m.module_path.to_string())
                .collect::<Vec<_>>();
            return Err(AssemblerError::circular_module_dependency(&dep_chain));
        }

        // push a new module context onto the module stack and return
        self.modules.push(ModuleContext::new(module_path));
        Ok(())
    }

    /// Completes compilation of the current module.
    ///
    /// This pops the module off the module stack and return all local procedures of the module
    /// (both exported and internal).
    pub fn complete_module(&mut self) -> Vec<Procedure> {
        let procs = self.modules.pop().expect("no modules").local_procs;
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

    /// Adds a compiled procedure to the context of the module currently being compiled.
    pub fn add_local_proc(&mut self, procedure: Procedure) {
        self.modules
            .last_mut()
            .expect("no modules")
            .local_procs
            .push(procedure);
    }

    /// Transforms this context into a Kernel.
    ///
    /// # Panics
    /// Panics if this context was not used for kernel compilation (i.e., was not instantiated with
    /// is_kernel = true) or if the kernel module has not been completed yet.
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
    pub fn new(module_path: &str) -> Self {
        Self {
            local_procs: Vec::new(),
            module_path: module_path.to_string(),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

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
}
