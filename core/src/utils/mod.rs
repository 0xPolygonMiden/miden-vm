use crate::Felt;
use alloc::{string::String, vec::Vec};
use core::{
    fmt::{self, Debug},
    ops::{Bound, Range},
};

// RE-EXPORTS
// ================================================================================================

pub use miden_crypto::utils::{
    collections, uninit_vector, ByteReader, ByteWriter, Deserializable, DeserializationError,
    Serializable, SliceReader,
};
pub use winter_utils::{group_slice_elements, group_vector_elements};

pub mod math {
    pub use math::{batch_inversion, log2};
}

// BYTE READERS/WRITERS
// ================================================================================================

#[cfg(feature = "std")]
use std::{
    cell::{Ref, RefCell},
    io::{BufRead, Write},
    string::ToString,
};

/// An adapter of [ByteWriter] to any type that implements [std::io::Write]
///
/// In particular, this covers things like [std::fs::File], standard output, etc.
///
/// This implementation uses a buffered writer internally for efficiency, and will
/// ensure the writer is flushed when the adapter is dropped.
#[cfg(feature = "std")]
pub struct WriteAdapter<'a> {
    writer: std::io::BufWriter<&'a mut dyn std::io::Write>,
}

#[cfg(feature = "std")]
impl<'a> WriteAdapter<'a> {
    pub fn new(writer: &'a mut dyn std::io::Write) -> Self {
        Self {
            writer: std::io::BufWriter::new(writer),
        }
    }
}

#[cfg(feature = "std")]
impl<'a> Drop for WriteAdapter<'a> {
    fn drop(&mut self) {
        self.writer.flush().expect("flush failed");
    }
}

#[cfg(feature = "std")]
impl<'a> ByteWriter for WriteAdapter<'a> {
    fn write_u8(&mut self, byte: u8) {
        self.writer.write_all(&[byte]).expect("write failed");
    }
    fn write_bytes(&mut self, bytes: &[u8]) {
        self.writer.write_all(bytes).expect("write failed");
    }
}

/// An adapter of [ByteReader] to any type that implements [std::io::Read]
///
/// In particular, this covers things like [std::fs::File], standard input, etc.
#[cfg(feature = "std")]
pub struct ReadAdapter<'a> {
    // NOTE: We use [RefCell] here to handle the fact that the [ByteReader]
    // trait does not support reading during certain methods, when those methods
    // might require reading from the underlying input to return a correct answer.
    //
    // To handle this, we wrap the reader in an [RefCell], and enforce the
    // mutable/immutable borrow semantics dynamically, allowing us to safely
    // mutate the reader during calls to `peek_u8`, and friends
    reader: RefCell<std::io::BufReader<&'a mut dyn std::io::Read>>,
    buf: alloc::vec::Vec<u8>,
    pos: usize,
    // This is set when we attempt to read from `reader` and get an empty
    // buffer. We will use this to more accurately handle functions like
    // `has_more_bytes` when this is set.
    guaranteed_eof: bool,
}

/// Builder
#[cfg(feature = "std")]
impl<'a> ReadAdapter<'a> {
    /// Create a new adapter for the given buffered reader
    pub fn new(reader: &'a mut dyn std::io::Read) -> Self {
        Self {
            reader: RefCell::new(std::io::BufReader::with_capacity(256, reader)),
            buf: Default::default(),
            pos: 0,
            guaranteed_eof: false,
        }
    }
}

/// Helpers
#[cfg(feature = "std")]
impl<'a> ReadAdapter<'a> {
    /// Get the internal adapter buffer as a (possibly empty) slice of bytes
    #[inline(always)]
    fn buffer(&self) -> &[u8] {
        self.buf.get(self.pos..).unwrap_or(&[])
    }

    /// Get the internal adapter buffer as a slice of bytes, or `None` if the buffer is empty
    #[inline(always)]
    fn non_empty_buffer(&self) -> Option<&[u8]> {
        self.buf.get(self.pos..).filter(|b| !b.is_empty())
    }

