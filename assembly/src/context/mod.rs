use super::{
    parse_local_procs, parse_module, AssemblyError, BTreeMap, CodeBlock, ModuleAst, ModuleMap,
    ProcMap, Procedure, StdLibrary, String, ToString, MODULE_PATH_DELIM,
};
use vm_core::{CodeBlockTable, Library};

// ASSEMBLY CONTEXT
// ================================================================================================

/// Context for a compilation of a given program.
///
/// An assembly context contains a set of procedures which can be called from the parsed code.
/// The procedures are divided into 3 groups:
/// 1. Local procedures, which are procedures parsed from the body of a program.
/// 2. Imported procedures, which are procedures imported from external libraries.
/// 3. Kernel procedures, which are procedures provided by the kernel specified for the program.
///
/// Local procedures are owned by the context, while imported and kernel procedures are stored by
/// reference.
pub struct AssemblyContext<'a> {
    local_procs: ProcMap,
    imported_procs: BTreeMap<String, &'a Procedure>,
    kernel_procs: Option<&'a ProcMap>,
    stdlib: StdLibrary,
    in_debug_mode: bool,
}

impl<'a> AssemblyContext<'a> {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [AssemblyContext] with the specified set of kernel procedures. If kernel
    /// procedures are not provided, it is assumed that the context is created for compiling kernel
    /// procedures. That implies that the context can be instantiated in two modes:
    /// - A mode for compiling a kernel (when kernel_procs parameter is set to None). In this mode
    ///   in_kernel() method returns true.
    /// - A mode for compiling regular programs (when kernel_procs parameter contains a set of
    ///   kernel procedures). In this mode, in_kernel() method returns false.
    pub fn new(kernel_procs: Option<&'a ProcMap>, in_debug_mode: bool) -> Self {
        Self {
            kernel_procs,
            local_procs: BTreeMap::new(),
            imported_procs: BTreeMap::new(),
            stdlib: StdLibrary::default(),
            in_debug_mode,
        }
    }

    // STATE ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns true if the assembly context was instantiated in debug mode.
    pub fn in_debug_mode(&self) -> bool {
        self.in_debug_mode
    }

    /// Returns true if a procedure with the specified label exists in this context.
    pub fn contains_proc(&self, label: &str) -> bool {
        self.local_procs.contains_key(label) || self.imported_procs.contains_key(label)
    }

    /// Returns a code root of a procedure for the specified label from this context.
    pub fn get_proc_code(&self, label: &str) -> Option<&CodeBlock> {
        // `expect()`'s are OK here because we first check if a given map contains the key
        if self.imported_procs.contains_key(label) {
            let proc = *self
                .imported_procs
                .get(label)
                .expect("no procedure after contains");
            Some(proc.code_root())
        } else if self.local_procs.contains_key(label) {
            let proc = self
                .local_procs
                .get(label)
                .expect("no procedure after contains");
            Some(proc.code_root())
        } else {
            None
        }
    }

    pub fn get_local_proc_code(&self, index: u32) -> Option<&CodeBlock> {
        // `expect()`'s are OK here because we first check if a given map contains the key
        for proc in self.local_procs.values() {
            if proc.index() == index {
                return Some(proc.code_root());
            }
        }

        None
    }

    /// Returns true if this context is used for compiling kernel module.
    pub fn in_kernel(&self) -> bool {
        self.kernel_procs.is_none()
    }

