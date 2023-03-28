use super::{
    AbsolutePath, ByteReader, ByteWriter, Deserializable, LibraryError, LibraryNamespace,
    ModuleAst, ModulePath, Serializable, SerializationError, Vec,
};
use core::{cmp::Ordering, fmt, slice::Iter};

// LIBRARY
// ================================================================================================

/// A library definition that provides AST modules for the compilation process.
pub trait Library {
    /// The concrete type used to iterate the modules of the library.
    type ModuleIterator<'a>: Iterator<Item = &'a Module>
    where
        Self: 'a;

    /// Returns the root namespace of this library.
    fn root_ns(&self) -> &LibraryNamespace;

    /// Returns the version number of this library.
    fn version(&self) -> &Version;

    /// Iterate the modules available in the library.
    fn modules(&self) -> Self::ModuleIterator<'_>;
}

/// A concrete implementation of the Library trait. Contains the minimal attributes of a functional
/// library.
///
/// Implementers of the library trait should use this base type to perform serialization into
/// `masl` files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaslLibrary {
    /// Root namespace of the library.
    pub namespace: LibraryNamespace,
    /// Version of the library.
    pub version: Version,
    /// Available modules.
    pub modules: Vec<Module>,
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
    // CONSTANTS
    // --------------------------------------------------------------------------------------------

    /// File extension for the Assembly Library.
    pub const LIBRARY_EXTENSION: &str = "masl";
    /// File extension for the Assembly Module.
    pub const MODULE_EXTENSION: &str = "masm";
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

            let module = ModulePath::empty();
            let modules = read_from_dir_helper(Default::default(), path, &module)?
                .into_iter()
                .map(|(path, ast)| {
                    let path = path.to_absolute(&namespace);
                    Module { path, ast }
                })
                .collect();

            Ok(Self {
                namespace,
                version,
                modules,
            })
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

        /// Write the library to a target dir, using its namespace as file name and the appropriate
        /// extension.
        pub fn write_to_dir<P>(&self, dir_path: P) -> io::Result<()>
        where
            P: AsRef<Path>,
        {
            fs::create_dir_all(&dir_path)?;
            let mut path = dir_path.as_ref().join(self.namespace.as_str());
            path.set_extension(Self::LIBRARY_EXTENSION);

            let bytes = self.to_bytes()?;
            fs::write(path, bytes)
        }
    }

    // MASL LIBRARY HELPERS
    // ================================================================================================

    /// Read a directory and recursively feed the state map with path->ast tuples.
    ///
    /// Helper for [`Self::read_from_dir`].
    fn read_from_dir_helper<P>(
        mut state: BTreeMap<ModulePath, ModuleAst>,
        dir: P,
        module: &ModulePath,
    ) -> io::Result<BTreeMap<ModulePath, ModuleAst>>
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
                let module = module.concatenate(name);
                state = read_from_dir_helper(state, path, &module)?;
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

                    let module = module.concatenate(name);
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
    fn write_into(&self, target: &mut ByteWriter) -> Result<(), SerializationError> {
        self.namespace.write_into(target)?;
        self.version.write_into(target)?;

        let mut modules = self.modules();
        target.write_len(modules.len())?;
        modules.try_for_each(|module| {
            ModulePath::strip_namespace(&module.path).write_into(target)?;
            module.ast.write_into(target)?;
            Ok(())
        })
    }
}

impl Deserializable for MaslLibrary {
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        let namespace = LibraryNamespace::read_from(bytes)?;
        let version = Version::read_from(bytes)?;

        let len = bytes.read_len()?;
        let modules = (0..len)
            .map(|_| {
                ModulePath::read_from(bytes)
                    .map(|path| path.to_absolute(&namespace))
                    .and_then(|path| ModuleAst::read_from(bytes).map(|ast| (path, ast)))
                    .map(|(path, ast)| Module { path, ast })
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            namespace,
            version,
            modules,
        })
    }
}

// MODULE
// ================================================================================================

/// A module containing its absolute path and parsed AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    /// Absolute path of the module.
    pub path: AbsolutePath,
    /// Parsed AST of the module.
    pub ast: ModuleAst,
}

