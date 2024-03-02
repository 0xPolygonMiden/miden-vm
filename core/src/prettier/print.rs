use alloc::vec::Vec;
use core::fmt::{self, Write};

use super::Document;

pub fn pretty_print(doc: &Document, width: usize, f: &mut fmt::Formatter) -> fmt::Result {
    let mut printer = PrettyPrinter::new(doc, width);
    printer.print(f)
}

struct PrettyPrinter<'a> {
    width: usize,
    col: u32,
    chunks: Vec<Chunk<'a>>,
}

#[derive(Debug, Clone, Copy)]
struct Chunk<'a> {
    doc: &'a Document,
    indent: u32,
    flat: bool,
}

impl<'a> Chunk<'a> {
    fn with_doc(self, doc: &'a Document) -> Self {
        Self {
            doc,
            indent: self.indent,
            flat: self.flat,
        }
    }

    fn indented(self, indent: u32, doc: &'a Document) -> Self {
        Self {
            doc,
            indent: self.indent + indent,
            flat: self.flat,
        }
    }

    fn flat(self, doc: &'a Document) -> Self {
        Self {
            doc,
            indent: self.indent,
            flat: true,
        }
    }
}

impl<'a> PrettyPrinter<'a> {
    fn new(doc: &'a Document, width: usize) -> Self {
        let chunk = Chunk {
            doc,
            indent: 0,
            flat: false,
        };
        Self {
            width,
            col: 0,
            chunks: vec![chunk],
        }
    }

    fn print(&mut self, f: &mut fmt::Formatter) -> fmt::Result {
        while let Some(chunk) = self.chunks.pop() {
            match chunk.doc {
                Document::Empty => (),
                Document::Newline => {
                    f.write_char('\n')?;
                    write!(f, "{1:0$}", chunk.indent as usize, "")?;
                    self.col = chunk.indent;
                }
                Document::Text(text, width) => {
                    f.write_str(text)?;
                    self.col += width;
                }
                Document::Flatten(x) => self.chunks.push(chunk.flat(x)),
                Document::Indent(i, x) => self.chunks.push(chunk.indented(*i, x)),
                Document::Concat(x, y) => {
                    self.chunks.push(chunk.with_doc(y));
                    self.chunks.push(chunk.with_doc(x));
                }
                Document::Choice(x, y) => {
                    if chunk.flat || self.fits(chunk.with_doc(x)) {
                        self.chunks.push(chunk.with_doc(x));
                    } else {
                        self.chunks.push(chunk.with_doc(y));
                    }
                }
            }
        }
        Ok(())
    }

    fn fits(&self, chunk: Chunk<'a>) -> bool {
        let mut remaining = self.width.saturating_sub(self.col as usize);
        let mut stack = vec![chunk];
        let mut chunks = self.chunks.as_slice();

        loop {
            let chunk = match stack.pop() {
                Some(chunk) => chunk,
                None => match chunks.split_last() {
                    None => return true,
                    Some((chunk, more_chunks)) => {
                        chunks = more_chunks;
                        *chunk
                    }
                },
            };

            match &chunk.doc {
                Document::Empty | Document::Newline => return true,
                Document::Text(_text, text_width) => {
                    if *text_width as usize <= remaining {
                        remaining -= *text_width as usize;
                    } else {
                        return false;
                    }
                }
                Document::Flatten(x) => stack.push(chunk.flat(x)),
                Document::Indent(i, x) => stack.push(chunk.indented(*i, x)),
                Document::Concat(x, y) => {
                    stack.push(chunk.with_doc(y));
                    stack.push(chunk.with_doc(x));
                }
                Document::Choice(x, y) => {
                    if chunk.flat {
                        stack.push(chunk.with_doc(x));
                    } else {
                        // Relies on the rule that for every choice `x | y`,
                        // the first line of `y` is no longer than the first line of `x`.
                        stack.push(chunk.with_doc(y));
                    }
                }
            }
        }
    }
}
