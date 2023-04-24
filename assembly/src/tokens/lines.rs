use super::{SourceLocation, Token, Vec};
use core::{iter, str::Lines};

// LINES STREAM
// ================================================================================================

/// A [LineInfo] iterator that will bind lines with tokens with doc comments.
#[derive(Debug, Clone)]
pub struct LinesStream<'a> {
    lines: Lines<'a>,
    current_line: Option<&'a str>,
    current_line_num: u32,
    line_char_offset: u32,
}

impl<'a> From<&'a str> for LinesStream<'a> {
    fn from(contents: &'a str) -> Self {
        Self {
            lines: contents.lines(),
            current_line: None,
            current_line_num: 0,
            line_char_offset: 0,
        }
    }
}

impl<'a> LinesStream<'a> {
    // HELPERS
    // --------------------------------------------------------------------------------------------

    /// Returns true if the current line is a token.
    fn is_token(&self) -> bool {
        self.current_line
            .filter(|line| !line.is_empty() && !line.starts_with(Token::COMMENT_PREFIX))
            .is_some()
    }

    /// Returns true if the current line is a token or a doc comment.
    fn is_token_or_doc_comment(&self) -> bool {
        self.current_line
            .filter(|line| {
                !line.is_empty() && !line.starts_with(Token::COMMENT_PREFIX)
                    || line.starts_with(Token::DOC_COMMENT_PREFIX)
            })
            .is_some()
    }

    /// Move the pointer to the next line, updating the control variables
    fn go_to_next_line(&mut self) {
        self.current_line = self.lines.next();
        if let Some(line) = self.current_line {
            let init_len = line.len();
            let trimmed = line.trim_start();

            self.current_line.replace(trimmed);
            self.line_char_offset = (init_len - trimmed.len()) as u32;
            self.current_line_num += 1;
        }
    }

    /// If the current line is a doc comment, take lines until EOF or not doc comment.
    fn take_docs_block(&mut self) -> Vec<&'a str> {
        iter::from_fn(|| {
            self.current_line
                .and_then(|line| line.strip_prefix(Token::DOC_COMMENT_PREFIX))
                .map(|doc| doc.trim())
                .map(|doc| {
                    self.go_to_next_line();
                    doc
                })
        })
        .fold(Vec::with_capacity(10), |mut v, doc| {
            if !doc.trim().is_empty() {
                v.push(doc)
            }
            v
        })
    }
}

impl<'a> Iterator for LinesStream<'a> {
    type Item = LineInfo<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // read next line and halt if empty
        self.go_to_next_line();
        while !self.is_token_or_doc_comment() {
            self.go_to_next_line();
            self.current_line?;
        }

        // fetch a docs block, returning if not followed by a token
        let docs = self.take_docs_block();
        if !docs.is_empty() && !self.is_token() {
            let line = if self.current_line.is_none() {
                self.current_line_num
            } else {
                self.current_line_num.saturating_sub(1)
            };
            let char_offset = 0;
            return Some(LineInfo::new(line, char_offset).with_docs(docs));
        }

        // read lines until line with tokens is found; halt if empty
        while !self.is_token() {
            self.go_to_next_line();
            self.current_line?;
        }

        // fetch current line
        match self.current_line {
            Some(line) => Some(
                LineInfo::new(self.current_line_num, self.line_char_offset)
                    .with_contents(line)
                    .with_docs(docs),
            ),
            None => {
                debug_assert!(false, "this is unreachable; these is a bug in `Self::is_token`");
                None
            }
        }
    }
}

// LINE INFO
// ================================================================================================

/// A processed line with source location.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LineInfo<'a> {
    contents: Option<&'a str>,
    docs: Vec<&'a str>,
    line_number: u32,
    char_offset: u32,
}

impl From<LineInfo<'_>> for SourceLocation {
    fn from(info: LineInfo<'_>) -> Self {
        let line = info.line_number();
        let column = info.char_offset();
        Self::new(line, column)
    }
}

impl<'a> LineInfo<'a> {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Create a new instance of [LineInfo] with the provided line number and first non-blank char
    /// offset.
    pub fn new(line_number: u32, char_offset: u32) -> Self {
        Self {
            contents: None,
            docs: Vec::new(),
            line_number,
            char_offset,
        }
    }

    /// Replaces the doc comments with the provided argument.
    pub fn with_docs<I>(mut self, docs: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.docs = docs.into_iter().collect();
        self
    }

