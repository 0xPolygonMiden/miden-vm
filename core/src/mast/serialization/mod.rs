//! The serialization format of MastForest is as follows:
//!
//! (Metadata)
//! - MAGIC
//! - VERSION
//!
//! (lengths)
//! - decorators length (`usize`)
//! - nodes length (`usize`)
//!
//! (procedure roots)
//! - procedure roots (`Vec<MastNodeId>`)
//!
//! (raw data)
//! - Decorator data
//! - Node data
//! - String table
//!
//! (info structs)
//! - decorator infos (`Vec<DecoratorInfo>`)
//! - MAST node infos (`Vec<MastNodeInfo>`)
//!
//! (before enter and after exit decorators)
//! - before enter decorators (`Vec<(MastNodeId, Vec<DecoratorId>)>`)
//! - after exit decorators (`Vec<(MastNodeId, Vec<DecoratorId>)>`)

use alloc::vec::Vec;

use decorator::{DecoratorDataBuilder, DecoratorInfo};
use string_table::{StringTable, StringTableBuilder};
use winter_utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

use super::{DecoratorId, MastForest, MastNode, MastNodeId};

mod decorator;

mod info;
use info::MastNodeInfo;

mod basic_blocks;
use basic_blocks::{BasicBlockDataBuilder, BasicBlockDataDecoder};

mod string_table;

#[cfg(test)]
mod tests;

// TYPE ALIASES
// ================================================================================================

/// Specifies an offset into the `node_data` section of an encoded [`MastForest`].
type NodeDataOffset = u32;

/// Specifies an offset into the `decorator_data` section of an encoded [`MastForest`].
type DecoratorDataOffset = u32;

/// Specifies an offset into the `strings_data` section of an encoded [`MastForest`].
type StringDataOffset = usize;

