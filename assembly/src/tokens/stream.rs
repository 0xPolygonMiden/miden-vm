use super::{BTreeMap, ParsingError, String, Token, Vec};
use core::fmt;

pub const DOC_COMMENT_PREFIX: &str = "#!";
pub const LINE_COMMENT_PREFIX: &str = "#";

// TOKEN STREAM
// ================================================================================================

#[derive(Debug)]
pub struct TokenStream<'a> {
    tokens: Vec<&'a str>,
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
        if source.is_empty() {
            return Err(ParsingError::empty_source());
        }
        let mut tokens = Vec::new();
        let mut proc_comments = BTreeMap::new();
        let mut module_comment = None;

        let mut comment_builder = CommentBuilder(None);

        for line in source.lines() {
            let line = line.trim();
            if line.starts_with(DOC_COMMENT_PREFIX) {
                comment_builder.append_line(line);
            } else if line.starts_with(LINE_COMMENT_PREFIX) {
                continue;
            } else if line.is_empty() {
                if !comment_builder.is_empty() {
                    if tokens.is_empty() && module_comment.is_none() {
                        // if we haven't read any tokens yet, but already have built a comment, a
                        // new line must indicate the end of a module comment.
                        module_comment = comment_builder.take_comment();
                    } else {
                        // since we already have a module comment, this is a procedure comment
                        // which is followed by a blank line.
                        return Err(ParsingError::dangling_procedure_comment(tokens.len()));
                    }
                }
            } else {
                let mut line_tokens = line
                    .split_whitespace()
                    .take_while(|&token| !token.starts_with(LINE_COMMENT_PREFIX))
                    .collect::<Vec<_>>();

                if !comment_builder.is_empty() {
                    // procedure comment should always be followed by a procedure token
                    debug_assert!(!line_tokens.is_empty());
                    let token = line_tokens[0];
                    if token.starts_with(Token::EXPORT) || token.starts_with(Token::PROC) {
                        proc_comments.insert(tokens.len(), comment_builder.take_comment());
                    } else {
                        return Err(ParsingError::dangling_procedure_comment(tokens.len()));
                    }
                }
                tokens.append(&mut line_tokens);
            }
        }

        if tokens.is_empty() {
            return Err(ParsingError::empty_source());
        }
        let current = Token::new(tokens[0], 0);
        Ok(Self {
            tokens,
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
            self.temp.update(self.tokens[pos], pos);
            Some(&self.temp)
        }
    }

    /// Increments the current token position by one. If the stream is at EOF, this is noop.
    pub fn advance(&mut self) {
        if !self.eof() {
            self.pos += 1;
            if !self.eof() {
                self.current.update(self.tokens[self.pos], self.pos);
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

#[derive(Debug)]
pub struct CommentBuilder(Option<String>);

impl CommentBuilder {
    pub fn append_line(&mut self, line: &str) {
        let prepared_line = prepare_line(line);
        if !prepared_line.is_empty() {
            match &mut self.0 {
                Some(comment) => {
                    comment.push('\n');
                    comment.push_str(prepared_line);
                }
                None => {
                    self.0 = Some(String::from(prepared_line));
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn take_comment(&mut self) -> Option<String> {
        self.0.take()
    }
}

/// Removes `prefix` from provided `line` and trims additional whitespaces from start and end of
/// the `line`
pub fn prepare_line(line: &str) -> &str {
    // We should panic if strip_prefix returns None since it is our internal parsing error
    line.trim()
        .strip_prefix(DOC_COMMENT_PREFIX)
        .expect("Current line is not a comment")
        .trim()
}
