mod location;
mod selection;
mod source_file;
mod source_manager;
mod span;

use alloc::{string::String, sync::Arc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "std")]
pub use self::source_manager::SourceManagerExt;
pub use self::{
    location::{FileLineCol, Location},
    selection::{Position, Selection},
    source_file::{
        ByteIndex, ByteOffset, ColumnIndex, ColumnNumber, LineIndex, LineNumber, SourceContent,
        SourceContentUpdateError, SourceFile, SourceFileRef, SourceLanguage,
    },
    source_manager::{DefaultSourceManager, SourceId, SourceManager},
    span::{SourceSpan, Span, Spanned},
};

/// A [URI reference](https://datatracker.ietf.org/doc/html/rfc3986#section-4.1) that specifies
/// the location of a source file, whether on disk, on the network, or elsewhere.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uri(Arc<str>);

impl Uri {
    pub fn new(uri: impl AsRef<str>) -> Self {
        uri.as_ref().into()
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Returns the scheme portion of this URI, if present.
    pub fn scheme(&self) -> Option<&str> {
        match self.0.split_once(':') {
            Some((prefix, _))
                if prefix.contains(|c: char| {
                    !c.is_ascii_alphanumeric() && !matches!(c, '+' | '-' | '.')
                }) =>
            {
                None
            },
            Some((prefix, _)) => Some(prefix),
            None => None,
        }
    }

    /// Returns the scheme portion of this URI, if present.
    pub fn authority(&self) -> Option<&str> {
        let uri = self.0.as_ref();
        let (_, rest) = uri.split_once("//")?;
        match rest.split_once(['/', '?', '#']) {
            Some((authority, _)) => Some(authority),
            None => Some(rest),
        }
    }

    /// Returns the path portion of this URI.
    pub fn path(&self) -> &str {
        let uri = self.0.as_ref();
        let path = match uri.split_once("//") {
            Some((_, rest)) => match rest.split_once('/') {
                Some((_, path)) => path,
                None => return "",
            },
            None => match uri.split_once(':') {
                Some((prefix, _))
                    if prefix.contains(|c: char| {
                        !c.is_ascii_alphanumeric() && !matches!(c, '+' | '-' | '.')
                    }) =>
                {
                    uri
                },
                Some((_, path)) => path,
                None => uri,
            },
        };
        match path.split_once(['?', '#']) {
            Some((path, _)) => path,
            None => path,
        }
    }
}

impl core::fmt::Display for Uri {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}

impl AsRef<str> for Uri {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<&str> for Uri {
    #[inline]
    fn from(value: &str) -> Self {
        use alloc::string::ToString;

        value.to_string().into()
    }
}

impl From<Uri> for Arc<str> {
    fn from(value: Uri) -> Self {
        value.0
    }
}

impl From<Arc<str>> for Uri {
    #[inline]
    fn from(uri: Arc<str>) -> Self {
        Self(uri)
    }
}

impl From<alloc::boxed::Box<str>> for Uri {
    #[inline]
    fn from(uri: alloc::boxed::Box<str>) -> Self {
        Self(uri.into())
    }
}

impl From<String> for Uri {
    #[inline]
    fn from(uri: String) -> Self {
        Self(uri.into_boxed_str().into())
    }
}

#[cfg(feature = "std")]
impl<'a> From<&'a std::path::Path> for Uri {
    fn from(path: &'a std::path::Path) -> Self {
        use alloc::string::ToString;

        Self::from(path.display().to_string())
    }
}

impl core::str::FromStr for Uri {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}
