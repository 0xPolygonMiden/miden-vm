use alloc::vec::Vec;
use winter_utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

use crate::mast::MerkleTreeNode;

use super::{MastForest, MastNode, MastNodeId};

mod decorator;

mod info;
use info::{MastNodeInfo, MastNodeType};

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
                let node = try_info_to_mast_node(
                    mast_node_info,
                    &mast_forest,
                    &mut basic_block_data_decoder,
                )?;
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

// TODOP: Make `MastNodeInfo` method
fn try_info_to_mast_node(
    mast_node_info: MastNodeInfo,
    mast_forest: &MastForest,
    basic_block_data_decoder: &mut BasicBlockDataDecoder,
) -> Result<MastNode, DeserializationError> {
    let mast_node = match mast_node_info.ty {
        MastNodeType::Block {
            len: num_operations_and_decorators,
        } => {
            let (operations, decorators) = basic_block_data_decoder
                .decode_operations_and_decorators(num_operations_and_decorators)?;

            Ok(MastNode::new_basic_block_with_decorators(operations, decorators))
        }
        MastNodeType::Join {
            left_child_id,
            right_child_id,
        } => {
            let left_child = MastNodeId::from_u32_safe(left_child_id, mast_forest)?;
            let right_child = MastNodeId::from_u32_safe(right_child_id, mast_forest)?;

            Ok(MastNode::new_join(left_child, right_child, mast_forest))
        }
        MastNodeType::Split {
            if_branch_id,
            else_branch_id,
        } => {
            let if_branch = MastNodeId::from_u32_safe(if_branch_id, mast_forest)?;
            let else_branch = MastNodeId::from_u32_safe(else_branch_id, mast_forest)?;

            Ok(MastNode::new_split(if_branch, else_branch, mast_forest))
        }
        MastNodeType::Loop { body_id } => {
            let body_id = MastNodeId::from_u32_safe(body_id, mast_forest)?;

            Ok(MastNode::new_loop(body_id, mast_forest))
        }
        MastNodeType::Call { callee_id } => {
            let callee_id = MastNodeId::from_u32_safe(callee_id, mast_forest)?;

            Ok(MastNode::new_call(callee_id, mast_forest))
        }
        MastNodeType::SysCall { callee_id } => {
            let callee_id = MastNodeId::from_u32_safe(callee_id, mast_forest)?;

            Ok(MastNode::new_syscall(callee_id, mast_forest))
        }
        MastNodeType::Dyn => Ok(MastNode::new_dynexec()),
        MastNodeType::External => Ok(MastNode::new_external(mast_node_info.digest)),
    }?;

    if mast_node.digest() == mast_node_info.digest {
        Ok(mast_node)
    } else {
        Err(DeserializationError::InvalidValue(format!(
            "MastNodeInfo's digest '{}' doesn't match deserialized MastNode's digest '{}'",
            mast_node_info.digest,
            mast_node.digest()
        )))
    }
}