    /// Return the current reader buffer as a (possibly empty) slice of bytes.
    ///
    /// This buffer being empty _does not_ mean we're at EOF, you must call [non_empty_reader_buffer_mut] first.
    #[inline(always)]
    fn reader_buffer(&self) -> Ref<'_, [u8]> {
        Ref::map(self.reader.borrow(), |r| r.buffer())
    }

    /// Return the current reader buffer, reading from the underlying reader
    /// if the buffer is empty.
    ///
    /// Returns `Ok` only if the buffer is non-empty, and no errors occurred
    /// while filling it (if filling was needed).
    fn non_empty_reader_buffer_mut(&mut self) -> Result<&[u8], DeserializationError> {
        use std::io::ErrorKind;
        let buf = self.reader.get_mut().fill_buf().map_err(|e| match e.kind() {
            ErrorKind::UnexpectedEof => DeserializationError::UnexpectedEOF,
            e => DeserializationError::UnknownError(e.to_string()),
        })?;
        if buf.is_empty() {
            self.guaranteed_eof = true;
            Err(DeserializationError::UnexpectedEOF)
        } else {
            Ok(buf)
        }
    }

    /// Same as [non_empty_reader_buffer_mut], but with dynamically-enforced
    /// borrow check rules so that it can be called in functions like `peek_u8`.
    ///
    /// This comes with overhead for the dynamic checks, so you should prefer
    /// to call [non_empty_reader_buffer_mut] if you already have a mutable
    /// reference to `self`
    fn non_empty_reader_buffer(&self) -> Result<Ref<'_, [u8]>, DeserializationError> {
        use std::io::ErrorKind;
        let mut reader = self.reader.borrow_mut();
        let buf = reader.fill_buf().map_err(|e| match e.kind() {
            ErrorKind::UnexpectedEof => DeserializationError::UnexpectedEOF,
            e => DeserializationError::UnknownError(e.to_string()),
        })?;
        if buf.is_empty() {
            Err(DeserializationError::UnexpectedEOF)
        } else {
            // Re-borrow immutably
            drop(reader);
            Ok(self.reader_buffer())
        }
    }

    #[inline]
    fn has_remaining_capacity(&self, n: usize) -> bool {
        let remaining = self.buf.capacity() - self.buffer().len();
        remaining >= n
    }

    /// Takes the next byte from the input, returning an error if the operation fails
    fn pop(&mut self) -> Result<u8, DeserializationError> {
        if let Some(byte) = self.non_empty_buffer().map(|b| b[0]) {
            self.pos += 1;
            return Ok(byte);
        }
        match self.non_empty_reader_buffer_mut().map(|b| b[0]) {
            ok @ Ok(_) => {
                self.reader.get_mut().consume(1);
                ok
            }
            err @ Err(_) => {
                self.guaranteed_eof = true;
                err
            }
        }
    }

    /// Takes the next `N` bytes from the input as an array, returning an error if the operation fails
    fn read_exact<const N: usize>(&mut self) -> Result<[u8; N], DeserializationError> {
        let buf = self.buffer();
        let mut output = [0; N];
        match buf.len() {
            0 => {
                let buf = self.non_empty_reader_buffer_mut()?;
                if buf.len() < N {
                    return Err(DeserializationError::UnexpectedEOF);
                }
                unsafe {
                    core::ptr::copy_nonoverlapping(buf.as_ptr(), output.as_mut_ptr(), N);
                    self.reader.get_mut().consume(N);
                }
            }
            n if n >= N => unsafe {
                core::ptr::copy_nonoverlapping(buf.as_ptr(), output.as_mut_ptr(), N);
                self.pos += N;
            },
            n => {
                // We have to fill from both the local and reader buffers
                self.non_empty_reader_buffer_mut()?;
                let reader_buf = self.reader_buffer();
                match reader_buf.len() {
                    // We've reached eof prematurely
                    //
                    // SAFETY: The implementation of non_empty_reader_buffer_mut
                    // makes an empty buffer impossible
                    0 => unsafe { core::hint::unreachable_unchecked() },
                    // We got enough in one request
                    m if m + n >= N => {
                        let needed = N - n;
                        let dst = output.as_mut_ptr();
                        // SAFETY: Both copies are guaranteed to be in-bounds
                        unsafe {
                            core::ptr::copy_nonoverlapping(self.buffer().as_ptr(), dst, n);
                            core::ptr::copy_nonoverlapping(reader_buf.as_ptr(), dst.add(n), needed);
                            drop(reader_buf);
                            self.pos += n;
                            self.reader.get_mut().consume(needed);
                        }
                    }
                    // We didn't get enough, but haven't
                    // necessarily reached eof yet, so
                    // fall back to filling `self.buf`
                    m => {
                        let needed = N - (m + n);
                        drop(reader_buf);
                        self.buffer_at_least(needed)?;
                        assert!(self.buffer().len() >= N);
                        // SAFETY: This is guaranteed to be an in-bounds copy
                        unsafe {
                            core::ptr::copy_nonoverlapping(
                                self.buffer().as_ptr(),
                                output.as_mut_ptr(),
                                N,
                            );
                            self.pos += N;
                        }
                        return Ok(output);
                    }
                }
            }
        }

        // Check if we should reset our internal buffer
        if self.buffer().is_empty() && self.pos > 0 {
            unsafe {
                self.buf.set_len(0);
            }
        }

        Ok(output)
    }

    /// Fill `self.buf` with `count` bytes
    ///
    /// This should only be called when we can't read from the reader directly
    fn buffer_at_least(&mut self, mut count: usize) -> Result<(), DeserializationError> {
        loop {
            if count == 0 || self.buf.len() >= count {
                break Ok(());
            }
            self.non_empty_reader_buffer_mut()?;
            // We have to re-borrow the reader buffer here
            // to copy bytes between the two buffers.
            let reader = self.reader.get_mut();
            let buf = reader.buffer();
            let consumed = buf.len();
            self.buf.extend_from_slice(buf);
            reader.consume(consumed);
            count = count.saturating_sub(consumed);
        }
    }
}

