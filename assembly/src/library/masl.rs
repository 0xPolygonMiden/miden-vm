use super::{Library, LibraryNamespace, Version, MAX_DEPENDENCIES, MAX_MODULES};
use crate::{
    ast::{self, AstSerdeOptions},
    ByteReader, ByteWriter, Deserializable, DeserializationError, LibraryError, Serializable,
};
use alloc::{collections::BTreeSet, sync::Arc, vec::Vec};

/// Serialization options for [ModuleAst]. Imports and information about imported procedures are
/// part of the ModuleAst serialization by default.
const AST_DEFAULT_SERDE_OPTIONS: AstSerdeOptions = AstSerdeOptions {
    serialize_imports: true,
    debug_info: true,
};

/// A concrete implementation of the Library trait. Contains the minimal attributes of a functional
/// library.
///
/// Implementers of the library trait should use this base type to perform serialization into
/// `masl` files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaslLibrary {
    /// Root namespace of the library.
    namespace: LibraryNamespace,
    /// Version of the library.
    version: Version,
    /// Available modules.
    modules: Vec<Arc<ast::Module>>,
    /// Dependencies of the library.
    dependencies: Vec<LibraryNamespace>,
}

impl Library for MaslLibrary {
    fn root_ns(&self) -> &LibraryNamespace {
        &self.namespace
    }

    fn version(&self) -> &Version {
        &self.version
    }

    fn modules(&self) -> impl ExactSizeIterator<Item = &ast::Module> + '_ {
        self.modules.iter().map(|m| m.as_ref())
    }

    fn dependencies(&self) -> &[LibraryNamespace] {
        &self.dependencies
    }
}

impl MaslLibrary {
    /// File extension for the Assembly Library.
    pub const LIBRARY_EXTENSION: &'static str = "masl";

    /// File extension for the Assembly Module.
    pub const MODULE_EXTENSION: &'static str = "masm";

    /// Name of the root module.
    pub const MOD: &'static str = "mod";

    /// Returns a new [Library] instantiated from the specified parameters.
    ///
    /// # Errors
    /// Returns an error if the provided `modules` vector is empty or contains more than
    /// [u16::MAX] elements.
    pub fn new<I, M>(
        namespace: LibraryNamespace,
        version: Version,
        modules: I,
        dependencies: Vec<LibraryNamespace>,
    ) -> Result<Self, LibraryError>
    where
        I: IntoIterator<Item = M>,
        Arc<ast::Module>: From<M>,
    {
        let modules = modules.into_iter().map(Arc::from).collect::<Vec<_>>();
        let library = Self {
            namespace,
            version,
            modules,
            dependencies,
        };

        library.validate()?;

        Ok(library)
    }

