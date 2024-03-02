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
mod location;
mod span;
mod token;

pub use self::error::{HexErrorKind, LiteralErrorKind, ParsingError};
pub use self::location::SourceLocation;
pub use self::span::{SourceSpan, Span, Spanned};
pub use self::token::{DocumentationType, HexEncodedValue, Token};

use crate::{
    ast,
    diagnostics::{Report, SourceFile},
    sema, LibraryPath,
};
use alloc::{boxed::Box, collections::BTreeSet, sync::Arc, vec::Vec};

type ParseError<'a> = lalrpop_util::ParseError<u32, Token<'a>, ParsingError>;

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
    use logos::Logos;

    let eof = source.inner().as_bytes().len() as u32;
    let lexer = Token::lexer(source.inner())
        .spanned()
        .map(|(t, span)| t.map(|t| (span.start as u32, t, span.end as u32)))
        .chain(core::iter::once(Ok((eof, Token::Eof, eof))));
    grammar::FormsParser::new()
        .parse(&source, interned, core::marker::PhantomData, lexer)
        .map_err(error::handle_parse_error)
}
