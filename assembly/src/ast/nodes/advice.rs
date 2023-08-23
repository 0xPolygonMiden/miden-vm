use super::super::{
    ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable, ToString,
    MAX_STACK_WORD_OFFSET,
};
use core::fmt;
use vm_core::{AdviceInjector, Felt, StarkField, Word, ZERO};

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
    PushSmtInsert,
    PushMapVal,
    PushMapValImm { offset: u8 },
    PushMapValN,
    PushMapValNImm { offset: u8 },
    PushMapValC { key: Word },
    PushMapValNC { key: Word },
    PushMapValM { addr: Felt },
    PushMapValNM { addr: Felt },
    PushMtNode,
    InsertMem,
    InsertHdword,
    InsertHdwordImm { domain: u8 },
    InsertHperm,
}

impl From<&AdviceInjectorNode> for AdviceInjector {
    fn from(value: &AdviceInjectorNode) -> Self {
        use AdviceInjectorNode::*;
        match value {
            PushU64div => Self::DivU64,
            PushExt2intt => Self::Ext2Intt,
            PushSmtGet => Self::SmtGet,
            PushSmtInsert => Self::SmtInsert,
            PushMapVal => Self::MapValueToStack {
                include_len: false,
                key_offset: 0,
            },
            PushMapValImm { offset } => Self::MapValueToStack {
                include_len: false,
                key_offset: (*offset) as usize,
            },
            PushMapValN => Self::MapValueToStack {
                include_len: true,
                key_offset: 0,
            },
            PushMapValNImm { offset } => Self::MapValueToStack {
                include_len: true,
                key_offset: (*offset) as usize,
            },
            PushMapValC { key } => Self::MapValueToStackConst {
                include_len: false,
                key: *key,
            },
            PushMapValNC { key } => Self::MapValueToStackConst {
                include_len: true,
                key: *key,
            },
            PushMapValM { addr } => Self::MapValueToStackMem {
                include_len: false,
                addr: *addr,
            },
            PushMapValNM { addr } => Self::MapValueToStackMem {
                include_len: true,
                addr: *addr,
            },

            PushMtNode => Self::MerkleNodeToStack,
            InsertMem => Self::MemToMap,
            InsertHdword => Self::HdwordToMap { domain: ZERO },
            InsertHdwordImm { domain } => Self::HdwordToMap {
                domain: Felt::from(*domain),
            },
            InsertHperm => Self::HpermToMap,
        }
    }
}

impl fmt::Display for AdviceInjectorNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AdviceInjectorNode::*;
        match self {
            PushU64div => write!(f, "push_u64div"),
            PushExt2intt => write!(f, "push_ext2intt"),
            PushSmtGet => write!(f, "push_smtget"),
            PushSmtInsert => write!(f, "push_smtinsert"),
            PushMapVal => write!(f, "push_mapval"),
            PushMapValImm { offset } => write!(f, "push_mapval.{offset}"),
            PushMapValN => write!(f, "push_mapvaln"),
            PushMapValNImm { offset } => write!(f, "push_mapvaln.{offset}"),
            PushMapValC { key } => {
                write!(f, "push_mapvalc.{}.{}.{}.{}", key[0], key[1], key[2], key[3])
            }
            PushMapValNC { key } => {
                write!(f, "push_mapvalnc.{}.{}.{}.{}", key[0], key[1], key[2], key[3])
            }
            PushMapValM { addr } => write!(f, "push_mapvalm.{}", addr),
            PushMapValNM { addr } => write!(f, "push_mapvalnm.{}", addr),
            PushMtNode => write!(f, "push_mtnode"),
            InsertMem => write!(f, "insert_mem"),
            InsertHdword => write!(f, "insert_hdword"),
            InsertHdwordImm { domain } => write!(f, "insert_hdword.{domain}"),
            InsertHperm => writeln!(f, "insert_hperm"),
        }
    }
}

// SERIALIZATION / DESERIALIZATION
// ================================================================================================

const PUSH_U64DIV: u8 = 0;
const PUSH_EXT2INTT: u8 = 1;
const PUSH_SMTGET: u8 = 2;
const PUSH_SMTINSERT: u8 = 3;
const PUSH_MAPVAL: u8 = 4;
const PUSH_MAPVAL_IMM: u8 = 5;
const PUSH_MAPVALN: u8 = 6;
const PUSH_MAPVALN_IMM: u8 = 7;
const PUSH_MAPVAL_CONST: u8 = 8;
const PUSH_MAPVALN_CONST: u8 = 9;
const PUSH_MAPVAL_MEM: u8 = 10;
const PUSH_MAPVALN_MEM: u8 = 11;
const PUSH_MTNODE: u8 = 12;
const INSERT_MEM: u8 = 13;
const INSERT_HDWORD: u8 = 14;
const INSERT_HDWORD_IMM: u8 = 15;
const INSERT_HPERM: u8 = 16;

