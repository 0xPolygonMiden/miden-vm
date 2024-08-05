use core::fmt;

use miden_crypto::{hash::rpo::RpoDigest, Felt};
use miden_formatting::prettier::PrettyPrint;

use crate::{
    chiplets::hasher,
    mast::{MastForest, MastForestError, MastNodeId},
    OPCODE_CALL, OPCODE_SYSCALL,
};

// CALL NODE
// ================================================================================================

/// A Call node describes a function call such that the callee is executed in a different execution
/// context from the currently executing code.
///
/// A call node can be of two types:
/// - A simple call: the callee is executed in the new user context.
/// - A syscall: the callee is executed in the root context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallNode {
    callee: MastNodeId,
    is_syscall: bool,
    digest: RpoDigest,
}

//-------------------------------------------------------------------------------------------------
/// Constants
impl CallNode {
    /// The domain of the call block (used for control block hashing).
    pub const CALL_DOMAIN: Felt = Felt::new(OPCODE_CALL as u64);
    /// The domain of the syscall block (used for control block hashing).
    pub const SYSCALL_DOMAIN: Felt = Felt::new(OPCODE_SYSCALL as u64);
}

//-------------------------------------------------------------------------------------------------
/// Constructors
impl CallNode {
    /// Returns a new [`CallNode`] instantiated with the specified callee.
    pub fn new(callee: MastNodeId, mast_forest: &MastForest) -> Result<Self, MastForestError> {
        if usize::from(callee) >= mast_forest.nodes.len() {
            return Err(MastForestError::NodeIdOverflow(callee, mast_forest.nodes.len()));
        }
        let digest = {
            let callee_digest = mast_forest[callee].digest();

            hasher::merge_in_domain(&[callee_digest, RpoDigest::default()], Self::CALL_DOMAIN)
        };

        Ok(Self { callee, is_syscall: false, digest })
    }

    /// Returns a new [`CallNode`] instantiated with the specified callee and marked as a kernel
    /// call.
    pub fn new_syscall(
        callee: MastNodeId,
        mast_forest: &MastForest,
    ) -> Result<Self, MastForestError> {
        if usize::from(callee) >= mast_forest.nodes.len() {
            return Err(MastForestError::NodeIdOverflow(callee, mast_forest.nodes.len()));
        }
        let digest = {
            let callee_digest = mast_forest[callee].digest();

            hasher::merge_in_domain(&[callee_digest, RpoDigest::default()], Self::SYSCALL_DOMAIN)
        };

        Ok(Self { callee, is_syscall: true, digest })
    }
}

//-------------------------------------------------------------------------------------------------
/// Public accessors
impl CallNode {
    /// Returns a commitment to this Call node.
    ///
    /// The commitment is computed as a hash of the callee and an empty word ([ZERO; 4]) in the
    /// domain defined by either [Self::CALL_DOMAIN] or [Self::SYSCALL_DOMAIN], depending on
    /// whether the node represents a simple call or a syscall - i.e.,:
    /// ```
    /// # use miden_core::mast::CallNode;
    /// # use miden_crypto::{hash::rpo::{RpoDigest as Digest, Rpo256 as Hasher}};
    /// # let callee_digest = Digest::default();
    /// Hasher::merge_in_domain(&[callee_digest, Digest::default()], CallNode::CALL_DOMAIN);
    /// ```
    /// or
    /// ```
    /// # use miden_core::mast::CallNode;
    /// # use miden_crypto::{hash::rpo::{RpoDigest as Digest, Rpo256 as Hasher}};
    /// # let callee_digest = Digest::default();
    /// Hasher::merge_in_domain(&[callee_digest, Digest::default()], CallNode::SYSCALL_DOMAIN);
    /// ```
    pub fn digest(&self) -> RpoDigest {
        self.digest
    }

    /// Returns the ID of the node to be invoked by this call node.
    pub fn callee(&self) -> MastNodeId {
        self.callee
    }

    /// Returns true if this call node represents a syscall.
    pub fn is_syscall(&self) -> bool {
        self.is_syscall
    }

    /// Returns the domain of this call node.
    pub fn domain(&self) -> Felt {
        if self.is_syscall() {
            Self::SYSCALL_DOMAIN
        } else {
            Self::CALL_DOMAIN
        }
    }
}

// PRETTY PRINTING
// ================================================================================================

impl CallNode {
    pub(super) fn to_pretty_print<'a>(
        &'a self,
        mast_forest: &'a MastForest,
    ) -> impl PrettyPrint + 'a {
        CallNodePrettyPrint { call_node: self, mast_forest }
    }

    pub(super) fn to_display<'a>(&'a self, mast_forest: &'a MastForest) -> impl fmt::Display + 'a {
        CallNodePrettyPrint { call_node: self, mast_forest }
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
