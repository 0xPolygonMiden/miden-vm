use alloc::sync::Arc;
use core::{fmt, ops::Range};

use super::ByteIndex;

/// A [Location] represents traditional file/line/column information for use in rendering.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    /// The path to the source file in which the relevant source code can be found
    pub path: Arc<str>,
    /// The starting byte index (inclusive) of this location
    pub start: ByteIndex,
    /// The ending byte index (exclusive) of this location
    pub end: ByteIndex,
}

impl Location {
    /// Creates a new [Location].
    pub const fn new(path: Arc<str>, start: ByteIndex, end: ByteIndex) -> Self {
        Self { path, start, end }
    }

    /// Get the name (or path) of the source file
    pub fn path(&self) -> Arc<str> {
        self.path.clone()
    }

    /// Returns the byte range represented by this location
    pub const fn range(&self) -> Range<ByteIndex> {
        self.start..self.end
    }
}

/// A [FileLineCol] represents traditional file/line/column information for use in rendering.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileLineCol {
    /// The path to the source file in which the relevant source code can be found
    pub path: Arc<str>,
    /// The one-indexed number of the line to which this location refers
    pub line: u32,
    /// The one-indexed column of the line on which this location starts
    pub column: u32,
}

impl FileLineCol {
    /// Creates a new [Location].
    pub const fn new(path: Arc<str>, line: u32, column: u32) -> Self {
        Self { path, line, column }
    }

    /// Get the name (or path) of the source file
    pub fn path(&self) -> Arc<str> {
        self.path.clone()
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

impl fmt::Display for FileLineCol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}@{}:{}]", &self.path, self.line, self.column)
    }
}
