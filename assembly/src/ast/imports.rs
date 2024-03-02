use core::fmt;

use crate::{
    ast::{AstSerdeOptions, Ident},
    ByteReader, ByteWriter, Deserializable, DeserializationError, LibraryNamespace, LibraryPath,
    Serializable, SourceSpan, Spanned,
};

/// Represents an import statement in Miden Assembly syntax
#[derive(Clone)]
pub struct Import {
    /// The source span of the statement
    pub span: SourceSpan,
    /// The local module name/alias
    ///
    /// When the imported item is aliased to a new name, this
    /// field contains the alias, while `path.last()` can be
    /// used to obtain the actual name.
    pub name: Ident,
    /// The fully-qualified path
    pub path: LibraryPath,
    /// The number of times this import has been used locally
    pub uses: usize,
}
impl fmt::Debug for Import {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Import")
            .field("name", &self.name)
            .field("path", &self.path)
            .field("uses", &self.uses)
            .finish()
    }
}
#[cfg(feature = "formatter")]
impl crate::prettier::PrettyPrint for Import {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;

        let mut doc = const_text("use") + const_text(".") + display(&self.path);
        if self.is_aliased() {
            doc += const_text("->") + display(&self.name);
        }
        doc
    }
}
impl Eq for Import {}
impl PartialEq for Import {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.path == other.path
    }
}
impl Spanned for Import {
    fn span(&self) -> SourceSpan {
        self.span
    }
}
impl Import {
    pub fn is_aliased(&self) -> bool {
        self.name.as_ref() != self.path.last()
    }

    pub fn namespace(&self) -> &LibraryNamespace {
        self.path.namespace()
    }

    pub fn path(&self) -> &LibraryPath {
        &self.path
    }

    pub fn is_used(&self) -> bool {
        self.uses > 0
    }

    /// Serialize this import to `target` with `options`
    pub fn write_into_with_options<W: ByteWriter>(&self, target: &mut W, options: AstSerdeOptions) {
        if options.debug_info {
            self.span.write_into(target);
        }
        self.name.write_into_with_options(target, options);
        self.path.write_into(target);
    }

    pub fn read_from_with_options<R: ByteReader>(
        source: &mut R,
        options: AstSerdeOptions,
    ) -> Result<Self, DeserializationError> {
        let span = if options.debug_info {
            SourceSpan::read_from(source)?
        } else {
            SourceSpan::default()
        };

        let name = Ident::read_from_with_options(source, options)?;
        let path = LibraryPath::read_from(source)?;
        Ok(Self {
            span,
            name,
            path,
            uses: 0,
        })
    }
}