#[cfg(feature = "std")]
impl<'a> ByteReader for ReadAdapter<'a> {
    #[inline]
    fn read_u8(&mut self) -> Result<u8, DeserializationError> {
        self.pop()
    }
    /// NOTE: If we happen to not have any bytes buffered yet
    /// when this is called
    fn peek_u8(&self) -> Result<u8, DeserializationError> {
        if let Some(byte) = self.buffer().first() {
            return Ok(*byte);
        }
        self.non_empty_reader_buffer().map(|b| b[0])
    }
    fn read_slice(&mut self, len: usize) -> Result<&[u8], DeserializationError> {
        // Edge case
        if len == 0 {
            return Ok(&[]);
        }

        // If we have unused buffer, and the consumed portion is
        // large enough, we will move the unused portion of the buffer
        // to the start, freeing up bytes at the end for more reads
        // before forcing a reallocation
        let should_optimize_storage = self.pos >= 16 && !self.has_remaining_capacity(len);
        if should_optimize_storage {
            // We're going to optimize storage first
            let buf = self.buffer();
            let src = buf.as_ptr();
            let count = buf.len();
            let dst = self.buf.as_mut_ptr();
            unsafe {
                core::ptr::copy(src, dst, count);
                self.buf.set_len(count);
                self.pos = 0;
            }
        }

        // Fill the buffer so we have at least `len` bytes available,
        // this will return an error if we hit EOF first
        self.buffer_at_least(len)?;

        Ok(&self.buffer()[0..len])
    }
    #[inline]
    fn read_array<const N: usize>(&mut self) -> Result<[u8; N], DeserializationError> {
        if N == 0 {
            return Ok([0; N]);
        }
        self.read_exact()
    }
    fn check_eor(&self, num_bytes: usize) -> Result<(), DeserializationError> {
        // Do we have sufficient data in the local buffer?
        let buffer_len = self.buffer().len();
        if buffer_len >= num_bytes {
            return Ok(());
        }

        // What about if we include what is in the local buffer and the reader's buffer?
        let reader_buffer_len = self.non_empty_reader_buffer().map(|b| b.len())?;
        let buffer_len = buffer_len + reader_buffer_len;
        if buffer_len >= num_bytes {
            return Ok(());
        }

        // We have no more input, thus can't fulfill a request of `num_bytes`
        if self.guaranteed_eof {
            return Err(DeserializationError::UnexpectedEOF);
        }

        // Because this function is read-only, we must optimistically assume we
        // can read `num_bytes` from the input, and fail later if that does not
        // hold. We know we're not at EOF yet, but that's all we can say without
        // buffering more from the reader. We could implement a different `buffer_at_least`
        // using dynamic borrow check rules, but there is little reason to do so
        // here
        Ok(())
    }
    #[inline]
    fn has_more_bytes(&self) -> bool {
        !self.buffer().is_empty() || self.non_empty_reader_buffer().is_ok()
    }
}

