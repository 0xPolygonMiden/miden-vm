use super::LineInfo;
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

impl Default for SourceLocation {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

impl From<LineInfo<'_>> for SourceLocation {
    fn from(info: LineInfo<'_>) -> Self {
        let line = info.line_number();
        let column = info.char_offset();
        Self::new(line, column)
    }
}

impl SourceLocation {
    // CONSTRUCTORS
    // -------------------------------------------------------------------------------------------------

    /// Creates a new instance of [SourceLocation].
    pub const fn new(line: u32, column: u32) -> Self {
        Self { line, column }
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
