use super::{BTreeMap, LineToken, LineTokenizer, ParsingError, SourceLocation, String, Token, Vec};
use core::fmt;

// TOKEN STREAM
// ================================================================================================

#[derive(Debug)]
pub struct TokenStream<'a> {
    tokens: Vec<&'a str>,
    locations: Vec<SourceLocation>,
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
        let mut locations = Vec::new();
        let mut proc_comments = BTreeMap::new();
        let mut tokenizer = LineTokenizer::from(source);

        for token in tokenizer.by_ref() {
            let LineToken {
                docs,
                location,
                token,
            } = token;

            // bind doc comments to proc/export; halt otherwise
            if token.starts_with(Token::EXPORT) || token.starts_with(Token::PROC) {
                proc_comments.insert(tokens.len(), docs);
            } else if docs.is_some() {
                return Err(ParsingError::dangling_procedure_comment(location));
            }

            tokens.push(token);
            locations.push(location);
        }

        // halt if dangling docs
        if let Some(location) = tokenizer.take_dangling_docs() {
            return Err(ParsingError::dangling_procedure_comment(location));
        }

        // invalid if no tokens
        if tokens.is_empty() {
            return Err(ParsingError::empty_source());
        }

        // set module comment & return
        let module_comment = tokenizer.take_module_docs();
        let location = SourceLocation::default();
        let current = Token::new(tokens[0], location);
        Ok(Self {
            tokens,
            locations,
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

    /// Returns the [SourceLocation] linked to the current [Token].
    pub fn location(&self) -> &SourceLocation {
        let idx = self.pos.min(self.locations.len().saturating_sub(1));
        &self.locations[idx]
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
            self.temp.update(self.tokens[pos], self.locations[pos]);
            Some(&self.temp)
        }
    }

    /// Increments the current token position by one. If the stream is at EOF, this is noop.
    pub fn advance(&mut self) {
        if !self.eof() {
            self.pos += 1;
            if !self.eof() {
                self.current.update(self.tokens[self.pos], self.locations[self.pos]);
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
