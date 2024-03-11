use alloc::string::ToString;
use core::fmt;

use crate::{
    ast::{ImmU8, MAX_STACK_WORD_OFFSET},
    ByteReader, ByteWriter, Deserializable, DeserializationError, Felt, Serializable, Span, ZERO,
};
use vm_core::AdviceInjector;

const PUSH_U64DIV: u8 = 0;
const PUSH_EXT2INTT: u8 = 1;
const PUSH_SMTGET: u8 = 2;
const PUSH_SMTSET: u8 = 3;
const PUSH_SMTPEEK: u8 = 4;
const PUSH_MAPVAL: u8 = 5;
const PUSH_MAPVAL_IMM: u8 = 6;
const PUSH_MAPVALN: u8 = 7;
const PUSH_MAPVALN_IMM: u8 = 8;
const PUSH_MTNODE: u8 = 9;
const INSERT_MEM: u8 = 10;
const INSERT_HDWORD: u8 = 11;
const INSERT_HDWORD_IMM: u8 = 12;
const INSERT_HPERM: u8 = 13;
const PUSH_SIG: u8 = 14;

/// Instructions which inject data into the advice provider.
///
/// These instructions can be used to perform two broad sets of operations:
/// - Push new data onto the advice stack.
/// - Insert new data into the advice map.
#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum AdviceInjectorNode {
    PushU64Div = PUSH_U64DIV,
    PushExt2intt = PUSH_EXT2INTT,
    PushSmtGet = PUSH_SMTGET,
    PushSmtSet = PUSH_SMTSET,
    PushSmtPeek = PUSH_SMTPEEK,
    PushMapVal = PUSH_MAPVAL,
    PushMapValImm { offset: ImmU8 } = PUSH_MAPVAL_IMM,
    PushMapValN = PUSH_MAPVALN,
    PushMapValNImm { offset: ImmU8 } = PUSH_MAPVALN_IMM,
    PushMtNode = PUSH_MTNODE,
    InsertMem = INSERT_MEM,
    InsertHdword = INSERT_HDWORD,
    InsertHdwordImm { domain: ImmU8 } = INSERT_HDWORD_IMM,
    InsertHperm = INSERT_HPERM,
    PushSignature { kind: SignatureKind } = PUSH_SIG,
}

impl AdviceInjectorNode {
    fn tag(&self) -> u8 {
        // SAFETY: This is safe because we have given this enum a
        // primitive representation with #[repr(u8)], with the first
        // field of the underlying union-of-structs the discriminant
        //
        // See the section on "accessing the numeric value of the discriminant"
        // here: https://doc.rust-lang.org/std/mem/fn.discriminant.html
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl From<&AdviceInjectorNode> for AdviceInjector {
    fn from(value: &AdviceInjectorNode) -> Self {
        use AdviceInjectorNode::*;
        match value {
            PushU64Div => Self::U64Div,
            PushExt2intt => Self::Ext2Intt,
            PushSmtGet => Self::SmtGet,
            PushSmtSet => Self::SmtSet,
            PushSmtPeek => Self::SmtPeek,
            PushMapVal => Self::MapValueToStack {
                include_len: false,
                key_offset: 0,
            },
            PushMapValImm {
                offset: ImmU8::Value(offset),
            } => Self::MapValueToStack {
                include_len: false,
                key_offset: offset.into_inner() as usize,
            },
            PushMapValImm { offset } => panic!("unresolved constant '{offset}'"),
            PushMapValN => Self::MapValueToStack {
                include_len: true,
                key_offset: 0,
            },
            PushMapValNImm {
                offset: ImmU8::Value(offset),
            } => Self::MapValueToStack {
                include_len: true,
                key_offset: offset.into_inner() as usize,
            },
            PushMapValNImm { offset } => panic!("unresolved constant '{offset}'"),
            PushMtNode => Self::MerkleNodeToStack,
            InsertMem => Self::MemToMap,
            InsertHdword => Self::HdwordToMap { domain: ZERO },
            InsertHdwordImm {
                domain: ImmU8::Value(domain),
            } => Self::HdwordToMap {
                domain: Felt::from(domain.into_inner()),
            },
            InsertHdwordImm { domain } => panic!("unresolved constant '{domain}'"),
            InsertHperm => Self::HpermToMap,
            PushSignature { kind } => Self::SigToStack {
                kind: (*kind).into(),
            },
        }
    }
}

#[cfg(feature = "formatter")]
impl crate::prettier::PrettyPrint for AdviceInjectorNode {
    fn render(&self) -> crate::prettier::Document {
        crate::prettier::display(self)
    }
}

impl fmt::Display for AdviceInjectorNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PushU64Div => write!(f, "push_u64div"),
            Self::PushExt2intt => write!(f, "push_ext2intt"),
            Self::PushSmtGet => write!(f, "push_smtget"),
            Self::PushSmtSet => write!(f, "push_smtset"),
            Self::PushSmtPeek => write!(f, "push_smtpeek"),
            Self::PushMapVal => write!(f, "push_mapval"),
            Self::PushMapValImm { offset } => write!(f, "push_mapval.{offset}"),
            Self::PushMapValN => write!(f, "push_mapvaln"),
            Self::PushMapValNImm { offset } => write!(f, "push_mapvaln.{offset}"),
            Self::PushMtNode => write!(f, "push_mtnode"),
            Self::InsertMem => write!(f, "insert_mem"),
            Self::InsertHdword => write!(f, "insert_hdword"),
            Self::InsertHdwordImm { domain } => write!(f, "insert_hdword.{domain}"),
            Self::InsertHperm => writeln!(f, "insert_hperm"),
            Self::PushSignature { kind } => write!(f, "push_sig.{kind}"),
        }
    }
}

