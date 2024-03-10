use crate::Felt;
use alloc::string::String;
use core::{num::IntErrorKind, ops::Range};

use super::{
    DocumentationType, HexEncodedValue, HexErrorKind, LiteralErrorKind, ParsingError, Scanner,
    SourceSpan, Token,
};

/// The value produced by the [Lexer] when iterated
pub type Lexed<'input> = Result<(u32, Token<'input>, u32), ParsingError>;

/// Pops a single token from the [Lexer]
macro_rules! pop {
    ($lex:ident) => {{
        $lex.skip();
    }};
    ($lex:ident, $token:expr) => {{
        $lex.skip();
        Ok($token)
    }};
}

/// Pops two tokens from the [Lexer]
macro_rules! pop2 {
    ($lex:ident) => {{
        $lex.skip();
        $lex.skip();
    }};
    ($lex:ident, $token:expr) => {{
        $lex.skip();
        $lex.skip();
        Ok($token)
    }};
}

/// The lexer that is used to perform lexical analysis Miden Assembly grammar. The lexer implements
/// the `Iterator` trait, so in order to retrieve the tokens, you simply have to iterate over it.
///
/// # Errors
///
/// Because the lexer is implemented as an iterator over tokens, this means that you can continue
/// to get tokens even if a lexical error occurs. The lexer will attempt to recover from an error
/// by injecting tokens it expects.
///
/// If an error is unrecoverable, the lexer will continue to produce tokens, but there is no
/// guarantee that parsing them will produce meaningful results, it is primarily to assist in
/// gathering as many errors as possible.
pub struct Lexer<'input> {
    /// The scanner produces a sequence of chars + location, and can be controlled
    /// The location type is usize
    scanner: Scanner<'input>,

    /// The most recent token to be lexed.
    /// At the start and end, this should be Token::Eof
    token: Token<'input>,

    /// The position in the input where the current token starts
    /// At the start this will be the byte index of the beginning of the input
    token_start: usize,

    /// The position in the input where the current token ends
    /// At the start this will be the byte index of the beginning of the input
    token_end: usize,

    /// The current line number
    line_num: usize,

    /// When we have reached true Eof, this gets set to true, and the only token
    /// produced after that point is Token::Eof, or None, depending on how you are
    /// consuming the lexer
    eof: bool,
    empty: bool,

    // A DFA for keyword matching
    keywords: aho_corasick::AhoCorasick,

    /// If an error occurs during tokenization, it is held here
    error: Option<ParsingError>,
}

impl<'input> Lexer<'input> {
    /// Produces an instance of the lexer with the lexical analysis to be performed on the `input`
    /// string. Note that no lexical analysis occurs until the lexer has been iterated over.
    pub fn new(scanner: Scanner<'input>) -> Self {
        let start = scanner.start();
        let keywords = Token::keyword_searcher();
        let mut lexer = Self {
            scanner,
            token: Token::Eof,
            token_start: start,
            token_end: start,
            line_num: 0,
            eof: false,
            empty: false,
            keywords,
            error: None,
        };
        lexer.advance();
        lexer
    }

    pub fn lex(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(err) = self.error.take() {
            return Some(Err(err));
        }

        if self.eof && matches!(self.token, Token::Eof) {
            // Emit a single Eof token at the end, then None after
            if self.empty {
                return None;
            } else {
                self.empty = true;
                let end = self.token_end as u32;
                return Some(Ok((end, Token::Eof, end)));
            }
        }

        let token = core::mem::replace(&mut self.token, Token::Eof);
        let start = self.token_start;
        let end = self.token_end;
        self.advance();
        Some(Ok((start as u32, token, end as u32)))
    }

    fn advance(&mut self) {
        self.advance_start();
        match self.tokenize() {
            Ok(tok) => {
                self.token = tok;
            }
            Err(err) => {
                self.error = Some(err);
            }
        }
    }

    #[inline]
    fn advance_start(&mut self) {
        let mut position: usize;
        loop {
            let (pos, c) = self.scanner.read();

            position = pos;

            if c == '\0' {
                self.eof = true;
                return;
            }

            if c.is_whitespace() {
                if c == '\n' {
                    self.line_num += 1;
                }
                self.scanner.advance();
                continue;
            }

            break;
        }

        self.token_start = position;
    }

