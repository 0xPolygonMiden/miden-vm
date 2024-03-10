use alloc::{collections::BTreeSet, string::String, sync::Arc};
use core::fmt;

use super::ProcedureName;
use crate::{
    ast::{AstSerdeOptions, Block, Invoke},
    diagnostics::SourceFile,
    ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable, SourceSpan, Span,
    Spanned,
};

/// Represents the visibility of a procedure globally
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Visibility {
    /// The procedure is visible for call/exec
    Public = 0,
    /// The procedure is visible to syscalls only
    Syscall = 1,
    /// The procedure is visible only locally to the exec instruction
    #[default]
    Private = 2,
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_exported() {
            f.write_str("export")
        } else {
            f.write_str("proc")
        }
    }
}

impl Visibility {
    /// Returns true if the procedure has been explicitly exported
    pub fn is_exported(&self) -> bool {
        matches!(self, Self::Public | Self::Syscall)
    }

    /// Returns true if the procedure is a syscall export
    pub fn is_syscall(&self) -> bool {
        matches!(self, Self::Syscall)
    }
}

impl Serializable for Visibility {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8(*self as u8)
    }
}

impl Deserializable for Visibility {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        match source.read_u8()? {
            0 => Ok(Self::Public),
            1 => Ok(Self::Syscall),
            2 => Ok(Self::Private),
            n => Err(DeserializationError::InvalidValue(format!("invalid visibility tag: {n}"))),
        }
    }
}

/// Represents a concrete procedure definition in Miden Assembly syntax
#[derive(Clone)]
pub struct Procedure {
    /// The source span of the full procedure body
    span: SourceSpan,
    /// The source file in which this procedure was defined, if available
    source_file: Option<Arc<SourceFile>>,
    /// The documentation attached to this procedure
    docs: Option<Span<String>>,
    /// The local name of this procedure
    name: ProcedureName,
    /// The visibility of this procedure (i.e. whether it is exported or not)
    visibility: Visibility,
    /// The number of locals to allocate for this procedure
    num_locals: u16,
    /// The body of the procedure
    body: Block,
    /// The set of callees for any call-like instruction in the procedure body.
    invoked: BTreeSet<Invoke>,
}

/// Construction
impl Procedure {
    /// Create a new [Procedure] from the given source span, visibility, name, number of locals,
    /// and code block.
    pub fn new(
        span: SourceSpan,
        visibility: Visibility,
        name: ProcedureName,
        num_locals: u16,
        body: Block,
    ) -> Self {
        Self {
            span,
            source_file: None,
            docs: None,
            name,
            visibility,
            num_locals,
            invoked: Default::default(),
            body,
        }
    }

    /// Add documentation to this procedure definition
    pub fn with_docs(mut self, docs: Option<Span<String>>) -> Self {
        self.docs = docs;
        self
    }

    /// Add source code to this procedure definition so we can render source snippets
    /// in diagnostics.
    pub fn with_source_file(mut self, source_file: Option<Arc<SourceFile>>) -> Self {
        self.source_file = source_file;
        self
    }

    /// Like [Procedure::with_source_file], but does not require ownership of the procedure.
    pub fn set_source_file(&mut self, source_file: Arc<SourceFile>) {
        self.source_file = Some(source_file);
    }

    /// Modify the visibility of this procedure.
    ///
    /// This is made crate-local as the visibility of a procedure is virtually always determined
    /// by the source code from which it was derived; the only exception being kernel modules,
    /// where exported procedures take on syscall visibility once the module is identified as
    /// a kernel.
    pub(crate) fn set_visibility(&mut self, visibility: Visibility) {
        self.visibility = visibility;
    }
}

/// Metadata
impl Procedure {
    /// Get the source file associated with this procedure
    pub fn source_file(&self) -> Option<Arc<SourceFile>> {
        self.source_file.clone()
    }

    /// Get the name of this procedure within its containing module.
    pub fn name(&self) -> &ProcedureName {
        &self.name
    }

    /// Get the visibility of this procedure
    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    /// Return the number of locals allocated by this procedure.
    pub fn num_locals(&self) -> u16 {
        self.num_locals
    }