/// Specifies an offset into the strings table of an encoded [`MastForest`].
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
        let mut decorator_data_builder = DecoratorDataBuilder::new();
        let mut string_table_builder = StringTableBuilder::default();

        // Set up "before enter" and "after exit" decorators by `MastNodeId`
        let mut before_enter_decorators: Vec<(usize, Vec<DecoratorId>)> = Vec::new();
        let mut after_exit_decorators: Vec<(usize, Vec<DecoratorId>)> = Vec::new();

        // magic & version
        target.write_bytes(MAGIC);
        target.write_bytes(&VERSION);

        // decorator & node counts
        target.write_usize(self.decorators.len());
        target.write_usize(self.nodes.len());

        // roots
        let roots: Vec<u32> = self.roots.iter().map(u32::from).collect();
        roots.write_into(target);

        // decorators
        let decorator_infos: Vec<DecoratorInfo> = self
            .decorators
            .iter()
            .map(|decorator| {
                DecoratorInfo::from_decorator(
                    decorator,
                    &mut decorator_data_builder,
                    &mut string_table_builder,
                )
            })
            .collect();

        // Prepare MAST node infos, but don't store them yet. We store them at the end to make
        // deserialization more efficient.
        let mast_node_infos: Vec<MastNodeInfo> = self
            .nodes
            .iter()
            .enumerate()
            .map(|(mast_node_id, mast_node)| {
                if !mast_node.before_enter().is_empty() {
                    before_enter_decorators.push((mast_node_id, mast_node.before_enter().to_vec()));
                }
                if !mast_node.after_exit().is_empty() {
                    after_exit_decorators.push((mast_node_id, mast_node.after_exit().to_vec()));
                }

                let (ops_offset, decorator_data_offset) = if let MastNode::Block(basic_block) =
                    mast_node
                {
                    let (ops_offset, decorator_data_offset) =
                        basic_block_data_builder.encode_basic_block(basic_block);

                    (ops_offset, decorator_data_offset.unwrap_or(MastForest::MAX_DECORATORS as u32))
                } else {
                    (0, 0)
                };

                MastNodeInfo::new(mast_node, ops_offset, decorator_data_offset)
            })
            .collect();

        let decorator_data = decorator_data_builder.finalize();
        let node_data = basic_block_data_builder.finalize();
        let string_table = string_table_builder.into_table();

        // Write 3 data buffers
        decorator_data.write_into(target);
        node_data.write_into(target);
        string_table.write_into(target);

        // Write decorator and node infos
        for decorator_info in decorator_infos {
            decorator_info.write_into(target);
        }

        for mast_node_info in mast_node_infos {
            mast_node_info.write_into(target);
        }

        // Write "before enter" and "after exit" decorators
        before_enter_decorators.write_into(target);
        after_exit_decorators.write_into(target);
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

        let decorator_count = source.read_usize()?;
        let node_count = source.read_usize()?;
        let roots: Vec<u32> = Deserializable::read_from(source)?;
        let decorator_data: Vec<u8> = Deserializable::read_from(source)?;
        let node_data: Vec<u8> = Deserializable::read_from(source)?;
        let string_table: StringTable = Deserializable::read_from(source)?;

        let mut mast_forest = {
            let mut mast_forest = MastForest::new();

            // decorators
            for _ in 0..decorator_count {
                let decorator_info = DecoratorInfo::read_from(source)?;
                let decorator =
                    decorator_info.try_into_decorator(&string_table, &decorator_data)?;

                mast_forest.add_decorator(decorator).map_err(|e| {
                    DeserializationError::InvalidValue(format!(
                        "failed to add decorator to MAST forest while deserializing: {e}",
                    ))
                })?;
            }

            // nodes
            let basic_block_data_decoder = BasicBlockDataDecoder::new(&node_data);
            for _ in 0..node_count {
                let mast_node_info = MastNodeInfo::read_from(source)?;

                let node = mast_node_info.try_into_mast_node(
                    &mast_forest,
                    node_count,
                    &basic_block_data_decoder,
                )?;

                mast_forest.add_node(node).map_err(|e| {
                    DeserializationError::InvalidValue(format!(
                        "failed to add node to MAST forest while deserializing: {e}",
                    ))
                })?;
            }

            // roots
            for root in roots {
                // make sure the root is valid in the context of the MAST forest
                let root = MastNodeId::from_u32_safe(root, &mast_forest)?;
                mast_forest.make_root(root);
            }

            mast_forest
        };

        // read "before enter" and "after exit" decorators, and update the corresponding nodes
        let before_enter_decorators: Vec<(usize, Vec<DecoratorId>)> =
            read_before_after_decorators(source, &mast_forest)?;
        for (node_id, decorator_ids) in before_enter_decorators {
            let node_id: u32 = node_id.try_into().map_err(|_| {
                DeserializationError::InvalidValue(format!(
                    "Invalid node id '{node_id}' while deserializing"
                ))
            })?;
            let node_id = MastNodeId::from_u32_safe(node_id, &mast_forest)?;
            mast_forest.set_before_enter(node_id, decorator_ids);
        }

        let after_exit_decorators: Vec<(usize, Vec<DecoratorId>)> =
            read_before_after_decorators(source, &mast_forest)?;
        for (node_id, decorator_ids) in after_exit_decorators {
            let node_id: u32 = node_id.try_into().map_err(|_| {
                DeserializationError::InvalidValue(format!(
                    "Invalid node id '{node_id}' while deserializing"
                ))
            })?;
            let node_id = MastNodeId::from_u32_safe(node_id, &mast_forest)?;
            mast_forest.set_after_exit(node_id, decorator_ids);
        }

        Ok(mast_forest)
    }
}

/// Reads the `before_enter_decorators` and `after_exit_decorators` of the serialized `MastForest`
/// format.
///
/// Note that we need this custom format because we cannot implement `Deserializable` for
/// `DecoratorId` (in favor of using [`DecoratorId::from_u32_safe`]).
fn read_before_after_decorators<R: ByteReader>(
    source: &mut R,
    mast_forest: &MastForest,
) -> Result<Vec<(usize, Vec<DecoratorId>)>, DeserializationError> {
    let vec_len: usize = source.read()?;
    let mut out_vec: Vec<_> = Vec::with_capacity(vec_len);

    for _ in 0..vec_len {
        let node_id: usize = source.read()?;

        let inner_vec_len: usize = source.read()?;
        let mut inner_vec: Vec<DecoratorId> = Vec::with_capacity(inner_vec_len);
        for _ in 0..inner_vec_len {
            let decorator_id = DecoratorId::from_u32_safe(source.read()?, mast_forest)?;
            inner_vec.push(decorator_id);
        }

        out_vec.push((node_id, inner_vec));
    }

    Ok(out_vec)
}