impl Serializable for AdviceInjectorNode {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        use AdviceInjectorNode::*;
        match self {
            PushU64div => target.write_u8(PUSH_U64DIV),
            PushExt2intt => target.write_u8(PUSH_EXT2INTT),
            PushSmtGet => target.write_u8(PUSH_SMTGET),
            PushSmtInsert => target.write_u8(PUSH_SMTINSERT),
            PushMapVal => target.write_u8(PUSH_MAPVAL),
            PushMapValImm { offset } => {
                target.write_u8(PUSH_MAPVAL_IMM);
                target.write_u8(*offset);
            }
            PushMapValN => target.write_u8(PUSH_MAPVALN),
            PushMapValNImm { offset } => {
                target.write_u8(PUSH_MAPVALN_IMM);
                target.write_u8(*offset);
            }
            PushMapValC { key } => {
                target.write_u8(PUSH_MAPVAL_CONST);
                target.write_u64(key[0].as_int());
                target.write_u64(key[1].as_int());
                target.write_u64(key[2].as_int());
                target.write_u64(key[3].as_int());
            }
            PushMapValNC { key } => {
                target.write_u8(PUSH_MAPVALN_CONST);
                target.write_u64(key[0].as_int());
                target.write_u64(key[1].as_int());
                target.write_u64(key[2].as_int());
                target.write_u64(key[3].as_int());
            }
            PushMapValM { addr } => {
                target.write_u8(PUSH_MAPVAL_MEM);
                target.write_u64(addr.as_int());
            }
            PushMapValNM { addr } => {
                target.write_u8(PUSH_MAPVALN_MEM);
                target.write_u64(addr.as_int());
            }
            PushMtNode => target.write_u8(PUSH_MTNODE),
            InsertMem => target.write_u8(INSERT_MEM),
            InsertHdword => target.write_u8(INSERT_HDWORD),
            InsertHdwordImm { domain } => {
                target.write_u8(INSERT_HDWORD_IMM);
                target.write_u8(*domain);
            }
            InsertHperm => target.write_u8(INSERT_HPERM),
        }
    }
}

impl Deserializable for AdviceInjectorNode {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        match source.read_u8()? {
            PUSH_U64DIV => Ok(AdviceInjectorNode::PushU64div),
            PUSH_EXT2INTT => Ok(AdviceInjectorNode::PushExt2intt),
            PUSH_SMTGET => Ok(AdviceInjectorNode::PushSmtGet),
            PUSH_SMTINSERT => Ok(AdviceInjectorNode::PushSmtInsert),
            PUSH_MAPVAL => Ok(AdviceInjectorNode::PushMapVal),
            PUSH_MAPVAL_IMM => {
                let offset = source.read_u8()?;
                if offset > MAX_STACK_WORD_OFFSET {
                    return Err(DeserializationError::InvalidValue("invalid offset".to_string()));
                }
                Ok(AdviceInjectorNode::PushMapValImm { offset })
            }
            PUSH_MAPVALN => Ok(AdviceInjectorNode::PushMapValN),
            PUSH_MAPVALN_IMM => {
                let offset = source.read_u8()?;
                if offset > MAX_STACK_WORD_OFFSET {
                    return Err(DeserializationError::InvalidValue("invalid offset".to_string()));
                }
                Ok(AdviceInjectorNode::PushMapValNImm { offset })
            }
            PUSH_MAPVAL_CONST => {
                let key = [
                    Felt::from(source.read_u64()?),
                    Felt::from(source.read_u64()?),
                    Felt::from(source.read_u64()?),
                    Felt::from(source.read_u64()?),
                ];
                Ok(AdviceInjectorNode::PushMapValC { key })
            }
            PUSH_MAPVALN_CONST => {
                let key = [
                    Felt::from(source.read_u64()?),
                    Felt::from(source.read_u64()?),
                    Felt::from(source.read_u64()?),
                    Felt::from(source.read_u64()?),
                ];
                Ok(AdviceInjectorNode::PushMapValNC { key })
            }
            PUSH_MAPVAL_MEM => {
                let addr = Felt::from(source.read_u64()?);
                Ok(AdviceInjectorNode::PushMapValM { addr })
            }
            PUSH_MAPVALN_MEM => {
                let addr = Felt::from(source.read_u64()?);
                Ok(AdviceInjectorNode::PushMapValNM { addr })
            }
            PUSH_MTNODE => Ok(AdviceInjectorNode::PushMtNode),
            INSERT_MEM => Ok(AdviceInjectorNode::InsertMem),
            INSERT_HDWORD => Ok(AdviceInjectorNode::InsertHdword),
            INSERT_HDWORD_IMM => {
                let domain = source.read_u8()?;
                Ok(AdviceInjectorNode::InsertHdwordImm { domain })
            }
            INSERT_HPERM => Ok(AdviceInjectorNode::InsertHperm),
            val => Err(DeserializationError::InvalidValue(val.to_string())),
        }
    }
}
