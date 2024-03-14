use core::{
    borrow::Borrow,
    fmt,
    hash::{Hash, Hasher},
    ops::{Bound, Deref, DerefMut, Index, Range, RangeBounds},
};

use crate::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

/// This trait should be implemented for any type that has
/// an associated [SourceSpan].
pub trait Spanned {
    fn span(&self) -> SourceSpan;
}

impl<T: Spanned> Spanned for &T {
    fn span(&self) -> SourceSpan {
        (**self).span()
    }
}

/// This type is used to wrap any `T` with a [SourceSpan],
/// and is typically used when it is not convenient to add
/// a [SourceSpan] to the type - most commonly because we
/// don't control the type.
pub struct Span<T> {
    span: SourceSpan,
    spanned: T,
}

impl<T> Spanned for Span<T> {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

impl<T: Copy> Copy for Span<T> {}

impl<T: Clone> Clone for Span<T> {
    fn clone(&self) -> Self {
        Self {
            span: self.span,
            spanned: self.spanned.clone(),
        }
    }
}

impl<T> Span<T> {
    /// Create a span for `spanned` with `span`
    #[inline]
    pub fn new(span: impl Into<SourceSpan>, spanned: T) -> Self {
        Self {
            span: span.into(),
            spanned,
        }
    }

    /// Create a span for `spanned` representing a single location, `offset`
    #[inline]
    pub fn at(offset: usize, spanned: T) -> Self {
        Self {
            span: SourceSpan::at(offset.try_into().expect("invalid source offset: too large")),
            spanned,
        }
    }

    /// Create a [Span] from a value with an unknown/default location
    pub fn unknown(spanned: T) -> Self {
        Self {
            span: Default::default(),
            spanned,
        }
    }

    /// Get the associated [SourceSpan] for this spanned item.
    #[inline(always)]
    pub const fn span(&self) -> SourceSpan {
        self.span
    }

    /// Apply a transformation to the spanned value while retaining the same [SourceSpan]
    #[inline]
    pub fn map<U, F>(self, mut f: F) -> Span<U>
    where
        F: FnMut(T) -> U,
    {
        Span {
            span: self.span,
            spanned: f(self.spanned),
        }
    }

    /// Like [`Option<T>::as_deref`], this constructs a [`Span<U>`] wrapping the result
    /// of dereferencing the inner value of type `T` as a value of type `U`.
    pub fn as_deref<U>(&self) -> Span<&U>
    where
        U: ?Sized,
        T: Deref<Target = U>,
    {
        Span {
            span: self.span,
            spanned: self.spanned.deref(),
        }
    }

    /// Get a new [Span] that borrows the inner value.
    pub fn as_ref(&self) -> Span<&T> {
        Span {
            span: self.span,
            spanned: &self.spanned,
        }
    }

    /// Shift the span right by `count` units
    #[inline]
    pub fn shift(&mut self, count: usize) {
        let count: u32 = count.try_into().expect("invalid count: must be smaller than 2^32");
        self.span.start += count;
        self.span.end += count;
    }

    /// Extend the end of the span by `count` units
    #[inline]
    pub fn extend(&mut self, count: usize) {
        let count: u32 = count.try_into().expect("invalid count: must be smaller than 2^32");
        self.span.end += count;
    }

    /// Consume this span, returning the component parts, i.e. the [SourceSpan] and value of type
    /// `T`.
    #[inline]
    pub fn into_parts(self) -> (SourceSpan, T) {
        (self.span, self.spanned)
    }

    /// Unwrap the spanned value of type `T`
    #[inline]
    pub fn into_inner(self) -> T {
        self.spanned
    }
}

impl<T: Borrow<str>, S: Borrow<T>> Borrow<T> for Span<S> {
    fn borrow(&self) -> &T {
        self.spanned.borrow()
    }
}

impl<T> Deref for Span<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.spanned
    }
}

impl<T> DerefMut for Span<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.spanned
    }
}

impl<T: ?Sized, U: AsRef<T>> AsRef<T> for Span<U> {
    fn as_ref(&self) -> &T {
        self.spanned.as_ref()
    }
}

impl<T: ?Sized, U: AsMut<T>> AsMut<T> for Span<U> {
    fn as_mut(&mut self) -> &mut T {
        self.spanned.as_mut()
    }
}

impl<T: fmt::Debug> fmt::Debug for Span<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.spanned, f)
    }
}

impl<T: fmt::Display> fmt::Display for Span<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.spanned, f)
    }
}

#[cfg(feature = "formatter")]
impl<T: crate::prettier::PrettyPrint> crate::prettier::PrettyPrint for Span<T> {
    fn render(&self) -> crate::prettier::Document {
        self.spanned.render()
    }
}

impl<T: Eq> Eq for Span<T> {}

