use crate::ast;

mod error;
mod masl;
mod namespace;
mod path;
mod version;

pub use self::error::LibraryError;
pub use self::masl::MaslLibrary;
pub use self::namespace::{LibraryNamespace, LibraryNamespaceError};
pub use self::path::{LibraryPath, LibraryPathComponent, PathError};
pub use self::version::{Version, VersionError};

#[cfg(test)]
mod tests;

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
