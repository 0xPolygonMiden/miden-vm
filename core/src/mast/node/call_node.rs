use core::fmt;

use miden_crypto::{hash::rpo::RpoDigest, Felt};
use miden_formatting::prettier::PrettyPrint;

use crate::{
    chiplets::hasher,
    mast::{MastForest, MastNodeId, MerkleTreeNode},
    OPCODE_CALL, OPCODE_SYSCALL,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallNode {
    callee: MastNodeId,
    is_syscall: bool,
    digest: RpoDigest,
}

/// Constants
impl CallNode {
    /// The domain of the call block (used for control block hashing).
    pub const CALL_DOMAIN: Felt = Felt::new(OPCODE_CALL as u64);
    /// The domain of the syscall block (used for control block hashing).
    pub const SYSCALL_DOMAIN: Felt = Felt::new(OPCODE_SYSCALL as u64);
}

/// Constructors
impl CallNode {
    /// Returns a new [`CallNode`] instantiated with the specified callee.
    pub fn new(callee: MastNodeId, mast_forest: &MastForest) -> Self {
        let digest = {
            let callee_digest = mast_forest[callee].digest();

            hasher::merge_in_domain(&[callee_digest, RpoDigest::default()], Self::CALL_DOMAIN)
        };

        Self {
            callee,
            is_syscall: false,
            digest,
        }
    }

    /// Returns a new [`CallNode`] instantiated with the specified callee and marked as a kernel
    /// call.
    pub fn new_syscall(callee: MastNodeId, mast_forest: &MastForest) -> Self {
        let digest = {
            let callee_digest = mast_forest[callee].digest();

            hasher::merge_in_domain(&[callee_digest, RpoDigest::default()], Self::SYSCALL_DOMAIN)
        };

        Self {
            callee,
            is_syscall: true,
            digest,
        }
    }
}

impl CallNode {
    pub fn callee(&self) -> MastNodeId {
        self.callee
    }

    pub fn is_syscall(&self) -> bool {
        self.is_syscall
    }

    /// Returns the domain of the call node.
    pub fn domain(&self) -> Felt {
        if self.is_syscall() {
            Self::SYSCALL_DOMAIN
        } else {
            Self::CALL_DOMAIN
        }
    }

    pub(super) fn to_pretty_print<'a>(
        &'a self,
        mast_forest: &'a MastForest,
    ) -> impl PrettyPrint + 'a {
        CallNodePrettyPrint {
            call_node: self,
            mast_forest,
        }
    }
}

impl MerkleTreeNode for CallNode {
    fn digest(&self) -> RpoDigest {
        self.digest
    }

    fn to_display<'a>(&'a self, mast_forest: &'a MastForest) -> impl fmt::Display + 'a {
        CallNodePrettyPrint {
            call_node: self,
            mast_forest,
        }
    }
}

struct CallNodePrettyPrint<'a> {
    call_node: &'a CallNode,
    mast_forest: &'a MastForest,
}

impl<'a> PrettyPrint for CallNodePrettyPrint<'a> {
    #[rustfmt::skip]
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;
        use miden_formatting::hex::ToHex;

        let callee_digest = self.mast_forest[self.call_node.callee].digest();

        let doc = if self.call_node.is_syscall {
            const_text("syscall")
        } else {
            const_text("call")
        };
        doc + const_text(".") + text(callee_digest.as_bytes().to_hex_with_prefix())
    }
}

impl<'a> fmt::Display for CallNodePrettyPrint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::prettier::PrettyPrint;
        self.pretty_print(f)
    }
}
