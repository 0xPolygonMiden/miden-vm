use crate::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};
use core::fmt;

// SOURCE LOCATION
// ================================================================================================

/// A struct containing information about the location of a source item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceLocation {
    // TODO add uri
    line: u32,
    column: u32,
}

impl SourceLocation {
    /// Creates a new instance of [SourceLocation].
    pub const fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }

    /// Returns the line of the location.
    pub const fn line(&self) -> u32 {
        self.line
    }

    /// Moves the column by the given offset.
    pub fn move_column(&mut self, offset: u32) {
        self.column += offset;
    }
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.line, self.column)
    }
}

impl Serializable for SourceLocation {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u32(self.line);
        target.write_u32(self.column);
    }
}

impl Deserializable for SourceLocation {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let line = source.read_u32()?;
        let column = source.read_u32()?;
        Ok(Self { line, column })
    }
}
