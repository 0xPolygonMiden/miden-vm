use alloc::{string::String, sync::Arc};
use core::fmt;

use super::{ProcedureName, QualifiedProcedureName};
use crate::{
    ast::{AstSerdeOptions, InvocationTarget},
    diagnostics::SourceFile,
    ByteReader, ByteWriter, DeserializationError, RpoDigest, SourceSpan, Span, Spanned,
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
    /// The name of this procedure
    name: ProcedureName,
    /// The underlying procedure being aliased.
    ///
    /// Alias targets are context-sensitive, depending on how they were defined and what stage of
    /// compilation we're in. See [AliasTarget] for semantics of each target type, but they closely
    /// correspond to [InvocationTarget].
    target: AliasTarget,
}

impl ProcedureAlias {
    /// Creates a new procedure alias called `name`, which resolves to `target`.
    pub fn new(name: ProcedureName, target: AliasTarget) -> Self {
        Self {
            docs: None,
            source_file: None,
            name,
            target,
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
    #[inline]
    pub fn name(&self) -> &ProcedureName {
        &self.name
    }

    /// Returns the target of this procedure alias
    #[inline]
    pub fn target(&self) -> &AliasTarget {
        &self.target
    }

    /// Returns a mutable reference to the target of this procedure alias
    #[inline]
    pub fn target_mut(&mut self) -> &mut AliasTarget {
        &mut self.target
    }

    /// Returns true if this procedure uses an absolute target path
    #[inline]
    pub fn is_absolute(&self) -> bool {
        matches!(self.target, AliasTarget::MastRoot(_) | AliasTarget::AbsoluteProcedurePath(_))
    }

    /// Returns true if this alias uses a different name than the target procedure
    #[inline]
    pub fn is_renamed(&self) -> bool {
        match self.target() {
            AliasTarget::MastRoot(_) => true,
            AliasTarget::ProcedurePath(fqn) | AliasTarget::AbsoluteProcedurePath(fqn) => {
                fqn.name != self.name
            }
        }
    }
}

/// Serialization
impl ProcedureAlias {
    pub fn write_into_with_options<W: ByteWriter>(&self, target: &mut W, options: AstSerdeOptions) {
        self.name.write_into_with_options(target, options);
        self.target.write_into_with_options(target, options);
    }

    pub fn read_from_with_options<R: ByteReader>(
        source: &mut R,
        options: AstSerdeOptions,
    ) -> Result<Self, DeserializationError> {
        let name = ProcedureName::read_from_with_options(source, options)?;
        let target = AliasTarget::read_from_with_options(source, options)?;
        Ok(Self {
            source_file: None,
            docs: None,
            name,
            target,
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
            target => {
                let prefix = if self.is_absolute() { "::" } else { "" };
                if self.is_renamed() {
                    display(format_args!("{}{}->{}", prefix, target, &self.name))
                } else {
                    display(format_args!("{}{}", prefix, target))
                }
            }
        };
        doc
    }
}

/// A fully-qualified external procedure that is the target of a procedure alias
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum AliasTarget {
    /// An alias of the procedure whose root is the given digest
    ///
    /// Corresponds to [`InvocationTarget::MastRoot`]
    MastRoot(Span<RpoDigest>),
    /// An alias of `name`, imported from `module`
    ///
    /// Corresponds to [`InvocationTarget::ProcedurePath`]
    ProcedurePath(QualifiedProcedureName),
    /// An alias of a procedure with the given absolute, fully-qualified path
    ///
    /// Corresponds to [InvocationTarget::AbsoluteProcedurePath]
    AbsoluteProcedurePath(QualifiedProcedureName),
}

impl Spanned for AliasTarget {
    fn span(&self) -> SourceSpan {
        match self {
            Self::MastRoot(spanned) => spanned.span(),
            Self::ProcedurePath(spanned) | Self::AbsoluteProcedurePath(spanned) => spanned.span(),
        }
    }
}

impl From<Span<RpoDigest>> for AliasTarget {
    fn from(digest: Span<RpoDigest>) -> Self {
        Self::MastRoot(digest)
    }
}

impl TryFrom<InvocationTarget> for AliasTarget {
    type Error = InvocationTarget;

    fn try_from(target: InvocationTarget) -> Result<Self, Self::Error> {
        let span = target.span();
        match target {
            InvocationTarget::MastRoot(digest) => Ok(Self::MastRoot(digest)),
            InvocationTarget::ProcedurePath { name, module } => {
                let ns = crate::LibraryNamespace::from_ident_unchecked(module);
                let module = crate::LibraryPath::new_from_components(ns, []);
                Ok(Self::ProcedurePath(QualifiedProcedureName { span, module, name }))
            }
            InvocationTarget::AbsoluteProcedurePath { name, path: module } => {
                Ok(Self::AbsoluteProcedurePath(QualifiedProcedureName { span, module, name }))
            }
            target @ InvocationTarget::ProcedureName(_) => Err(target),
        }
    }
}

impl From<&AliasTarget> for InvocationTarget {
    fn from(target: &AliasTarget) -> Self {
        match target {
            AliasTarget::MastRoot(digest) => Self::MastRoot(*digest),
            AliasTarget::ProcedurePath(ref fqn) => {
                let name = fqn.name.clone();
                let module = fqn.module.last_component().to_ident();
                Self::ProcedurePath { name, module }
            }
            AliasTarget::AbsoluteProcedurePath(ref fqn) => Self::AbsoluteProcedurePath {
                name: fqn.name.clone(),
                path: fqn.module.clone(),
            },
        }
    }
}
impl From<AliasTarget> for InvocationTarget {
    fn from(target: AliasTarget) -> Self {
        match target {
            AliasTarget::MastRoot(digest) => Self::MastRoot(digest),
            AliasTarget::ProcedurePath(fqn) => {
                let name = fqn.name;
                let module = fqn.module.last_component().to_ident();
                Self::ProcedurePath { name, module }
            }
            AliasTarget::AbsoluteProcedurePath(fqn) => Self::AbsoluteProcedurePath {
                name: fqn.name,
                path: fqn.module,
            },
        }
    }
}

impl crate::prettier::PrettyPrint for AliasTarget {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;
        use vm_core::utils::DisplayHex;

        match self {
            Self::MastRoot(digest) => display(DisplayHex(digest.as_bytes().as_slice())),
            Self::ProcedurePath(fqn) => display(fqn),
            Self::AbsoluteProcedurePath(fqn) => display(format_args!("::{}", fqn)),
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
            Self::ProcedurePath(path) => path.write_into_with_options(target, options),
            Self::AbsoluteProcedurePath(path) => path.write_into_with_options(target, options),
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
                let path = QualifiedProcedureName::read_from_with_options(source, options)?;
                Ok(Self::ProcedurePath(path))
            }
            2 => {
                let path = QualifiedProcedureName::read_from_with_options(source, options)?;
                Ok(Self::AbsoluteProcedurePath(path))
            }
            n => Err(DeserializationError::InvalidValue(format!(
                "{} is not a valid alias target type",
                n
            ))),
        }
    }
}
