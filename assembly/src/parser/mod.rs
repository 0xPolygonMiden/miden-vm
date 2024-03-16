/// Simple macro used in the grammar definition for constructing spans
macro_rules! span {
    ($l:expr, $r:expr) => {
        crate::SourceSpan::new($l..$r)
    };
    ($i:expr) => {
        crate::SourceSpan::new($i..$i)
    };
}

lalrpop_util::lalrpop_mod!(
    #[allow(clippy::all)]
    grammar,
    "/parser/grammar.rs"
);

mod error;
mod lexer;
mod location;
mod scanner;
mod span;
mod token;

pub use self::error::{HexErrorKind, LiteralErrorKind, ParsingError};
pub use self::lexer::Lexer;
pub use self::location::SourceLocation;
pub use self::scanner::Scanner;
pub use self::span::{SourceSpan, Span, Spanned};
pub use self::token::{DocumentationType, HexEncodedValue, Token};

use crate::{
    ast,
    diagnostics::{Report, SourceFile},
    sema, LibraryPath,
};
use alloc::{boxed::Box, collections::BTreeSet, sync::Arc, vec::Vec};

type ParseError<'a> = lalrpop_util::ParseError<u32, Token<'a>, ParsingError>;

// MODULE PARSER
// ================================================================================================

/// This is a wrapper around the lower-level parser infrastructure which handles orchestrating all
/// of the pieces needed to parse a [Module] from source, and run semantic analysis on it.
///
/// In the vast majority of cases though, you will want to use the more ergonomic [Module::parse_str]
/// or [Module::parse_file] APIs instead.
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
    /// at the time the new parser was implemented. In essence, we avoid duplicating allocations for
    /// frequently occuring strings, by tracking which strings we've seen before, and sharing a reference
    /// counted pointer instead.
    ///
    /// We may want to replace this eventually with a proper interner, so that we can also gain the
    /// benefits commonly provided by interned string handles (e.g. cheap equality comparisons, no ref-
    /// counting overhead, copyable and of smaller size).
    ///
    /// Note that [Ident], [ProcedureName], [LibraryPath] and others are all implemented in terms of
    /// either the actual reference-counted string, e.g. `Arc<str>`, or in terms of [Ident], which is
    /// essentially the former wrapped in a [SourceSpan]. If we ever replace this with a better interner,
    /// we will also want to update those types to be in terms of whatever the handle type of the interner
    /// is.
    interned: BTreeSet<Arc<str>>,
}
impl ModuleParser {
    /// Construct a new parser for the given `kind` of [Module]
    pub fn new(kind: ast::ModuleKind) -> Self {
        Self {
            kind,
            interned: Default::default(),
        }
    }

    /// Parse a [Module] from `source`, and give it the provided `path`.
    pub fn parse(
        &mut self,
        path: LibraryPath,
        source: Arc<SourceFile>,
    ) -> Result<Box<ast::Module>, Report> {
        let forms = parse_forms_internal(source.clone(), &mut self.interned)
            .map_err(|err| Report::new(err).with_source_code(source.clone()))?;
        sema::analyze(source, self.kind, path, forms).map_err(Report::new)
    }
}

/// This is used in tests to parse `source` as a set of raw [ast::Form]s rather than as a [Module].
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
    let scanner = Scanner::new(source.inner().as_ref());
    let lexer = Lexer::new(scanner);
    grammar::FormsParser::new()
        .parse(&source, interned, core::marker::PhantomData, lexer)
        .map_err(error::handle_parse_error)
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::assert_matches;

    // This test checks the lexer behavior with regard to tokenizing `exp(.u?[\d]+)?`
    #[test]
    fn lex_exp() {
        let scanner = Scanner::new("begin exp.u9 end");
        let mut lexer = Lexer::new(scanner).map(|result| result.map(|(_, t, _)| t));
        assert_matches!(lexer.next(), Some(Ok(Token::Begin)));
        assert_matches!(lexer.next(), Some(Ok(Token::ExpU)));
        assert_matches!(lexer.next(), Some(Ok(Token::Int(n))) if n == 9);
        assert_matches!(lexer.next(), Some(Ok(Token::End)));
    }

    #[test]
    fn lex_block() {
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
        let mut lexer = Lexer::new(scanner).map(|result| result.map(|(_, t, _)| t));
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
        let scanner = Scanner::new(
            "\
begin
    push.1
    emit.1
end
",
        );
        let mut lexer = Lexer::new(scanner).map(|result| result.map(|(_, t, _)| t));
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
