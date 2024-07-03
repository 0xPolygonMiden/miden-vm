use alloc::vec::Vec;
use winter_utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

use super::{MastForest, MastNode, MastNodeId};

mod decorator;

mod info;
use info::MastNodeInfo;

mod basic_block_data_builder;
use basic_block_data_builder::BasicBlockDataBuilder;

mod basic_block_data_decoder;
use basic_block_data_decoder::BasicBlockDataDecoder;

#[cfg(test)]
mod tests;

// TYPE ALIASES
// ===============================================================================================

/// Specifies an offset into the `data` section of an encoded [`MastForest`].
type DataOffset = u32;

/// Specifies an offset into the `strings` table of an encoded [`MastForest`]
type StringIndex = usize;

// CONSTANTS
// ===============================================================================================

/// Magic string for detecting that a file is binary-encoded MAST.
const MAGIC: &[u8; 5] = b"MAST\0";

/// The format version.
///
/// If future modifications are made to this format, the version should be incremented by 1. A
/// version of `[255, 255, 255]` is reserved for future extensions that require extending the
/// version field itself, but should be considered invalid for now.
const VERSION: [u8; 3] = [0, 0, 0];

// STRING REF
// ===============================================================================================

/// An entry in the `strings` table of an encoded [`MastForest`].
///
/// Strings are UTF8-encoded.
#[derive(Debug)]
pub struct StringRef {
    /// Offset into the `data` section.
    offset: DataOffset,

    /// Length of the utf-8 string.
    len: u32,
}

impl Serializable for StringRef {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.offset.write_into(target);
        self.len.write_into(target);
    }
}

impl Deserializable for StringRef {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let offset = DataOffset::read_from(source)?;
        let len = source.read_u32()?;

        Ok(Self { offset, len })
    }
}

// MAST FOREST SERIALIZATION/DESERIALIZATION
// ===============================================================================================

impl Serializable for MastForest {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let mut basic_block_data_builder = BasicBlockDataBuilder::new();

        // magic & version
        target.write_bytes(MAGIC);
        target.write_bytes(&VERSION);

        // node count
        target.write_usize(self.nodes.len());

        // roots
        self.roots.write_into(target);

        // MAST node infos
        for mast_node in &self.nodes {
            let mast_node_info =
                MastNodeInfo::new(mast_node, basic_block_data_builder.current_data_offset());

            if let MastNode::Block(basic_block) = mast_node {
                basic_block_data_builder.encode_basic_block(basic_block);
            }

            mast_node_info.write_into(target);
        }

        let (data, string_table) = basic_block_data_builder.into_parts();

        string_table.write_into(target);
        data.write_into(target);
    }
}

impl Deserializable for MastForest {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let magic: [u8; 5] = source.read_array()?;
        if magic != *MAGIC {
            return Err(DeserializationError::InvalidValue(format!(
                "Invalid magic bytes. Expected '{:?}', got '{:?}'",
                *MAGIC, magic
            )));
        }

        let version: [u8; 3] = source.read_array()?;
        if version != VERSION {
            return Err(DeserializationError::InvalidValue(format!(
                "Unsupported version. Got '{version:?}', but only '{VERSION:?}' is supported",
            )));
        }

        let node_count = source.read_usize()?;

        let roots: Vec<MastNodeId> = Deserializable::read_from(source)?;

        let mast_node_infos = {
            let mut mast_node_infos = Vec::with_capacity(node_count);
            for _ in 0..node_count {
                let mast_node_info = MastNodeInfo::read_from(source)?;
                mast_node_infos.push(mast_node_info);
            }

            mast_node_infos
        };

        let strings: Vec<StringRef> = Deserializable::read_from(source)?;

        let data: Vec<u8> = Deserializable::read_from(source)?;

        let mut basic_block_data_decoder = BasicBlockDataDecoder::new(&data, &strings);

        let mast_forest = {
            let mut mast_forest = MastForest::new();

            for mast_node_info in mast_node_infos {
                let node = mast_node_info
                    .try_into_mast_node(&mast_forest, &mut basic_block_data_decoder)?;

                mast_forest.add_node(node);
            }

            for root in roots {
                mast_forest.make_root(root);
            }

            mast_forest
        };

        Ok(mast_forest)
    }
}