    #[inline]
    fn pop(&mut self) -> char {
        let (pos, c) = self.scanner.pop();
        self.token_end = pos + c.len_utf8();
        c
    }

    #[inline]
    fn peek(&mut self) -> char {
        let (_, c) = self.scanner.peek();
        c
    }

    #[inline]
    #[allow(unused)]
    fn peek_next(&mut self) -> char {
        let (_, c) = self.scanner.peek_next();
        c
    }

    #[inline]
    fn read(&mut self) -> char {
        let (_, c) = self.scanner.read();
        c
    }

    #[inline]
    fn skip(&mut self) {
        self.pop();
    }

    /// Get the span for the current token in `Source`.
    #[inline]
    fn span(&self) -> SourceSpan {
        assert!(self.token_start <= self.token_end, "invalid range");
        assert!(self.token_end <= u32::MAX as usize, "file too large");
        SourceSpan::from((self.token_start as u32)..(self.token_end as u32))
    }

    #[inline]
    fn slice_span(&self, span: impl Into<Range<u32>>) -> &'input str {
        let range = span.into();
        self.scanner.slice((range.start as usize)..(range.end as usize))
    }

    /// Get a string slice of the current token.
    #[inline]
    fn slice(&self) -> &'input str {
        self.slice_span(self.span())
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        let mut c: char;
        loop {
            c = self.read();

            if !c.is_whitespace() {
                break;
            }

            if c == '\n' {
                self.line_num += 1;
            }

            self.skip();
        }
    }

    fn tokenize(&mut self) -> Result<Token<'input>, ParsingError> {
        let c = self.read();

        if c == '#' {
            match self.peek() {
                '!' => {
                    self.skip();
                    self.skip();
                    return self.lex_docs();
                }
                _ => {
                    self.skip();
                    self.skip_comment();
                    return Ok(Token::Comment);
                }
            }
        }

        if c == '\0' {
            self.eof = true;
            return Ok(Token::Eof);
        }

        if c.is_whitespace() {
            self.skip_whitespace();
        }

        match self.read() {
            '!' => pop!(self, Token::Bang),
            ':' => match self.peek() {
                ':' => pop2!(self, Token::ColonColon),
                _ => Err(ParsingError::InvalidToken { span: self.span() }),
            },
            '.' => pop!(self, Token::Dot),
            '=' => pop!(self, Token::Equal),
            '(' => pop!(self, Token::Lparen),
            ')' => pop!(self, Token::Rparen),
            '-' => match self.peek() {
                '>' => pop2!(self, Token::Rstab),
                _ => pop!(self, Token::Minus),
            },
            '+' => pop!(self, Token::Plus),
            '/' => match self.peek() {
                '/' => pop2!(self, Token::SlashSlash),
                _ => pop!(self, Token::Slash),
            },
            '*' => pop!(self, Token::Star),
            '"' => self.lex_quoted_identifier(),
            '0' => match self.peek() {
                'x' => {
                    self.skip();
                    self.skip();
                    self.lex_hex()
                }
                '0'..='9' => self.lex_number(),
                _ => pop!(self, Token::Int(0)),
            },
            '1'..='9' => self.lex_number(),
            'a'..='z' => self.lex_keyword_or_ident(),
            'A'..='Z' => self.lex_const_identifier(),
            '_' => match self.peek() {
                c if c.is_ascii_alphanumeric() => self.lex_identifier(),
                _ => Err(ParsingError::InvalidToken { span: self.span() }),
            },
            _ => Err(ParsingError::InvalidToken { span: self.span() }),
        }
    }

    fn lex_docs(&mut self) -> Result<Token<'input>, ParsingError> {
        let mut buf = String::new();

        let mut c;
        let mut line_start = self.token_start + 2;
        let is_module_doc = self.line_num == 0;
        loop {
            c = self.read();

            if c == '\0' {
                self.eof = true;
                buf.push_str(self.slice_span((line_start as u32)..(self.token_end as u32)).trim());

                let is_first_line = self.line_num == 0;
                break Ok(Token::DocComment(if is_first_line {
                    DocumentationType::Module(buf)
                } else {
                    DocumentationType::Form(buf)
                }));
            }

            if c == '\n' {
                self.line_num += 1;

                buf.push_str(self.slice_span((line_start as u32)..(self.token_end as u32)).trim());
                buf.push('\n');

                self.skip();
                c = self.read();
                match c {
                    '#' if self.peek() == '!' => {
                        self.skip();
                        self.skip();
                        line_start = self.token_end;
                        continue;
                    }
                    _ if is_module_doc => {
                        break Ok(Token::DocComment(DocumentationType::Module(buf)));
                    }
                    _ => {
                        break Ok(Token::DocComment(DocumentationType::Form(buf)));
                    }
                }
            }

            self.skip();
        }
    }

    fn skip_comment(&mut self) {
        let mut c;
        loop {
            c = self.read();

            if c == '\n' {
                self.skip();
                self.line_num += 1;
                break;
            }

            if c == '\0' {
                self.eof = true;
                break;
            }

            self.skip();
        }
    }

    fn lex_keyword_or_ident(&mut self) -> Result<Token<'input>, ParsingError> {
        let c = self.pop();
        debug_assert!(c.is_ascii_alphabetic() && c.is_lowercase());

        loop {
            match self.read() {
                '_' | '0'..='9' => self.skip(),
                c if c.is_ascii_alphabetic() => self.skip(),
                _ => break,
            }
        }

        let name = self.slice();
        match name {
            "exp" => {
                // Special handling for the `exp.uXX` tokenizatioon
                if self.read() == '.' && self.peek() == 'u' {
                    pop2!(self, Token::ExpU)
                } else {
                    Ok(Token::Exp)
                }
            }
            _ => Ok(Token::from_keyword_or_ident_with_searcher(name, &self.keywords)),
        }
    }

    fn lex_quoted_identifier(&mut self) -> Result<Token<'input>, ParsingError> {
        // Skip quotation mark
        self.skip();

        let quote_size = '"'.len_utf8() as u32;
        loop {
            match self.read() {
                '\0' | '\n' => {
                    break Err(ParsingError::UnclosedQuote {
                        start: SourceSpan::at(self.span().start() as u32),
                    });
                }
                '"' => {
                    let span = self.span();
                    let start = span.start() as u32 + quote_size;
                    let span = SourceSpan::from(start..(span.end() as u32));

                    self.skip();
                    break Ok(Token::QuotedIdent(self.slice_span(span)));
                }
                c if c.is_ascii_alphanumeric() => {
                    self.skip();
                    continue;
                }
                '_' | '.' | '$' => {
                    self.skip();
                    continue;
                }
                c => {
                    let loc = self.span().end() - c.len_utf8();
                    break Err(ParsingError::InvalidIdentCharacter {
                        span: SourceSpan::at(loc as u32),
                    });
                }
            }
        }
    }

    fn lex_identifier(&mut self) -> Result<Token<'input>, ParsingError> {
        let c = self.pop();
        debug_assert!(c.is_ascii_lowercase() || c == '_');

        loop {
            match self.read() {
                '_' | '0'..='9' => self.skip(),
                c if c.is_ascii_lowercase() => self.skip(),
                _ => break,
            }
        }

        Ok(Token::Ident(self.slice()))
    }

    fn lex_const_identifier(&mut self) -> Result<Token<'input>, ParsingError> {
        let c = self.pop();
        debug_assert!(c.is_ascii_uppercase() || c == '_');

        loop {
            match self.read() {
                '_' | '0'..='9' => self.skip(),
                c if c.is_ascii_uppercase() => self.skip(),
                _ => break,
            }
        }

        Ok(Token::ConstantIdent(self.slice()))
    }

    fn lex_number(&mut self) -> Result<Token<'input>, ParsingError> {
        // Expect the first character to be a digit or sign
        let c = self.read();
        debug_assert!(c.is_ascii_digit());

        while let '0'..='9' = self.read() {
            self.skip();
        }

        self.slice()
            .parse::<u64>()
            .map_err(|error| ParsingError::InvalidLiteral {
                span: self.span(),
                kind: int_error_kind_to_literal_error_kind(
                    error.kind(),
                    LiteralErrorKind::FeltOverflow,
                ),
            })
            .map(Token::Int)
    }

    fn lex_hex(&mut self) -> Result<Token<'input>, ParsingError> {
        // Expect the first character to be a valid hexadecimal digit
        debug_assert!(self.read().is_ascii_hexdigit());

        loop {
            // If we hit a non-hex digit, we're done
            let c1 = self.read();
            if !c1.is_ascii_hexdigit() {
                break;
            }
            self.skip();

            // All hex-encoded bytes are zero-padded, and thus occur
            // in pairs, if we observe a non-hex digit at this point,
            // it is invalid
            let c2 = self.read();
            if !c2.is_ascii_hexdigit() {
                return Err(ParsingError::InvalidHexLiteral {
                    span: self.span(),
                    kind: HexErrorKind::Invalid,
                });
            }
            self.skip();
        }

        let span = self.span();
        let start = span.start() as u32;
        let digit_start = start + 2;
        let end = span.end() as u32;
        let span = SourceSpan::from(start..end);
        let value = parse_hex(span, self.slice_span(digit_start..end))?;
        Ok(Token::HexValue(value))
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Lexed<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = self.lex();
        while let Some(Ok((_, Token::Comment, _))) = res {
            res = self.lex();
        }
        res
    }
}

