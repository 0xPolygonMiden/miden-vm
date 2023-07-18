use super::{
    ast::{AstSerdeOptions, ModuleAst},
    ByteReader, ByteWriter, Deserializable, DeserializationError, LibraryError, PathError,
    Serializable, String, ToString, Vec, MAX_LABEL_LEN, NAMESPACE_LABEL_PARSER,
};
use core::{cmp::Ordering, fmt, ops::Deref, str::from_utf8};

mod masl;
pub use masl::MaslLibrary;

mod path;
pub use path::LibraryPath;

#[cfg(test)]
mod tests;

/// Maximum number of modules in a library.
const MAX_MODULES: usize = u16::MAX as usize;

/// Maximum number of dependencies in a library.
const MAX_DEPENDENCIES: usize = u16::MAX as usize;

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

    /// Returns the dependency libraries of this library.
    fn dependencies(&self) -> &[LibraryNamespace];
}

impl<T> Library for &T
where
    T: Library,
{
    type ModuleIterator<'a> = T::ModuleIterator<'a>
    where
        Self: 'a;

    fn root_ns(&self) -> &LibraryNamespace {
        T::root_ns(self)
    }

    fn version(&self) -> &Version {
        T::version(self)
    }

    fn modules(&self) -> Self::ModuleIterator<'_> {
        T::modules(self)
    }

    fn dependencies(&self) -> &[LibraryNamespace] {
        T::dependencies(self)
    }
}

// MODULE
// ================================================================================================

/// A module containing its absolute path and parsed AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    /// Absolute path of the module.
    pub path: LibraryPath,
    /// Parsed AST of the module.
    pub ast: ModuleAst,
}

impl Module {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Create a new module from a path and ast.
    pub const fn new(path: LibraryPath, ast: ModuleAst) -> Self {
        Self { path, ast }
    }

    /// Create a new kernel module from a AST using the constant [`LibraryPath::kernel_path`].
    pub fn kernel(ast: ModuleAst) -> Self {
        Self {
            path: LibraryPath::kernel_path(),
            ast,
        }
    }

    // VALIDATIONS
    // --------------------------------------------------------------------------------------------

    /// Validate if the module belongs to the provided namespace.
    pub fn check_namespace(&self, namespace: &LibraryNamespace) -> Result<(), LibraryError> {
        (self.path.first() == namespace.as_str()).then_some(()).ok_or_else(|| {
            LibraryError::inconsistent_namespace(self.path.first(), namespace.as_str())
        })
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Clears the source locations from this module.
    pub fn clear_locations(&mut self) {
        self.ast.clear_locations()
    }

    // SERIALIZATION / DESERIALIZATION
    // --------------------------------------------------------------------------------------------

    /// Loads the [SourceLocation] of the procedures via [ModuleAst::load_source_locations].
    pub fn load_source_locations<R: ByteReader>(
        &mut self,
        source: &mut R,
    ) -> Result<(), DeserializationError> {
        self.ast.load_source_locations(source)
    }

    /// Writes the [SourceLocation] of the procedures via [ModuleAst::write_source_locations].
    pub fn write_source_locations<W: ByteWriter>(&self, target: &mut W) {
        self.ast.write_source_locations(target)
    }

    /// Serialization of [Module] via [LibraryPath::write_into] and
    /// [ModuleAst::write_into]. [AstSerdeOptions] are used to direct serialization of [ModuleAst].
    pub fn write_into<W: ByteWriter>(&self, target: &mut W, options: AstSerdeOptions) {
        self.path.write_into(target);
        self.ast.write_into(target, options);
    }

    /// Deserialization of [Module] via [LibraryPath::read_from] and
    /// [ModuleAst::read_from]. [AstSerdeOptions] are used to direct deserialization of [ModuleAst].
    pub fn read_from<R: ByteReader>(
        source: &mut R,
        options: AstSerdeOptions,
    ) -> Result<Self, DeserializationError> {
        let path = LibraryPath::read_from(source)?;
        let ast = ModuleAst::read_from(source, options)?;
        Ok(Self { path, ast })
    }
}

impl PartialOrd for Module {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Module {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
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
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u16(self.major);
        target.write_u16(self.minor);
        target.write_u16(self.patch);
    }
}

impl Deserializable for Version {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let major = source.read_u16()?;
        let minor = source.read_u16()?;
        let patch = source.read_u16()?;
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
    type Error = LibraryError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut components = value.split('.');

        let major = components
            .next()
            .ok_or(LibraryError::missing_version_component(value, "major"))?
            .parse::<u16>()
            .map_err(|err| LibraryError::invalid_version_number(value, err.to_string()))?;
        let minor = components
            .next()
            .ok_or(LibraryError::missing_version_component(value, "minor"))?
            .parse::<u16>()
            .map_err(|err| LibraryError::invalid_version_number(value, err.to_string()))?;
        let patch = components
            .next()
            .ok_or(LibraryError::missing_version_component(value, "patch"))?
            .parse::<u16>()
            .map_err(|err| LibraryError::invalid_version_number(value, err.to_string()))?;

        if components.next().is_some() {
            Err(LibraryError::too_many_version_components(value))
        } else {
            Ok(Self {
                major,
                minor,
                patch,
            })
        }
    }
}

// LIBRARY NAMESPACE
// ================================================================================================

/// Library namespace.
///
/// Will be `std` in the absolute procedure name `std::foo::bar::baz`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LibraryNamespace {
    name: String,
}

impl LibraryNamespace {
    /// Returns an new [LibraryNamespace] instantiated from the provided source.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The source string is empty or requires more than 255 bytes to serialize.
    /// - Does not start with a ASCII letter.
    /// - Contains characters other than ASCII letters, numbers, and underscores.
    pub fn new<S>(source: S) -> Result<Self, LibraryError>
    where
        S: AsRef<str>,
    {
        let name = NAMESPACE_LABEL_PARSER
            .parse_label(source.as_ref())
            .map_err(LibraryError::invalid_namespace)?;
        Ok(Self {
            name: name.to_string(),
        })
    }
}

impl TryFrom<String> for LibraryNamespace {
    type Error = LibraryError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        Self::new(name)
    }
}

impl Deref for LibraryNamespace {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

impl AsRef<str> for LibraryNamespace {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl Serializable for LibraryNamespace {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        // this assertion should pass because library namespace constructor enforces max allowed
        // length at 255 bytes
        debug_assert!(self.name.len() <= u8::MAX as usize, "namespace too long");
        target.write_u8(self.name.len() as u8);
        target.write_bytes(self.name.as_bytes());
    }
}

impl Deserializable for LibraryNamespace {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let num_bytes = source.read_u8()? as usize;
        let name = source.read_vec(num_bytes)?;
        let name =
            from_utf8(&name).map_err(|e| DeserializationError::InvalidValue(e.to_string()))?;
        Self::new(name).map_err(|e| DeserializationError::InvalidValue(e.to_string()))
    }
}
