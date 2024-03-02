mod alias;
mod id;
mod name;
#[allow(clippy::module_inception)]
mod procedure;
mod resolver;

pub use self::alias::ProcedureAlias;
pub use self::id::ProcedureIndex;
pub use self::name::{FullyQualifiedProcedureName, ProcedureName};
pub use self::procedure::{Procedure, Visibility};
pub use self::resolver::{LocalNameResolver, ResolvedProcedure};

use crate::{
    ast::{AstSerdeOptions, Invoke},
    diagnostics::SourceFile,
    ByteReader, ByteWriter, DeserializationError, SourceSpan, Span, Spanned,
};
use alloc::{string::String, sync::Arc};

/// Represents an exportable entity from a [Module]
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Export {
    /// A locally-defined procedure
    Procedure(Procedure) = 0,
    /// An alias for an externally-defined procedure, i.e. a re-exported import
    Alias(ProcedureAlias) = 1,
}
#[cfg(feature = "formatter")]
impl crate::prettier::PrettyPrint for Export {
    fn render(&self) -> crate::prettier::Document {
        match self {
            Self::Procedure(ref proc) => proc.render(),
            Self::Alias(ref proc) => proc.render(),
        }
    }
}
impl Spanned for Export {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Procedure(ref spanned) => spanned.span(),
            Self::Alias(ref spanned) => spanned.span(),
        }
    }
}
impl Export {
    pub fn with_docs(self, docs: Option<Span<String>>) -> Self {
        match self {
            Self::Procedure(proc) => Self::Procedure(proc.with_docs(docs)),
            Self::Alias(alias) => Self::Alias(alias.with_docs(docs)),
        }
    }

    pub fn with_source_file(self, source_file: Option<Arc<SourceFile>>) -> Self {
        match self {
            Self::Procedure(proc) => Self::Procedure(proc.with_source_file(source_file)),
            Self::Alias(alias) => Self::Alias(alias.with_source_file(source_file)),
        }
    }

    pub fn source_file(&self) -> Option<Arc<SourceFile>> {
        match self {
            Self::Procedure(ref proc) => proc.source_file(),
            Self::Alias(ref alias) => alias.source_file(),
        }
    }

    pub fn name(&self) -> &ProcedureName {
        match self {
            Self::Procedure(ref proc) => proc.name(),
            Self::Alias(ref alias) => alias.name(),
        }
    }

    pub fn docs(&self) -> Option<&str> {
        match self {
            Self::Procedure(ref proc) => proc.docs().map(|spanned| spanned.as_deref().into_inner()),
            Self::Alias(ref alias) => alias.docs().map(|spanned| spanned.as_deref().into_inner()),
        }
    }

    pub fn visibility(&self) -> Visibility {
        match self {
            Self::Procedure(ref proc) => proc.visibility(),
            Self::Alias(_) => Visibility::Public,
        }
    }

    pub fn num_locals(&self) -> usize {
        match self {
            Self::Procedure(ref proc) => proc.num_locals() as usize,
            Self::Alias(_) => 0,
        }
    }

    pub fn is_main(&self) -> bool {
        self.name().is_main()
    }

    #[track_caller]
    pub fn unwrap_procedure(&self) -> &Procedure {
        match self {
            Self::Procedure(ref proc) => proc,
            Self::Alias(_) => panic!("attempted to unwrap alias export as procedure definition"),
        }
    }

    pub(crate) fn invoked(&self) -> impl Iterator<Item = &Invoke> + '_ {
        match self {
            Self::Procedure(ref proc) => proc.invoked(),
            Self::Alias(_) => procedure::InvokedIter::Empty,
        }
    }
}

/// Serialization
impl Export {
    pub fn write_into_with_options<W: ByteWriter>(&self, target: &mut W, options: AstSerdeOptions) {
        target.write_u8(self.tag());
        match self {
            Self::Procedure(ref proc) => proc.write_into_with_options(target, options),
            Self::Alias(ref proc) => proc.write_into_with_options(target, options),
        }
    }

    pub fn read_from_with_options<R: ByteReader>(
        source: &mut R,
        options: AstSerdeOptions,
    ) -> Result<Self, DeserializationError> {
        match source.read_u8()? {
            0 => Procedure::read_from_with_options(source, options).map(Self::Procedure),
            1 => ProcedureAlias::read_from_with_options(source, options).map(Self::Alias),
            n => {
                Err(DeserializationError::InvalidValue(format!("invalid procedure kind tag: {n}")))
            }
        }
    }

    fn tag(&self) -> u8 {
        // SAFETY: This is safe because we have given this enum a
        // primitive representation with #[repr(u8)], with the first
        // field of the underlying union-of-structs the discriminant
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}