fn parse_hex(span: SourceSpan, hex_digits: &str) -> Result<HexEncodedValue, ParsingError> {
    use vm_core::{FieldElement, StarkField};
    match hex_digits.len() {
        n if n <= 16 && n % 2 == 0 => {
            let value = u64::from_str_radix(hex_digits, 16).map_err(|error| {
                ParsingError::InvalidLiteral {
                    span,
                    kind: int_error_kind_to_literal_error_kind(
                        error.kind(),
                        LiteralErrorKind::FeltOverflow,
                    ),
                }
            })?;
            if value > Felt::MODULUS {
                return Err(ParsingError::InvalidLiteral {
                    span,
                    kind: LiteralErrorKind::FeltOverflow,
                });
            }
            Ok(shrink_u64(value))
        }
        // Word
        64 => {
            let mut word = [Felt::ZERO; 4];
            for (index, element) in word.iter_mut().enumerate() {
                let offset = index * 16;
                let mut felt_bytes = [0u8; 8];
                let digits = &hex_digits[offset..(offset + 16)];
                for (byte_idx, byte) in felt_bytes.iter_mut().enumerate() {
                    let byte_str = &digits[(byte_idx * 2)..((byte_idx * 2) + 2)];
                    *byte = u8::from_str_radix(byte_str, 16).map_err(|error| {
                        ParsingError::InvalidLiteral {
                            span,
                            kind: int_error_kind_to_literal_error_kind(
                                error.kind(),
                                LiteralErrorKind::FeltOverflow,
                            ),
                        }
                    })?;
                }
                let value = u64::from_le_bytes(felt_bytes);
                if value > Felt::MODULUS {
                    return Err(ParsingError::InvalidLiteral {
                        span,
                        kind: LiteralErrorKind::FeltOverflow,
                    });
                }
                *element = Felt::new(value);
            }
            Ok(HexEncodedValue::Word(word))
        }
        // Invalid
        n if n > 64 => Err(ParsingError::InvalidHexLiteral {
            span,
            kind: HexErrorKind::TooLong,
        }),
        n if n % 2 != 0 && n < 64 => Err(ParsingError::InvalidHexLiteral {
            span,
            kind: HexErrorKind::MissingDigits,
        }),
        _ => Err(ParsingError::InvalidHexLiteral {
            span,
            kind: HexErrorKind::Invalid,
        }),
    }
}

#[inline]
fn shrink_u64(n: u64) -> HexEncodedValue {
    if n <= (u8::MAX as u64) {
        HexEncodedValue::U8(n as u8)
    } else if n <= (u16::MAX as u64) {
        HexEncodedValue::U16(n as u16)
    } else if n <= (u32::MAX as u64) {
        HexEncodedValue::U32(n as u32)
    } else {
        HexEncodedValue::Felt(Felt::new(n))
    }
}

#[inline]
fn int_error_kind_to_literal_error_kind(
    kind: &IntErrorKind,
    overflow: LiteralErrorKind,
) -> LiteralErrorKind {
    match kind {
        IntErrorKind::Empty => LiteralErrorKind::Empty,
        IntErrorKind::InvalidDigit => LiteralErrorKind::InvalidDigit,
        IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => overflow,
        _ => unreachable!(),
    }
}
