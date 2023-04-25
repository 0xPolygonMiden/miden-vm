use super::{LineInfo, SourceLocation, Token};

// LINE TOKENIZER
// ================================================================================================

/// A line tokenizer that will generate tokens with their locations from [LinesStream] and
/// [LineInfo].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LineTokenizer<'a> {
    line: &'a str,
    location: SourceLocation,
    dangling: Option<SourceLocation>,
}

impl<'a> LineTokenizer<'a> {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Creates a new [LineTokenizer] from the contents of a [LineInfo], if present.
    pub fn new(line_info: &LineInfo<'a>) -> Option<Self> {
        let line_number = line_info.line_number();
        let column = line_info.char_offset() + 1;
        let location = SourceLocation::new(line_number, column);
        let line = line_info.contents()?;
        Some(Self {
            line,
            location,
            dangling: None,
        })
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Takes dangling docs location, if present.
    pub fn take_dangling(&mut self) -> Option<SourceLocation> {
        self.dangling.take()
    }
}

impl<'a> Iterator for LineTokenizer<'a> {
    type Item = (&'a str, SourceLocation);

    fn next(&mut self) -> Option<Self::Item> {
        if self.line.is_empty() {
            return None;
        }

        if self.line.starts_with(Token::DOC_COMMENT_PREFIX) {
            let mut location = self.location;
            location.move_column(1);
            self.dangling.replace(location);
            return None;
        }

        if self.line.starts_with(Token::COMMENT_PREFIX) {
            return None;
        }

        let token_loc = self.location;
        let (token, offset) = match self.line.split_once(char::is_whitespace) {
            Some((token, remainder)) => {
                let offset = remainder.find(|c: char| !c.is_whitespace()).unwrap_or_default();
                (token, token.len() + offset + 1)
            }
            None => (self.line, self.line.len()),
        };

        let (_, remainder) = self.line.split_at(offset);
        self.line = remainder;
        self.location.move_column(offset as u32);

        Some((token, token_loc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // UNIT TESTS
    // ============================================================================================

    #[test]
    fn empty_line() {
        let info = LineInfo::new(1, 0).with_contents("");
        let mut tokenizer = LineTokenizer::new(&info).unwrap();
        assert_eq!(None, tokenizer.next());
        assert!(tokenizer.take_dangling().is_none());
    }

    #[test]
    fn blank_line() {
        let info = LineInfo::new(1, 0).with_contents("     \t");
        let mut tokenizer = LineTokenizer::new(&info).unwrap();
        assert_eq!(None, tokenizer.next());
        assert!(tokenizer.take_dangling().is_none());
    }

    #[test]
    fn comment_line() {
        let info = LineInfo::new(1, 0).with_contents("# foo");
        let mut tokenizer = LineTokenizer::new(&info).unwrap();
        assert_eq!(None, tokenizer.next());
        assert!(tokenizer.take_dangling().is_none());
    }

    #[test]
    fn single_token() {
        let info = LineInfo::new(1, 0).with_contents("begin");
        let mut tokenizer = LineTokenizer::new(&info).unwrap();
        assert_eq!(l("begin", 1, 1), tokenizer.next());
        assert_eq!(None, tokenizer.next());
        assert!(tokenizer.take_dangling().is_none());
    }

    #[test]
    fn single_line_multiple_token() {
        let info = LineInfo::new(1, 0).with_contents("begin  add mul  \t end ");
        let mut tokenizer = LineTokenizer::new(&info).unwrap();
        assert_eq!(l("begin", 1, 1), tokenizer.next());
        assert_eq!(l("add", 1, 8), tokenizer.next());
        assert_eq!(l("mul", 1, 12), tokenizer.next());
        assert_eq!(l("end", 1, 19), tokenizer.next());
        assert_eq!(None, tokenizer.next());
        assert!(tokenizer.take_dangling().is_none());
    }

    #[test]
    fn single_line_multiple_token_with_comment() {
        let info = LineInfo::new(10, 15).with_contents("begin  add mul  \t end # foo");
        let mut tokenizer = LineTokenizer::new(&info).unwrap();
        assert_eq!(l("begin", 10, 16), tokenizer.next());
        assert_eq!(l("add", 10, 23), tokenizer.next());
        assert_eq!(l("mul", 10, 27), tokenizer.next());
        assert_eq!(l("end", 10, 34), tokenizer.next());
        assert_eq!(None, tokenizer.next());
        assert!(tokenizer.take_dangling().is_none());
    }

    #[test]
    fn single_line_multiple_token_with_dangling_comment() {
        let info = LineInfo::new(1, 0).with_contents("begin  add mul  \t end #! foo");
        let mut tokenizer = LineTokenizer::new(&info).unwrap();
        assert_eq!(l("begin", 1, 1), tokenizer.next());
        assert_eq!(l("add", 1, 8), tokenizer.next());
        assert_eq!(l("mul", 1, 12), tokenizer.next());
        assert_eq!(l("end", 1, 19), tokenizer.next());
        assert_eq!(None, tokenizer.next());
        assert_eq!(Some(SourceLocation::new(1, 24)), tokenizer.take_dangling());
    }

    // TESTS HELPERS
    // ============================================================================================

    fn l(token: &str, line: u32, col: u32) -> Option<(&str, SourceLocation)> {
        Some((token, SourceLocation::new(line, col)))
    }
}
