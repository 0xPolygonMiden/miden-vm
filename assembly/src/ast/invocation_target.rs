use crate::{
    ast::{AstSerdeOptions, Ident, ProcedureName},
    ByteReader, ByteWriter, Deserializable, DeserializationError, LibraryPath, RpoDigest,
    Serializable, SourceSpan, Span, Spanned,
};

/// Represents the kind of an invocation
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum InvokeKind {
    Exec = 0,
    Call,
    SysCall,
}

/// Represents a specific invocation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Invoke {
    pub kind: InvokeKind,
    pub target: InvocationTarget,
}

impl Spanned for Invoke {
    fn span(&self) -> SourceSpan {
        self.target.span()
    }
}

impl Invoke {
    pub fn new(kind: InvokeKind, target: InvocationTarget) -> Self {
        Self { kind, target }
    }
}

/// Describes targets of `exec`, `call`, and `syscall` instructions.
///
/// A label of an invoked procedure must comply with the following rules:
/// - It can be a hexadecimal string representing a MAST root digest ([RpoDigest]). In this case,
///   the label must start with "0x" and must be followed by a valid hexadecimal string
///   representation of an [RpoDigest].
/// - It can contain a single procedure name. In this case, the label must comply with procedure
///   name rules.
/// - It can contain module name followed by procedure name (e.g., "module::procedure"). In this
///   case both module and procedure name must comply with relevant name rules.
///
/// All other combinations will result in an error.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum InvocationTarget {
    /// An absolute procedure reference, but opaque in that we do not know where the callee is
    /// defined. However, it does not actually matter, we consider such references to be _a priori_
    /// valid.
    MastRoot(Span<RpoDigest>) = 0,
    /// A locally-defined procedure
    ProcedureName(ProcedureName) = 1,
    /// A context-sensitive procedure path, which references the name of an import in the containing
    /// module.
    ProcedurePath { name: ProcedureName, module: Ident } = 2,
    /// A fully-resolved procedure path, which refers to a specific externally-defined procedure
    /// with its full path
    AbsoluteProcedurePath {
        name: ProcedureName,
        path: LibraryPath,
    } = 3,
}

impl Spanned for InvocationTarget {
    fn span(&self) -> SourceSpan {
        match self {
            Self::MastRoot(ref spanned) => spanned.span(),
            Self::ProcedureName(ref spanned) => spanned.span(),
            Self::ProcedurePath { ref name, .. } | Self::AbsoluteProcedurePath { ref name, .. } => {
                name.span()
            }
        }
    }
}

impl InvocationTarget {
    fn tag(&self) -> u8 {
        // SAFETY: This is safe because we have given this enum a
        // primitive representation with #[repr(u8)], with the first
        // field of the underlying union-of-structs the discriminant
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl Serializable for InvocationTarget {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8(self.tag());
        match self {
            Self::MastRoot(spanned) => {
                spanned.write_into(target, AstSerdeOptions::new(false, true))
            }
            Self::ProcedureName(name) => {
                name.write_into_with_options(target, AstSerdeOptions::new(false, true))
            }
            Self::ProcedurePath { name, module } => {
                name.write_into_with_options(target, AstSerdeOptions::new(false, true));
                module.write_into(target);
            }
            Self::AbsoluteProcedurePath { name, path } => {
                name.write_into_with_options(target, AstSerdeOptions::new(false, true));
                path.write_into(target);
            }
        }
    }
}

impl Deserializable for InvocationTarget {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        match source.read_u8()? {
            0 => {
                let root = Span::<RpoDigest>::read_from(source, AstSerdeOptions::new(false, true))?;
                Ok(Self::MastRoot(root))
            }
            1 => {
                let name = ProcedureName::read_from_with_options(
                    source,
                    AstSerdeOptions::new(false, true),
                )?;
                Ok(Self::ProcedureName(name))
            }
            2 => {
                let name = ProcedureName::read_from_with_options(
                    source,
                    AstSerdeOptions::new(false, true),
                )?;
                let module = Ident::read_from(source)?;
                Ok(Self::ProcedurePath { name, module })
            }
            3 => {
                let name = ProcedureName::read_from_with_options(
                    source,
                    AstSerdeOptions::new(false, true),
                )?;
                let path = LibraryPath::read_from(source)?;
                Ok(Self::AbsoluteProcedurePath { name, path })
            }
            n => Err(DeserializationError::InvalidValue(format!(
                "{} is not a valid invocation target type",
                n
            ))),
        }
    }
}
