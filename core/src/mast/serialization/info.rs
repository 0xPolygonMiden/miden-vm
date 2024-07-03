use miden_crypto::hash::rpo::RpoDigest;
use winter_utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

use crate::mast::{MastNode, MerkleTreeNode};

use super::DataOffset;

// MAST NODE INFO
// ===============================================================================================

#[derive(Debug)]
pub struct MastNodeInfo {
    // TODOP: Remove pub(super)?
    pub(super) ty: MastNodeType,
    pub(super) offset: DataOffset,
    pub(super) digest: RpoDigest,
}

impl MastNodeInfo {
    pub fn new(mast_node: &MastNode, basic_block_offset: DataOffset) -> Self {
        let ty = MastNodeType::new(mast_node);

        let offset = if let MastNode::Block(_) = mast_node {
            basic_block_offset
        } else {
            0
        };

        Self {
            ty,
            offset,
            digest: mast_node.digest(),
        }
    }
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

// MAST NODE TYPE
// ===============================================================================================

const JOIN: u8 = 0;
const SPLIT: u8 = 1;
const LOOP: u8 = 2;
const BLOCK: u8 = 3;
const CALL: u8 = 4;
const SYSCALL: u8 = 5;
const DYN: u8 = 6;
const EXTERNAL: u8 = 7;

/// TODOP: Document the fact that encoded representation is always 8 bytes
#[derive(Debug)]
#[repr(u8)]
pub enum MastNodeType {
    Join {
        left_child_id: u32,
        right_child_id: u32,
    } = JOIN,
    Split {
        if_branch_id: u32,
        else_branch_id: u32,
    } = SPLIT,
    Loop {
        body_id: u32,
    } = LOOP,
    Block {
        /// The number of operations and decorators in the basic block
        len: u32,
    } = BLOCK,
    Call {
        callee_id: u32,
    } = CALL,
    SysCall {
        callee_id: u32,
    } = SYSCALL,
    Dyn = DYN,
    External = EXTERNAL,
}

/// Constructors
impl MastNodeType {
    pub fn new(mast_node: &MastNode) -> Self {
        use MastNode::*;

        match mast_node {
            Block(block_node) => {
                let len = block_node.num_operations_and_decorators();

                Self::Block { len }
            }
            Join(join_node) => Self::Join {
                left_child_id: join_node.first().0,
                right_child_id: join_node.second().0,
            },
            Split(split_node) => Self::Split {
                if_branch_id: split_node.on_true().0,
                else_branch_id: split_node.on_false().0,
            },
            Loop(loop_node) => Self::Loop {
                body_id: loop_node.body().0,
            },
            Call(call_node) => {
                if call_node.is_syscall() {
                    Self::SysCall {
                        callee_id: call_node.callee().0,
                    }
                } else {
                    Self::Call {
                        callee_id: call_node.callee().0,
                    }
                }
            }
            Dyn => Self::Dyn,
            External(_) => Self::External,
        }
    }
}

impl Serializable for MastNodeType {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let serialized_bytes = {
            let mut serialized_bytes = self.inline_data_to_bytes();

            // Tag is always placed in the first four bytes
            let tag = self.tag();
            assert!(tag <= 0b1111);
            serialized_bytes[0] |= tag << 4;

            serialized_bytes
        };

        serialized_bytes.write_into(target)
    }
}

/// Serialization helpers
impl MastNodeType {
    fn tag(&self) -> u8 {
        // SAFETY: This is safe because we have given this enum a primitive representation with
        // #[repr(u8)], with the first field of the underlying union-of-structs the discriminant.
        //
        // See the section on "accessing the numeric value of the discriminant"
        // here: https://doc.rust-lang.org/std/mem/fn.discriminant.html
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    fn inline_data_to_bytes(&self) -> [u8; 8] {
        match self {
            MastNodeType::Join {
                left_child_id: left,
                right_child_id: right,
            } => Self::encode_join_or_split(*left, *right),
            MastNodeType::Split {
                if_branch_id: if_branch,
                else_branch_id: else_branch,
            } => Self::encode_join_or_split(*if_branch, *else_branch),
            MastNodeType::Loop { body_id: body } => Self::encode_u32_payload(*body),
            MastNodeType::Block { len } => Self::encode_u32_payload(*len),
            MastNodeType::Call { callee_id } => Self::encode_u32_payload(*callee_id),
            MastNodeType::SysCall { callee_id } => Self::encode_u32_payload(*callee_id),
            MastNodeType::Dyn => [0; 8],
            MastNodeType::External => [0; 8],
        }
    }

    // TODOP: Make a diagram of how the bits are split
    fn encode_join_or_split(left_child_id: u32, right_child_id: u32) -> [u8; 8] {
        assert!(left_child_id < 2_u32.pow(30));
        assert!(right_child_id < 2_u32.pow(30));

        let mut result: [u8; 8] = [0_u8; 8];

        // write left child into result
        {
            let [lsb, a, b, msb] = left_child_id.to_le_bytes();
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
            let [msb, a, b, lsb] = right_child_id.to_be_bytes();

            result[4] |= msb;
            result[5] = a;
            result[6] = b;
            result[7] = lsb;
        };

        result
    }

    fn encode_u32_payload(payload: u32) -> [u8; 8] {
        let [payload_byte1, payload_byte2, payload_byte3, payload_byte4] = payload.to_be_bytes();

        [0, payload_byte1, payload_byte2, payload_byte3, payload_byte4, 0, 0, 0]
    }
}

impl Deserializable for MastNodeType {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let bytes: [u8; 8] = source.read_array()?;

