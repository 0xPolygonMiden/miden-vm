use alloc::{borrow::Cow, rc::Rc, string::ToString};
use core::fmt;

#[derive(Debug, Default, Clone)]
pub enum Document {
    /// An empty document, rendered as an empty string
    #[default]
    Empty,
    /// A line break, rendered as a single '\n' char
    Newline,
    /// A literal text string of width `n`
    Text(Cow<'static, str>, u32),
    /// A combinator which chooses the leftmost of each
    /// choice in the given document
    Flatten(Rc<Document>),
    /// Increase the indentation of the given document by `n`
    Indent(u32, Rc<Document>),
    /// Concatenate two documents
    Concat(Rc<Document>, Rc<Document>),
    /// Choose the more optimal of two documents depending on
    /// the amount of space remaining in the layout
    Choice(Rc<Document>, Rc<Document>),
}

/// Render a line break (i.e. newline) in the output
pub fn nl() -> Document {
    Document::Newline
}

/// Display the given value.
///
/// This function expects that the display format does
/// not contain any newlines. Violating this expectation
/// may produce incorrect output.
pub fn display(s: impl ToString) -> Document {
    let string = Cow::<'static, str>::Owned(s.to_string());
    text(string)
}

/// Display the given string exactly.
///
/// Like [display], this function expects the string
/// does not contain any newlines. Violating this expectation
/// may produce incorrect output.
pub fn text(s: impl ToString) -> Document {
    let string = Cow::<'static, str>::Owned(s.to_string());
    let width = unicode_width::UnicodeWidthStr::width(string.as_ref()) as u32;
    Document::Text(string, width)
}

/// Same as [text], but for static/constant strings
pub fn const_text(s: &'static str) -> Document {
    let string = Cow::Borrowed(s);
    let width = unicode_width::UnicodeWidthStr::width(string.as_ref()) as u32;
    Document::Text(string, width)
}

/// Create a document by splitting `input` on line breaks
/// so ensure the invariants of [text] are upheld.
#[allow(unused)]
pub fn split<S: AsRef<str>>(input: S) -> Document {
    let input = input.as_ref();
    input
        .lines()
        .map(text)
        .reduce(|acc, doc| match acc {
            Document::Empty => doc + nl(),
            other => other + doc + nl(),
        })
        .unwrap_or(Document::Empty)
}

/// Concatenate two documents, producing a single document representing both.
#[inline(always)]
pub fn concat(left: Document, right: Document) -> Document {
    left + right
}

/// Use the leftmost option of every choice in the given document.
///
/// If the given document upholds the expectation that none of the
/// leftmost choices contain newlines, then this combinator has the
/// effect of displaying all choices on one line.
pub fn flatten(doc: Document) -> Document {
    Document::Flatten(Rc::new(doc))
}

/// Increase the indentation level of the given document by `width`.
///
/// The indentation level determines the number of spaces put after newlines.
///
/// NOTE: Indentation is applied following newlines, therefore, the first
/// line of a document is _not_ indented.
pub fn indent(indent: u32, doc: Document) -> Document {
    Document::Indent(indent, Rc::new(doc))
}

impl core::ops::Add for Document {
    type Output = Document;

    /// Concatenate the two documents
    fn add(self: Document, other: Document) -> Self::Output {
        Document::Concat(Rc::new(self), Rc::new(other))
    }
}

impl core::ops::AddAssign for Document {
    /// Append `rhs` to `self`
    fn add_assign(&mut self, rhs: Document) {
        let lhs = core::mem::take(self);
        if matches!(lhs, Document::Empty) {
            *self = rhs;
            return;
        }
        *self = Document::Concat(Rc::new(lhs), Rc::new(rhs));
    }
}

impl core::ops::BitOr for Document {
    type Output = Document;

    /// If inside a `flat`, _or_ the first line of the left document fits within
    /// the required width, then display the left document. Otherwise, display
    /// the right document.
    fn bitor(self: Document, other: Document) -> Self::Output {
        Document::Choice(Rc::new(self), Rc::new(other))
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = f.width().unwrap_or(80);
        super::print::pretty_print(self, width, f)
    }
}
