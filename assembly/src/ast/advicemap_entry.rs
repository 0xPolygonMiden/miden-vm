use super::DocString;
use crate::{
    SourceSpan,
    ast::{ConstantExpr, Ident},
};

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
    pub key: ConstantExpr,
    /// The value to insert in the Advice Map.
    pub value: ConstantExpr,
}

impl AdviceMapEntry {
    pub fn new(span: SourceSpan, name: Ident, key: ConstantExpr, value: ConstantExpr) -> Self {
        Self { span, docs: None, name, key, value }
    }
}
