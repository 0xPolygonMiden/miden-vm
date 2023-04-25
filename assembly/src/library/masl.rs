use super::{
    ByteReader, ByteWriter, Deserializable, DeserializationError, Library, LibraryError,
    LibraryNamespace, LibraryPath, Module, ModuleAst, Serializable, Vec, Version,
};
use core::slice::Iter;

// LIBRARY IMPLEMENTATION FOR MASL FILES
// ================================================================================================

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
    modules: Vec<Module>,
}

impl Library for MaslLibrary {
    type ModuleIterator<'a> = Iter<'a, Module>;

    fn root_ns(&self) -> &LibraryNamespace {
        &self.namespace
    }

    fn version(&self) -> &Version {
        &self.version
    }

    fn modules(&self) -> Self::ModuleIterator<'_> {
        self.modules.iter()
    }
}

impl MaslLibrary {
    /// File extension for the Assembly Library.
    pub const LIBRARY_EXTENSION: &str = "masl";
    /// File extension for the Assembly Module.
    pub const MODULE_EXTENSION: &str = "masm";

    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [Library] instantiated from the specified parameters.
    ///
    /// # Errors
    /// Returns an error if the provided `modules` vector is empty or contains more than
    /// [u16::MAX] elements.
    fn new(
        namespace: LibraryNamespace,
        version: Version,
        modules: Vec<Module>,
    ) -> Result<Self, LibraryError> {
        if modules.is_empty() {
            return Err(LibraryError::no_modules_in_library(namespace));
        } else if modules.len() > u16::MAX as usize {
            return Err(LibraryError::too_many_modules_in_library(
                namespace,
                modules.len(),
                u16::MAX as usize,
            ));
        }

        Ok(Self {
            namespace,
            version,
            modules,
        })
    }
}

#[cfg(feature = "std")]
mod use_std {
    use super::*;
    use crate::{parse_module, BTreeMap};
    use std::{fs, io, path::Path};

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
        ) -> io::Result<Self>
        where
            P: AsRef<Path>,
        {
            if !path.as_ref().is_dir() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "the provided path '{}' isn't a valid directory",
                        path.as_ref().display()
                    ),
                ));
            }

            let module_path = LibraryPath::new(&namespace)
                .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{err}")))?;
            let modules = read_from_dir_helper(Default::default(), path, &module_path)?
                .into_iter()
                .map(|(path, ast)| Module { path, ast })
                .collect();

            Self::new(namespace, version, modules)
                .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{err}")))
        }

        /// Read a library from a file.
        pub fn read_from_file<P>(path: P) -> Result<MaslLibrary, LibraryError>
        where
            P: AsRef<Path>,
        {
            // convert path to str
            let path_str = path.as_ref().to_str().unwrap_or("path contains invalid unicode");

            // read bytes from file
            let contents =
                fs::read(&path).map_err(|e| LibraryError::file_error(path_str, &e.to_string()))?;

            // read library from bytes
            Self::read_from_bytes(&contents)
                .map_err(|e| LibraryError::deserialization_error(path_str, &e.to_string()))
        }

        /// Write the library to a target director, using its namespace as file name and the
        /// appropriate extension.
        pub fn write_to_dir<P>(&self, dir_path: P) -> io::Result<()>
        where
            P: AsRef<Path>,
        {
            fs::create_dir_all(&dir_path)?;
            let mut path = dir_path.as_ref().join(self.namespace.as_str());
            path.set_extension(Self::LIBRARY_EXTENSION);

            let bytes = self.to_bytes();
            fs::write(path, bytes)
        }
    }

    // HELPER FUNCTIONS
    // ============================================================================================

    /// Read a directory and recursively feed the state map with path->ast tuples.
    ///
    /// Helper for [`Self::read_from_dir`].
    fn read_from_dir_helper<P>(
        mut state: BTreeMap<LibraryPath, ModuleAst>,
        dir: P,
        module_path: &LibraryPath,
    ) -> io::Result<BTreeMap<LibraryPath, ModuleAst>>
    where
        P: AsRef<Path>,
    {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let ty = entry.file_type()?;

            // if dir, concatenate its name and perform recursion
            if ty.is_dir() {
                let path = entry.path();
                let name = path.file_name().and_then(|s| s.to_str()).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::Other, "invalid directory entry!")
                })?;
                let module_path = module_path
                    .append(name)
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{err}")))?;
                state = read_from_dir_helper(state, path, &module_path)?;
            // if file, check if `masm`, parse & append; skip otherwise
            } else if ty.is_file() {
                let path = entry.path();

                // extension is optional for the OS
                let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                if extension == MaslLibrary::MODULE_EXTENSION {
                    // the file has extension so it must have stem
                    let name = path.file_stem().and_then(|s| s.to_str()).ok_or_else(|| {
                        io::Error::new(io::ErrorKind::Other, "invalid directory entry!")
                    })?;

                    // read & parse file
                    let contents = fs::read_to_string(&path)?;
                    let ast = parse_module(&contents)?;

                    let module = module_path
                        .append(name)
                        .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{err}")))?;
                    if state.insert(module, ast).is_some() {
                        unreachable!(
                            "the filesystem is inconsistent as it produced duplicated module paths"
                        );
                    }
                }
            }
        }
        Ok(state)
    }
}

impl Serializable for MaslLibrary {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.namespace.write_into(target);
        self.version.write_into(target);

        let modules = self.modules();
        // this assert is OK because maximum number of modules is enforced by Library constructor
        debug_assert!(modules.len() <= u16::MAX as usize, "too many modules");

        target.write_u16(modules.len() as u16);
        modules.for_each(|module| {
            LibraryPath::strip_first(&module.path)
                .expect("module path consists of a single component")
                .write_into(target);
            module.ast.write_into(target);
        });
    }
}

impl Deserializable for MaslLibrary {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let namespace = LibraryNamespace::read_from(source)?;
        let version = Version::read_from(source)?;

        let num_modules = source.read_u16()? as usize;
        let mut modules = Vec::with_capacity(num_modules);
        for _ in 0..num_modules {
            let path = LibraryPath::read_from(source)?
                .prepend(&namespace)
                .map_err(|err| DeserializationError::InvalidValue(format!("{err}")))?;
            let ast = ModuleAst::read_from(source)?;
            modules.push(Module { path, ast });
        }

        Self::new(namespace, version, modules)
            .map_err(|err| DeserializationError::InvalidValue(format!("{err}")))
    }
}
