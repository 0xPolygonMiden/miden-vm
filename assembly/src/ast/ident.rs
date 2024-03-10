use alloc::{string::ToString, sync::Arc};
use core::{
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};

use crate::{
    ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable, SourceSpan, Span,
    Spanned,
};

/// Represents the types of errors that can occur when parsing/validating an [Ident]
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum IdentError {
    #[error("invalid identifier: cannot be empty")]
    Empty,
    #[error("invalid identifier: must contain only lowercase, ascii alphanumeric characters, or underscores")]
    InvalidChars,
    #[error("invalid identifier: must start with lowercase ascii alphabetic character")]
    InvalidStart,
    #[error("invalid identifier: length exceeds the maximum of {max} bytes")]
    InvalidLength { max: usize },
    #[error("invalid identifier: {0}")]
    Casing(CaseKindError),
}

/// Represents the various types of casing errors that can occur, e.g. using an identifier
/// with `SCREAMING_CASE` where one with `snake_case` is expected.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CaseKindError {
    #[error("only uppercase characters or underscores are allowed, and must start with an alphabetic character")]
    Screaming,
    #[error("only lowercase characters or underscores are allowed, and must start with an alphabetic character")]
    Snake,
    #[error("only alphanumeric characters are allowed, and must start with a lowercase alphabetic character")]
    Camel,
}

/// Represents a generic identifier in Miden Assembly source code.
///
/// This type is used internally by all other specialized identifier types, e.g.
/// [super::ProcedureName], and enforces the baseline rules for bare identifiers in Miden Assembly.
/// Higher-level types, such as `ProcedureName`, can implement their own variations on these rules,
/// and construct an [Ident] using [Ident::new_unchecked].
///
/// All identifiers are associated with a source span, and are interned to the extent possible, i.e.
/// rather than allocating a new `String` for every use of the same identifier, we attempt to have
/// all such uses share a single reference-counted allocation. This interning is not perfect or
/// guaranteed globally, but generally holds within a given module. In the future we may make these
/// actually interned strings with a global interner, but for now it is simply best-effort.
#[derive(Clone)]
pub struct Ident {
    /// The source span associated with this identifier.
    ///
    /// NOTE: To make use of this span, we need to know the context in which it was
    /// used, i.e. either the containing module or procedure, both of which have a
    /// source file which we can use to render a source snippet for this span.
    ///
    /// If a span is not known, the default value is used, which has zero-length and
    /// thus will not be rendered as a source snippet.
    span: SourceSpan,
    /// The actual content of the identifier
    name: Arc<str>,
}
impl Ident {
    /// Parse an [Ident] from `source`
    pub fn new(source: impl AsRef<str>) -> Result<Self, IdentError> {
        source.as_ref().parse()
    }

    /// Parse an [Ident] from `source`
    pub fn new_with_span(span: SourceSpan, source: impl AsRef<str>) -> Result<Self, IdentError> {
        source.as_ref().parse::<Self>().map(|id| id.with_span(span))
    }

    /// Set the span for this identifier
    pub fn with_span(mut self, span: SourceSpan) -> Self {
        self.span = span;
        self
    }

    /// This allows constructing an [Ident] directly from a ref-counted
    /// string that is known to be a valid identifier, and so does not
    /// require re-parsing/re-validating. This must _not_ be used to
    /// bypass validation when you have an identifier that is not valid,
    /// and such identifiers will be caught during compilation and result
    /// in a panic being raised.
    pub(crate) fn new_unchecked(name: Span<Arc<str>>) -> Self {
        let (span, name) = name.into_parts();
        Self { span, name }
    }

    /// Unwrap this [Ident], extracting the inner [`Arc<str>`].
    pub fn into_inner(self) -> Arc<str> {
        self.name
    }

    /// Get the content of this identifier as a `str`
    pub fn as_str(&self) -> &str {
        self.name.as_ref()
    }

    /// Apply the default [Ident] validation rules to `source`
    pub fn validate(source: impl AsRef<str>) -> Result<(), IdentError> {
        let source = source.as_ref();
        if source.is_empty() {
            return Err(IdentError::Empty);
        }
        if !source.starts_with(|c: char| c.is_ascii_alphabetic()) {
            return Err(IdentError::InvalidStart);
        }
        if !source.chars().all(|c| c.is_ascii_alphabetic() || matches!(c, '_' | '0'..='9')) {
            return Err(IdentError::InvalidChars);
        }
        Ok(())
    }
}

impl fmt::Debug for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Ident").field(&self.name).finish()
    }
}

impl Eq for Ident {}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Ord for Ident {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Ident {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Ident {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Spanned for Ident {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

impl core::ops::Deref for Ident {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.name.as_ref()
    }
}

impl AsRef<str> for Ident {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.name, f)
    }
}

impl FromStr for Ident {
    type Err = IdentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::validate(s)?;
        let name = Arc::from(s.to_string().into_boxed_str());
        Ok(Self {
            span: SourceSpan::default(),
            name,
        })
    }
}

/// Serialization
impl Ident {
    pub fn write_into_with_options<W: ByteWriter>(
        &self,
        target: &mut W,
        options: crate::ast::AstSerdeOptions,
    ) {
        if options.debug_info {
            self.span.write_into(target);
        }
        target.write_usize(self.name.as_bytes().len());
        target.write_bytes(self.name.as_bytes());
    }

    pub fn read_from_with_options<R: ByteReader>(
        source: &mut R,
        options: crate::ast::AstSerdeOptions,
    ) -> Result<Self, DeserializationError> {
        let span = if options.debug_info {
            SourceSpan::read_from(source)?
        } else {
            SourceSpan::default()
        };
        let nlen = source.read_usize()?;
        let name = source.read_slice(nlen)?;
        let name = core::str::from_utf8(name)
            .map_err(|e| DeserializationError::InvalidValue(e.to_string()))?;
        name.parse::<Ident>()
            .map_err(|e| DeserializationError::InvalidValue(e.to_string()))
            .map(|id| id.with_span(span))
    }
}

impl Serializable for Ident {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.span.write_into(target);
        target.write_usize(self.name.as_bytes().len());
        target.write_bytes(self.name.as_bytes());
    }
}

impl Deserializable for Ident {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let span = SourceSpan::read_from(source)?;
        let nlen = source.read_usize()?;
        let name = source.read_slice(nlen)?;
        let name = core::str::from_utf8(name)
            .map_err(|e| DeserializationError::InvalidValue(e.to_string()))?;
        name.parse::<Ident>()
            .map_err(|e| DeserializationError::InvalidValue(e.to_string()))
            .map(|id| id.with_span(span))
    }
}
