use alloc::{borrow::Cow, boxed::Box, sync::Arc, vec::Vec};
use core::{fmt, ops::Range};

pub use miette::{
    self, Diagnostic, IntoDiagnostic, LabeledSpan, NamedSource, Report, Result, Severity,
    SourceCode, WrapErr,
};
pub use tracing;
pub use vm_core::debuginfo::*;

#[macro_export]
macro_rules! report {
    ($($key:ident = $value:expr,)* $fmt:literal $($arg:tt)*) => {
        $crate::diagnostics::Report::from(
            $crate::diagnostic!($($key = $value,)* $fmt $($arg)*)
        )
    };
}

#[macro_export]
macro_rules! diagnostic {
    ($fmt:literal $($arg:tt)*) => {{
        $crate::diagnostics::miette::MietteDiagnostic::new(format!($fmt $($arg)*))
    }};
    ($($key:ident = $value:expr,)+ $fmt:literal $($arg:tt)*) => {{
        let mut diag = $crate::diagnostics::miette::MietteDiagnostic::new(format!($fmt $($arg)*));
        $(diag.$key = Some($value.into());)*
        diag
    }};
}

// LABEL
// ================================================================================================

/// Represents a diagnostic label.
///
/// A label is a source span and optional diagnostic text that should be laid out next to the
/// source snippet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    span: miette::SourceSpan,
    label: Option<Cow<'static, str>>,
}

impl Label {
    /// Construct a label for the given range of bytes, expressible as any type which can be
    /// converted to a [`Range<usize>`], e.g. [miette::SourceSpan].
    pub fn at<R>(range: R) -> Self
    where
        Range<usize>: From<R>,
    {
        let range = Range::<usize>::from(range);
        Self { span: range.into(), label: None }
    }

    /// Construct a label which points to a specific offset in the source file.
    pub fn point<L>(at: usize, label: L) -> Self
    where
        Cow<'static, str>: From<L>,
    {
        Self {
            span: miette::SourceSpan::from(at),
            label: Some(Cow::from(label)),
        }
    }

    /// Construct a label from the given source range and diagnostic text.
    pub fn new<R, L>(range: R, label: L) -> Self
    where
        Range<usize>: From<R>,
        Cow<'static, str>: From<L>,
    {
        let range = Range::<usize>::from(range);
        Self {
            span: range.into(),
            label: Some(Cow::from(label)),
        }
    }

    /// Returns the diagnostic text, the actual "label", for this label.
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

impl From<Label> for miette::SourceSpan {
    #[inline(always)]
    fn from(label: Label) -> Self {
        label.span
    }
}

impl From<Label> for LabeledSpan {
    #[inline]
    fn from(label: Label) -> LabeledSpan {
        if let Some(message) = label.label {
            LabeledSpan::at(label.span, message)
        } else {
            LabeledSpan::underline(label.span)
        }
    }
}

// RELATED LABEL
// ================================================================================================

/// This type is used to associate a more complex label or set of labels with some other error.
///
/// In particular, it is used to reference related bits of source code distinct from that of the
/// original error. A related label can have a distinct severity, its own message, and its own
/// sub-labels, and may reference code in a completely different source file that the original
/// error.
#[derive(Debug)]
pub struct RelatedLabel {
    /// The severity for this related label
    pub severity: Severity,
    /// The message for this label
    pub message: Cow<'static, str>,
    /// The sub-labels for this label
    pub labels: Vec<Label>,
    /// If provided as a standalone diagnostic, the help message to provide
    pub help: Option<Cow<'static, str>>,
    /// The source file to use when rendering source spans of this label as snippets.
    pub file: Option<Arc<SourceFile>>,
}

impl fmt::Display for RelatedLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_ref())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RelatedLabel {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[cfg(not(feature = "std"))]
impl miette::StdError for RelatedLabel {
    fn source(&self) -> Option<&(dyn miette::StdError + 'static)> {
        None
    }
}

impl RelatedLabel {
    pub fn new<S>(severity: Severity, message: S) -> Self
    where
        Cow<'static, str>: From<S>,
    {
        Self {
            severity,
            message: Cow::from(message),
            help: None,
            labels: vec![],
            file: None,
        }
    }

    pub fn error<S>(message: S) -> Self
    where
        Cow<'static, str>: From<S>,
    {
        Self::new(Severity::Error, message)
    }

    #[allow(unused)]
    pub fn warning<S>(message: S) -> Self
    where
        Cow<'static, str>: From<S>,
    {
        Self::new(Severity::Warning, message)
    }

    #[allow(unused)]
    pub fn advice<S>(message: S) -> Self
    where
        Cow<'static, str>: From<S>,
    {
        Self::new(Severity::Advice, message)
    }

    pub fn with_source_file(mut self, file: Option<Arc<SourceFile>>) -> Self {
        self.file = file;
        self
    }

    pub fn with_labeled_span<S>(self, span: SourceSpan, message: S) -> Self
    where
        Cow<'static, str>: From<S>,
    {
        let range = span.into_range();
        self.with_label(Label::new((range.start as usize)..(range.end as usize), message))
    }

    pub fn with_label(mut self, label: Label) -> Self {
        self.labels.push(label);
        self
    }

    #[allow(unused)]
    pub fn with_labels<I>(mut self, labels: I) -> Self
    where
        I: IntoIterator<Item = Label>,
    {
        self.labels.extend(labels);
        self
    }

