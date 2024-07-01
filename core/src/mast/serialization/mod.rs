use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use thiserror::Error;
use winter_utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

use crate::mast::MerkleTreeNode;

use super::{MastForest, MastNode, MastNodeId};

mod info;
use info::{EncodedMastNodeType, MastNodeInfo, MastNodeTypeVariant};

/// Specifies an offset into the `data` section of an encoded [`MastForest`].
type DataOffset = u32;

/// Magic string for detecting that a file is binary-encoded MAST.
const MAGIC: &[u8; 5] = b"MAST\0";

/// The format version.
///
/// If future modifications are made to this format, the version should be incremented by 1. A
/// version of `[255, 255, 255]` is reserved for future extensions that require extending the
/// version field itself, but should be considered invalid for now.
const VERSION: [u8; 3] = [0, 0, 0];

// TODOP: move into info.rs? Make public?
#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid discriminant '{discriminant}' for type '{ty}'")]
    InvalidDiscriminant { ty: String, discriminant: u8 },
}

/// An entry in the `strings` table of an encoded [`MastForest`].
///
/// Strings are UTF8-encoded.
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

impl Serializable for MastForest {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        // TODOP: make sure padding is in accordance with Paul's docs
        let mut strings: Vec<StringRef> = Vec::new();
        let mut data: Vec<u8> = Vec::new();

        // magic & version
        target.write_bytes(MAGIC);
        target.write_bytes(&VERSION);

        // node count
        target.write_usize(self.nodes.len());

        // roots
        self.roots.write_into(target);

        // MAST node infos
        for mast_node in &self.nodes {
            let mast_node_info = mast_node_to_info(mast_node, &mut data, &mut strings);

            mast_node_info.write_into(target);
        }

        // strings table
        strings.write_into(target);

        // data blob
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

        let mast_forest = {
            let mut mast_forest = MastForest::new();

            for mast_node_info in mast_node_infos {
                let node = try_info_to_mast_node(mast_node_info, &mast_forest, &data, &strings)?;
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

fn mast_node_to_info(
    mast_node: &MastNode,
    data: &mut Vec<u8>,
    strings: &mut Vec<StringRef>,
) -> MastNodeInfo {
    use MastNode::*;

    let ty = EncodedMastNodeType::new(mast_node);
    let digest = mast_node.digest();

    let offset = match mast_node {
        Block(_) => todo!(),
        Join(_) | Split(_) | Loop(_) | Call(_) | Dyn | External(_) => 0,
    };

    MastNodeInfo { ty, offset, digest }
}

fn try_info_to_mast_node(
    mast_node_info: MastNodeInfo,
    mast_forest: &MastForest,
    data: &[u8],
    strings: &[StringRef],
) -> Result<MastNode, DeserializationError> {
    let mast_node_variant = mast_node_info
        .ty
        .variant()
        .map_err(|err| DeserializationError::InvalidValue(err.to_string()))?;

    // TODOP: Make a faillible version of `MastNode` ctors
    // TODOP: Check digest of resulting `MastNode` matches `MastNodeInfo.digest`?
    match mast_node_variant {
        MastNodeTypeVariant::Block => todo!(),
        MastNodeTypeVariant::Join => {
            let (left_child, right_child) =
                EncodedMastNodeType::decode_join_or_split(&mast_node_info.ty);

            Ok(MastNode::new_join(left_child, right_child, mast_forest))
        }
        MastNodeTypeVariant::Split => {
            let (if_branch, else_branch) =
                EncodedMastNodeType::decode_join_or_split(&mast_node_info.ty);

            Ok(MastNode::new_split(if_branch, else_branch, mast_forest))
        }
        MastNodeTypeVariant::Loop => {
            let body_id = EncodedMastNodeType::decode_u32_payload(&mast_node_info.ty);

            Ok(MastNode::new_loop(MastNodeId(body_id), mast_forest))
        }
        MastNodeTypeVariant::Call => {
            let callee_id = EncodedMastNodeType::decode_u32_payload(&mast_node_info.ty);

            Ok(MastNode::new_call(MastNodeId(callee_id), mast_forest))
        }
        MastNodeTypeVariant::Syscall => {
            let callee_id = EncodedMastNodeType::decode_u32_payload(&mast_node_info.ty);

            Ok(MastNode::new_syscall(MastNodeId(callee_id), mast_forest))
        }
        MastNodeTypeVariant::Dyn => Ok(MastNode::new_dynexec()),
        MastNodeTypeVariant::External => Ok(MastNode::new_external(mast_node_info.digest)),
    }
}
