use alloc::{string::ToString, sync::Arc};
use core::{
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};

use crate::{
    ast::{AstSerdeOptions, CaseKindError, Ident, IdentError},
    diagnostics::{IntoDiagnostic, Report},
    ByteReader, ByteWriter, Deserializable, DeserializationError, LibraryPath, Serializable,
    SourceSpan, Span, Spanned,
};

/// Represents a fully-qualified procedure name, e.g. `std::math::u64::add`, parsed into it's
/// contituent [LibraryPath] and [ProcedureName] components.
#[derive(Clone)]
pub struct FullyQualifiedProcedureName {
    /// The source span associated with this identifier
    pub span: SourceSpan,
    /// The module path for this procedure
    pub module: LibraryPath,
    /// The name of the procedure
    pub name: ProcedureName,
}

impl FullyQualifiedProcedureName {
    /// Create a new [FullyQualifiedProcedureName] with the given fully-qualified module path
    /// and procedure name.
    pub fn new(module: LibraryPath, name: ProcedureName) -> Self {
        Self {
            span: SourceSpan::default(),
            module,
            name,
        }
    }
}

impl FromStr for FullyQualifiedProcedureName {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.rsplit_once("::") {
            None => Err(Report::msg("invalid fully-qualified procedure name, expected namespace")),
            Some((path, name)) => {
                let name = name.parse::<ProcedureName>().into_diagnostic()?;
                let path = path.parse::<LibraryPath>().into_diagnostic()?;
                Ok(Self::new(path, name))
            }
        }
    }
}

impl Eq for FullyQualifiedProcedureName {}

impl PartialEq for FullyQualifiedProcedureName {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.module == other.module
    }
}

impl Ord for FullyQualifiedProcedureName {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.module.cmp(&other.module).then_with(|| self.name.cmp(&other.name))
    }
}

impl PartialOrd for FullyQualifiedProcedureName {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<FullyQualifiedProcedureName> for miette::SourceSpan {
    fn from(fqn: FullyQualifiedProcedureName) -> Self {
        fqn.span.into()
    }
}

impl Spanned for FullyQualifiedProcedureName {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

impl fmt::Debug for FullyQualifiedProcedureName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FullyQualifiedProcedureName")
            .field("module", &self.module)
            .field("name", &self.name)
            .finish()
    }
}

impl fmt::Display for FullyQualifiedProcedureName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}::{}", &self.module, &self.name)
    }
}

/// Serialization
impl FullyQualifiedProcedureName {
    /// Serialize to `target` using `options`
    pub fn write_into_with_options<W: ByteWriter>(&self, target: &mut W, options: AstSerdeOptions) {
        if options.debug_info {
            self.span.write_into(target);
        }
        self.module.write_into(target);
        self.name.write_into_with_options(target, options);
    }

    /// Deserialize from `source` using `options`
    pub fn read_from_with_options<R: ByteReader>(
        source: &mut R,
        options: AstSerdeOptions,
    ) -> Result<Self, DeserializationError> {
        let span = if options.debug_info {
            SourceSpan::read_from(source)?
        } else {
            SourceSpan::default()
        };
        let module = LibraryPath::read_from(source)?;
        let name = ProcedureName::read_from_with_options(source, options)?;
        Ok(Self { span, module, name })
    }
}

/// Procedure name.
///
/// The symbol represented by this type must comply with the following rules:
///
/// - It must start with an ASCII alphabetic character, or one of: `_`, `.`, or `$`
/// - If it starts with a non-ASCII alphabetic character, it must contain at least one ASCII
///   alphabetic character, e.g. `_`, `$_` are not valid symbols, but `_a` or `$_a` are.
/// - Otherwise, the name may consist of any number of printable ASCII characters,
///   e.g. alphanumerics, punctuation, etc. Control characters and the like are explicitly
///   not allowed.
///
/// NOTE: In Miden Assembly source files, a procedure name must be quoted in double-quotes if it
/// contains any characters other than ASCII alphanumerics, `_`, or `::`. See examples
/// below.
///
/// ## Examples
///
/// ```masm,ignore
/// # All ASCII alphanumeric, bare identifier
/// proc.foo
///   ...
/// end
///
/// # All ASCII alphanumeric, namespaced
/// proc.std::foo
///   ...
/// end
///
/// # A variation of the first example with a leading `_`, but this is still allowed bare
/// proc._foo
///   ...
/// end
///
/// # A complex procedure name representing a monomorphized Rust function
/// # This name must be quoted in order to parse it correctly.
/// proc."alloc::alloc::box_free::<dyn alloc::boxed::FnBox<(), Output = ()>>"
///   ...
/// end
/// ```
#[derive(Debug, Clone)]
pub struct ProcedureName(Ident);

