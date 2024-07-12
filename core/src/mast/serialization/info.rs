use miden_crypto::hash::rpo::RpoDigest;
use winter_utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

use crate::mast::{MastForest, MastNode, MastNodeId, MerkleTreeNode};

use super::{basic_block_data_decoder::BasicBlockDataDecoder, DataOffset};

// MAST NODE INFO
// ================================================================================================

/// Represents a serialized [`MastNode`], with some data inlined in its [`MastNodeType`].
///
/// The serialized representation of [`MastNodeInfo`] is guaranteed to be fixed width, so that the
/// nodes stored in the `nodes` table of the serialized [`MastForest`] can be accessed quickly by
/// index.
#[derive(Debug)]
pub struct MastNodeInfo {
    ty: MastNodeType,
    digest: RpoDigest,
}

impl MastNodeInfo {
    pub fn new(mast_node: &MastNode, basic_block_offset: DataOffset) -> Self {
        let ty = MastNodeType::new(mast_node, basic_block_offset);

        Self {
            ty,
            digest: mast_node.digest(),
        }
    }

    pub fn try_into_mast_node(
        self,
        mast_forest: &MastForest,
        basic_block_data_decoder: &BasicBlockDataDecoder,
    ) -> Result<MastNode, DeserializationError> {
        let mast_node = match self.ty {
            MastNodeType::Block {
                offset,
                len: num_operations_and_decorators,
            } => {
                let (operations, decorators) = basic_block_data_decoder
                    .decode_operations_and_decorators(offset, num_operations_and_decorators)?;

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
            MastNodeType::External => Ok(MastNode::new_external(self.digest)),
        }?;

        if mast_node.digest() == self.digest {
            Ok(mast_node)
        } else {
            Err(DeserializationError::InvalidValue(format!(
                "MastNodeInfo's digest '{}' doesn't match deserialized MastNode's digest '{}'",
                self.digest,
                mast_node.digest()
            )))
        }
    }
}

impl Serializable for MastNodeInfo {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let Self { ty, digest } = self;

        ty.write_into(target);
        digest.write_into(target);
    }
}

impl Deserializable for MastNodeInfo {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let ty = Deserializable::read_from(source)?;
        let digest = RpoDigest::read_from(source)?;

        Ok(Self { ty, digest })
    }
}

// MAST NODE TYPE
// ================================================================================================

const JOIN: u8 = 0;
const SPLIT: u8 = 1;
const LOOP: u8 = 2;
const BLOCK: u8 = 3;
const CALL: u8 = 4;
const SYSCALL: u8 = 5;
const DYN: u8 = 6;
const EXTERNAL: u8 = 7;

