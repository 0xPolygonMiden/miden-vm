use alloc::{collections::BTreeSet, string::String};
use core::fmt;

use super::ProcedureName;
use crate::{
    ast::{Block, Invoke},
    SourceSpan, Span, Spanned,
};

// PROCEDURE VISIBILITY
// ================================================================================================

/// Represents the visibility of a procedure globally.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Visibility {
    /// The procedure is visible for call/exec.
    Public = 0,
    /// The procedure is visible to syscalls only.
    Syscall = 1,
    /// The procedure is visible only locally to the exec instruction.
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

// PROCEDURE
// ================================================================================================

/// Represents a concrete procedure definition in Miden Assembly syntax
#[derive(Clone)]
pub struct Procedure {
    /// The source span of the full procedure body
    span: SourceSpan,
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
    pub(super) invoked: BTreeSet<Invoke>,
}

/// Construction
impl Procedure {
    /// Creates a new [Procedure] from the given source span, visibility, name, number of locals,
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
            docs: None,
            name,
            visibility,
            num_locals,
            invoked: Default::default(),
            body,
        }
    }

    /// Adds documentation to this procedure definition
    pub fn with_docs(mut self, docs: Option<Span<String>>) -> Self {
        self.docs = docs;
        self
    }

    /// Modifies the visibility of this procedure.
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
    /// Returns the name of this procedure within its containing module.
    pub fn name(&self) -> &ProcedureName {
        &self.name
    }

    /// Returns the visibility of this procedure
    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    /// Returns the number of locals allocated by this procedure.
    pub fn num_locals(&self) -> u16 {
        self.num_locals
    }

    /// Returns true if this procedure corresponds to the `begin`..`end` block of an executable
    /// module.
    pub fn is_entrypoint(&self) -> bool {
        self.name.is_main()
    }

    /// Returns the documentation for this procedure, if present.
    pub fn docs(&self) -> Option<&Span<String>> {
        self.docs.as_ref()
    }

    /// Returns a reference to the [Block] containing the body of this procedure.
    pub fn body(&self) -> &Block {
        &self.body
    }

    /// Returns a mutable reference to the [Block] containing the body of this procedure.
    pub fn body_mut(&mut self) -> &mut Block {
        &mut self.body
    }

    /// Returns an iterator over the operations of the top-level [Block] of this procedure.
    pub fn iter(&self) -> core::slice::Iter<'_, crate::ast::Op> {
        self.body.iter()
    }

    /// Returns an iterator over the set of invocation targets of this procedure, i.e. the callees
    /// of any call instructions in the body of this procedure.
    pub fn invoked<'a, 'b: 'a>(&'b self) -> impl Iterator<Item = &'a Invoke> + 'a {
        if self.invoked.is_empty() {
            InvokedIter::Empty
        } else {
            InvokedIter::NonEmpty(self.invoked.iter())
        }
    }

    /// Extends the set of procedures known to be invoked by this procedure.
    ///
    /// This is for internal use only, and is called during semantic analysis once we've identified
    /// the set of invoked procedures for a given definition.
    pub fn extend_invoked<I>(&mut self, iter: I)
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
            },
        }
    }
}

impl Spanned for Procedure {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

impl crate::prettier::PrettyPrint for Procedure {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;

        let mut doc = Document::Empty;
        if let Some(docs) = self.docs.as_deref() {
            doc = docs
                .lines()
                .map(text)
                .reduce(|acc, line| acc + nl() + const_text("#! ") + line)
                .unwrap_or(Document::Empty);
        }

        doc += display(self.visibility) + const_text(".") + display(&self.name);
        if self.num_locals > 0 {
            doc += const_text(".") + display(self.num_locals);
        }

        doc += indent(4, nl() + self.body.render()) + nl();

        doc + const_text("end") + nl() + nl()
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
