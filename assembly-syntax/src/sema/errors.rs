use alloc::{sync::Arc, vec::Vec};
use core::fmt;

use crate::{
    SourceFile, SourceSpan,
    diagnostics::{Diagnostic, miette},
};

/// The high-level error type for all semantic analysis errors.
///
/// This rolls up multiple errors into a single one, and as such, can emit many
/// diagnostics at once. You should prefer to gather up errors and continue with
/// analysis for as long as possible before returning an error of this type, so
/// as to provide as much information to the user as possible, but there are cases
/// where doing so would not be profitable - it is a judgement call.
///
/// The semantic analyzer does this though, and you can examine its implementation
/// to see how we approach this during the semantic analysis phase specifically.
#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("syntax error")]
#[diagnostic(help("see emitted diagnostics for details"))]
pub struct SyntaxError {
    #[source_code]
    pub source_file: Arc<SourceFile>,
    #[related]
    pub errors: Vec<SemanticAnalysisError>,
}

/// This type is used when emitting advice/warnings, when those are not being treated as errors.
///
/// Like [SyntaxError], this rolls up all such notices into a single batch, and emits them all
/// at once. The difference is that we never return this as an error from any API, it simply
/// exists to leverage the diagnostic infrastructure of `miette`.
#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("one or more warnings were emitted")]
#[diagnostic(help("see below for details"))]
pub struct SyntaxWarning {
    #[source_code]
    pub source_file: Arc<SourceFile>,
    #[related]
    pub errors: Vec<SemanticAnalysisError>,
}

/// Represents an error that occurs during semantic analysis
#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum SemanticAnalysisError {
    #[error("invalid program: no entrypoint defined")]
    #[diagnostic(help(
        "ensure you define an entrypoint somewhere in the body with `begin`..`end`"
    ))]
    MissingEntrypoint,
    #[error("invalid module: unexpected entrypoint definition")]
    #[diagnostic(help("library modules cannot contain `begin`..`end` blocks"))]
    UnexpectedEntrypoint {
        #[label]
        span: SourceSpan,
    },
    #[error("invalid module: multiple conflicting entrypoints defined")]
    #[diagnostic(help("an executable module can only have a single `begin`..`end` block"))]
    MultipleEntrypoints {
        #[label]
        span: SourceSpan,
        #[label]
        prev_span: SourceSpan,
    },
    #[error("invalid program: procedure exports are not allowed")]
    #[diagnostic(help("perhaps you meant to use `proc` instead of `export`?"))]
    UnexpectedExport {
        #[label]
        span: SourceSpan,
    },
    #[error("symbol conflict: found duplicate definitions of the same name")]
    #[diagnostic()]
    SymbolConflict {
        #[label("conflict occurs here")]
        span: SourceSpan,
        #[label("previously defined here")]
        prev_span: SourceSpan,
    },
    #[error("symbol undefined: no such name found in scope")]
    #[diagnostic(help("are you missing an import?"))]
    SymbolUndefined {
        #[label]
        span: SourceSpan,
    },
    #[error("unused import")]
    #[diagnostic(severity(Warning), help("this import is never used and can be safely removed"))]
    UnusedImport {
        #[label]
        span: SourceSpan,
    },
    #[error("missing import: the referenced module has not been imported")]
    #[diagnostic()]
    MissingImport {
        #[label("this reference is invalid without a corresponding import")]
        span: SourceSpan,
    },
    #[error("symbol conflict: import would shadow a previous import of the same name")]
    #[diagnostic(help(
        "imports must have unique names within a module, \
        try aliasing one of the imports if both are needed"
    ))]
    ImportConflict {
        #[label("caused by this import")]
        span: SourceSpan,
        #[label("previously imported here")]
        prev_span: SourceSpan,
    },
    #[error(
        "invalid re-exported procedure: kernel modules may not re-export procedures from other modules"
    )]
    #[diagnostic()]
    ReexportFromKernel {
        #[label]
        span: SourceSpan,
    },
    #[error("invalid syscall: cannot make a syscall from within the kernel")]
    #[diagnostic(help("syscalls are only valid outside the kernel, you should use exec instead"))]
    SyscallInKernel {
        #[label]
        span: SourceSpan,
    },
    #[error("invalid call: kernel modules cannot make calls to external procedures")]
    #[diagnostic(help(
        "this call is being made from a kernel module, and may only refer to local procedures"
    ))]
    CallInKernel {
        #[label]
        span: SourceSpan,
    },
    #[error("invalid instruction usage: 'caller' is only valid in kernel modules")]
    #[diagnostic()]
    CallerInKernel {
        #[label]
        span: SourceSpan,
    },
    #[error("invalid syscall: callee must be resolvable to kernel module")]
    #[diagnostic()]
    InvalidSyscallTarget {
        #[label]
        span: SourceSpan,
    },
    #[error("invalid recursive procedure call")]
    #[diagnostic(help(
        "this call induces a cycle that returns back to the caller, you must break that cycle"
    ))]
    InvalidRecursiveCall {
        #[label("caused by this call")]
        span: SourceSpan,
    },
    #[error("invalid recursive procedure call")]
    #[diagnostic(help("this call is self-recursive, which is not allowed"))]
    SelfRecursive {
        #[label]
        span: SourceSpan,
    },
    #[error("invalid immediate: value is larger than expected range")]
    #[diagnostic()]
    ImmediateOverflow {
        #[label]
        span: SourceSpan,
    },
    #[error("invalid module: {}", kind)]
    #[diagnostic(help("try breaking this module up into submodules"))]
    LimitExceeded {
        #[label]
        span: SourceSpan,
        kind: LimitKind,
    },
    #[error("unused docstring")]
    #[diagnostic(
        severity(Warning),
        help(
            "this docstring is immediately followed by at least one empty line, then another docstring,\
            if you intended these to be a single docstring, you should remove the empty lines"
        )
    )]
    UnusedDocstring {
        #[label]
        span: SourceSpan,
    },
    #[error("unused docstring")]
    #[diagnostic(
        severity(Warning),
        help(
            "module imports cannot have docstrings, you should use line comment syntax here instead"
        )
    )]
    ImportDocstring {
        #[label]
        span: SourceSpan,
    },
    #[error("invalid constant")]
    #[diagnostic(help("this constant does not resolve to a value of the right type"))]
    InvalidConstant {
        #[label]
        span: SourceSpan,
    },
    #[error("advmap key already defined")]
    AdvMapKeyAlreadyDefined {
        #[label]
        span: SourceSpan,
    },
}

/// Represents a system limit that was exceeded
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LimitKind {
    /// The total number of procedures in a module
    Procedures,
    /// The total number of procedure locals
    Locals,
    /// The total number of imports in a module
    Imports,
    /// The total number of calls to imports
    ///
    /// TODO(pauls): Is this even a limit anymore?
    CalledImports,
    /// The total number of instructions in a procedure.
    Instructions,
}

impl fmt::Display for LimitKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Procedures => f.write_str("too many procedures in module"),
            Self::Locals => f.write_str("too many procedure locals"),
            Self::Imports => f.write_str("too many imported procedures"),
            Self::CalledImports => f.write_str("too many calls to imported procedures"),
            Self::Instructions => f.write_str("too many instructions in block"),
        }
    }
}
