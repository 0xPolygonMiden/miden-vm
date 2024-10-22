use alloc::{string::String, sync::Arc};

use crate::{ast::Ident, parser::HexEncodedValue, prettier, Felt, SourceSpan, Span, Spanned};

/// Represents a metadata expression of an [crate::ast::Attribute]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MetaExpr {
    /// An identifier/keyword, e.g. `inline`
    Ident(Ident),
    /// A decimal or hexadecimal integer value
    Int(Span<HexEncodedValue>),
    /// A quoted string or identifier
    String(Ident),
}

impl prettier::PrettyPrint for MetaExpr {
    fn render(&self) -> prettier::Document {
        use prettier::*;

        match self {
            Self::Ident(id) => text(id),
            Self::Int(value) => text(value),
            Self::String(id) => text(format!("\"{}\"", id.as_str().escape_default())),
        }
    }
}

impl From<Ident> for MetaExpr {
    fn from(value: Ident) -> Self {
        Self::Ident(value)
    }
}

impl From<&str> for MetaExpr {
    fn from(value: &str) -> Self {
        Self::String(Ident::new_unchecked(Span::new(SourceSpan::UNKNOWN, Arc::from(value))))
    }
}

impl From<String> for MetaExpr {
    fn from(value: String) -> Self {
        Self::String(Ident::new_unchecked(Span::new(
            SourceSpan::UNKNOWN,
            Arc::from(value.into_boxed_str()),
        )))
    }
}

impl From<u8> for MetaExpr {
    fn from(value: u8) -> Self {
        Self::Int(Span::new(SourceSpan::UNKNOWN, HexEncodedValue::U8(value)))
    }
}

impl From<u16> for MetaExpr {
    fn from(value: u16) -> Self {
        Self::Int(Span::new(SourceSpan::UNKNOWN, HexEncodedValue::U16(value)))
    }
}

impl From<u32> for MetaExpr {
    fn from(value: u32) -> Self {
        Self::Int(Span::new(SourceSpan::UNKNOWN, HexEncodedValue::U32(value)))
    }
}

impl From<Felt> for MetaExpr {
    fn from(value: Felt) -> Self {
        Self::Int(Span::new(SourceSpan::UNKNOWN, HexEncodedValue::Felt(value)))
    }
}

impl From<[Felt; 4]> for MetaExpr {
    fn from(value: [Felt; 4]) -> Self {
        Self::Int(Span::new(SourceSpan::UNKNOWN, HexEncodedValue::Word(value)))
    }
}

impl Spanned for MetaExpr {
    fn span(&self) -> SourceSpan {
        match self {
            Self::Ident(spanned) | Self::String(spanned) => spanned.span(),
            Self::Int(spanned) => spanned.span(),
        }
    }
}