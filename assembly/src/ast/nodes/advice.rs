use super::super::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};
use core::fmt;
use vm_core::{AdviceInjector, Felt};

// ADVICE INJECTORS
// ================================================================================================

/// Instructions which inject data into the advice provider.
///
/// These instructions can be used to perform two broad sets of operations:
/// - Push new data onto the advice stack.
/// - Insert new data into the advice map.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AdviceInjectorNode {
    PushU64div,
    PushExt2intt,
    PushSmtGet,
    PushMapVal,
    PushMtNode,
    InsertMem,
    InsertHdword { domain: u8 },
}

impl From<&AdviceInjectorNode> for AdviceInjector {
    fn from(value: &AdviceInjectorNode) -> Self {
        match value {
            AdviceInjectorNode::PushU64div => Self::DivU64,
            AdviceInjectorNode::PushExt2intt => Self::Ext2Intt,
            AdviceInjectorNode::PushSmtGet => Self::SmtGet,
            AdviceInjectorNode::PushMapVal => Self::MapValueToStack,
            AdviceInjectorNode::PushMtNode => Self::MerkleNodeToStack,
            AdviceInjectorNode::InsertMem => Self::MemToMap,
            AdviceInjectorNode::InsertHdword { domain } => Self::HdwordToMap {
                domain: Felt::from(*domain),
            },
        }
    }
}

impl fmt::Display for AdviceInjectorNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdviceInjectorNode::PushU64div => write!(f, "push_u64div"),
            AdviceInjectorNode::PushExt2intt => write!(f, "push_ext2intt"),
            AdviceInjectorNode::PushSmtGet => write!(f, "push_smtget"),
            AdviceInjectorNode::PushMapVal => write!(f, "push_mapval"),
            AdviceInjectorNode::PushMtNode => write!(f, "push_mtnode"),
            AdviceInjectorNode::InsertMem => write!(f, "insert_mem"),
            AdviceInjectorNode::InsertHdword { domain } => match domain {
                0 => write!(f, "insert_hdword"),
                _ => write!(f, "insert_hdword.{domain}"),
            },
        }
    }
}

// SERIALIZATION / DESERIALIZATION
// ================================================================================================

const PUSH_U64DIV: u8 = 0;
const PUSH_EXT2INTT: u8 = 1;
const PUSH_SMTGET: u8 = 2;
const PUSH_MAPVAL: u8 = 3;
const PUSH_MTNODE: u8 = 4;
const INSERT_MEM: u8 = 5;
const INSERT_HDWORD: u8 = 6;

impl Serializable for AdviceInjectorNode {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        match self {
            AdviceInjectorNode::PushU64div => target.write_u8(PUSH_U64DIV),
            AdviceInjectorNode::PushExt2intt => target.write_u8(PUSH_EXT2INTT),
            AdviceInjectorNode::PushSmtGet => target.write_u8(PUSH_SMTGET),
            AdviceInjectorNode::PushMapVal => target.write_u8(PUSH_MAPVAL),
            AdviceInjectorNode::PushMtNode => target.write_u8(PUSH_MTNODE),
            AdviceInjectorNode::InsertMem => target.write_u8(INSERT_MEM),
            AdviceInjectorNode::InsertHdword { domain } => {
                target.write_u8(INSERT_HDWORD);
                target.write_u8(*domain);
            }
        }
    }
}

impl Deserializable for AdviceInjectorNode {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        match source.read_u8()? {
            PUSH_U64DIV => Ok(AdviceInjectorNode::PushU64div),
            PUSH_EXT2INTT => Ok(AdviceInjectorNode::PushExt2intt),
            PUSH_SMTGET => Ok(AdviceInjectorNode::PushSmtGet),
            PUSH_MAPVAL => Ok(AdviceInjectorNode::PushMapVal),
            PUSH_MTNODE => Ok(AdviceInjectorNode::PushMtNode),
            INSERT_MEM => Ok(AdviceInjectorNode::InsertMem),
            INSERT_HDWORD => {
                let domain = source.read_u8()?;
                Ok(AdviceInjectorNode::InsertHdword { domain })
            }
            val => Err(DeserializationError::InvalidValue(val.to_string())),
        }
    }
}