    fn validate(&self) -> Result<(), LibraryError> {
        if self.modules.is_empty() {
            return Err(LibraryError::Empty(self.namespace.clone()));
        }
        if self.modules.len() > MAX_MODULES {
            return Err(LibraryError::TooManyModulesInLibrary {
                name: self.namespace.clone(),
                count: self.modules.len(),
                max: MAX_MODULES,
            });
        }

        if self.dependencies.len() > MAX_DEPENDENCIES {
            return Err(LibraryError::TooManyDependenciesInLibrary {
                name: self.namespace.clone(),
                count: self.dependencies.len(),
                max: MAX_DEPENDENCIES,
            });
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
mod use_std {
    use super::*;
    use crate::{
        ast::{instrument, ModuleKind},
        diagnostics::{IntoDiagnostic, Report},
    };
    use std::{collections::BTreeMap, fs, io, path::Path};

    impl MaslLibrary {
        /// Read a directory and recursively create modules from its `masm` files.
        ///
        /// For every directory, concatenate the module path with the dir name and proceed.
        ///
        /// For every file, pick and parse the ones with `masm` extension; skip otherwise.
        ///
        /// Example:
        ///
        /// - ./sys.masm            -> ("sys",          ast(./sys.masm))
        /// - ./crypto/hash.masm    -> ("crypto::hash", ast(./crypto/hash.masm))
        /// - ./math/u32.masm       -> ("math::u32",    ast(./math/u32.masm))
        /// - ./math/u64.masm       -> ("math::u64",    ast(./math/u64.masm))
        pub fn read_from_dir<P>(
            path: P,
            namespace: LibraryNamespace,
            version: Version,
        ) -> Result<Self, Report>
        where
            P: AsRef<Path>,
        {
            let path = path.as_ref();
            if !path.is_dir() {
                return Err(Report::msg(format!(
                    "the provided path '{}' is not a valid directory",
                    path.display()
                )));
            }

            // mod.masm is not allowed in the root directory
            if path.join("mod.masm").exists() {
                return Err(Report::msg("mod.masm is not allowed in the root directory"));
            }

            let library = Self {
                namespace,
                version,
                modules: Default::default(),
                dependencies: Default::default(),
            };
            library.load_modules_from_dir(path)
        }

        /// Read a library from a file.
        #[instrument(name = "read_library_file", fields(path = %path.as_ref().display()))]
        pub fn read_from_file<P>(path: P) -> Result<Self, LibraryError>
        where
            P: AsRef<Path>,
        {
            use vm_core::utils::ReadAdapter;

            let path = path.as_ref();
            let mut file = fs::File::open(path)?;
            let mut adapter = ReadAdapter::new(&mut file);
            <Self as Deserializable>::read_from(&mut adapter).map_err(|error| {
                LibraryError::DeserializationFailed {
                    path: path.to_string_lossy().into_owned(),
                    error,
                }
            })
        }

        /// Write the library to a target director, using its namespace as file name and the
        /// appropriate extension.
        pub fn write_to_dir<P>(&self, dir_path: P) -> io::Result<()>
        where
            P: AsRef<Path>,
        {
            fs::create_dir_all(&dir_path)?;
            let mut path = dir_path.as_ref().join(self.namespace.as_ref());
            path.set_extension(Self::LIBRARY_EXTENSION);

            // NOTE: We catch panics due to i/o errors here due to the fact
            // that the ByteWriter trait does not provide fallible APIs, so
            // WriteAdapter will panic if the underlying writes fail. This
            // needs to be addressed in winterfall at some point
            std::panic::catch_unwind(|| {
                let mut file = fs::File::create(path)?;
                self.write_into(&mut file);
                Ok(())
            })
            .map_err(|p| {
                match p.downcast::<io::Error>() {
                    // SAFETY: It is guaranteed safe to read Box<std::io::Error>
                    Ok(err) => unsafe { core::ptr::read(&*err) },
                    Err(err) => std::panic::resume_unwind(err),
                }
            })?
        }

        /// Read the contents (modules) of this library from `dir`, returning any errors
        /// that occur while traversing the file system.
        ///
        /// Errors may also be returned if traversal discovers issues with the library,
        /// such as invalid names, etc.
        ///
        /// Returns the set of modules that were parsed
        fn load_modules_from_dir(mut self, dir: &Path) -> Result<Self, Report> {
            use crate::diagnostics::WrapErr;
            use alloc::collections::btree_map::Entry;

            let mut modules = BTreeMap::default();
            let mut dependencies = BTreeSet::default();

            let walker = WalkLibrary::new(self.namespace.clone(), dir)
                .into_diagnostic()
                .wrap_err_with(|| format!("failed to load library from '{}'", dir.display()))?;
            for entry in walker {
                let LibraryEntry {
                    mut name,
                    source_path,
                } = entry?;
                if name.last() == MaslLibrary::MOD {
                    name.pop();
                }
                // Parse module at the given path
                let ast = ast::Module::parse_file(name.clone(), ModuleKind::Library, &source_path)?;
                // Add dependencies of this module to the global set
                for path in ast.import_paths() {
                    let ns = path.namespace();
                    if ns != &self.namespace {
                        dependencies.insert(ns.clone());
                    }
                }
                match modules.entry(name) {
                    Entry::Occupied(ref entry) => {
                        return Err(LibraryError::DuplicateModulePath(entry.key().clone()))
                            .into_diagnostic();
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(Arc::from(ast));
                    }
                }
            }

            self.modules.extend(modules.into_values());
            self.dependencies.extend(dependencies);

            self.validate()?;

            Ok(self)
        }
    }
}

#[cfg(feature = "std")]
struct LibraryEntry {
    name: super::LibraryPath,
    source_path: std::path::PathBuf,
}

#[cfg(feature = "std")]
struct WalkLibrary<'a> {
    namespace: LibraryNamespace,
    root: &'a std::path::Path,
    stack: alloc::collections::VecDeque<std::io::Result<std::fs::DirEntry>>,
}

#[cfg(feature = "std")]
impl<'a> WalkLibrary<'a> {
    fn new(namespace: LibraryNamespace, path: &'a std::path::Path) -> std::io::Result<Self> {
        use alloc::collections::VecDeque;

        let stack = VecDeque::from_iter(std::fs::read_dir(path)?);

        Ok(Self {
            namespace,
            root: path,
            stack,
        })
    }

    fn next_entry(
        &mut self,
        entry: &std::fs::DirEntry,
        ty: &std::fs::FileType,
    ) -> Result<Option<LibraryEntry>, crate::diagnostics::Report> {
        use crate::{
            diagnostics::{IntoDiagnostic, Report},
            LibraryPath,
        };
        use std::{ffi::OsStr, fs};

        if ty.is_dir() {
            let dir = entry.path();
            self.stack.extend(fs::read_dir(dir).into_diagnostic()?);
            return Ok(None);
        }

        let mut file_path = entry.path();
        let is_module = file_path
            .extension()
            .map(|ext| ext == AsRef::<OsStr>::as_ref(MaslLibrary::MODULE_EXTENSION))
            .unwrap_or(false);
        if !is_module {
            return Ok(None);
        }

        // Remove the file extension, and the root prefix, leaving us
        // with a namespace-relative path
        file_path.set_extension("");
        if file_path.is_dir() {
            return Err(Report::msg(format!(
                "file and directory with same name are not allowed: {}",
                file_path.display()
            )));
        }
        let relative_path = file_path
            .strip_prefix(self.root)
            .expect("expected path to be a child of the root directory");

        // Construct a [LibraryPath] from the path components, after validating them
        let mut libpath = LibraryPath::from(self.namespace.clone());
        for component in relative_path.iter() {
            let component = component.to_str().ok_or_else(|| {
                let p = entry.path();
                Report::msg(format!("{} is an invalid directory entry", p.display()))
            })?;
            libpath.push(component).into_diagnostic()?;
        }
        Ok(Some(LibraryEntry {
            name: libpath,
            source_path: entry.path(),
        }))
    }
}

#[cfg(feature = "std")]
impl<'a> Iterator for WalkLibrary<'a> {
    type Item = Result<LibraryEntry, crate::diagnostics::Report>;
    fn next(&mut self) -> Option<Self::Item> {
        use crate::diagnostics::IntoDiagnostic;
        loop {
            let entry = self
                .stack
                .pop_front()?
                .and_then(|entry| entry.file_type().map(|ft| (entry, ft)))
                .into_diagnostic();

            match entry {
                Ok((ref entry, ref file_type)) => {
                    match self.next_entry(entry, file_type).transpose() {
                        None => continue,
                        result => break result,
                    }
                }
                Err(err) => break Some(Err(err)),
            }
        }
    }
}

impl Serializable for MaslLibrary {
    #[inline]
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.write_into_with_options(target, AST_DEFAULT_SERDE_OPTIONS)
    }
}

/// Serialization
impl MaslLibrary {
    pub fn write_into_with_options<W: ByteWriter>(&self, target: &mut W, options: AstSerdeOptions) {
        self.namespace.write_into(target);
        self.version.write_into(target);

        let modules = self.modules();

        // write dependencies
        target.write_u16(self.dependencies.len() as u16);
        self.dependencies.iter().for_each(|dep| dep.write_into(target));

        // this assert is OK because maximum number of modules is enforced by Library constructor
        debug_assert!(modules.len() <= MAX_MODULES, "too many modules");

        target.write_u16(modules.len() as u16);
        modules.for_each(|module| {
            module.write_into_with_options(target, options);
        });
    }
}

impl Deserializable for MaslLibrary {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let namespace = LibraryNamespace::read_from(source)?;
        let version = Version::read_from(source)?;

        // read dependencies
        let num_deps = source.read_u16()? as usize;
        // TODO: check for duplicate/self-referential dependencies?
        let deps_set: BTreeSet<LibraryNamespace> = (0..num_deps)
            .map(|_| LibraryNamespace::read_from(source))
            .collect::<Result<_, _>>()?;

        // read modules
        let num_modules = source.read_u16()? as usize;
        let mut modules = Vec::with_capacity(num_modules);
        for _ in 0..num_modules {
            let ast = ast::Module::read_from(source)?;
            modules.push(ast);
        }

        let deps = deps_set.into_iter().collect();
        Self::new(namespace, version, modules, deps)
            .map_err(|err| DeserializationError::InvalidValue(format!("{err}")))
    }
}
