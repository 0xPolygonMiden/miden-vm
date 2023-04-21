use super::{BTreeMap, LinesStream, ParsingError, String, Token, Vec};
use core::fmt;

// TOKEN STREAM
// ================================================================================================

#[derive(Debug)]
pub struct TokenStream<'a> {
    tokens: Vec<&'a str>,
    lines: Vec<usize>,
    current: Token<'a>,
    pos: usize,
    temp: Token<'a>,
    proc_comments: BTreeMap<usize, Option<String>>,
    module_comment: Option<String>,
}

impl<'a> TokenStream<'a> {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// TODO: add comments
    pub fn new(source: &'a str) -> Result<Self, ParsingError> {
        // initialize the attributes
        let mut tokens = Vec::new();
        let mut lines = Vec::new();
        let mut proc_comments = BTreeMap::new();
        let mut module_comment = None;

        // fetch all tokens
        for line_info in LinesStream::from(source) {
            let line_number = line_info.line_number() as usize;

            match line_info.contents() {
                Some(line) => {
                    // fill the doc comments for procedures
                    if line.starts_with(Token::EXPORT) || line.starts_with(Token::PROC) {
                        let doc_comment = build_comment(line_info.docs());
                        proc_comments.insert(tokens.len(), doc_comment);
                    } else if !line_info.docs().is_empty() {
                        return Err(ParsingError::dangling_procedure_comment(line_number));
                    }

                    // for each token, skip comments & err when dangling docs; push otherwise
                    for token in line.split_whitespace() {
                        if token.starts_with(Token::DOC_COMMENT_PREFIX) {
                            return Err(ParsingError::dangling_procedure_comment(line_number));
                        } else if token.starts_with(Token::COMMENT_PREFIX) {
                            break;
                        }

                        tokens.push(token);
                    }
                }

                // if first dangling comment, then module docs
                // TODO consider using a dedicated symbol for module docs such as `//!`
                None if tokens.is_empty() => {
                    module_comment = build_comment(line_info.docs());
                }

                // if has tokens, then dangling docs are illegal
                None => {
                    return Err(ParsingError::dangling_procedure_comment(
                        line_info.line_number() as usize
                    ));
                }
            }

            // extend lines until it fits the added tokens
            lines.resize(tokens.len(), line_number);
        }

        // invalid if no tokens
        if tokens.is_empty() {
            return Err(ParsingError::empty_source());
        }

        let current = Token::new(tokens[0], 1);
        Ok(Self {
            tokens,
            lines,
            current,
            pos: 0,
            temp: Token::default(),
            proc_comments,
            module_comment,
        })
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns position of the current token in this stream.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Returns the current lines count for the stream.
    pub fn num_lines(&self) -> usize {
        self.lines[self.pos.min(self.lines.len().saturating_sub(1))]
    }

    /// Returns 'true' all tokens from this stream have been read.
    pub fn eof(&self) -> bool {
        self.pos == self.tokens.len()
    }

    // TOKEN READERS
    // --------------------------------------------------------------------------------------------

    /// Returns a token from this stream located at the current position. If all the tokens have
    /// been read, returns None.
    pub fn read(&self) -> Option<&Token> {
        if self.eof() {
            None
        } else {
            Some(&self.current)
        }
    }

    /// Returns a token read from the specified position. The token must have been previously
    /// read.
    ///
    /// # Panics
    /// Panics if the specified position is greater than the current token position in the stream.
    pub fn read_at(&mut self, pos: usize) -> Option<&Token> {
        assert!(pos <= self.pos, "cannot read from future positions");
        if pos == self.pos {
            self.read()
        } else {
            self.temp.update(self.tokens[pos], self.lines[pos]);
            Some(&self.temp)
        }
    }

    /// Increments the current token position by one. If the stream is at EOF, this is noop.
    pub fn advance(&mut self) {
        if !self.eof() {
            self.pos += 1;
            if !self.eof() {
                self.current.update(self.tokens[self.pos], self.lines[self.pos]);
            }
        }
    }

    pub fn take_doc_comment_at(&mut self, pos: usize) -> Option<String> {
        self.proc_comments.remove(&pos)?
    }

    pub fn take_module_comments(self) -> Option<String> {
        self.module_comment
    }
}

impl<'a> fmt::Display for TokenStream<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.tokens[self.pos..])
    }
}

// HELPERS
// ================================================================================================

fn build_comment(docs: &[&str]) -> Option<String> {
    let last = docs.len().saturating_sub(1);
    let docs: String = docs
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let lb = if last == i { "" } else { "\n" };
            format!("{d}{lb}")
        })
        .collect();
    (!docs.is_empty()).then_some(docs)
}
