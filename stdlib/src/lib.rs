#![cfg_attr(not(feature = "std"), no_std)]

use assembly::{
    utils::{collections::Vec, string::ToString},
    Library, LibraryNamespace, Module, ModuleAst, ModulePath,
};
use core::slice::Iter;

pub mod asm;
use asm::MODULES;

// CONSTANTS
// ================================================================================================

const NAMESPACE: &str = "std";
const VERSION: &str = env!("CARGO_PKG_VERSION");

// STANDARD LIBRARY
// ================================================================================================

/// TODO: add docs
pub struct StdLibrary {
    namespace: LibraryNamespace,
    modules: Vec<Module>,
}

impl Library for StdLibrary {
    type ModuleIterator<'a> = Iter<'a, Module>;

    fn root_ns(&self) -> &LibraryNamespace {
        &self.namespace
    }

    fn version(&self) -> &str {
        VERSION
    }

    fn modules(&self) -> Self::ModuleIterator<'_> {
        self.modules.iter()
    }
}

impl Default for StdLibrary {
    /// Returns a new [StdLibrary] instance instantiated with default parameters.
    fn default() -> Self {
        let namespace =
            LibraryNamespace::try_from(NAMESPACE.to_string()).expect("malformed library namespace");
        let modules = MODULES
            .iter()
            .map(|(path, bytes)| {
                let (ns, path) = path.split_once("::").expect("malformed module path");
                let path = ModulePath::try_from(path.to_string()).expect("malformed module path");
                let path = path.to_absolute(&namespace);
                assert_eq!(namespace.as_str(), ns, "invalid namespace");

                // deserialize module AST
                let ast = ModuleAst::from_bytes(bytes)
                    .expect("static module deserialization should be infallible");

                Module::new(path, ast)
            })
            .collect();
        Self { namespace, modules }
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
        assert_eq!("0.3.0", stdlib.version())
    }
}
