use super::{BTreeMap, LinesStream, ParsingError, SourceLocation, String, Token, Vec};
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
        let mut module_comment = None;

        // fetch all tokens
        for info in LinesStream::from(source) {
            let offset = info.char_offset();
            let mut location = SourceLocation::new(info.line_number(), 1 + offset);

            // fetch contents line
            let mut contents = match info.contents() {
                // if not first token & has docs without being export or proc, then dangling
                Some(contents)
                    if !(tokens.is_empty()
                        || info.docs().is_empty()
                        || contents.trim().starts_with(Token::EXPORT)
                        || contents.trim().starts_with(Token::PROC)) =>
                {
                    return Err(ParsingError::dangling_procedure_comment(location));
                }

                Some(contents) => contents,

                // first dangling comments are module docs
                None if tokens.is_empty() => {
                    module_comment = build_comment(info.docs());
                    continue;
                }

                // other dangling docs are forbidden
                None => {
                    return Err(ParsingError::dangling_procedure_comment(location));
                }
            };

            while !contents.is_empty() {
                // ignore comments; halt if dangling comment
                if contents.starts_with(Token::DOC_COMMENT_PREFIX) {
                    return Err(ParsingError::dangling_procedure_comment(location));
                } else if contents.starts_with(Token::COMMENT_PREFIX) {
                    break;
                }

                // fill the doc comments for procedures
                if contents.starts_with(Token::EXPORT) || contents.starts_with(Token::PROC) {
                    proc_comments.insert(tokens.len(), build_comment(info.docs()));
                }

                // pick the current token & remainder
                let (token, remainder) = match contents.split_once(char::is_whitespace) {
                    Some(split) => split,

                    // last token; push and break
                    None => {
                        tokens.push(contents);
                        locations.push(location);
                        break;
                    }
                };

                // append the token
                tokens.push(token);
                locations.push(location);

                // seek next token
                let n = match remainder.find(|c: char| !c.is_whitespace()) {
                    Some(n) => n,
                    None => break,
                };

                // update the offset; add extra char consumed by `split_once`
                location.move_column(token.len() as u32 + n as u32 + 1);
                contents = remainder.split_at(n).1;
            }
        }

        // invalid if no tokens
        if tokens.is_empty() {
            return Err(ParsingError::empty_source());
        }

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
