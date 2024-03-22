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
#[cfg(feature = "std")]
pub use winter_utils::ReadAdapter;
pub use winter_utils::{group_slice_elements, group_vector_elements};

pub mod math {
    pub use math::{batch_inversion, log2};
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
