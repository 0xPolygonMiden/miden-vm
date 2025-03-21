use core::fmt;

use vm_core::sys_events::SystemEvent;

// SYSTEM EVENT NODE
// ================================================================================================

/// Instructions which inject data into the advice provider.
///
/// These instructions can be used to perform two broad sets of operations:
/// - Push new data onto the advice stack.
/// - Insert new data into the advice map.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SystemEventNode {
    PushU64Div,
    PushFalconDiv,
    PushExt2intt,
    PushSmtPeek,
    PushMapVal,
    PushMapValN,
    PushMtNode,
    InsertMem,
    InsertHdword,
    InsertHdwordWithDomain,
    InsertHperm,
}

impl From<&SystemEventNode> for SystemEvent {
    fn from(value: &SystemEventNode) -> Self {
        use SystemEventNode::*;
        match value {
            PushU64Div => Self::U64Div,
            PushFalconDiv => Self::FalconDiv,
            PushExt2intt => Self::Ext2Intt,
            PushSmtPeek => Self::SmtPeek,
            PushMapVal => Self::MapValueToStack,
            PushMapValN => Self::MapValueToStackN,
            PushMtNode => Self::MerkleNodeToStack,
            InsertMem => Self::MemToMap,
            InsertHdword => Self::HdwordToMap,
            InsertHdwordWithDomain => Self::HdwordToMapWithDomain,
            InsertHperm => Self::HpermToMap,
        }
    }
}

impl crate::prettier::PrettyPrint for SystemEventNode {
    fn render(&self) -> crate::prettier::Document {
        crate::prettier::display(self)
    }
}

impl fmt::Display for SystemEventNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PushU64Div => write!(f, "push_u64div"),
            Self::PushFalconDiv => write!(f, "push_falcon_div"),
            Self::PushExt2intt => write!(f, "push_ext2intt"),
            Self::PushSmtPeek => write!(f, "push_smtpeek"),
            Self::PushMapVal => write!(f, "push_mapval"),
            Self::PushMapValN => write!(f, "push_mapvaln"),
            Self::PushMtNode => write!(f, "push_mtnode"),
            Self::InsertMem => write!(f, "insert_mem"),
            Self::InsertHdword => write!(f, "insert_hdword"),
            Self::InsertHdwordWithDomain => write!(f, "insert_hdword_d"),
            Self::InsertHperm => writeln!(f, "insert_hperm"),
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
