use super::{
    ByteReader, ByteWriter, Deserializable, DeserializationError, PathError, Serializable,
};
use core::{ops::Deref, str::from_utf8};

// CONSTANTS
// ================================================================================================

const MAX_PATH_LEN: usize = 1000;
const MAX_COMPONENT_LEN: usize = 100;

// LIBRARY PATH
// ================================================================================================

/// Path to a module or a procedure.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LibraryPath {
    path: String,
    num_components: usize,
}

impl LibraryPath {
    // CONSTANTS
    // --------------------------------------------------------------------------------------------

    /// Path delimiter.
    const MODULE_PATH_DELIM: &str = "::";

    /// Base kernel path.
    pub const KERNEL_PATH: &str = "$sys";

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
    /// - The path is over 1000 characters long.
    /// - Any of the path's components is empty, is over 100 characters long, does not start with
    ///   a letter, or contains non-alphanumeric characters.
    pub fn new<S>(source: S) -> Result<Self, PathError>
    where
        S: AsRef<str>,
    {
        // make sure the path is not empty and is not over max length of 1000 chars
        if source.as_ref().is_empty() {
            return Err(PathError::EmptyPath);
        }
        validate_path_len(source.as_ref())?;

        // count the number of components in the path and make sure each component is valid
        let mut num_components = 0;
        for component in source.as_ref().split(Self::MODULE_PATH_DELIM) {
            validate_component(component)?;
            num_components += 1;
        }

        Ok(Self {
            path: source.as_ref().to_string(),
            num_components,
        })
    }

    /// Returns a path for a kernel module.
    pub fn kernel_path() -> Self {
        Self {
            path: Self::KERNEL_PATH.into(),
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
            .split_once(Self::MODULE_PATH_DELIM)
            .expect("a valid library path must always have at least one component")
            .0
    }

    /// Returns the last component of the path.
    ///
    /// The last component is the rightmost token separated by `::`.
    pub fn last(&self) -> &str {
        self.path
            .rsplit_once(Self::MODULE_PATH_DELIM)
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
        self.path.split(Self::MODULE_PATH_DELIM)
    }

    // TYPE-SAFE TRANSFORMATION
    // --------------------------------------------------------------------------------------------

    /// Appends the provided path to this path and returns the result.
    ///
    /// # Errors
    /// Returns an error if the resulting path is longer than max path length of 1000 chars.
    pub fn join(&self, other: &Self) -> Result<Self, PathError> {
        let new_path = format!("{}{}{}", self.path, Self::MODULE_PATH_DELIM, other.path);
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
    /// - The components is empty, is over 100 characters long, does not start with a letter, or
    ///   contains non-alphanumeric characters.
    pub fn append<S>(&self, component: S) -> Result<Self, PathError>
    where
        S: AsRef<str>,
    {
        let new_path = format!("{}{}{}", self.path, Self::MODULE_PATH_DELIM, component.as_ref());
        Self::new(new_path)
    }

    /// Adds the provided component to the front of this path and returns the result.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The resulting path is over 1000 characters long.
    /// - The components is empty, is over 100 characters long, does not start with a letter, or
    ///   contains non-alphanumeric characters.
    pub fn prepend<S>(&self, component: S) -> Result<Self, PathError>
    where
        S: AsRef<str>,
    {
        let new_path = format!("{}{}{}", component.as_ref(), Self::MODULE_PATH_DELIM, self.path);
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
            .split_once(Self::MODULE_PATH_DELIM)
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
            .rsplit_once(Self::MODULE_PATH_DELIM)
            .expect("failed to split path on module delimiter")
            .1;

        Ok(Self {
            path: rem.to_string(),
            num_components: self.num_components - 1,
        })
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
        debug_assert!(self.path.len() < u16::MAX as usize, "path too long");
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
    } else if component.len() > MAX_COMPONENT_LEN {
        Err(PathError::component_too_long(component, MAX_COMPONENT_LEN))
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
    }
}
