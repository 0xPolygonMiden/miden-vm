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
// ================================================================================================

/// Specifies an offset into the `data` section of an encoded [`MastForest`].
type DataOffset = u32;

/// Specifies an offset into the `strings` table of an encoded [`MastForest`]
type StringIndex = usize;

// CONSTANTS
// ================================================================================================

/// Magic string for detecting that a file is binary-encoded MAST.
const MAGIC: &[u8; 5] = b"MAST\0";

/// The format version.
///
/// If future modifications are made to this format, the version should be incremented by 1. A
/// version of `[255, 255, 255]` is reserved for future extensions that require extending the
/// version field itself, but should be considered invalid for now.
const VERSION: [u8; 3] = [0, 0, 0];

// MAST FOREST SERIALIZATION/DESERIALIZATION
// ================================================================================================

impl Serializable for MastForest {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let mut basic_block_data_builder = BasicBlockDataBuilder::new();

        // magic & version
        target.write_bytes(MAGIC);
        target.write_bytes(&VERSION);

        // node count
        target.write_usize(self.nodes.len());

        // roots
        let roots: Vec<u32> = self.roots.iter().map(u32::from).collect();
        roots.write_into(target);

        // Prepare MAST node infos, but don't store them yet. We store them at the end to make
        // deserialization more efficient.
        let mast_node_infos: Vec<MastNodeInfo> = self
            .nodes
            .iter()
            .map(|mast_node| {
                let mast_node_info =
                    MastNodeInfo::new(mast_node, basic_block_data_builder.get_offset());

                if let MastNode::Block(basic_block) = mast_node {
                    basic_block_data_builder.encode_basic_block(basic_block);
                }

                mast_node_info
            })
            .collect();

        let (data, string_table) = basic_block_data_builder.into_parts();

        string_table.write_into(target);
        data.write_into(target);

        for mast_node_info in mast_node_infos {
            mast_node_info.write_into(target);
        }
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
        let roots: Vec<u32> = Deserializable::read_from(source)?;
        let strings: Vec<DataOffset> = Deserializable::read_from(source)?;
        let data: Vec<u8> = Deserializable::read_from(source)?;

        let basic_block_data_decoder = BasicBlockDataDecoder::new(&data, &strings);

        let mast_forest = {
            let mut mast_forest = MastForest::new();

            for _ in 0..node_count {
                let mast_node_info = MastNodeInfo::read_from(source)?;

                let node =
                    mast_node_info.try_into_mast_node(&mast_forest, &basic_block_data_decoder)?;

                mast_forest.add_node(node).map_err(|e| {
                    DeserializationError::InvalidValue(format!(
                        "failed to add node to MAST forest while deserializing: {e}",
                    ))
                })?;
            }

            for root in roots {
                // make sure the root is valid in the context of the MAST forest
                let root = MastNodeId::from_u32_safe(root, &mast_forest)?;
                mast_forest.make_root(root);
            }

            mast_forest
        };

        Ok(mast_forest)
    }
}
