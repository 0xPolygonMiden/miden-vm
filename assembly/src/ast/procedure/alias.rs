use alloc::{string::String, sync::Arc};
use core::fmt;

use super::{FullyQualifiedProcedureName, ProcedureName};
use crate::{
    ast::AstSerdeOptions, diagnostics::SourceFile, ByteReader, ByteWriter, DeserializationError,
    RpoDigest, SourceSpan, Span, Spanned,
};

// PROCEDURE ALIAS
// ================================================================================================

/// Represents a procedure that acts like it is locally-defined, but delegates to an externally-
/// defined procedure.
///
/// These procedure "aliases" do not have a concrete representation in the module, but are instead
/// resolved during compilation to refer directly to the aliased procedure, regardless of whether
/// the caller is in the current module, or in another module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcedureAlias {
    /// The source file in which this alias was defined, if available
    source_file: Option<Arc<SourceFile>>,
    /// The documentation attached to this procedure
    docs: Option<Span<String>>,
    /// The name of the re-exported procedure.
    name: ProcedureName,
    /// The fully-qualified name of the imported procedure
    ///
    /// NOTE: This is fully-qualified from the perspective of the containing [Module], but may not
    /// be fully-resolved to the concrete definition until compilation time.
    pub(crate) target: AliasTarget,
    /// If true, this alias was created with an absolute path, bypassing the need for an import
    absolute: bool,
}

impl ProcedureAlias {
    /// Creates a new procedure alias called `name`, which resolves to `target`.
    pub fn new(name: ProcedureName, target: impl Into<AliasTarget>, absolute: bool) -> Self {
        let target = target.into();
        // Ignore the absolute flag if the target is implicitly absolute
        let absolute = matches!(target, AliasTarget::MastRoot(_)) || absolute;
        Self {
            docs: None,
            source_file: None,
            name,
            target,
            absolute,
        }
    }

    /// Adds documentation to this procedure alias.
    pub fn with_docs(mut self, docs: Option<Span<String>>) -> Self {
        self.docs = docs;
        self
    }

    /// Adds source code to this declaration, so that we can render source snippets in diagnostics.
    pub fn with_source_file(mut self, source_file: Option<Arc<SourceFile>>) -> Self {
        self.source_file = source_file;
        self
    }

    /// Returns the source file associated with this declaration.
    pub fn source_file(&self) -> Option<Arc<SourceFile>> {
        self.source_file.clone()
    }

    /// Returns the documentation associated with this declaration.
    pub fn docs(&self) -> Option<&Span<String>> {
        self.docs.as_ref()
    }

    /// Returns the name of this alias within its containing module.
    ///
    /// If the procedure is simply re-exported with the same name, this will be equivalent to
    /// `self.target().name`
    pub fn name(&self) -> &ProcedureName {
        &self.name
    }

    /// Returns the target of this procedure alias
    pub fn target(&self) -> &AliasTarget {
        &self.target
    }

    /// Returns true if this procedure uses an absolute target path
    #[inline]
    pub fn absolute(&self) -> bool {
        self.absolute
    }
}

/// Serialization
impl ProcedureAlias {
    pub fn write_into_with_options<W: ByteWriter>(&self, target: &mut W, options: AstSerdeOptions) {
        self.name.write_into_with_options(target, options);
        self.target.write_into_with_options(target, options);
        target.write_bool(self.absolute);
    }

    pub fn read_from_with_options<R: ByteReader>(
        source: &mut R,
        options: AstSerdeOptions,
    ) -> Result<Self, DeserializationError> {
        let name = ProcedureName::read_from_with_options(source, options)?;
        let target = AliasTarget::read_from_with_options(source, options)?;
        let absolute = source.read_bool()?;
        Ok(Self {
            source_file: None,
            docs: None,
            name,
            target,
            absolute,
        })
    }
}

impl Spanned for ProcedureAlias {
    fn span(&self) -> SourceSpan {
        self.target.span()
    }
}

impl crate::prettier::PrettyPrint for ProcedureAlias {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;

        let mut doc = Document::Empty;
        if let Some(docs) = self.docs.as_deref() {
            doc = docs
                .lines()
                .map(text)
                .reduce(|acc, line| acc + nl() + text("#! ") + line)
                .unwrap_or_default();
        }

        doc += const_text("export.");
        doc += match &self.target {
            target @ AliasTarget::MastRoot(_) => display(format_args!("{}->{}", target, self.name)),
            AliasTarget::Path(name) => {
                let prefix = if self.absolute { "::" } else { "" };
                if name.name == self.name {
                    display(format_args!("{}{}", prefix, name))
                } else {
                    display(format_args!("{}{}->{}", prefix, name, &self.name))
                }
            }
        };
        doc
    }
}

/// A fully-qualified external procedure that is the target of a procedure alias
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AliasTarget {
    /// An alias of a procedure with the given digest
    MastRoot(Span<RpoDigest>),
    /// An alias of a procedure with the given fully-qualified path
    Path(FullyQualifiedProcedureName),
}

impl Spanned for AliasTarget {
    fn span(&self) -> SourceSpan {
        match self {
            Self::MastRoot(spanned) => spanned.span(),
            Self::Path(spanned) => spanned.span(),
        }
    }
}

impl From<Span<RpoDigest>> for AliasTarget {
    fn from(digest: Span<RpoDigest>) -> Self {
        Self::MastRoot(digest)
    }
}

impl From<FullyQualifiedProcedureName> for AliasTarget {
    fn from(path: FullyQualifiedProcedureName) -> Self {
        Self::Path(path)
    }
}

impl crate::prettier::PrettyPrint for AliasTarget {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;
        use vm_core::utils::DisplayHex;

        match self {
            Self::MastRoot(digest) => display(DisplayHex(digest.as_bytes().as_slice())),
            Self::Path(path) => display(format_args!("{}", path)),
        }
    }
}

impl fmt::Display for AliasTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::prettier::PrettyPrint;

        self.pretty_print(f)
    }
}

impl AliasTarget {
    fn tag(&self) -> u8 {
        // SAFETY: This is safe because we have given this enum a primitive representation with
        // #[repr(u8)], with the first field of the underlying union-of-structs the discriminant.
        //
        // See the section on "accessing the numeric value of the discriminant"
        // here: https://doc.rust-lang.org/std/mem/fn.discriminant.html
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    /// Serialize to `target` using `options`
    pub fn write_into_with_options<W: ByteWriter>(&self, target: &mut W, options: AstSerdeOptions) {
        target.write_u8(self.tag());
        match self {
            Self::MastRoot(spanned) => spanned.write_into(target, options),
            Self::Path(path) => path.write_into_with_options(target, options),
        }
    }

    /// Deserialize from `source` using `options`
    pub fn read_from_with_options<R: ByteReader>(
        source: &mut R,
        options: AstSerdeOptions,
    ) -> Result<Self, DeserializationError> {
        match source.read_u8()? {
            0 => {
                let root = Span::<RpoDigest>::read_from(source, options)?;
                Ok(Self::MastRoot(root))
            }
            1 => {
                let path = FullyQualifiedProcedureName::read_from_with_options(source, options)?;
                Ok(Self::Path(path))
            }
            n => Err(DeserializationError::InvalidValue(format!(
                "{} is not a valid alias target type",
                n
            ))),
        }
    }
}
