use super::{LineInfo, SourceLocation, Token};
use core::mem;

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

impl<'a> From<&LineInfo<'a>> for LineTokenizer<'a> {
    fn from(info: &LineInfo<'a>) -> Self {
        let line_number = info.line_number();
        let column = info.char_offset() + 1;
        let location = SourceLocation::new(line_number, column);
        let line = info.contents().unwrap_or("");
        Self {
            line,
            location,
            dangling: None,
        }
    }
}

impl<'a> LineTokenizer<'a> {
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

        // [LinesStream] generates [LineInfo] without leading whitespaces
        let (token, remainder) = match self.line.split_once(char::is_whitespace) {
            Some(split) => split,
            None => {
                let line = mem::take(&mut self.line);
                let location = mem::take(&mut self.location);
                return Some((line, location));
            }
        };

        // set location & find next non-whitespace
        let location = self.location;
        let at = match remainder.find(|c: char| !c.is_whitespace()) {
            Some(at) => at,
            None => {
                mem::take(&mut self.line);
                mem::take(&mut self.location);
                return Some((token, location));
            }
        };

        // update location, line & return
        self.location.move_column((token.len() + at + 1) as u32);
        self.line = remainder.split_at(at).1;
        Some((token, location))
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
        let mut tokenizer = LineTokenizer::from(&info);
        assert_eq!(None, tokenizer.next());
        assert!(tokenizer.take_dangling().is_none());
    }

    #[test]
    fn blank_line() {
        let info = LineInfo::new(1, 0).with_contents("     \t");
        let mut tokenizer = LineTokenizer::from(&info);
        assert_eq!(None, tokenizer.next());
        assert!(tokenizer.take_dangling().is_none());
    }

    #[test]
    fn comment_line() {
        let info = LineInfo::new(1, 0).with_contents("# foo");
        let mut tokenizer = LineTokenizer::from(&info);
        assert_eq!(None, tokenizer.next());
        assert!(tokenizer.take_dangling().is_none());
    }

    #[test]
    fn single_token() {
        let info = LineInfo::new(1, 0).with_contents("begin");
        let mut tokenizer = LineTokenizer::from(&info);
        assert_eq!(l("begin", 1, 1), tokenizer.next());
        assert_eq!(None, tokenizer.next());
        assert!(tokenizer.take_dangling().is_none());
    }

    #[test]
    fn single_line_multiple_token() {
        let info = LineInfo::new(1, 0).with_contents("begin  add mul  \t end ");
        let mut tokenizer = LineTokenizer::from(&info);
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
        let mut tokenizer = LineTokenizer::from(&info);
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
        let mut tokenizer = LineTokenizer::from(&info);
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
