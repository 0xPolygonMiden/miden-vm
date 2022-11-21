#![cfg_attr(not(feature = "std"), no_std)]

use vm_assembly::{Library, LibraryError, ModuleAst, ModuleProvider, NamedModuleAst, ProcedureId};
use vm_core::utils::{
    collections::{BTreeMap, Vec},
    string::{String, ToString},
};

pub mod asm;
use asm::MODULES;

// CONSTANTS
// ================================================================================================

const VERSION: &str = env!("CARGO_PKG_VERSION");

// STANDARD LIBRARY
// ================================================================================================

/// TODO: add docs
pub struct StdLibrary {
    modules: Vec<(String, ModuleAst)>,
    proc_to_module: BTreeMap<ProcedureId, usize>,
}

impl ModuleProvider for StdLibrary {
    fn get_module(&self, proc_id: &ProcedureId) -> Option<NamedModuleAst<'_>> {
        self.proc_to_module
            .get(proc_id)
            .map(|&module_idx| &self.modules[module_idx])
            .map(|(path, ast)| ast.named_ref(path))
    }
}

impl Library for StdLibrary {
    type Module = ModuleAst;

    /// Returns root namespace of the standard library, which is always "std".
    fn root_ns(&self) -> &str {
        "std"
    }

    /// Returns the current version of the standard library.
    fn version(&self) -> &str {
        VERSION
    }

    /// Returns the source code of the module located at the specified path.
    ///
    /// # Errors
    /// Returns an error if the modules for the specified path does not exist in the standard
    /// library.
    fn get_module(&self, module_path: &str) -> Result<&ModuleAst, LibraryError> {
        self.modules
            .iter()
            .find(|(path, _)| path == module_path)
            .map(|(_, ast)| ast)
            .ok_or_else(|| LibraryError::ModuleNotFound(module_path.to_string()))
    }
}

impl Default for StdLibrary {
    /// Returns a new [StdLibrary] instance instantiated with default parameters.
    fn default() -> Self {
        let mut modules = Vec::with_capacity(MODULES.len());
        let mut proc_to_module = BTreeMap::new();

        for (i, (module_path, module_bytes)) in MODULES.iter().enumerate() {
            // deserialize module AST
            let module_ast = ModuleAst::from_bytes(module_bytes)
                .expect("static module deserialization should be infallible");

            // for each procedure in the module, compute its ID and create a map between procedure
            // ID and its module
            for proc_ast in module_ast.local_procs.iter() {
                let proc_id = ProcedureId::from_name(&proc_ast.name, module_path);
                proc_to_module.insert(proc_id, i);
            }

            // add the module together with its path to the module list
            modules.push((module_path.to_string(), module_ast));
        }

        Self {
            modules,
            proc_to_module,
        }
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::Library;

    #[test]
    fn lib_version() {
        let stdlib = super::StdLibrary::default();
        assert_eq!("0.2.0", stdlib.version())
    }
}