    /// Returns a code root of a kernel procedure for the specified label in this context.
    pub fn get_kernel_proc_code(&self, label: &str) -> Option<&CodeBlock> {
        // `expect()` is OK here because we first check if the kernel is set
        self.kernel_procs
            .expect("no kernel")
            .get(label)
            .map(|c| c.code_root())
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Adds a local procedure to this context.
    ///
    /// A label for a local procedure is set simply `proc.label`.
    ///
    /// # Panics
    /// Panics if a procedure with the specified label already exists in this context.
    pub fn add_local_proc(&mut self, proc: Procedure) {
        let label = proc.label();
        assert!(!self.contains_proc(label), "duplicate procedure: {}", label);
        self.local_procs.insert(label.to_string(), proc);
    }

    /// Adds an imported procedure to this context.
    ///
    /// A label for an imported procedure is set to  `prefix::proc.label`.
    ///
    /// # Panics
    /// Panics if a procedure with the specified label already exists in this context.
    pub fn add_imported_proc(&mut self, prefix: &str, proc: &'a Procedure) {
        let label = format!("{}{}{}", prefix, MODULE_PATH_DELIM, proc.label());
        assert!(
            !self.contains_proc(&label),
            "duplicate procedure: {}",
            label
        );
        self.imported_procs.insert(label, proc);
    }

    /// Extracts local procedures from this context.
    pub fn into_local_procs(self) -> BTreeMap<String, Procedure> {
        self.local_procs
    }

    /// Handles module import from exec/call/syscall operations.
    /// For each module import, it retrieves exported procedures from the specified module and
    /// inserts them into the provided context.
    ///
    /// If the module hasn't been parsed yet, parses it, and adds
    /// the parsed module into the provided `self.parsed_modules`.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The `use` instruction is malformed.
    /// - A module specified by the `use` instruction could not be found.
    /// - Parsing the specified module results in an error.
    #[allow(clippy::cast_ref_to_mut)]
    pub fn parse_imports(
        &self,
        module_path: &String,
        dep_chain: &mut Vec<String>,
        parsed_modules: &'a mut ModuleMap,
        cb_table: &mut CodeBlockTable,
    ) -> Result<(), AssemblyError> {
        // check if a module with the same path is currently being parsed somewhere up
        // the chain; if it is, then we have a circular dependency.
        if dep_chain.iter().any(|v| v == module_path) {
            dep_chain.push(module_path.clone());
            return Err(AssemblyError::circular_module_dependency(dep_chain));
        }

        // add the current module to the dependency chain
        dep_chain.push(module_path.clone());

        // if the module hasn't been parsed yet, retrieve its source from the library
        // and attempt to parse it; if the parsing is successful, this will also add
        // the parsed module to the parsed_module cache.
        if !parsed_modules.contains_key(module_path) {
            let module_source = self
                .stdlib
                .get_module_source(module_path)
                .map_err(|_| AssemblyError::missing_import_source(module_path))?;
            let module_ast = parse_module(module_source)?;
            self.parse_module(
                module_path,
                &module_ast,
                parsed_modules,
                dep_chain,
                cb_table,
            )?;
        }

        // get procedures from the module at the specified path; we are guaranteed to
        // not fail here because the above code block ensures that either there is a
        // parsed module for the specified path, or the function returns with an error
        let module_procs = parsed_modules.get(module_path).expect("no module procs");

        let path_parts = module_path.split(MODULE_PATH_DELIM).collect::<Vec<_>>();
        let num_parts = path_parts.len();

        unsafe {
            let mutable_self = &mut *(self as *const _ as *mut AssemblyContext);

            // add all procedures to the current context; procedure labels are set to be
            // `last_part_of_module_path::procedure_name`. For example, `u256::add`.
            for proc in module_procs.values() {
                mutable_self.add_imported_proc(path_parts[num_parts - 1], proc);
            }
        }

        dep_chain.pop();

        Ok(())
    }

    /// Parses a set of exported procedures from the specified source code and adds these
    /// procedures to `self.parsed_modules` using the specified path as the key.
    #[allow(clippy::cast_ref_to_mut)]
    fn parse_module(
        &self,
        path: &str,
        module_ast: &ModuleAst,
        parsed_modules: &'a mut ModuleMap,
        dep_chain: &mut Vec<String>,
        cb_table: &mut CodeBlockTable,
    ) -> Result<(), AssemblyError> {
        let mut context = AssemblyContext::new(self.kernel_procs, self.in_debug_mode);

        // parse imported modules (if any), and add exported procedures from these modules to
        // the current context
        self.parse_imports(&String::from(path), dep_chain, parsed_modules, cb_table)?;

        parse_local_procs(
            &module_ast.procedures,
            &mut context,
            cb_table,
            parsed_modules,
        )?;

        // extract the exported local procedures from the context
        let mut module_procs = context.into_local_procs();
        module_procs.retain(|_, p| p.is_export());

        parsed_modules.insert(String::from(path), module_procs);

        Ok(())
    }
}