        let tag = bytes[0] >> 4;

        match tag {
            JOIN => {
                let (left_child_id, right_child_id) = Self::decode_join_or_split(bytes);
                Ok(Self::Join {
                    left_child_id,
                    right_child_id,
                })
            }
            SPLIT => {
                let (if_branch_id, else_branch_id) = Self::decode_join_or_split(bytes);
                Ok(Self::Split {
                    if_branch_id,
                    else_branch_id,
                })
            }
            LOOP => {
                let body_id = Self::decode_u32_payload(bytes);
                Ok(Self::Loop { body_id })
            }
            BLOCK => {
                let len = Self::decode_u32_payload(bytes);
                Ok(Self::Block { len })
            }
            CALL => {
                let callee_id = Self::decode_u32_payload(bytes);
                Ok(Self::Call { callee_id })
            }
            SYSCALL => {
                let callee_id = Self::decode_u32_payload(bytes);
                Ok(Self::SysCall { callee_id })
            }
            DYN => Ok(Self::Dyn),
            EXTERNAL => Ok(Self::External),
            _ => {
                Err(DeserializationError::InvalidValue(format!("Invalid tag for MAST node: {tag}")))
            }
        }
    }
}

/// Deserialization helpers
impl MastNodeType {
    fn decode_join_or_split(buffer: [u8; 8]) -> (u32, u32) {
        let first = {
            let mut first_le_bytes = [0_u8; 4];

            first_le_bytes[0] = buffer[0] << 4;
            first_le_bytes[0] |= buffer[1] >> 4;

            first_le_bytes[1] = buffer[1] << 4;
            first_le_bytes[1] |= buffer[2] >> 4;

            first_le_bytes[2] = buffer[2] << 4;
            first_le_bytes[2] |= buffer[3] >> 4;

            first_le_bytes[3] = (buffer[3] & 0b1111) << 2;
            first_le_bytes[3] |= buffer[4] >> 6;

            u32::from_le_bytes(first_le_bytes)
        };

        let second = {
            let mut second_be_bytes = [0_u8; 4];

            second_be_bytes[0] = buffer[4] & 0b0011_1111;
            second_be_bytes[1] = buffer[5];
            second_be_bytes[2] = buffer[6];
            second_be_bytes[3] = buffer[7];

            u32::from_be_bytes(second_be_bytes)
        };

        (first, second)
    }

    pub fn decode_u32_payload(payload: [u8; 8]) -> u32 {
        let payload_be_bytes = [payload[1], payload[2], payload[3], payload[4]];

        u32::from_be_bytes(payload_be_bytes)
    }
}

// TESTS
// ===============================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn mast_node_type_serde_join() {
        let left_child_id = 0b00111001_11101011_01101100_11011000;
        let right_child_id = 0b00100111_10101010_11111111_11001110;

        let mast_node_type = MastNodeType::Join {
            left_child_id,
            right_child_id,
        };

        let mut encoded_mast_node_type: Vec<u8> = Vec::new();
        mast_node_type.write_into(&mut encoded_mast_node_type);

        // Note: Join's discriminant is 0
        let expected_encoded_mast_node_type = [
            0b00001101, 0b10000110, 0b11001110, 0b10111110, 0b01100111, 0b10101010, 0b11111111,
            0b11001110,
        ];

        assert_eq!(expected_encoded_mast_node_type.to_vec(), encoded_mast_node_type);

        let (decoded_left, decoded_right) =
            MastNodeType::decode_join_or_split(expected_encoded_mast_node_type);
        assert_eq!(left_child_id, decoded_left);
        assert_eq!(right_child_id, decoded_right);
    }

    #[test]
    fn mast_node_type_serde_split() {
        let if_branch_id = 0b00111001_11101011_01101100_11011000;
        let else_branch_id = 0b00100111_10101010_11111111_11001110;

        let mast_node_type = MastNodeType::Split {
            if_branch_id,
            else_branch_id,
        };

        let mut encoded_mast_node_type: Vec<u8> = Vec::new();
        mast_node_type.write_into(&mut encoded_mast_node_type);

        // Note: Split's discriminant is 1
        let expected_encoded_mast_node_type = [
            0b00011101, 0b10000110, 0b11001110, 0b10111110, 0b01100111, 0b10101010, 0b11111111,
            0b11001110,
        ];

        assert_eq!(expected_encoded_mast_node_type.to_vec(), encoded_mast_node_type);

        let (decoded_if_branch, decoded_else_branch) =
            MastNodeType::decode_join_or_split(expected_encoded_mast_node_type);
        assert_eq!(if_branch_id, decoded_if_branch);
        assert_eq!(else_branch_id, decoded_else_branch);
    }

    // TODOP: Test all other variants
}