// TO ELEMENTS
// ================================================================================================

pub trait ToElements {
    fn to_elements(&self) -> Vec<Felt>;
}

impl<const N: usize> ToElements for [u64; N] {
    fn to_elements(&self) -> Vec<Felt> {
        self.iter().map(|&v| Felt::new(v)).collect()
    }
}

impl ToElements for Vec<u64> {
    fn to_elements(&self) -> Vec<Felt> {
        self.iter().map(|&v| Felt::new(v)).collect()
    }
}

// INTO BYTES
// ================================================================================================

pub trait IntoBytes<const N: usize> {
    fn into_bytes(self) -> [u8; N];
}

impl IntoBytes<32> for [Felt; 4] {
    fn into_bytes(self) -> [u8; 32] {
        let mut result = [0; 32];

        result[..8].copy_from_slice(&self[0].as_int().to_le_bytes());
        result[8..16].copy_from_slice(&self[1].as_int().to_le_bytes());
        result[16..24].copy_from_slice(&self[2].as_int().to_le_bytes());
        result[24..].copy_from_slice(&self[3].as_int().to_le_bytes());

        result
    }
}

// PUSH MANY
// ================================================================================================

pub trait PushMany<T> {
    fn push_many(&mut self, value: T, n: usize);
}

impl<T: Copy> PushMany<T> for Vec<T> {
    fn push_many(&mut self, value: T, n: usize) {
        let new_len = self.len() + n;
        self.resize(new_len, value);
    }
}

// RANGE
// ================================================================================================

/// Returns a [Range] initialized with the specified `start` and with `end` set to `start` + `len`.
pub const fn range(start: usize, len: usize) -> Range<usize> {
    Range {
        start,
        end: start + len,
    }
}

/// Converts and parses a [Bound] into an included u64 value.
pub fn bound_into_included_u64<I>(bound: Bound<&I>, is_start: bool) -> u64
where
    I: Clone + Into<u64>,
{
    match bound {
        Bound::Excluded(i) => i.clone().into().saturating_sub(1),
        Bound::Included(i) => i.clone().into(),
        Bound::Unbounded => {
            if is_start {
                0
            } else {
                u64::MAX
            }
        }
    }
}

// ARRAY CONSTRUCTORS
// ================================================================================================

/// Returns an array of N vectors initialized with the specified capacity.
pub fn new_array_vec<T: Debug, const N: usize>(capacity: usize) -> [Vec<T>; N] {
    (0..N)
        .map(|_| Vec::with_capacity(capacity))
        .collect::<Vec<_>>()
        .try_into()
        .expect("failed to convert vector to array")
}

#[test]
#[should_panic]
fn debug_assert_is_checked() {
    // enforce the release checks to always have `RUSTFLAGS="-C debug-assertions".
    //
    // some upstream tests are performed with `debug_assert`, and we want to assert its correctness
    // downstream.
    //
    // for reference, check
    // https://github.com/0xPolygonMiden/miden-vm/issues/433
    debug_assert!(false);
}

// FORMATTING
// ================================================================================================

/// Utility to convert a sequence of bytes to hex.
pub fn to_hex(bytes: &[u8]) -> Result<String, fmt::Error> {
    use core::fmt::Write;

    let mut s = String::with_capacity(bytes.len() * 2);
    write!(s, "{:x}", DisplayHex(bytes))?;
    Ok(s)
}

/// A display helper for formatting a slice of bytes as hex
/// with different options using Rust's builtin format language
pub struct DisplayHex<'a>(pub &'a [u8]);

