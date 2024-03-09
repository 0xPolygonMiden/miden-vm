use crate::{
    assembler::{Assembler, AssemblyContext, ProcedureCache},
    ast::{Form, FullyQualifiedProcedureName, Module, ModuleKind},
    diagnostics::{
        reporting::{set_hook, ReportHandlerOpts},
        Report, SourceFile,
    },
    parser::ModuleParser,
    Library, LibraryNamespace, LibraryPath, RpoDigest,
};

#[cfg(feature = "std")]
use crate::diagnostics::reporting::set_panic_hook;

use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use core::fmt;
use vm_core::{utils::DisplayHex, Program};

/// Represents a pattern for matching text abstractly
/// for use in asserting contents of complex diagnostics
#[derive(Debug)]
pub enum Pattern {
    /// Searches for an exact match of the given literal in the input string
    Literal(alloc::borrow::Cow<'static, str>),
    /// Searches for a match of the given regular expression in the input string
    Regex(regex::Regex),
}
impl Pattern {
    /// Construct a [Pattern] representing the given regular expression
    #[track_caller]
    pub fn regex(pattern: impl AsRef<str>) -> Self {
        Self::Regex(regex::Regex::new(pattern.as_ref()).expect("invalid regex"))
    }

    /// Check if this pattern matches `input`
    pub fn is_match(&self, input: impl AsRef<str>) -> bool {
        match self {
            Self::Literal(pattern) => input.as_ref().contains(pattern.as_ref()),
            Self::Regex(ref regex) => regex.is_match(input.as_ref()),
        }
    }

    /// Assert that this pattern matches `input`.
    ///
    /// This behaves like `assert_eq!` or `assert_matches!`, i.e. it
    /// will produce a helpful panic message on failure that renders
    /// the difference between what the pattern expected, and what
    /// it actually was matched against.
    #[track_caller]
    pub fn assert_match(&self, input: impl AsRef<str>) {
        let input = input.as_ref();
        if !self.is_match(input) {
            panic!(
                r"expected string was not found in emitted diagnostics:
expected input to {expected}
matched against: `{actual}`
",
                expected = self,
                actual = input
            );
        }
    }

    /// Like [Pattern::assert_match], but renders additional context
    /// in the case of failure to aid in troubleshooting.
    #[track_caller]
    pub fn assert_match_with_context(&self, input: impl AsRef<str>, context: impl AsRef<str>) {
        let input = input.as_ref();
        let context = context.as_ref();
        if !self.is_match(input) {
            panic!(
                r"expected string was not found in emitted diagnostics:
expected input to {expected}
matched against: `{actual}`
full output: `{context}`
",
                expected = self,
                actual = input
            );
        }
    }
}
impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Literal(ref lit) => write!(f, "contain `{}`", lit),
            Self::Regex(ref pat) => write!(f, "match regular expression `{}`", pat.as_str()),
        }
    }
}
impl From<&'static str> for Pattern {
    fn from(s: &'static str) -> Self {
        Self::Literal(alloc::borrow::Cow::Borrowed(s.trim()))
    }
}
impl From<String> for Pattern {
    fn from(s: String) -> Self {
        Self::Literal(alloc::borrow::Cow::Owned(s))
    }
}
impl From<regex::Regex> for Pattern {
    fn from(pat: regex::Regex) -> Self {
        Self::Regex(pat)
    }
}

/// Create a [Pattern::Regex] from the given input
#[macro_export]
macro_rules! regex {
    ($source:literal) => {
        Pattern::regex($source)
    };

    ($source:expr) => {
        Pattern::regex($source)
    };
}

/// Construct an [`Arc<SourceFile>`] from a string literal or expression,
/// such that emitted diagnostics reference the file and line on which
/// the source file was constructed.
#[macro_export]
macro_rules! source_file {
    ($source:literal) => {
        ::alloc::sync::Arc::new($crate::diagnostics::SourceFile::new(
            concat!("test", line!()),
            $source.to_string(),
        ))
    };
    ($source:expr) => {
        ::alloc::sync::Arc::new($crate::diagnostics::SourceFile::new(
            concat!("test", line!()),
            $source,
        ))
    };
}

/// Assert that the given diagnostic/error value, when rendered to stdout,
/// contains the given pattern
#[macro_export]
macro_rules! assert_diagnostic {
    ($diagnostic:expr, $expected:literal) => {{
        let actual = format!("{}", PrintDiagnostic::new_without_color($diagnostic));
        Pattern::from($expected).assert_match(actual);
    }};

    ($diagnostic:expr, $expected:expr) => {{
        let actual = format!("{}", PrintDiagnostic::new_without_color($diagnostic));
        Pattern::from($expected).assert_match(actual);
    }};
}

