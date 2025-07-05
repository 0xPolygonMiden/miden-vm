use alloc::string::String;

use super::{AdviceMapEntry, Block, Constant, Export, Import};
use crate::{SourceSpan, Span, Spanned};

/// This type represents the top-level forms of a Miden Assembly module
#[derive(Debug, PartialEq, Eq)]
pub enum Form {
    /// A documentation string for the entire module
    ModuleDoc(Span<String>),
    /// A documentation string
    Doc(Span<String>),
    /// An import from another module
    Import(Import),
    /// A constant definition, possibly unresolved
    Constant(Constant),
    /// An executable block, represents a program entrypoint
    Begin(Block),
    /// A procedure
    Procedure(Export),
    /// An entry into the Advice Map
    AdviceMapEntry(AdviceMapEntry),
}

impl From<Span<String>> for Form {
    fn from(doc: Span<String>) -> Self {
        Self::Doc(doc)
    }
}

impl From<Import> for Form {
    fn from(import: Import) -> Self {
        Self::Import(import)
    }
}

impl From<Constant> for Form {
    fn from(constant: Constant) -> Self {
        Self::Constant(constant)
    }
}

impl From<Block> for Form {
    fn from(block: Block) -> Self {
        Self::Begin(block)
    }
}

impl From<Export> for Form {
    fn from(export: Export) -> Self {
        Self::Procedure(export)
    }
}

impl Spanned for Form {
    fn span(&self) -> SourceSpan {
        match self {
            Self::ModuleDoc(spanned) | Self::Doc(spanned) => spanned.span(),
            Self::Import(Import { span, .. })
            | Self::Constant(Constant { span, .. })
            | Self::AdviceMapEntry(AdviceMapEntry { span, .. }) => *span,
            Self::Begin(spanned) => spanned.span(),
            Self::Procedure(spanned) => spanned.span(),
        }
    }
}
