/// Simple macro used in the grammar definition for constructing spans
macro_rules! span {
    ($id:expr, $l:expr, $r:expr) => {
        crate::SourceSpan::new($id, $l..$r)
    };
    ($id:expr, $i:expr) => {
        crate::SourceSpan::at($id, $i)
    };
}

lalrpop_util::lalrpop_mod!(
    #[allow(clippy::all)]
    grammar,
    "/parser/grammar.rs"
);

mod error;
mod lexer;
mod scanner;
mod token;

use alloc::{boxed::Box, collections::BTreeSet, string::ToString, sync::Arc, vec::Vec};

pub use self::{
    error::{BinErrorKind, HexErrorKind, LiteralErrorKind, ParsingError},
    lexer::Lexer,
    scanner::Scanner,
    token::{BinEncodedValue, DocumentationType, HexEncodedValue, Token},
};
use crate::{
    ast,
    diagnostics::{Report, SourceFile, SourceSpan, Span, Spanned},
    sema, LibraryPath, SourceManager,
};

type ParseError<'a> = lalrpop_util::ParseError<u32, Token<'a>, ParsingError>;

// MODULE PARSER
// ================================================================================================

/// This is a wrapper around the lower-level parser infrastructure which handles orchestrating all
/// of the pieces needed to parse a [ast::Module] from source, and run semantic analysis on it.
#[derive(Default)]
pub struct ModuleParser {
    /// The kind of module we're parsing.
    ///
    /// This is used when performing semantic analysis to detect when various invalid constructions
    /// are encountered, such as use of the `syscall` instruction in a kernel module.
    kind: ast::ModuleKind,
    /// A set of interned strings allocated during parsing/semantic analysis.
    ///
    /// This is a very primitive and imprecise way of interning strings, but was the least invasive
    /// at the time the new parser was implemented. In essence, we avoid duplicating allocations
    /// for frequently occurring strings, by tracking which strings we've seen before, and
    /// sharing a reference counted pointer instead.
    ///
    /// We may want to replace this eventually with a proper interner, so that we can also gain the
    /// benefits commonly provided by interned string handles (e.g. cheap equality comparisons, no
    /// ref- counting overhead, copyable and of smaller size).
    ///
    /// Note that [Ident], [ProcedureName], [LibraryPath] and others are all implemented in terms
    /// of either the actual reference-counted string, e.g. `Arc<str>`, or in terms of [Ident],
    /// which is essentially the former wrapped in a [SourceSpan]. If we ever replace this with
    /// a better interner, we will also want to update those types to be in terms of whatever
    /// the handle type of the interner is.
    interned: BTreeSet<Arc<str>>,
    /// When true, all warning diagnostics are promoted to error severity
    warnings_as_errors: bool,
}

impl ModuleParser {
    /// Construct a new parser for the given `kind` of [ast::Module].
    pub fn new(kind: ast::ModuleKind) -> Self {
        Self {
            kind,
            interned: Default::default(),
            warnings_as_errors: false,
        }
    }

    /// Configure this parser so that any warning diagnostics are promoted to errors.
    pub fn set_warnings_as_errors(&mut self, yes: bool) {
        self.warnings_as_errors = yes;
    }

    /// Parse a [ast::Module] from `source`, and give it the provided `path`.
    pub fn parse(
        &mut self,
        path: LibraryPath,
        source: Arc<SourceFile>,
    ) -> Result<Box<ast::Module>, Report> {
        let forms = parse_forms_internal(source.clone(), &mut self.interned)
            .map_err(|err| Report::new(err).with_source_code(source.clone()))?;
        sema::analyze(source, self.kind, path, forms, self.warnings_as_errors).map_err(Report::new)
    }

    /// Parse a [ast::Module], `name`, from `path`.
    #[cfg(feature = "std")]
    pub fn parse_file<P>(
        &mut self,
        name: LibraryPath,
        path: P,
        source_manager: &dyn SourceManager,
    ) -> Result<Box<ast::Module>, Report>
    where
        P: AsRef<std::path::Path>,
    {
        use vm_core::debuginfo::SourceManagerExt;

        use crate::diagnostics::{IntoDiagnostic, WrapErr};

        let path = path.as_ref();
        let source_file = source_manager
            .load_file(path)
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to load source file from '{}'", path.display()))?;
        self.parse(name, source_file)
    }

    /// Parse a [ast::Module], `name`, from `source`.
    pub fn parse_str(
        &mut self,
        name: LibraryPath,
        source: impl ToString,
        source_manager: &dyn SourceManager,
    ) -> Result<Box<ast::Module>, Report> {
        use vm_core::debuginfo::SourceContent;

        let path = Arc::from(name.path().into_owned().into_boxed_str());
        let content = SourceContent::new(Arc::clone(&path), source.to_string().into_boxed_str());
        let source_file = source_manager.load_from_raw_parts(path, content);
        self.parse(name, source_file)
    }
}

