use alloc::{collections::BTreeMap, vec::Vec};
use miden_crypto::{hash::rpo::RpoDigest, Felt};

use crate::{program::blocks::OpBatch, Kernel, Operation};

pub trait MerkleTreeNode {
    fn digest(&self) -> RpoDigest;
}

// TODOP: equality can only be checked by accessing the node it refers too,
// and should be a node digest equality checks.
// Otherwise our mapping `node_hash -> MastNodeId` breaks
// And e.g. 2 dyn nodes would be considered "not equal"
#[derive(Debug, Clone, Copy)]
pub struct MastNodeId(usize);

pub struct MastForest {
    /// All of the blocks local to the trees comprising the MAST forest
    nodes: Vec<MastNode>,
    node_id_by_hash: BTreeMap<RpoDigest, MastNodeId>,
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

    pub fn get_node_id_by_digest(&self, digest: RpoDigest) -> Option<MastNodeId> {
        self.node_id_by_hash.get(&digest).copied()
    }
}

pub enum MastNode {
    Block(BasicBlockNode),
    Join(JoinNode),
    Split(SplitNode),
    Loop(LoopNode),
    Call(CallNode),
    Dyn,
    /// A reference to a node whose definition is not
    /// local to the containing `MastForest`.
    External(RpoDigest),
}

impl MerkleTreeNode for MastNode {
    fn digest(&self) -> RpoDigest {
        match self {
            MastNode::Block(_) => todo!(),
            MastNode::Join(node) => node.digest(),
            MastNode::Split(node) => node.digest(),
            MastNode::Loop(node) => node.digest(),
            MastNode::Call(node) => node.digest(),
            MastNode::Dyn => DynNode.digest(),
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
    branches: [MastNodeId; 2],
    digest: RpoDigest,
}

impl SplitNode {
    pub fn on_true(&self) -> MastNodeId {
        self.branches[0]
    }

    pub fn on_false(&self) -> MastNodeId {
        self.branches[1]
    }
}

impl MerkleTreeNode for SplitNode {
    fn digest(&self) -> RpoDigest {
        self.digest
    }
}

pub struct LoopNode {
    body: MastNodeId,
    digest: RpoDigest,
}

impl LoopNode {
    pub fn body(&self) -> MastNodeId {
        self.body
    }
}

impl MerkleTreeNode for LoopNode {
    fn digest(&self) -> RpoDigest {
        self.digest
    }
}

pub struct CallNode {
    // Q: This prevents encoding `DYN_DIGEST`
    callee: MastNodeId,
    is_syscall: bool,
    digest: RpoDigest,
}

impl CallNode {
    // CONSTANTS
    // --------------------------------------------------------------------------------------------
    /// The domain of the call block (used for control block hashing).
    pub const CALL_DOMAIN: Felt = Felt::new(Operation::Call.op_code() as u64);
    /// The domain of the syscall block (used for control block hashing).
    pub const SYSCALL_DOMAIN: Felt = Felt::new(Operation::SysCall.op_code() as u64);

    pub fn callee(&self) -> MastNodeId {
        self.callee
    }

    pub fn is_syscall(&self) -> bool {
        self.is_syscall
    }

    /// Returns the domain of the call node.
    pub fn hash_domain(&self) -> Felt {
        if self.is_syscall() {
            Self::SYSCALL_DOMAIN
        } else {
            Self::CALL_DOMAIN
        }
    }
}

impl MerkleTreeNode for CallNode {
    fn digest(&self) -> RpoDigest {
        self.digest
    }
}

pub struct DynNode;

impl DynNode {
    // CONSTANTS
    // --------------------------------------------------------------------------------------------
    /// The domain of the Dyn block (used for control block hashing).
    pub const DOMAIN: Felt = Felt::new(Operation::Dyn.op_code() as u64);
}

impl MerkleTreeNode for DynNode {
    fn digest(&self) -> RpoDigest {
        // The Dyn node is represented by a constant, which is set to be the hash of two empty
        // words ([ZERO, ZERO, ZERO, ZERO]) with a domain value of `DYN_DOMAIN`, i.e.
        // hasher::merge_in_domain(&[Digest::default(), Digest::default()], DynNode::DOMAIN)
        RpoDigest::new([
            Felt::new(8115106948140260551),
            Felt::new(13491227816952616836),
            Felt::new(15015806788322198710),
            Felt::new(16575543461540527115),
        ])
    }
}
