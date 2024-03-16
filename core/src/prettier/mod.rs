mod document;
mod print;

pub use self::document::{concat, const_text, display, flatten, indent, nl, text, Document};

use alloc::string::String;
use core::fmt;

/// The [PrettyPrint] trait is used as a building block for printing code using an algorithm
/// derived from the one described by Philip Wadler in
/// [_A prettier printer_](https://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf).
///
/// By implementing [PrettyPrint::render], you can trivially implement [core::fmt::Display]
/// for your pretty-printable types using the output of the pretty printer. It is also possible
/// to implement `Display` and `PrettyPrint` with different representations, but obtain the
/// pretty-printed representation using [PrettyPrint::to_pretty_string] or by calling
/// [PrettyPrint::pretty_print] from within a context where you have a [core::fmt::Formatter].
/// You can manufacture one using the [Prettier] wrapper type, which will render the contained
/// object using its [PrettyPrint] implementation.
///
/// To implement [PrettyPrint], you need to make use of the different [Document] constructors
/// exported from this module, e.g. [text] or [indent]. See the function documentation for each
/// off those constructors to see how they can be used to acheive the output you desire.
pub trait PrettyPrint {
    /// The core of the [PrettyPrint] functionality.
    ///
    /// When called, the implementation must render a [Document] which represents the layout
    /// of the thing being pretty-printed. The rendered [Document] is then displayed via the
    /// pretty printer, using details about the output stream, and the structure of the document,
    /// to make decisions about when and where to introduce line breaks, etc.
    ///
    /// Implementations do not need to worry about or manage things like the width of the output,
    /// indentation level, etc. Instead the focus is purely on the layout, leaving the heavy
    /// lifting to the pretty printer.
    ///
    /// This method is the only one required to be implemented.
    fn render(&self) -> Document;

    /// Produce a [String] containing the results of pretty-printing this object.
    ///
    /// The string is formatted with an assumed width of 80 columns. If you wish to customize this,
    /// you should instead prefer to use [PrettyPrint::pretty_print], or if you have implemented
    /// [core::fmt::Display] for this type by delegating to [PrettyPrint::pretty_print], you can
    /// use the Rust formatting syntax to do this, e.g. `format!("{:width$}", self, width = 100)`
    fn to_pretty_string(&self) -> String {
        format!("{:width$}", Prettier(self), width = 80)
    }

    /// Pretty-print this object to the given [core::fmt::Formatter].
    ///
    /// You may implement [core::fmt::Display] for your type in terms of this function like so:
    ///
    /// ```rust,ignore
    /// impl fmt::Display for Foo {
    ///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    ///         self.pretty_print(f)
    ///     }
    /// }
    /// ```
    fn pretty_print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let doc = self.render();
        let width = f.width().unwrap_or(80);
        print::pretty_print(&doc, width, f)
    }
}

impl fmt::Display for dyn PrettyPrint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        PrettyPrint::pretty_print(self, f)
    }
}

macro_rules! pretty_via_display {
    ($name:ty) => {
        impl PrettyPrint for $name {
            fn render(&self) -> Document {
                display(*self)
            }
        }
    };
}

macro_rules! pretty_via_string {
    ($name:ty) => {
        impl PrettyPrint for $name {
            fn render(&self) -> Document {
                text(&**self)
            }
        }
    };
}

pretty_via_display!(bool);
pretty_via_display!(u8);
pretty_via_display!(i8);
pretty_via_display!(u16);
pretty_via_display!(i16);
pretty_via_display!(u32);
pretty_via_display!(i32);
pretty_via_display!(u64);
pretty_via_display!(i64);
pretty_via_display!(crate::Felt);
pretty_via_string!(alloc::rc::Rc<str>);
pretty_via_string!(alloc::sync::Arc<str>);

impl PrettyPrint for crate::crypto::hash::RpoDigest {
    fn render(&self) -> Document {
        use crate::utils::DisplayHex;

        DisplayHex(self.as_bytes().as_slice()).render()
    }
}

impl<'a, T: ?Sized + PrettyPrint> PrettyPrint for &'a T {
    #[inline]
    fn render(&self) -> Document {
        (**self).render()
    }
    #[inline]
    fn to_pretty_string(&self) -> String {
        (**self).to_pretty_string()
    }
    #[inline]
    fn pretty_print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (**self).pretty_print(f)
    }
}

impl PrettyPrint for str {
    fn render(&self) -> Document {
        self.lines()
            .map(text)
            .reduce(|acc, doc| match acc {
                Document::Empty => doc + nl(),
                other => other + doc + nl(),
            })
            .unwrap_or(Document::Empty)
    }
}

impl PrettyPrint for String {
    fn render(&self) -> Document {
        PrettyPrint::render(self.as_str())
    }
    fn pretty_print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        PrettyPrint::pretty_print(self.as_str(), f)
    }
}

impl<'a> PrettyPrint for alloc::borrow::Cow<'a, str> {
    fn render(&self) -> Document {
        PrettyPrint::render(self.as_ref())
    }
    fn pretty_print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        PrettyPrint::pretty_print(self.as_ref(), f)
    }
}

struct Prettier<'a, P: ?Sized + PrettyPrint>(&'a P);

impl<'a, P: ?Sized + PrettyPrint> fmt::Display for Prettier<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.pretty_print(f)
    }
}
