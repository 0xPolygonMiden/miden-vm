use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};
use core::cell::RefCell;

use miden_crypto::hash::blake::{Blake3Digest, Blake3_256};
use winter_utils::{
    ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable, SliceReader,
};

use super::StringDataOffset;

pub struct StringTable {
    data: Vec<u8>,

    /// This field is used to allocate an `Arc` for any string in `strings` where the decoder
    /// requests a reference-counted string rather than a fresh allocation as a `String`.
    ///
    /// Currently, this is only used for debug information (source file names), but most cases
    /// where strings are stored in MAST are stored as `Arc` in practice, we just haven't yet
    /// updated all of the decoders.
    ///
    /// We lazily allocate an `Arc` when strings are decoded as an `Arc`, but the underlying
    /// string data corresponds to the same index in `strings`. All future requests for a
    /// ref-counted string we've allocated an `Arc` for, will clone the `Arc` rather than
    /// allocate a fresh string.
    refc_strings: Vec<RefCell<Option<Arc<str>>>>,
}

impl StringTable {
    pub fn new(data: Vec<u8>) -> Self {
        // TODO(plafer): we no longer store the strings table (i.e. where all strings start), so
        // this is *way* bigger than it needs to be. It is currently *correct* but very memory
        // inefficient. Bring back string table (with offsets), or use a `BTreeMap<usize,
        // RefCell<_>>`.
        let mut refc_strings = Vec::with_capacity(data.len());
        refc_strings.resize(data.len(), RefCell::new(None));

        Self { data, refc_strings }
    }

    pub fn read_arc_str(
        &self,
        str_offset: StringDataOffset,
    ) -> Result<Arc<str>, DeserializationError> {
        if let Some(cached) =
            self.refc_strings.get(str_offset).and_then(|cell| cell.borrow().clone())
        {
            return Ok(cached);
        }

        let string = Arc::from(self.read_string(str_offset)?.into_boxed_str());
        *self.refc_strings[str_offset].borrow_mut() = Some(Arc::clone(&string));
        Ok(string)
    }

    pub fn read_string(
        &self,
        str_offset: StringDataOffset,
    ) -> Result<String, DeserializationError> {
        let mut reader = SliceReader::new(&self.data[str_offset..]);
        reader.read()
    }
}

impl Serializable for StringTable {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let Self { data, refc_strings: _ } = self;

        data.write_into(target);
    }
}

impl Deserializable for StringTable {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let data = source.read()?;

        Ok(Self::new(data))
    }
}

// STRING TABLE BUILDER
// ================================================================================================

#[derive(Debug, Default)]
pub struct StringTableBuilder {
    str_to_offset: BTreeMap<Blake3Digest<32>, StringDataOffset>,
    strings_data: Vec<u8>,
}

impl StringTableBuilder {
    pub fn add_string(&mut self, string: &str) -> StringDataOffset {
        if let Some(str_idx) = self.str_to_offset.get(&Blake3_256::hash(string.as_bytes())) {
            // return already interned string
            *str_idx
        } else {
            // add new string to table
            let str_offset = self.strings_data.len();
            assert!(str_offset <= u32::MAX as usize, "strings table larger than 2^32 bytes");

            string.write_into(&mut self.strings_data);
            self.str_to_offset.insert(Blake3_256::hash(string.as_bytes()), str_offset);

            str_offset
        }
    }

    pub fn into_table(self) -> StringTable {
        StringTable::new(self.strings_data)
    }
}