// SERIALIZATION / DESERIALIZATION
// ================================================================================================

impl Serializable for AdviceInjectorNode {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8(self.tag());
        match self {
            Self::PushU64Div
            | Self::PushExt2intt
            | Self::PushSmtGet
            | Self::PushSmtSet
            | Self::PushSmtPeek
            | Self::PushMapVal
            | Self::PushMapValN
            | Self::PushMtNode
            | Self::InsertMem
            | Self::InsertHdword
            | Self::InsertHperm => (),
            Self::PushMapValImm {
                offset: ImmU8::Value(offset),
            } => {
                target.write_u8(offset.into_inner());
            }
            Self::PushMapValImm { offset } => panic!("unresolved constant '{offset}'"),
            Self::PushMapValNImm {
                offset: ImmU8::Value(offset),
            } => {
                target.write_u8(offset.into_inner());
            }
            Self::PushMapValNImm { offset } => panic!("unresolved constant '{offset}'"),
            Self::InsertHdwordImm {
                domain: ImmU8::Value(domain),
            } => {
                target.write_u8(domain.into_inner());
            }
            Self::InsertHdwordImm { domain } => panic!("unresolved constant '{domain}'"),
            Self::PushSignature { kind } => kind.write_into(target),
        }
    }
}

impl Deserializable for AdviceInjectorNode {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        match source.read_u8()? {
            PUSH_U64DIV => Ok(Self::PushU64Div),
            PUSH_EXT2INTT => Ok(Self::PushExt2intt),
            PUSH_SMTGET => Ok(Self::PushSmtGet),
            PUSH_SMTSET => Ok(Self::PushSmtSet),
            PUSH_SMTPEEK => Ok(Self::PushSmtPeek),
            PUSH_MAPVAL => Ok(Self::PushMapVal),
            PUSH_MAPVAL_IMM => {
                let offset = source.read_u8()?;
                if offset > MAX_STACK_WORD_OFFSET {
                    return Err(DeserializationError::InvalidValue("invalid offset".to_string()));
                }
                Ok(Self::PushMapValImm {
                    offset: ImmU8::Value(Span::unknown(offset)),
                })
            }
            PUSH_MAPVALN => Ok(Self::PushMapValN),
            PUSH_MAPVALN_IMM => {
                let offset = source.read_u8()?;
                if offset > MAX_STACK_WORD_OFFSET {
                    return Err(DeserializationError::InvalidValue("invalid offset".to_string()));
                }
                Ok(Self::PushMapValNImm {
                    offset: ImmU8::Value(Span::unknown(offset)),
                })
            }
            PUSH_MTNODE => Ok(Self::PushMtNode),
            INSERT_MEM => Ok(Self::InsertMem),
            INSERT_HDWORD => Ok(Self::InsertHdword),
            INSERT_HDWORD_IMM => {
                let domain = source.read_u8()?;
                Ok(Self::InsertHdwordImm {
                    domain: ImmU8::Value(Span::unknown(domain)),
                })
            }
            INSERT_HPERM => Ok(Self::InsertHperm),
            PUSH_SIG => Ok(Self::PushSignature {
                kind: SignatureKind::read_from(source)?,
            }),
            val => Err(DeserializationError::InvalidValue(val.to_string())),
        }
    }
}

/// A newtype wrapper for [vm_core::SignatureKind]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum SignatureKind {
    RpoFalcon512 = 0,
}

impl From<SignatureKind> for vm_core::SignatureKind {
    fn from(kind: SignatureKind) -> Self {
        match kind {
            SignatureKind::RpoFalcon512 => Self::RpoFalcon512,
        }
    }
}

impl fmt::Display for SignatureKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind: vm_core::SignatureKind = (*self).into();
        write!(f, "{kind}")
    }
}

impl Serializable for SignatureKind {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8(*self as u8);
    }
}

impl Deserializable for SignatureKind {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        match source.read_u8()? {
            0 => Ok(Self::RpoFalcon512),
            val => Err(DeserializationError::InvalidValue(val.to_string())),
        }
    }
}
