use super::{LineInfo, LinesStream, SourceLocation, Token};
use core::mem;

// LINE TOKENIZER
// ================================================================================================

#[derive(Debug, Clone)]
pub struct LineTokenizer<'a> {
    // current token variables
    docs: Option<String>,
    line: Option<&'a str>,
    location: SourceLocation,

    // internal variables
    is_first: bool,
    dangling_docs: Option<SourceLocation>,
    lines: LinesStream<'a>,
    module_docs: Option<String>,
}

impl<'a> From<&'a str> for LineTokenizer<'a> {
    fn from(source: &'a str) -> Self {
        LinesStream::from(source).into()
    }
}

impl<'a> From<LinesStream<'a>> for LineTokenizer<'a> {
    fn from(lines: LinesStream<'a>) -> Self {
        Self {
            docs: None,
            line: None,
            location: SourceLocation::default(),
            is_first: true,
            dangling_docs: None,
            lines,
            module_docs: None,
        }
    }
}

impl<'a> LineTokenizer<'a> {
    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Takes dangling docs location, if present.
    pub fn take_dangling_docs(&mut self) -> Option<SourceLocation> {
        self.dangling_docs.take()
    }

    /// Takes the module docs, if present.
    pub fn take_module_docs(&mut self) -> Option<String> {
        self.module_docs.take()
    }

    // HELPERS
    // --------------------------------------------------------------------------------------------

    /// Fetches a new line with a token on its first position.
    fn take_new_line(&mut self) -> Option<&'a str> {
        let LineInfo {
            contents,
            docs,
            line_number,
            char_offset,
        } = self.lines.next()?;

        // set source location according to the provided offset
        let is_first = mem::take(&mut self.is_first);
        self.location = SourceLocation::new(line_number, char_offset + 1);

        // if line contains doc comments, then it is dangling
        if contents.and_then(|l| l.find(Token::DOC_COMMENT_PREFIX)).is_some() {
            self.dangling_docs.replace(self.location);
            return None;
        }

        // remove trailing comments & whitespaces
        let line = contents
            .map(|line| {
                line.split_once(Token::COMMENT_PREFIX)
                    .map(|(l, _comments)| l.trim_end())
                    .unwrap_or(line)
            })
            .and_then(|line| {
                line.find(|c: char| !c.is_whitespace()).map(|n| {
                    self.location.move_column(n as u32);
                    line.split_at(n).1
                })
            })
            .filter(|line| !line.is_empty());

        // if first line is empty & has docs, set the module docs
        if line.is_none() && is_first && !docs.is_empty() {
            self.module_docs = build_comment(&docs);
            return self.take_new_line();
        }

        // if line is empty, then dangling docs as first line is already checked
        if line.is_none() {
            self.dangling_docs.replace(self.location);
            return None;
        }

        // set token docs & return
        self.docs = build_comment(&docs);
        line
    }
}

impl<'a> Iterator for LineTokenizer<'a> {
    type Item = LineToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.line.take().or_else(|| self.take_new_line())?;
        let location = self.location;
        let docs = self.docs.take();

        // fetch token & move offset
        let (token, remainder) = line.split_once(char::is_whitespace).unwrap_or((line, ""));
        self.location.move_column(token.len() as u32);

        // update remainder line
        self.line = remainder
            .find(|c: char| !c.is_whitespace())
            .map(|n| {
                self.location.move_column(n as u32 + 1);
                remainder.split_at(n).1
            })
            .filter(|line| !line.is_empty());

        Some(LineToken {
            docs,
            location,
            token,
        })
    }
}

// LINE TOKEN
// ================================================================================================

/// A processed line with source location.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LineToken<'a> {
    pub docs: Option<String>,
    pub location: SourceLocation,
    pub token: &'a str,
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
