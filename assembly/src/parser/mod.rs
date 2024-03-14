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

#[derive(Default)]
pub struct ModuleParser {
    kind: ast::ModuleKind,
    interned: BTreeSet<Arc<str>>,
}
impl ModuleParser {
    pub fn new(kind: ast::ModuleKind) -> Self {
        Self {
            kind,
            interned: Default::default(),
        }
    }

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

#[cfg(any(test, feature = "testing"))]
pub fn parse_forms(source: Arc<SourceFile>) -> Result<Vec<ast::Form>, ParsingError> {
    let mut interned = BTreeSet::default();
    parse_forms_internal(source, &mut interned)
}

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