impl ProcedureName {
    /// Reserved name for a main procedure.
    pub const MAIN_PROC_NAME: &'static str = "#main";

    /// Create a [ProcedureName] from `name`
    pub fn new(name: impl AsRef<str>) -> Result<Self, IdentError> {
        name.as_ref().parse()
    }

    /// Create a [ProcedureName] from `name`
    pub fn new_with_span(span: SourceSpan, name: impl AsRef<str>) -> Result<Self, IdentError> {
        name.as_ref().parse::<Self>().map(|name| name.with_span(span))
    }

    /// Set the span for this [ProcedureName]
    pub fn with_span(self, span: SourceSpan) -> Self {
        Self(self.0.with_span(span))
    }

    /// Create a [ProcedureName] from its raw components
    ///
    /// It is expected that the caller has already validated that the
    /// name meets all validity criteria for procedure names, for example,
    /// the parser only lexes/parses valid identifiers, so by construction
    /// all such identifiers are valid.
    pub(crate) fn new_unchecked(name: Ident) -> Self {
        Self(name)
    }

    /// Obtain a procedure name representing the reserved name for the executable entrypoint (i.e. `main`)
    pub fn main() -> Self {
        let name = Arc::from(Self::MAIN_PROC_NAME.to_string().into_boxed_str());
        Self(Ident::new_unchecked(Span::unknown(name)))
    }

    /// Is this the reserved name for the executable entrypoint (i.e. `main`)?
    pub fn is_main(&self) -> bool {
        self.0.as_str() == Self::MAIN_PROC_NAME
    }
}

impl Eq for ProcedureName {}

impl PartialEq for ProcedureName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for ProcedureName {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for ProcedureName {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for ProcedureName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Spanned for ProcedureName {
    fn span(&self) -> SourceSpan {
        self.0.span()
    }
}

impl From<ProcedureName> for miette::SourceSpan {
    fn from(name: ProcedureName) -> Self {
        name.span().into()
    }
}

impl core::ops::Deref for ProcedureName {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl AsRef<Ident> for ProcedureName {
    #[inline(always)]
    fn as_ref(&self) -> &Ident {
        &self.0
    }
}

impl AsRef<str> for ProcedureName {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl PartialEq<str> for ProcedureName {
    fn eq(&self, other: &str) -> bool {
        self.0.as_ref() == other
    }
}

impl PartialEq<Ident> for ProcedureName {
    fn eq(&self, other: &Ident) -> bool {
        &self.0 == other
    }
}

impl fmt::Display for ProcedureName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Parsing
impl FromStr for ProcedureName {
    type Err = IdentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.char_indices();
        let raw = match chars.next() {
            None => Err(IdentError::Empty),
            Some((_, '"')) => loop {
                if let Some((pos, c)) = chars.next() {
                    match c {
                        '"' => {
                            if chars.next().is_some() {
                                break Err(IdentError::InvalidChars);
                            }
                            let tok = &s[1..pos];
                            break Ok(Arc::from(tok.to_string().into_boxed_str()));
                        }
                        c if c.is_alphanumeric() => continue,
                        '_' | '$' | '-' | '!' | '?' => continue,
                        _ => break Err(IdentError::InvalidChars),
                    }
                } else {
                    break Err(IdentError::InvalidChars);
                }
            },
            Some((_, c)) if c.is_ascii_lowercase() || c == '_' || c == '$' => {
                if chars.as_str().contains(|c| match c {
                    c if c.is_ascii_alphanumeric() => false,
                    '_' | '$' => false,
                    _ => true,
                }) {
                    Err(IdentError::InvalidChars)
                } else {
                    Ok(Arc::from(s.to_string().into_boxed_str()))
                }
            }
            Some((_, c)) if c.is_ascii_uppercase() => Err(IdentError::Casing(CaseKindError::Snake)),
            Some(_) => Err(IdentError::InvalidChars),
        }?;
        Ok(Self(Ident::new_unchecked(Span::unknown(raw))))
    }
}

/// Serialization
impl ProcedureName {
    pub fn write_into_with_options<W: ByteWriter>(
        &self,
        target: &mut W,
        options: crate::ast::AstSerdeOptions,
    ) {
        if options.debug_info {
            self.span().write_into(target);
        }
        target.write_usize(self.0.as_bytes().len());
        target.write_bytes(self.0.as_bytes());
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
        name.parse::<Self>()
            .map_err(|e| DeserializationError::InvalidValue(e.to_string()))
            .map(|id| id.with_span(span))
    }
}

impl Serializable for ProcedureName {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.write_into_with_options(target, Default::default())
    }
}

impl Deserializable for ProcedureName {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        Self::read_from_with_options(source, Default::default())
    }
}
