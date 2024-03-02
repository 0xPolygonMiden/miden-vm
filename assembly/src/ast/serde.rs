//! Serialization and deserialization of Abstract syntax trees (ASTs).
//!
//! Structs in this module are used to serialize and deserialize ASTs into a binary format.

use crate::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

/// Serialization options
/// Used to enable or disable serialization of parts of the AST.  Serialization options are
/// serialized along with the AST to make the serialization format self-contained.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct AstSerdeOptions {
    pub serialize_imports: bool,
    /// Include source spans and file paths in the output
    pub debug_info: bool,
}

impl AstSerdeOptions {
    pub const fn new(serialize_imports: bool, debug_info: bool) -> Self {
        Self {
            serialize_imports,
            debug_info,
        }
    }

    pub fn with_debug_info(mut self, yes: bool) -> Self {
        self.debug_info = yes;
        self
    }
}

impl Serializable for AstSerdeOptions {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_bool(self.serialize_imports);
        target.write_bool(self.debug_info);
    }
}

impl Deserializable for AstSerdeOptions {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let serialize_imports = source.read_bool()?;
        let debug_info = source.read_bool()?;
        Ok(Self::new(serialize_imports, debug_info))
    }
}