/// This is used in tests to parse `source` as a set of raw [ast::Form]s rather than as a
/// [ast::Module].
///
/// NOTE: This does _not_ run semantic analysis.
#[cfg(any(test, feature = "testing"))]
pub fn parse_forms(source: Arc<SourceFile>) -> Result<Vec<ast::Form>, ParsingError> {
    let mut interned = BTreeSet::default();
    parse_forms_internal(source, &mut interned)
}

/// Parse `source` as a set of [ast::Form]s
///
/// Aside from catching syntax errors, this does little validation of the resulting forms, that is
/// handled by semantic analysis, which the caller is expected to perform next.
fn parse_forms_internal(
    source: Arc<SourceFile>,
    interned: &mut BTreeSet<Arc<str>>,
) -> Result<Vec<ast::Form>, ParsingError> {
    let source_id = source.id();
    let scanner = Scanner::new(source.as_str());
    let lexer = Lexer::new(source_id, scanner);
    grammar::FormsParser::new()
        .parse(&source, interned, core::marker::PhantomData, lexer)
        .map_err(|err| ParsingError::from_parse_error(source_id, err))
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use vm_core::assert_matches;

    use super::*;
    use crate::SourceId;

    // This test checks the lexer behavior with regard to tokenizing `exp(.u?[\d]+)?`
    #[test]
    fn lex_exp() {
        let source_id = SourceId::default();
        let scanner = Scanner::new("begin exp.u9 end");
        let mut lexer = Lexer::new(source_id, scanner).map(|result| result.map(|(_, t, _)| t));
        assert_matches!(lexer.next(), Some(Ok(Token::Begin)));
        assert_matches!(lexer.next(), Some(Ok(Token::ExpU)));
        assert_matches!(lexer.next(), Some(Ok(Token::Int(n))) if n == 9);
        assert_matches!(lexer.next(), Some(Ok(Token::End)));
    }

    #[test]
    fn lex_block() {
        let source_id = SourceId::default();
        let scanner = Scanner::new(
            "\
const.ERR1=1

begin
    u32assertw
    u32assertw.err=ERR1
    u32assertw.err=2
end
",
        );
        let mut lexer = Lexer::new(source_id, scanner).map(|result| result.map(|(_, t, _)| t));
        assert_matches!(lexer.next(), Some(Ok(Token::Const)));
        assert_matches!(lexer.next(), Some(Ok(Token::Dot)));
        assert_matches!(lexer.next(), Some(Ok(Token::ConstantIdent("ERR1"))));
        assert_matches!(lexer.next(), Some(Ok(Token::Equal)));
        assert_matches!(lexer.next(), Some(Ok(Token::Int(1))));
        assert_matches!(lexer.next(), Some(Ok(Token::Begin)));
        assert_matches!(lexer.next(), Some(Ok(Token::U32Assertw)));
        assert_matches!(lexer.next(), Some(Ok(Token::U32Assertw)));
        assert_matches!(lexer.next(), Some(Ok(Token::Dot)));
        assert_matches!(lexer.next(), Some(Ok(Token::Err)));
        assert_matches!(lexer.next(), Some(Ok(Token::Equal)));
        assert_matches!(lexer.next(), Some(Ok(Token::ConstantIdent("ERR1"))));
        assert_matches!(lexer.next(), Some(Ok(Token::U32Assertw)));
        assert_matches!(lexer.next(), Some(Ok(Token::Dot)));
        assert_matches!(lexer.next(), Some(Ok(Token::Err)));
        assert_matches!(lexer.next(), Some(Ok(Token::Equal)));
        assert_matches!(lexer.next(), Some(Ok(Token::Int(2))));
        assert_matches!(lexer.next(), Some(Ok(Token::End)));
        assert_matches!(lexer.next(), Some(Ok(Token::Eof)));
    }

    #[test]
    fn lex_emit() {
        let source_id = SourceId::default();
        let scanner = Scanner::new(
            "\
begin
    push.1
    emit.1
end
",
        );
        let mut lexer = Lexer::new(source_id, scanner).map(|result| result.map(|(_, t, _)| t));
        assert_matches!(lexer.next(), Some(Ok(Token::Begin)));
        assert_matches!(lexer.next(), Some(Ok(Token::Push)));
        assert_matches!(lexer.next(), Some(Ok(Token::Dot)));
        assert_matches!(lexer.next(), Some(Ok(Token::Int(1))));
        assert_matches!(lexer.next(), Some(Ok(Token::Emit)));
        assert_matches!(lexer.next(), Some(Ok(Token::Dot)));
        assert_matches!(lexer.next(), Some(Ok(Token::Int(1))));
        assert_matches!(lexer.next(), Some(Ok(Token::End)));
        assert_matches!(lexer.next(), Some(Ok(Token::Eof)));
    }
}