    /// Returns true if this procedure corresponds to the `begin`..`end` block of an
    /// executable module.
    pub fn is_entrypoint(&self) -> bool {
        self.name.is_main()
    }

    /// Get the documentation for this procedure, if present.
    pub fn docs(&self) -> Option<&Span<String>> {
        self.docs.as_ref()
    }

    /// Get a reference to the [Block] containing the body of this procedure.
    pub fn body(&self) -> &Block {
        &self.body
    }

    /// Get a mutable reference to the [Block] containing the body of this procedure.
    pub fn body_mut(&mut self) -> &mut Block {
        &mut self.body
    }

    /// Get an iterator over the operations of the top-level [Block] of this procedure.
    pub fn iter(&self) -> core::slice::Iter<'_, crate::ast::Op> {
        self.body.iter()
    }

    /// Get an iterator over the set of invocation targets of this procedure,
    /// i.e. the callees of any call instructions in the body of this procedure.
    pub(crate) fn invoked(
        &self,
    ) -> InvokedIter<'_, alloc::collections::btree_set::Iter<'_, Invoke>> {
        if self.invoked.is_empty() {
            InvokedIter::Empty
        } else {
            InvokedIter::NonEmpty(self.invoked.iter())
        }
    }

    /// Extend the set of procedures known to be invoked by this procedure.
    ///
    /// This is for internal use only, and is called during semantic analysis
    /// once we've identified the set of invoked procedures for a given definition.
    pub(crate) fn extend_invoked<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Invoke>,
    {
        self.invoked.extend(iter);
    }
}

#[doc(hidden)]
pub(crate) enum InvokedIter<'a, I: Iterator<Item = &'a Invoke> + 'a> {
    Empty,
    NonEmpty(I),
}

impl<'a, I> Iterator for InvokedIter<'a, I>
where
    I: Iterator<Item = &'a Invoke> + 'a,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Empty => None,
            Self::NonEmpty(ref mut iter) => {
                let result = iter.next();
                if result.is_none() {
                    *self = Self::Empty;
                }
                result
            }
        }
    }
}

/// Serialization
impl Procedure {
    pub fn write_into_with_options<W: ByteWriter>(&self, target: &mut W, options: AstSerdeOptions) {
        if options.debug_info {
            self.span.write_into(target);
        }
        self.name.write_into_with_options(target, options);
        self.visibility.write_into(target);
        target.write_u16(self.num_locals);
        self.body.write_into_with_options(target, options);
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

        let name = ProcedureName::read_from_with_options(source, options)?;
        let visibility = Visibility::read_from(source)?;
        let num_locals = source.read_u16()?;
        let body = Block::read_from_with_options(source, options)?;
        Ok(Self {
            span,
            source_file: None,
            docs: None,
            name,
            visibility,
            num_locals,
            invoked: Default::default(),
            body,
        })
    }
}

impl Spanned for Procedure {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

#[cfg(feature = "formatter")]
impl crate::prettier::PrettyPrint for Procedure {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;

        let mut doc = Document::Empty;
        if let Some(docs) = self.docs.as_deref() {
            doc = docs
                .lines()
                .map(text)
                .reduce(|acc, line| acc + nl() + text("#! ") + line)
                .unwrap_or(Document::Empty);
        }

        doc += display(self.visibility) + text(".") + display(&self.name);
        if self.num_locals > 0 {
            doc += text(".") + display(self.num_locals);
        }

        doc += indent(4, nl() + self.body.render()) + nl();

        doc + text("end") + nl() + nl()
    }
}

impl fmt::Debug for Procedure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Procedure")
            .field("docs", &self.docs)
            .field("name", &self.name)
            .field("visibility", &self.visibility)
            .field("num_locals", &self.num_locals)
            .field("body", &self.body)
            .field("invoked", &self.invoked)
            .finish()
    }
}

impl Eq for Procedure {}

impl PartialEq for Procedure {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.visibility == other.visibility
            && self.num_locals == other.num_locals
            && self.body == other.body
            && self.docs == other.docs
    }
}
