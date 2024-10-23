use alloc::vec::Vec;

use serde::{Deserialize, Serialize};
use vm_core::{utils::ToElements, Felt, FieldElement};

use super::{de, se};
use crate::Digest;

/// This represents a descriptor for a pointer referencing data in Miden's linear memory.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PtrDesc {
    /// This is the address of the word containing the first byte of data
    pub waddr: u32,
    /// This is the element index of the word referenced by `waddr` containing the first byte of
    /// data
    ///
    /// Each element is assumed to be a 32-bit value/chunk
    pub index: u8,
    /// This is the byte offset into the 32-bit chunk referenced by `index`
    ///
    /// This offset is where the data referenced by the pointer actually starts.
    pub offset: u8,
}

/// Represents a read-only data segment, combined with its content digest
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rodata {
    /// The content digest computed for `data`
    #[serde(
        serialize_with = "se::serialize_digest",
        deserialize_with = "de::deserialize_digest"
    )]
    pub digest: Digest,
    /// The address at which the data for this segment begins
    pub start: PtrDesc,
    /// The raw binary data for this segment
    pub data: Vec<u8>,
}

impl Rodata {
    /// Returns the size of the data in bytes
    pub fn size_in_bytes(&self) -> usize {
        self.data.len()
    }

    /// Returns the size of the data in felts
    pub fn size_in_felts(&self) -> usize {
        self.data.len().next_multiple_of(4) / 4
    }

    /// Returns the size of the data in VM words
    pub fn size_in_words(&self) -> usize {
        self.size_in_felts().next_multiple_of(4) / 4
    }

    /// Converts this rodata object to its equivalent representation in felts
    ///
    /// The resulting felts will be in padded out to the nearest number of words, i.e. if the data
    /// only takes up 3 felts worth of bytes, then the resulting `Vec` will contain 4 felts, so that
    /// the total size is a valid number of words.
    pub fn to_elements(&self) -> Vec<Felt> {
        let mut felts = self.data.as_slice().to_elements();
        let padding = (self.size_in_words() * 4).abs_diff(felts.len());
        felts.resize(felts.len() + padding, Felt::ZERO);
        felts
    }
}