impl<'a> fmt::Display for DisplayHex<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

#[cfg(feature = "formatter")]
impl<'a> crate::prettier::PrettyPrint for DisplayHex<'a> {
    fn render(&self) -> crate::prettier::Document {
        crate::prettier::text(format!("{:#x}", self))
    }
}

impl<'a> fmt::LowerHex for DisplayHex<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }
        for byte in self.0.iter() {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

/// Builds a hex string from a byte slice
pub fn write_hex_bytes(f: &mut fmt::Formatter<'_>, bytes: &[u8]) -> fmt::Result {
    write!(f, "{:#x}", DisplayHex(bytes))
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn read_adapter_empty() -> Result<(), DeserializationError> {
        let mut reader = std::io::empty();
        let mut adapter = ReadAdapter::new(&mut reader);
        assert!(!adapter.has_more_bytes());
        assert_eq!(adapter.check_eor(8), Err(DeserializationError::UnexpectedEOF));
        assert_eq!(adapter.peek_u8(), Err(DeserializationError::UnexpectedEOF));
        assert_eq!(adapter.read_u8(), Err(DeserializationError::UnexpectedEOF));
        assert_eq!(adapter.read_slice(0), Ok([].as_slice()));
        assert_eq!(adapter.read_slice(1), Err(DeserializationError::UnexpectedEOF));
        assert_eq!(adapter.read_array(), Ok([]));
        assert_eq!(adapter.read_array::<1>(), Err(DeserializationError::UnexpectedEOF));
        Ok(())
    }

    #[test]
    fn read_adapter_passthrough() -> Result<(), DeserializationError> {
        let mut reader = std::io::repeat(0b101);
        let mut adapter = ReadAdapter::new(&mut reader);
        assert!(adapter.has_more_bytes());
        assert_eq!(adapter.check_eor(8), Ok(()));
        assert_eq!(adapter.peek_u8(), Ok(0b101));
        assert_eq!(adapter.read_u8(), Ok(0b101));
        assert_eq!(adapter.read_slice(0), Ok([].as_slice()));
        assert_eq!(adapter.read_slice(4), Ok([0b101, 0b101, 0b101, 0b101].as_slice()));
        assert_eq!(adapter.read_array(), Ok([]));
        assert_eq!(adapter.read_array(), Ok([0b101, 0b101]));
        Ok(())
    }

    #[test]
    fn read_adapter_exact() {
        const VALUE: usize = 2048;
        let mut reader = Cursor::new(VALUE.to_le_bytes());
        let mut adapter = ReadAdapter::new(&mut reader);
        assert_eq!(usize::from_le_bytes(adapter.read_array().unwrap()), VALUE);
        assert!(!adapter.has_more_bytes());
        assert_eq!(adapter.peek_u8(), Err(DeserializationError::UnexpectedEOF));
        assert_eq!(adapter.read_u8(), Err(DeserializationError::UnexpectedEOF));
    }

    #[test]
    fn read_adapter_roundtrip() {
        const VALUE: usize = 2048;

        // Write VALUE to storage
        let mut storage = Cursor::new([0; core::mem::size_of::<usize>()]);
        let mut adapter = WriteAdapter::new(&mut storage);
        adapter.write_usize(VALUE);
        drop(adapter);

        // Read VALUE from storage
        storage.set_position(0);
        let mut adapter = ReadAdapter::new(&mut storage);

        assert_eq!(adapter.read_usize(), Ok(VALUE));
    }

    #[test]
    fn write_adapter_passthrough() {
        let mut writer = Cursor::new([0u8; 128]);
        let mut adapter = WriteAdapter::new(&mut writer);
        adapter.write_bytes(b"nope");
        drop(adapter);
        let buf = writer.get_ref();
        assert_eq!(&buf[..4], b"nope");
    }

    #[test]
    #[should_panic]
    fn write_adapter_writer_out_of_capacity() {
        let mut writer = Cursor::new([0; 2]);
        let mut adapter = WriteAdapter::new(&mut writer);
        adapter.write_bytes(b"nope");
    }
}
