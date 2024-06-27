use alloc::{string::String, vec::Vec};
use miden_crypto::hash::rpo::RpoDigest;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToBytes, ToPrimitive};
use thiserror::Error;
use winter_utils::{ByteWriter, Serializable};

use super::{MastForest, MastNode, MastNodeId};

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

pub struct MastNodeInfo {
    ty: MastNodeType,
    offset: DataOffset,
    digest: RpoDigest,
}

impl Serializable for MastNodeInfo {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.ty.write_into(target);
        self.offset.write_into(target);
        self.digest.write_into(target);
    }
}

// TODOP: Describe how first 4 bits (i.e. high order bits of first byte) are the discriminant
pub struct MastNodeType([u8; 8]);

impl MastNodeType {
    pub fn new(mast_node: &MastNode) -> Self {
        use MastNode::*;

        let discriminant = MastNodeTypeVariant::from_mast_node(mast_node).discriminant();
        assert!(discriminant < 2_u8.pow(4_u32));

        match mast_node {
            Block(block_node) => {
                let num_ops = block_node.num_operations_and_decorators().to_be_bytes();

                Self([discriminant << 4, num_ops[0], num_ops[1], num_ops[2], num_ops[3], 0, 0, 0])
            }
            Join(join_node) => {
                Self::encode_join_or_split(discriminant, join_node.first(), join_node.second())
            }
            Split(split_node) => Self::encode_join_or_split(
                discriminant,
                split_node.on_true(),
                split_node.on_false(),
            ),
            Loop(loop_node) => {
                let [body_byte1, body_byte2, body_byte3, body_byte4] =
                    loop_node.body().0.to_be_bytes();

                Self([discriminant << 4, body_byte1, body_byte2, body_byte3, body_byte4, 0, 0, 0])
            }
            Call(_) | Dyn | External(_) => Self([discriminant << 4, 0, 0, 0, 0, 0, 0, 0]),
        }
    }

    // TODOP: Make a diagram of how the bits are split
    fn encode_join_or_split(
        discriminant: u8,
        left_child_id: MastNodeId,
        right_child_id: MastNodeId,
    ) -> Self {
        assert!(left_child_id.0 < 2_u32.pow(30));
        assert!(right_child_id.0 < 2_u32.pow(30));

        let mut result: [u8; 8] = [0_u8; 8];

        result[0] = discriminant << 4;

        // write left child into result
        {
            let [lsb, a, b, msb] = left_child_id.0.to_le_bytes();
            result[0] |= lsb >> 4;
            result[1] |= lsb << 4;
            result[1] |= a >> 4;
            result[2] |= a << 4;
            result[2] |= b >> 4;
            result[3] |= b << 4;

            // msb is different from lsb, a and b since its 2 most significant bits are guaranteed
            // to be 0, and hence not encoded.
            //
            // More specifically, let the bits of msb be `00abcdef`. We encode `abcd` in `result[3]`,
            // and `ef` as the most significant bits of `result[4]`.
            result[3] |= msb >> 2;

            result[4] |= msb << 6;
        };

        // write right child into result
        {
            // Recall that `result[4]` contains 2 bits from the left child id in the most
            // significant bits. Also, the most significant byte of the right child is guaranteed to
            // fit in 6 bits. Hence, we use big endian format for the right child id to simplify
            // encoding and decoding.
            let [msb, a, b, lsb] = right_child_id.0.to_be_bytes();

            result[4] |= msb;
            result[5] = a;
            result[6] = b;
            result[7] = lsb;
        };

        Self(result)
    }
}

impl Serializable for MastNodeType {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.0.write_into(target);
    }
}

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum MastNodeTypeVariant {
    Join,
    Split,
    Loop,
    Call,
    Syscall,
    Dyn,
    Block,
    External,
}

impl MastNodeTypeVariant {
    pub fn discriminant(&self) -> u8 {
        self.to_u8().expect("guaranteed to fit in a `u8` due to #[repr(u8)]")
    }

    pub fn try_from_discriminant(discriminant: u8) -> Result<Self, Error> {
        Self::from_u8(discriminant).ok_or_else(|| Error::InvalidDiscriminant {
            ty: "MastNode".into(),
            discriminant,
        })
    }

    pub fn from_mast_node(mast_node: &MastNode) -> Self {
        match mast_node {
            MastNode::Block(_) => Self::Block,
            MastNode::Join(_) => Self::Join,
            MastNode::Split(_) => Self::Split,
            MastNode::Loop(_) => Self::Loop,
            MastNode::Call(call_node) => {
                if call_node.is_syscall() {
                    Self::Syscall
                } else {
                    Self::Call
                }
            }
            MastNode::Dyn => Self::Dyn,
            MastNode::External(_) => Self::External,
        }
    }
}

impl Serializable for MastForest {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
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
            let mast_node_info = convert_mast_node(mast_node, &mut data, &mut strings);

            mast_node_info.write_into(target);
        }

        // strings table
        strings.write_into(target);

        // data blob
        data.write_into(target);
    }
}

fn convert_mast_node(
    mast_node: &MastNode,
    data: &mut Vec<u8>,
    strings: &mut Vec<StringRef>,
) -> MastNodeInfo {
    // mast node info

    // fill out encoded operations/decorators in data
    todo!()
}
