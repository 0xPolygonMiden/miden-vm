use alloc::{string::String, vec::Vec};

use super::DocString;
use crate::{Felt, SourceSpan, Span, Word, ast::Ident};

// Advice Map data that the host populate before the VM starts.
// ============================================================

#[derive(Debug, PartialEq, Eq)]
pub struct AdviceMapEntry {
    /// The source span of the definition.
    pub span: SourceSpan,
    /// The documentation string attached to this definition.
    pub docs: Option<DocString>,
    /// The name of the constant.
    pub name: Ident,
    /// The key to insert in the Advice Map.
    pub key: Span<Word>,
    /// The value to insert in the Advice Map.
    pub value: Vec<Felt>,
}

impl AdviceMapEntry {
    pub fn new(span: SourceSpan, name: Ident, key: Span<Word>, value: Vec<Felt>) -> Self {
        Self { span, docs: None, name, key, value }
    }

    /// Adds documentation to this constant declaration.
    pub fn with_docs(mut self, docs: Option<Span<String>>) -> Self {
        self.docs = docs.map(DocString::new);
        self
    }
}
