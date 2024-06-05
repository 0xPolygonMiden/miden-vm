use alloc::vec::Vec;
use miden_crypto::hash::rpo::RpoDigest;

use crate::{program::blocks::OpBatch, Kernel};

pub trait MerkleTreeNode {
    fn digest(&self) -> RpoDigest;
}

#[derive(Debug, Clone, Copy)]
pub struct MastNodeId(usize);

pub struct MastForest {
    /// All of the blocks local to the trees comprising the MAST forest
    nodes: Vec<MastNode>,
    roots: Vec<MastNodeId>,
    /// The "entrypoint", when set, is the root of the entire forest, i.e.
    /// a path exists from this node to all other roots in the forest. This
    /// corresponds to the executable entry point. When not set, the forest
    /// may or may not have such a root in `roots`, but is not required.
    /// Whether or not the entrypoint is set distinguishes a MAST which is
    /// executable, versus a MAST which represents a library.
    ///
    /// NOTE: The entrypoint is also present in `roots` if set
    entrypoint: Option<MastNodeId>,
    kernel: Kernel,
}

impl MastForest {
    pub fn entrypoint(&self) -> Option<MastNodeId> {
        self.entrypoint
    }

    pub fn get_node_by_id(&self, node_id: MastNodeId) -> &MastNode {
        &self.nodes[node_id.0]
    }
}

pub enum MastNode {
    Block(BasicBlockNode),
    Join(JoinNode),
    Split(SplitNode),
    Loop(LoopNode),
    Call(CallNode),
    Dyn(DynNode),
    /// A reference to a node whose definition is not
    /// local to the containing `MastForest`.
    External(RpoDigest),
}

impl MerkleTreeNode for MastNode {
    fn digest(&self) -> RpoDigest {
        match self {
            MastNode::Block(_) => todo!(),
            MastNode::Join(node) => node.digest(),
            MastNode::Split(_) => todo!(),
            MastNode::Loop(_) => todo!(),
            MastNode::Call(_) => todo!(),
            MastNode::Dyn(_) => todo!(),
            MastNode::External(_) => todo!(),
        }
    }
}

pub struct BasicBlockNode {
    /// The primitive operations contained in this basic block.
    ///
    /// The operations are broken up into batches of 8 groups,
    /// with each group containing up to 9 operations, or a
    /// single immediates. Thus the maximum size of each batch
    /// is 72 operations. Multiple batches are used for blocks
    /// consisting of more than 72 operations.
    batches: Vec<OpBatch>,
}

pub struct JoinNode {
    children: [MastNodeId; 2],
    digest: RpoDigest,
}

impl JoinNode {
    pub fn first(&self) -> MastNodeId {
        self.children[0]
    }

    pub fn second(&self) -> MastNodeId {
        self.children[1]
    }
}

impl MerkleTreeNode for JoinNode {
    fn digest(&self) -> RpoDigest {
        self.digest
    }
}

pub struct SplitNode {
    children: [MastNodeId; 2],
}

pub struct LoopNode {
    body: MastNodeId,
}

pub struct CallNode {
    callee: MastNodeId,
    is_syscall: bool,
}

pub struct DynNode;
