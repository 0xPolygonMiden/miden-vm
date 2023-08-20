use super::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};
use core::fmt;

// SOURCE LOCATION
// ================================================================================================

/// A struct containing information about the location of a source item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceLocation {
    module_id: u32,
    line: u32,
    column: u32,
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self {
            module_id: 0,
            line: 1,
            column: 1,
        }
    }
}

impl SourceLocation {
    // CONSTRUCTORS
    // -------------------------------------------------------------------------------------------------

    /// Creates a new instance of [SourceLocation].
    pub const fn new(module_id: u32, line: u32, column: u32) -> Self {
        Self {
            module_id,
            line,
            column,
        }
    }

    // PUBLIC ACCESSORS
    // -------------------------------------------------------------------------------------------------

    /// Returns the line of the location.
    pub const fn line(&self) -> u32 {
        self.line
    }

    // STATE MUTATORS
    // -------------------------------------------------------------------------------------------------

    /// Moves the column by the given offset.
    pub fn move_column(&mut self, offset: u32) {
        self.column += offset;
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.line, self.column)
    }
}

impl Serializable for SourceLocation {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u32(self.module_id);
        target.write_u32(self.line);
        target.write_u32(self.column);
    }
}

impl Deserializable for SourceLocation {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let module_id = source.read_u32()?;
        let line = source.read_u32()?;
        let column = source.read_u32()?;
        Ok(Self {
            module_id,
            line,
            column,
        })
    }
}
