mod location;
mod source_file;
mod source_manager;
mod span;

pub use self::location::{FileLineCol, Location};
pub use self::source_file::{
    ByteIndex, ByteOffset, ColumnIndex, LineIndex, SourceContent, SourceFile, SourceFileRef,
};
pub use self::source_manager::{DefaultSourceManager, SourceId, SourceManager};
pub use self::span::{SourceSpan, Span, Spanned};

#[cfg(feature = "std")]
pub use self::source_manager::SourceManagerExt;
