use super::{
    ByteReader, ByteWriter, Deserializable, DeserializationError, PathError, Serializable, String,
    ToString, MAX_LABEL_LEN,
};
use core::{ops::Deref, str::from_utf8};

// CONSTANTS
// ================================================================================================

const MAX_PATH_LEN: usize = 1023;

// LIBRARY PATH
// ================================================================================================

/// Path to a module or a procedure.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LibraryPath {
    path: String,
    num_components: usize,
}

impl LibraryPath {
    // CONSTANTS
    // --------------------------------------------------------------------------------------------

    /// Path delimiter.
    pub const PATH_DELIM: &str = "::";

    /// Base kernel path.
    pub const KERNEL_PATH: &str = "#sys";

    /// Path for an executable module.
    pub const EXEC_PATH: &str = "#exec";

    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Returns a new path created from the provided source.
    ///
    /// A path consists of at list of components separated by `::` delimiter. A path must contain
    /// at least one component.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The path is empty.
    /// - The path requires more than 1KB to serialize.
    /// - Any of the path's components is empty, requires more than 255 bytes to serialize, does
    ///   not start with a letter, or contains non-alphanumeric characters.
    pub fn new<S>(source: S) -> Result<Self, PathError>
    where
        S: AsRef<str>,
    {
        Ok(Self {
            path: source.as_ref().to_string(),
            num_components: Self::validate(source)?,
        })
    }

    /// Returns a path for a kernel module.
    pub fn kernel_path() -> Self {
        Self {
            path: Self::KERNEL_PATH.into(),
            num_components: 1,
        }
    }