    /// Replaces the line comments with the provided argument.
    pub fn with_contents(mut self, contents: &'a str) -> Self {
        self.contents.replace(contents.trim_end());
        self
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the line contents, if present.
    ///
    /// # Examples
    ///
    /// ```masm
    /// #! some doc comment
    /// # some line comment
    ///
    /// add mul
    /// ```
    ///
    /// `add mul` is returned.
    pub const fn contents(&self) -> Option<&'a str> {
        self.contents
    }

    /// Returns the doc comments.
    ///
    /// # Examples
    ///
    /// ```masm
    /// #!      doc comments
    /// #! for foo procedure          
    /// #! with examples
    /// export.foo
    ///   add
    /// end
    /// ```
    ///
    /// `["doc comments", "for foo procedure", "with examples"]` is returned.
    pub fn docs(&self) -> &[&'a str] {
        &self.docs
    }

    /// Returns the line number, starting at `1`.
    ///
    /// # Examples
    ///
    /// ```masm
    /// #! doc comments
    /// export.foo
    ///   add
    /// end
    /// ```
    ///
    /// `add` will return `3`.
    pub const fn line_number(&self) -> u32 {
        self.line_number
    }

    /// Returns the first non-whitespace character offset.
    ///
    /// # Examples
    ///
    /// ```masm
    ///   add mul
    /// ```
    ///
    /// `2` is returned.
    pub const fn char_offset(&self) -> u32 {
        self.char_offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // UNIT TESTS
    // ============================================================================================

    #[test]
    fn token_lines_single_token() {
        let source = r#"
        begin
        "#;
        let mut lines = LinesStream::from(source);
        assert_eq!(t(2, 8, "begin"), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn token_lines_single_comment() {
        let source = r#"
        # foo
        "#;
        let mut lines = LinesStream::from(source);
        assert_eq!(None, lines.next());
    }

    #[test]
    fn token_lines_single_doc_comment() {
        let source = r#"
        #! foo
        "#;
        let mut lines = LinesStream::from(source);
        assert_eq!(tdangling(2, 0, ["foo"]), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn token_lines_inline_tokens() {
        let source = "begin add mul end";
        let mut lines = LinesStream::from(source);
        assert_eq!(t(1, 0, "begin add mul end"), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn token_lines_multiline_tokens() {
        let source = r#"begin add # foo
            mul end"#;
        let mut lines = LinesStream::from(source);
        assert_eq!(t(1, 0, "begin add # foo"), lines.next());
        assert_eq!(t(2, 12, "mul end"), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn token_lines_multiline_doc_comments_and_tokens() {
        let source = r#"begin add #! foo
            mul end"#;
        let mut lines = LinesStream::from(source);
        assert_eq!(t(1, 0, "begin add #! foo"), lines.next());
        assert_eq!(t(2, 12, "mul end"), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn token_lines_simple_mod_comment() {
        let source = r#"#! some mod comment
    begin add # foo

            # bar

            mul
      end


            # baz"#;
        let mut lines = LinesStream::from(source);
        assert_eq!(tdocs(2, 4, "begin add # foo", ["some mod comment"]), lines.next());
        assert_eq!(t(6, 12, "mul"), lines.next());
        assert_eq!(t(7, 6, "end"), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn token_lines_dangling_comment() {
        let source = r#"#! some mod comment
    begin add # foo

            # bar

            mul
      end


            #! dangling comment"#;
        let mut lines = LinesStream::from(source);
        assert_eq!(tdocs(2, 4, "begin add # foo", ["some mod comment"]), lines.next());
        assert_eq!(t(6, 12, "mul"), lines.next());
        assert_eq!(t(7, 6, "end"), lines.next());
        assert_eq!(tdangling(10, 0, ["dangling comment"]), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn token_lines_inline_doc_comment() {
        let source = r#"#! some mod comment
    begin add # foo

            #! bar

            mul
    #!                    end doc comment with trailing spaces    
        #! and multiple lines
      end


            #! baz"#;
        let mut lines = LinesStream::from(source);
        assert_eq!(tdocs(2, 4, "begin add # foo", ["some mod comment"]), lines.next());
        assert_eq!(tdangling(4, 0, ["bar"]), lines.next());
        assert_eq!(t(6, 12, "mul"), lines.next());
        assert_eq!(
            tdocs(9, 6, "end", ["end doc comment with trailing spaces", "and multiple lines"]),
            lines.next()
        );
        assert_eq!(tdangling(12, 0, ["baz"]), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn token_lines_inline_multiline_doc_comment() {
        let source = r#"#! some mod comment
            #!
        #!      additional docs
    begin add # foo

            #! bar

            mul
    #!                    end doc comment with trailing spaces
            #! more lines....
      end


            #! some dangling doc comment"#;
        let mut lines = LinesStream::from(source);
        assert_eq!(
            tdocs(4, 4, "begin add # foo", ["some mod comment", "additional docs",]),
            lines.next()
        );
        assert_eq!(tdangling(6, 0, ["bar"]), lines.next());
        assert_eq!(t(8, 12, "mul"), lines.next());
        assert_eq!(
            tdocs(11, 6, "end", ["end doc comment with trailing spaces", "more lines....",]),
            lines.next()
        );
        assert_eq!(tdangling(14, 0, ["some dangling doc comment"]), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn token_lines_simple_proc() {
        let source = r#"#! some proc comment
        #!      additional docs
    proc.foo # foo
        add mul.5
    end"#;
        let mut lines = LinesStream::from(source);
        assert_eq!(
            tdocs(3, 4, "proc.foo # foo", ["some proc comment", "additional docs",]),
            lines.next()
        );
        assert_eq!(t(4, 8, "add mul.5"), lines.next());
        assert_eq!(t(5, 4, "end"), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn token_lines_multiproc_module() {
        let source = r#"
#! Some multiline mod
#! docs
#! with more than two lines

#! Some multiline proc
#! docs
#! additional comments
export.foo.1
    loc_load.0
end

#! More multiline proc
#! docs
proc.bar.2
    padw
end

#! final dangling comment     "#;
        let mut lines = LinesStream::from(source);
        assert_eq!(
            tdangling(4, 0, ["Some multiline mod", "docs", "with more than two lines",]),
            lines.next()
        );
        assert_eq!(
            tdocs(9, 0, "export.foo.1", ["Some multiline proc", "docs", "additional comments",]),
            lines.next()
        );
        assert_eq!(t(10, 4, "loc_load.0"), lines.next());
        assert_eq!(t(11, 0, "end"), lines.next());
        assert_eq!(tdocs(15, 0, "proc.bar.2", ["More multiline proc", "docs",]), lines.next());
        assert_eq!(t(16, 4, "padw"), lines.next());
        assert_eq!(t(17, 0, "end"), lines.next());
        assert_eq!(tdangling(19, 0, ["final dangling comment"]), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn token_lines_trailing_spaces() {
        let source = r#"
export.verify
    #=> [main_trace_commitment]
    exec.random_coin::reseed
end
"#;
        let mut lines = LinesStream::from(source);
        assert_eq!(t(2, 0, "export.verify"), lines.next());
        assert_eq!(t(4, 4, "exec.random_coin::reseed"), lines.next());
        assert_eq!(t(5, 0, "end"), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn token_lines_docs_multiple_filter_empty_lines() {
        let source = r#"
#! Foo
#! Bar
#!
#!
#!
#! Baz
#!
export.verify
    push.0
end
"#;
        let mut lines = LinesStream::from(source);
        assert_eq!(tdocs(9, 0, "export.verify", ["Foo", "Bar", "Baz"]), lines.next());
        assert_eq!(t(10, 4, "push.0"), lines.next());
        assert_eq!(t(11, 0, "end"), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn token_lines_docs_with_line_comment_header() {
        let source = r#"# ========== SOME HEADER =========

#! Foo
export.verify
    push.0
end
"#;
        let mut lines = LinesStream::from(source);
        assert_eq!(tdocs(4, 0, "export.verify", ["Foo"]), lines.next());
        assert_eq!(t(5, 4, "push.0"), lines.next());
        assert_eq!(t(6, 0, "end"), lines.next());
        assert_eq!(None, lines.next());
    }

    // TESTS HELPERS
    // ============================================================================================

    fn t(num: u32, offset: u32, contents: &str) -> Option<LineInfo> {
        Some(LineInfo::new(num, offset).with_contents(contents))
    }

    fn tdocs<'a, I>(num: u32, offset: u32, contents: &'a str, docs: I) -> Option<LineInfo<'a>>
    where
        I: IntoIterator<Item = &'a str>,
    {
        Some(LineInfo::new(num, offset).with_contents(contents).with_docs(docs))
    }

    fn tdangling<'a, I>(num: u32, offset: u32, docs: I) -> Option<LineInfo<'a>>
    where
        I: IntoIterator<Item = &'a str>,
    {
        Some(LineInfo::new(num, offset).with_docs(docs))
    }
}