/// Like [assert_diagnostic], but matches each non-empty line of the rendered output
/// to a corresponding pattern. So if the output has 3 lines, the second of which is
/// empty, and you provide 2 patterns, the assertion passes if the first line matches
/// the first pattern, and the third line matches the second pattern - the second
/// line is ignored because it is empty.
#[macro_export]
macro_rules! assert_diagnostic_lines {
    ($diagnostic:expr, $($expected:expr),+) => {{
        let actual = format!("{}", $crate::diagnostics::reporting::PrintDiagnostic::new_without_color($diagnostic));
        let lines = actual.lines().filter(|l| !l.trim().is_empty()).zip([$(Pattern::from($expected)),*].into_iter());
        for (actual_line, expected) in lines {
            expected.assert_match_with_context(actual_line, &actual);
        }
    }};
}

pub struct TestContext {
    assembler: Assembler,
}
impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}
impl TestContext {
    pub fn new() -> Self {
        #[cfg(feature = "std")]
        {
            let result = set_hook(Box::new(|_| Box::new(ReportHandlerOpts::new().build())));
            #[cfg(feature = "std")]
            if result.is_ok() {
                set_panic_hook();
            }
        }

        #[cfg(not(feature = "std"))]
        {
            let _ = set_hook(Box::new(|_| Box::new(ReportHandlerOpts::new().build())));
        }
        Self {
            assembler: Assembler::default().with_debug_mode(true),
        }
    }

    #[track_caller]
    pub fn parse_forms(&mut self, source: Arc<SourceFile>) -> Result<Vec<Form>, Report> {
        crate::parser::parse_forms(source.clone())
            .map_err(|err| Report::new(err).with_source_code(source))
    }

    #[track_caller]
    pub fn parse_program(&mut self, source: Arc<SourceFile>) -> Result<Box<Module>, Report> {
        let mut parser = ModuleParser::new(ModuleKind::Executable);
        parser.parse(LibraryNamespace::Exec.into(), source)
    }

    #[allow(unused)]
    #[track_caller]
    pub fn parse_kernel(&mut self, source: Arc<SourceFile>) -> Result<Box<Module>, Report> {
        let mut parser = ModuleParser::new(ModuleKind::Kernel);
        parser.parse(LibraryNamespace::Kernel.into(), source)
    }

    #[track_caller]
    pub fn parse_module(&mut self, source: Arc<SourceFile>) -> Result<Box<Module>, Report> {
        let mut parser = ModuleParser::new(ModuleKind::Library);
        parser.parse(LibraryNamespace::Anon.into(), source)
    }

    #[track_caller]
    pub fn parse_module_with_path(
        &mut self,
        path: LibraryPath,
        source: Arc<SourceFile>,
    ) -> Result<Box<Module>, Report> {
        let mut parser = ModuleParser::new(ModuleKind::Library);
        parser.parse(path, source)
    }

    #[track_caller]
    pub fn add_module(&mut self, module: Box<Module>) -> Result<(), Report> {
        self.assembler.add_module(module)
    }

    #[track_caller]
    pub fn add_module_from_source(
        &mut self,
        path: LibraryPath,
        source: Arc<SourceFile>,
    ) -> Result<(), Report> {
        let ast = self.parse_module_with_path(path, source)?;
        self.assembler.add_module(ast)
    }

    #[track_caller]
    pub fn add_library<L>(&mut self, library: &L) -> Result<(), Report>
    where
        L: ?Sized + Library + 'static,
    {
        self.assembler.add_library(library)
    }

    #[track_caller]
    pub fn compile(&mut self, source: Arc<SourceFile>) -> Result<Program, Report> {
        self.assembler.compile_source(source)
    }

    #[track_caller]
    pub fn compile_ast(&mut self, ast: Box<Module>) -> Result<Program, Report> {
        self.assembler.compile_ast(ast)
    }

    #[track_caller]
    pub fn compile_module_from_source(
        &mut self,
        path: LibraryPath,
        source: Arc<SourceFile>,
    ) -> Result<Vec<RpoDigest>, Report> {
        let ast = self.parse_module_with_path(path.clone(), source)?;
        self.assembler.compile_module(ast, &mut AssemblyContext::for_library(&path))
    }

    pub fn procedure_cache(&self) -> &ProcedureCache {
        self.assembler.procedure_cache()
    }

    pub fn display_digest_from_cache(
        &self,
        name: &FullyQualifiedProcedureName,
    ) -> impl fmt::Display {
        self.procedure_cache()
            .get_by_name(name)
            .map(|p| p.code().hash())
            .map(DisplayDigest)
            .unwrap()
    }
}

struct DisplayDigest(RpoDigest);
impl fmt::Display for DisplayDigest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", DisplayHex(self.0.as_bytes().as_slice()))
    }
}
