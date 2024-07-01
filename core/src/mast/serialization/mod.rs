use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use miden_crypto::hash::rpo::RpoDigest;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use thiserror::Error;
use winter_utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

use crate::mast::MerkleTreeNode;

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

impl Deserializable for StringRef {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let offset = DataOffset::read_from(source)?;
        let len = source.read_u32()?;

        Ok(Self { offset, len })
    }
}

pub struct MastNodeInfo {
    ty: EncodedMastNodeType,
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

impl Deserializable for MastNodeInfo {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let ty = Deserializable::read_from(source)?;
        let offset = DataOffset::read_from(source)?;
        let digest = RpoDigest::read_from(source)?;

        Ok(Self { ty, offset, digest })
    }
}

// TODOP: Describe how first 4 bits (i.e. high order bits of first byte) are the discriminant
pub struct EncodedMastNodeType([u8; 8]);

/// Constructors
impl EncodedMastNodeType {
    pub fn new(mast_node: &MastNode) -> Self {
        use MastNode::*;

        let discriminant = MastNodeTypeVariant::from_mast_node(mast_node).discriminant();
        assert!(discriminant < 2_u8.pow(4_u32));

        match mast_node {
            Block(block_node) => {
                let num_ops = block_node.num_operations_and_decorators();

                Self::encode_u32_payload(discriminant, num_ops)
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
                let child_id = loop_node.body().0;

                Self::encode_u32_payload(discriminant, child_id)
            }
            Call(call_node) => {
                let child_id = call_node.callee().0;

                Self::encode_u32_payload(discriminant, child_id)
            }
            Dyn | External(_) => Self([discriminant << 4, 0, 0, 0, 0, 0, 0, 0]),
        }
    }
}

/// Accessors
impl EncodedMastNodeType {
    pub fn variant(&self) -> Result<MastNodeTypeVariant, Error> {
        let discriminant = self.0[0] >> 4;

        MastNodeTypeVariant::try_from_discriminant(discriminant)
    }
}

/// Helpers
impl EncodedMastNodeType {
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
            // More specifically, let the bits of msb be `00abcdef`. We encode `abcd` in
            // `result[3]`, and `ef` as the most significant bits of `result[4]`.
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

    fn decode_join_or_split(&self) -> (MastNodeId, MastNodeId) {
        let first = {
            let mut first_le_bytes = [0_u8; 4];

            first_le_bytes[0] = self.0[0] << 4;
            first_le_bytes[0] |= self.0[1] >> 4;

            first_le_bytes[1] = self.0[1] << 4;
            first_le_bytes[1] |= self.0[2] >> 4;

            first_le_bytes[2] = self.0[2] << 4;
            first_le_bytes[2] |= self.0[3] >> 4;

            first_le_bytes[3] = (self.0[3] & 0b1111) << 2;
            first_le_bytes[3] |= self.0[4] >> 6;

            u32::from_le_bytes(first_le_bytes)
        };

        let second = {
            let mut second_be_bytes = [0_u8; 4];

            second_be_bytes[0] = self.0[4] & 0b0011_1111;
            second_be_bytes[1] = self.0[5];
            second_be_bytes[2] = self.0[6];
            second_be_bytes[3] = self.0[7];

            u32::from_be_bytes(second_be_bytes)
        };

        (MastNodeId(first), MastNodeId(second))
    }

    fn encode_u32_payload(discriminant: u8, payload: u32) -> Self {
        let [payload_byte1, payload_byte2, payload_byte3, payload_byte4] = payload.to_be_bytes();

        Self([
            discriminant << 4,
            payload_byte1,
            payload_byte2,
            payload_byte3,
            payload_byte4,
            0,
            0,
            0,
        ])
    }

    fn decode_u32_payload(&self) -> u32 {
        let payload_be_bytes = [self.0[1], self.0[2], self.0[3], self.0[4]];

        u32::from_be_bytes(payload_be_bytes)
    }
}

impl Serializable for EncodedMastNodeType {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.0.write_into(target);
    }
}

impl Deserializable for EncodedMastNodeType {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let bytes = source.read_array()?;

        Ok(Self(bytes))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mast::{JoinNode, SplitNode};

    #[test]
    fn mast_node_type_serde_join() {
        let left_child_id = MastNodeId(0b00111001_11101011_01101100_11011000);
        let right_child_id = MastNodeId(0b00100111_10101010_11111111_11001110);
        let mast_node = MastNode::Join(JoinNode::new_test(
            [left_child_id, right_child_id],
            RpoDigest::default(),
        ));

        let mast_node_type = EncodedMastNodeType::new(&mast_node);

        // Note: Join's discriminant is 0
        let expected_mast_node_type = [
            0b00001101, 0b10000110, 0b11001110, 0b10111110, 0b01100111, 0b10101010, 0b11111111,
            0b11001110,
        ];

        assert_eq!(expected_mast_node_type, mast_node_type.0);

        let (decoded_left, decoded_right) = mast_node_type.decode_join_or_split();
        assert_eq!(left_child_id, decoded_left);
        assert_eq!(right_child_id, decoded_right);
    }

    #[test]
    fn mast_node_type_serde_split() {
        let on_true_id = MastNodeId(0b00111001_11101011_01101100_11011000);
        let on_false_id = MastNodeId(0b00100111_10101010_11111111_11001110);
        let mast_node =
            MastNode::Split(SplitNode::new_test([on_true_id, on_false_id], RpoDigest::default()));

        let mast_node_type = EncodedMastNodeType::new(&mast_node);

        // Note: Split's discriminant is 0
        let expected_mast_node_type = [
            0b00011101, 0b10000110, 0b11001110, 0b10111110, 0b01100111, 0b10101010, 0b11111111,
            0b11001110,
        ];

        assert_eq!(expected_mast_node_type, mast_node_type.0);

        let (decoded_on_true, decoded_on_false) = mast_node_type.decode_join_or_split();
        assert_eq!(on_true_id, decoded_on_true);
        assert_eq!(on_false_id, decoded_on_false);
    }

    // TODOP: Test all other variants
}
