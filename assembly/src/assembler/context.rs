use super::{
    AbsolutePath, AssemblyError, CallSet, CodeBlock, CodeBlockTable, Kernel, Procedure,
    ProcedureCache, ProcedureId, String, ToString, Vec,
};
use crate::{ProcedureName, MODULE_PATH_DELIM};

// ASSEMBLY CONTEXT
// ================================================================================================

/// Contains information about compilation of a program or a kernel module.
///
/// Assembly context contains a stack of [ModuleContext]'s, each of which, in turn, contains a
/// stack of [ProcedureContext]'s. Thus, at any point in time, we are in a context of compiling a
/// procedure within a module, and we have access to the info about the current module/procedure
/// tuple bing compiled.
pub struct AssemblyContext {
    module_stack: Vec<ModuleContext>,
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
            vec![ModuleContext::for_program()]
        };

        Self {
            module_stack: modules,
            is_kernel,
            kernel: None,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns true if this context is used for compiling a kernel.
    pub fn is_kernel(&self) -> bool {
        self.is_kernel
    }

    /// Returns the number of memory locals allocated for the procedure currently being compiled.
    pub fn num_proc_locals(&self) -> u16 {
        self.current_proc_context().expect("no procedures").num_locals
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Initiates compilation of a new module.
    ///
    /// This puts a new module onto the module stack and ensures that there are no circular module
    /// dependencies.
    ///
    /// # Errors
    /// Returns an error if a module with the same path already exists in the module stack.
    pub fn begin_module(&mut self, module_path: &str) -> Result<(), AssemblyError> {
        if self.is_kernel && self.module_stack.is_empty() {
            // a kernel context must be initialized with a kernel module path
            debug_assert_eq!(
                module_path,
                AbsolutePath::KERNEL_PATH,
                "kernel context not initialized with kernel module"
            );
        }

        // make sure this module is not in the chain of modules which are currently being compiled
        if self.module_stack.iter().any(|m| m.path == module_path) {
            let dep_chain =
                self.module_stack.iter().map(|m| m.path.to_string()).collect::<Vec<_>>();
            return Err(AssemblyError::circular_module_dependency(&dep_chain));
        }

        // push a new module context onto the module stack and return
        self.module_stack.push(ModuleContext::for_module(module_path));
        Ok(())
    }

    /// Completes compilation of the current module.
    ///
    /// This pops the module off the module stack and return all local procedures of the module
    /// (both exported and internal) together with the combined callset of module's procedures.
    pub fn complete_module(&mut self) -> (Vec<Procedure>, CallSet) {
        let module_ctx = self.module_stack.pop().expect("no modules");
        if self.is_kernel && self.module_stack.is_empty() {
            // if we are compiling a kernel and this is the last module on the module stack, then
            // it must be the Kernel module; thus, we build a Kernel struct from the procedures
            // exported from the kernel module
            let hashes = module_ctx
                .compiled_procs
                .iter()
                .filter(|proc| proc.is_export())
                .map(|proc| proc.code_root().hash())
                .collect::<Vec<_>>();
            self.kernel = Some(Kernel::new(&hashes));
        }

        // return compiled procedures and callset from the module
        (module_ctx.compiled_procs, module_ctx.callset)
    }

    // PROCEDURE PROCESSORS
    // --------------------------------------------------------------------------------------------

    /// Initiates compilation compilation of a new procedure within the current module.
    ///
    /// This puts a new procedure context on the procedure stack of the current module, and also
    /// ensures that there are no procedures with identical name in the same module.
    ///
    /// # Errors
    /// Returns an error if a procedure with the specified name already exists in the current
    /// module.
    pub fn begin_proc(
        &mut self,
        name: &ProcedureName,
        is_export: bool,
        num_locals: u16,
    ) -> Result<(), AssemblyError> {
        self.module_stack
            .last_mut()
            .expect("no modules")
            .begin_proc(name, is_export, num_locals)
    }

    /// Completes compilation of the current procedure and adds the compiled procedure to the list
    /// of the current module's compiled procedures.
    pub fn complete_proc(&mut self, code_root: CodeBlock) {
        self.module_stack.last_mut().expect("no modules").complete_proc(code_root);
    }

    // CALL PROCESSORS
    // --------------------------------------------------------------------------------------------

    /// Registers a call to a procedure in the current module located at the specified index. This
    /// also returns a reference to the invoked procedure.
    ///
    /// A procedure can be called in two modes:
    /// - inlined, when the procedure body is inlined into the MAST.
    /// - not inlined: when a new CALL block is created for the procedure call.
    ///
    /// # Errors
    /// Returns an error if:
    /// - A procedure at the specified index could not be found.
    /// - We are compiling a kernel and the procedure is not inlined.
    pub fn register_local_call(
        &mut self,
        proc_idx: u16,
        inlined: bool,
    ) -> Result<&Procedure, AssemblyError> {
        // non-inlined calls (i.e., `call` instructions) cannot be executed in a kernel
        if self.is_kernel && !inlined {
            let proc_name = &self.current_proc_context().expect("no procedure").name;
            return Err(AssemblyError::call_in_kernel(proc_name));
        }

        self.module_stack
            .last_mut()
            .expect("no modules")
            .register_local_call(proc_idx, inlined)
    }

    /// Registers a call to the specified external procedure (i.e., a procedure which is not a part
    /// of the current module).
    ///
    /// A procedure can be called in two modes:
    /// - inlined, when the procedure body is inlined into the MAST.
    /// - not inlined: when a new CALL or SYSCALL block is created for the procedure call.
    ///
    /// # Errors
    /// Returns an error if:
    /// - A procedure at the specified index could not be found.
    /// - We are compiling a kernel and the procedure is not inlined.
    pub fn register_external_call(
        &mut self,
        proc: &Procedure,
        inlined: bool,
    ) -> Result<(), AssemblyError> {
        // non-inlined calls (i.e., `call` instructions) cannot be executed in a kernel
        if self.is_kernel && !inlined {
            let proc_name = &self.current_proc_context().expect("no procedure").name;
            return Err(AssemblyError::call_in_kernel(proc_name));
        }

        self.module_stack
            .last_mut()
            .expect("no modules")
            .register_external_call(proc, inlined);

        Ok(())
    }

    // CONTEXT FINALIZERS
    // --------------------------------------------------------------------------------------------

    /// Transforms this context into a [Kernel].
    ///
    /// This method is invoked at the end of the compilation of a kernel module.
    ///
    /// # Panics
    /// Panics if this context was not used for kernel compilation (i.e., was not instantiated with
    /// is_kernel == true) or if the kernel module has not been completed yet.
    pub fn into_kernel(self) -> Kernel {
        self.kernel.expect("no kernel")
    }

    /// Transforms this context into a [CodeBlockTable] for the compiled program.
    ///
    /// This method is invoked at the end of the compilation of an executable program.
    ///
    /// # Panics
    /// Panics if:
    /// - There is not exactly one module left on the module stack.
    /// - If this module is not an executable module.
    /// - If any of the procedures in the module's callset cannot be found in the specified
    ///   procedure cache or the local procedure set of the module.
    pub fn into_cb_table(mut self, proc_cache: &ProcedureCache) -> CodeBlockTable {
        // get the last module off the module stack
        assert_eq!(self.module_stack.len(), 1, "module stack must contain exactly one module");
        let mut main_module_context = self.module_stack.pop().unwrap();
        // complete compilation of the executable module; this appends the callset of the main
        // procedure to the callset of the executable module
        main_module_context.complete_executable();

        // build the code block table based on the callset of the executable module; called
        // procedures can be either in the specified procedure cache (for procedures imported from
        // other modules) or in the module's procedures (for procedures defined locally).
        let mut cb_table = CodeBlockTable::default();
        for proc_id in main_module_context.callset.iter() {
            let proc = proc_cache
                .get(proc_id)
                .or_else(|| main_module_context.find_local_proc(proc_id))
                .expect("callset procedure not found");

            cb_table.insert(proc.code_root().clone());
        }

        cb_table
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Returns the context of the procedure currently being compiled, or None if module or
    /// procedure stacks are empty.
    fn current_proc_context(&self) -> Option<&ProcedureContext> {
        self.module_stack.last().and_then(|m| m.proc_stack.last())
    }

    /// Returns the name of the current procedure, or the reserved name for the main block.
    pub(crate) fn current_context_name(&self) -> &str {
        self.current_proc_context()
            .map(|p| p.name().as_ref())
            .expect("library compilation mode is currently not supported!")
    }
}

// MODULE CONTEXT
// ================================================================================================

/// Contains information about compilation of a single module. This includes both library modules
/// and executable modules.
#[derive(Debug)]
struct ModuleContext {
    /// A stack of procedures which are in the process of being compiled. The procedure which
    /// is currently being compiled is at the top of this list.
    proc_stack: Vec<ProcedureContext>,
    /// List of local procedures which have already been compiled for this module.
    compiled_procs: Vec<Procedure>,
    /// Fully qualified path of this module.
    path: String,
    /// A combined callset of all procedure callsets in this module.
    callset: CallSet,
}

impl ModuleContext {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Returns a new [ModuleContext] instantiated for compiling an executable module.
    ///
    /// Procedure in the returned module context is initialized with procedure context for the
    /// "main" procedure.
    pub fn for_program() -> Self {
        let name = ProcedureName::main();
        let main_proc_context = ProcedureContext::new(name, false, 0);
        Self {
            proc_stack: vec![main_proc_context],
            compiled_procs: Vec::new(),
            path: MODULE_PATH_DELIM.to_string(),
            callset: CallSet::default(),
        }
    }

    /// Returns a new [ModuleContext] instantiated for compiling library modules.
    ///
    /// A library module must be identified by a unique module path.
    pub fn for_module(module_path: &str) -> Self {
        Self {
            proc_stack: Vec::new(),
            compiled_procs: Vec::new(),
            path: module_path.to_string(),
            callset: CallSet::default(),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns true if this module is the executable module of a program.
    pub fn is_executable(&self) -> bool {
        // module path for executable modules is "::"
        self.path == MODULE_PATH_DELIM
    }

    /// Returns a [Procedure] with the specified ID, or None if a compiled procedure with such ID
    /// could not be found in this context.
    pub fn find_local_proc(&self, proc_id: &ProcedureId) -> Option<&Procedure> {
        self.compiled_procs.iter().find(|proc| proc.id() == proc_id)
    }

    // PROCEDURE PROCESSORS
    // --------------------------------------------------------------------------------------------

    /// Puts a new procedure context on the stack procedure stack to indicate that we started
    /// compilation of a new procedure.
    ///
    /// # Errors
    /// Returns an error if a procedure with the same name has already been complied or is in the
    /// process of being compiled.
    pub fn begin_proc(
        &mut self,
        name: &ProcedureName,
        is_export: bool,
        num_locals: u16,
    ) -> Result<(), AssemblyError> {
        // make sure a procedure with this name as not been compiled yet and is also not currently
        // on the stack of procedures being compiled
        if self.compiled_procs.iter().any(|p| p.label() == name)
            || self.proc_stack.iter().any(|p| &p.name == name)
        {
            return Err(AssemblyError::duplicate_proc_name(name, &self.path));
        }

        let name = ProcedureName::try_from(name.to_string())?;
        self.proc_stack.push(ProcedureContext::new(name, is_export, num_locals));
        Ok(())
    }

    /// Completes compilation of a procedure currently on the top of procedure stack.
    ///
    /// This pops a procedure context off the top of the procedure stack, converts it into a
    /// compiled procedure, and adds it to the list of compiled procedures.
    ///
    /// This also updates module callset to include the callset of the newly compiled procedure.
    pub fn complete_proc(&mut self, code_root: CodeBlock) {
        let proc_context = self.proc_stack.pop().expect("no procedures");

        // build an ID for the procedure as follows:
        // - for exported procedures: hash("module_path::proc_name")
        // - for internal procedures: hash("module_path::proc_index")
        let proc_id = if proc_context.is_export {
            ProcedureId::from_name(&proc_context.name, &self.path)
        } else {
            let proc_idx = self.compiled_procs.len() as u16;
            ProcedureId::from_index(proc_idx, &self.path)
        };

        let proc = proc_context.into_procedure(proc_id, code_root);
        self.callset.append(proc.callset());
        self.compiled_procs.push(proc);
    }

    // CALL PROCESSORS
    // --------------------------------------------------------------------------------------------

    /// Registers a call to a local procedure in this module located at the specified index and
    /// returns a reference to the invoked procedure.
    ///
    /// This appends the callset of the called procedure to the callset of the current procedure at
    /// the top of procedure stack. If inlined == false, the called procedure itself is added to
    /// the callset of the current procedure as well.
    ///
    /// # Errors
    /// Returns an error if a procedure at the specified index could not be found.
    pub fn register_local_call(
        &mut self,
        proc_idx: u16,
        inlined: bool,
    ) -> Result<&Procedure, AssemblyError> {
        // get the called procedure from the listed of already compiled local procedures
        let called_proc = self
            .compiled_procs
            .get(proc_idx as usize)
            .ok_or_else(|| AssemblyError::local_proc_not_found(proc_idx, &self.path))?;

        // get the context of the procedure currently being compiled
        let context = self.proc_stack.last_mut().expect("no proc context");

        // append the callset of the called procedure to the current callset as all calls made as
        // the result of the called procedure may be made as a result of current procedure as well
        context.callset.append(called_proc.callset());

        // if the called procedure was not inlined, we include it in the current callset as well
        if !inlined {
            context.callset.insert(*called_proc.id());
        }
        Ok(called_proc)
    }

    /// Registers a call to the specified external procedure (i.e., a procedure which is not a part
    /// of the current module).
    ///
    /// This also appends the callset of the called procedure to the callset of the current
    /// procedure at the top of procedure stack. If inlined == false, the called procedure itself
    /// is added to the callset of the current procedure as well.
    pub fn register_external_call(&mut self, called_proc: &Procedure, inlined: bool) {
        // get the context of the procedure currently being compiled
        let context = self.proc_stack.last_mut().expect("no proc context");

        // append the callset of the called procedure to the current callset as all calls made as
        // the result of the called procedure may be made as a result of current procedure as well
        context.callset.append(called_proc.callset());

        // if the called procedure was not inlined, we include it in the current callset as well
        if !inlined {
            context.callset.insert(*called_proc.id());
        }
    }

    // EXECUTABLE FINALIZER
    // --------------------------------------------------------------------------------------------
    /// Completes compilation of the executable module.
    ///
    /// Executable modules are not completed the same way library modules are. Thus, at the end of
    /// compiling a program, the executable module will have the main procedure left on the
    /// procedure stack. To complete the module we need to pop the main procedure off the stack and
    /// append its callset to the callset of the module context.
    ///
    /// # Panics
    /// - If this module is not an executable module.
    /// - If there is not exactly one procedure left on the procedure stack.
    /// - If the procedure left on the procedure stack is not main procedure.
    pub fn complete_executable(&mut self) {
        assert!(self.is_executable(), "module not executable");
        assert_eq!(self.proc_stack.len(), 1, "procedure stack must contain exactly one procedure");
        let main_proc_context = self.proc_stack.pop().unwrap();
        assert!(main_proc_context.is_main(), "not main procedure");
        self.callset.append(&main_proc_context.callset);
    }
}

// PROCEDURE CONTEXT
// ================================================================================================

/// Contains information about compilation of a single procedure.
#[derive(Debug)]
struct ProcedureContext {
    name: ProcedureName,
    is_export: bool,
    num_locals: u16,
    callset: CallSet,
}

impl ProcedureContext {
    pub fn new(name: ProcedureName, is_export: bool, num_locals: u16) -> Self {
        Self {
            name,
            is_export,
            num_locals,
            callset: CallSet::default(),
        }
    }

    pub fn is_main(&self) -> bool {
        self.name.is_main()
    }

    /// Returns the current context name.
    ///
    /// Check [AssemblyContext::current_context_name] for reference.
    pub const fn name(&self) -> &ProcedureName {
        &self.name
    }

    pub fn into_procedure(self, id: ProcedureId, code_root: CodeBlock) -> Procedure {
        let Self {
            name,
            is_export,
            num_locals,
            callset,
        } = self;

        Procedure::new(id, name, is_export, num_locals as u32, code_root, callset)
    }
}
