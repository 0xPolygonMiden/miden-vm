use super::{BTreeMap, Library, LibraryError, Module, ProcedureId, Vec};

// MODULE PROVIDER
// ================================================================================================

/// A module provider contains all modules from libraries available to a given assembler. It is
/// used during compilation to resolve references to imported procedures.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ModuleProvider {
    modules: Vec<Module>,
    /// Map from procedure id to the index of a module in which the procedure is defined.
    procedures: BTreeMap<ProcedureId, usize>,
}

impl ModuleProvider {
    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Fetch a module that contains the provided procedure id.
    pub fn get_module(&self, id: &ProcedureId) -> Option<&Module> {
        // this will panic only if there is a bug in `Self::add_module`.
        self.procedures.get(id).map(|i| &self.modules[*i])
    }

    // MODULE AND LIBRARY MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Adds the provided module to this module provider.
    ///
    /// # Errors
    ///
    /// Will error if there is a duplicated module path.
    fn add_module(&mut self, module: Module) -> Result<(), LibraryError> {
        if self.modules.iter().any(|m| module.path == m.path) {
            return Err(LibraryError::duplicate_module_path(&module.path));
        }
        let module_idx = self.modules.len();
        module.ast.local_procs.iter().for_each(|proc| {
            let proc_name = proc.name.to_absolute(&module.path);
            let proc_id = ProcedureId::from(&proc_name);
            self.procedures.insert(proc_id, module_idx);
        });
        self.modules.push(module);
        Ok(())
    }

    /// Adds all modules from the provided library to this module provider.
    ///
    /// # Errors
    ///
    /// Will error if there is a duplicated module path.
    pub fn add_library<L>(&mut self, library: &L) -> Result<(), LibraryError>
    where
        L: Library,
    {
        let namespace = library.root_ns();
        library.modules().try_for_each(|module| {
            module.check_namespace(namespace)?;
            self.add_module(module.clone())
        })
    }
}