/// Represents the variant of a [`MastNode`], as well as any additional data. For example, for more
/// efficient decoding, and because of the frequency with which these node types appear, we directly
/// represent the child indices for `Join`, `Split`, and `Loop`, `Call` and `SysCall` inline.
///
/// The serialized representation of the MAST node type is guaranteed to be 8 bytes, so that
/// [`MastNodeInfo`] (which contains it) can be of fixed width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        /// Offset of the basic block in the data segment
        offset: u32,
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
    /// Constructs a new [`MastNodeType`] from a [`MastNode`].
    pub fn new(mast_node: &MastNode, basic_block_offset: u32) -> Self {
        use MastNode::*;

        match mast_node {
            Block(block_node) => {
                let len = block_node.num_operations_and_decorators();

                Self::Block {
                    len,
                    offset: basic_block_offset,
                }
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
        let discriminant = self.discriminant() as u64;
        assert!(discriminant <= 0b1111);

        let payload = match *self {
            MastNodeType::Join {
                left_child_id: left,
                right_child_id: right,
            } => Self::encode_u32_pair(left, right),
            MastNodeType::Split {
                if_branch_id: if_branch,
                else_branch_id: else_branch,
            } => Self::encode_u32_pair(if_branch, else_branch),
            MastNodeType::Loop { body_id: body } => Self::encode_u32_payload(body),
            MastNodeType::Block { offset, len } => Self::encode_u32_pair(offset, len),
            MastNodeType::Call { callee_id } => Self::encode_u32_payload(callee_id),
            MastNodeType::SysCall { callee_id } => Self::encode_u32_payload(callee_id),
            MastNodeType::Dyn => 0,
            MastNodeType::External => 0,
        };

        let value = (discriminant << 60) | payload;
        target.write_u64(value);
    }
}

/// Serialization helpers
impl MastNodeType {
    fn discriminant(&self) -> u8 {
        // SAFETY: This is safe because we have given this enum a primitive representation with
        // #[repr(u8)], with the first field of the underlying union-of-structs the discriminant.
        //
        // See the section on "accessing the numeric value of the discriminant"
        // here: https://doc.rust-lang.org/std/mem/fn.discriminant.html
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    /// Encodes two u32 numbers in the first 60 bits of a `u64`.
    ///
    /// # Panics
    /// - Panics if either `left_value` or `right_value` doesn't fit in 30 bits.
    fn encode_u32_pair(left_value: u32, right_value: u32) -> u64 {
        assert!(
            left_value.leading_zeros() < 2,
            "MastNodeType::encode_u32_pair: left value doesn't fit in 30 bits: {}",
            left_value
        );
        assert!(
            right_value.leading_zeros() < 2,
            "MastNodeType::encode_u32_pair: right value doesn't fit in 30 bits: {}",
            right_value
        );

        ((left_value as u64) << 30) | (right_value as u64)
    }

    fn encode_u32_payload(payload: u32) -> u64 {
        payload as u64
    }
}

impl Deserializable for MastNodeType {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let (discriminant, payload) = {
            let value = source.read_u64()?;

            // 4 bits
            let discriminant = (value >> 60) as u8;
            // 60 bits
            let payload = value & 0x0F_FF_FF_FF_FF_FF_FF_FF;

            (discriminant, payload)
        };

        match discriminant {
            JOIN => {
                let (left_child_id, right_child_id) = Self::decode_u32_pair(payload);
                Ok(Self::Join {
                    left_child_id,
                    right_child_id,
                })
            }
            SPLIT => {
                let (if_branch_id, else_branch_id) = Self::decode_u32_pair(payload);
                Ok(Self::Split {
                    if_branch_id,
                    else_branch_id,
                })
            }
            LOOP => {
                let body_id = Self::decode_u32_payload(payload)?;
                Ok(Self::Loop { body_id })
            }
            BLOCK => {
                let (offset, len) = Self::decode_u32_pair(payload);
                Ok(Self::Block { offset, len })
            }
            CALL => {
                let callee_id = Self::decode_u32_payload(payload)?;
                Ok(Self::Call { callee_id })
            }
            SYSCALL => {
                let callee_id = Self::decode_u32_payload(payload)?;
                Ok(Self::SysCall { callee_id })
            }
            DYN => Ok(Self::Dyn),
            EXTERNAL => Ok(Self::External),
            _ => Err(DeserializationError::InvalidValue(format!(
                "Invalid tag for MAST node: {discriminant}"
            ))),
        }
    }
}

/// Deserialization helpers
impl MastNodeType {
    /// Decodes two `u32` numbers from a 60-bit payload.
    fn decode_u32_pair(payload: u64) -> (u32, u32) {
        let left_value = (payload >> 30) as u32;
        let right_value = (payload & 0x3F_FF_FF_FF) as u32;

        (left_value, right_value)
    }

    /// Decodes one `u32` number from a 60-bit payload.
    ///
    /// Returns an error if the payload doesn't fit in a `u32`.
    pub fn decode_u32_payload(payload: u64) -> Result<u32, DeserializationError> {
        payload.try_into().map_err(|_| {
            DeserializationError::InvalidValue(format!(
                "Invalid payload: expected to fit in u32, but was {payload}"
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use super::*;

    #[test]
    fn serialize_deserialize_60_bit_payload() {
        // each child needs 30 bits
        let mast_node_type = MastNodeType::Join {
            left_child_id: 0x3F_FF_FF_FF,
            right_child_id: 0x3F_FF_FF_FF,
        };

        let serialized = mast_node_type.to_bytes();
        let deserialized = MastNodeType::read_from_bytes(&serialized).unwrap();

        assert_eq!(mast_node_type, deserialized);
    }

    #[test]
    #[should_panic]
    fn serialize_large_payloads_fails_1() {
        // left child needs 31 bits
        let mast_node_type = MastNodeType::Join {
            left_child_id: 0x4F_FF_FF_FF,
            right_child_id: 0x0,
        };

        // must panic
        let _serialized = mast_node_type.to_bytes();
    }

    #[test]
    #[should_panic]
    fn serialize_large_payloads_fails_2() {
        // right child needs 31 bits
        let mast_node_type = MastNodeType::Join {
            left_child_id: 0x0,
            right_child_id: 0x4F_FF_FF_FF,
        };

        // must panic
        let _serialized = mast_node_type.to_bytes();
    }

    #[test]
    fn deserialize_large_payloads_fails() {
        // Serialized `CALL` with a 33-bit payload
        let serialized = {
            let serialized_value = ((CALL as u64) << 60) | (u32::MAX as u64 + 1_u64);

            let mut serialized_buffer: Vec<u8> = Vec::new();
            serialized_value.write_into(&mut serialized_buffer);

            serialized_buffer
        };

        let deserialized_result = MastNodeType::read_from_bytes(&serialized);

        assert_matches!(deserialized_result, Err(DeserializationError::InvalidValue(_)));
    }
}