impl<T: PartialEq> PartialEq for Span<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.spanned.eq(&other.spanned)
    }
}

impl<T: PartialEq> PartialEq<T> for Span<T> {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        self.spanned.eq(other)
    }
}

impl<T: Ord> Ord for Span<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.spanned.cmp(&other.spanned)
    }
}

impl<T: PartialOrd> PartialOrd for Span<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.spanned.partial_cmp(&other.spanned)
    }
}

impl<T: Hash> Hash for Span<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.spanned.hash(state);
    }
}

/// Serialization
impl<T: Serializable> Span<T> {
    pub fn write_into<W: ByteWriter>(&self, target: &mut W, options: crate::ast::AstSerdeOptions) {
        if options.debug_info {
            self.span.write_into(target);
        }
        self.spanned.write_into(target);
    }
}

/// Deserialization
impl<T: Deserializable> Span<T> {
    pub fn read_from<R: ByteReader>(
        source: &mut R,
        options: crate::ast::AstSerdeOptions,
    ) -> Result<Self, DeserializationError> {
        let span = if options.debug_info {
            SourceSpan::read_from(source)?
        } else {
            SourceSpan::default()
        };
        let spanned = T::read_from(source)?;
        Ok(Self { span, spanned })
    }
}

impl<T: Serializable> Serializable for Span<T> {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.span.write_into(target);
        self.spanned.write_into(target);
    }
}

impl<T: Deserializable> Deserializable for Span<T> {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let span = SourceSpan::read_from(source)?;
        let spanned = T::read_from(source)?;
        Ok(Self { span, spanned })
    }
}

/// This represents a span of bytes in a Miden Assembly source file.
///
/// It is compact, using only 8 bytes to represent the full span. This
/// does, however, come at the tradeoff of only supporting source
/// files of up to 2^32 bytes.
///
/// This type is produced by the lexer and carried through parsing.
/// It can be converted into a line/column range as well, if needed.
///
/// This representation is more convenient to produce, and allows
/// showing source spans in error messages, whereas line/column
/// information is useful at a glance in debug output, it is harder
/// to produce nice errors with it compared to this representation.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceSpan {
    start: u32,
    end: u32,
}

impl SourceSpan {
    /// Create a new [SourceSpan] from the given range.
    pub fn new(range: Range<u32>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }

    /// Create a new [SourceSpan] for a specific offset.
    pub fn at(offset: u32) -> Self {
        Self {
            start: offset,
            end: offset,
        }
    }

    /// Get the offset in bytes corresponding to the start of this span (inclusive).
    #[inline(always)]
    pub fn start(&self) -> usize {
        self.start as usize
    }

    /// Get the offset in bytes corresponding to the end of this span (exclusive).
    #[inline(always)]
    pub fn end(&self) -> usize {
        self.end as usize
    }

    /// Get the length of this span in bytes
    #[inline(always)]
    pub fn len(&self) -> usize {
        (self.end - self.start) as usize
    }

    /// Returns true if this span is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Convert this span into a [`Range<u32>`]
    #[inline]
    pub fn into_range(self) -> Range<u32> {
        self.start..self.end
    }
}

impl Serializable for SourceSpan {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u32(self.start);
        target.write_u32(self.end)
    }
}

impl Deserializable for SourceSpan {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let start = source.read_u32()?;
        let end = source.read_u32()?;
        Ok(Self { start, end })
    }
}

impl TryFrom<Range<usize>> for SourceSpan {
    type Error = ();

    fn try_from(range: Range<usize>) -> Result<Self, Self::Error> {
        const MAX: usize = u32::MAX as usize;
        if range.start > MAX || range.end > MAX {
            return Err(());
        }
        Ok(SourceSpan {
            start: range.start as u32,
            end: range.end as u32,
        })
    }
}

impl From<Range<u32>> for SourceSpan {
    #[inline(always)]
    fn from(range: Range<u32>) -> Self {
        Self::new(range)
    }
}

impl From<SourceSpan> for Range<u32> {
    #[inline(always)]
    fn from(span: SourceSpan) -> Self {
        span.into_range()
    }
}

impl From<SourceSpan> for miette::SourceSpan {
    #[inline]
    fn from(span: SourceSpan) -> Self {
        miette::SourceSpan::new(miette::SourceOffset::from(span.start as usize), span.len())
    }
}

impl Index<SourceSpan> for [u8] {
    type Output = [u8];

    #[inline]
    fn index(&self, index: SourceSpan) -> &Self::Output {
        &self[index.start()..index.end()]
    }
}

impl RangeBounds<u32> for SourceSpan {
    #[inline(always)]
    fn start_bound(&self) -> Bound<&u32> {
        Bound::Included(&self.start)
    }

    #[inline(always)]
    fn end_bound(&self) -> Bound<&u32> {
        Bound::Excluded(&self.end)
    }
}
