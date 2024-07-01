use miden_crypto::hash::rpo::RpoDigest;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use winter_utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

use crate::mast::{MastNode, MastNodeId};

use super::DataOffset;

pub struct MastNodeInfo {
    pub(super) ty: EncodedMastNodeType,
    pub(super) offset: DataOffset,
    pub(super) digest: RpoDigest,
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
pub struct EncodedMastNodeType(pub(super) [u8; 8]);

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
    pub fn variant(&self) -> Result<MastNodeTypeVariant, super::Error> {
        let discriminant = self.0[0] >> 4;

        MastNodeTypeVariant::try_from_discriminant(discriminant)
    }
}

/// Helpers
impl EncodedMastNodeType {
    // TODOP: Make a diagram of how the bits are split
    pub fn encode_join_or_split(
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

    pub fn decode_join_or_split(&self) -> (MastNodeId, MastNodeId) {
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

    pub fn encode_u32_payload(discriminant: u8, payload: u32) -> Self {
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

    pub fn decode_u32_payload(&self) -> u32 {
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

    pub fn try_from_discriminant(discriminant: u8) -> Result<Self, super::Error> {
        Self::from_u8(discriminant).ok_or_else(|| super::Error::InvalidDiscriminant {
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
