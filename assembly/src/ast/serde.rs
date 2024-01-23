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
        target.write_bool(self.serialize_imports);
        target.write_bool(self.serialize_source_locations);
    }
}

impl Deserializable for AstSerdeOptions {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let serialize_imports = source.read_bool()?;
        let serialize_source_locations = source.read_bool()?;
        Ok(Self::new(serialize_imports, serialize_source_locations))
    }
}