    pub fn with_help<S>(mut self, help: S) -> Self
    where
        Cow<'static, str>: From<S>,
    {
        self.help = Some(help.into());
        self
    }
}

impl Diagnostic for RelatedLabel {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        None
    }
    fn severity(&self) -> Option<Severity> {
        Some(self.severity)
    }
    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        self.help.as_deref().map(|help| Box::new(help) as Box<_>)
    }
    fn url<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        None
    }
    fn source_code(&self) -> Option<&dyn SourceCode> {
        self.file.as_ref().map(|f| f as &dyn SourceCode)
    }
    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        if self.labels.is_empty() {
            None
        } else {
            Some(Box::new(self.labels.iter().cloned().map(|l| l.into())))
        }
    }
    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        None
    }
    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        None
    }
}

// RELATED ERROR
// ================================================================================================

/// This type allows rolling up a diagnostic into a parent error
///
/// This is necessary as [Report] cannot be used as the source error when deriving [Diagnostic].
#[derive(Debug)]
pub struct RelatedError(Report);

impl RelatedError {
    pub fn into_report(self) -> Report {
        self.0
    }

    #[inline(always)]
    pub fn as_diagnostic(&self) -> &dyn Diagnostic {
        self.0.as_ref()
    }
}

impl Diagnostic for RelatedError {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        self.as_diagnostic().code()
    }
    fn severity(&self) -> Option<Severity> {
        self.as_diagnostic().severity()
    }
    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        self.as_diagnostic().help()
    }
    fn url<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        self.as_diagnostic().url()
    }
    fn source_code(&self) -> Option<&dyn SourceCode> {
        self.as_diagnostic().source_code()
    }
    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        self.as_diagnostic().labels()
    }
    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        self.as_diagnostic().related()
    }
    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        self.as_diagnostic().diagnostic_source()
    }
}

impl fmt::Display for RelatedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RelatedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        AsRef::<dyn std::error::Error>::as_ref(&self.0).source()
    }
}

#[cfg(not(feature = "std"))]
impl miette::StdError for RelatedError {
    fn source(&self) -> Option<&(dyn miette::StdError + 'static)> {
        AsRef::<dyn miette::StdError>::as_ref(&self.0).source()
    }
}

impl From<Report> for RelatedError {
    fn from(report: Report) -> Self {
        Self(report)
    }
}

impl RelatedError {
    pub const fn new(report: Report) -> Self {
        Self(report)
    }

    pub fn wrap<E>(error: E) -> Self
    where
        E: Diagnostic + Send + Sync + 'static,
    {
        Self(Report::new_boxed(Box::new(error)))
    }
}

// REPORTING
// ================================================================================================

/// Rendering and error reporting implementation details.
pub mod reporting {
    use core::fmt;

    pub use miette::{
        DebugReportHandler, JSONReportHandler, NarratableReportHandler, ReportHandler, set_hook,
    };
    #[cfg(feature = "std")]
    pub use miette::{GraphicalReportHandler, GraphicalTheme, set_panic_hook};

    pub type ReportHandlerOpts = miette::MietteHandlerOpts;

    #[cfg(feature = "std")]
    pub type DefaultReportHandler = miette::GraphicalReportHandler;

    #[cfg(not(feature = "std"))]
    pub type DefaultReportHandler = miette::DebugReportHandler;

    /// A type that can be used to render a [super::Diagnostic] via [core::fmt::Display]
    pub struct PrintDiagnostic<D, R = DefaultReportHandler> {
        handler: R,
        diag: D,
    }

    impl<D: AsRef<dyn super::Diagnostic>> PrintDiagnostic<D> {
        pub fn new(diag: D) -> Self {
            Self { handler: Default::default(), diag }
        }
        #[cfg(feature = "std")]
        pub fn new_without_color(diag: D) -> Self {
            Self {
                handler: DefaultReportHandler::new_themed(GraphicalTheme::none()),
                diag,
            }
        }
        #[cfg(not(feature = "std"))]
        pub fn new_without_color(diag: D) -> Self {
            Self::new(diag)
        }
    }

    impl<D: AsRef<dyn super::Diagnostic>> PrintDiagnostic<D, NarratableReportHandler> {
        pub fn narrated(diag: D) -> Self {
            Self {
                handler: NarratableReportHandler::default(),
                diag,
            }
        }
    }

    impl<D: AsRef<dyn super::Diagnostic>> PrintDiagnostic<D, JSONReportHandler> {
        pub fn json(diag: D) -> Self {
            Self { handler: JSONReportHandler, diag }
        }
    }

    impl<D: AsRef<dyn super::Diagnostic>> fmt::Display for PrintDiagnostic<D> {
        fn fmt(&self, f: &mut fmt::Formatter) -> core::fmt::Result {
            self.handler.render_report(f, self.diag.as_ref())
        }
    }

    impl<D: AsRef<dyn super::Diagnostic>> fmt::Display for PrintDiagnostic<D, NarratableReportHandler> {
        fn fmt(&self, f: &mut fmt::Formatter) -> core::fmt::Result {
            self.handler.render_report(f, self.diag.as_ref())
        }
    }

    impl<D: AsRef<dyn super::Diagnostic>> fmt::Display for PrintDiagnostic<D, JSONReportHandler> {
        fn fmt(&self, f: &mut fmt::Formatter) -> core::fmt::Result {
            self.handler.render_report(f, self.diag.as_ref())
        }
    }
}
