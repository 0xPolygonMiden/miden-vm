#![cfg_attr(not(feature = "std"), no_std)]

use vm_assembly::{ModuleAst, ModuleProvider, ProcedureId};
use vm_core::{
    errors::LibraryError,
    utils::{collections::BTreeMap, string::ToString},
    Library,
};

pub mod asm;
use asm::MODULES;

// CONSTANTS
// ================================================================================================

const VERSION: &str = env!("CARGO_PKG_VERSION");

// TYPE ALIASES
// ================================================================================================

type ModuleMap = BTreeMap<ProcedureId, ModuleAst>;
type ModuleNamedMap = BTreeMap<&'static str, ModuleAst>;
type ModuleSource = BTreeMap<&'static str, &'static str>;

// STANDARD LIBRARY
// ================================================================================================

/// TODO: add docs
pub struct StdLibrary {
    modules: ModuleMap,
    named: ModuleNamedMap,
    sources: ModuleSource,
}

impl ModuleProvider for StdLibrary {
    fn get_source(&self, path: &str) -> Option<&str> {
        self.sources.get(path).copied()
    }

    fn get_module(&self, id: &ProcedureId) -> Option<&ModuleAst> {
        self.modules.get(id)
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
        self.named
            .get(module_path)
            .ok_or_else(|| LibraryError::ModuleNotFound(module_path.to_string()))
    }
}

impl Default for StdLibrary {
    /// Returns a new [StdLibrary] instance instantiated with default parameters.
    fn default() -> Self {
        // TODO this will be trimmed in the future to `ids` as the only provider for std library

        let modules = MODULES
            .into_iter()
            .map(|(_, id, _, bytes)| {
                let ast = ModuleAst::from_bytes(bytes)
                    .expect("static module deserialization should be infallible");

                (id, ast)
            })
            .collect();

        let named = MODULES
            .into_iter()
            .map(|(label, _, _, bytes)| {
                let ast = ModuleAst::from_bytes(bytes)
                    .expect("static module deserialization should be infallible");

                (label, ast)
            })
            .collect();

        let sources = MODULES
            .into_iter()
            .map(|(label, _, source, _)| (label, source))
            .collect();

        Self {
            modules,
            named,
            sources,
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
