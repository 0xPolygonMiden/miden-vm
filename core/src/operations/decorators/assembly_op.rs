use alloc::{
    string::{String, ToString},
    sync::Arc,
};
use core::fmt;

/// A [SourceFile] represents a path to source code corresponding to a Miden operation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceFile(Arc<str>);
impl SourceFile {
    /// Create a new [SourceFile] from a path to the file it represents
    #[cfg(feature = "std")]
    pub fn from_path<P>(path: P) -> Self
    where
        P: AsRef<std::path::Path>,
    {
        let path = path.as_ref().to_string_lossy().into_owned();
        Self::from(path)
    }

    /// Create a new [SourceFile] from a string representing the path to the file it represents
    pub fn new(path: impl AsRef<str>) -> Self {
        Self::from(path.as_ref().to_string())
    }

    /// Get a [std::path::Path] representing this [SourceFile]
    #[cfg(feature = "std")]
    #[inline]
    pub fn as_path(&self) -> &std::path::Path {
        self.as_ref()
    }

    /// Obtain the inner `Arc<str>` representing the source file path
    pub fn into_inner(self) -> Arc<str> {
        self.0
    }
}

impl AsRef<str> for SourceFile {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[cfg(feature = "std")]
impl AsRef<std::path::Path> for SourceFile {
    #[inline(always)]
    fn as_ref(&self) -> &std::path::Path {
        std::path::Path::new(self.0.as_ref())
    }
}

#[cfg(feature = "std")]
impl From<&std::path::Path> for SourceFile {
    fn from(path: &std::path::Path) -> Self {
        Self::from_path(path)
    }
}

#[cfg(feature = "std")]
impl From<std::path::PathBuf> for SourceFile {
    fn from(path: std::path::PathBuf) -> Self {
        Self::from_path(path)
    }
}

impl From<alloc::boxed::Box<str>> for SourceFile {
    #[inline]
    fn from(path: alloc::boxed::Box<str>) -> Self {
        Self(Arc::from(path))
    }
}

impl From<String> for SourceFile {
    #[inline]
    fn from(path: String) -> Self {
        Self(Arc::from(path.into_boxed_str()))
    }
}

/// A [SourceLocation] represents a range of bytes in a source file corresponding to something
/// relevant to a given Miden Assembly operation. Typically this will be the source code from which
/// the Miden Assembly operation was derived.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceLocation {
    /// The path to the source file in which the relevant source code can be found
    pub source_file: SourceFile,
    /// The byte offset (inclusive) representing the start of the relevant location in
    /// `source_file`
    pub start: u32,
    /// The byte offset (exclusive) representing the end of the relevant location in `source_file`
    pub end: u32,
}

// ASSEMBLY OP
// ================================================================================================

/// Contains information corresponding to an assembly instruction (only applicable in debug mode).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssemblyOp {
    location: Option<SourceLocation>,
    context_name: String,
    op: String,
    num_cycles: u8,
    should_break: bool,
}

impl AssemblyOp {
    /// Returns [AssemblyOp] instantiated with the specified assembly instruction string and number
    /// of cycles it takes to execute the assembly instruction.
    pub fn new(
        location: Option<SourceLocation>,
        context_name: String,
        num_cycles: u8,
        op: String,
        should_break: bool,
    ) -> Self {
        Self {
            location,
            context_name,
            op,
            num_cycles,
            should_break,
        }
    }

    /// Returns the [SourceLocation] for this operation, if known
    pub fn location(&self) -> Option<&SourceLocation> {
        self.location.as_ref()
    }

    /// Returns the context name for this operation.
    pub fn context_name(&self) -> &str {
        &self.context_name
    }

    /// Returns the number of VM cycles taken to execute the assembly instruction of this decorator.
    pub const fn num_cycles(&self) -> u8 {
        self.num_cycles
    }

    /// Returns the assembly instruction corresponding to this decorator.
    pub fn op(&self) -> &str {
        &self.op
    }

    /// Returns `true` if there is a breakpoint for the current operation.
    pub const fn should_break(&self) -> bool {
        self.should_break
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Change cycles corresponding to an AsmOp decorator to the specified number of cycles.
    pub fn set_num_cycles(&mut self, num_cycles: u8) {
        self.num_cycles = num_cycles;
    }

    /// Change the [SourceLocation] of this [AssemblyOp]
    pub fn set_location(&mut self, location: SourceLocation) {
        self.location = Some(location);
    }
}

impl fmt::Display for AssemblyOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "context={}, operation={}, cost={}",
            self.context_name, self.op, self.num_cycles,
        )
    }
}