impl Module {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Create a new module from a path and ast.
    pub const fn new(path: AbsolutePath, ast: ModuleAst) -> Self {
        Self { path, ast }
    }

    /// Create a new kernel module from a AST using the constant [`AbsolutePath::kernel_path`].
    pub fn kernel(ast: ModuleAst) -> Self {
        Self {
            path: AbsolutePath::kernel_path(),
            ast,
        }
    }

    // VALIDATIONS
    // --------------------------------------------------------------------------------------------

    /// Validate if the module belongs to the provided namespace.
    pub fn check_namespace(&self, namespace: &LibraryNamespace) -> Result<(), LibraryError> {
        (self.path.namespace() == namespace.as_str()).then_some(()).ok_or_else(|| {
            LibraryError::namespace_violation(self.path.namespace(), namespace.as_str())
        })
    }
}

impl PartialOrd for Module {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.path.partial_cmp(&other.path)
    }
}

impl Ord for Module {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}

impl Serializable for Module {
    fn write_into(&self, target: &mut ByteWriter) -> Result<(), SerializationError> {
        self.path.write_into(target)?;
        self.ast.write_into(target)?;
        Ok(())
    }
}

impl Deserializable for Module {
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        let path = AbsolutePath::read_from(bytes)?;
        let ast = ModuleAst::read_from(bytes)?;
        Ok(Self { path, ast })
    }
}

// VERSION
// ================================================================================================

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Version {
    // CONSTANTS
    // --------------------------------------------------------------------------------------------

    /// Minimal version
    pub const MIN: Self = Self {
        major: 0,
        minor: 1,
        patch: 0,
    };

    // COMPARISON HELPERS
    // --------------------------------------------------------------------------------------------

    /// Compare two versions considering only the major value.
    pub fn cmp_major(&self, other: &Self) -> Ordering {
        self.major.cmp(&other.major)
    }

    /// Compare two versions considering only the major and minor values.
    pub fn cmp_minor(&self, other: &Self) -> Ordering {
        match self.cmp_major(other) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.minor.cmp(&other.minor),
            Ordering::Greater => Ordering::Greater,
        }
    }

    /// Compare two versions considering the major, minor, and patch values.
    pub fn cmp_patch(&self, other: &Self) -> Ordering {
        match self.cmp_minor(other) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.patch.cmp(&other.patch),
            Ordering::Greater => Ordering::Greater,
        }
    }

    // INCREMENT HELPERS
    // --------------------------------------------------------------------------------------------

    /// Increment the major version value.
    pub const fn inc_major(self) -> Self {
        Self {
            major: self.major + 1,
            minor: 0,
            patch: 0,
        }
    }

    /// Increment the minor version value.
    pub const fn inc_minor(self) -> Self {
        Self {
            major: self.major,
            minor: self.minor + 1,
            patch: 0,
        }
    }

    /// Increment the patch version value.
    pub const fn inc_patch(self) -> Self {
        Self {
            major: self.major,
            minor: self.minor,
            patch: self.patch + 1,
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::MIN
    }
}

impl Serializable for Version {
    fn write_into(&self, target: &mut ByteWriter) -> Result<(), SerializationError> {
        target.write_u16(self.major);
        target.write_u16(self.minor);
        target.write_u16(self.patch);
        Ok(())
    }
}

impl Deserializable for Version {
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        let major = bytes.read_u16()?;
        let minor = bytes.read_u16()?;
        let patch = bytes.read_u16()?;
        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl TryFrom<&str> for Version {
    type Error = SerializationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut tokens = value
            .split('.')
            .map(|n| n.parse::<u16>().map_err(|_| SerializationError::InvalidNumber));
        let major = tokens.next().transpose()?.ok_or(SerializationError::UnexpectedEndOfStream)?;
        let minor = tokens.next().transpose()?.ok_or(SerializationError::UnexpectedEndOfStream)?;
        let patch = tokens.next().transpose()?.ok_or(SerializationError::UnexpectedEndOfStream)?;
        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}
