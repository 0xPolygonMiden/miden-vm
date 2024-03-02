use alloc::vec::Vec;
use core::fmt;

use super::Op;
use crate::{
    ast::AstSerdeOptions, ByteReader, ByteWriter, Deserializable, DeserializationError,
    Serializable, SourceSpan, Spanned,
};

/// Represents a basic block in Miden Assembly syntax
///
/// Blocks can be nested, see [Op] for details.
#[derive(Clone, Default)]
pub struct Block {
    span: SourceSpan,
    body: Vec<Op>,
}
impl Block {
    /// Create a new [Block]
    pub fn new(span: SourceSpan, body: Vec<Op>) -> Self {
        Self { span, body }
    }

    /// Append `op` to this block
    pub fn push(&mut self, op: Op) {
        self.body.push(op);
    }

    /// Get the number of ops in this block
    ///
    /// NOTE: The count does not include nested ops,
    /// only those at the root of the block.
    pub fn len(&self) -> usize {
        self.body.len()
    }

    /// Returns true if this block is empty
    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    /// Get an iterator for the operations in this block
    pub fn iter(&self) -> core::slice::Iter<'_, Op> {
        self.body.iter()
    }

    /// Get a mutable iterator for the operations in this block
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Op> {
        self.body.iter_mut()
    }

    /// Serialize this block to `target` with `options`
    pub fn write_into_with_options<W: ByteWriter>(&self, target: &mut W, options: AstSerdeOptions) {
        if options.debug_info {
            self.span.write_into(target);
        }
        target.write_u16(self.body.len() as u16);
        for op in self.body.iter() {
            op.write_into_with_options(target, options);
        }
    }

    /// Deserialize this block from `source` with `options`
    pub fn read_from_with_options<R: ByteReader>(
        source: &mut R,
        options: AstSerdeOptions,
    ) -> Result<Self, DeserializationError> {
        let span = if options.debug_info {
            SourceSpan::read_from(source)?
        } else {
            SourceSpan::default()
        };

        let body_len = source.read_u16()? as usize;
        let mut body = Vec::with_capacity(body_len);
        for _ in 0..body_len {
            let op = Op::read_from_with_options(source, options)?;
            body.push(op);
        }
        Ok(Self { span, body })
    }
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(&self.body).finish()
    }
}
#[cfg(feature = "formatter")]
impl crate::prettier::PrettyPrint for Block {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;

        let body = self.body.iter().map(PrettyPrint::render).reduce(|acc, doc| acc + nl() + doc);
        body.map(|body| indent(4, body)).unwrap_or(Document::Empty)
    }
}
impl Spanned for Block {
    fn span(&self) -> SourceSpan {
        self.span
    }
}
impl Eq for Block {}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body
    }
}
