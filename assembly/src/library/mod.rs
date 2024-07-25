use alloc::vec::Vec;

use vm_core::mast::MastForest;

use crate::ast::{self, FullyQualifiedProcedureName};

mod error;
mod masl;
mod namespace;
mod path;
mod version;

pub use self::error::{CompiledLibraryError, LibraryError};
pub use self::masl::MaslLibrary;
pub use self::namespace::{LibraryNamespace, LibraryNamespaceError};
pub use self::path::{LibraryPath, LibraryPathComponent, PathError};
pub use self::version::{Version, VersionError};

#[cfg(test)]
mod tests;

// COMPILED LIBRARY
// ===============================================================================================

/// Represents a library where all modules modules were compiled into a [`MastForest`].
pub struct CompiledLibrary {
    mast_forest: MastForest,
    // a path for every `root` in the associated MAST forest
    exports: Vec<FullyQualifiedProcedureName>,
}

/// Constructors
impl CompiledLibrary {
    /// Constructs a new [`CompiledLibrary`].
    pub fn new(
        mast_forest: MastForest,
        exports: Vec<FullyQualifiedProcedureName>,
    ) -> Result<Self, CompiledLibraryError> {
        if mast_forest.num_procedures() as usize != exports.len() {
            return Err(CompiledLibraryError::InvalidExports {
                exports_len: exports.len(),
                roots_len: mast_forest.num_procedures() as usize,
            });
        }

        Ok(Self {
            mast_forest,
            exports,
        })
    }
}

impl CompiledLibrary {
    /// Returns the inner [`MastForest`].
    pub fn mast_forest(&self) -> &MastForest {
        &self.mast_forest
    }

    /// Returns the fully qualified name of all procedures exported by the library.
    pub fn exports(&self) -> &[FullyQualifiedProcedureName] {
        &self.exports
    }
}

// LIBRARY
// ===============================================================================================

/// Maximum number of modules in a library.
const MAX_MODULES: usize = u16::MAX as usize;

/// Maximum number of dependencies in a library.
const MAX_DEPENDENCIES: usize = u16::MAX as usize;

/// A library definition that provides AST modules for the compilation process.
pub trait Library {
    /// Returns the root namespace of this library.
    fn root_ns(&self) -> &LibraryNamespace;

    /// Returns the version number of this library.
    fn version(&self) -> &Version;

    /// Iterate the modules available in the library.
    fn modules(&self) -> impl ExactSizeIterator<Item = &ast::Module> + '_;

    /// Returns the dependency libraries of this library.
    fn dependencies(&self) -> &[LibraryNamespace];

    /// Returns the module stored at the provided path.
    fn get_module(&self, path: &LibraryPath) -> Option<&ast::Module> {
        self.modules().find(|&module| module.path() == path)
    }
}

impl<T> Library for &T
where
    T: Library,
{
    fn root_ns(&self) -> &LibraryNamespace {
        T::root_ns(self)
    }

    fn version(&self) -> &Version {
        T::version(self)
    }

    fn modules(&self) -> impl ExactSizeIterator<Item = &ast::Module> + '_ {
        T::modules(self)
    }

    fn dependencies(&self) -> &[LibraryNamespace] {
        T::dependencies(self)
    }

    fn get_module(&self, path: &LibraryPath) -> Option<&ast::Module> {
        T::get_module(self, path)
    }
}
