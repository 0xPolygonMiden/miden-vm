//! Serialization and deserialization of Abstract syntax trees (ASTs).
//!
//! Structs in this module are used to serialize and deserialize ASTs into a binary format.

use super::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

/// Serialization options
/// Used to enable or disable serialization of parts of the AST.  Serialization options are
/// serialized along with the AST to make the serialization format self-contained.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AstSerdeOptions {
    pub serialize_imports: bool,
    pub serialize_source_locations: bool,
}

impl AstSerdeOptions {
    pub const fn new(serialize_imports: bool, serialize_source_locations: bool) -> Self {
        Self {
            serialize_imports,
            serialize_source_locations,
        }
    }
}

impl Serializable for AstSerdeOptions {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        // Encode two boolean values in one u8 value, where the first byte is the
        // `serialize_source_locations` flag and the second byte is `serialize_imports` flag
        let options =
            ((self.serialize_imports as u8) << 1u8) + self.serialize_source_locations as u8;
        target.write_u8(options);
    }
}

impl Deserializable for AstSerdeOptions {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let options = source.read_u8()?;
        Ok(Self::new((options >> 1 & 1) != 0, (options & 1) != 0))
    }
}