    /// Returns a path for an executable module.
    pub fn exec_path() -> Self {
        Self {
            path: Self::EXEC_PATH.into(),
            num_components: 1,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the first component of the path.
    ///
    /// The first component is the leftmost token separated by `::`.
    pub fn first(&self) -> &str {
        self.path
            .split_once(Self::PATH_DELIM)
            .expect("a valid library path must always have at least one component")
            .0
    }

    /// Returns the last component of the path.
    ///
    /// The last component is the rightmost token separated by `::`.
    pub fn last(&self) -> &str {
        self.path
            .rsplit_once(Self::PATH_DELIM)
            .expect("a valid library path must always have at least one component")
            .1
    }

    /// Returns the number of components in the path.
    ///
    /// This is guaranteed to return at least 1.
    pub fn num_components(&self) -> usize {
        self.num_components
    }

    /// Returns an iterator over all components of the path.
    pub fn components(&self) -> core::str::Split<&str> {
        self.path.split(Self::PATH_DELIM)
    }

    /// Returns true if this path is for a kernel module.
    pub fn is_kernel_path(&self) -> bool {
        self.path == Self::KERNEL_PATH
    }

    /// Returns true if this path is for an executable module.
    pub fn is_exec_path(&self) -> bool {
        self.path == Self::EXEC_PATH
    }

    // TYPE-SAFE TRANSFORMATION
    // --------------------------------------------------------------------------------------------

    /// Appends the provided path to this path and returns the result.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The joined path is either for a kernel or an executable module.
    /// - The resulting path requires more than 1KB to serialize.
    pub fn join(&self, other: &Self) -> Result<Self, PathError> {
        if other.path.starts_with(Self::KERNEL_PATH) || other.starts_with(Self::EXEC_PATH) {
            return Err(PathError::component_invalid_char(&other.path));
        }

        let new_path = format!("{}{}{}", self.path, Self::PATH_DELIM, other.path);
        validate_path_len(&new_path)?;
        Ok(Self {
            path: new_path,
            num_components: self.num_components + other.num_components,
        })
    }

    /// Adds the provided component to the end of this path and returns the result.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The resulting path is over 1000 characters long.
    /// - The component is empty, requires more than 255 bytes to serialize, does not start with
    ///   a letter, or contains non-alphanumeric characters.
    pub fn append<S>(&self, component: S) -> Result<Self, PathError>
    where
        S: AsRef<str>,
    {
        let new_path = format!("{}{}{}", self.path, Self::PATH_DELIM, component.as_ref());
        Self::new(new_path)
    }

    /// Adds the provided component to the front of this path and returns the result.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The resulting path is over 1000 characters long.
    /// - The component is empty, requires more than 255 bytes to serialize, does not start with
    ///   a letter, or contains non-alphanumeric characters.
    pub fn prepend<S>(&self, component: S) -> Result<Self, PathError>
    where
        S: AsRef<str>,
    {
        let new_path = format!("{}{}{}", component.as_ref(), Self::PATH_DELIM, self.path);
        Self::new(new_path)
    }

    /// Returns the path with the first component removed.
    ///
    /// # Errors
    /// Returns an error if the path consists of only one component.
    pub fn strip_first(&self) -> Result<Self, PathError> {
        if self.num_components == 1 {
            return Err(PathError::too_few_components(&self.path, 2));
        }

        let rem = self
            .path
            .split_once(Self::PATH_DELIM)
            .expect("failed to split path on module delimiter")
            .1;

        Ok(Self {
            path: rem.to_string(),
            num_components: self.num_components - 1,
        })
    }

    /// Returns the path with the last component removed.
    ///
    /// # Errors
    /// Returns an error if the path consists of only one component.
    pub fn strip_last(&self) -> Result<Self, PathError> {
        if self.num_components == 1 {
            return Err(PathError::too_few_components(&self.path, 2));
        }

        let rem = self
            .path
            .rsplit_once(Self::PATH_DELIM)
            .expect("failed to split path on module delimiter")
            .1;

        Ok(Self {
            path: rem.to_string(),
            num_components: self.num_components - 1,
        })
    }

    // UTILITY FUNCTIONS
    // --------------------------------------------------------------------------------------------

    /// Validates the specified path and returns the number of components in the path.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The path is empty.
    /// - The path requires more than 1KB to serialize.
    /// - Any of the path's components is empty, requires more than 255 bytes to serialize, does
    ///   not start with a letter, or contains non-alphanumeric characters.
    pub fn validate<S>(source: S) -> Result<usize, PathError>
    where
        S: AsRef<str>,
    {
        // make sure the path is not empty and is not over max length of 255 bytes
        if source.as_ref().is_empty() {
            return Err(PathError::EmptyPath);
        }
        validate_path_len(source.as_ref())?;

        // special handling of the first component as it may contain non-alphanumeric characters
        let (path, mut num_components) = if source.as_ref().starts_with(Self::KERNEL_PATH) {
            let split_at = Self::KERNEL_PATH.len() + Self::PATH_DELIM.len();
            (source.as_ref().split_at(split_at).1, 1)
        } else if source.as_ref().starts_with(Self::EXEC_PATH) {
            let split_at = Self::EXEC_PATH.len() + Self::PATH_DELIM.len();
            (source.as_ref().split_at(split_at).1, 1)
        } else {
            (source.as_ref(), 0)
        };

        // count the number of components in the path and make sure each component is valid
        for component in path.split(Self::PATH_DELIM) {
            validate_component(component)?;
            num_components += 1;
        }

        Ok(num_components)
    }

    /// Appends the specified component to the end of this path and returns the resulting string
    /// representation of the path.
    ///
    /// This does not check whether the component or the resulting path are valid.
    pub fn append_unchecked<S>(&self, component: S) -> String
    where
        S: AsRef<str>,
    {
        format!("{}{}{}", self.path, Self::PATH_DELIM, component.as_ref())
    }
}

impl Deref for LibraryPath {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl AsRef<str> for LibraryPath {
    fn as_ref(&self) -> &str {
        &self.path
    }
}

impl TryFrom<String> for LibraryPath {
    type Error = PathError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for LibraryPath {
    type Error = PathError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Serializable for LibraryPath {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        debug_assert!(self.path.len() < MAX_PATH_LEN, "path too long");
        target.write_u16(self.path.len() as u16);
        target.write_bytes(self.path.as_bytes());
    }
}

impl Deserializable for LibraryPath {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let path_len = source.read_u16()? as usize;
        let path = source.read_vec(path_len)?;
        let path =
            from_utf8(&path).map_err(|e| DeserializationError::InvalidValue(e.to_string()))?;
        Self::new(path).map_err(|e| DeserializationError::InvalidValue(e.to_string()))
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn validate_path_len(path: &str) -> Result<(), PathError> {
    if path.len() > MAX_PATH_LEN {
        Err(PathError::path_too_long(path, MAX_PATH_LEN))
    } else {
        Ok(())
    }
}

fn validate_component(component: &str) -> Result<(), PathError> {
    if component.is_empty() {
        Err(PathError::EmptyComponent)
    } else if component.len() > MAX_LABEL_LEN {
        Err(PathError::component_too_long(component, MAX_LABEL_LEN))
    } else if !component.chars().next().unwrap().is_ascii_alphabetic() {
        Err(PathError::component_invalid_first_char(component))
    } else if !component.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        Err(PathError::component_invalid_char(component))
    } else {
        Ok(())
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{LibraryPath, PathError};

    #[test]
    fn new_path() {
        let path = LibraryPath::new("foo").unwrap();
        assert_eq!(path.num_components(), 1);

        let path = LibraryPath::new("foo::bar").unwrap();
        assert_eq!(path.num_components(), 2);

        let path = LibraryPath::new("foo::bar::baz").unwrap();
        assert_eq!(path.num_components(), 3);

        let path = LibraryPath::new("#exec::bar::baz").unwrap();
        assert_eq!(path.num_components(), 3);

        let path = LibraryPath::new("#sys::bar::baz").unwrap();
        assert_eq!(path.num_components(), 3);
    }

    #[test]
    fn new_path_fail() {
        let path = LibraryPath::new("");
        assert!(matches!(path, Err(PathError::EmptyPath)));

        let path = LibraryPath::new("::");
        assert!(matches!(path, Err(PathError::EmptyComponent)));

        let path = LibraryPath::new("foo::");
        assert!(matches!(path, Err(PathError::EmptyComponent)));

        let path = LibraryPath::new("::foo");
        assert!(matches!(path, Err(PathError::EmptyComponent)));

        let path = LibraryPath::new("foo::1bar");
        assert!(matches!(path, Err(PathError::ComponentInvalidFirstChar { component: _ })));

        let path = LibraryPath::new("foo::b@r");
        assert!(matches!(path, Err(PathError::ComponentInvalidChar { component: _ })));

        let path = LibraryPath::new("#foo::bar");
        assert!(matches!(path, Err(PathError::ComponentInvalidFirstChar { component: _ })));
    }
}
