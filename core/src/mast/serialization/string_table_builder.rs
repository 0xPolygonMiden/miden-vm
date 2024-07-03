use alloc::{collections::BTreeMap, vec::Vec};
use miden_crypto::hash::rpo::{Rpo256, RpoDigest};

use super::{StringIndex, StringRef};

#[derive(Debug, Default)]
pub struct StringTableBuilder {
    table: Vec<StringRef>,
    str_to_index: BTreeMap<RpoDigest, StringIndex>,
    strings_data: Vec<u8>,
}

impl StringTableBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_string(&mut self, string: &str) -> StringIndex {
        if let Some(str_idx) = self.str_to_index.get(&Rpo256::hash(string.as_bytes())) {
            // return already interned string
            *str_idx
        } else {
            // add new string to table
            // NOTE: these string refs' offset will need to be shifted again in `into_buffer()`
            let str_ref = StringRef {
                offset: self
                    .strings_data
                    .len()
                    .try_into()
                    .expect("strings table larger than 2^32 bytes"),
                len: string.len().try_into().expect("string larger than 2^32 bytes"),
            };
            let str_idx = self.table.len();

            self.strings_data.extend(string.as_bytes());
            self.table.push(str_ref);
            self.str_to_index.insert(Rpo256::hash(string.as_bytes()), str_idx);

            str_idx
        }
    }

    pub fn into_table(self, data: &mut Vec<u8>) -> Vec<StringRef> {
        let table_offset: u32 = data
            .len()
            .try_into()
            .expect("MAST forest serialization: data field longer than 2^32 bytes");
        data.extend(self.strings_data);

        self.table
            .into_iter()
            .map(|str_ref| StringRef {
                offset: str_ref.offset + table_offset,
                len: str_ref.len,
            })
            .collect()
    }
}
