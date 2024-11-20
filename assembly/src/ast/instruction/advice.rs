use core::fmt;

use vm_core::AdviceInjector;

use crate::{ast::ImmU8, Felt, ZERO};

// ADVICE INJECTOR NODE
// ================================================================================================

/// Instructions which inject data into the advice provider.
///
/// These instructions can be used to perform two broad sets of operations:
/// - Push new data onto the advice stack.
/// - Insert new data into the advice map.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AdviceInjectorNode {
    PushU64Div,
    PushExt2intt,
    PushSmtPeek,
    PushMapVal,
    PushMapValN,
    PushMtNode,
    InsertMem,
    InsertHdword,
    InsertHdwordImm { domain: ImmU8 },
    InsertHperm,
    PushSignature { kind: SignatureKind },
}

impl From<&AdviceInjectorNode> for AdviceInjector {
    fn from(value: &AdviceInjectorNode) -> Self {
        use AdviceInjectorNode::*;
        match value {
            PushU64Div => Self::U64Div,
            PushExt2intt => Self::Ext2Intt,
            PushSmtPeek => Self::SmtPeek,
            PushMapVal => Self::MapValueToStack { include_len: false },
            PushMapValN => Self::MapValueToStack { include_len: true },
            PushMtNode => Self::MerkleNodeToStack,
            InsertMem => Self::MemToMap,
            InsertHdword => Self::HdwordToMap { domain: ZERO },
            InsertHdwordImm { domain: ImmU8::Value(domain) } => {
                Self::HdwordToMap { domain: Felt::from(domain.into_inner()) }
            },
            InsertHdwordImm { domain } => panic!("unresolved constant '{domain}'"),
            InsertHperm => Self::HpermToMap,
            PushSignature { kind } => match kind {
                SignatureKind::RpoFalcon512 => Self::FalconSigToStack,
            },
        }
    }
}

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
            Self::PushSmtPeek => write!(f, "push_smtpeek"),
            Self::PushMapVal => write!(f, "push_mapval"),
            Self::PushMapValN => write!(f, "push_mapvaln"),
            Self::PushMtNode => write!(f, "push_mtnode"),
            Self::InsertMem => write!(f, "insert_mem"),
            Self::InsertHdword => write!(f, "insert_hdword"),
            Self::InsertHdwordImm { domain } => write!(f, "insert_hdword.{domain}"),
            Self::InsertHperm => writeln!(f, "insert_hperm"),
            Self::PushSignature { kind } => write!(f, "push_sig.{kind}"),
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
